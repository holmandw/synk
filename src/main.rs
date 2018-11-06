#![allow(dead_code)]
use std::io;
use std::thread::sleep;
use std::time::Duration;

use synk::synkfile::SynkFile;
const SLEEP_DURATION: Duration = Duration::from_millis(2000);

fn main_loop(a: SynkFile, b: SynkFile) -> io::Result<()> {
    loop {
        match SynkFile::sync(&a, &b) {
            Ok(Some(_)) => println!("synced successfully"),
            Ok(None) => println!("no sync needed"),
            Err(err) => println!("{:?}", err),
        }
        sleep(SLEEP_DURATION);
    }
}

fn main() -> io::Result<()> {
    let a = SynkFile::new("data_a/a");
    let b = SynkFile::new("data_b/a");
    main_loop(a, b)
}
