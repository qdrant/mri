use clap::Parser;



#[derive(Parser, Debug, Clone)]
#[clap(version, about)]
pub struct Args {
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
}