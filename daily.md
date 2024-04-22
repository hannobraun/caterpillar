# Daily Thoughts

These are my daily thoughts on Caterpillar. If you have any questions, comments,
or feedback, please [get in touch](mailto:hello@hannobraun.com).

## 2024-04-22

If Caterpillar's interactivity can be more effective if the application is built
with an Event Sourcing architecture, and the Caterpillar language and/or
standard library have features to support that, then Event Sourcing might become
a kind of default architecture for Caterpillar programs. Like the actor model is
to Erlang, or the Elm Architecture is to Elm.

To an extent, that's not bad. Event sourcing is great, and I've often wished for
more applications to use it. If your data is important, then you absolutely
should have a way to inspect and undo all changes to it. My calendar app
certainly doesn't work that way, and I've suffered from that often.

But I worry that such a focus would create a perception of Caterpillar as a
specialized language, when it's intended to be general-purpose. Maybe it's fine
to say "Caterpillar applications are Event Sourcing applications, and it's
better that way". Or maybe that paints Caterpillar into a corner that I don't
want it to be in. I don't know.

## 2024-04-21

Event Sourcing is a software architecture in which every change to application
state is encoded as a value, an event. These events are executed
deterministically, so you can use them to reconstruct any past state. And if you
add the required information to these events, you can also undo them, rewinding
to an earlier state.

Over the last few days, I've been talking about how to step backwards through a
Caterpillar program. If Event Sourcing is to play a role in supporting this,
then the language runtime and debugger need to understand events. Where they are
stored, how to apply them to application state, how to undo them. All of this
implies a pretty close relationship between the Caterpillar runtime and an
application.

The runtime would act more like a framework than a typical language runtime,
which certainly wouldn't be a good fit for all use cases. But thanks to the
platform concept, it could be optional. Maybe I can come up with an approach
that allows applications to easily opt into event sourcing functionality when
they want to improve their interactivity, without weighing down applications
that don't need it.

## 2024-04-20

I've been talking about logging "undo instructions" into a ring buffer, as
Caterpillar code gets executed, so you can always rewind your program to an
earlier state. But you have to limit the size of this buffer, and thus the
utility of this feature.

I think this can be solved though, by complementing this fine-grained
instruction-level approach with application-level events. Then we can take big
steps from event to event, and only use the more limited undo instruction buffer
to take closer looks in between.

This sounds like a job for Event Sourcing, but that's not an architecture you
can just implement in a language runtime. I needs to be supported on an
application or framework level. And I haven't fully figured out how that should
work.

## 2024-04-19

Yesterday, I talked about stepping backwards in a debugger. There is an inherent
overhead to this. Not only to you need to generate the instructions to do this
on the fly, you also need to store them. And computers can execute a lot of
instructions in a short time, so such an "undo instruction" buffer might get
quite long.

I'm not too worried about the performance overhead of logging the undo
instructions. For now, performance is not a priority anyway. And I have ideas on
how to alleviate, and in many cases completely eliminate, the overhead. Maybe
I'm being too optimistic, but either way, that's a topic for another day.

I'm currently more worried about the length of the undo buffer. It would need to
be limited, and on memory-restricted platforms (consider microcontrollers), it
might have to be quite short. This restricts the usefulness of this feature, but
I have ideas on how to get around that too.

## 2024-04-18

Let's talk about debuggers. I think there's a core set of features that you
would expect from one: set breakpoint, continue execution, step over this
expression, step into this function call... you know, the well-known stuff. But
what about stepping backwards?

I think to make Caterpillar truly interactive, this is a feature we need. No
more, "oops, stepped too far; guess I'll have to restart". Instead, just rewind
the program whenever you want. Maybe make a change to the code. Step forward
again to observe your change. Repeat until you're happy.

I think this might be quite straight-forward to implement: For every instruction
that gets executed, log the instructions that would undo its effect into a ring
buffer. When stepping back, execute instructions from that ring buffer. But I
think there is more to consider beyond this naive implementation, and that's
what I'd like to write about over the next few days.

## 2024-04-17

As I said yesterday, Caterpillar is currently very confusing and hard to use.
I've run into this problem with many previous prototypes. It seems you just need
a critical mass of language features that promote clarity, until you cross some
threshold where the language becomes productive.

But what if I have tooling that tells me exactly what's going on at runtime?
Will that, by itself, make an otherwise confusing language practical to use?
This is not a ridiculous thought, I think. People write non-trivial stuff in
assembly languages. Presumably, at least in some cases, with the help of
debuggers.

I'm wondering if developing a debugger first is a more practical path to
productivity, than reaching this critical mass of features. If so, I could have
a usable language from early on, and could take my time expanding it. Only
adding features incrementally, as I deem it necessary and appropriate. But I
need to figure out if this approach can work, and that's the focus of the
current prototype.

## 2024-04-16

These are some notes I made while developing a program written with the current
prototype version of Caterpillar:

![Notes for a prototype Caterpillar program, showing the full stack contents for each instruction](daily-files/2024-04-16/program-notes.png)

This was very tedious, but weirdly fun. It also contains mistakes, and even
though I found some of those while transcribing it to digital code, it didn't
end up working. (Not surprising!)

All that is to say that the language, in its current basic form, is confusing as
hell and borderline impossible to use. But I'm not fixing that. Not now. I'd
like to know what happens, if I combine this confusing language with good
tooling for understanding it.

## 2024-04-15

Writing daily is awesome. I feel myself constantly immersed in Caterpillar, the
topic I'm writing about. When I write weekly or monthly, there's always this
hurdle. It can be difficult to get into again.

Not so with daily writing! The ideas just flow, and the list of future topic
keeps growing day by day. This requires a setup to capture ideas, and I'm using
[Joplin] to do that (I sync the notes between my devices with [Syncthing]). And
since ideas come whenever, wherever, I find myself writing on my phone a lot.
Something I've never enjoyed before, but that works surprisingly well.

This makes me want a full programming environment on my phone. Even if just for
quickly demonstrating something on the go. It will be a while before I can
provide something like that for Caterpillar, but I'd love to have it some day.

[Joplin]: https://joplinapp.org/
[Syncthing]: https://syncthing.net/

## 2024-04-14

For the last few day's, I've been talking about the platform concept that I'm
stealing from Roc. I initially thought about platforms in terms of, CLI,
desktop, servers, browsers; places where you can run your applications, that
might look very different from each other. But I discovered that the platform
concept is widely applicable, and can be used in more subtle ways.

Earlier on, I talked about compile-time execution of Caterpillar code. This is a
very different environment, with different considerations. And it's not an easy
thing to do, if your language wasn't designed for it! Using Rust as an example
again, at some point it gained `const fn`. But that didn't just solve the
problem. What you can or can't do in a `const` context has been a perpetual
topic ever since, with every new release unlocking at least a few more standard
library functions.

But what if your language can do nothing by default, and gets all its
capabilities from an external platform? Then suddenly it's very easy. Because
the compiler is just another platform, which can provide you with I/O primitives
that are suitable for its context. Everything that can be powered by those I/O
primitives _just works_. And everything else can still be made to work, as long
as you manage to mock the required primitives.

## 2024-04-13

Yesterday, I talked about the platform concept that I'm stealing from [Roc].
That concept doesn't just improve portability. It's also good for security.

If every I/O primitive that a library can use must be passed as an argument,
that means you know exactly what a given library can do. It won't just access
the filesystem and read your data unexpectedly. And you can further increase
security by making access more fine-grained. Pass a limited instance of the
filesystem interface, that just allows access to the specific directory that you
expect a library to write to.

And by treating access to heap memory as an I/O primitive, as I intend to do in
Caterpillar, you can even apply this concept to memory safety.

[Roc]: https://www.roc-lang.org/

## 2024-04-12

I think it is a mistake to give a language inherent capabilities, that then
might or might not be available on the platform where you actually want to run
it. Look at Rust's standard library, for example. It's not available at all on
microcontrollers. It is available in WebAssembly, but then some things just
don't work (like `print!`) or you might get an error when using others (like
threads). And yet, it is the baseline that most Rust code is written against.

I really like what [Roc] does instead: Code inherently can't do anything, except
pure computation. If it wants to do I/O, anything that relies on platform
capabilities, it specifies the I/O primitives it needs as arguments. These I/O
primitives are provided by something called a platform, which is basically a
framework that runs your application, and you have to pass those primitives on
to the libraries you want to use.

As a result, libraries end up much more portable (in principle; I don't know,
about the specific implementation in Roc). A library can't just use the
filesystem. It can at most expect to use something that looks like a filesystem.
As long as what you give it has the correct interface, you can make that do
whatever you need. Store "files" in your microcontroller's flash memory, for
example, or in RAM.

This is a concept I'm stealing for Caterpillar.

[Roc]: https://www.roc-lang.org/

## 2024-04-11

Yesterday, I talked about how Caterpillar can be a simpler form of Rust. I also
said that it can be a more powerful one, but then glossed over that completely.
Here's one of the ways in which I'd like to make Caterpillar more powerful than
Rust.

I think that most programming languages are really two languages in one. One to
describe the runtime behavior, like defining variables and calling functions.
Another for the compile-time behavior; defining functions, the type system. What
if we used the same language for both parts? Then a macro would just be a
function. The type system would just be a library. Can this work? I don't know.
(Then again, I'm probably not the first to come up with this idea. I wouldn't be
surprised if Lisp worked exactly like this.)

Maybe it would be a total mess. But what if it works? Would it mean that we can
have an advanced type system that every competent Caterpillar developer can just
innately understand, because they can always jump to the code that does whatever
currently confuses them? Could we use Caterpillar's interactivity tooling to
debug what's happening at compile time? I don't know, but it sounds like a
promising possibility!

## 2024-04-10

My vision for Caterpillar started with interactivity, but that's not where it
stayed. I've developed many more ideas that I want to realize with this project.
And since I've been using Rust as my primary language for over 10 years, it's my
frame of reference. I can't help but think of Caterpillar as an evolution of
Rust. A simpler and more powerful form of it.

I think it's possible to get Rust's core benefit, memory safety without runtime
overhead, in a much simpler language. A functional language that does away with
mutability, hence mutable references. In fact, performance consideration that
the compiler can take care of under the hood aside, a reference is basically the
same as a clone, so I think we can avoid them completely! (This is not my own
idea. I got it from [HVM].)

A Rust without references doesn't need a borrow checker, which is one of its
most complicated parts. We can get all the benefits with just a linear type
system, but none of the other complications. I can't know for sure that this
will work out, but I'm optimistic. It's one of the many things that excite me
about Caterpillar.

[HVM]: https://github.com/HigherOrderCO/HVM

## 2024-04-09

Many years ago, I watched Bret Victor's [Inventing on Principle]. It was
extremely inspiring, but I never used that inspiration. I never created
something from it.

Years later, I saw [Stop Writing Dead Programs] by Jack Rusher, and it hit me
like a brick. It shamed me! All these years that I knew of a better approach,
and I wasn't doing anything about it. This couldn't continue. And so I started
working on Caterpillar, an interactive language.

Interactivity isn't all that Caterpillar is about. I have many more ideas, some
of which [I already wrote about][design.md]. But it's how it all started. The
first seed of this project.

[Inventing on Principle]: https://vimeo.com/906418692
[Stop Writing Dead Programs]: https://www.youtube.com/watch?v=8Ab3ArE8W3s
[design.md]: https://github.com/hannobraun/caterpillar/blob/main/design.md

## 2024-04-08

I sometimes have this tendency to overthink and overplan. Projects that should
have been quick experiments get delayed again and again, then wrapped up in a
grandiose strategy, before I finally get started. More than once, that grandiose
strategy then turned out to be a mistake. At that point, I was committed. The
project felt too important to just abandon.

I'm glad that I (maybe accidentally) chose the opposite strategy with
Caterpillar. It's been all about prototypes and learning from the start. And
I've never been shy to throw away any of those prototypes to just start fresh
with a new approach.

I'm doing a similar thing again, with this new format. Start small and, more
importantly, get started. Then learn, correct, or abandon. Whatever turns out to
be appropriate.

## 2024-04-07

I've been working on Caterpillar for over a year now, sinking countless hours
into it. So far, I've seen it as a side project. With long-term potential, yes,
but mostly as something to learn from and have fun with.

This perspective is starting to change. I'm starting to take the project more
seriously. As a result of that, I'd like to talk about it more. And I figure, if
I want to do so consistently, a daily cadence (mostly) is probably easiest to
sustain.

This is an experiment. I don't know for how long I will want to keep doing it.
And so I'm starting small, with just a file in the repository. If this turns
into something real later on, I can take it to a website, an email newsletter,
maybe social media.
