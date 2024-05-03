mod args;
mod common;
mod merge;
mod save_csv;
mod save_html;
mod save_jsonl;

use crate::args::{CommandLine, Tools};
use clap::Parser;
use procfs::process::Process;
use std::collections::HashMap;
use std::time::Duration;

use crate::merge::merge_results;
use serde::Serialize;
use serde_json::Value;
use shlex::split;
use std::process::{Child, Command};

struct Nanny {
    pub child: Child,
}

impl Drop for Nanny {
    fn drop(&mut self) {
        self.child.kill().unwrap();
    }
}

#[derive(Serialize)]
struct Record {
    #[serde(skip)]
    pub timestamp: std::time::Instant,
    pub system_time: std::time::SystemTime,
    pub status: procfs::process::Status,
    pub stat: procfs::process::Stat,
    pub io: procfs::process::Io,
}

fn get_float_from_value(value: &Value, key: &str) -> Option<f64> {
    // Iterate key separated by dot
    if key.is_empty() {
        return value.as_f64();
    }

    let Some((head, tail)) = key.split_once('.') else {
        return value.get(key).and_then(|v| v.as_f64());
    };

    if let Some(next) = value.get(head) {
        return get_float_from_value(next, tail);
    }

    None
}

fn main() {
    let args = CommandLine::parse();

    match args.tool {
        None => {}
        Some(Tools::Merge(merge_command)) => {
            merge_results(merge_command);
            return;
        }
    }

    let mut _process_guard = None;

    let me = if let Some(pid) = args.pid {
        Process::new(pid).unwrap()
    } else if let Some(command_line) = args.command {
        let Some(mut command_args) = split(&command_line) else {
            panic!("Invalid command line")
        };

        assert!(!command_args.is_empty(), "Command must be provided");

        let command_name = command_args.remove(0);
        let mut command = Command::new(command_name);
        for arg in command_args.iter() {
            command.arg(arg);
        }
        let subproc = command.spawn().unwrap();
        let nanny = Nanny { child: subproc };
        let pid = nanny.child.id();
        _process_guard = Some(nanny);
        Process::new(pid as i32).unwrap()
    } else {
        panic!("Either PID or command must be provided")
    };

    let mut data = vec![];
    let time_delay = Duration::from_millis(args.interval);
    let start = std::time::Instant::now();
    let time_limit = Duration::from_secs(args.duration);

    loop {
        if !me.is_alive() {
            break;
        }

        let status = me.status().unwrap();
        let stat = me.stat().unwrap();
        let io = me.io().unwrap();

        data.push(Record {
            timestamp: std::time::Instant::now(),
            system_time: std::time::SystemTime::now(),
            status,
            stat,
            io,
        });
        std::thread::sleep(time_delay);

        if start.elapsed() > time_limit {
            break;
        }
    }

    let mut collected_data = vec![];

    let mut previous: HashMap<String, f64> = HashMap::new();
    let mut previous_absolute: HashMap<String, f64> = HashMap::new();

    for rec in data {
        if args.relative_time {
            let delay = rec.timestamp.duration_since(start).as_millis();
            previous.insert("delay".to_string(), delay as f64);
        } else {
            previous.insert(
                "timestamp".to_string(),
                rec.system_time
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as f64,
            );
        }

        let json_value: Value = serde_json::to_value(rec).unwrap();
        for key in args.absolute.iter() {
            let value = get_float_from_value(&json_value, key);
            if let Some(value) = value {
                let previous_value = previous.entry(key.clone()).or_insert(value);
                *previous_value = value;
            }
        }

        for key in args.relative.iter() {
            let value = get_float_from_value(&json_value, key);
            if let Some(value) = value {
                let previous_abs_value = previous_absolute.entry(key.clone()).or_insert(value);
                let previous_value = previous.entry(key.clone()).or_insert(0.0);
                *previous_value = value - *previous_abs_value;
                *previous_abs_value = value;
            }
        }
        collected_data.push(previous.clone());
    }

    if let Some(path) = args.jsonl {
        save_jsonl::save_data_as_jsonl(&path, &collected_data).unwrap();
    }

    if let Some(path) = args.csv {
        save_csv::save_csv(&path, &collected_data).unwrap();
    }

    if let Some(path) = args.html {
        save_html::save_data_as_html(&path, &collected_data).unwrap();
    }
}
