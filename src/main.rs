#![allow(dead_code)]
use std::fs::{self, Metadata, Permissions};
use std::io;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;

const SLEEP_DURATION: Duration = Duration::from_millis(2000);

struct SynkFile<'a> {
    p: &'a Path,
}

impl<'a> SynkFile<'a> {
    fn new(fname: &'a str) -> Self {
        let pt = Path::new(fname);
        SynkFile { p: pt }
    }

    fn metadata(&self) -> io::Result<Metadata> {
        fs::metadata(&self.p)
    }

    fn size(&self) -> u64 {
        self.metadata().unwrap().len()
    }

    fn mod_time(&self) -> u64 {
        let st = self.metadata().unwrap().modified().unwrap();
        st.duration_since(SystemTime::UNIX_EPOCH)
            .expect("cannot get mtime")
            .as_secs()
    }

    fn permissions(&self) -> Permissions {
        self.metadata().unwrap().permissions()
    }

    fn overwrite_with(&self, with: &SynkFile) -> io::Result<u64> {
        fs::copy(with.p, self.p)
    }
}

fn sync(a: &SynkFile, b: &SynkFile) -> io::Result<Option<()>> {
    let a_len = a.size();
    let a_mod = a.mod_time();
    let b_len = b.size();
    let b_mod = b.mod_time();
    let retval = if a_len == b_len {
        None
    } else if a_mod > b_mod && a_len > b_len {
        b.overwrite_with(&a)?;
        Some(())
    } else if a_mod < b_mod && a_len < b_len {
        a.overwrite_with(&b)?;
        Some(())
    } else {
        None
    };
    Ok(retval)
}

fn main_loop(a: SynkFile, b: SynkFile) -> io::Result<()> {
    loop {
        let res = sync(&a, &b);
        match res {
            Ok(Some(())) => println!("synced successfully"),
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
