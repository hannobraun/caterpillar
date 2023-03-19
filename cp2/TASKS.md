# Tasks

- Color test output
- Print error location with error message.
- Don't use round brackets to delineate bindings.
  Eventually, it should be possible to employ pattern matching here, and then the round brackets would just add syntax noise. In addition, round brackets might eventually be used for tuples. This would exacerbate the syntax noise problem, if the two features were combined. Also, using round brackets for two different features might be confusing.
  Instead, we can remove the brackets and just use an "end of expression" token. Such a token is likely to be useful in other situations too.
  In many languages, `;` is used for this kind of thing, but I would actually prefer `.`. I find it more aesthetically pleasing (which is of course highly subjective) and it might be more understandable to new programmers, at least those that know natural languages that use the period to end sentences.
