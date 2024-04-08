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
use std::convert::TryInto;
use zenoh::prelude::sync::*;
use zenoh::publication::CongestionControl;
use zenoh_examples::CommonArgs;
use std::thread;
//use zenoh::prelude::r#async::*;

fn main() {
    // initiate logging
    env_logger::init();
    let args = Args::parse();

    // let mut prio = Priority::default();
    // if let Some(p) = args.priority {
    //     prio = p.try_into().unwrap();
    // }

    let payload_size = args.payload_size;

    let data: Value = (0..payload_size)
        .map(|i| (i % 10) as u8)
        .collect::<Vec<u8>>()
        .into();

    //let session = zenoh::open(args.common).res().unwrap();
    let mut threads = vec![]; // Vector to store thread handles
    let session = zenoh::open(args.common).res().unwrap();
    let session = session.into_arc();
    if let Some(priorities) = args.priority {
        for prio_str in priorities {
            let session_clone = session.clone();
            let data_clone = data.clone(); // Clone the data for each thread
            //let common_args_clone = args.common.clone(); // Clone common arguments for each thread
            let thread_handle = thread::spawn(move || {
                
                // Parse priority from string to u8
                let prio: u8 = prio_str.parse().unwrap();
                
                let topic = format!("test{}/thr", prio);
                println!("Topic: {topic}");
                let publisher = session_clone
                    .declare_publisher(topic)
                    .congestion_control(CongestionControl::Block)
                    .priority(prio.try_into().unwrap())
                    .res()
                    .unwrap();
                let mut count: usize = 0;
                let mut start = std::time::Instant::now();
                loop {
                    publisher.put(data_clone.clone()).res().unwrap();

                    if args.print {
                        if count < args.number {
                            count += 1;
                        } else {
                            let thpt = count as f64 / start.elapsed().as_secs_f64();
                            println!("{thpt} msg/s");
                            count = 0;
                            start = std::time::Instant::now();
                        }
                    }
                }
            });
            threads.push(thread_handle); // Store the thread handle
        }
    }
    for thread in threads {
        thread.join().unwrap();
    }
}

#[derive(Parser, Clone, PartialEq, Eq, Hash, Debug)]
struct Args {
    /// Priority for sending data
    #[arg(short, long, value_delimiter = ' ')]
    priority: Option<Vec<String>>, // Change type to Vec<String>
    //priority: Option<Vec<u8>>,
    /// Print the statistics
    #[arg(short = 't', long)]
    print: bool,
    /// Number of messages in each throughput measurements
    #[arg(short, long, default_value = "100000")]
    number: usize,
    /// Sets the size of the payload to publish
    payload_size: usize,
    #[command(flatten)]
    common: CommonArgs,
}
