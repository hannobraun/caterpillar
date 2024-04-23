# Caterpillar

## About

The goal of the Caterpillar project is to create a programming language for
controlling computers **precisely**, **intuitively**, and **reliably**:

- **Precisely:** Provide low-level control over what the computer does, while
  enabling high-level code via zero-overhead (also called zero-cost)
  abstractions.
- **Intuitively:** Create an immediate feedback loop by allowing direct
  interaction with a running program, as opposed to having to re-compile,
  re-start, and re-find the relevant place in your program every single time.
- **Reliably:** Make whole classes of errors impossible, by using a powerful
  static type system.

Caterpillar is still early-stage and experimental. This goal has not nearly been
achieved yet. I'm incrementally approaching this long-term vision through a
series of practical prototypes with specific, short-term goals.

You can find the currently active prototype in the [`capi/` directory](capi/).
The [`archive/` directory](archive/) contains a number of previous prototypes at
various levels of completion.

## Design

I'm currently working on collecting the ideas that have been guiding me into a
rough design document: [design.md](design.md)

But please don't read too much into that. This project has already gone through
a large number of prototypes, and I'm sure it will go through more. My ideas
have changed significantly during that time, and they will keep evolving.

At some point, I hope to develop one of these prototypes into a productive
language. The only design that is really relevant is the one implemented in
that.

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
