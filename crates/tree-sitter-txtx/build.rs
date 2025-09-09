use std::path::PathBuf;

fn main() {
    let src_dir = PathBuf::from("src");

    let mut config = cc::Build::new();
    config.include(&src_dir);
    config.file(src_dir.join("parser.c"));

    // tree-sitter uses C99
    config.std("c99");

    config.compile("tree-sitter-txtx");
}
