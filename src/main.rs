use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;

const EXIT_NO_PROBLEM: i32 = 0;
const EXIT_READ_PROBLEM: i32 = 1;
const EXIT_WRITE_PROBLEM: i32 = 2;

const RSV_VALUE_TERMINATOR: u8 = 0xFF;
const RSV_NULL_VALUE: u8 = 0xFE;
const RSV_ROW_TERMINATOR: u8 = 0xFD;
const ASCII_CR: u8 = 0x0D;
const ASCII_LF: u8 = 0x0A;

fn main() {

    let (mut args, argv,) = argmap::parse(std::env::args());
    args.remove(0);

    let mut string_args = default_args();
    
    if argv.contains_key("h") || argv.contains_key("help") {
        about(&string_args);
        exit(EXIT_NO_PROBLEM);
    }

    parse_arg(&argv, &mut string_args.null_value, "n", "null-value");
    parse_arg(&argv, &mut string_args.field_separator, "f", "field-separator");
    parse_arg(&argv, &mut string_args.field_opening, "o", "field-opening");
    parse_arg(&argv, &mut string_args.field_closing, "c", "field-closing");
    parse_arg(&argv, &mut string_args.line_starting, "s", "line-starting");
    parse_arg(&argv, &mut string_args.line_ending, "e", "line-ending");

    let byte_args = convert_args(string_args);
    
    if args.is_empty() {
        process(std::io::stdin(), &byte_args, "(stdin)");
    }

    for fnam in args {
        match File::open(&fnam) {
            Err(reason) => {
                println!("error opening {}: {}", &fnam, reason);
                exit(EXIT_READ_PROBLEM);
            }
            Ok(file) => process(file, &byte_args, &fnam),
        }
    }
}

fn parse_arg(argv: &HashMap<String, Vec<String>>, value: &mut String, short: &str, long: &str) {

    for key_slice in [short, long].iter() {
        
        let key = &key_slice.to_string();

            if let Some(found) = argv.get(key) {
                if found.is_empty() {
                    *value = "".to_string();
                } else {
                    *value = found.first().unwrap().to_string();
                }
        }

    }
}

struct StringArgs {
    null_value: String,
    field_separator: String,
    field_opening: String,
    field_closing: String,
    line_starting: String,
    line_ending: String,
}

fn default_args() -> StringArgs {
     StringArgs {
        null_value: String::from("null"),
        field_separator: String::from("|"),
        field_opening: String::from("<"),
        field_closing: String::from(">"),
        line_starting: String::from("["),
        line_ending: String::from("]"),
    }
}

struct ByteArgs {
    null_value: Vec<u8>,
    field_separator: Vec<u8>,
    field_opening: Vec<u8>,
    field_closing: Vec<u8>,
    line_starting: Vec<u8>,
    line_ending: Vec<u8>,
}

fn convert_args(string_args: StringArgs) -> ByteArgs {
    ByteArgs {
        null_value: Vec::from(string_args.null_value),
        field_separator: Vec::from(string_args.field_separator),
        field_opening: Vec::from(string_args.field_opening),
        field_closing: Vec::from(string_args.field_closing),
        line_starting: Vec::from(string_args.line_starting),
        line_ending: Vec::from(string_args.line_ending),
    }
}

fn about(string_args: &StringArgs) {
    println!(
r###"rsvw {} - RSV viewer - https://github.com/ern0/rsvw"

  Usage: rsvw [options] [files]...

  Options:
    -n, --null-value         default: "{}"
    -f, --field-separator    default: "{}"
    -o, --field-opening      default: "{}"
    -c, --field-closing      default: "{}"
    -s, --line-starting      default: "{}" 
    -e, --line-ending        default: "{}" 
    -h, --help
"###,
            env!("CARGO_PKG_VERSION"),
            string_args.null_value,
            string_args.field_separator,
            string_args.field_opening,
            string_args.field_closing,
            string_args.line_starting,
            string_args.line_ending,
    );
}

struct ParserVars {
    line_begin: bool,
    value_begin: bool,
    value_end: bool,
    value_separator: bool,
    content: bool,
    null: bool,
    line_end: bool,
}

fn process(file: impl Read, args: &ByteArgs, fnam: &str) {

    let mut parser = ParserVars {
        line_begin: true,
        value_begin: false,
        value_end: false,
        value_separator: false,
        content: false,
        null: false,
        line_end: false,
    };

    let mut first = true;
    let mut byte = 0;
    for byte_read in file.bytes() {
        
        let next = process_read_byte(fnam, byte_read);
        if next == ASCII_CR { 
            continue; 
        }

        if !first {
            process_char(byte, next, args, &mut parser);
        }
        first = false;
        byte = next;
    }
    process_char(byte, 0, args, &mut parser);
}

fn process_char(byte: u8, next: u8, args: &ByteArgs, parser: &mut ParserVars) {

    match byte {
        RSV_NULL_VALUE => parser.null = true,
        RSV_VALUE_TERMINATOR => parser.value_separator = true,
        RSV_ROW_TERMINATOR | ASCII_LF => parser.line_end = true,
        _ => parser.content = true,
    }

    if parser.line_begin {
        parser.line_begin = false;
        process_write_bytes(&args.line_starting);
        if byte != RSV_ROW_TERMINATOR && byte != ASCII_LF {
            parser.value_begin = true;
        }
    }
    if parser.value_begin {
        parser.value_begin = false;
        if byte != RSV_NULL_VALUE {
            process_write_bytes(&args.field_opening);
            if next == RSV_VALUE_TERMINATOR || next == RSV_NULL_VALUE || next == RSV_ROW_TERMINATOR {
                parser.value_end = true;
            }
        }
    }
    if parser.content {
        parser.content = false;
        process_write_bytes(&[byte]);
        if next == RSV_VALUE_TERMINATOR || next == RSV_ROW_TERMINATOR || next == ASCII_LF {
            parser.value_end = true;
        }
    }
    if parser.value_end {
        parser.value_end = false;
        process_write_bytes(&args.field_closing);
    }
    if parser.value_separator {
        if next == RSV_VALUE_TERMINATOR {
            parser.value_end = true;
        }
        if next != RSV_ROW_TERMINATOR && next != ASCII_LF {
            process_write_bytes(&args.field_separator);
            parser.value_begin = true;
        }
        parser.value_separator = false;
    }
    if parser.null {
        parser.null = false;
        process_write_bytes(&args.null_value);
    }
    if parser.line_end {
        parser.line_end = false;
        process_write_bytes(&args.line_ending);
        process_write_bytes(&[ASCII_LF]);
        parser.line_begin = true;
    }
}

fn process_read_byte(fnam: &str, byte_read: Result<u8, std::io::Error>) -> u8 {
    match byte_read {
        Ok(byte) => byte,
        Err(reason) => {
            println!("error reading {}: {}", &fnam, reason);
            exit(EXIT_READ_PROBLEM);
        }
    }
}

fn process_write_bytes(bytes: &[u8]) {
    if let Err(reason) = std::io::stdout().write_all(bytes) {
        println!("error writing <stdout>: {}", reason);
        exit(EXIT_WRITE_PROBLEM);
    }
}
