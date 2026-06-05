# plaudit-core

Rust client library for the [Plaud](https://www.plaud.ai/) developer API
(`platform.plaud.ai/developer/api`). Handles OAuth 2.0 + PKCE authentication,
token persistence, and typed access to recordings, transcripts, and AI summaries.

This is the engine behind [`plaudit-cli`](https://crates.io/crates/plaudit-cli).
It's frontend-agnostic — all public functions return structured types and
`Result`s with no direct I/O to stdout.

## Quick start

```rust
use plaudit_core::{Client, run_login, transcript_segments, segments_to_text, summary_markdown};

// First run: authenticate via browser OAuth
let client = Client::new();
run_login(client.oauth())?;

// List recordings
let page = client.list_files(1, 20)?;
for file in &page.data {
    println!("{} — {:?}", file.id, file.name);
}

// Fetch a transcript
let detail = client.get_file("some-file-id")?;
if let Some(segments) = transcript_segments(&detail) {
    println!("{}", segments_to_text(&segments));
}

// Fetch the AI summary
if let Some(md) = summary_markdown(&detail) {
    println!("{md}");
}
```

## Modules

### `client` — API client

`Client` wraps authenticated requests to the Plaud REST API. Tokens are loaded
from disk and refreshed transparently.

| Method | Endpoint |
|---|---|
| `get_current_user()` → `Value` | `GET /users/current` |
| `revoke_current_user()` | `POST /users/current/revoke` |
| `list_files(page, page_size)` → `FilesPage` | `GET /files/` |
| `get_file(id)` → `FileDetail` | `GET /files/{id}` |

### `oauth` — PKCE login and token management

- `run_login(oauth)` — full browser-based PKCE flow (binds `:8199`, opens
  browser, exchanges the authorization code)
- `OAuth::get_access_token()` — returns a valid token, refreshing if needed
- `TokenStore` — reads/writes `~/.plaud/tokens.json` with `0600` permissions

### `types` — Data structures

- `FileSummary` / `FileDetail` — recording metadata, audio URL, embedded
  source and note data
- `Segment` — a single transcript segment with `start_time`, `end_time`,
  optional `speaker`, and `content`
- `DataItem` — raw `data_type` + `data_content` container used by the API for
  transcripts (`"transaction"`) and summaries (`"auto_sum_note"`)
- `TokenSet` — persisted OAuth tokens

### `fmt` — Formatting and extraction

- `transcript_segments(file)` — extracts `Vec<Segment>` from a `FileDetail`
- `summary_markdown(file)` — extracts the AI summary string from a `FileDetail`
- `segments_to_text(segs)` — renders `[MM:SS - MM:SS] Speaker: content` lines
- `segments_to_srt(segs)` — renders SRT subtitle format
- `format_duration(ms)` / `format_date(iso)` — human-friendly display helpers

## License

MIT
