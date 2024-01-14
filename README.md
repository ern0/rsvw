# rsvw - RSV viewer 

## What is `rsvw`?

This is a simple 
RSV (Rows of String Values) 
file viewer and filter
command-line utility
for Unix-like platforms, 
like GNU/Linux, MacOs etc.

RSV in, human readable text out.

## What is RSV?

RSV is a schemeless text-based data transport format,
very similar to CSV, but with binary separator characters,
so there's no quotation marks, no escaping at all.
Easy to create, easy to read, 
payload can be any UTF-8 character.

For more info and examples,
see the (specification)[https://github.com/Stenway/RSV-Specification].

## Usage

This little utility can be used 
ca. like `cat`: 
if files are specified, concatenates them, 
if not, uses `stdin`, can be used for piping.

```
rsvw [OPTIONS] [FILES]...

Arguments:
  [FILES]...  List of RSV (or other) files to print

Options:
  -n, --null-value <NULL_VALUE>            Set NULL value - "null"
  -f, --field-separator <FIELD_SEPARATOR>  Set field separator - "|"
  -o, --field-opening <FIELD_OPENING>      Set field opening - "<"
  -c, --field-closing <FIELD_CLOSING>      Set field closing - ">"
  -s, --line-starting <LINE_STARTING>      Set line starting - "["
  -e, --line-ending <LINE_ENDING>          Set line ending - "]"
  -h, --help                               Print help
  -V, --version                            Print version
```

## Plans

There's no such as final version of a software.

### Make ASCII support optional

Now, this utility omits all CR characters,
and takes LFs as end-of-line.
Probably, this is a bad behaviour,
these characters should be in the payload.

Solution: add command line switch to
interpret these characters as data.

### Use configuration file 

Delimiter characters 
should be read 
from a user preference file,
e.g. `~/.rsvw-rc`.

### Technical: use smaller arg parser

I'm very disappointed with the `clap` library.
E.g. derive API does not support empty values, wtf.
