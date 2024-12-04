use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let filename = "pride_and_prejudice.txt";
    let content = fs::read_to_string(filename)?;

    println!("Analyzing text from: {filename}");
    analyze_text(&content);

    Ok(())
}

fn analyze_text(text: &str) {
    let char_count = text.chars().count();
    let word_count = text.split_whitespace().count();
    let line_count = text.lines().count();

    println!("Characters: {char_count}");
    println!("Words: {word_count}");
    println!("Lines: {line_count}");
}