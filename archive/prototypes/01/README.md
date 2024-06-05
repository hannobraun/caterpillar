# Caterpillar

An experimental programming language, to explore interactive programming.

Caterpillar builds on the results of an [earlier experiment](../cp0/) of the
same name. Its predecessor used a broad, exploratory approach that in the end
got bogged down in too many details. This new experiment is more focused on the
language itself, trying to keep all other aspects as simple as possible.

## Status

This prototype has been abandoned. [There was a successor.](../cp2/)

## Concept

Caterpillar is a language with the following attributes:

- **Interactive:** During development, the programmer manipulates a running
  program.
- **Concatenative, stack-based:** Among many other nice attributes,
  concatenative languages provide a certain simplicity, making implementation
  easier.
- **Purely functional:** This is a design space that I'd like to explore, mostly
  because I have vague notions that the restrictions that purely functional
  programming brings, can make many things easier.
- **Homoiconic:** Making code easily processable in the language itself should
  make interactivity easier. Also, once you have a language with a simple
  syntax, there seems to be little reason not to make it homoiconic.
- **Interpreted, dynamically typed:** These are actually the opposite of what
  I'd like to do eventually. This is simply to save work.

The language is run by an interpreter which is implemented in a host language
(Rust). That interpreter provides specialized I/O primitives to allow for some
experimentation with the language (see next section).

The interpreter has a simple terminal-like interface, also implemented in Rust
and initially living in the same monolithic process. All of this is text-based,
to keep complexity low.

## Objective

The objective of this experiment is to implement an interpreter for Caterpillar
and use it to implement a
[one-dimensional Game of Life](http://jonmillen.com/1dlife/index.html) variant
in Caterpillar. This should be relatively simple, both in terms of the
infrastructure it requires (graphics, for example; text-based should be fine)
and in terms of the actual logic.

At the same time, it should be substantial enough to provide insights into the
language it is implemented with.

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
- **Self-hosting:** Split the system into an interface and a language runtime,
  run that language runtime in WebAssembly, implement a Caterpillar to WASM
  compiler in Caterpillar, then re-implement the language runtime in
  Caterpillar.

There are many other things I can imagine, but let's leave it at that for now. I
wouldn't be surprised if this prototype ran its course and I felt the need to
start again with a modified approach, even before I make it to stretch goals.
