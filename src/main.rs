use std::env;
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();
    let username = match env::var("GIT_HTTPSABLE_USERNAME") {
        Ok(result) => result,
        Err(_) => panic!("GIT_HTTPSABLE_USERNAME is required.")
    };
    let password = match env::var("GIT_HTTPSABLE_PASSWORD") {
        Ok(result) => result,
        Err(_) => panic!("GIT_HTTPSABLE_PASSWORD is required.")
    };
    let repo_url = "https://github.com/packsaddle/example-circle_ci-pull_request.git";
    let target_dir = "./for_target_dir";
    //    git httpsable clone --depth=1
    let mut child = Command::new("git")
        .args(&["clone", "--depth", "1", repo_url, target_dir])
        .stderr(Stdio::null())
        .spawn()
        .expect("git command failed to start");
    let ecode = child.wait().expect("failed to wait on child");

    assert!(ecode.success());
}
