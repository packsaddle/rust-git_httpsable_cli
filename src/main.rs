extern crate git_httpsable;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let username = match env::var("GIT_HTTPSABLE_USERNAME") {
        Ok(result) => result,
        Err(_) => panic!("GIT_HTTPSABLE_USERNAME is required."),
    };
    let password = match env::var("GIT_HTTPSABLE_PASSWORD") {
        Ok(result) => result,
        Err(_) => panic!("GIT_HTTPSABLE_PASSWORD is required."),
    };
    let mut child = git_httpsable::run(&args[1..], &username, &password)
        .expect("git command failed to start");
    let ecode = child.wait().expect("failed to wait on child");
    std::process::exit(ecode.code().unwrap());
}
