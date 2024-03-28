# Caterpillar - Prototype 9

## About

This is a prototype (of many) for the Caterpillar programming language. Check
out the top-level README for more information.

All of Caterpillar's previous prototypes have failed in some way, but all of
which have done so in a way that provided insights. These are the main takeaways
these previous prototypes have provided:

- Creating a programming language is not hard.
- Making it productive very much is.
- The top problem is to understand what the language is actually doing.
- The lack of a clear use case invites meandering and uncertainty.
- The lack of a clear benefit has the same effect.
- The full infrastructure required for a language inhibits agility.
- Syntax, interactivity, and other aspects, are mostly a solved problem.

The purpose of this prototype is to take these insights and explore a different
approach to building the language. If it is successful, it can serve as the
basis for further development. If it is not, it will add more insights to this
list, which the next prototype can incorporate.

## Status

This prototype has been archived, in favor of a new prototype with a very
similar approach, but reduced scope. Here's what led to this decision:

- I had initially hoped to use [Trunk] to build and package the Caterpillar
  runtime, but then decided not to do that. It would have required buying into
  `wasm-bindgen`, which I didn't want to do. I wanted a simpler interface
  between the runtime and the JavaScript host, figuring this would lead to less
  complication. (The description below already reflects this, even though I
  didn't go into any detail.)
- Developing my own build tool was indeed a limited effort, and once I had that
  figured out, I was very happy with how it worked. But when I started to
  closely consider the debugger, I found problems with my approach. I would have
  preferred to build the debugger with [Leptos], as I figured that a Rust-based
  solution would present the fewest hurdles in displaying my Rust-based data
  from the Caterpillar runtime.
- I saw the following options for realizing that:
  - Putting everything into a single WebAssembly module. Leptos requires Trunk,
    `wasm-bindgen`, and all the stuff I was trying to avoid, so I wasn't sure
    about that option. It also would have meant a lot of fiddling with tasks,
    channels, etc, to integrate the Catterpillar runtime with Leptops. (I've had
    this experience with Sycamore, in an earlier prototype, and believe working
    with Leptos would be similar in that regard.).
  - Loading a second WebAssembly module, this one Leptos-based, into the same
    website. But then I would have had to figure out how to build and deploy
    that during development, which would have likely required a significant
    investment into my builder, reinventing parts of Trunk. Or possibly dealing
    with advanced and unfamiliar tooling from the JavaScript world. Also, as I'm
    writing this, I realize the communication between the two WebAssembly
    modules would have been an additional pain.
  - Putting the debugger in a separate site, building that with Trunk and
    Leptos. But then the communication would have been an even bigger pain, with
    my builder having to become a full-blown development server that can serve
    both sites and intermediate between them. I think this is the direction that
    the project needs to go eventually, but that would have front-loaded a lot
    of effort that I wanted to avoid with this prototype.
- As the web-based nature of this prototype turned into a substantial problem, I
  realized that I was still far away from anything that I would like to deploy
  to users. Which ironically meant that the only reason I wanted to make this
  prototype web-based in the first place was irrelevant, for the time being.

So I decided to correct course and start a new prototype that is even more
streamlined, with a similar scope as this one, but with the runtime running on
the desktop-based development system.

[Trunk]: https://trunkrs.dev/
[Leptos]: https://leptos.dev/

## Approach

### Productive use, as early as possible

The language doesn't have to be the best at anything. It doesn't even have to be
good. But it needs to be a practical option to develop things that I care about
and that are fun.

If I can manage that, then further development will merely be an issue of
iterating on a working state, continuously improving it. Something I have proven
to be good at.

And since making a productive language has turned out to be the core challenge,
getting there early means I can iterate without being weighed down by
infrastructure for aspects that I consider solved problems. ("Solved", to the
extent that I've explored them. Beyond this prototype, there is certainly more
to figure out in these areas.)

### A single use case

To provide focus, this prototype will focus on a single use case, the
development of browser games. I believe this is a good fit, for the following
reasons:

- Games are a good use case for an interactive language.
- Working on them is fun.
- They are easy to deploy. (Everyone has a browser, and web servers are easy.)
- Thanks to my previous experiences, I have a good handle on WebAssembly.

The main drawback is that I need some kind of build system, but that is a
limited effort, and separate from the development of the language.

Other than that, there shouldn't be any drawbacks. Given the other limitations
laid out in this document, a server and communication with that shouldn't be
required for this prototype.

### No syntax

Syntax is one of those things that I consider solved (to the extent that I've
explored it), but that requires lots of infrastructure: Loading from files,
watching those files for changes, tokenizing, parsing, etc. All of this gets in
the way while iterating on other things.

I'm going to avoid this problem completely. This prototype won't have any
syntax. Instead, the data structures that make up the runtime representation of
the language will be produced directly by calling Rust APIs.

### Simple semantics

I will keep doing what made previous iterations of the language simple, while
going beyond that where I can:

- It will stay functional and stack-based.
- No local variables.
- Completely untyped. Every value is a 32-bit integer.

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

I intend to create the interface for this in the browser and display it right
next to the game.

### Postponing interactivity

Interactivity is still part of the core premise of what Caterpillar will be
about, but I'm confident that I can retrofit it, once the prototype reaches its
objectives. It's one of the better-understood aspects of Caterpillar. Let's
worry about something else for now.

## Next steps

Once the level of functionality outlined here has been achieved, and provided
the approach explored by this prototype turns out viable, it can turn into the
basis for further development.

What exactly the next steps will be remains to be seen, but here are some
possibilities:

- Higher level of abstraction: Expanding the language semantics, to make the
  language more productive.
- Extract the debugger as a dedicated web app: This implies communication
  between runtime and debugger through a backend.
- Interactivity: This can be realized by defining the language in a separate
  Rust library, and sending the runtime representation into the language
  runtime, through a backend.
- Syntax: Define a syntax for the language, and a pipeline to turn it into the
  runtime representation.
