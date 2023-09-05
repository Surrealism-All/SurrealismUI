use std::fmt::format;
use std::io::Write;
use std::{
    fs::read_dir,
    fs::{read_to_string, write, File},
    path::Path,
};
fn main() {
    let words = read_to_string("E:\\Rust\\try\\surrealism-ui\\name.txt")
        .unwrap()
        .split("\n")
        .map(String::from)
        .collect::<Vec<String>>();
    // println!("{:?}", words);

    if let Ok(entries) = read_dir("E:\\Rust\\try\\surrealism-ui\\components\\src\\icon\\icons") {
        let mut file =
            File::create("E:\\Rust\\try\\surrealism-ui\\name2.txt").expect("Failed to create file");
        let mut names = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                // @image-url(\"{}\")
                let formatted_name =
                    format!("@image-url(\"{}\")", entry.file_name().to_str().unwrap());
                names.push(formatted_name);
            }
        }
        let mut res = Vec::new();
        for i in 0..words.len() {
            res.push(format!("{}{}{}", words[i], names[i], "}"));
        }
        writeln!(file, "{}", res.join(",")).expect("Failed to write to file");
    }
}
