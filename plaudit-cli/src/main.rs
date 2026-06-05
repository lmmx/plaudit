use clap::{Parser, Subcommand};
use plaudit_core::{
    format_date, format_duration, run_login, segments_to_srt, segments_to_text, summary_markdown,
    transcript_segments, Client,
};
use std::fs;
use std::path::Path;

type R = plaudit_core::Result<()>;

#[derive(Parser)]
#[command(name = "plaudit", about = "Export your Plaud recordings and transcripts")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Authenticate via browser OAuth
    Login,
    /// Revoke and forget credentials
    Logout,
    /// Show the authenticated account
    Me,
    /// List one page of recordings
    Files {
        #[arg(short, long, default_value_t = 1)]
        page: u32,
        #[arg(short = 's', long = "page-size", default_value_t = 20)]
        page_size: u32,
    },
    /// Show one recording's details
    File { id: String },
    /// Print or save a transcript
    Transcript {
        id: String,
        #[arg(long)]
        srt: bool,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Print or save the AI summary
    Summary {
        id: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Print the 24h audio download URL
    Audio { id: String },
    /// Export every recording to a folder as Markdown
    Sync { folder: String },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("\u{2717} {e}");
        std::process::exit(1);
    }
}

fn run() -> R {
    let c = Client::new();
    match Cli::parse().cmd {
        Cmd::Login => {
            run_login(c.oauth())?;
            println!("Logged in.");
        }
        Cmd::Logout => {
            c.revoke_current_user();
            c.oauth().logout();
            println!("Logged out.");
        }
        Cmd::Me => {
            let u = c.get_current_user()?;
            match u.as_object() {
                Some(obj) => {
                    for (k, v) in obj {
                        println!("{k}: {}", v.to_string().trim_matches('"'));
                    }
                }
                None => println!("{u}"),
            }
        }
        Cmd::Files { page, page_size } => {
            let r = c.list_files(page, page_size)?;
            for f in &r.data {
                println!(
                    "{}  {}  {}  {}",
                    f.id,
                    format_date(f.created_at.as_deref().unwrap_or("")),
                    f.name.as_deref().unwrap_or(""),
                    format_duration(f.duration.unwrap_or(0)),
                );
            }
            println!("\npage {}", r.page);
        }
        Cmd::File { id } => {
            let f = c.get_file(&id)?;
            println!("id:         {}", f.id);
            println!("name:       {}", f.name.as_deref().unwrap_or("-"));
            println!("created_at: {}", f.created_at.as_deref().unwrap_or("-"));
            println!("duration:   {}", format_duration(f.duration.unwrap_or(0)));
            println!("audio:      {}", if f.presigned_url.is_some() { "available" } else { "-" });
            println!("transcript: {}", if transcript_segments(&f).is_some() { "available" } else { "-" });
            println!("summary:    {}", if summary_markdown(&f).is_some() { "available" } else { "-" });
        }
        Cmd::Transcript { id, srt, output } => {
            let f = c.get_file(&id)?;
            let segs = transcript_segments(&f).ok_or("Transcript not available for this recording.")?;
            let out = if srt { segments_to_srt(&segs) } else { segments_to_text(&segs) };
            emit(output, &out)?;
        }
        Cmd::Summary { id, output } => {
            let f = c.get_file(&id)?;
            let md = summary_markdown(&f).ok_or("Summary not available for this recording.")?;
            emit(output, &md)?;
        }
        Cmd::Audio { id } => match c.get_file(&id)?.presigned_url {
            Some(u) => {
                println!("{u}");
                eprintln!("(expires in 24h)");
            }
            None => println!("Audio not available."),
        },
        Cmd::Sync { folder } => sync(&c, &folder)?,
    }
    Ok(())
}

fn emit(output: Option<String>, content: &str) -> R {
    match output {
        Some(p) => {
            fs::write(&p, content)?;
            println!("Saved to {p}");
        }
        None => println!("{content}"),
    }
    Ok(())
}

fn sync(c: &Client, folder: &str) -> R {
    fs::create_dir_all(folder)?;
    let mut page = 1u32;
    let mut total = 0;
    loop {
        let r = c.list_files(page, 100)?;
        if r.data.is_empty() {
            break;
        }
        for f in &r.data {
            let date = format_date(f.created_at.as_deref().unwrap_or(""));
            let slug = slugify(f.name.as_deref().unwrap_or(""), &f.id);
            let path = Path::new(folder).join(format!("{date}_{slug}.md"));
            if path.exists() {
                continue;
            }
            let detail = c.get_file(&f.id)?;
            let transcript = transcript_segments(&detail)
                .map(|s| segments_to_text(&s))
                .unwrap_or_else(|| "*(no transcript)*".into());
            let summary = summary_markdown(&detail).unwrap_or_default();
            let name = f.name.as_deref().unwrap_or("");
            let summary_block = if summary.is_empty() {
                String::new()
            } else {
                format!("## Summary\n\n{summary}\n\n")
            };
            let body = format!(
                "---\nplaud_id: {}\nname: \"{}\"\ndate: {}\n---\n\n# {}\n\n{}## Transcript\n\n{}\n",
                f.id, name, date, name, summary_block, transcript
            );
            fs::write(&path, body)?;
            total += 1;
            println!("Synced {}", f.id);
        }
        if r.data.len() < 100 {
            break;
        }
        page += 1;
    }
    println!(
        "{}",
        if total > 0 {
            format!("Synced {total} new recording(s).")
        } else {
            "Already up to date.".into()
        }
    );
    Ok(())
}

fn slugify(name: &str, id: &str) -> String {
    if name.is_empty() {
        return id.into();
    }
    let mut s = String::new();
    let mut us = false;
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            s.push(ch);
            us = false;
        } else if !us {
            s.push('_');
            us = true;
        }
    }
    s.chars().take(50).collect()
}
