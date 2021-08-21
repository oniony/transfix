![TRANSFIX](https://raw.githubusercontent.com/oniony/transfix/master/graphics/transfix.png)

[![Build Status](https://app.travis-ci.com/oniony/transfix.svg?branch=master)

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

    $ transfix decode </some/file
    OR
    $ cat /some/file | transfix decode

## Options

* `--id` ― show the original identifiers for decode tags and values
* `--color` ― color the tags and values
* `--tag-per-line` ― show each tag on a new line

# About

Transfix is written and maintained by Paul Ruane (<paul.ruane@oniony.com>) and is available at <http://github.com/oniony/transfix/>.

Transfix is written in Rust: <http://rust-lang.org/>

- - -

Copyright 2017‒2021 Paul Ruane

Copying and distribution of this file, with or without modification,
are permitted in any medium without royalty provided the copyright
notice and this notice are preserved.  This file is offered as-is,
without any warranty.
