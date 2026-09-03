# Workspace Commands

Create a new binary crate in the workspace:

```sh
cargo new --bin crates/catr
```

The `crates/*` workspace member pattern includes the new crate automatically.

Run a specific tool from the workspace root:

```sh
cargo run -p echor -- hello world
cargo run -p catr -- some-file.txt
```

Use `-p` to select the package to run. Everything after `--` is passed to that
tool as command-line arguments.
