# Caterpillar

## About

Caterpillar is an experimental programming language that is inspired by two
insights:

1. Interactive programming is an under-explored area of software development.
2. By combining pure functions with affine or linear types we can get memory
   safety without runtime overhead, but without the complexity of Rust's borrow
   checker.

I'm also exploring other aspects of programming language design, but those two
are at the core of what this project is about.

## Status & Organization

Caterpillar is still in a state of early exploration. It's far off from being
usable for anything yet.

The project is organized as a series of prototypes, each labeled `cp<n>`, where
`<n>` is a sequential integer. Check out the top-level directory for currently
active prototypes, and the [`archive/`](archive/) directory for completed or
abandoned ones.

## Concept

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
  extends, or a bare-metal microcontroller. (Inpsired by
  [Roc](https://www.roc-lang.org/).)

This list is not complete, and my vision keeps developing as I experiment.

## Long-term goals

I have some use cases in mind, that I think would be perfect applications of
interactive programming:

- **Synthesizer:** Define a code-based synthesizer that can be programmed in
  Caterpillar.
- **Fantasy Console:** Define a
  [fantasy video game console](https://en.wikipedia.org/wiki/Fantasy_video_game_console)
  that can be programmed in Caterpillar.
- **Embedded Runtime:** Direct interaction with and development of a program
  running on a microcontroller.

## Acknowledgements

Thanks go to [Martin Dederer](https://github.com/martindederer) for suggesting
the name!

## License

This project is open source, licensed under the terms of the
[Zero Clause BSD License] (0BSD, for short). This basically means you can do
anything with it, without any restrictions, but you can't hold the authors
liable for problems.

See [LICENSE.md] for full details.

[Zero Clause BSD License]: https://opensource.org/licenses/0BSD
[LICENSE.md]: LICENSE.md
