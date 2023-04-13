![Build and test](https://github.com/Skallwar/suckit/workflows/Build%20and%20test/badge.svg)
[![codecov](https://codecov.io/gh/Skallwar/suckit/branch/master/graph/badge.svg?token=ZLD369AY2G)](https://codecov.io/gh/Skallwar/suckit)
[![Crates.io](https://img.shields.io/crates/v/suckit.svg)](https://crates.io/crates/suckit)
[![Docs](https://docs.rs/suckit/badge.svg)](https://docs.rs/suckit)
[![Deps](https://deps.rs/repo/github/Skallwar/suckit/status.svg)](https://deps.rs/repo/github/Skallwar/suckit)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![MSRV](https://img.shields.io/badge/MSRV-1.63.0-blue)

# SuckIT

`SuckIT` allows you to recursively visit and download a website's content to
your disk.

![SuckIT Logo](media/suckit_logo.png)

# Features

* [x] Vacuums the entirety of a website recursively
* [x] Uses multithreading
* [x] Writes the website's content to your disk
* [x] Enables offline navigation
* [x] Offers random delays to avoid IP banning
* [ ] Saves application state on CTRL-C for later pickup

# Options
```console
USAGE:
    suckit [FLAGS] [OPTIONS] <url>

FLAGS:
    -c, --continue-on-error                  Flag to enable or disable exit on error
        --dry-run                            Do everything without saving the files to the disk
    -h, --help                               Prints help information
    -V, --version                            Prints version information
    -v, --verbose                            Enable more information regarding the scraping process
        --visit-filter-is-download-filter    Use the dowload filter in/exclude regexes for visiting as well

OPTIONS:
    -a, --auth <auth>...
            HTTP basic authentication credentials space-separated as "username password host". Can be repeated for
            multiple credentials as "u1 p1 h1 u2 p2 h2"
        --delay <delay>
            Add a delay in seconds between downloads to reduce the likelihood of getting banned [default: 0]

    -d, --depth <depth>
            Maximum recursion depth to reach when visiting. Default is -1 (infinity) [default: -1]

    -e, --exclude-download <exclude-download>
            Regex filter to exclude saving pages that match this expression [default: $^]

        --exclude-visit <exclude-visit>
            Regex filter to exclude visiting pages that match this expression [default: $^]

        --ext-depth <ext-depth>
            Maximum recursion depth to reach when visiting external domains. Default is 0. -1 means infinity [default:
            0]
    -i, --include-download <include-download>
            Regex filter to limit to only saving pages that match this expression [default: .*]

        --include-visit <include-visit>
            Regex filter to limit to only visiting pages that match this expression [default: .*]

    -j, --jobs <jobs>                            Maximum number of threads to use concurrently [default: 1]
    -o, --output <output>                        Output directory
        --random-range <random-range>
            Generate an extra random delay between downloads, from 0 to this number. This is added to the base delay
            seconds [default: 0]
    -t, --tries <tries>                          Maximum amount of retries on download failure [default: 20]
    -u, --user-agent <user-agent>                User agent to be used for sending requests [default: suckit]

ARGS:
    <url>    Entry point of the scraping
```

# Example

A common use case could be the following:

`suckit http://books.toscrape.com -j 8 -o /path/to/downloaded/pages/`

![asciicast](media/suckit-adjusted-120cols-40rows-100ms.svg)

# Installation

As of right now, `SuckIT` does not work on Windows.

To install it, you need to have Rust installed.

* Check out [this link](https://www.rust-lang.org/learn/get-started) for
instructions on how to install Rust.

* If you just want to install the suckit executable, you can simply run
`cargo install --git https://github.com/skallwar/suckit`

* Now, run it from anywhere with the `suckit` command.

### Arch Linux

`suckit` can be installed from available [AUR packages](https://aur.archlinux.org/packages/?O=0&SeB=b&K=suckit&outdated=&SB=n&SO=a&PP=50&do_Search=Go) using an [AUR helper](https://wiki.archlinux.org/index.php/AUR_helpers). For example,

```
yay -S suckit
```

__Want to contribute ? Feel free to
[open an issue](https://github.com/Skallwar/suckit/issues/new) or
[submit a PR](https://github.com/Skallwar/suckit/compare) !__

# License

SuckIT is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0)

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
