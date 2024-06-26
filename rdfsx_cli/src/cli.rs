use std::fmt::Display;
use std::{fmt::Formatter, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};
use srdf::RDFFormat;

#[derive(Parser, Debug)]
#[command(author, version, about)]
// #[command(name = "rdfsx")]
// #[command(author = "Jose Emilio Labra Gayo <labra@uniovi.es>")]
// #[command(version = "0.1")]
#[command(
    arg_required_else_help = true,
    long_about = r#"
 A tool to process and validate RDF data using shapes, and convert between different RDF data models"#
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Shapemap {
        #[arg(short = 'm', long = "shapemap", value_name = "ShapeMap file name")]
        shapemap: PathBuf,

        #[arg(
            long = "shapemap-format",
            value_name = "ShapeMap format",
            default_value_t = ShapeMapFormat::Compact
        )]
        shapemap_format: ShapeMapFormat,

        #[arg(
            long = "result-shapemap-format",
            value_name = "Result shapemap format",
            default_value_t = ShapeMapFormat::Compact
        )]
        result_shapemap_format: ShapeMapFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,
    },

    Schema {
        #[arg(short = 's', long = "schema", value_name = "Schema file name")]
        schema: PathBuf,

        #[arg(
            short = 'f',
            long = "schema-format",
            value_name = "Schema format",
            default_value_t = ShExFormat::ShExC
        )]
        schema_format: ShExFormat,

        #[arg(
            short = 'r',
            long = "result-schema-format",
            value_name = "Result schema format",
            default_value_t = ShExFormat::ShExJ
        )]
        result_schema_format: ShExFormat,

        #[arg(short = 't', long = "show elapsed time", default_value_t = false)]
        show_time: bool,

        #[arg(long = "statistics", default_value_t = false)]
        show_statistics: bool,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,
    },

    Validate {
        #[arg(short = 's', long = "schema", value_name = "Schema file name")]
        schema: PathBuf,

        #[arg(
            short = 'f',
            long = "schema-format",
            value_name = "Schema format",
            default_value_t = ShExFormat::ShExC
        )]
        schema_format: ShExFormat,

        #[arg(short = 'm', long = "shapemap", value_name = "ShapeMap file name")]
        shapemap: Option<PathBuf>,

        #[arg(
            long = "shapemap-format",
            value_name = "ShapeMap format",
            default_value_t = ShapeMapFormat::Compact,
        )]
        shapemap_format: ShapeMapFormat,

        #[arg(short = 'n', long = "node")]
        node: Option<String>,

        #[arg(
            short = 'l',
            long = "shape-label",
            value_name = "shape label (default = START)",
            group = "node_shape"
        )]
        shape: Option<String>,

        #[arg(short = 'd', long = "data", value_name = "RDF data path")]
        data: Option<PathBuf>,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        #[arg(short = 'e', long = "endpoint", value_name = "Endpoint with RDF data")]
        endpoint: Option<String>,

        #[arg(
            long = "max-steps",
            value_name = "max steps to run",
            default_value_t = 100
        )]
        max_steps: usize,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,
    },

    Data {
        #[arg(short = 'd', long = "data", value_name = "RDF data path")]
        data: PathBuf,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,
    },

    Node {
        #[arg(short = 'n', long = "node")]
        node: String,

        #[arg(short = 'd', long = "data", value_name = "RDF data path")]
        data: Option<PathBuf>,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        #[arg(short = 'e', long = "endpoint", value_name = "Endpoint with RDF data")]
        endpoint: Option<String>,

        #[arg(
            short = 'm',
            long = "show-node-mode",
            value_name = "Show Node Mode",
            default_value_t = ShowNodeMode::Outgoing
        )]
        show_node_mode: ShowNodeMode,

        #[arg(long = "show hyperlinks")]
        show_hyperlinks: bool,

        #[arg(short = 'p', long = "predicates")]
        predicates: Vec<String>,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,
    },

    Shacl {
        #[arg(short = 's', long = "shapes", value_name = "Shapes file name")]
        shapes: PathBuf,

        #[arg(
            short = 'f',
            long = "shapes-format",
            value_name = "Shapes file format",
            default_value_t = ShaclFormat::Turtle
        )]
        shapes_format: ShaclFormat,

        #[arg(
            short = 'r',
            long = "result-shapes-format",
            value_name = "Result shapes format",
            default_value_t = ShaclFormat::Internal
        )]
        result_shapes_format: ShaclFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,
    },

    #[command(name = "dctap")]
    DCTap {
        #[arg(short = 'd', long = "DCTap file", value_name = "DCTap file name")]
        file: PathBuf,

        #[arg(
            short = 'f',
            long = "File format",
            value_name = "DCTap file format",
            default_value_t = DCTapFormat::CSV
        )]
        format: DCTapFormat,

        #[arg(
            short = 'r',
            long = "Result format",
            value_name = "Ouput results format",
            default_value_t = DCTapResultFormat::Internal
        )]
        result_format: DCTapResultFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,
    },

    #[command(name = "convert")]
    Convert {
        #[arg(short = 'm', long = "Input mode", value_name = "Input mode")]
        input_mode: InputConvertMode,

        #[arg(short = 'd', long = "Source file", value_name = "Source file name")]
        file: PathBuf,

        #[arg(
            short = 'f',
            long = "Input format",
            value_name = "Input file format",
            default_value_t = InputConvertFormat::ShExC
        )]
        format: InputConvertFormat,

        #[arg(
            short = 'r',
            long = "export-format",
            value_name = "Ouput result format",
            default_value_t = OutputConvertFormat::Default
        )]
        result_format: OutputConvertFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            short = 'l',
            long = "shape-label",
            value_name = "shape label (default = START)"
        )]
        shape: Option<String>,

        #[arg(short = 'x', long = "export-mode", value_name = "Result mode")]
        output_mode: OutputConvertMode,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ShowNodeMode {
    Outgoing,
    Incoming,
    Both,
}

impl Display for ShowNodeMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShowNodeMode::Outgoing => write!(dest, "outgoing"),
            ShowNodeMode::Incoming => write!(dest, "incoming"),
            ShowNodeMode::Both => write!(dest, "both"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ShExFormat {
    Internal,
    ShExC,
    ShExJ,
    Turtle,
}

impl Display for ShExFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShExFormat::Internal => write!(dest, "internal"),
            ShExFormat::ShExC => write!(dest, "shexc"),
            ShExFormat::ShExJ => write!(dest, "shexj"),
            ShExFormat::Turtle => write!(dest, "turtle"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ShapeMapFormat {
    Compact,
    Internal,
}

impl Display for ShapeMapFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShapeMapFormat::Compact => write!(dest, "compact"),
            ShapeMapFormat::Internal => write!(dest, "internal"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum DataFormat {
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
}

impl From<DataFormat> for RDFFormat {
    fn from(val: DataFormat) -> Self {
        match val {
            DataFormat::Turtle => RDFFormat::Turtle,
            DataFormat::NTriples => RDFFormat::NTriples,
            DataFormat::RDFXML => RDFFormat::RDFXML,
            DataFormat::TriG => RDFFormat::TriG,
            DataFormat::N3 => RDFFormat::N3,
            DataFormat::NQuads => RDFFormat::NQuads,
        }
    }
}

impl Display for DataFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DataFormat::Turtle => write!(dest, "turtle"),
            DataFormat::NTriples => write!(dest, "ntriples"),
            DataFormat::RDFXML => write!(dest, "rdfxml"),
            DataFormat::TriG => write!(dest, "trig"),
            DataFormat::N3 => write!(dest, "n3"),
            DataFormat::NQuads => write!(dest, "nquads"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ShaclFormat {
    Internal,
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
}

impl Display for ShaclFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShaclFormat::Internal => write!(dest, "internal"),
            ShaclFormat::Turtle => write!(dest, "turtle"),
            ShaclFormat::NTriples => write!(dest, "NTriples"),
            ShaclFormat::RDFXML => write!(dest, "rdfxml"),
            ShaclFormat::TriG => write!(dest, "trig"),
            ShaclFormat::N3 => write!(dest, "n3"),
            ShaclFormat::NQuads => write!(dest, "nquads"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum DCTapFormat {
    CSV,
}

impl Display for DCTapFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DCTapFormat::CSV => write!(dest, "csv"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum DCTapResultFormat {
    Internal,
    JSON,
}

impl Display for DCTapResultFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DCTapResultFormat::Internal => write!(dest, "internal"),
            DCTapResultFormat::JSON => write!(dest, "json"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum InputConvertFormat {
    ShExC,
}

impl Display for InputConvertFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InputConvertFormat::ShExC => write!(dest, "shexc"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum InputConvertMode {
    ShEx,
}

impl Display for InputConvertMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InputConvertMode::ShEx => write!(dest, "shex"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum OutputConvertMode {
    SPARQL,
}

impl Display for OutputConvertMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            OutputConvertMode::SPARQL => write!(dest, "sparql"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum OutputConvertFormat {
    Default,
    Internal,
    JSON,
}

impl Display for OutputConvertFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            OutputConvertFormat::Internal => write!(dest, "internal"),
            OutputConvertFormat::JSON => write!(dest, "json"),
            OutputConvertFormat::Default => write!(dest, "default"),
        }
    }
}
