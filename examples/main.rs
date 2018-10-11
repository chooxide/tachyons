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
