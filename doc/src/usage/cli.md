# CLI Reference

This page covers a few handy `fuzzamoto-cli` workflows. The CLI is built from the
`fuzzamoto-cli` crate in this repository and provides utilities for working with
IR corpora, scenarios, and coverage reports.

## Generate a sample IR program

Most commands operate on IR programs (`.ir` postcard files). You can generate a
single sample program using the IR generators:

```bash
cargo run -p fuzzamoto-cli -- ir generate \
  --context /path/to/share/dump/ir.context \
  --output /tmp/ir-samples \
  --programs 1 --iterations 8
```

This writes a single `*.ir` file under `/tmp/ir-samples`.

## Inspect an IR program

To print the human-readable SSA form:

```bash
cargo run -p fuzzamoto-cli -- ir print /tmp/ir-samples/<file>.ir
```

Pass `--json` to emit JSON instead.

## View mutation traces

When [fuzzing with `fuzzamoto-libafl`](https://dergoegge.github.io/fuzzamoto/usage/libafl.html), every saved testcase now records the
sequence of mutators/generators that modified it. You can inspect this metadata
with `ir trace`:

```bash
cargo run -p fuzzamoto-cli -- ir trace /tmp/out/cpu_000/queue/<file>.ir
```

Sample output:

```
Mutation trace (3 entries):
[00] generator AddrRelayGenerator
[01] mutator   OperationMutator
[02] splice    CombineMutator

// Context: nodes=1 connections=8 timestamp=1296688802
v0 <- LoadConnection(5)
v1 <- LoadMsgType("tx")
...
```

Use `--quiet` to suppress printing the IR body and show only the trace.
