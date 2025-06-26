use extendr_api::prelude::*;
use std::process::{Command};

/// @export
#[extendr]
fn fqtk_demux(
    inputs: Vec<String>,              
    max_mismatches: usize,            
    read_structures: Vec<String>,      
    sample_metadata: String,           
    output: String                    
) -> String {

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

extendr_module! {
    mod fqtkWrapper;
    fn fqtk_demux;
}
