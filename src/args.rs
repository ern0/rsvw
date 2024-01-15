use clap::Parser;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    #[clap(short = 'n', long = "null-value", default_value = "null")]
    pub null_value: String,
    #[clap(short = 'f', long = "field-separator", default_value = "|")]
    pub field_separator: String,
    #[clap(short = 'o', long = "field-opening", default_value = "<")]
    pub field_opening: String,
    #[clap(short = 'c', long = "field-closing", default_value = "<")]
    pub field_closing: String,
    #[clap(short = 's', long = "line-starting", default_value = "[")]
    pub line_starting: String,
    #[clap(short = 'e', long = "line-ending", default_value = "]")]
    pub line_ending: String,
    pub files: Vec<String>,
}
