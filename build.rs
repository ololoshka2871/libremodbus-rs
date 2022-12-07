use std::{env, fs, path::PathBuf};

fn generate_config() -> PathBuf {
    let outpath = PathBuf::from(env::var("OUT_DIR").unwrap());
    let infile = "include/mbconfig.h.in";

    let cfg = fs::read_to_string("include/mbconfig.h.in")
        .expect(format!("Failed to read {}", infile).as_str());

    let out_cfg = cfg
        .replace(
            "%MB_ASCII_ENABLED%",
            if cfg!(feature = "ascii") { "1" } else { "0" },
        )
        .replace(
            "%MB_RTU_ENABLED%",
            if cfg!(feature = "rtu") { "1" } else { "0" },
        )
        /*
        .replace(
            "%MB_TCP_ENABLED%",
            if cfg!(feature = "tcp") { "1" } else { "0" },
        )*/.replace(
            "%MB_INPUT_EN%",
            if cfg!(feature = "inputs") { "1" } else { "0" },
        ).replace(
            "%MB_HOLDINGS_EN%",
            if cfg!(feature = "holdings") { "1" } else { "0" },
        ).replace(
            "%MB_COILS_EN%",
            if cfg!(feature = "coils") { "1" } else { "0" },
        ).replace(
            "%MB_DISCRETE_EN%",
            if cfg!(feature = "d_inputs") { "1" } else { "0" },
        );

    let mut out_file = outpath.clone();
    out_file.push("mbconfig.h");

    fs::write(out_file.clone(), out_cfg)
        .expect(format!("Failed to write {}", out_file.to_str().unwrap()).as_str());

    outpath
}

fn main() {
    let mut src = vec![
        "libremodbus-dist/mb.c",
        "libremodbus-dist/functions/mbfunccoils.c",
        "libremodbus-dist/functions/mbfuncdiag.c",
        "libremodbus-dist/functions/mbfuncdisc.c",
        "libremodbus-dist/functions/mbfuncholding.c",
        "libremodbus-dist/functions/mbfuncinput.c",
        "libremodbus-dist/functions/mbfuncother.c",
        "libremodbus-dist/functions/mbutils.c",
    ];

    if cfg!(feature = "rtu") {
        src.push("libremodbus-dist/rtu/mbcrc.c");
        src.push("libremodbus-dist/rtu/mbrtu.c");
    }

    if cfg!(feature = "ascii") {
        src.push("libremodbus-dist/ascii/mbascii.c");
    }

    /*
    if cfg!(feature = "tcp") {
        src.push("libremodbus-dist/tcp/mbtcp.c");
    }
    */

    let cfg_dir = generate_config();

    let mut inc = vec![
        cfg_dir.to_str().unwrap(),
        "include",
        "libremodbus-dist/include",
    ];

    if cfg!(feature = "rtu") {
        inc.push("libremodbus-dist/rtu");
    }

    if cfg!(feature = "ascii") {
        inc.push("libremodbus-dist/ascii");
    }

    /*
    if cfg!(feature = "tcp") {
        inc.push("libremodbus-dist/tcp");
    }
    */

    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())
        .includes(inc)
//        .flag("-Wno-unused-parameter")
//        .flag("-fno-aggressive-loop-optimizations")
//        .flag("-Wno-stringop-overflow")
//        .define("SOME_MACRO", Some("0"))
        ;
    build.compile("libremodbus");
}
