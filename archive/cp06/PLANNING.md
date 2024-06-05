# Planning

## Core Ideas

My motivation for working on Caterpillar is built on top of a few ideas. None of
them are original, but some of them were quite profound insights to me, as I
started to understand them. In this section, I'd like to present those core
ideas, while providing links to sources, where I can trace my insights back to
them.

The primary motivator for Caterpillar was the insight (which I haven't proven
yet) that our usual mode of software development is outdated, and that
programming can be much more effective and approachable using an **interactive
programming** paradigm, where you always manipulate a running program.
([Inventing on Principle](https://www.youtube.com/watch?v=PUv66718DII),
[Stop Writing Dead Programs](https://www.youtube.com/watch?v=8Ab3ArE8W3s))

When your program is interactive, it makes sense to **store code in a structured
format**, as that makes it much easier to figure out what needs to happen when a
given function is updated. ([Unison](https://www.unison-lang.org/))

I'm interested in an interactive programming language that is just as universal
as Rust is, while being no less safe. However, the complexity of Rust is a big
drawback. I realized that by combining affine or **linear types with pure
functions**, you can get the zero-overhead automatic memory management of Rust
without the complexity. ([Rust](https://www.rust-lang.org/),
[HVM](https://github.com/HigherOrderCO/HVM))

As I kept working on Caterpillar, I realized that by providing all capabilities
in terms of functions that exist in a namespace (as opposed to built-in
operators) and that by exclusively using **dependency injection** for providing
access to those functions (as opposed to importing them from a globally
accessible namespace), I can address safety (like memory safety, but also other
relevant effects like randomness) and security (by sandboxing all code by
default). (Inspiration for this one came from many diverse sources that I was
exposed to over many years and can no longer attribute. But more recently,
[Roc](https://www.roc-lang.org/) served as inspiration here.)

I expect this list of core ideas to change and grow, as I keep working on
Caterpillar and refine my ideas.

## Notes

### Can functions and bindings be unified?

It seems overly complicated to have _two_ concepts that bind a name to some kind
of value. However, they work differently:

- A word that refers to a function _evaluates_ that function.
- A word that refers to a binding puts the value of that binding onto the stack.
  So if bindings were used to assign names for functions, there would have to be
  some kind of special rule. Maybe the rules could look like this:
- There is a _function_ type that is created from a block with an intrinsic.
  Something like this: `{ true } fn`
- Evaluating a word that refers to a binding _evaluates_ that word.
- Evaluating a _function_ evaluates the contents of the block that is associated
  with the function.
- Evaluating any other value (including a bare block) just puts that value on
  the stack. So bindings would work according to consistent rules in all cases.
  But there would be this notion of _evaluation_ that works differently for
  functions (and maybe other types, like modules) compares to all other values.

### Module System

One goal I have for Caterpillar is to sandbox all code. I want to achieve that
by not allowing code to access anything global, and by that I don't only mean
global data, but also types in the standard library (which, in most programming
languages, live in a global namespace).

If a mode wants to use another module, it needs to specify that as an argument,
and whatever is using that module needs to provide those arguments. So,
basically, dependency injection.

There's one question that stumped me for a bit: If a module needs to specify
everything it needs as an argument, how does it know to name those arguments? If
I need to tell my caller that I need "X", and I don't have access to any global
namespace, where exactly does the name for "X" come from?

I've come up with the following solution. Consider this to be pseudo-code, as
the final syntax will certainly be different.

```
# Our application has a `root` module, which gets passed an index of all the
# names that exist in our system. What exactly those are is going to be
# different, based on the platform that's running the application.
#
# This module is the first thing that the platform evaluates on startup. During
# that initial evaluation (which could happen at compile- or runtime, depending
# on the use case and how the language design shakes out), all the function
# within it get defined.
mod root(index) {
    # Our `root` module needs to define a `main` function. It gets called after
    # the `root` module is evaluated, and it gets all of the standard library as
    # a parameter.
    #
    # What is in that standard library is, again, going to depend on the
    # platform. And there might be other parameters too, depending on what runs
    # us, like CLI arguments.
    fn main(std) {
        std.fs some_function_dealing_with_files
    }

    fn some_function_dealing_with_files(fs: index.Fs) {
        # doing stuff with files here, using `fs`
    }
}
```

It would become pretty inconvenient if every function would have to define all
of its dependencies all of the time. So here's another rule: modules and
functions have access to all of their parent scopes (lexical scoping). Here's
how that would work with dependencies.

```
mod root(index) {
    fn main(std) {
        std.fs file_stuff => fs
        "/path/to/file" write_file
        "/path/to/file" read_file
    }

    mod file_stuff(fs: index.Fs) {
        fn write_file(path) {
            path fs.write_file
        }

        fn read_file(path) {
            path fs.read_file
        }
    }
}
```

### Persistence

At some point, we should store functions persistently, not just load some
hardcoded ones in at startup and forget anything defined at runtime. Since the
time to rewrite all software from the ground up has not come yet, that means
storing the canonical representation in files, so they can be version-controlled
using an external tool.

I've found the File System Access API:

- https://developer.mozilla.org/en-US/docs/Web/API/File_System_Access_API
- https://developer.chrome.com/articles/file-system-access/

Unfortunately it's not sufficiently supported yet:
https://caniuse.com/native-filesystem-api

At this point, it's probably best to wait and see. If we need persistency before
this API is ready, we can do it through a background service.

### Function Lookup

I want to implement multiple dispatch, so a given function might only be one of
a number of candidates for a call. As a result, the following won't work in the
general case:

```
mod my_mod(fs) {
    fn my_fn(path) {
        path fs.open
    }
}
```

This could be how some specific cases work, to call one specific function, but
that would not be general enough to cover all cases, nor would it need to be in
the language at all.

Maybe there can be a `use` function that loads functions into the local scope,
where they are available for lookup:

```
mod my_mod(index) {
    index.fs use # `open` gets loaded into local scope here

    fn my_fn(path) {
        path open
    }
}
```
