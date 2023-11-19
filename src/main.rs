use anyhow::{Context, Result};
use clap::Parser;

// Search for pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // PathBuf is like a String but for file system paths that work cross-platform
    // The path to the file to read
    path: std::path::PathBuf,
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let _ = find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n")
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<(), std::io::Error> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    return Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let _ = find_matches(&content, &args.pattern, &mut std::io::stdout());
    return Ok(());
}
