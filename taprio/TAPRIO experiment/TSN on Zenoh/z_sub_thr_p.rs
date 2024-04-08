//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
use clap::Parser;
use std::io::{stdin, Read};
use std::time::{Duration, Instant};
use zenoh::config::Config;
use zenoh::prelude::sync::*;
use zenoh_examples::CommonArgs;
use std::thread;
use std::collections::HashMap;
use std::sync::mpsc;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Write};
struct Stats {
    round_count: usize,
    round_size: usize,
    finished_rounds: usize,
    round_start: Instant,
    global_start: Option<Instant>,
    priority: String,
    path: String,
    throughputs: Vec<f64>, // Vector to store data
}
impl Stats {
    fn new(round_size: usize, priority: &str, path: &str) -> Self {
        Stats {
            round_count: 0,
            round_size,
            finished_rounds: 0,
            round_start: Instant::now(),
            global_start: None,
            priority: priority.to_string(),
            path: format!("{}{}.txt", path, priority),
            throughputs: Vec::new(), // Vector to store data
        }
    }
    fn increment(&mut self) {
        if self.round_count == 0 {
            self.round_start = Instant::now();
            if self.global_start.is_none() {
                self.global_start = Some(self.round_start)
            }
            self.round_count += 1;
        } else if self.round_count < self.round_size {
            self.round_count += 1;
        } else {
            self.print_round();
            self.add_throughput();
            self.finished_rounds += 1;
            self.round_count = 0;
        }
    }
    fn print_round(&self)-> io::Result<()> {
        let elapsed = self.round_start.elapsed().as_secs_f64();
        let throughput = (self.round_size as f64) / elapsed;
        println!("Priority {}: {throughput} msg/s", self.priority);
        let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&self.path)?;
        writeln!(file, "{}", throughput)?;
        Ok(())
    }
    fn add_throughput(&mut self) {
        let elapsed = self.round_start.elapsed().as_secs_f64();
        let throughput = (self.round_size as f64) / elapsed;
        self.throughputs.push(throughput); // Add data to the vector
    }
    fn print_throughputs(&mut self) {
        println!("throughput: {:?}", self.throughputs); // Print the vector
    }

}
impl Drop for Stats {
    fn drop(&mut self) {
        let Some(global_start) = self.global_start else {
            return;
        };
        let elapsed = global_start.elapsed().as_secs_f64();
        let total = self.round_size * self.finished_rounds + self.round_count;
        let throughtput = total as f64 / elapsed;
        self.print_throughputs();
    }
}

fn main() {
    // initiate logging
    env_logger::init();
    let args = Args::parse();
    let (mut config, m, n, no_stdin) = parse_args();
    let mut threads = vec![]; // Vector to store thread handles
    let mut all_stats: Vec<String> = Vec::new();
    // A probing procedure for shared memory is performed upon session opening. To enable `z_pub_shm_thr` to operate
    // over shared memory (and to not fallback on network mode), shared memory needs to be enabled also on the
    // subscriber side. By doing so, the probing procedure will succeed and shared memory will operate as expected.
    config.transport.shared_memory.set_enabled(true).unwrap();
    let session = zenoh::open(config).res().unwrap();
    let session = session.into_arc();


    if let Some(priorities) = args.priority {
        for prio_str in priorities {
            let prio_str_clone = prio_str.clone();
            let session_clone = session.clone();
            let file_path_clone=args.file_path.clone();
            let thread_handle = thread::spawn(move || {
            let topic = format!("test{}/thr", prio_str);
            println!("topic: {topic}");

            let mut stats = Stats::new(n, &prio_str_clone, &file_path_clone);
            let _sub = session_clone
                .declare_subscriber(topic)
                .callback_mut(move |_sample| {
                    stats.increment();
                    if stats.finished_rounds >= m {
                        std::process::exit(0)
                    }
                })
                .res()
                .unwrap();
            
            if no_stdin {
                loop {
                    std::thread::park();
                }
            } else {
                for byte in stdin().bytes() {
                    match byte {
                        Ok(b'q') => break,
                        _ => std::thread::yield_now(),
                    }
                }
            }
        });
        threads.push(thread_handle);
    }
    
    for thread in threads {
        thread.join().unwrap();
    }
    
}
}
#[derive(clap::Parser, Clone, PartialEq, Eq, Hash, Debug)]
struct Args {
    #[arg(short, long, value_delimiter = ' ')]
    priority: Option<Vec<String>>, // Change type to Vec<String>
    #[arg(short, long, default_value = "10")]
    /// Number of throughput measurements.
    samples: usize,
    #[arg(short, long, default_value = "100000")]
    /// Number of messages in each throughput measurements.
    number: usize,
    /// Do not read standard input.
    #[arg(long)]
    no_stdin: bool,
    #[command(flatten)]
    common: CommonArgs,
    #[arg(short, long, value_delimiter = ' ')]
    file_path: String,
}

fn parse_args() -> (Config, usize, usize, bool) {
    let args = Args::parse();
    (args.common.into(), args.samples, args.number, args.no_stdin)
}
