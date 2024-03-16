# Caterpillar - Web Platform

A browser-based Caterpillar platform.

To try this out, run the following command from the repository root (required
[Trunk](https://trunkrs.dev/)):

```shell
trunk serve
```

By default, the runner bundles a simple "Hello, world!" script. To bundle a
different script, pass its path at build time using an environment variable:

```shell
CAPI_SCRIPT=../path/to/script.capi trunk serve
```

The path must be relative to the `capi-web/` directory. If you're building
without Trunk, you can use the environment variable with `cargo build` (or any
Cargo command) instead.

For more information about Caterpillar, check out the
[top-level README](../README.md).
