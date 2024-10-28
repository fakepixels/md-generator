use std::env;
use std::fs::{self, File};
use std::io::Write;
use anyhow::{Context, Result};
use walkdir::WalkDir;

fn main() -> Result<()> {
    // Get the directory path from command line arguments
    let args: Vec<String> = env::args().collect();
    let dir_path = args.get(1).context("Please provide a directory path")?;
    
    // Create output file
    let output_path = "combined.mdx";
    let mut output_file = File::create(output_path)
        .context("Failed to create output file")?;
    
    // Walk through directory
    for entry in WalkDir::new(dir_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok()) {
            
        let path = entry.path();
        
        // Check if file is an .mdx file
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("mdx") {
            // Read file contents
            let contents = fs::read_to_string(path)
                .with_context(|| format!("Failed to read file: {}", path.display()))?;
            
            // Write file name as comment
            writeln!(output_file, "\n<!-- File: {} -->\n", path.display())
                .context("Failed to write file name")?;
            
            // Write contents
            writeln!(output_file, "{}\n", contents)
                .context("Failed to write contents")?;
        }
    }
    
    println!("Successfully combined MDX files into: {}", output_path);
    Ok(())
}
