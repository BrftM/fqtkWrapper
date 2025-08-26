use extendr_api::prelude::*;
use std::process::{Command};

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
) -> RobjResult<String> {

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

    match command.output() {
        Ok(out) => {
            if out.status.success() {
                "Demux operation completed successfully.".to_string()
            } else {
                let code = out.status.code().unwrap_or(-1);
                let err_msg = String::from_utf8_lossy(&out.stderr).to_string();
                Err(Error::from(format!(
                    "fqtk demux failed (exit code {}): {}",
                    code, err_msg
                )))
            }
        }
        Err(e) => Err(Error::from(format!("Failed to execute 'fqtk': {e}"))),
    }
}

extendr_module! {
    mod fqtkWrapper;
    fn fqtk_demux;
}
