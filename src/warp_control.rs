use std::ffi::OsStr;

pub enum WarpModes {
    Start,
    Stop,
}
pub fn warpctl(mode: WarpModes) {
    let output = std::process::Command::new("warp-cli")
        .arg(mode)
        .output()
        .unwrap();
    dbg!(output);
}
impl AsRef<OsStr> for WarpModes {
    fn as_ref(&self) -> &OsStr {
        match self {
            WarpModes::Start => OsStr::new("connect"),
            WarpModes::Stop => OsStr::new("disconnect"),
        }
    }
}
