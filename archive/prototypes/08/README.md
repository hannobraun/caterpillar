# Caterpillar - Prototype 8

## About

This is a prototype (of many) for the Caterpillar programming language. Check
out the top-level README for more information.

This prototype doesn't neatly fit into the line of numbered prototypes archived
in the same directory. It doesn't directly follow prototype 6. Instead, it
branched off of the (at the time of writing) main development branch, which
itself was a direct follow-up to prototype 6.

## Concept

This prototype was born out of the frustration with the (again, the the time of
writing) current development branch, which has a pretty low ration of usefulness
to complexity. (Not that it's overly complex, but it's not very useful either.)
I had two key ideas on how to more approach Caterpillar in a more practicable
manner:

1. Focus on creating a simple virtual machine and program it with an assembler.
   The hope was, that this would provide a stable baseline to explore
   interactive programming with, without getting bogged down in programming
   language design from day one.
2. Keep it simple and make it practical by focussing on one single use case. I
   chose browser-based games, because that's what was bumping around my head at
   the time. The specific game I chose to implement was a Snake variant, as that
   is pretty easy to do.

I think idea 2 has merit. This isn't surprising, but it's a matter of discipline
to stick to one thing long-term, instead of going and exploring. (After all,
this is a side project, and it's not monetized, so it does take some discipline
to not just wander off, following my whims.)

Idea 1 lead to mixed results. This might have been a better approach long-term,
or it might not. Fact is, it caused a lot of work in the short term, without a
clear benefit over the previous approach.

## Status

I managed to create the following components, all of which work:

1. A browser-based host written in JavaScript, that loads the WebAssembly-based
   runtime and serves as an interface between that and the browser APIs.
2. A runtime, written in Rust, which interfaces with the host on the WebAssembly
   side, manages the Caterpillar Virtual Machine, and also still contains most
   of the game logic for the Snake game.
3. The Caterpillar Virtual Machine, which is a low-level, stack-based VM that
   can run Caterpillar bytecode.
4. The Caterpillar Assembler, which compiles Caterpillar assembly to bytecode.
5. A tiny bit of assembly code that implements a tiny bit of the game logic.
6. A builder that monitors most of the former for changes and deploys new
   versions. This doesn't work fully reliably.

As I said, all of this works, but it isn't really useful, even after a huge
amount of work. The assembler (and bytecode) had just started to be
Turing-complete, ore thereabouts, but was barely usable without a debugger,
which perhaps is not surprising.

## Flaws

I realized that this low-level VM wasn't a great idea. Low-level meant simple,
and easy to implement, but it wasn't really any less work than my
interpreter-based approaches had been previously.

On a more fundamental level, the idea was flawed because the level of
abstraction was _too_ low. Creating something higher-level might not have been
any more difficult, would have made it easier to eventually compile a high-level
language to it, and would actually have been more portable.

I'm going to gloss over those first two reasons, as they seem obvious enough,
but I'd like to expand on the last. If you compile a high-level language to a
low-level instruction set, you use context. For example, vectors (as in math,
not the ill-named data structure) just become numbers in registers (or in this
case, numbers on the stack).

But the target your instruction set runs on in the end (whether interpreted or
compiled), might have use for that high-level information you lost. I chose
vectors as an example, because they are relevant if your target supports SIMD,
or if your target is a shader language (which supports vectors natively).

## Conclusion

I believe that creating a VM for Caterpillar can make a lot of sense. It means
that the (possibly limited) target doesn't have to run a whole language
implementation. Stuff like tokenizing, parsing, type checking, and whatever else
needs to happen to make a programming language work, can stay on the more
powerful dev machine.

But if I attempt this in the future, I will design an instruction set that is
closer to the level of abstraction of the language that it is designed to
support. I don't know _how_ close they should be, but definitely close enough to
capture the key concepts from the language that make sense to know about when
running on the specific target.
