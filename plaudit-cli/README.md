# plaudit-cli

A single-binary CLI for exporting your [Plaud](https://www.plaud.ai/) recordings,
transcripts, and AI summaries to local files.

This is a Rust port of Plaud's official npm-based CLI (`@plaud-ai/cli`). It speaks
the same OAuth 2.0 + PKCE flow and the same REST API, but compiles to a static
binary with no runtime dependencies. Built on [`plaudit-core`](https://crates.io/crates/plaudit-core).

## Install

```sh
cargo install plaudit-cli --locked
```

## Usage

```sh
# Authenticate (opens browser, catches OAuth callback on :8199)
plaudit login

# Account info
plaudit me

# Browse recordings
plaudit files --page 1 --page-size 20
plaudit file <id>

# Export a single recording
plaudit transcript <id>              # plain text to stdout
plaudit transcript <id> --srt        # SRT subtitles
plaudit transcript <id> -o out.txt   # write to file
plaudit summary <id>                 # AI summary as Markdown
plaudit audio <id>                   # 24h presigned download URL

# Bulk export everything as Markdown
plaudit sync ~/plaud-export

# Tear down
plaudit logout
```

### `sync`

`plaudit sync <folder>` paginates through every recording and writes one
Markdown file per recording (`YYYY-MM-DD_slug.md`) containing YAML frontmatter,
the AI summary, and the full transcript. Files already on disk are skipped, so
it's safe to run repeatedly as an incremental export.

## Auth

`plaudit login` opens your default browser for Plaud's OAuth consent screen.
The callback is caught on `127.0.0.1:8199`. Tokens are stored in
`~/.plaud/tokens.json` (mode `0600`) and refresh automatically. This is the
same config directory used by Plaud's official CLI, so credentials interoperate.

## License

MIT