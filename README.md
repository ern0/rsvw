# rsvw - RSV viewer 

## What is `rsvw`?

This simple CLI utility 
converts
RSV (Rows of String Values) files
to human readable format.

## What is RSV?

RSV is a schemeless text-based data transport format,
very similar to CSV, but with binary separator characters,
so there's no quotation marks, no escaping at all.
Easy to create, easy to read, 
payload can be any UTF-8 character.

For more info and examples,
see the [specification](https://github.com/Stenway/RSV-Specification).

## Usage

If any files are specified, 
the program concatenates them, 
if not, it uses `stdin` (piping support).

```
rsvw 1.0.0 - RSV viewer - https://github.com/ern0/rsvw"

  Usage: rsvw [options] [files]...

  Options:
    -n, --null-value         default: "null"
    -f, --field-separator    default: "|"
    -o, --field-opening      default: "<"
    -c, --field-closing      default: ">"
    -s, --line-starting      default: "["
    -e, --line-ending        default: "]"
    -h, --help

    How to set empty string value to an option:
      rsvw -f'' -- example.rsv
```
The empty string (`''`) is unnecessary, 
but it's easier to understand.

## Build and install

You will need [Rust compiler installed](https://www.rust-lang.org/learn/get-started).

Build:
```
$ cargo build --release
(...)
```

Install:
```
$ sudo cp target/release/rsvw /usr/local/bin
```

## Plans

There's no such as final version of a software.

### Make ASCII input support optional

Now, this utility omits all CR characters,
and takes LFs as end-of-line.
Probably, this is a bad behaviour,
these characters may appear in the payload.

Solution: add command line switch to
interpret CR and LF characters as data.

### Make ASCII output optional

Now the program prints LF after each line,
regardless of *line ending* parameter's value.

Solution: make it switchable.

### Use configuration file 

Delimiter characters 
should be read 
from a user preference file,
e.g. `~/.rsvw-rc`.
