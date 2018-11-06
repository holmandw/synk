use std::fs::{self, Metadata, Permissions};
use std::io;
use std::path::Path;
use std::time::SystemTime;

struct Copier<'a> {
    from: &'a SynkFile<'a>,
    to: &'a SynkFile<'a>,
}

#[derive(Clone, Copy)]
pub struct SynkFile<'a> {
    p: &'a Path,
}

impl<'a> SynkFile<'a> {
    pub fn new(fname: &'a str) -> Self {
        let pt = Path::new(fname);
        SynkFile { p: pt }
    }

    fn metadata(&self) -> io::Result<Metadata> {
        fs::metadata(&self.p)
    }

    pub fn len(&self) -> u64 {
        self.metadata().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn mod_time(&self) -> u64 {
        let st = self.metadata().unwrap().modified().unwrap();
        st.duration_since(SystemTime::UNIX_EPOCH)
            .expect("cannot get mod_time")
            .as_secs()
    }

    pub fn permissions(&self) -> Permissions {
        self.metadata().unwrap().permissions()
    }

    fn overwrite_with(&self, with: &SynkFile) -> io::Result<u64> {
        fs::copy(with.p, self.p)
    }

    fn decide(a: &'a SynkFile, b: &'a SynkFile) -> Option<Copier<'a>> {
        let a_len = a.len();
        let a_mod = a.mod_time();
        let b_len = b.len();
        let b_mod = b.mod_time();
        if a_len == b_len {
            None
        } else if a_mod > b_mod && a_len > b_len {
            Some(Copier { from: a, to: b })
        } else if a_mod < b_mod && a_len < b_len {
            Some(Copier { from: &b, to: &a })
        } else {
            None
        }
    }
    pub fn sync(a: &SynkFile, b: &SynkFile) -> io::Result<Option<u64>> {
        let res = match SynkFile::decide(a, b) {
            Some(c) => Some(c.to.overwrite_with(c.from)?),
            _ => None,
        };
        Ok(res)
    }
}
