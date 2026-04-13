use std::path::PathBuf;

use content::build_content_database_and_bundle;

fn parse_args() -> (PathBuf, PathBuf) {
    let mut args = std::env::args().skip(1);

    let mut content_root = PathBuf::from("contents");
    let mut output_dir = PathBuf::from("target/content-build");

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--content-root" => {
                if let Some(value) = args.next() {
                    content_root = PathBuf::from(value);
                }
            }
            "--output-dir" => {
                if let Some(value) = args.next() {
                    output_dir = PathBuf::from(value);
                }
            }
            "--help" | "-h" => {
                println!(
                    "Usage: content-build [--content-root <path>] [--output-dir <path>]\n\nDefaults:\n  --content-root contents\n  --output-dir target/content-build"
                );
                std::process::exit(0);
            }
            _ => {}
        }
    }

    (content_root, output_dir)
}

fn main() {
    let (content_root, output_dir) = parse_args();

    match build_content_database_and_bundle(&content_root, &output_dir) {
        Ok((database, bundle)) => {
            println!("Content build generated successfully.");
            println!("Entries: {}", database.entries.len());
            println!("Bundle root: {}", bundle.root_dir.display());
            println!("Database: {}", bundle.db_path.display());
            println!("Public dir: {}", bundle.public_dir.display());
        }
        Err(error) => {
            eprintln!("Content build failed: {error}");
            std::process::exit(1);
        }
    }
}
