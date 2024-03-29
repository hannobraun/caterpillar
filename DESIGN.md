# Design

## About

Caterpillar has always been developed as a series of prototypes. Each of those
prototypes had a limited scope, exploring specific objectives. This document
attempts to explain the long-term vision I've been working towards, separate
from any specific prototype.

Please note that while I have been a student of programming for most of my life,
I am not an experienced language designer. These ideas present a snapshot of my
current thinking, and are sure to change as they come into contact with reality.

Please also note that this document is currently incomplete and a work in
progress. I intend to keep working on it, adding new concepts and keeping the
existing ones up-to-date.

## Concepts

### Interactive

Developing in Caterpillar will be interactive, meaning that you apply changes to
the running program directly, instead of having to restart it after every
change.

I hope that this can help close the gap between the programmer and the computer,
leading to a much more direct experience of controlling and understanding what
the computer does.

This is inspired by the work of [Bret Victor], specifically
[Inventing on Principle].

[Bret Victor]: https://worrydream.com/
[Inventing on Principle]: https://vimeo.com/906418692

### Zero overhead

My frame of reference for what a language can and should be, is [Rust] (I was
already nearing a decade of experience with it when I first started working on
Caterpillar).

That doesn't mean that Rust is the final word. I think that we can and should do
better. But it does mean, that I'd be able to do the same kinds of things with
Caterpillar that I can do with Rust.

This implies that Caterpillar needs to provide a static type system and
[zero cost abstractions], just like Rust. I do prefer the term "zero overhead"
though, as that seems clearer to me.

[Rust]: https://www.rust-lang.org/
[zero cost abstractions]: https://boats.gitlab.io/blog/post/zero-cost-abstractions/

### Functional

Caterpillar will be purely functional, without mutable state. I won't justify
this much, as there lots of material about the advantages of functional
programming already exists.

The aspect that interests me most, is that the effects that a piece of code can
have are local, and thus visible when you look at it. This can obviously help
you understand what the code does.

Languages like [Haskell] (to my understanding; not an expert) model I/O by
having the functional code only describe operations, while the runtime then
executes that description. I want to model I/O in Caterpillar differently, using
opaque values that represent the thing outside of Caterpillar that the I/O has
an effect on.

These values could represent a file, a UI, hardware peripherals, or memory. I
hope that this approach is simpler to understand and easier to implement with
zero overhead.

[Haskell]: https://www.haskell.org/

### Sandboxed

Code written in Caterpillar will be sandboxed. It won't be able to have any
effect on the world, except through the facilities that are provided to it. This
should mesh well with representing resources outside of the Caterpillar runtime
with values, as described above.

As this holds true also for top-level code as well, all applications are run by
a platform, which injects the platform-specific resources into Caterpillar,
providing the I/O primitives that the Caterpillar code can use.

I believe I mostly came up with this concept myself (inspired by decades of
prior art in research and practice, of course), but reading about [Roc] really
helped clarify my thinking, and I've adopted their nomenclature.

[Roc]: https://www.roc-lang.org/

### Single language

Many programming languages are actually at least two languages in one package.
One language for defining what's happening at runtime, concerning itself with
data and effects. The other for compile-time, concerning itself with functions
and types. (Even if a language is interpreted, this distinction is still
relevant. It's just not so clearly divided in a temporal sense.)

For Caterpillar, I want to use a single language everywhere. This means that
functions and types are just values that can be manipulated using Caterpillar's
normal facilities.

It turns out that the platform concept described above helps a lot with this, as
that makes it easy to provide different I/O primitives (like "define a function"
vs "create a file") in different contexts.

I can't credit this idea to a single source of inspiration. It just came to me
while working on Caterpillar. But I'm certain I haven't invented it. Lisp
probably works like that.

### Concatenative, stack-based

Many languages provide a mix of prefix, infix, and postfix operators (the latter
often as method syntax). I would like to avoid this kind of complexity.

If I have to choose only one of those models, postfix seems to provide the best
attributes. No operator precedence, no parentheses, and easy to parse. And it
lends itself to stack-based model evaluation, which is simple and efficient.

Please note that having a stack does not imply that there won't be local
variables and named arguments. How heavily the stack will be used in idiomatic
code will depend on the experiences we get from practical use.

Mainstream programming languages typically use a tree model for evaluation, but
that doesn't mean we have to think about this tree constantly. It is usually
used in a very limited way, for single expressions between assignments to
variables.

We can do the same in a stack-based language. [Forth] is simply one possible
expression of that concept.

[Forth]: https://en.wikipedia.org/wiki/Forth_(programming_language)

### Memory safe

As I stated above, my frame of reference for programming languages is mostly
Rust these days, which provides memory safety without runtime overhead. I think
Caterpillar can do the same, with significantly less complexity.

As the basis of that, I want to have [linear types] (which is one step beyond
Rust's affine types, but essentially very similar).

Here are the specific simplifications I think are possible to achieve over Rust:

- Without mutability, we don't need mutable references.
- Values that can't be copied don't need to be referenced either. A read-only
  reference and a copy are the same, semantically.
- Values that can't be copied can be moved into and out of functions instead,
  which is syntactically very light in a stack-based language.
- No references means no lifetimes, means no borrows, means no borrow checker.
  The compiler only needs to track if values have been moved.

What Rust can do that isn't covered in this model, is storing references in
structs and keeping them around indefinitely. I don't know whether this will be
important, and if so, what to do about it. Since the potential upside is so
significant, I'm willing to try.

Besides Rust, all of this is heavily inspired by [HVM].

[linear types]: https://en.wikipedia.org/wiki/Substructural_type_system#Linear_type_systems
[HVM]: https://github.com/HigherOrderCO/HVM

### Content-addressed definitions

Definitions in Caterpillar, functions, types, etc, will be content-addressed,
meaning they are identified by a hash of their contents.

This means that multiple versions of the same definition can exist and be
referred to at the same time. It also implies that the canonical form of code is
stored in a form that is not the same as the textual representation that a
programmer would write.

This idea is lifted from [Unison]. I won't go into justification here, as
Unison's documentation already does a great job of explaining the benefits. I
would like to expand on some points though, that I haven't seen addressed on
their side.

The straight-forward way to implement this, is to store code in some kind of
structured database. I think, but I'm not sure, that that's how Unison does
that. This has the disadvantage of either being tedious to use, or requiring
specialized tooling, or likely both.

I have come up with an alternative way: The written form of Caterpillar form
lives in regular text files, meaning no special tooling is required to edit it.
Since Caterpillar is interactive, it needs to constantly monitor those files for
changes anyway, to apply changes to the running program. When it processes these
files, it can create, update, and take into account a second set of files, which
contains the canonical representation.

Here's an example, to hopefully make that understandable:

1. The programmer writes code that calls a function: `x`
2. Caterpillar sees that no canonical representation of that code exists yet,
   and will now create it.
3. Caterpillar resolves this function call to the function `x` with hash `1`.
4. Caterpillar writes the canonical representation of the new code `x@1`.
5. A new version of function `x` with the hash `2` is defined.
6. The programmer makes changes to the original code; since the canonical
   representation exists, Caterpillar knows that the original mention of `x`
   still refers to `x@1`.
7. New mentions of `x` will resolve to `x@2`. This distinction will be displayed
   by tooling in a way that preserves clarity.
8. The programmer can upgrade the original mentions of `x` to refer to `x@2`,
   through some kind of interface (could be CLI; GUI, integrated into the IDE,
   ...). This upgrade could possibly be automatic, if `x@1` and `x@2` have
   compatible signatures.

Both the written and the canonical representation would live side-by-side in
version control.

[Unison]: https://www.unison-lang.org/

## Future Extensions

This design is, as stated in the introduction, not complete. Besides accidental
omissions, I'm actively thinking about the following topics:

- [Interaction nets]: Those could be a better basis for computation, but I need
  to study them more. This is also inspired by HVM.
- Homoiconicity: Seems like a desirable property, but it's not something I have
  thought about much, so far.
- Effects: Tracking effects is desirable, and I think the sandboxing I described
  meshes well with that. I'm not sure what, if anything, is required beyond
  that, and haven't looked into that too much. [Koka] seems like an interesting
  role model.

[Interaction nets]: https://en.wikipedia.org/wiki/Interaction_nets
[Koka]: https://koka-lang.github.io/
