# Tasks

- Look into defining local variables.
  Something like `[ "a" "b" ] def` to name the two top entries on the stack `a` and `b`.
- Fix busy loop in `main`
  As per the comment there, this causes the UI to flicker. Which can probably be avoided by adding a UI buffer, compare it to the previous buffer, and only write what has changed.
