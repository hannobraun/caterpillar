# Caterpillar Prototype

## About

This is (one of many) prototypes for the Caterpillar programming language. Check
out the top-level README in this repository for more info.

This prototype requires [`trunk`](https://crates.io/crates/trunk). Run the
prototype using the following command:

```
cd cp6
trunk serve
```

## Objective

Create a basic web-based programming environment for Caterpillar, with the main
feature being a test runner. Basically, port the [previous prototype](../cp5/)
to the web.

Compared to the previous prototype, it would be nice to have a more powerful UI
that goes beyond a simple command-line interface.

## Stretch Goals

It's quite possible that I'll abandon this prototype once the objective is
reached (or even before!) to move on to another prototype that focuses on a
different aspect of Caterpillar. If, however, I decide to stay with this
prototype, here are some stretch goals I could work on:

- **Persistence:** Manipulating a running program, loading new code into it, is
  fine. But it's even better, if you can store that code you loaded into it,
  preferably in a form that works with Git, so someone else can start their own
  process with the same code.
- **Affine/linear types:** Implement affine or linear types.
- **Compile-time metaprogramming:** I'd like to experiment with writing a static
  type system for Caterpillar in Caterpillar itself.
- **Self-hosting:** Split the system into an interface and a language runtime,
  run that language runtime in WebAssembly, implement a Caterpillar to WASM
  compiler in Caterpillar, then re-implement the language runtime in
  Caterpillar.
- **Content-addressable functions:** See
  [Unison](https://www.unison-lang.org/learn/the-big-idea/).

## Status

In progress.
