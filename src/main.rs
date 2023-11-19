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

fn main() {
    let args = Cli::parse();

    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => content,
        Err(error) => {
            panic!("Oh no!: {}", error)
        }
    };
    
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
