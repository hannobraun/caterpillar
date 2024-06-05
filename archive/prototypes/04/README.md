# Caterpillar

## About

An experimental programming language that is inspired by two insights:

1. Interactive programming is an under-explored area of software development.
2. By combining pure functions with affine or linear types, we can get memory
   safety without runtime overhead, without the complexity of Rust's borrow
   checker.

I'm also exploring other aspects of programming language design, but those two
are at the core of what this project is about.

This iteration of Caterpillar is a variation of an [earlier prototype](../cp3/),
which it is identical to in objective and scope. It deviates in the architecture
of the language implementation though, experimenting with a piecemeal and
pull-based (i.e. async) approach to the execution pipeline.

## Status

Abandoned. The new execution pipeline architecture didn't work out like I hoped.
I've come up with a different approach which I'll try in a new prototype.

## Concept

Caterpillar is a language with the following attributes:

- **Interactive:** During development, the programmer always manipulates a
  running program. This leads to much shorter feedback loops and less need to
  emulating the computer in your head.
- **Concatenative, stack-based:** I don't want to support prefix _and_ infix
  _and_ postfix notation all at once, in different variations, as languages
  these days tend to do. Postfix notation seems to have the nicest attributes
  between all of them, so that's what I'm going with.
- **Purely functional:** Immutability combined with affine or linear types
  should allow for (memory-safe) automatic memory management without any runtime
  overhead, but also without most of the complexity that Rust employs to achieve
  the same thing.
- **Interpreted, dynamically typed:** This is actually the opposite of what I'd
  like to do eventually, but it's where I'll start. It just makes the
  implementation of the language much easier.

The language is run by an interpreter which is implemented in a host language
(Rust). That interpreter provides specialized I/O primitives to allow for some
experimentation with the language (see next section).

The interpreter has a simple terminal-like interface, also implemented in Rust
and initially living in the same monolithic process. All of this is
terminal-based, to keep complexity low.

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

## Long-term goals

I have some use cases in mind, that would be perfect applications of interactive
programming. These are unlikely to be implemented as part of this prototype, but
might be the topic of future prototypes:

- **Synthesizer:** Define a code-based synthesizer that can be programmed in
  Caterpillar.
- **Fantasy Console:** Define a
  [fantasy video game console](https://en.wikipedia.org/wiki/Fantasy_video_game_console)
  that can be programmed in Caterpillar.
- **Embedded Runtime:** Direct interaction with and development of a program
  running on a microcontroller.
