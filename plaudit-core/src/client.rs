use crate::oauth::OAuth;
use crate::types::{FileDetail, FilesPage};
use crate::{Result, API_BASE};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub struct Client {
    oauth: OAuth,
}

impl Client {
    pub fn new() -> Self {
        Client {
            oauth: OAuth::new(),
        }
    }

    pub fn oauth(&self) -> &OAuth {
        &self.oauth
    }

    fn request<T: DeserializeOwned>(&self, method: &str, path: &str) -> Result<T> {
        let token = self
            .oauth
            .get_access_token()?
            .ok_or("Not authenticated. Run `plaud login` first.")?;
        let url = format!("{API_BASE}{path}");
        let req = if method == "POST" {
            ureq::post(&url)
        } else {
            ureq::get(&url)
        };
        match req
            .set("Authorization", &format!("Bearer {token}"))
            .set("Accept", "application/json")
            .call()
        {
            Ok(r) => Ok(r.into_json::<T>()?),
            Err(ureq::Error::Status(code, r)) => {
                let body = r.into_string().unwrap_or_default();
                if code == 422 {
                    if let Ok(v) = serde_json::from_str::<Value>(&body) {
                        if let Some(detail) = v.get("detail").and_then(|d| d.as_array()) {
                            let msgs: Vec<String> = detail
                                .iter()
                                .map(|d| {
                                    let loc = d
                                        .get("loc")
                                        .and_then(|l| l.as_array())
                                        .and_then(|a| a.last())
                                        .and_then(|x| x.as_str())
                                        .unwrap_or("");
                                    let m = d.get("msg").and_then(|x| x.as_str()).unwrap_or("");
                                    format!("{loc}: {m}")
                                })
                                .collect();
                            return Err(msgs.join("; ").into());
                        }
                    }
                }
                Err(format!("API error: {code}").into())
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn get_current_user(&self) -> Result<Value> {
        self.request("GET", "/open/third-party/users/current")
    }

    /// Best-effort revoke; ignores body and errors (matches official CLI).
    pub fn revoke_current_user(&self) {
        let _ = self.request::<Value>("POST", "/open/third-party/users/current/revoke");
    }

    pub fn list_files(&self, page: u32, page_size: u32) -> Result<FilesPage> {
        self.request(
            "GET",
            &format!("/open/third-party/files/?page={page}&page_size={page_size}"),
        )
    }

    pub fn get_file(&self, id: &str) -> Result<FileDetail> {
        self.request("GET", &format!("/open/third-party/files/{id}"))
    }
}
