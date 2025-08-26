use extendr_api::prelude::*;
use std::process::{Command};
use std::io::{BufRead, BufReader};

/// Exposes the `fqtk demux` functionality as a Rust function that can be called from R.
/// 
/// @param inputs A character vector of input FASTQ file paths.
/// @param max_mismatches An integer specifying the maximum number of mismatches allowed during demultiplexing.
/// @param read_structures A character vector specifying the read structures for parsing barcodes and sequences.
/// @param sample_metadata A string specifying the path to the CSV or TSV file containing sample metadata.
/// @param output A string specifying the output directory or file path for demultiplexed results.
/// 
/// @return A character string indicating success or the error message.
/// @export
#[extendr]
fn fqtk_demux(
    inputs: Vec<String>,              
    max_mismatches: usize,            
    read_structures: Vec<String>,      
    sample_metadata: String,           
    output: String                    
) -> Result<String> {

    let mut command = Command::new("fqtk");
    command.arg("demux");

    for input in inputs {
        command.arg("--inputs").arg(input);
    }

    for read_structure in read_structures {
        command.arg("--read-structures").arg(read_structure);
    }

    command.arg("--sample-metadata").arg(sample_metadata)
           .arg("--output").arg(output)
           .arg("--max-mismatches").arg(max_mismatches.to_string());
    
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());

    let mut child = command.spawn()
        .map_err(|e| Error::from(format!("Failed to run fqtk: {e}")))?;

    // stdout
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(l) = line {
                // print directly to R console
                rprintln!("{}", l);
            }
        }
    }

    // stderr
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(l) = line {
                rprintln!("stderr: {}", l);
            }
        }
    }

    let status = child.wait()
        .map_err(|e| Error::from(format!("Failed to wait for fqtk: {e}")))?;

    if status.success() {
        Ok("fqtk demux completed successfully.".to_string())
    } else {
        Err(Error::from(format!("fqtk demux failed: {:?}", status)))
    }
}

extendr_module! {
    mod fqtkWrapper;
    fn fqtk_demux;
}
