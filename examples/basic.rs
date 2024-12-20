extern crate faster_rs;

use faster_rs::{status, FasterKv};
use local_channel::mpsc::Receiver;

#[monoio::main]
async fn main() {
    // Create a Key-Value Store
    let store = FasterKv::default();
    let key0: u64 = 1;
    let value0: u64 = 1000;
    let modification: u64 = 5;

    // Upsert
    for i in 0..1000 {
        let upsert = store.upsert(&(key0 + i), &(value0 + i), i);
        assert!(upsert == status::OK || upsert == status::PENDING);
    }

    // Read-Modify-Write
    for i in 0..1000 {
        let rmw = store.rmw(&(key0 + i), &(5 as u64), i + 1000);
        assert!(rmw == status::OK || rmw == status::PENDING);
    }

    assert!(store.size() > 0);

    // Read
    for i in 0..1000 {
        // Note: need to provide type annotation for the Receiver
        let (read, mut recv): (u8, Receiver<u64>) = store.read(&(key0 + i), i);
        assert!(read == status::OK || read == status::PENDING);
        let val = recv.recv().await.unwrap();
        assert_eq!(val, value0 + i + modification);
        println!("Key: {}, Value: {}", key0 + i, val);
    }

    // Clear used storage
    match store.clean_storage() {
        Ok(()) => {}
        Err(_err) => panic!("Unable to clear FASTER directory"),
    }
}
