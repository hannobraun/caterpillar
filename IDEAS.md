# Ideas

## Can functions and bindings be unified?

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
