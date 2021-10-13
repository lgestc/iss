# Git iss 🛰️

## What?

This utility parses current branch name to pull out the issue name from it. Once retrieved, it will be printed to the standard output.

Probably will just work for me.

## How?

It uses libgit2 under the hood and looks for JIRA-like issue codes in the branch names.

## Installation

Clone the repository and run `cargo install` in the root directory.

## Example use case

You can use it to add jira code to the commit message easily, for instance (in vim):

`:r! iss`

## Extras

If your project uses some suffixes to task codes depending on the project (eg in a monorepo setup),
you can define those in `$HOME/.config/iss/suffixes` like this:

```
proj [Some Suffix]
proj2 AnotherSuffix
```

Then you can have the suffix appended to your project if the first arg matches the prefix alias (first string in line).
