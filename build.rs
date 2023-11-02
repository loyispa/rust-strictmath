use std::path::PathBuf;
use std::{env, path};

pub fn main() {
    println!("cargo:rerun-if-changed=src/fdlibm");

    cc::Build::new()
        .include("src/fdlibm")
        .define("_IEEE_LIBM", None)
        .define("__LITTLE_ENDIAN", None)
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
        .compile("fdlibm");
}
