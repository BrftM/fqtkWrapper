// We need to forward routine registration from C to Rust
// to avoid the linker removing the static library.

void R_init_fqtk_wrapper_extendr(void *dll);

void R_init_fqtk_wrapper(void *dll) {
    R_init_fqtk_wrapper_extendr(dll);
}
