# Caterpillar

An experiment in describing geometry programmatically. Thanks go to [Martin Dederer](https://github.com/martindederer) for suggesting the name!

This experiment has run its course and [has been superseded](../cp1/).


## How to Run

Run using [Trunk](https://trunkrs.dev/):

```
trunk serve
```


## Concept

Here's an example of what I currently have in mind. Many details are bound to change as the implementation comes along, but for now, this is what I'm working towards, language-wise.

```
# Here we define a function using a symbol (for the name) and the `fn` function.
# This puts the function value on the stack, to be modified now.
#
# The signature of `fn` would be something like this: `-> function`
:triangle fn
    # Define the function input. In principle, this can be optional when
    # defining a function in this language. It is not optional in this specific
    # case though, as the host application can/should use this information to
    # generate an interface (CLI, GUI, HTML, whatever the context is), to
    # manipulate these parameters.
    #
    # The signature of `in`: `function [type] -> function`
    [ point point point ] in
    # Define the function output. This can be completely optional, I think, as
    # it can just be inferred from the function body. But it documents intent
    # and can be used for type checking.
    #
    # Signature: `function [type] -> function`
    [ sketch ] out
    # The function body that defines and manipulates our sketch.
    [
        # Take the top three values (the function parameters) from the stack and
        # define functions in the local scope that return them.
        [ a b c ] set
        # `sketch` puts a value representing the `sketch` type on the stack,
        # `make` consumes it and puts the constructed value on the stack.
        sketch make
            # Add an edge to the sketch: `sketch -> sketch edge`
            # The sketch has been modified, and the sketch will stay in there,
            # even if we immediately drop the edge. The `edge` object is a
            # representation of the edge within the sketch, which we can use to
            # manipulate this specific edge within the sketch.
            add_edge
                # Make point `a` the point that the edge originates from.
                a from
                # Put a value representing the `curve` type on the stack, and
                # use that basically as a selector for that aspect of the edge.
                # `as_line` defines that curve to be a line.
                curve as_line
                # Drop the edge, so it's not in the way when we add the next
                # one.
                drop
            # Repeat two more times to create the triangle.
            add_edge
                b from
                curve as_line
                drop
            add_edge
                c from
                curve as_line
                drop
    ]
    # End the function definition. This moves everything off the stack, and puts the
    # function into the local scope, so it can be called:
    # `function [word] ->`
    def
```
