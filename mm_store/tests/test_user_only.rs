// SPDX-FileCopyrightText: Charles Barto
//
// SPDX-License-Identifier: LGPL-3.0-only

use camino::{Utf8Path, Utf8PathBuf};

use mm_store::{*, mutable_tree::MutableTree};

fn datapath() -> Utf8PathBuf {
    let mut datapath = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    datapath.push("testdata");
    datapath
}

fn testrepo() -> Result<OsTreeRepo, RepoError> {
    let repo_path = Utf8PathBuf::from_iter(
        [env!("CARGO_TARGET_TMPDIR"), "testrepo1"].iter(),
    );
    _ = std::fs::remove_dir_all(&repo_path);
    OsTreeRepo::create(&repo_path)
}

#[test]
fn test1() -> Result<(), RepoError> {
    let tmpdir = env!("CARGO_TARGET_TMPDIR");
    println!("{}", tmpdir);
    let repo = testrepo()?;
    Ok(())
}

#[test]
fn test_write_tree_1() {
    let mut repo = testrepo().unwrap();
    let mut mtree = MutableTree::new();
    repo.write_dirpath_to_mtree(&datapath(), &mut mtree).unwrap()

}
// #[test]
// fn test_matches_hash_file() -> io::Result<()> {

// }
