# git_httpsable_cli

[![crates version][crates-image]][crates-url] [![Travis-CI Status][travis-image]][travis-url] [![Appveyor Status][appveyor-image]][appveyor-url] ![license][license-image]

> Execute git command with https-url.

`git push` to https-url with access token.

See [github_httpsable_cli](https://github.com/packsaddle/rust-github_httpsable_cli) for GitHub speficic version.
See [git_httpsable](https://github.com/packsaddle/rust-git_httpsable) for the programmatic API.

## Example

```bash
$ GIT_HTTPSABLE_USERNAME=__your_user_name__ GIT_HTTPSABLE_PASSWORD=__your_password__ \
  git-httpsable clone https://example.com/git/repo ./target_dir
# => git clone https://__your_user_name__:__your_password__@example.com/git/repo ./target_dir
```

```bash
$ GIT_HTTPSABLE_USERNAME=__your_user_name__ GIT_HTTPSABLE_PASSWORD=__your_password__ \
  git-httpsable push https://example.com/git/repo your_branch
# => git push https://__your_user_name__:__your_password__@example.com/git/repo your_branch
```

## Install

Download from [Latest release](https://github.com/packsaddle/rust-git_httpsable_cli/releases/latest) for your own environment.

or

```
$ cargo install git_httpsable_cli
```

## changelog

[changelog](./changelog.md)

## License

MIT/Apache-2.0 © [sanemat](http://sane.jp)


[travis-url]: https://travis-ci.org/packsaddle/rust-git_httpsable_cli
[travis-image]: https://img.shields.io/travis/packsaddle/rust-git_httpsable_cli/master.svg?style=flat-square&label=travis
[appveyor-url]: https://ci.appveyor.com/project/sanemat/rust-git-httpsable-cli/branch/master
[appveyor-image]: https://img.shields.io/appveyor/ci/sanemat/rust-git-httpsable-cli/master.svg?style=flat-square&label=appveyor
[crates-url]: https://crates.io/crates/git_httpsable_cli
[crates-image]: https://img.shields.io/crates/v/git_httpsable_cli.svg?style=flat-square
[license-image]: https://img.shields.io/crates/l/git_httpsable_cli.svg?style=flat-square
