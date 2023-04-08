# Tasks

- Print error location with error message
- Prevent shadowing between functions and bindings
- Simplify tokenizer
  There are a few too many edge cases for my taste.
- Organize tests in modules
  Just having all tests side-by-side is getting a bit unwieldy. I think the following syntax should work:
  ```
  mod my_module {
      test "my test" { ... }
      ...
  }
  ```
