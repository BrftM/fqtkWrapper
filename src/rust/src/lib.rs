use extendr_api::prelude::*;
use std::process::{Command};

/// Function to call the fqtk CLI command for demultiplexing.
/// Exposes the demux functionality as a Rust function that can be called from R.
/// @export
#[extendr]
fn fqtk_demux(
    inputs: Vec<String>,               // List of input files
    max_mismatches: usize,             // Max mismatches allowed
    read_structures: Vec<String>,      // List of read structures
    sample_metadata: String,           // Path to sample metadata file
    output: String                     // Output directory
) -> String {

    // Build the `fqtk demux` command using the provided parameters
    let mut command = Command::new("fqtk");
    command.arg("demux");

    // Add inputs (path to input files)
    for input in inputs {
        command.arg("--inputs").arg(input);
    }

    // Add read structures
    for read_structure in read_structures {
        command.arg("--read-structures").arg(read_structure);
    }

    // Add other parameters
    command.arg("--sample-metadata").arg(sample_metadata)
           .arg("--output").arg(output)
           .arg("--max-mismatches").arg(max_mismatches.to_string());

    // Execute the command
    match command.output() {
        Ok(output) => {
            if output.status.success() {
                "Demux operation completed successfully.".to_string()
            } else {
                let err_msg = String::from_utf8_lossy(&output.stderr);
                format!("Demux failed: {}", err_msg)
            }
        }
        Err(e) => format!("Failed to execute command: {}", e),
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod fqtkWrapper;
    fn fqtk_demux;
}
