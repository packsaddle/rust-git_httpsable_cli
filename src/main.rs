use std::env;

fn main() {
    let username = env::var("GIT_HTTPSABLE_USERNAME").unwrap();
    let password = env::var("GIT_HTTPSABLE_PASSWORD").unwrap();
    let repo_url = "https://github.com/packsaddle/example-circle_ci-pull_request.git";
    let target_dir = "./target_dir";
//    git httpsable clone --depth=1
    println!("Hello, world!");
}
