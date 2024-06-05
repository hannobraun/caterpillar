# Caterpillar

## Vision

**Caterpillar is a programming language** with a dual goal:

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
language right now. Development is focused on creating a basic solution for one
use case (game development) on one platform (browsers).

Caterpillar has gone through [a series of prototypes](archive/), of varying
levels of sophistication, each of which provided answers to different questions.
The [current prototype](capi/) explores whether an early focus on tooling is a
practical approach.

You can keep up with the project by reading my [daily thoughts], which include
development updates.

## Design

This section aims to document the decisions that have gone into the language
design. Due to the current state of the project, this isn't (yet) a long list,
nor is anything here going to be final.

For more information on future design directions, please follow my
[daily thoughts]. There's also a [design document](design.md), which I'd like to
phase out, but that still provides some value.

### Postfix operators

The language uses postfix operators, like `arg1 arg2 do_thing` or `1 2 +`, as
opposed to prefix (`do_thing(arg1, arg2)`, `(+ 1 2)`) or infix (`1 + 2`)
operators.

To keep the language simple, I want to (at least initially) restrict it to one
type of operator. I believe postfix operators are the best option under that
constraint, due to their combination of simplicity, conciseness, and natural
support for chaining operations. That comes at the cost of familiarity.

Further information in daily thoughts
[2024-05-03](https://capi.hannobraun.com/daily/2024-05-03),
[2024-05-04](https://capi.hannobraun.com/daily/2024-05-04),
[2024-05-05](https://capi.hannobraun.com/daily/2024-05-05),
[2024-05-06](https://capi.hannobraun.com/daily/2024-05-06),
[2024-05-07](https://capi.hannobraun.com/daily/2024-05-07),
[2024-05-08](https://capi.hannobraun.com/daily/2024-05-08),
[2024-05-09](https://capi.hannobraun.com/daily/2024-05-09),
[2024-05-10](https://capi.hannobraun.com/daily/2024-05-10), and
[2024-05-11](https://capi.hannobraun.com/daily/2024-05-11).

### No commitment to backwards compatibility

I'm not targeting a 1.0 release after which the language is expected to have few
or no breaking changes. Right now, everything is early-stage and experimental
anyway. But even long-term, I don't want to commit to backwards compatibility.

As the language matures, the focus will be on making any given upgrade easy. But
each release might introduce changes that require updates to Caterpillar code.
Where possible, users will be given ample time to make those changes, or they
will be automated outright.

See the following daily thoughts for further information:
[2024-05-28](https://capi.hannobraun.com/daily/2024-05-28),
[2024-05-29](https://capi.hannobraun.com/daily/2024-05-29),
[2024-05-31](https://capi.hannobraun.com/daily/2024-05-31),
[2024-06-01](https://capi.hannobraun.com/daily/2024-06-01),
[2024-06-02](https://capi.hannobraun.com/daily/2024-06-02),
[2024-06-03](https://capi.hannobraun.com/daily/2024-06-03), and
[2024-06-05](https://capi.hannobraun.com/daily/2024-06-05).

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
