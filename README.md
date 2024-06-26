# Caterpillar

## Vision

Caterpillar aims to create a better way to develop software. It is a
**programming language** with a dual goal:

- Create an **immediate connection to the code you're writing**, to give you an
  intuitive understanding of what it's doing.
- Bring this experience to many places: browsers, servers, desktops, phones,
  watches, microcontrollers; CPUs and GPUs.

Caterpillar aims to achieve this via **interactive programming**; the practice
of directly manipulating your running program, instead of going through an
extended loop of re-compiling, re-starting, then navigating to where you can
test your change.

## Status

Caterpillar is still early-stage and experimental. It can hardly be called a
programming language right now. Development is focused on creating a basic
solution for one use case (game development) on one platform (browsers).

You can keep up with the project by reading my [daily thoughts], which include
development updates.

## Inspiration

Caterpillar draws inspiration from many sources. The following is an
**incomplete** list:

- [Inventing on Principle](https://vimeo.com/906418692)
- [Tomorrow Corporation Tech Demo](https://www.youtube.com/watch?v=72y2EC5fkcE)
- [Stop Writing Dead Programs](https://jackrusher.com/strange-loop-2022/)

## Design

This section aims to document the decisions that have gone into the language
design. Due to the current state of the project, this isn't (yet) a long list,
nor is anything here going to be final.

For more information on future design directions, please follow my
[daily thoughts]. There's also a [design document](design.md), which I'd like to
phase out, but that still provides some value.

### Experimentation first; conservative decisions later, as necessary

I want Caterpillar to be adopted. That could mean that I need to focus
innovation where that provides the most benefit, and keep other aspects of the
language conservative and familiar.

Before that becomes necessary, I want to experiment though. At least give the
language to chance to be more than an incremental improvement over the status
quo.

The following daily thoughts provide more context:
[2024-06-18](https://capi.hannobraun.com/daily/2024-06-18) and
[2024-06-19](https://capi.hannobraun.com/daily/2024-06-19).

### Continued evolution over backwards compatibility

I'm not targeting a 1.0 release after which the language is expected to have few
or no breaking changes. Right now, everything is early-stage and experimental
anyway. But even long-term, I don't want to commit to backwards compatibility.
The continued evolution of the language and the costs of ongoing maintenance
will be prioritized instead.

As the language matures, there will be a growing focus on making any given
upgrade easy. But each release might introduce changes that require updates to
Caterpillar code. Where possible, users will be given ample time to make those
changes, or they will be automated outright.

The following daily thoughts provide more context:
[2024-05-28](https://capi.hannobraun.com/daily/2024-05-28),
[2024-05-29](https://capi.hannobraun.com/daily/2024-05-29),
[2024-05-31](https://capi.hannobraun.com/daily/2024-05-31),
[2024-06-01](https://capi.hannobraun.com/daily/2024-06-01),
[2024-06-02](https://capi.hannobraun.com/daily/2024-06-02),
[2024-06-03](https://capi.hannobraun.com/daily/2024-06-03), and
[2024-06-05](https://capi.hannobraun.com/daily/2024-06-05).

### Postfix operators

The language uses postfix operators, like `arg1 arg2 do_thing` or `1 2 +`, as
opposed to prefix (`do_thing(arg1, arg2)`, `(+ 1 2)`) or infix (`1 + 2`)
operators.

To keep the language simple, I want to (at least initially) restrict it to one
type of operator. I believe postfix operators are the best option under that
constraint, due to their combination of simplicity, conciseness, and natural
support for chaining operations. That comes at the cost of familiarity.

The following daily thoughts provide more context:
[2024-05-03](https://capi.hannobraun.com/daily/2024-05-03),
[2024-05-04](https://capi.hannobraun.com/daily/2024-05-04),
[2024-05-05](https://capi.hannobraun.com/daily/2024-05-05),
[2024-05-06](https://capi.hannobraun.com/daily/2024-05-06),
[2024-05-07](https://capi.hannobraun.com/daily/2024-05-07),
[2024-05-08](https://capi.hannobraun.com/daily/2024-05-08),
[2024-05-09](https://capi.hannobraun.com/daily/2024-05-09),
[2024-05-10](https://capi.hannobraun.com/daily/2024-05-10), and
[2024-05-11](https://capi.hannobraun.com/daily/2024-05-11).

### Stack-based evaluation, but not a stack-based language

Caterpillar, featuring postfix operators, has a stack-based evaluation model.
But it is not a stack-based language. There is no single data stack that is used
to pass arguments and return values between functions.

Instead, Caterpillar uses a much more conventional model, with a regular stack
and explicit function arguments. Each operand stack is local to a (function)
scope.

This approach is less error-prone, but also less flexible and more verbose. It
seems to make sense right now, but as the language grows other features that
make it less error-prone (like static typing and better tooling), this decision
can be revisited.

The following daily thoughts provide more context:
[2024-05-10](https://capi.hannobraun.com/daily/2024-05-10),
[2024-05-11](https://capi.hannobraun.com/daily/2024-05-11),
[2024-06-20](https://capi.hannobraun.com/daily/2024-06-20),
[2024-06-21](https://capi.hannobraun.com/daily/2024-06-21),
[2024-06-22](https://capi.hannobraun.com/daily/2024-06-22),
[2024-06-23](https://capi.hannobraun.com/daily/2024-06-23),
[2024-06-24](https://capi.hannobraun.com/daily/2024-06-24), and
[2024-06-25](https://capi.hannobraun.com/daily/2024-06-25).

## Acknowledgements

I'd like to thank [Martin Dederer](https://github.com/martindederer) for
suggesting the name!

## License

This project is open source, licensed under the terms of the
[Zero Clause BSD License] (0BSD, for short). This basically means you can do
anything with it, without any restrictions, but you can't hold the authors
liable for problems.

See [LICENSE.md] for full details.

[daily thoughts]: https://capi.hannobraun.com/daily
[Zero Clause BSD License]: https://opensource.org/licenses/0BSD
[LICENSE.md]: LICENSE.md
