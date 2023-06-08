use pdf_extract;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    // Open the PDF file
    // let bytes = std::fs::read("/home/siphiwe/Downloads/FAIS.pdf").unwrap();
    let out = pdf_extract::extract_text("path").expect("Couldn't extract text from this path.");
    let mut file = File::create("results.txt").expect("Failed to create file");

    file.write_all(out.as_bytes())
        .expect("Failed to write file.");
    let elapsed = start.elapsed();

    println!("Time taken: {:.2?}", elapsed);
}
