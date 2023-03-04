# Tasks

- Fix memory leak in event loop
  Figure out how many generations to retain.
- Implement local scopes
  Currently all variables are global.
- Implement linear or affine types
  Types on the stack are basically affine by default, but variables are implicitly cloned on every use.
