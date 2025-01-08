// Static Website Generator in Rust

use std::fs;
use std::path::Path;

fn main() {
    println!("Static Website Generator");

    let markdown_dir = "markdown_files";
    let output_dir = "output_html";

    create_directory(markdown_dir);
    create_directory(output_dir);

    loop {
        println!("Enter the full path of a Markdown file to process or type 'done' to finish:");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("done") {
            break;
        }

        let source_path = Path::new(input);
        if source_path.exists() && source_path.extension().unwrap_or_default() == "md" {
            let file_name = source_path.file_name().unwrap().to_str().unwrap();
            let destination_path = format!("{}/{}", markdown_dir, file_name);

            fs::copy(&source_path, &destination_path).unwrap();
            println!("Copied file to: {}", destination_path);
        } else {
            println!("Invalid file path or file is not a Markdown file. Please try again.");
        }
    }

    let markdown_files = fs::read_dir(markdown_dir).unwrap();

    for file in markdown_files {
        let file = file.unwrap();
        let file_path = file.path();

        if file_path.extension().unwrap_or_default() == "md" {
            let file_name = file_path.file_stem().unwrap().to_str().unwrap();
            let markdown_content = fs::read_to_string(&file_path).unwrap();
            let html_content = markdown_to_html(&markdown_content);

            let output_file = format!("{}/{}.html", output_dir, file_name);
            fs::write(&output_file, html_content).unwrap();

            println!("Generated: {}", output_file);
        }
    }

    println!("HTML files generated in the '{}' directory.", output_dir);
}

fn create_directory(dir: &str) {
    if !Path::new(dir).exists() {
        fs::create_dir(dir).unwrap();
        println!("Created directory: {}", dir);
    }
}

fn markdown_to_html(markdown: &str) -> String {
    let mut html = String::new();
    html.push_str("<html>\n<head>\n<title>Static Page</title>\n</head>\n<body>\n");

    for line in markdown.lines() {
        if line.starts_with("# ") {
            html.push_str(&format!("<h1>{}</h1>\n", &line[2..]));
        } else if line.starts_with("## ") {
            html.push_str(&format!("<h2>{}</h2>\n", &line[3..]));
        } else if line.starts_with("### ") {
            html.push_str(&format!("<h3>{}</h3>\n", &line[4..]));
        } else if line.trim().is_empty() {
            html.push_str("<br/>\n");
        } else {
            html.push_str(&format!("<p>{}</p>\n", line));
        }
    }

    html.push_str("</body>\n</html>\n");
    html
}
