use crate::types::{TokenResponse, TokenSet};
use crate::{Result, AUTH_URL, CALLBACK_PORT, CLIENT_ID, REDIRECT_URI, REFRESH_URL, TOKEN_URL};
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TokenStore {
    path: PathBuf,
}

impl Default for TokenStore {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenStore {
    pub fn new() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        TokenStore {
            path: PathBuf::from(home).join(".plaud").join("tokens.json"),
        }
    }

    pub fn load(&self) -> Option<TokenSet> {
        fs::read_to_string(&self.path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
    }

    pub fn save(&self, t: &TokenSet) -> Result<()> {
        if let Some(dir) = self.path.parent() {
            fs::create_dir_all(dir)?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = fs::set_permissions(dir, fs::Permissions::from_mode(0o700));
            }
        }
        fs::write(&self.path, serde_json::to_string_pretty(t)?)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&self.path, fs::Permissions::from_mode(0o600))?;
        }
        Ok(())
    }

    pub fn clear(&self) {
        let _ = fs::remove_file(&self.path);
    }
}

pub struct OAuth {
    store: TokenStore,
}

impl Default for OAuth {
    fn default() -> Self {
        Self::new()
    }
}

impl OAuth {
    pub fn new() -> Self {
        OAuth {
            store: TokenStore::new(),
        }
    }

    pub fn get_access_token(&self) -> Result<Option<String>> {
        let Some(t) = self.store.load() else {
            return Ok(None);
        };
        if let Some(exp) = t.expires_at {
            if now_ms() > exp - 60_000 {
                return match &t.refresh_token {
                    Some(rt) => Ok(self.refresh(rt).ok().map(|n| n.access_token)),
                    None => Ok(None),
                };
            }
        }
        Ok(Some(t.access_token))
    }

    pub fn logout(&self) {
        self.store.clear();
    }

    fn exchange(&self, code: &str, verifier: &str, state: &str) -> Result<TokenSet> {
        // Mirrors the official client: Basic "clientId:" (empty secret) + form body.
        let basic = STANDARD.encode(format!("{CLIENT_ID}:"));
        let resp = ureq::post(TOKEN_URL)
            .set("Accept", "application/json")
            .set("Authorization", &format!("Basic {basic}"))
            .send_form(&[
                ("code", code),
                ("redirect_uri", REDIRECT_URI),
                ("code_verifier", verifier),
                ("state", state),
            ])?;
        let set = to_set(resp.into_json()?, None);
        self.store.save(&set)?;
        Ok(set)
    }

    fn refresh(&self, refresh_token: &str) -> Result<TokenSet> {
        let resp = ureq::post(REFRESH_URL)
            .set("Accept", "application/json")
            .send_form(&[("refresh_token", refresh_token)])?;
        let set = to_set(resp.into_json()?, Some(refresh_token.to_string()));
        self.store.save(&set)?;
        Ok(set)
    }
}

fn to_set(d: TokenResponse, fallback_refresh: Option<String>) -> TokenSet {
    TokenSet {
        access_token: d.access_token,
        refresh_token: d.refresh_token.or(fallback_refresh),
        token_type: d.token_type.unwrap_or_else(|| "Bearer".into()),
        expires_at: d.expires_in.map(|s| now_ms() + s * 1000),
    }
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// Full PKCE login: open browser, catch the callback on 8199, exchange the code.
pub fn run_login(oauth: &OAuth) -> Result<()> {
    let verifier = rand_b64url(32);
    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
    let state = rand_b64url(16);

    let url = format!(
        "{AUTH_URL}?client_id={}&redirect_uri={}&response_type=code\
         &code_challenge={}&code_challenge_method=S256&state={}",
        pct(CLIENT_ID),
        pct(REDIRECT_URI),
        pct(&challenge),
        pct(&state),
    );

    // Bind first so we fail fast if the fixed redirect port is taken.
    let listener = TcpListener::bind(("127.0.0.1", CALLBACK_PORT)).map_err(|e| {
        format!("callback port {CALLBACK_PORT} unavailable ({e}). Another login or plaud-mcp may hold it.")
    })?;

    println!("Opening browser for authentication...");
    if open_browser(&url).is_err() {
        println!("Could not open browser. Open this URL manually:\n  {url}");
    }

    let code = wait_for_code(listener, &state)?;
    oauth.exchange(&code, &verifier, &state)?;
    Ok(())
}

fn wait_for_code(listener: TcpListener, expected_state: &str) -> Result<String> {
    for stream in listener.incoming() {
        let mut stream = stream?;
        let target = {
            let mut reader = BufReader::new(&stream);
            let mut line = String::new();
            reader.read_line(&mut line)?;
            line.split_whitespace().nth(1).unwrap_or("").to_string()
        };

        if !target.starts_with("/auth/callback") {
            respond(&mut stream, "Continue in the original window.");
            continue;
        }

        let query = target.split_once('?').map(|(_, q)| q).unwrap_or("");
        let (mut code, mut state, mut err) = (None, None, None);
        for pair in query.split('&') {
            let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
            match k {
                "code" => code = Some(urldecode(v)),
                "state" => state = Some(urldecode(v)),
                "error" => err = Some(urldecode(v)),
                _ => {}
            }
        }

        if let Some(e) = err {
            respond(&mut stream, "Authorization failed.");
            return Err(format!("authorization denied: {e}").into());
        }
        if state.as_deref() != Some(expected_state) {
            respond(&mut stream, "State mismatch; ignoring.");
            continue;
        }
        match code {
            Some(c) => {
                respond(
                    &mut stream,
                    "Authorization successful — you can close this tab.",
                );
                return Ok(c);
            }
            None => respond(&mut stream, "Continue in the original window."),
        }
    }
    Err("callback listener closed without a code".into())
}

fn respond(stream: &mut TcpStream, msg: &str) {
    let body = format!(
        "<!doctype html><meta charset=utf-8>\
         <body style=\"font-family:system-ui;padding:2rem;text-align:center\">\
         <h1>Plaud</h1><p>{msg}</p>"
    );
    let _ = write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    let _ = stream.flush();
}

fn rand_b64url(n: usize) -> String {
    let mut buf = vec![0u8; n];
    getrandom::getrandom(&mut buf).expect("OS RNG unavailable");
    URL_SAFE_NO_PAD.encode(&buf)
}

fn pct(s: &str) -> String {
    let mut o = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                o.push(b as char)
            }
            _ => o.push_str(&format!("%{b:02X}")),
        }
    }
    o
}

fn urldecode(s: &str) -> String {
    let b = s.as_bytes();
    let mut o = Vec::new();
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'%' if i + 2 < b.len() => match u8::from_str_radix(&s[i + 1..i + 3], 16) {
                Ok(byte) => {
                    o.push(byte);
                    i += 3;
                }
                Err(_) => {
                    o.push(b'%');
                    i += 1;
                }
            },
            b'+' => {
                o.push(b' ');
                i += 1;
            }
            c => {
                o.push(c);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&o).into_owned()
}

#[cfg(target_os = "macos")]
fn open_browser(url: &str) -> Result<()> {
    std::process::Command::new("open").arg(url).spawn()?;
    Ok(())
}
#[cfg(target_os = "linux")]
fn open_browser(url: &str) -> Result<()> {
    std::process::Command::new("xdg-open").arg(url).spawn()?;
    Ok(())
}
#[cfg(target_os = "windows")]
fn open_browser(url: &str) -> Result<()> {
    std::process::Command::new("cmd")
        .args(["/C", "start", "", url])
        .spawn()?;
    Ok(())
}
