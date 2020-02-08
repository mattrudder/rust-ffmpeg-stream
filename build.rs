// TODO: Make this configurable;
// for now, download from https://mattrudder.com/libs/vs2019/FFmpeg-4.2.1.zip
// and extract to C:/dev/lib/FFmpeg-4.2.1
const FFMPEG_PATH: &'static str = "C:\\dev\\lib\\FFmpeg-4.2.1";
fn main() {
    println!("cargo:rustc-env=CFLAGS=/I{}\\include", FFMPEG_PATH);
    println!("cargo:rustc-link-search={}\\bin", FFMPEG_PATH);
}
