use git_hash::ObjectId;
use std::path::{Path, PathBuf};

mod access;
mod entry;
mod file;
mod init;

pub fn hex_to_id(hex: &str) -> ObjectId {
    ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}

pub fn fixture_index_path(name: &str) -> PathBuf {
    let dir =
        git_testtools::scripted_fixture_read_only_standalone(Path::new("make_index").join(name).with_extension("sh"))
            .expect("script works");
    dir.join(".git").join("index")
}

pub fn loose_file_path(name: &str) -> PathBuf {
    git_testtools::fixture_path_standalone(Path::new("loose_index").join(name).with_extension("git-index"))
}

#[test]
fn size_of_entry() {
    assert_eq!(std::mem::size_of::<git_index::Entry>(), 80);

    // the reason we have our own time is half the size.
    assert_eq!(std::mem::size_of::<git_index::entry::Time>(), 8);
    assert_eq!(std::mem::size_of::<filetime::FileTime>(), 16);
}

enum Fixture {
    Generated(&'static str),
    Loose(&'static str),
}

impl Fixture {
    pub fn to_path(&self) -> PathBuf {
        match self {
            Fixture::Generated(name) => fixture_index_path(name),
            Fixture::Loose(name) => loose_file_path(name),
        }
    }
    pub fn to_name(&self) -> &'static str {
        match self {
            Fixture::Generated(name) | Fixture::Loose(name) => name,
        }
    }

    pub fn open(&self) -> git_index::File {
        git_index::File::at(self.to_path(), git_hash::Kind::Sha1, Default::default())
            .expect("fixtures are always readable")
    }
}
