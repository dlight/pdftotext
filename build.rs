fn main() {
    if let Ok(_) = std::env::var("DOCS_RS") {
        return;
    }
    
    let poppler = pkg_config::Config::new()
    .cargo_metadata(true)
    .probe("poppler")
    .expect("pkg-config could not find poppler");

    let mut build = cc::Build::new();

    let mut callpoppler = build
        .cpp(true)
        .file("src/callpoppler.cc");
        
    for dir in &poppler.include_paths {
        callpoppler = callpoppler.include(dir);
    }

    callpoppler.compile("callpoppler.a");

    //shouldn't cc take care of this?
    println!("cargo:rerun-if-changed=src/callpoppler.cc");
}