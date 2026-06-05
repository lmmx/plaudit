# plaudit-core

Library crate for [`plaudit`](../README.md): the OAuth 2.0 + PKCE client, REST
client, data types, and transcript/summary formatting for Plaud's developer API.

Frontend-agnostic — it returns structured data and `Result`s and does no I/O to
stdout. Consumed by the `plaudit-cli` binary crate.

## Modules

- `oauth` — PKCE login flow, token store (`~/.plaud/tokens.json`), refresh
- `client` — `Client` over `platform.plaud.ai/developer/api`
- `types` — `FileSummary`, `FileDetail`, `Segment`, token types
- `fmt` — duration/date formatting, transcript → text/SRT, summary extraction

## License

MIT