use std::env::var;
use std::fs;
use std::io::*;
use std::path::*;
use std::process::Command;

const SFMT: &'static str = "SFMT";
const SFMT_VERSION: &'static str = "1.5.1";
const DSFMT: &'static str = "dSFMT";
const DSFMT_VERSION: &'static str = "2.2.3";
const SITE: &'static str = "http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/SFMT";

fn download(url: &str, out: &Path) {
    let mut f = BufWriter::new(fs::File::create(out).unwrap());
    let p = Command::new("curl")
        .arg(url)
        .output()
        .expect("Failed to start download");
    f.write(&p.stdout).unwrap();
}

fn expand(archive: &str, out_dir: &Path) {
    let st = Command::new("tar")
        .args(&["xvf", archive])
        .current_dir(&out_dir)
        .status()
        .expect("Failed to start expanding archive");
    if !st.success() {
        panic!("Failed to expand archive");
    }
}

fn compile(c_code: &str, out_dir: &Path) {
    let warnings = ["-Wmissing-prototypes", "-Wall"];
    let optimize = [
        "-O3",
        "-finline-functions",
        "-fomit-frame-pointer",
        "-DNDEBUG",
        "-fno-strict-aliasing",
    ];
    let flags = ["-std=c99", "-fPIC"];
    let sse2 = ["-msse2", "-DHAVE_SSE2"];
    let st = Command::new("gcc")
        .args(&warnings)
        .args(&optimize)
        .args(&flags)
        .args(&sse2)
        .args(&["-c", &c_code])
        .current_dir(out_dir)
        .status()
        .expect("Failed to start gcc");
    if !st.success() {
        panic!("Failed to Compile {}", c_code);
    }
}

fn main() {
    let out_dir = PathBuf::from(var("OUT_DIR").unwrap());

    for &(name, version) in [(SFMT, SFMT_VERSION), (DSFMT, DSFMT_VERSION)].iter() {
        let archive = format!("{}-src-{}.tar.gz", name, version);
        download(&format!("{}/{}", SITE, archive), &out_dir.join(&archive));
        expand(&archive, &out_dir);
        compile(&format!("{}-src-{}/{}.c", name, version, name), &out_dir);
    }
    let st = Command::new("ar")
        .args(&["-rc", "libsfmt.a"])
        .args(&[SFMT, DSFMT]
            .iter()
            .map(|name| format!("{}.o", name))
            .collect::<Vec<_>>())
        .current_dir(&out_dir)
        .status()
        .expect("Failed to start ar");
    if !st.success() {
        panic!("Failed to create static library")
    }

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=sfmt");
}
