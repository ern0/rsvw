use std::io;
use std::fs::File;
use std::process::ExitCode;
use clap::Parser;
use crate::{args::Args, error::RsvError, transcoder::RsvTranscoder};

mod args;
mod error;
mod transcoder;


fn main() -> ExitCode {
    let args = Args::parse();
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    if args.files.is_empty() {
        let mut transcoder = RsvTranscoder::new(stdin, stdout, args, String::from("stdin"));

        if let Err(error) = transcoder.process() {
            return error.print_and_report();
        }

        return ExitCode::SUCCESS;
    }

    for path in &args.files {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(error) => {
                return RsvError::open(path.to_owned(), error).print_and_report();
            }
        };
        let mut transcoder = RsvTranscoder::new(file, &mut stdout, args.clone(), path.to_owned());

        if let Err(error) = transcoder.process() {
            return error.print_and_report();
        }
    }

    ExitCode::SUCCESS
}
