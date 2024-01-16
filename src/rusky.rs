use std::io::Write;
use std::ops::Add;
use git2::Repository;

const HOOKS_FOLDER: &str = "../.git/hooks/";
const PRE_COMMIT_SCRIPT: &str = "#!/usr/bin/env bash

./.rusky/rusky";

pub fn rusky_setup() {
    Repository::open("..").expect("Make sure you execute the setup script inside a repository.");
    if !std::fs::try_exists(HOOKS_FOLDER).unwrap_or(false) {
        std::fs::create_dir(HOOKS_FOLDER).expect("The .git folder is missing and cannot be created");
    }

    let mut file = std::fs::File::create(String::from(HOOKS_FOLDER).add("pre-commit")).expect("Couldn't create pre-commit file. Make sure you do not have any pre-commit script setup.");
    file.write(PRE_COMMIT_SCRIPT.as_bytes()).expect("Error writing to file.");
}
pub fn rusky_default() {

}