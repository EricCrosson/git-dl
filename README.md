# git-dl

**git-dl** is a simple tool to download git repositories into a structured location on disk.
There are many like it[^1][^2], but this one is mine.

[^1]: https://github.com/wezm/git-grab

[^2]: https://github.com/jmcgarr/git-grab

## Why?

**git-dl** is an exercise in controlling my own software supply-chain security and writing simple scripts in Rust with minimal dependencies.

## Use

To download a repository into `$HOME/workspace/{owner}/{repo}`, run:

```shell
git dl <repo>
```
