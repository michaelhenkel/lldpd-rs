extern crate bindgen;

use std::io;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    
    // Generate Rust bindings for lldpctl.h
    let bindings = bindgen::Builder::default()
        .header("/usr/include/lldpctl.h") // Path to the lldpctl.h file
        .generate()
        .expect("Unable to generate bindings");

        let out_path = PathBuf::from("src/bindings/");
        let bindings_path = out_path.join("bindings.rs");
        let mut file = File::create(&bindings_path)?;
        writeln!(file, "#![allow(non_camel_case_types)]")?;
        writeln!(file, "#![allow(non_upper_case_globals)]")?;
        writeln!(file, "#![allow(non_snake_case)]")?;
        writeln!(file, "#![allow(dead_code)]")?;
        writeln!(file, "#![allow(improper_ctypes)]")?;
        
        // Write the bindings to the file
        file.write_all(bindings.to_string().as_bytes())?;
    
    // Link with the lldpctl library
    println!("cargo:rustc-link-lib=lldpctl");
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");

    Ok(())
}