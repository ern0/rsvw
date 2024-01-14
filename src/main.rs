use argmap;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;

const EXIT_READ_PROBLEM: i32 = 1;
const EXIT_WRITE_PROBLEM: i32 = 2;

const RSV_VALUE_TERMINATOR: u8 = 0xFF;
const RSV_NULL_VALUE: u8 = 0xFE;
const RSV_ROW_TERMINATOR: u8 = 0xFD;
const ASCII_CR: u8 = 0x0D;
const ASCII_LF: u8 = 0x0A;

struct ByteArgs {
    null_value: Vec<u8>,
    field_separator: Vec<u8>,
    field_opening: Vec<u8>,
    field_closing: Vec<u8>,
    line_starting: Vec<u8>,
    line_ending: Vec<u8>,
}

fn main() {

    let (mut args, argv,) = argmap::parse(std::env::args());
    args.remove(0);

    eprintln!["args={:?}", &args];
    eprintln!["argv={:?}", &argv];

     let mut byte_args = ByteArgs {
        null_value: Vec::from("null"),
        field_separator: Vec::from("|"),
        field_opening: Vec::from("<"),
        field_closing: Vec::from(">"),
        line_starting: Vec::from("["),
        line_ending: Vec::from("]"),
    };

    //...

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
        process_write_bytes(&vec![byte]);
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
        process_write_bytes(&vec![ASCII_LF]);
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

fn process_write_bytes(bytes: &Vec<u8>) {
    if let Err(reason) = std::io::stdout().write_all(bytes) {
        println!("error writing <stdout>: {}", reason);
        exit(EXIT_WRITE_PROBLEM);
    }
}
