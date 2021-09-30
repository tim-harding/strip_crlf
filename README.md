# Strip CRLF

Replaces escaped or unescaped `CRLF` line endings with `LF` endings.

## Installation

With a Rust toolchain installed, run

`cargo install --git https://github.com/tim-harding/strip_crlf`

Alternatively, download a Windows or Ubuntu executable from the [releases](https://github.com/tim-harding/strip_crlf/releases/).

## Use

To replace unescaped line endings from some `file.txt`, use

`cat ./file.txt | strip_crlf > file.txt`

To replace escaped line endings, use

`cat ./file.txt | strip_crlf --escaped > file.txt`