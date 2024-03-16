# Concept

## About

This is the planning document for the next prototype. I want to use this to
clarify my own thinking, by way of writing. If it has the side effect of
communicating my ideas to someone else, all the better.

This is currently a work in progress.

## Problems

The previous prototypes have shown that creating a programming language is not
hard, but making it productive very much is. They exhibited several problems.

### Lack of understanding

Making a language understandable can be achieved in several ways:

- Features that prevent you from making mistakes, like static typing.
- Features that help you understand your mistakes, like infrastructure for
  formatting and printing values.
- Tooling that helps you understand your mistakes, like debuggers.

Most languages I know use a combination of those. Previous prototypes didn't
have enough of it, and as a result, it was hard to figure out what was going on.

In every case, implementing more of these measures would have been a big effort.

### Lack of benefit

Working on the previous prototypes was fun and taught me a lot. But otherwise,
there wasn't much point to them.

This lack of productive use invited meandering, and led to regular uncertainty
on how to proceed.

## Ideas

The core challenge of the next prototype will be to solve these problems, and
thus create a productive language, as early as possible.

If the language is productive, if it fulfills an actual purpose, then the rest
of the project will "merely" be an issue of iterating on that productive state,
further improving it.

After more than a year of working on this without getting to a something that I
actually like, that seems like a very attractive outlook.

- focus on a single purpose
  - web-based games?
  - embedded?
  - maybe text-based games?
- language is defined within a host language
  - no files, no watching files
  - not even parsing; define it as data structures that are then interpreted
  - syntax can come later
- super-deformed scope (going beyond "reduced"): just the inside of functions
  - functions themselves can be defined in the host language
  - just make a language that can do the actual stuff; would be a win already
- super-simple language
  - still stack-based
    - it _is_ the simplest way to create a language
    - and postfix _has_ advantages vs the other fixes
    - slightly offset by lack of understanding
  - untyped
    - all values are 32-bit words
    - all functions are interpretations of those
- immediate debugger
  - providing full insight into what's going on is an immediate concern
  - manipulating it, like stepping through code, comes right after
  - "writing a debugger" should never be a concern; it should just be there
- interactive-last
  - interactivity is still part of the core premise
  - but I feel like I have a pretty good handle on the technical side now
  - I feel confident that I can retrofit it easily
  - so first, let's create a productive language, which is the hard part now
  - make it interactive as soon as it's productive
