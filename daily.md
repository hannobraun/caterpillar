# Daily Thoughts

These are my daily thoughts on Caterpillar. If you have any questions, comments,
or feedback, please [get in touch](mailto:hello@hannobraun.com).

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

I'm doing again a similar thing again, with this new format. Start small and,
more importantly, get started. Then learn, correct, or abandon. Whatever turns
out to be appropriate.

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
