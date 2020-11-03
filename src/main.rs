use git2::Repository;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let repo = Repository::discover(".");

    match repo {
        Ok(repository) => {
            let branch_name = branch_name(&repository);
            println!("Branch Name: {}", branch_name);
            let result = repository.find_reference("HEAD");
            let remote = result.unwrap();
            // let answer = remote.as_str().unwrap();
        }
        Err(_v) => process::exit(0x0100),
    }
}

fn branch_name(repository: &Repository) -> String {
    let result = repository.path();
    let path = format!("{}{}", result.to_str().unwrap(), "HEAD");
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let array = contents.split("/");
    array.last().unwrap().to_string()
}
