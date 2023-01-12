# This is an attempt to imagine constructing a triangle from low-level building
# blocks in a kind of concatenative, object-oriented language that is based on
# message passing.

# Here we define three functions, each of which represent a parameter of this
# program. The parameters are points, but the functions we define won't return
# a point, but a signal (in the FRP sense) of points. Meaning the user, through
# the application that executes this program, can manipulate the parameters, and
# the changes will be passed through to where the values are used, without the
# need to execute this program again.
#
# This is just a placeholder, and everything about this is preliminary:
#
# - No idea what the syntax should be.
# - No idea what `point` is. A function that returns a representation of the
#   type that represents a point?
# - No idea how the runtime knows that this program defines three parameters.
#   Maybe there can be some magic that allows `param` to know the name of the
#   function it was called from, but there must be better ways to handle this.
: a = param point
: b = param point
: c = param point

# Create an empty sketch.
sketch
    # Select the exterior of that sketch, which is a cycle. As a general rule,
    # objects are only accessible from the same line, or from deeper levels of
    # indentation. Once we execute a line on the same level of indentation, the
    # cycle object will be dropped, but as long as we're executing lines on
    # deeper levels of indentation, it will remain accessible.
    exterior
        # Add an edge to the exterior cycle.
        edge
            # Select the vertex the edge points to. By sending it a `:=`
            # message, we let it know that the following message is supposed to
            # manipulate it. The type of `a` makes sure that the correct thing
            # is manipulated.
            to := a
            # Similar thing, except this time we're defining the edge's curve to
            # be an empty line. Not sure how that works. I guess we can later
            # infer what kind of line that is, specifically, once we know both
            # vertices?
            curve := line
        # Define more edges. We want to make a triangle.
        edge
            to := b
            curve := line
        edge
            to := c
            curve := line
        # A cycle always keeps itself connected as edges are added, so now we
        # have a triangle. We defined that each edge is a line, and defined the
        # three points, so everything should be well-formed.

# At this point, everything is dropped according to our scoping rules. How does
# the runtime know that the program is supposed to return the sketch/triangle?
# Because it was the last thing that was dropped?
#
# And alternative would be to define outputs explicitly.
