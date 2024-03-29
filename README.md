![TRANSFIX](https://raw.githubusercontent.com/oniony/transfix/main/graphics/transfix.png)

[![Build Status](https://github.com/oniony/transfix/actions/workflows/build.yml/badge.svg)](https://github.com/oniony/transfix/actions/workflows/build.yml)

# Overview

Transfix is a command-line utility to translate FIX protocol
messages into a more readable form.

# Compilation

* Install Rust from <https://www.rust-lang.org/>
* Build Transfix:

      $ git clone git@github.com:oniony/transfix.git
      $ cd transfix
      $ cargo build
    
# Usage

    $ transfix </some/file
    OR
    $ cat /some/file | transfix

## Options

* `--id` ― show the original identifiers for decode tags and values
* `--color` ― color the tags and values
* `--tag-per-line` ― show each tag on a new line

# Versions

## v1.0.0

* First published release

# About

Transfix is written and maintained by Paul Ruane (<paul.ruane@oniony.com>) and is available at <http://github.com/oniony/transfix/>.

Transfix is written in Rust: <http://rust-lang.org/>

- - -

Copyright 2017‒2023 Paul Ruane

Copying and distribution of this file, with or without modification,
are permitted in any medium without royalty provided the copyright
notice and this notice are preserved.  This file is offered as-is,
without any warranty.
