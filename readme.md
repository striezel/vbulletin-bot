# vBulletin moderation bot

This repository contains an attempt for an moderation bot for use with the
vBulletin forum API. It is still in an exploratory proof of concept stage and
does not provide much useful features (yet).

## Building the application from source

### Prerequisites

To build the application you need the Rust compiler. The Minimum Supported Rust
Version (MSRV) is Rust 1.47. Furthermore, you need Cargo (Rust's package
manager), the development libraries for OpenSSL, and pkg-config.

It also helps to have Git, a distributed version control system, on your system
to get the latest source code directly from the Git repository.

All of that can usually be installed be typing

    # Debian-based Linux distribution
    apt-get install cargo git libssl-dev pkg-config rustc

or

    # CentOS 8
    yum install cargo git rust openssl-devel

or

    # Alpine
    apk add cargo git rust openssl-dev pkgconfig

into a root terminal.

### Getting the source code

Get the source directly from Git by cloning the Git repository and change to
the directory after the repository is completely cloned:

    git clone https://gitlab.com/striezel/vbulletin-bot.git
    cd vbulletin-bot

That's it, you should now have the current source code on your machine.

### Build process

The build process is relatively easy, because Cargo can handle that for you.
Starting in the root directory of the source, you can invoke the following
command in a terminal to build the application:

    cargo build

Or, if you want the optimized release version, type

    cargo build --release

instead.

That's it. It may take a minute for Cargo to download the dependencies and
compile them, but after that you are ready to start using the application.

## Using the application

Currently, the bot only supports login via the vBulletin API and no more real
actions like showing threads or similar stuff that will follow later.

## Copyright and Licensing

Copyright 2021  Dirk Stolle

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
