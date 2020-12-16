use pdftotext::pdftotext_layout;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let text = pdftotext_layout(&args[1]).unwrap();

    for i in text {
        println!("{}", i);
    }
}
