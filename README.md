# plaudit

A from-source Rust CLI for exporting your own [Plaud](https://www.plaud.ai/)
recordings, transcripts, and AI summaries to local files.

Plaud's official CLI is a closed-source npm package (`@plaud-ai/cli`). `plaudit`
is a clean-room Rust port of the same client: same OAuth 2.0 + PKCE flow, same
REST API (`platform.plaud.ai/developer/api`), zero runtime dependencies. One
static binary, fully auditable dependency tree.

## How it works

`plaudit` authenticates via browser-based OAuth (PKCE; no password stored on
disk), reusing Plaud's public CLI client registration. It calls the REST API to
list recordings and fetch transcripts, summaries, and audio URLs, writing results
to stdout or to files. It is a plain HTTP client — not an MCP server, and it
registers no tools. For AI-assistant integration, use Plaud's official MCP server.

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

## Status

This is a community port, not an official Plaud product. It only accesses your
own account data through Plaud's documented public developer API — the same
endpoints their own CLI uses.

## License

MIT