# HTML Demo Procedure

Please run the following to generate an html lifetime highlighting (simple
version) by the following code.

```bash
# clone this repo
cd rust-analyzer/
git fetch; git checkout html-demo
./html_regen.sh
```
This will build the rust analyzer project, and then run the ra_cli module,
specifically run `highlight -r`.

The script is set to compute any .rs file in `html/src/*.rs` into 
`html/*.html`. Give it a try on other rust code of your choice!

## Known issue

I realized that the current Rust Analyzer doesn't understand the syntax
in macro (e.g. `println!(...)`, `format!(...)`) very well yet. 

## License

Rust analyzer is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
