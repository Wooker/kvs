use assert_cmd::prelude::*;
use kvs::{KvsEngine, engines::{kvstore::KvStore, sled::SledStore}};
use predicates::ord::eq;
use predicates::str::{contains, is_empty, PredicateStrExt};
use std::process::Command;
use tempfile::TempDir;
use walkdir::WalkDir;

#[test]
fn temp_dir() {
    let temp_dir = TempDir::new().unwrap();
    dbg!(&temp_dir);
    dbg!(&temp_dir.path());
    let kvs = KvStore::open(temp_dir.path()).unwrap();
}
