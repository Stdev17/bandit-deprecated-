

fn main() {
    println!("cargo:rustc-flags=-lgcc_s");
    cc::Build::new()
        .file("src/img/crc.c")
        .compile("crc");
}