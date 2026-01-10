pub fn main() {
    #[cfg(all(target_os = "windows", feature = "runtime-d3d12"))]
    {
        println!("cargo:rustc-link-lib=dylib=delayimp");
        println!("cargo:rustc-link-arg=/DELAYLOAD:dxcompiler.dll");
        println!("cargo:rustc-link-arg=/DELAYLOAD:d3d12.dll");
    }

    // Debug build setup
    // todo: figure out the conditional to auto apply
    /*
    // Don't link the default CRT
    println!("cargo::rustc-link-arg=/nodefaultlib:msvcrt");
    // Link the debug CRT instead
    println!("cargo::rustc-link-arg=/defaultlib:msvcrtd");
    */
}
