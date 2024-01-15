use std::process::{ExitCode, Termination};
use std::io::Error as IoError;
use thiserror::Error;


#[derive(Debug, Error)]
#[error("in `{file_name}`: {kind} ({source})")]
pub struct RsvError {
    pub kind: RsvErrorKind,
    pub file_name: String,
    #[source]
    pub source: IoError,
}

impl RsvError {
    pub fn print_and_report(self) -> ExitCode {
        eprintln!("{self}");
        self.report()
    }

    pub fn read(file_name: String, source: IoError) -> Self {
        RsvError { kind: RsvErrorKind::Read, file_name, source }
    }

    pub fn write(file_name: String, source: IoError) -> Self {
        RsvError { kind: RsvErrorKind::Write, file_name, source }
    }

    pub fn open(file_name: String, source: IoError) -> Self {
        RsvError { kind: RsvErrorKind::Open, file_name, source }
    }
}

impl Termination for RsvError {
    fn report(self) -> ExitCode {
        self.kind.report()
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Error)]
pub enum RsvErrorKind {
    #[error("reading error")]
    Read = 1,
    #[error("writing error")]
    Write = 2,
    #[error("opening error")]
    Open = 3,
}

impl Termination for RsvErrorKind {
    fn report(self) -> ExitCode {
        ExitCode::from(self as u8)
    }
}
