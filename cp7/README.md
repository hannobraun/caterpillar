# Caterpillar - Prototype 7

## About

This is a prototype (one of many) for the Caterpillar programming language.
Check out the top-level README in this repository for more info.

## Objective

Previous prototypes have explored various aspects of the language design, with
various levels of success. I think the project, along with my understanding of
what I'd like to achieve, have reached a point where it's viable to try and get
a less ambitious prototype to a usable level.

The objective of this prototype is to implement a practical language that allows
for exploring Caterpillar's core concept, interactive programming, while keeping
all other aspects of the language as easy as possible.

Here's what I have in mind:

- Developing with the language is **fully interactive**. You always manipulate a
  running program, instead of having to restart to test your changes. This is
  the core concept that is being explored.
- All **code is stored in text files**. Even though the previous approach,
  making the code's canonical form a parsed/analyzed representation, has many
  advantages that I'd like to explore in the future, it requires too much custom
  infrastructure to be viable. For now, I'll stick to text files, and thus get
  editors and version control for free.
- A language that is **unapologetically dynamic**. Interpreted, dynamically
  typed, and no design considerations made to change any of that. Even though
  I'd like to create a fully interactive language with static typing and zero
  runtime overhead at some point, for now even thinking about this is making
  things way too complicated.
- The language is **purely functional, concatenative, and stack-based**. Besides
  providing desirable properties, this is also as simple (and easy to implement)
  as it gets.
- Memory management is yet to be determined. I expect to get away with just
  the stack for a while. After that, I'll probably go for reference counting, or
  (runtime-managed) affine or linear types.

## Status

In progress! Basic example exist and work. The language is neither Turing-complete nor interactive yet.
