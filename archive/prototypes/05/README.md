# Caterpillar Prototype

## About

This is (one of many) prototypes for the Caterpillar programming language. Check
out the top-level README in this repository for more info.

This iteration of Caterpillar is a variation of an [earlier prototype](../cp3/),
which it is identical to in objective and scope. It deviates in the architecture
of the language implementation.

## Objective

The objective of this experiment is to implement an interactive programming
system that contains a test framework. Having one in place from the beginning
should make it practical to build up language features that support the
programmer in writing correct code.

It should be possible to define test cases, as well as free functions that can
be called by one or more test cases. Ideally, the system would know exactly
which tests to re-run in reaction to a given change.

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
- **Web-based UI:** Browsers are the most universal runtime that we have. Having
  the development environment running there will provide a nice baseline of
  platform support.

## Status

The objective has been reached. Work is continuing in a
[follow-up prototype](../cp6/).
