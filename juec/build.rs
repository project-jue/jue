fn main() {
    println!("cargo:rerun-if-changed=src/frontend/jue.pest");
    println!("cargo:rerun-if-changed=src/frontend/parser.rs");
}
