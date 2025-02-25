# Getting Started

Broc is a language for making delightful software. It does not have an 0.1 release yet, and we
certainly don't recommend using it in production in its current state! However, it can be fun to
play around with as long as you have a tolerance for missing features and compiler bugs. :)

The [tutorial](https://roc-lang.org/tutorial) is the best place to learn about how to use the language - it assumes no prior knowledge of Broc or similar languages. (If you already know [Elm](https://elm-lang.org/), then [Broc for Elm Programmers](https://github.com/roc-lang/broc/blob/main/broc-for-elm-programmers.md) may be of interest.)

If you have a specific question, the [FAQ](../FAQ.md) might have an answer, although [Broc Zulip chat](https://broc.zulipchat.com) is overall the best place to ask questions and get help! It's also where we discuss [ideas](https://broc.zulipchat.com/#narrow/stream/304641-ideas) for the language. If you want to get involved in contributing to the language, Zulip is also a great place to ask about good first projects.

## Installation

- [Linux x86_64](linux_x86_64.md)
- [MacOS Apple Silicon](macos_apple_silicon.md)
- [MacOS x86_64](macos_x86_64.md)
- [Windows](windows.md)
- [Other](other.md)

## Running Examples

You can run examples as follows:

```sh
cd examples
broc dev helloWorld.broc
```

[crates/cli_testing_examples/benchmarks](https://github.com/roc-lang/broc/tree/main/crates/cli_testing_examples/benchmarks) contains more examples.


## Getting Involved

The number of people involved in Broc's development has been steadily increasing
over time - which has been great, because it's meant we've been able to onboard
people at a nice pace. (Most people who have contributed to Broc had previously
never done anything with Rust and also never worked on a compiler, but we've
been able to find beginner-friendly projects to get people up to speed gradually.)

If you're interested in getting involved, check out
[CONTRIBUTING.md](https://github.com/roc-lang/broc/blob/main/CONTRIBUTING.md)!

If you're interested in substantial implementation- or research-heavy projects
related to Broc, check out [Broc Project Ideas][project-ideas]!

[project-ideas]: https://docs.google.com/document/d/1mMaxIi7vxyUyNAUCs98d68jYj6C9Fpq4JIZRU735Kwg/edit?usp=sharing
