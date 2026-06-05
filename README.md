# plaudit

An auditable, from-source CLI for exporting your own [Plaud](https://www.plaud.ai/)
recordings, transcripts, and AI summaries to local files.

Plaud ships an official CLI, but it's a closed-source npm package (`@plaud-ai/cli`)
installed via `npx`/`npm -g`. `plaudit` is a Rust reimplementation of the same
client: it speaks the same OAuth 2.0 + PKCE flow and the same REST developer API
(`platform.plaud.ai/developer/api`), but as a single static binary with a
dependency tree you can read end to end — no npm, no install-time scripts.

## How it works

`plaudit` authenticates with browser-based OAuth (PKCE; no password stored on
disk), reusing Plaud's public CLI client registration. It then calls the REST API
to list recordings and fetch transcripts, summaries, and audio URLs, writing
results to stdout or to files. It is a plain HTTP client — not an MCP server or
client, and it registers no tools. For AI-assistant access, use Plaud's official
MCP server instead.

## Install

    cargo install --path plaudit-cli --locked

## Usage

    plaudit login                         # browser OAuth; tokens in ~/.plaud
    plaudit logout
    plaudit me
    plaudit files [--page N --page-size M]
    plaudit file <id>
    plaudit transcript <id> [--srt] [-o FILE]
    plaudit summary <id> [-o FILE]
    plaudit audio <id>                    # prints a 24h presigned URL
    plaudit sync <folder>                 # export everything as Markdown

## Auth

`plaudit login` opens your browser; the OAuth callback is caught locally on port
8199. Tokens land in `~/.plaud/tokens.json` (mode 0600) and refresh automatically.
The `~/.plaud` directory is Plaud's shared client config location, reused here so
credentials interoperate with the official CLI.

## Disclaimer

Unofficial client. Uses Plaud's documented developer API for your own account.
Not affiliated with or endorsed by Plaud.

## License

MIT