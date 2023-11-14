use std::{env, path};

pub fn main() {
    println!("cargo:rerun-if-changed=src/fdlibm");
    println!("cargo:rerun-if-changed=build.rs");

    bindgen::Builder::default()
        .header("src/fdlibm/fdlibm.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .use_core()
        .no_convert_floats()
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/strict_math.rs")
        .expect("Couldn't write bindings!");

    let mut cfg = cc::Build::new();
    if env::var("CARGO_CFG_TARGET_ENDIAN").unwrap() == "little" {
        cfg.define("__LITTLE_ENDIAN", None);
    }
    cfg.include("src/fdlibm")
        .file(path::Path::new("src/fdlibm/w_acos.c"))
        .file(path::Path::new("src/fdlibm/e_acos.c"))
        .file(path::Path::new("src/fdlibm/e_sqrt.c"))
        .file(path::Path::new("src/fdlibm/w_sqrt.c"))
        .file(path::Path::new("src/fdlibm/w_asin.c"))
        .file(path::Path::new("src/fdlibm/e_asin.c"))
        .file(path::Path::new("src/fdlibm/s_fabs.c"))
        .file(path::Path::new("src/fdlibm/e_atan2.c"))
        .file(path::Path::new("src/fdlibm/w_atan2.c"))
        .file(path::Path::new("src/fdlibm/s_expm1.c"))
        .file(path::Path::new("src/fdlibm/e_hypot.c"))
        .file(path::Path::new("src/fdlibm/w_hypot.c"))
        .file(path::Path::new("src/fdlibm/e_sinh.c"))
        .file(path::Path::new("src/fdlibm/w_sinh.c"))
        .file(path::Path::new("src/fdlibm/e_exp.c"))
        .file(path::Path::new("src/fdlibm/w_exp.c"))
        .file(path::Path::new("src/fdlibm/s_tanh.c"))
        .file(path::Path::new("src/fdlibm/s_cbrt.c"))
        .file(path::Path::new("src/fdlibm/e_cosh.c"))
        .file(path::Path::new("src/fdlibm/w_cosh.c"))
        .file(path::Path::new("src/fdlibm/s_log1p.c"))
        .file(path::Path::new("src/fdlibm/e_log.c"))
        .file(path::Path::new("src/fdlibm/w_log.c"))
        .file(path::Path::new("src/fdlibm/s_atan.c"))
        .file(path::Path::new("src/fdlibm/k_tan.c"))
        .file(path::Path::new("src/fdlibm/s_tan.c"))
        .file(path::Path::new("src/fdlibm/e_rem_pio2.c"))
        .file(path::Path::new("src/fdlibm/k_rem_pio2.c"))
        .file(path::Path::new("src/fdlibm/s_floor.c"))
        .file(path::Path::new("src/fdlibm/s_cos.c"))
        .file(path::Path::new("src/fdlibm/k_cos.c"))
        .file(path::Path::new("src/fdlibm/k_sin.c"))
        .file(path::Path::new("src/fdlibm/s_sin.c"))
        .file(path::Path::new("src/fdlibm/e_log10.c"))
        .file(path::Path::new("src/fdlibm/w_log10.c"))
        .file(path::Path::new("src/fdlibm/e_pow.c"))
        .file(path::Path::new("src/fdlibm/w_pow.c"))
        .compile("fdlibm");
}
