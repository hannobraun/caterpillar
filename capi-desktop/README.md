# Caterpillar - Desktop Platform

A Caterpillar platform for desktop operating systems. Caterpillar is developed
(and tested) mostly on Linux. But in principle, this should work at least on
Linux, macOS, and Windows. Support for other platforms is
[limited by wgpu](https://github.com/gfx-rs/wgpu#supported-platforms).

To try this out, run the following from the repository root:

```shell
cargo run -- examples/hello.capi run
```

This runs the "Hello, world!" example, but you can also run
[any other example](../examples/) this way.

If you want to install the Caterpillar Desktop Platform on your system, you're
going to need a
[working Rust installation](https://www.rust-lang.org/tools/install). Once you
have that, run this:

```shell
cargo install \
  --force \
  --git https://github.com/hannobraun/caterpillar.git capi-desktop
```

You don't need the `--force` for the initial install, but it's going to be
required to overwrite your existing version, if you upgrade later.

If the installation was successful, you can run any Caterpillar script like
this:

```shell
capi path/to/script.capi run
```

If a script defines tests, you can run those tests like this:

```shell
capi path/to/script.capi test
```

For more information about Caterpillar, check out the
[top-level README](../README.md).
