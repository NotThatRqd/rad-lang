# rad lang
rad lang is my personal attempt at creating something similar to
YAML or TOML.

Right now if you run it, it will just read from
`resources/test/test_1.rad_lang` but eventually it will be able to
read any file.

The file extension is `.rad_lang` (subject to change)
and the file is expected to be UTF-8 encoded.

# Syntax
```
[version 0.1.0]
// this is a comment
// every file must start by declaring the version in use

[main]
// this is inside the main namespace

    [main:foo]
    // this is inside the sub namespace main:foo
    // you can indent, but it is not needed
```