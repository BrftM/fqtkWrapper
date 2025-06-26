
# fqtkWrapper

`fqtkWrapper` is an [extendr-based-wrapper](https://extendr.github.io/) R package around the command-line tool `fqtk`, developed by Fulcrum Genomics. The wrapper allows users to run `fqtk`'s demultiplexing functionalities directly from R, without having to manually invoke the CLI. This package simplifies the integration of `fqtk` into bioinformatics workflows, making it easier to automate tasks such as barcode demultiplexing within R-based data analysis pipelines.

This wrapper is based on the original `fqtk` tool, which is developed by Nils Homer, Tim Fennell, Jason Fan, and Matt Stone.

### Citation

If you use `fqtk` or this wrapper in your work, please cite the original tool as follows:

- Nils Homer, Tim Fennell, Jason Fan, & Matt Stone. (2024). *fulcrumgenomics/fqtk: v0.3.1 (v0.3.1)*. Zenodo. [https://doi.org/10.5281/zenodo.13345414](https://doi.org/10.5281/zenodo.13345414).

### Installation

You'll need to install Rust on your system if you don't have it already. Then you can install `fqtkWrapper` directly from GitHub using the `remotes` package in R.

```r
# Installation of remotes
if (!requireNamespace("remotes", quietly = TRUE)) {
  install.packages("remotes")
}

# Installation from GitHub in the same way as pepitope
remotes::install_github("BrftM/fqtkWrapper")
````

Use it like:
```r

# Load the package
library(fqtk)

# Define the input parameters e.g.
inputs <- c("input_file.fastq.gz")
read_structures <- c("7B+T")
sample_metadata <- "samples.tsv"
output <- "output_folder"

# Call function
fqtk_demux(
    inputs = inputs,
    max_mismatches = max_mismatches,
    read_structures = read_structures,
    sample_metadata = sample_metadata,
    output = output
)
```

For more informations check officiall documentation at [fqtk´s github](https://github.com/fulcrumgenomics/fqtk) or at use cases like [pepitope](https://mschubert.github.io/pepitope/).