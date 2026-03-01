# Description
A CLI tool for embedding [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code) properly into text. For example, you may have some text `\x1b[1;31mThis should be bold and red!` which has the bytes:

| \ | 1 | b | [ | 1 | ; | ... |

This would not properly render since the ansii escape code is not stored as the escape code byte `\x1b`. This tool will convert the above text into:

| \x1b | [ | 1 | ; | ... |

which would properly render in the terminal.

# Install

## Prerequisites
 - [Cargo](https://rust-lang.org/tools/install/)
 - [Git](https://git-scm.com/install/)

`cargo install --git https://github.com/tomna1/anbed`

