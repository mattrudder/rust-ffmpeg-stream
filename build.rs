use std::path::{Path, PathBuf};
use std::{fs, env};
use std::ffi::OsStr;

// TODO: Make this configurable;
// for now, download from https://mattrudder.com/libs/vs2019/FFmpeg-4.2.1.zip
// and extract to C:/dev/lib/FFmpeg-4.2.1
const FFMPEG_PATH: &'static str = "Z:\\dev\\lib\\FFmpeg-4.2.1";
fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("pc-windows") {
        println!("cargo:rustc-link-search={}\\bin", FFMPEG_PATH);
        println!("cargo:rustc-env=CFLAGS=/I{}\\include", FFMPEG_PATH);

        let mut manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        manifest_dir.push("target");
        manifest_dir.push(env::var("PROFILE").unwrap());
        
        for entry in fs::read_dir(Path::new(FFMPEG_PATH).join("bin")).unwrap() {
            if let Ok(x) = entry {
                if let Some("dll") = x.path().extension().and_then(OsStr::to_str) {
                    copy(&manifest_dir, &x.path());
                }
            }
        }
    }
}


fn copy<S: AsRef<std::ffi::OsStr> + ?Sized, P: Copy + AsRef<Path>>(target_dir_path: &S, file_name: P) {
    if let Some(base_name) = file_name.as_ref().file_name() {
        fs::copy(file_name, Path::new(&target_dir_path).join(base_name)).unwrap();
    }
}
