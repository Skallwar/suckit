![Build and test](https://github.com/Skallwar/suckit/workflows/Build%20and%20test/badge.svg)

# SuckIT

`SuckIT` allows you to recursively visit and download a website's content to
your disk.

![SuckIT Logo](suckit_logo.png)

# Features

* [x] Vacuums the entirety of a website recursively
* [x] Uses multithreading
* [x] Writes the website's content to your disk
* [x] Enables offline navigation
* [ ] Saves application state on CTRL-C for later pickup
* [ ] Offers random delays to avoid IP banning

# Options

|Option|Behavior|
|---|---|
|`-h, --help`|Displays help information|
|`-v, --verbose`|Activate Verbose output|
|`-d, --depth`|Specify the level of depth to go to when visiting the website|
|`-j, --jobs`|Number of threads to use|
|`-o, --output`|Output directory where the downloaded files are written|
|`-t, --tries`|Number of times to retry when the downloading of a page fails|

# Example

A common use case could be the following:

`suckit http://books.toscrape.com -j 8 -o /path/to/downloaded/pages/`

[![asciicast](https://asciinema.org/a/17XpBXaZhpIX41w7nRF6i3M9y.svg)](https://asciinema.org/a/17XpBXaZhpIX41w7nRF6i3M9y)

# Installation

As of right now, `SuckIT` does not work on Windows.

To install it, you need to have Rust installed.

* Check out [this link](https://www.rust-lang.org/learn/get-started) for
instructions on how to install Rust.

* Then, download the `suckit` repository by cloning it or downloading a zipped
version of it and enter the directory.

```shell
git clone https://github.com/skallwar/suckit
cd suckit
```

* You can now install SuckIT!

```shell
cargo install --path .
```

* Now, run it from anywhere with the `suckit` command.

__Want to contribute ? Feel free to
[open an issue](https://github.com/Skallwar/suckit/issues/new) or
[submit a PR](https://github.com/Skallwar/suckit/compare) !__

# License

SuckIT is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0)

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
