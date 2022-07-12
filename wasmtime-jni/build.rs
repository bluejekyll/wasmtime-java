use std::{borrow::Cow, error::Error, path::PathBuf};

use jaffi::Jaffi;

fn class_path() -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"))
        .join("../target/classes")
}

fn main() -> Result<(), Box<dyn Error>> {
    // only need this if you need to compile the java, this is needed for the integration tests...

    let class_path = class_path();
    let classes = vec![Cow::from("net.bluejekyll.wasmtime.WasmEngine")];
    let classes_to_wrap = vec![Cow::from("net.bluejekyll.wasmtime.AbstractOpaquePtr")];
    let output_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));

    eprintln!("using classpath: {}", class_path.display());

    let jaffi = Jaffi::builder()
        .native_classes(classes)
        .classes_to_wrap(classes_to_wrap)
        .classpath(vec![Cow::from(class_path)])
        .output_dir(Some(Cow::from(output_dir)))
        .build();

    jaffi.generate()?;

    Ok(())
}
