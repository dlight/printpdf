# printpdf

Just run `cargo run file.pdf` and it will (hopefully) print the same as `pdftotext -layout file.pdf`.

To link with poppler statically, do `cargo build --features static-poppler`.