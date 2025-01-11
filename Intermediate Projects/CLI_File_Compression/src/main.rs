use clap::{Arg, Command};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("CLI File Compression Tool")
        .version("1.0")
        .author("Your Name")
        .about("Compresses and decompresses files")
        .arg(
            Arg::new("action")
                .short('a')
                .long("action")
                .value_name("ACTION")
                .help("Specifies the action: compress or decompress")
                .required(true),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("INPUT")
                .help("Path to the input file")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .help("Path to the output file")
                .required(true),
        )
        .get_matches();

    let action = matches.get_one::<String>("action").unwrap();
    let input_path = matches.get_one::<String>("input").unwrap();
    let output_path = matches.get_one::<String>("output").unwrap();

    match action.as_str() {
        "compress" => compress_file(input_path, output_path),
        "decompress" => decompress_file(input_path, output_path),
        _ => Err("Invalid action. Use 'compress' or 'decompress'.".into()),
    }
}

fn compress_file(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let input_file = File::open(input)?;
    let buffered_reader = BufReader::new(input_file);

    let output_file = File::create(output)?;
    let buffered_writer = BufWriter::new(output_file);
    let mut encoder = GzEncoder::new(buffered_writer, Compression::default());

    io::copy(&mut buffered_reader.take(u64::MAX), &mut encoder)?;
    encoder.finish()?;

    println!("File compressed successfully to {}", output);
    Ok(())
}

fn decompress_file(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let input_file = File::open(input)?;
    let buffered_reader = BufReader::new(input_file);
    let mut decoder = GzDecoder::new(buffered_reader);

    let output_file = File::create(output)?;
    let mut writer = BufWriter::new(output_file);

    io::copy(&mut decoder, &mut writer)?;
    writer.flush()?;

    println!("File decompressed successfully to {}", output);
    Ok(())
}



#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;
    use assert_cmd::Command;
    use tempfile::tempdir;

    #[test]
    fn test_compress_decompress() {
        let dir = tempdir().unwrap();
        let input_path = dir.path().join("example.txt");
        let compressed_path = dir.path().join("example.gz");
        let decompressed_path = dir.path().join("output.txt");

        // Create a sample input file
        let mut file = File::create(&input_path).unwrap();
        writeln!(file, "Hello, World!").unwrap();

        // Compress the file
        Command::cargo_bin("CLI_File_Compression")
            .unwrap()
            .args(&[
                "--action",
                "compress",
                "--input",
                input_path.to_str().unwrap(),
                "--output",
                compressed_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        // Decompress the file
        Command::cargo_bin("CLI_File_Compression")
            .unwrap()
            .args(&[
                "--action",
                "decompress",
                "--input",
                compressed_path.to_str().unwrap(),
                "--output",
                decompressed_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        // Verify content matches
        let original_content = fs::read_to_string(&input_path).unwrap();
        let decompressed_content = fs::read_to_string(&decompressed_path).unwrap();
        assert_eq!(original_content, decompressed_content);
    }

    #[test]
    fn test_invalid_action() {
        Command::cargo_bin("CLI_File_Compression")
            .unwrap()
            .args(&["--action", "invalid", "--input", "example.txt", "--output", "output.gz"])
            .assert()
            .failure();
    }
}
