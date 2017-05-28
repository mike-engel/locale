# Locale
> A polyfill to understand your users' preferred languages

Locale is a small back end app that will return a sorted list of the user's preferred languages based on the `Accept-Language` header.

It's written in Rust and should therefore be very fast! You can view the website which provides examples and how to use it at [locale.now.sh](https://locale.now.sh).

# Contributing

If you find a bug or would like to add a feature, I welcome all pull requests!

As this is written in Rust, you'll need to have Rust installed. I recommend using [rustup](https://rustup.rs) since it's pretty great! Additionally, since this is using [Rocket](https://rocket.rs), you'll need to be using Rust nightly.

```sh
# install nightly if you don't already have it
rustup toolchain install nightly

# use nightly for this folder explicitly (won't affect any other rust projects)
rustup override set nightly
```

Once you have nightly downloaded and set via rustup, install the cargo dependencies and run the development version.

```sh
# update/install dependencies
cargo update

# run the project
cargo run
```

If you're working on the HTML page, it's helpful to have the server restart when you make changes. I recommend using `watchexec` to handle this.

```sh
# install watchexec
cargo install watchexec

# run watchexec and have it restart the server on changes
watchexec --exts hbs --restart "cargo run"
```

Finally, this project uses the awesome `rustfmt` crate for automatic formatting of rust files. To install it, use Cargo. If you don't want it in your IDE, you can run it from the command line.


```sh
# install rustfmt
cargo install rustfmt

# format the rust file
rustfmt src/main.rs
```

# [Code of Conduct](CODE_OF_CONDUCT.md)

# [License (MIT)](LICENSE.md)
