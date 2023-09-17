# Caterpillar

## About

Caterpillar is an experimental programming language that aims to explore
interactive programming.

```
"Hello, world!" print
```

## Status

After [many experiments](archive/), I've decided to create a language that is
practical, but as simple as it can be, to enable exploration of the core
premise: interactivity.

Here's what that means in practice:

- Developing with the language is **fully interactive**. You manipulate a
  running program, instead of having to restart to test your changes. This is
  the core concept that is being explored.
- All **code is stored in text files**. Previous prototypes followed the
  approach of using a parsed/analyzed form of the code its canonical
  representation. This has many advantages that I'd like to explore in the
  future, but requires too much custom infrastructure to be viable.
- A language that is **unapologetically dynamic**. Interpreted, dynamically
  typed, and no design considerations made to change any of that. At some point,
  I'd like to create a fully interactive language with static typing and zero
  runtime overhead (beyond what is required for interactivity). For now, even
  thinking about this would be too difficult.
- The language is **purely functional, concatenative, and stack-based**. Besides
  providing desirable properties, this is also as simple (and easy to implement)
  as it gets.
- Memory management is yet to be determined. I expect to get away with just the
  stack for a while. After that, I'll probably go for reference counting, or
  (runtime-managed) affine or linear types.

Caterpillar is still in a state of early exploration, and barely usable.

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
