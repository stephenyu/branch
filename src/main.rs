use clap::{App, Arg};
use colored::*;
use git2::{BranchType, Repository};
use loading::Loading;
use std::process;

fn main() {
    let matches = App::new("Branches")
        .version("1.0")
        .author("Stephen Y. <s.yu@less3.com>")
        .about("Quickly switch Git Branches by name")
        .arg(
            Arg::with_name("FILTER")
                .help("If FILTER is numerical, switch to that Git Branch.  Otherwise, use it to filter the original branches.")
                .required(false)
                .index(1)
        )
        .arg(
            Arg::with_name("INDEX")
                .help("Switch to the Git Branch, given the previous FILTER")
                .required(false)
                .index(2),
        )
        .get_matches();
    let repo = Repository::discover(".");

    match repo {
        Ok(repository) => {
            let branches = branches(&repository);

            if let Some(string_index) = matches.value_of("FILTER") {
                let parsed_index = string_index.parse::<usize>();

                match parsed_index {
                    Ok(number) => switch_selected_branch(&branches, number, &repository),
                    Err(_v) => {
                        let filtered_branches: Vec<String> = branches
                            .iter()
                            .filter(|x| x.contains(string_index))
                            .cloned()
                            .collect();

                        if filtered_branches.len() == 1 {
                            switch_selected_branch(&filtered_branches, 1, &repository);
                            process::exit(0);
                        }

                        if let Some(string_index) = matches.value_of("INDEX") {
                            let parsed_index = string_index.parse::<usize>();

                            match parsed_index {
                                Ok(number) => {
                                    switch_selected_branch(&filtered_branches, number, &repository);
                                    process::exit(0);
                                }
                                Err(_v) => println!("{} is not a number", string_index),
                            }
                        } else {
                            display_list(&filtered_branches);
                        }
                    }
                }
            } else {
                display_list(&branches);
                process::exit(0)
            }
        }
        Err(_v) => process::exit(0),
    }
}

fn switch_selected_branch(branches: &Vec<String>, index: usize, repository: &Repository) {
    let selected_branch = &branches[index - 1];
    let mut loading = Loading::new();
    loading.start();

    loading.text(format!(
        "{} {}",
        "Switching to Branch:".green(),
        selected_branch.white()
    ));

    checkout_branch(&repository, &selected_branch);

    loading.success(format!(
        "{} {}",
        "Switched to Branch:".green(),
        selected_branch.white()
    ));

    loading.end();
}

fn branches(repository: &Repository) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let branches = repository.branches(Some(BranchType::Local)).unwrap();

    for b in branches {
        let (branch, _branch_type) = b.unwrap();
        let branch_name = branch.name().unwrap().unwrap();
        result.push(String::from(branch_name));
    }

    return result;
}

fn display_list(branches: &Vec<String>) {
    let mut index = 1;
    for branch_name in branches {
        println!("{}\t{}", index, branch_name.blue());
        index += 1;
    }
}

fn checkout_branch(repository: &Repository, branch_name: &String) {
    let obj = repository
        .revparse_single(&("refs/heads/".to_owned() + branch_name))
        .unwrap();

    let result = repository.checkout_tree(&obj, None);

    match result {
        Ok(_v) => {
            repository
                .set_head(&("refs/heads/".to_owned() + branch_name))
                .unwrap();
        }
        Err(_v) => error_exit("Unable to Checkout Tree"),
    }
}

fn error_exit(message: &str) {
    println!("Error: {}", message);
    process::exit(1);
}
