use clap::Parser;
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

/// Show RSV files - https://github.com/ern0/cat-rsv/
#[derive(Parser, Debug)]
struct StringArgs {

    /// Custom NULL value
    #[arg(short = 'n', long)]
    null_value: Option<String>,

    /// Custom field separator
    #[arg(short = 'f', long)]
    field_separator: Option<String>,

    /// Custom line beginning
    #[arg(short = 'b', long)]
    line_beginning: Option<String>,

    /// Custom line ending
    #[arg(short = 'e', long)]
    line_ending: Option<String>,

    /// List of RSV (or other) files to print
    files: Vec<String>,
}

struct ByteArgs {
    null_value: Vec<u8>,
    field_separator: Vec<u8>,
    line_beginning: Vec<u8>,
    line_ending: Vec<u8>,
}

fn main() {

    let mut string_args = StringArgs::parse();
    cleanup_args(&mut string_args);
    let byte_args = convert_args(&string_args);

    if string_args.files.is_empty() {
        process(std::io::stdin(), &byte_args, "(stdin)");
    }

    for fnam in &string_args.files {
        match File::open(fnam) {
            Err(reason) => {
                println!("error opening {}: {}", &fnam, reason);
                exit(EXIT_READ_PROBLEM);
            }
            Ok(file) => process(file, &byte_args, fnam),
        }
    }
}

fn cleanup_args(args: &mut StringArgs) {

    if args.null_value.is_none() {
        args.null_value = Some(String::from("(null)"));
    }

    if args.field_separator.is_none() {
        args.field_separator = Some(String::from("|"));
    }
    if args.line_beginning.is_none() {
        args.line_beginning = Some(String::from("["));
    }
    if args.line_ending.is_none() {
        args.line_ending = Some(String::from("]"));
    }

    args.files.retain(|value| *value != "-");
    args.files.retain(|value| *value != "--");
}

fn convert_args(string_args: &StringArgs) -> ByteArgs {

    ByteArgs {
        null_value: string_args.null_value.clone().unwrap().into_bytes(),
        field_separator: string_args.field_separator.clone().unwrap().into_bytes(),
        line_beginning: string_args.line_beginning.clone().unwrap().into_bytes(),
        line_ending: string_args.line_ending.clone().unwrap().into_bytes(),
    }
}

fn process(file: impl Read, args: &ByteArgs, fnam: &str) {

    let mut line_begins = true;
    let mut maybe_value_separator = false;

    for byte_read in file.bytes() {
        let byte = process_read_byte(fnam, byte_read);

        if line_begins {
            process_write_bytes(&args.line_beginning);
            line_begins = false;
        }

        match byte {
            RSV_NULL_VALUE => {
                process_write_bytes(&args.null_value);
            },
            RSV_VALUE_TERMINATOR => {
                if maybe_value_separator {
                    process_write_bytes(&args.field_separator);
                }
                maybe_value_separator = true;
            },
            RSV_ROW_TERMINATOR | ASCII_LF => {
                line_begins = true;
                maybe_value_separator = false;
                process_write_bytes(&args.line_ending);
                process_write_bytes(&vec!(ASCII_LF));
            },
            ASCII_CR => {
                ()
            },
            _ => {
                if maybe_value_separator {
                    maybe_value_separator = false;
                    process_write_bytes(&args.field_separator);
                }
                process_write_bytes(&vec![byte]);
            }
        }
    }
}

fn process_read_byte(fnam: &str, byte_read: Result<u8, std::io::Error>) -> u8 {

    match byte_read {
        Ok(byte) => byte,
        Err(reason) => {
            println!("error reading {}: {}", &fnam, reason);
            exit(EXIT_READ_PROBLEM);
        },
    }
}

fn process_write_bytes(bytes: &Vec<u8>) {

    if let Err(reason) = std::io::stdout().write_all(bytes) {
        println!("error writing <stdout>: {}", reason);
        exit(EXIT_WRITE_PROBLEM);
    }
}
