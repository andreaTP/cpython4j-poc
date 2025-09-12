use std::path::PathBuf;

fn main() {
    use wlr_libpy::bld_cfg::configure_static_libs;
    configure_static_libs().unwrap().emit_link_flags();


    // TODO: automate this: curl -L https://github.com/kateinoigakukun/wasi-vfs/releases/download/v0.5.0/libwasi_vfs-wasm32-unknown-unknown.zip --output libwasi_vfs-wasm32-unknown-unknown.zip
    let lib_dir = PathBuf::from("libs");
    println!("cargo:rerun-if-changed={}", lib_dir.join("libwasi_vfs.a").display());
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=wasi_vfs");
}
