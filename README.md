# DelayUsRepayUs

## Bad Idea, Fun Execution

[![CI](https://github.com/ColonelPanicc/DelayUsRepayUs/actions/workflows/ci.yml/badge.svg)](https://github.com/ColonelPanicc/DelayUsRepayUs/actions/workflows/ci.yml)
[![brainmade.org](https://img.shields.io/badge/brainmade.org-000000?style=flat&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxZW0iIGhlaWdodD0iNzkiIHZpZXdCb3g9IjAgMCA2NyA3OSIgZmlsbD0ibm9uZSI%2BPHBhdGggZmlsbD0iI2ZmZiIgZD0iTTUyLjYxMiA3OC43ODJIMjMuMzNhMi41NTkgMi41NTkgMCAwIDEtMi41Ni0yLjU1OHYtNy42NzdoLTcuOTczYTIuNTYgMi41NiAwIDAgMS0yLjU2LTIuNTZWNTUuMzE1bC04LjgyLTQuMzk3YTIuNTU5IDIuNTU5IDAgMCAxLS45ODYtMy43MWw5LjgwNy0xNC43MTR2LTQuMzVDMTAuMjQgMTIuNTk5IDIyLjg0MyAwIDM4LjM4OCAwIDUzLjkzMiAwIDY2LjUzNCAxMi42IDY2LjUzOCAyOC4xNDNjLS42MzIgMjcuODI0LTEwLjc2IDIzLjUxNi0xMS4xOCAzNC4wNDVsLS4xODcgMTQuMDM1YTIuNTkgMi41OSAwIDAgMS0uNzUgMS44MSAyLjU1IDIuNTUgMCAwIDEtMS44MDkuNzVabS0yNi43MjMtNS4xMTdoMjQuMTY0bC4yODYtMTQuNTQyYy0uMjYzLTYuNjU2IDExLjcxNi04LjI0MyAxMS4wOC0zMC43MzQtLjM1OC0xMi43MTMtMTAuMzEzLTIzLjI3MS0yMy4wMzEtMjMuMjcxLTEyLjcxOCAwLTIzLjAyOSAxMC4zMDctMjMuMDMyIDIzLjAyNXY1LjExN2MwIC41MDYtLjE1IDEtLjQzIDEuNDJsLTguNjMgMTIuOTQxIDcuNjQ1IDMuODJhMi41NTkgMi41NTkgMCAwIDEgMS40MTUgMi4yOTF2OS42OTdoNy45NzRhMi41NTkgMi41NTkgMCAwIDEgMi41NiAyLjU1OXY3LjY3N1oiLz48cGF0aCBmaWxsPSIjZmZmIiBkPSJNNDAuMzcyIDU4LjIyMlYzOC45MzRjLjExOCAwIC4yMzcuMDE4LjM1NS4wMTggOS43NjktLjAxMiAxNy4wNS05LjAxMiAxNS4wMjItMTguNTY3YTIuMzY2IDIuMzY2IDAgMCAwLTEuODIxLTEuODIyYy04LjEwNi0xLjczLTE2LjEyMSAzLjI5Mi0xOC4wOTggMTEuMzQxLS4wMjQtLjAyNC0uMDQzLS4wNS0uMDY2LS4wNzNhMTUuMzIzIDE1LjMyMyAwIDAgMC0xNC4wNi00LjE3IDIuMzY1IDIuMzY1IDAgMCAwLTEuODIxIDEuODJjLTIuMDI4IDkuNTU1IDUuMjUyIDE4LjU1NCAxNS4wMiAxOC41NjguMjM2IDAgLjQ5Mi0uMDI4LjczOC0uMDR2MTIuMjEzaDQuNzMxWm0yLjgzOS0zMi4xNDNhMTAuNjQ2IDEwLjY0NiAwIDAgMSA4LjEyNC0zLjEwNmMuMzUgNi4zNC00Ljg4OCAxMS41NzctMTEuMjI4IDExLjIzYTEwLjU4IDEwLjU4IDAgMCAxIDMuMTA0LTguMTI0Wk0yNy40MDMgMzguMTkzYTEwLjYwNyAxMC42MDcgMCAwIDEtMy4xMTgtOC4xMjNjNi4zNDQtLjM1OCAxMS41ODcgNC44ODYgMTEuMjI4IDExLjIzLTMuMDIzLjE2OS01Ljk3My0uOTYxLTguMTEtMy4xMDdaIi8%2BPC9zdmc%2B)](https://brainmade.org/)

## Developer Onboarding

Install `prek` ([docs](https://prek.j178.dev/)).

```bash
cargo install --locked prek
```

Install a git commit hook to run `prek` hooks

```bash
prek install
```

Try running `prek` hooks manually

```bash
prek run
```

You may need to install some tools that `prek` runs, for example

```bash
cargo install --locked cargo-deny
cargo install --locked cargo-machete
cargo install --locked cargo-sort
```

### DURU CLI

Try running `duru-cli` from the repo root directory as follows

```bash
cargo r -p duru-cli -- --help
```

We also provide a cargo alias to run this more easily

```bash
cargo duru --help
```

For active use, it is recommended to use the completions subcommand to generate completions for your shell. Example shown for zsh - requires `~/.zsh/completions` to be on `fpath` in your `.zshrc` before running `compinit`. Note this will only return completions if your shell has an alias `duru-cli`, or you run the `duru-cli` binary directly.

```bash
cargo duru completions zsh > ~/.zsh/completions/_duru-cli
```

For active development, it may also be useful to shim the `duru-cli` binary such that a `cargo build` occurs before running the binary. Example shown for zsh. Keeping the shim named the same as the binary enables the completions to work for the shim.

```bash
duru-cli() {
    REPO_ROOT="$HOME/git/delay-us-repay-us"
    cargo build --manifest-path="$REPO_ROOT/crates/duru-cli/Cargo.toml" || return
    "$REPO_ROOT/target/debug/duru-cli" "$@"
}
```
