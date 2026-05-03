use std::fs;
use chrono::Utc;

fn main() {
    let html = fs::read_to_string("page.html").unwrap();

    let converter = htmd::HtmlToMarkdown::builder().build();
    let md = converter.convert(&html);

    let filename = format!(
        "pages/page_{}.md",
        Utc::now().format("%Y%m%d_%H%M%S")
    );

    let content = format!(
        "# Archived Page\n\nCaptured: {}\n\n---\n\n{}",
        Utc::now(),
        md
    );

    fs::write(&filename, content).unwrap();

    println!("Saved {}", filename);
}
