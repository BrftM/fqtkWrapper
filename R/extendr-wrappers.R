# Generated by extendr: Do not edit by hand

# nolint start

#
# This file was created with the following call:
#   .Call("wrap__make_fqtkWrapper_wrappers", use_symbols = TRUE, package_name = "fqtkWrapper")

#' @usage NULL
#' @useDynLib fqtkWrapper, .registration = TRUE
NULL

#' Function to call the fqtk CLI command for demultiplexing.
#' Exposes the demux functionality as a Rust function that can be called from R.
#' @export
fqtk_demux <- function(inputs, max_mismatches, read_structures, sample_metadata, output) .Call(wrap__fqtk_demux, inputs, max_mismatches, read_structures, sample_metadata, output)


# nolint end
