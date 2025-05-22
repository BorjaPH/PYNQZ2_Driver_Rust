// build.rs

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());

    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:warning=Compilando para: {}", target);

    if target.contains("armv7") {
        println!("cargo:warning=Compilación detectada para ARMv7 (PYNQ-Z2)");
    } else {
        println!("cargo:warning=Compilación NO es para PYNQ-Z2 (ARMv7). Usa cross-compiling si es necesario.");
    }
}
