# cursive-rust-workspace

[![rust](https://github.com/vpayno/cursive-rust-workspace/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/vpayno/cursive-rust-workspace/actions/workflows/rust.yml)
[![actionlint](https://github.com/vpayno/cursive-rust-workspace/actions/workflows/gh-actions.yml/badge.svg?branch=main)](https://github.com/vpayno/cursive-rust-workspace/actions/workflows/gh-actions.yml)
[![spellcheck](https://github.com/vpayno/cursive-rust-workspace/actions/workflows/spellcheck.yml/badge.svg?branch=main)](https://github.com/vpayno/cursive-rust-workspace/actions/workflows/spellcheck.yml)

Personal workspace for learning Ratatui & Rust.

## Links

- [Cursive GitHub](https://github.com/gyscos/cursive)
- [Cursive Crate](https://docs.rs/cursive/latest/cursive/)
- [Rust](https://www.rust-lang.org/)

## RunMe Playbook

This and other readme files in this repo are RunMe Plabooks.

Use this playbook step/task to update the [RunMe](https://runme.dev) cli.

If you don't have runme installed, you'll need to copy/paste the command. :)

```bash { background=false category=runme closeTerminalOnSuccess=true excludeFromRunAll=true interactive=true interpreter=bash name=setup-install-runme promptEnv=true terminalRows=10 }
go install github.com/stateful/runme/v3@v3
```

Install Playbook dependencies:

```bash { background=false category=runme closeTerminalOnSuccess=true excludeFromRunAll=true interactive=true interpreter=bash name=setup-runme-deps promptEnv=true terminalRows=10 }
go install github.com/charmbracelet/gum@latest
```

## Installing Tools

Install project dependencies.

```bash { background=false category=setup closeTerminalOnSuccess=true excludeFromRunAll=true interactive=true interpreter=bash name=setup-install-tools promptEnv=true terminalRows=10 }
# install tool dependencies

set -e
set -x

printf "\n"

: nothing to install

set +x
```
