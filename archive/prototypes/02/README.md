# Caterpillar

## About

An experimental programming language, to explore interactive programming and
other aspects of the programming language design space.

Caterpillar builds on the results of an [earlier experiment](../cp1/) of the
same name. Its predecessor had similar goals, but a different starting point. It
became hard to work with, when the code written in Caterpillar reached a level
of complexity that the language features and debugging infrastructure couldn't
support (which was surprisingly soon).

## Status

This prototype is abandoned. The approach of building a test framework first
worked out nicely, but the user interface was still too ambitious and distracted
from the language design aspect. [There is a more focused successor.](../cp3/).

## Concept

Caterpillar is a language with the following attributes:

- **Interactive:** During development, the programmer always manipulates a
  running program, which leads to much shorter feedback loops and less emulating
  the computer in your head.
- **Concatenative, stack-based:** I don't want to support prefix _and_ infix
  _and_ postfix notation all at once, in various variations, as languages these
  days tend to do. Postfix notation seems to have the nicest attributes between
  all of them, so that's what I'm going with.
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
programmer in writing correct code. The previous prototype started with making
the language as simple as possible, which made it difficult to use.

In addition, as a vehicle to try out the the language and interactive
programming system with, a test framework might be even easier than the
1-dimensional Game of Life variant of the previous prototype.

It should be possible to define test cases, as well as free functions that can
be called by one or more test cases. Ideally, the system would know exactly
which tests to re-run in reaction to a given change.

## Stretch Goals

It's quite possible that I'll abandon this prototype once the objective is
reached (or even before!) to move on to another prototype that focuses on a
different aspect of Caterpillar. If, however, I decide to stay with this
prototype, here are some stretch goals I could work on:

- **Editor:** Replace the terminal-like interface with a full-blown text-based
  editor.
- **Persistence:** Manipulating a running program, loading new code into it, is
  fine. But it's even better, if you can store that code you loaded into it,
  preferably in a form that works with Git, so someone else can start their own
  process with the same code.
- **Content-addressable functions:** See
  [Unison](https://www.unison-lang.org/learn/the-big-idea/).
- **Static typing:** Implement a static type system. I don't have many plans or
  ideas here. I figure I'd start with something very simple and see where that
  leads.
- **Affine/linear types:** Implement affine or linear types.
- **Self-hosting:** Split the system into an interface and a language runtime,
  run that language runtime in WebAssembly, implement a Caterpillar to WASM
  compiler in Caterpillar, then re-implement the language runtime in
  Caterpillar.

There are many other things I can imagine, but let's leave it at that for now. I
wouldn't be surprised if this prototype ran its course and I felt the need to
start again with a modified approach, even before I make it to stretch goals.
