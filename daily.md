# Daily Thoughts

These are my daily thoughts on Caterpillar. If you have any questions, comments,
or feedback, please [get in touch](mailto:hello@hannobraun.com).

## 2024-05-13

Yesterday, I was musing about how I can implement breakpoints more efficiently,
by deploying new code on the fly that implements the breakpoint (or deploy code
that no longer implements it, when removing one). Turns out, perhaps
unsurprisingly, that this is how debuggers work anyway! At least where hardware
support for debugging is not available.

This technique is called "software breakpoints". I found some
[documentation on GDB internals](https://sourceware.org/gdb/wiki/Internals/Breakpoint%20Handling)
that explains it.

## 2024-05-12

The current Caterpillar prototype is focused around the question, if a debugger
is a practical solution for getting the language to a useful state relatively
quickly. A debugger needs breakpoints, and right now those are implemented by
always checking between instructions, if there's a breakpoint at the current
one.

Which is fine, for now. But it seems like the kind of thing that the language
won't be able to afford in the long-term, for performance reasons. So today, I
had this idea: If the language is interactive anyway, meaning I can deploy code
into the running program, then why don't I just deploy new code that implements
the breakpoint?

Then breakpoints would only cause overhead right where you need them. Which
would still leave the overhead of the interactive runtime, of course, but since
that is the central premise of the language, I'll have to make that work
_somehow_.

## 2024-05-11

Languages that predominantly employ postfix operators, use a stack-based
evaluation model. I talked about this yesterday. To the best of my knowledge,
this stack-based model is inherent to postfix operators. At least I can't come
up with an alternative, nor can I find examples of languages that use postfix
notation, but no stack.

What I find surprising though, is how closely these languages adhere to the
stack-based model. Languages that emphasize prefix and infix operators are not
completely based around, and allow direct manipulation of, their respective
model (although that sounds like an interesting experiment).

But languages that focus on postfix operators often display a particular purity.
Other means of defining data flow, like named variables, are either strongly
discouraged or not part of the language at all. And I think this might be to
their detriment. Employing postfix operators within a more traditional
structure, with expressions, statements, and variables, is possibly a better
direction.

## 2024-05-10

I've been talking about postfix operators for days now. If you've already been
familiar with them before, then you might be surprised that I haven't mentioned
the stack once. I did that to keep my explanations simple, but also to make a
point. We can develop an intuition for this, without constantly thinking about
the stack.

For those not familiar, the evaluation model behind postfix operators is a
stack. Let's consider a simple math example again, `1 2 +`. We can say that `1`
is a function which puts `1` on the stack. The stack contains `1` afterwards.
After `2`, the stack contains `1 2`. `+` is a function that takes two numbers
from the stack, and puts their sum back (the stack contains `3` afterwards).

The stack _is_ important. It's how this model is implemented behind the scenes,
and it's a method for understanding how it works. But my point is, we don't need
to constantly think (and talk!) about the stack. Just like we don't constantly
think and talk about graphs, when working with operations involving infix
operators.

## 2024-05-09

There's one thing prefix operators do, that postfix operators typically don't:
clearly delimit which arguments belong to an operation, using parentheses. This
is more verbose, but also more readable. If we had an IDE though, that shows us
where arguments belong, we could re-gain that readability.

But there's one disadvantage to not having delimiters, that an IDE can't save us
from: It restricts how we can overload functions. Let's say we have a function
`f` that takes an argument of type `B`. We can't create an overloaded variant
that takes two arguments of type `A` and `B` without introducing ambiguity at
the call site.

So here's a thought experiment: What if functions can have only one argument
(which could be a tuple)? Then there would be a clear difference between `a b f`
(we need the `B` variant) and `( a b ) f` (we need the `A` and `B` variant). The
more I think about that, the more I like it!

## 2024-05-08

I've been explaining and justifying why I'm using postfix operators in
Caterpillar. I feel pretty good about this decision, but of course, it's not
without trade-offs. First and foremost, postfix operators are unfamiliar, and
will for that reason be off-putting to many. This can hurt adoption.

But I think there needs to be a balance. Diverging from established and familiar
practices, provides the opportunity to create something better than what we
already have. Which provides reasons for adoption in the first place.

And we have to keep in mind, Caterpillar is still very experimental, and no
decision is permanent. If postfix operators turn out to be a huge mistake, I can
redesign the syntax, while keeping the parts of the language and runtime that
work. If specific features and use cases are hampered by postfix operators, it's
possible to augment the language with prefix syntax where necessary.

## 2024-05-07

Yesterday, I talked about how prefix operators can cause a mismatch between
reading order and evaluation order. Today, I'd like to look at how other
programming languages address this problem by using postfix operators, or at
least constructs that have some postfix-like qualities to them.

Let's start with something that we might see in JavaScript:
`const c = (await (await a()).b()).c();`. Here we're dealing with multiple
chained promises, but of course we shouldn't write it this way. We could use
`then` instead:

```js
const c = a()
  .then((a) => a.b())
  .then((b) => b.c());
```

Much nicer to read, and while `then` isn't quite a postfix operator, there's
some postfix-ness in there! And of course, the Rust developers presumably
studied this and decided to avoid the whole thing, by making their `.await` a
postfix keyword (there's also `?`). I think this shows that postfix operators
have some nice properties, that even established languages want to benefit from.

## 2024-05-06

I explained why I like postfix operators, and why I think infix operators aren't
an option for Caterpillar. But if I want to restrict Caterpillar to one kind of
operator, I could use prefix operators, right? They are just as universal as
postfix operators, and much more common. All true, but I think in a direct
comparison, postfix operators win.

My main gripe with prefix operators, is that they cause a mismatch between the
order of operations as they are written down, and as they actually happen.
Consider something like `work_with_thing(configure_thing(construct_thing()))`.
When reading this, you have to mentally evaluate it from the inside out.

And yes, you could use variables to store intermediate results, splitting this
confusing expression over multiple lines, thereby fixing the mismatch. But you
can also use variables with postfix operators, where they make things more
clear. In cases where variables don't make things more clear, prefix operators
end up more verbose.

## 2024-05-05

I've been talking about why Caterpillar uses postfix operators. Yesterday, I
introduced how those work for simple math operations, comparing them to infix.
Today, I'd like to conclude the comparison to infix, by explaining why infix
operators aren't an option as the only kind of operator in Caterpillar.

So, why is that? For a start, because they only work with two operands. If you
have fewer, you need prefix or postfix anyway. If you have more, you need to
repeat the operator (`1 + 2 + 3`). Postfix operators have a similar problem
(`1 2 + 3 +`), but also an easy way out. If you use arrays, you can write
something like `[ 1 2 3 ] +` instead.

Infix operators have the advantage of being familiar. But in addition to not
being generally applicable, they require parentheses and operator precedence
(otherwise, their familiarity is subverted), each of which add complexity to the
language. I think that makes avoiding them the right call for Caterpillar.

## 2024-05-04

Yesterday, I said that I only want to provide postfix operator in Caterpillar.
I'd like to explain why I like them, starting with simple math operations. If we
want to add two numbers, we'd typically write this with an infix operator:
`1 + 2`. In postfix, this would be `1 2 +`.

This might look unfamiliar, but it's extremely simple to work with: Just start
at the left. There's one value (`1`), then there's another (`2`), and then we
add those (`+`). If we want to multiply the result by `3`, we can write this as
`1 2 + 3 *` (infix: `(1 + 2) * 3`). Multiplying first is easy too: `2 3 * 1 +`
(infix: `1 + 2 * 3`).

This exposes a neat thing about postfix operators: You never need parentheses,
nor is there any operator precedence. It always goes left to right. `1` and `2`
added is `3`. `3` and `3` multiplied is `9`. Or in the second example, `2` and
`3` multiplied is `6`. `6` and `1` added is `7`.

## 2024-05-03

Caterpillar uses postfix operators, instead of the more common prefix and infix
operators. Postfix operators are less familiar, which can make them unappealing.
So why did I decide to go for them anyway?

First off, I'd like to observe that many languages provide all three kinds of
operators. Let's take Rust as an example. It provides prefix operators through
function calls (`add(a, b)`), infix operators through built-in operators
(`a + b`), and a limited form of postfix operators through method calls
(`(a, b).add()`).

I want to avoid this complexity in Caterpillar. Obviously, one of them would do;
Lisp and Forth are proofs of that. And I believe, if you can only have one of
the three, then it should be postfix. Over the next few days, I'd like to talk
about why I think that is.

## 2024-05-02

I've been talking about the distinction between "solid" and "fluid" code. Solid
code is compiled, optimized, and might take a few seconds to deploy a new
version of. Fluid code is fully interactive; but how would it be implemented?

It could be bytecode, run by an interpreter. Maybe even just-in-time-compiled,
for better performance, although I have doubts that this is a path I want to go
down. Maybe it would still be compiled code, just not as heavily optimized.
Machine code in a form that is similarly structured to the source code, so you
can swap a function for a new version, without having to undo all kinds of
inlining optimizations.

I don't know what the answer is. For now, everything's interpreted, and I guess
that's a good place to start. From here, I can expand towards more compilation
and optimization, incrementally, as the situation demands.

## 2024-05-01

Last week, I wrote about that ugly debugger I wrote for the current Caterpillar
prototype. I still needed to make some improvements to the debugger and the
language runtime, but I was finally able to debug that broken code I had sitting
in a local branch.

![A screenshot of a browser window containing Caterpillar's debugger](daily-files/2024-05-01/debugger.png)

The debugger is still ugly as hell, but it's starting to shed some light on the
core questions that the current prototype was created to answer: Yes, it seems
practical to build a debugger for Caterpillar. Yes, this debugger makes it
practical to work with a language that is otherwise very confusing.

My next goal is to further substantiate these answers, by writing more code.
What I currently have just draws a white background into a window. I want to
turn this into a small [snake] game.

[snake]: https://en.wikipedia.org/wiki/Snake_(video_game_genre)

## 2024-04-30

When I talked about the concept of "solid" and "fluid" code yesterday, I glossed
over one thing. I said you could change the boundary between solid and fluid
code on the fly, but that would still require machinery that you can connect to
and deploy new versions with. Would that still be zero-overhead?

Maybe not, but I don't think it matters. If you're deploying to a server, you
would have some means to upgrade your application in any case. If you're
deploying to microcontrollers, you might have a bootloader that you can use to
upgrade your firmware over the network. In these situations, you have that
overhead anyway. It just moves from SSH, or whatever you're using, into the
language runtime.

That still leaves some other cases, like function-as-a-service platforms, or
deeply embedded systems that don't allow upgrading the firmware. If you're
running something like that, and you can't afford the additional overhead, there
could still be a "all-solid/no-updates" fallback mode. This would basically be
the same as deploying any other compiled, non-interactive language.

## 2024-04-29

I've talked about the possibility of exporting a program from the interactive
runtime, into a fully compiled and optimized form for production deployment. But
what if we had a more advanced, more flexible form of that concept? What if we
could decide on the fly which parts of the program are "solid", i.e. compiled
and optimized, or "fluid", i.e. with full support for interactive programming.

At any point, we could decide to change the boundary between solid and fluid,
whether the program is running in a local development environment or a remote
staging/production environment. That might take a few seconds,
compiling/optimizing/linking the solid code and deploying the new version, but
so what? Afterwards, we could manipulate the fluid parts of the program in a
fully interactive way.

This is just an idea. Maybe it wouldn't be as useful as I imagine it. But if it
worked out, it would turn interactive programming in Caterpillar into a
zero-overhead abstraction. It would no longer slow down code that doesn't use
it.

## 2024-04-28

Yesterday, I implied that supporting interactive programming in production
environments can be useful. Maybe to log into your server system to inspect a
weird edge case, instead of having to reproduce that locally. Or even a
customer's local system, working with them to reproduce an elusive bug.

Obviously this needs to be regulated somehow. You probably don't want to just
deploy a change directly to production, without any review or testing. And it
would be very rude (at the very least) to ship your application with a built-in
backdoor.

But I think there are ways to make this work. Permissions. Some kind of "sudo"
mode. Providing ways for the user to consent to remote access. Maybe even
teaching the runtime about staging and production environments, and integrate
with a CI/CD pipeline. I think there's a lot of potential here waiting to be
explored!

## 2024-04-27

When it comes to zero-overhead abstractions and Caterpillar, the elephant in the
room is interactivity. Making immediate changes to a running program has an
inherent cost. How can such a feature be supported without a heavy runtime?

The short answer is, it probably can't. But I think there are ways to alleviate
this. For a start, interactivity is going to be most useful during development,
and there you might be able to afford the cost. For production deployments, we
can support "exporting" your program from the interactive runtime, into a fully
compiled and optimized form.

And that might be fine in many cases. But I think giving up on interactivity for
production deployments would fall short of Caterpillar's potential. And I do
have ideas on how to support that, without making the whole language slow.

## 2024-04-26

My primary language for the last 10+ years has been Rust, which has zero-cost
abstractions (I prefer the term "zero-overhead"). An abstraction is zero-cost
(or zero-overhead), if you couldn't implement it any faster "by hand", and if it
doesn't impose a performance cost on code that doesn't use it.

While I rarely, if ever, use Rust to its full potential in terms of performance,
I very much appreciate that concept. It ensures that the language can be used in
many different environments. Heavier abstractions, like garbage collection, can
rule out many microcontroller or WebAssembly use cases, for example.

It is unlikely that Caterpillar will support as diverse a set of use cases as
Rust does. But I don't want to make design decisions that rule out whole classes
of those outright. And for that reason, I absolutely want to implement the
concept of zero-overhead abstractions in Caterpillar.

## 2024-04-25

The core questions I want to answer with the current prototype are a) whether
building a debugger is feasible, and b) whether having one will help make an
otherwise confusing language practical to use. I have some news on that first
question!

![A screenshot of a browser window containing Caterpillar's debugger](daily-files/2024-04-25/debugger.png)

This doesn't have any of the buttons you'd expect from a debugger, but you can
set and remove breakpoints. And that enables you to step over, step into, or
continue to wherever you want; just in the most inconvenient way possible. And
you have to scroll all the time to see the information you need. But hey, it
works!

It would sure be great for someone with an eye for design and a preference for
frontend work to show up and save me right about now, but I guess I'm on my own
for the time being. Oh well. Time to figure out if I can use this thing to make
sense of that broken code I have sitting in a local branch.

## 2024-04-24

I've talked about the platform concept before, and how Caterpillar code will be
sandboxed. Code you call will be able to do very little, unless you explicitly
provide capabilities as arguments.

In that context I wonder, would it be practical to make Caterpillar [total] by
default? Then you could be sure that a function you call always terminates,
which protects against bugs and restricts what malicious code can do. A function
that needs it, could accept an argument (which could be a built-in function)
that allows unrestricted recursion, unlocking full Turing-completeness.

This was inspired by hearing about [Dhall] on the Software Unscripted podcast.
I've never worked with Dhall or another total programming language, so I don't
have a good intuition for what's possible. But this is something to keep in
mind, for sure.

[total]: https://en.wikipedia.org/wiki/Total_functional_programming
[Dhall]: https://dhall-lang.org/

## 2024-04-23

I've been talking about how to step through Caterpillar programs, forward and
backward, undoing and redoing changes to their state. But so far, I've
completely glossed over the topic of I/O. How would that interact with such a
capability?

Graphics are typically re-rendered every frame, based on the current frame. So
it wouldn't matter in that case. File I/O could work, if instructions/events
carry enough information to rewind changes there too. With a log file, you could
acknowledge the rewind by logging a message about it, then just continue from
there.

What's not at all clear to me, is how to hook I/O into this whole system in the
first place. It seems pretty straight-forward on the instruction level. But what
about Event Sourcing? That's designed to explicitly _not_ do any I/O when
applying events. More to think about, for sure.

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
