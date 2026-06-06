# AMD Core Tools

AMD Core Tools is a Rust workspace for experimenting with AMD CPU feature detection and low-level processor tooling.

The top-level binary currently reports selected AMD CPU capabilities, attempts a hardware RNG read, and initializes the x87 FPU from user space. Kernel-level examples are kept commented in `src/main.rs` because they require ring 0 privileges.

## Workspace

- `amd-cpuid` - CPUID and AMD feature detection helpers.
- `amd-rng` - Hardware random number helpers.
- `amd-sev` - Secure Encrypted Virtualization helpers.
- `amd-virt` - AMD virtualization helpers.
- `amd-perf` - Performance feature helpers.
- `amd-exec-protect` - Execution protection helpers.
- `amd-legacy` - Legacy processor initialization helpers.

## Usage

```sh
cargo run
```

## Development

```sh
cargo test
cargo fmt
```

Some low-level operations in this workspace require AMD hardware and elevated privileges. Review the comments in the relevant modules before enabling privileged code paths.
