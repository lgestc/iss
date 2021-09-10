# Git iss 🛰️

## What?

This utility parses current branch name to pull out the issue name from it. Once retrieved, it will be printed to the standard output.

## How?

It uses libgit2 under the hood and looks for JIRA-like issue codes in the branch names.

## Installation

Clone the repository and run `cargo install` in the root directory.

## Example use case

You can use it to add jira code to the commit message easily, for instance (in vim):

`:r! iss`
