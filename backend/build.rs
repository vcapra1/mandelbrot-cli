extern crate cc;

fn main() {
    cc::Build::new()
        .cuda(true)
        //.flag("-cudart=shared")
        //.flag("-gencode")
        //.flag("arch=compute_61,code=sm_61")
        .flag("--dont-use-profile")
        .flag("-ldir=/opt/cuda/nvvm/libdevice")
        .flag("-I/opt/cuda/include")
        //.flag("-rdc=true")
        .file("src/compute.cu")
        .compile("libcompute.a");

    println!("cargo:rustc-link-search=native=/opt/cuda/lib64");
	println!("cargo:rustc-link-lib=cudart");
}
