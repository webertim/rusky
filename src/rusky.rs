use std::io::{Read, Write};
use std::ops::Add;
use std::process::{Command};
use git2::{DiffOptions, Repository, Tree};
use serde::{Deserialize, Serialize};

const REPOSITORY_PATH: &str = "..";
const HOOKS_FOLDER: &str = "../.git/hooks/";
const PRE_COMMIT_SCRIPT: &str = "#!/usr/bin/env bash

./.rusky/rusky";

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Config {
    rules: Vec<Rule>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Rule {
    file_match: String,
    commands: Vec<String>
}

pub fn rusky_setup() {
    Repository::open(REPOSITORY_PATH).expect("Make sure you execute the setup script inside a repository.");
    if !std::fs::try_exists(HOOKS_FOLDER).unwrap_or(false) {
        std::fs::create_dir(HOOKS_FOLDER).expect("The .git folder is missing and cannot be created");
    }

    let mut file = std::fs::File::create(String::from(HOOKS_FOLDER).add("pre-commit")).expect("Couldn't create pre-commit file. Make sure you do not have any pre-commit script setup.");
    file.write(PRE_COMMIT_SCRIPT.as_bytes()).expect("Error writing to file.");
}
pub fn rusky_default() {
    let mut buffer = vec![];
    let mut file = std::fs::File::open("rusky.yaml").expect("Error opening config");
    file.read_to_end(&mut buffer).expect("Error reading config");
    let config: Config = serde_yaml::from_str(String::from_utf8(buffer.to_vec()).unwrap().as_str()).expect("Error parsing config");

    let repository = Repository::open(REPOSITORY_PATH).expect("Make sure you execute the setup script inside a repository.");
    let head = repository.head().expect("Error getting HEAD of repository").peel_to_tree().expect("Error getting HEAD of repository");

    for rule in config.rules {
        check_and_execute_rule(
            rule,
            &repository,
            &head
        );
    }
}

fn check_and_execute_rule(rule: Rule, repository: &Repository, head: &Tree) {
    let mut diff_options = DiffOptions::new();
    diff_options.pathspec(rule.file_match);

    if let Ok(diff) = repository.diff_tree_to_index(Some(head), None, Some(&mut diff_options)) {
        if diff.deltas().count() == 0 {
            return;
        }

        let commands_joined = rule.commands.join(" && ");
        let mut command: Command;
        if cfg!(windows) {
            command = Command::new("cmd");
            command.arg("/C").arg(commands_joined);
        } else {
            command = Command::new("sh");
            command.arg("-c").arg(commands_joined);
        }

        let output = command.output().expect("Error");

        if output.status.code().unwrap_or(1) != 0 {
            panic!("{}", String::from_utf8(output.stderr).unwrap_or("Unknown error".to_string()));
        }
    };
}