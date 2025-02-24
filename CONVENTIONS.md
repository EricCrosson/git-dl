# Overview

**git-dl** is a simple tool to download git repositories into a structured location on disk.
**git-dl** is an exercise in controlling my own software supply-chain security and writing simple scripts in Rust with minimal dependencies.

To download a repository into `$HOME/workspace/{owner}/{repo}`, run:

```shell
git dl <repo>
```

## Dependencies

You may not use any dependencies.
Stick to first-class, native Rust functionality.

## Errors

Use errors as described in Modular Errors in Rust, by Sabrina Jewson.

## Error messages

Write error messages like a master technical writer. Errors should convey two
things to the reader (who can be assumed to be a maintainer):

1. What went wrong?
2. What do you need to do to fix it?

Liberally document and assert (using the Node.js assert module) invariants.
When you throw an error to close off a code path (for example, when parsing
an environment variable that is expected to be populated, but it is undefined)
follow the Rust programming language's advice on wording error or assertion messages:

> Use the message to present information to developers debugging the panic
> (“expect as precondition”).
>
> The “expect as precondition” style instead focuses on source code readability,
> making it easier to understand what must have gone wrong in situations where
> panics are being used to represent bugs exclusively. Also, by framing our message
> in terms of what “SHOULD” have happened to prevent the source error, we end up
> introducing new information that is independent from our source error.
>
> ```
> thread 'main' panicked at src/main.rs:4:6:
> env variable `IMPORTANT_PATH` should be set by `wrapper_script.sh`: NotPresent
> ```
>
> In this example we are communicating not only the name of the environment
> variable that should have been set, but also an explanation for why it should
> have been set, and we let the source error display as a clear contradiction to
> our expectation.
>
> Hint: If you’re having trouble remembering how to phrase
> expect-as-precondition style error messages remember to focus on the word
> “should” as in “env variable should be set by blah” or “the given binary should
> be available and executable by the current user”.
