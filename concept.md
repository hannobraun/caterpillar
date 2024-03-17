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

And since making the language productive has turned out to be the hard part, getting there early means that there won't be lots of code already built for other aspects, that slow down any work on that.

I will now present my ideas for getting there.

### Focus on a single use case

The previous prototype focused on a single use case, and I think that was a good idea. The question is, which use case should that be, and will it bring its own complications.

I believe browser games are the best options for this. Games are a nice use case for an interactive language, and working on them is fun. Running in the browser makes them easy to deploy, which scores points in the "lack of benefit" department.

Since I have a good handle on WebAssembly now, setting that up won't be a problem. And with [Trunk](https://trunkrs.dev/) (which I used successfully in earlier prototypes), the development experience is pleasant and reliable.

Since I'm ignoring interactivity for now (see below), there won't be any additional complications due to needing a custom server, communication with that server, etc.

Here are some alternatives I've considered:

- Desktop games:
  - Those seem easier to handle, but given my experience and available tools, I've come to the conclusion that the difficulty would be about the same.
  - They are hard to deploy. Cross-compiling binaries to send to your friends in a pain.
  - They would make things easier once interactivity is added to the mix (everything can still be a single application). But at that point, I can decide to first port what I have to desktop, and I can't think of a reason why that wouldn't be straight-forward.
- Text-based games:
  - I enjoy writing fiction, and I don't do enough of it. But I worry that I'll run out of ideas for game mechanics pretty soon, and then I'll have a creative writing project, not a programming language project.
  - A terminal-based interface would be simple, but I've had the experience in other projects, that this simplicity is actually deceptive. Because terminal also come with lots of legacy cruft, and once you want to do certain things, like basic input that's not just writing text and hitting enter, you either need specific modern terminals (not deployable), or you start building your own text rendering (not simple).
- Microcontrollers:
  - This is a path I'd definitely like to take with Caterpillar at some point, so it seems attractive to do so right now.
  - But it's also less accessible. I can come up with new game ideas and start applying Caterpillar to those immediately, while most microcontroller projects require components that I need to research, order, and wire up, which distracts from Caterpillar's part in the project.
  - Also, when it comes to debugging (see below), this would be more complicated. With browser games, I have a browser right there and can display whatever I want in it. With microcontrollers, I'd have to learn more about their debugging capabilities and related tooling first, which presents an additional hurdle.


### Define the language within a host language

Programming languages typically have syntax, and infrastructure to deal with that syntax. Code to load from files, code to watch those files for changes, a tokenizer, a parser, ... all of this is tedious and can get in the way while iterating on other things.

Syntax is also a solved problem. After eight prototypes, I have no doubt that I can handle this, and it won't be a problem. So let's ignore it and go right for the semantics.

This is my idea: Instead of writing an interpreter in Rust, I will define the language in Rust itself. The data structures that would otherwise be produced by a pipeline reading code from text files, I will instead produce using a Rust API.

This will allow me to focus on the other aspects of the language that are not a solved problem yet. Syntax can be added later, once I feel that the other aspects of the language are under control.

### Radically reduced scope

- just the inside of functions
- functions themselves can be defined in the host language
- just make a language that can do the actual stuff; would be a win already

### Extremely simple language

- still stack-based
  - it _is_ the simplest way to create a language
  - and postfix _has_ advantages vs the other fixes
  - slightly offset by lack of understanding
- untyped
  - all values are 32-bit words
  - all functions are interpretations of those

### Building the debugger immediately

- providing full insight into what's going on is an immediate concern
- manipulating it, like stepping through code, comes right after
- "writing a debugger" should never be a concern; it should just be there

### Postponing interactivity

- interactivity is still part of the core premise
- but I feel like I have a pretty good handle on the technical side now
- I feel confident that I can retrofit it easily
- so first, let's create a productive language, which is the hard part now
- make it interactive as soon as it's productive
