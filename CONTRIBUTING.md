# Contributing to delta-sharing-rs

Development on this project is driven by volunteer contributors. We welcome new contributors, including not only those who develop new features, but also those who are able to help with documentation and provide detailed bug reports. 

Please take note of our [code of conduct](CODE_OF_CONDUCT.md).

If you want to start contributing, take a look at the issues: https://github.com/delta-incubator/delta-sharing-rs/issues (implementation of `good first issue` is coming soon). 

## Claiming an issue

If you want to claim an issue to work on, you can write the word `take` as a comment in it and you will be automatically assigned.

## Quick start

Full instructions are currently in [readme](CODE_OF_CONDUCT.md)

## To make a pull request (PR)
- Install Rust, e.g. as described [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- Make sure all the following steps run/pass locally before submitting a PR (requires `clippy`)
```
cargo fmt -- --check
cargo clippy --tests
cargo doc --no-deps
```

## Developing in VSCode

*These are just some basic steps/components to get you started, there are many other very useful extensions for VSCode*

- For a better Rust development experience, install [rust extention](https://marketplace.visualstudio.com/items?itemName=1YiB.rust-bundle)
- For debugging Rust code, install [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb). The extension should even create Debug launch configurations for the project if you allow it, an easy way to get started. Just set a breakpoint and run the relevant configuration.


