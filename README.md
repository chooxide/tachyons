# tachyons
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Rust port of the Tachyons CSS framework.

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Examples
### Basic
```rs
extern crate html_index;
extern crate tachyons;

pub fn main() {
  let res = html_index::Builder::new()
    .raw_body(
      r#"<body class="measure white bg-black">
      hello world
    </body>"#,
    )
    .inline_style(tachyons::TACHYONS_DEFAULT)
    .inline_style(tachyons::TACHYONS)
    .build();
  println!("{}", res);
}
```

## Installation
```sh
$ cargo add tachyons
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## References
None.

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/tachyons.svg?style=flat-square
[2]: https://crates.io/crates/tachyons
[3]: https://img.shields.io/travis/chooxide/tachyons.svg?style=flat-square
[4]: https://travis-ci.org/chooxide/tachyons
[5]: https://img.shields.io/crates/d/tachyons.svg?style=flat-square
[6]: https://crates.io/crates/tachyons
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/tachyons

[releases]: https://github.com/chooxide/tachyons/releases
[contributing]: https://github.com/chooxide/tachyons/.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/chooxide/tachyons/labels/good%20first%20issue
[help-wanted]: https://github.com/chooxide/tachyons/labels/help%20wanted
