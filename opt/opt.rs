//! Options for running `svelte`.

#![deny(missing_debug_implementations)]

#[macro_use]
extern crate failure;

#[macro_use]
extern crate structopt;

use std::fs;
use std::io;
use std::path;
use std::str::FromStr;

/// Options for controlling `svelte`.
#[derive(Clone, Debug, StructOpt)]
pub enum Options {
    #[structopt(name = "top")]
    Top(Top),
}

/// Options that are common to all commands.
pub trait CommonOptions {
    /// Get the input file path.
    fn input(&self) -> &path::Path;

    /// Get the output destination.
    fn output_destination(&self) -> &OutputDestination;

    /// Get the output format.
    fn output_format(&self) -> OutputFormat;
}

impl CommonOptions for Options {
    fn input(&self) -> &path::Path {
        match *self {
            Options::Top(ref top) => top.input(),
        }
    }

    fn output_destination(&self) -> &OutputDestination {
        match *self {
            Options::Top(ref top) => top.output_destination(),
        }
    }

    fn output_format(&self) -> OutputFormat {
        match *self {
            Options::Top(ref top) => top.output_format(),
        }
    }
}

/// List the top code size offenders in a binary.
#[derive(Clone, Debug, StructOpt)]
pub struct Top {
    /// The path to the input binary to size profile.
    #[structopt(parse(from_os_str))]
    pub input: path::PathBuf,

    /// The destination to write the output to. Defaults to `stdout`.
    #[structopt(short = "o", default_value = "-")]
    pub output_destination: OutputDestination,

    /// The format the output should be written in.
    #[structopt(short = "f", long = "format", default_value = "text")]
    pub output_format: OutputFormat,

    /// The maximum number of items to display.
    #[structopt(short = "n")]
    pub number: Option<u32>,

    /// Display retaining paths.
    #[structopt(short = "r", long = "retaining-paths")]
    pub retaining_paths: bool,

    /// Choose how to sort the list. Choices are "shallow" or "retained".
    #[structopt(short = "s", long = "sort-by", default_value = "shallow")]
    pub sort_by: SortBy,
}

impl CommonOptions for Top {
    fn input(&self) -> &path::Path {
        &self.input
    }

    fn output_destination(&self) -> &OutputDestination {
        &self.output_destination
    }

    fn output_format(&self) -> OutputFormat {
        self.output_format
    }
}

/// Whether to sort by shallow or retained size.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SortBy {
    Shallow,
    Retained,
}

impl FromStr for SortBy {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<SortBy, failure::Error> {
        match s {
            "shallow" => Ok(SortBy::Shallow),
            "retained" => Ok(SortBy::Retained),
            _ => bail!("unknown sort order: '{}'", s),
        }
    }
}

/// Where to output results.
#[derive(Clone, Debug)]
pub enum OutputDestination {
    /// Emit the results to `stdout`.
    Stdout,

    /// Write the results to a file at the given path.
    Path(path::PathBuf),
}

impl Default for OutputDestination {
    fn default() -> OutputDestination {
        OutputDestination::Stdout
    }
}

impl FromStr for OutputDestination {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, failure::Error> {
        if s == "-" {
            Ok(OutputDestination::Stdout)
        } else {
            let path = path::PathBuf::from(s.to_string());
            Ok(OutputDestination::Path(path))
        }
    }
}

impl OutputDestination {
    /// Open the output destination as an `io::Write`.
    pub fn open(&self) -> Result<Box<io::Write>, failure::Error> {
        Ok(match *self {
            OutputDestination::Path(ref path) => {
                Box::new(io::BufWriter::new(fs::File::open(path)?)) as Box<io::Write>
            }
            OutputDestination::Stdout => Box::new(io::stdout()) as Box<io::Write>,
        })
    }
}

/// The format of the output.
#[derive(Clone, Copy, Debug)]
pub enum OutputFormat {
    /// Human readable text.
    Text,
    // /// Hyper Text Markup Language.
    // Html,

    // /// Graphviz dot format.
    // Dot,

    // /// Comma-separated values (CSV) format.
    // Csv,

    // /// JavaScript Object Notation format.
    // Json,
}

impl Default for OutputFormat {
    fn default() -> OutputFormat {
        OutputFormat::Text
    }
}

impl FromStr for OutputFormat {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, failure::Error> {
        match s {
            "text" => Ok(OutputFormat::Text),
            _ => bail!("Unknown output format: {}", s),
        }
    }
}
