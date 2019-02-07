# Locale

> A polyfill to understand your users' preferred languages

Locale is a small back end app that will return a sorted list of the user's preferred languages based on the `Accept-Language` header.

It's written in Rust and should therefore be very fast! You can view the website which provides examples and how to use it at [locale.now.sh](https://locale.now.sh).

# Contributing

If you find a bug or would like to add a feature, I welcome all pull requests!

As this is written in Rust, you'll need to have Rust installed. I recommend using [rustup](https://rustup.rs) since it's pretty great!

Once you have rust downloaded via rustup, install the cargo dependencies.

```sh
# update/install dependencies
cargo update
```

Since this is build around lambdas using [ZEIT's now](https://zeit.co/now), there isn't currently a way to test this locally. They're working on it, however. Until then, submit a PR to this repo and it will automatically build a version and deploy it for you.

Finally, this project uses the awesome `rustfmt` crate for automatic formatting of rust files. To install it, use Cargo. If you don't want it in your IDE, you can run it from the command line.

```sh
# install rustfmt
cargo install rustfmt

# format the rust file
rustfmt src/main.rs
```

# [Changelog](CHANGELOG.md)

# [Code of Conduct](CODE_OF_CONDUCT.md)

# [License (MIT)](LICENSE.md)
