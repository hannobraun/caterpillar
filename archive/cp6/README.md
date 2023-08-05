# Caterpillar - Prototype 6

## About

This is (one of many) prototypes for the Caterpillar programming language. Check
out the top-level README in this repository for more info.

This prototype requires [`trunk`](https://crates.io/crates/trunk). Run the
prototype using the following command:

```
cd cp6
trunk serve
```

## Concept

Caterpillar is an experimental programming language that is inspired by two
insights:

1. Interactive programming is an under-explored area of software development.
2. By combining pure functions with affine or linear types we can get memory
   safety without runtime overhead, but without the complexity of Rust's borrow
   checker.

I'm also exploring other aspects of programming language design, but those two
are at the core of what this project is about.

Caterpillar aims to be a language with the following attributes:

- **Interactive:** During development, the programmer always manipulates a
  running program. This leads to much shorter feedback loops and less need for
  emulating the computer in your head while you program.
- **Concatenative, stack-based:** I don't want to support prefix _and_ infix
  _and_ postfix notation all at once, in different variations, as languages
  these days tend to do. Postfix notation seems to have the nicest attributes
  between all of them, so that's what I'm going with.
- **Purely functional:** Immutability combined with affine or linear types
  should allow for (memory-safe) automatic memory management without any runtime
  overhead, but also without most of the complexity that Rust employs to achieve
  the same thing.
- **Sandboxed:** Any piece of Caterpillar code can only access what is being
  provided to it via dependency injection.
- **Embeddable:** All I/O primitives are provided via an external platform, and
  the language itself doesn't care whether that's some kind of application it
  extends, or a bare-metal microcontroller. (Inspired by
  [Roc](https://www.roc-lang.org/).)

This list is not complete, and my vision keeps developing as I experiment.

## Objective

Create a basic web-based programming environment for Caterpillar, with the main
feature being a test runner. Basically, port the
[previous prototype](../archive/cp5/) to the web.

Compared to the previous prototype, it would be nice to have a more powerful UI
that goes beyond a simple command-line interface.

## Stretch Goals

It's quite possible that I'll abandon this prototype once the objective is
reached (or even before!) to move on to another prototype that focuses on a
different aspect of Caterpillar. If, however, I decide to stay with this
prototype, here are some stretch goals I could work on:

- **Persistence:** Manipulating a running program, loading new code into it, is
  fine. But it's even better, if you can store that code you loaded into it.
  Preferably in a form that works with Git, so someone else can start their own
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

The objective has been reached and this prototype is completed. It has been
succeeded by a [follow-up prototype](../../cp7/) which aims to take a subset of
the ideas explored here to a usable state.
