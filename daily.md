# Daily Thoughts

These are my daily thoughts on Caterpillar. If you have any questions, comments,
or feedback, please [get in touch](mailto:hello@hannobraun.com).

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
