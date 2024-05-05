use clap::{Args, Parser, Subcommand};

/// Post process the output of previous runs
/// by merging multiple jsonl files into a HTML graph
#[derive(Debug, Args, Clone)]
pub struct MergeOutput {
    /// The path to the output HTML file
    #[clap(long)]
    pub html: String,

    /// The path jsonl files to merge
    #[clap(long)]
    pub jsonl: Vec<String>,

    /// Remove recordings older than the given value
    #[clap(long)]
    pub cut_to: Option<f64>,

    /// Remove recordings newer than the given value
    #[clap(long)]
    pub cut_from: Option<f64>,

    /// Time offset applied for individual files
    /// Can specify multiple values, one for each merged file
    #[clap(long)]
    pub offset: Vec<f64>,

    /// If set, convert timestamps to relative time
    #[clap(long)]
    pub to_relative: Option<bool>,

    /// Sample max amount of data points
    /// If not set, all data points will be used
    /// If set, evenly selects at most the given amount of data points per file
    /// TIP: Use this to reduce the amount of data points for large files
    #[clap(long)]
    pub sample: Option<usize>,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Tools {
    Merge(MergeOutput),
}

#[derive(Parser, Debug, Clone)]
#[clap(version, about)]
pub struct CommandLine {
    #[clap(subcommand)]
    pub tool: Option<Tools>,

    /// The PID of the process to monitor.
    #[clap(short, long)]
    pub pid: Option<i32>,

    /// Command and arguments to run.
    /// Used if the PID is not provided.
    #[clap(short, long)]
    pub command: Option<String>,

    /// The interval between samples in milliseconds.
    #[clap(short, long, default_value = "10")]
    pub interval: u64,
    /// The duration of the monitoring in seconds.
    #[clap(short, long, default_value = "10")]
    pub duration: u64,

    /// Switch time to relative (in milliseconds).
    #[clap(long, default_value = "false")]
    pub relative_time: bool,

    /// list of absolute values to display. Example: "status.rssanon"
    #[clap(short, long)]
    pub absolute: Vec<String>,

    /// list of relative values to display. Example: "stat.stime"
    /// The value will be displayed as the difference between the current value and the previous value.
    /// The first value will be displayed as 0.
    #[clap(short, long)]
    pub relative: Vec<String>,

    /// Output path for the CSV file. If "-" is provided, the CSV will be printed to stdout.
    #[clap(long)]
    pub csv: Option<String>,

    /// Output path for the JSONL file. If "-" is provided, the JSONL will be printed to stdout.
    #[clap(long)]
    pub jsonl: Option<String>,

    /// Output path for the HTML file.
    #[clap(long)]
    pub html: Option<String>,
}
