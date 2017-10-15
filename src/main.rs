extern crate url;

use std::env;
use std::process::{Command, Stdio};
use std::error::Error;
use url::Url;

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
    let repo_url = "https://github.com/packsaddle/example-circle_ci-pull_request.git";
    let target_dir = "./for_target_dir";
    //    git httpsable clone --depth=1
    let mut child = Command::new("git")
        .args(&["clone", "--depth", "1", repo_url, target_dir])
        .stderr(Stdio::null())
        .spawn()
        .expect("git command failed to start");
    let ecode = child.wait().expect("failed to wait on child");
    std::process::exit(ecode.code().unwrap());
}

pub fn adjust(https_url: &str, username: &str, password: &str) -> Result<String, Box<Error>> {
    let mut parsed = Url::parse(https_url)?;
    parsed.set_username(username).expect("failed to set username");
    parsed.set_password(Some(password)).expect("failed to set password");
    Ok(parsed.as_str().to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adjust_both() {
        let https_url = "https://example.com/foo";
        let username = "username";
        let password = "password";
        let expected = "https://username:password@example.com/foo";
        assert_eq!(adjust(https_url, username, password).unwrap(), expected);
    }
}
