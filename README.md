# fontconfig-parser

This crate provide parsing fontconfig file but not yet complete all features

see <https://www.freedesktop.org/software/fontconfig/fontconfig-user.html> for more detail infomation of fontconfig file

## Example

```rust
use fontconfig_parser::parse_document_from_str;

if let Ok(document_str) = std::fs::read_to_string("/etc/fonts/fonts.conf") {
    let document = parse_document_from_str(&document_str).unwrap();
}
```

License: MIT
