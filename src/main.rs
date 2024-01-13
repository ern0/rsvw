use clap::Parser;
use std::fs::File;
use std::io::{Read, Write};

/// Print RSV files - https://github.com/ern0/cat-rsv/
#[derive(Parser, Debug)]
struct CatArgs {
    /// Custom field separator
    #[arg(short = 'f', long)]
    field_separator: Option<String>,

    /// Custom field beginning
    #[arg(short = 'b', long)]
    field_beginning: Option<String>,

    /// Custom field ending
    #[arg(short = 'e', long)]
    field_ending: Option<String>,

    /// Custom line beginning
    #[arg(short = 'a', long)]
    line_beginning: Option<String>,

    /// Custom line ending
    #[arg(short = 'z', long)]
    line_ending: Option<String>,

    /// Number all output lines
    #[arg(short = 'n', long)]
    number: bool,

    /// List of RSV (or other) files to print
    files: Vec<String>,
}

fn main() {
    let mut args = CatArgs::parse();
    cleanup_args(&mut args);

    if args.files.is_empty() {
        process(std::io::stdin(), &args);
    }

    for fnam in &args.files {
        match File::open(fnam) {
            Err(reason) => println!("error opening {}: {}", &fnam, reason),
            Ok(file) => process(file, &args),
        }
    }
}

fn cleanup_args(args: &mut CatArgs) {
    args.files.retain(|x| *x != "-");
    args.files.retain(|x| *x != "--");

    if args.field_beginning.is_none()
        && args.field_ending.is_none()
        && args.field_separator.is_none()
    {
        args.field_separator = Some(String::from(", "));
    }
}

fn process(file: impl Read, args: &CatArgs) {

    for byte in file.bytes() {
        std::io::stdout().write_all(&[byte.unwrap()]).unwrap();
    }
}
