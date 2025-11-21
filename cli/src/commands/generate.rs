use anyhow::{Context, Result};
use colored::Colorize;
use std::path::PathBuf;
use std::process::Command;

/// Generate a docpack from a source file
pub fn run(input: PathBuf) -> Result<()> {
    // Verify input exists
    if !input.exists() {
        anyhow::bail!("Input file does not exist: {:?}", input);
    }

    // Verify it's a zip file
    if input.extension().and_then(|s| s.to_str()) != Some("zip") {
        anyhow::bail!("Input must be a .zip file, got: {:?}", input);
    }

    println!("\n{}", "Generating Docpack".bright_cyan().bold());
    println!("{}", format!("Input: {:?}", input).bright_black());
    println!("{}", "=".repeat(80).bright_black());

    // Find the builder binary
    let builder_path = find_builder_binary()?;
    println!(
        "\n{}",
        format!("Using builder: {:?}", builder_path).bright_black()
    );

    // Run the builder
    println!("\n{}", "Running builder...".bright_yellow());
    let status = Command::new(&builder_path)
        .arg(input.to_string_lossy().as_ref())
        .status()
        .context("Failed to execute builder")?;

    if !status.success() {
        anyhow::bail!("Builder failed with exit code: {:?}", status.code());
    }

    println!(
        "\n{}",
        "✓ Docpack generation complete!".bright_green().bold()
    );
    println!(
        "{}",
        "Use 'localdoc list' to see installed docpacks".bright_black()
    );

    Ok(())
}

/// Find the builder binary in common locations
fn find_builder_binary() -> Result<PathBuf> {
    // Try common locations
    let candidates = vec![
        // Release build in builder directory
        PathBuf::from("../builder/target/release/doctown-builder"),
        PathBuf::from("builder/target/release/doctown-builder"),
        // Debug build in builder directory
        PathBuf::from("../builder/target/debug/doctown-builder"),
        PathBuf::from("builder/target/debug/doctown-builder"),
        // System path
        PathBuf::from("doctown-builder"),
    ];

    for candidate in candidates {
        // Check if it's an absolute or relative path that exists
        if candidate.exists() {
            return Ok(candidate);
        }

        // If it's just a binary name, check if it's in PATH
        if candidate.file_name().is_some() && candidate.parent().is_none() {
            if let Ok(output) = Command::new("which")
                .arg(candidate.to_string_lossy().as_ref())
                .output()
            {
                if output.status.success() {
                    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !path.is_empty() {
                        return Ok(PathBuf::from(path));
                    }
                }
            }
        }
    }

    Err(anyhow::anyhow!(
        "Could not find builder binary. Please build it first:\n  \
        cd builder && cargo build --release"
    ))
}
