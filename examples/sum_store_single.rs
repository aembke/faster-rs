extern crate faster_rs;

use faster_rs::*;
use std::env;
use local_channel::mpsc::Receiver;

const TABLE_SIZE: u64 = 1 << 15;
const LOG_SIZE: u64 = 1024 * 1024 * 1024;
const NUM_OPS: u64 = 1 << 25;
const NUM_UNIQUE_KEYS: u64 = 1 << 23;
const REFRESH_INTERVAL: u64 = 1 << 8;
const COMPLETE_PENDING_INTERVAL: u64 = 1 << 12;
const CHECKPOINT_INTERVAL: u64 = 1 << 20;

const STORAGE_DIR: &str = "sum_store_single_storage";

// More or less a copy of the single-threaded sum_store populate/recover example from FASTER

#[monoio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let operation = &args[1].to_string();

        if operation == "populate" {
            println!(
                "{}",
                "This may take a while, and make sure you have disk space"
            );
            populate().await;
        } else if operation == "recover" {
            if args.len() > 2 {
                let token = &args[2];
                recover(token.to_string()).await;
            } else {
                println!("Second argument required is token checkpoint to recover");
            }
        }
    } else {
        println!("Populate: args {}", "1. populate");
        println!("Recover: args {}, {}", "1. recover", "2. checkpoint token");
    }
}

async fn populate() -> () {
    if let Ok(store) = FasterKvBuilder::new(TABLE_SIZE, LOG_SIZE)
        .with_disk(STORAGE_DIR)
        .set_pre_allocate_log(true)
        .build()
    {
        // Populate Store
        let session = store.start_session();
        println!("Starting Session {}", session);

        for i in 0..NUM_OPS {
            let idx = i as u64;
            store.rmw(&(idx % NUM_UNIQUE_KEYS), &(1u64), idx);

            if (idx % CHECKPOINT_INTERVAL) == 0 {
                let check = store.checkpoint().unwrap();
                println!("Calling checkpoint with token {}", check.token);
            }

            if (idx % COMPLETE_PENDING_INTERVAL) == 0 {
                store.complete_pending(false);
            } else if (idx % REFRESH_INTERVAL) == 0 {
                store.refresh();
            }
        }

        println!("Dumping distribution");
        store.dump_distribution();
        println!("Stopping Session {}", session);
        store.complete_pending(true);
        store.stop_session();
        println!("Store size: {}", store.size());
    } else {
        println!("Failed to create FasterKV store");
    }
}

async fn recover(token: String) -> () {
    println!("Attempting to recover");
    if let Ok(recover_store) = FasterKvBuilder::new(TABLE_SIZE, LOG_SIZE)
        .with_disk(STORAGE_DIR)
        .set_pre_allocate_log(true)
        .build()
    {
        match recover_store.recover(token.clone(), token.clone()) {
            Ok(rec) => {
                println!("Recover version: {}", rec.version);
                println!("Recover status: {}", rec.status);
                println!("Recovered sessions: {:?}", rec.session_ids);
                let persisted_count =
                    recover_store.continue_session(rec.session_ids.first().cloned().unwrap());
                println!("Session persisted until: {}", persisted_count);

                let mut expected_results = Vec::with_capacity(NUM_UNIQUE_KEYS as usize);
                expected_results.resize(NUM_UNIQUE_KEYS as usize, 0);
                for i in 0..(persisted_count + 1) {
                    let elem = expected_results
                        .get_mut((i % NUM_UNIQUE_KEYS) as usize)
                        .unwrap();
                    *elem += 1;
                }

                println!("Verifying recovered values!");
                let mut incorrect = 0;
                for i in 0..NUM_OPS {
                    let idx = i as u64;
                    let (status, mut recv): (u8, Receiver<u64>) =
                        recover_store.read(&(idx % NUM_UNIQUE_KEYS), idx);
                    if let Some(val) = recv.recv().await {
                        let expected = *expected_results
                            .get((idx % NUM_UNIQUE_KEYS) as usize)
                            .unwrap();
                        if expected != val {
                            println!(
                                "Error recovering {}, expected {}, got {}",
                                idx, expected, val
                            );
                            incorrect += 1;
                        }
                    } else {
                        println!("Failure to read with status: {}, and key: {}", status, idx);
                    }
                }
                println!("{} incorrect recoveries", incorrect);
                recover_store.stop_session();
            }
            Err(_) => println!("Recover operation failed"),
        }
    } else {
        println!("{}", "Failed to create recover store");
    }
}
