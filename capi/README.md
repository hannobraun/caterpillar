# Caterpillar - Prototype 10

## About

This is a prototype (of many) for the Caterpillar programming language. Check
out the top-level README for more information.

All of Caterpillar's previous prototypes have failed in some way, but all have
done so in a way that provided insight. These are my main takeaways from this
process:

- Creating a programming language is easy.
- Creating a _productive_ one is very hard.
- The top problem is understanding what the language actually does at runtime.
- The lack of a clear use case invites meandering and uncertainty.
- The lack of a clear benefit has the same effect.
- The full infrastructure required for a language inhibits agility.
- Syntax, interactivity, and other aspects, are mostly solved problems.

The purpose of this prototype is to take these insights and explore a different
approach to building the language. If it is successful, it can serve as the
basis for further development. If it is not, it will add more insights to this
list, which the next prototype can incorporate.

## Status

I'm just getting started. Not much to see yet.

## Approach

### Productive use, as early as possible

The language doesn't have to be the best at anything. It doesn't even have to be
good. But it needs to be a practical option to develop things that I care about
and that are fun.

If I can manage that, then further development will merely be an issue of
iterating on a working state, continuously improving it. That is something I
have proven to be good at.

And since making a productive language has turned out to be the core challenge,
getting there early means I can iterate without being weighed down by
infrastructure for aspects that I consider solved problems. ("Solved", to the
extent that I've explored them. Beyond this prototype, there is certainly more
to figure out in these areas.)

### A single use case

To provide focus, this prototype will focus on a single use case, the
development of simple 2D games. I believe this is a good fit. Games are a good
use case for an interactive language, and working on them is fun.

### No syntax

Syntax is one of those things that I consider solved (to the extent that I've
explored it), but that requires lots of infrastructure: Loading from files,
watching those files for changes, tokenizing, parsing, etc. All of this gets in
the way of iterating on more important things.

I'm going to avoid this problem completely. This prototype won't have any
syntax. Instead, the data structures that make up the runtime representation of
the language will be produced directly by calling Rust APIs.

### Simple semantics

I will keep doing what made previous iterations of the language simple, while
going beyond that where I can:

- It will stay functional and stack-based.
- No local variables.
- Completely untyped. Every value is an 8-bit integer.

Having no local variables doesn't help with understanding what's going on, but I
hope that I can more than offset this with an early focus on tooling (see
below).

Being untyped takes the language to a level of abstraction similar to an
assembler or a Forth. This won't help with productivity, but it's unlikely to be
a deal breaker. These are proven languages. This should do for a first
iteration.

### A debugger from the start

Since the lack of understanding has been such a massive problem in every
prototype so far, this time around, I want to lean heavily on tooling to fix
that.

Formatting infrastructure for printing debug messages would certainly help, and
might become an area of focus for this prototype. But I want to go one step
further: a full debugger, from the start.

Providing full insight into what's going on inside of the language runtime will
be an immediate concern. Manipulating the runtime, via stepping through code and
other typical debugger features, comes right after.

"Writing a debugger" should never become a concern. It will just be there from
the start.

### Postponing interactivity

Interactivity is still part of the core premise of what Caterpillar will be
about, but based on my experiences with previous prototypes, I'm confident that
I can retrofit it, once the prototype reaches its objectives.

## Next steps

Once the level of functionality outlined here has been achieved, and provided
the approach explored by this prototype turns out to be viable, it can turn into
the basis for further development.

What exactly the next steps will be remains to be seen, but here are some
possibilities:

- Running games in the browser: Once I have something I can make games with, I'd
  like to deploy those to players easily.
- Higher level of abstraction: Expanding the language semantics, to make the
  language more productive.
  - A prime candidate for this is making the type system dynamically typed, as
    opposed to untyped.
    - This shouldn't be too much work, and would enable additional types to be
      introduced.
    - The main thing I'm missing is function types. Those would enable a
      built-in `if` function, to replace the current placeholders.
    - It would also provide a pathway to the eventual goal of a static type
      system, through gradual typing.
  - Another good option would be to introduce variables, to replace the tedious
    stack manipulation that the language currently requires.
- Interactivity: This can be realized by defining the language in a separate
  Rust library, and sending the runtime representation into the language
  runtime.
- Syntax: Define a syntax for the language, and a pipeline to turn it into the
  runtime representation.
