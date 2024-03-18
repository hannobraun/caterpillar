# Concept

## About

This is the planning document for the next prototype. I want to use this to
clarify my own thinking, by way of writing. If it has the side effect of
communicating my ideas to someone else, all the better.

The goal of this next prototype is to figure out a better approach to building
Caterpillar, based on what I've learned so far with previous prototypes.

It might be successful or not. If it is, it can be the basis for further
development. If it's not, it will yield more insights, and show more avenues for
further exploration in future prototypes.

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

And since making the language productive has turned out to be the hard part,
getting there early means that there won't be lots of code already built for
other aspects, that slow down any work on that.

I will now present my ideas for getting there.

### Focus on a single use case

The previous prototype focused on a single use case, and I think that was a good
idea. The question is, which use case should that be, and will it bring its own
complications.

I believe browser games are the best options for this. Games are a nice use case
for an interactive language, and working on them is fun. Running in the browser
makes them easy to deploy, which scores points in the "lack of benefit"
department.

Since I have a good handle on WebAssembly now, setting that up won't be a
problem. And with [Trunk](https://trunkrs.dev/) (which I used successfully in
earlier prototypes), the development experience is pleasant and reliable.

Since I'm ignoring interactivity for now (see below), there won't be any
additional complications due to needing a custom server, communication with that
server, etc.

Here are some alternatives I've considered:

- Desktop games:
  - Those seem easier to handle, but given my experience and available tools,
    I've come to the conclusion that the difficulty would be about the same.
  - They are hard to deploy. Cross-compiling binaries to send to your friends in
    a pain.
  - They would make things easier once interactivity is added to the mix
    (everything can still be a single application). But at that point, I can
    decide to first port what I have to desktop, and I can't think of a reason
    why that wouldn't be straight-forward.
- Text-based games:
  - I enjoy writing fiction, and I don't do enough of it. But I worry that I'll
    run out of ideas for game mechanics pretty soon, and then I'll have a
    creative writing project, not a programming language project.
  - A terminal-based interface would be simple, but I've had the experience in
    other projects, that this simplicity is actually deceptive. Because terminal
    also come with lots of legacy cruft, and once you want to do certain things,
    like basic input that's not just writing text and hitting enter, you either
    need specific modern terminals (not deployable), or you start building your
    own text rendering (not simple).
- Microcontrollers:
  - This is a path I'd definitely like to take with Caterpillar at some point,
    so it seems attractive to do so right now.
  - But it's also less accessible. I can come up with new game ideas and start
    applying Caterpillar to those immediately, while most microcontroller
    projects require components that I need to research, order, and wire up,
    which distracts from Caterpillar's part in the project.
  - Also, when it comes to debugging (see below), this would be more
    complicated. With browser games, I have a browser right there and can
    display whatever I want in it. With microcontrollers, I'd have to learn more
    about their debugging capabilities and related tooling first, which presents
    an additional hurdle.

### Define the language within a host language

Programming languages typically have syntax, and infrastructure to deal with
that syntax. Code to load from files, code to watch those files for changes, a
tokenizer, a parser, ... all of this is tedious and can get in the way while
iterating on other things.

Syntax is also a solved problem. After eight prototypes, I have no doubt that I
can handle this, and it won't be a problem. So let's ignore it and go right for
the semantics.

This is my idea: Instead of writing an interpreter in Rust, I will define the
language in Rust itself. The data structures that would otherwise be produced by
a pipeline reading code from text files, I will instead produce using a Rust
API.

This will allow me to focus on the other aspects of the language that are not a
solved problem yet. Syntax can be added later, once I feel that the other
aspects of the language are under control.

### Extremely simple language

Creating a simple language has always been the goal, but I've not always done it
as minimally as I could have. I still want to make it stack-based, but I will
try again without local variables. This hurts the "lack of understanding"
problem, but I've decided to lean heavily on tooling this time (see below), so
I'm hoping that will be enough.

Whereas previous prototypes were dynamically typed, I want to go completely
untyped this time. Every value is a 32-bit integer, so the abstraction level is
more like what you'd expect from an Assembler or a Forth.

### Building a debugger immediately

Since the lack of understanding has been such a massive problem in every
prototype so far, I want to lean heavily on tools to fix that, this time around.

Formatting infrastructure for printing debug messages will certainly help, and
might become an area of focus for this prototype, but I want to go one step
further: a full debugger, from the get go.

Providing full insight into everything that's going on with the language runtime
will be an immediate concern. Since I want to go for browser games as a use
case, I intend to display this information in the browser, right next to the
game graphics.

Manipulating this information by stepping through the code, and with other
typical debugger functionality, will come right next after displaying it. I want
"writing a debugger" to just never be a concern I have with this prototype. It
will be there from the start.

Later on, this can be moved to another browser window (even running on another
computer), as the infrastructure around the language becomes more sophisticated.
But initially, having a non-optional debugger, right where the game is being
played, will be enough.

It's not unlikely that this experiment will the thing that derails the prototype
this time, as I can foresee this being a huge amount of work, long before it
yields any real results. But I'll risk it. I want to learn.

### Postponing interactivity

Interactivity is still part of the core premise of what Caterpillar will be
about, but I feel like I've got a good handle on the technical side of it by
now. I'm confident that I can retrofit it, once the prototype reaches the
functionality outlined here.

Creating a productive language has turned out to be the hard part. I'm sure
there are still many challenges to make the language interactive, but as far as
the previous prototypes are concerned, it's actually one of the
better-understood aspects. Let's worry about other things for now.
