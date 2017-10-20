use std::error::Error;
use url::Url;
use std::process::{Command, Stdio, Child};

pub fn run(items: &[String], username: &str, password: &str) -> Result<Child, ::std::io::Error> {
    Command::new("git")
        .args(
            items
                .iter()
                .map(|input| filter_schema(&input, &username, &password))
                .collect::<Vec<_>>(),
        )
        .stderr(Stdio::null())
        .spawn()
}

pub fn filter_schema(input: &str, username: &str, password: &str) -> String {
    if input.starts_with("https://") || input.starts_with("http://") {
        adjust(input, username, password).unwrap()
    } else {
        input.to_string()
    }
}

pub fn adjust(schema_url: &str, username: &str, password: &str) -> Result<String, Box<Error>> {
    let mut parsed = Url::parse(schema_url)?;
    parsed.set_username(username).expect(
        "failed to set username",
    );
    parsed.set_password(Some(password)).expect(
        "failed to set password",
    );
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
