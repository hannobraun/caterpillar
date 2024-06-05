# Tasks

## Implementation

- Fix memory leak in event loop
  Figure out how many generations to retain.
- Replace built-in stack operations
  Rewrite them using bindings.

## Variables

- Implement local scopes
  Currently all variables are global.
- Implement linear or affine types
  Values on the stack are basically affine by default, but variables are implicitly cloned on every use.
- Allow binding to empty values
  Effectively, this means dropping values from the stack. Like this (new syntax): `=> ( _ )`
- Prevent shadowing of functions
  A variable shadowing a function should not happen by accident. Eventually, it could be a warning, but for now, a hard error will do.

## Syntax

- Improve the parsing to be less whitespace-sensitive
  Right now, a list needs to be written like this: `[ 1 2 3 ]`
  But there's no good reason not to allow this: `[1 2 3]`
- Consider replacing `bind` builtin with syntax
  Replace this: `[ :a :c :c ] bind`
  With this: `=> ( a b c )`
