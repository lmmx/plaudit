# plaudit-cli

The `plaudit` binary — a command-line tool to export your Plaud recordings,
transcripts, and summaries. A thin clap frontend over [`plaudit-core`](../plaudit-core).

See the [repo README](../README.md) for full usage. Quick reference:

    plaudit login | logout | me
    plaudit files [--page N --page-size M]
    plaudit file <id>
    plaudit transcript <id> [--srt] [-o FILE]
    plaudit summary <id> [-o FILE]
    plaudit audio <id>
    plaudit sync <folder>

## License

MIT