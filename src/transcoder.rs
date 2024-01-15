use std::io::{Read, Write};
use crate::{error::RsvError, args::Args};


const RSV_VALUE_TERMINATOR: u8 = 0xFF;
const RSV_NULL_VALUE: u8 = 0xFE;
const RSV_ROW_TERMINATOR: u8 = 0xFD;
const ASCII_LF: u8 = b'\n';

#[derive(Clone, Debug)]
struct ParserState {
    line_begin: bool,
    value_begin: bool,
    value_end: bool,
    value_separator: bool,
    content: bool,
    null: bool,
    line_end: bool,
}

impl ParserState {
    fn process_char<W: Write>(
        &mut self,
        args: &Args,
        writer: &mut W,
        file_name: &str,
        byte: u8,
        next: u8,
    ) -> Result<(), RsvError> {
        match byte {
            RSV_NULL_VALUE => self.null = true,
            RSV_VALUE_TERMINATOR => self.value_separator = true,
            RSV_ROW_TERMINATOR | ASCII_LF => self.line_end = true,
            _ => self.content = true,
        }

        if self.line_begin {
            self.line_begin = false;
            process_write_bytes(&mut *writer, file_name, args.line_starting.as_bytes())?;

            if byte != RSV_ROW_TERMINATOR && byte != ASCII_LF {
                self.value_begin = true;
            }
        }

        if self.value_begin {
            self.value_begin = false;

            if byte != RSV_NULL_VALUE {
                process_write_bytes(&mut *writer, file_name, args.field_opening.as_bytes())?;

                if matches!(next, RSV_VALUE_TERMINATOR | RSV_NULL_VALUE | RSV_ROW_TERMINATOR) {
                    self.value_end = true;
                }
            }
        }

        if self.content {
            self.content = false;
            process_write_bytes(&mut *writer, file_name, &[byte])?;

            if matches!(next, RSV_VALUE_TERMINATOR | RSV_ROW_TERMINATOR | ASCII_LF) {
                self.value_end = true;
            }
        }

        if self.value_end {
            self.value_end = false;
            process_write_bytes(&mut *writer, file_name, args.field_closing.as_bytes())?;
        }

        if self.value_separator {
            if next == RSV_VALUE_TERMINATOR {
                self.value_end = true;
            }

            if next != RSV_ROW_TERMINATOR && next != ASCII_LF {
                process_write_bytes(&mut *writer, file_name, args.field_separator.as_bytes())?;
                self.value_begin = true;
            }

            self.value_separator = false;
        }

        if self.null {
            self.null = false;
            process_write_bytes(&mut *writer, file_name, args.null_value.as_bytes())?;
        }

        if self.line_end {
            self.line_end = false;
            process_write_bytes(&mut *writer, file_name, args.line_ending.as_bytes())?;
            process_write_bytes(&mut *writer, file_name, &[ASCII_LF])?;
            self.line_begin = true;
        }

        Ok(())
    }
}

fn process_write_bytes<W: Write>(mut writer: W, name: &str, bytes: &[u8]) -> Result<(), RsvError> {
    writer.write_all(bytes).map_err(|source| RsvError::write(name.to_owned(), source))
}

impl Default for ParserState {
    fn default() -> Self {
        ParserState {
            line_begin: true,
            value_begin: false,
            value_end: false,
            value_separator: false,
            content: false,
            null: false,
            line_end: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RsvTranscoder<R, W> {
    parser_state: ParserState,
    reader: R,
    writer: W,
    args: Args,
    file_name: String,
}

impl<R, W> RsvTranscoder<R, W> {
    pub fn new(reader: R, writer: W, args: Args, file_name: String) -> Self {
        RsvTranscoder {
            parser_state: ParserState::default(),
            reader,
            writer,
            args,
            file_name,
        }
    }
}

impl<R: Read, W: Write> RsvTranscoder<R, W> {
    pub fn process(&mut self) -> Result<(), RsvError> {
        let mut first = true;
        let mut byte = 0;

        for byte_read in self.reader.by_ref().bytes() {
            let next = byte_read.map_err(|source| RsvError::read(self.file_name.clone(), source))?;

            if next == b'\r' {
                continue;
            }

            if !first {
                self.parser_state.process_char(&self.args, &mut self.writer, &self.file_name, byte, next)?;
            }

            first = false;
            byte = next;
        }

        self.parser_state.process_char(&self.args, &mut self.writer, &self.file_name, byte, 0)
    }
}
