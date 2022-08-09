use curl::easy::Easy;
use std::ffi::OsStr;
pub enum WarpModes {
    Start,
    Stop,
}
pub fn chck_warp() -> bool {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle
        .url("https://www.cloudflare.com/cdn-cgi/trace")
        .unwrap();
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    // Convert it to `String`
    let body = String::from_utf8(data).expect("body is not valid UTF8!");
    if body.find("warp=on").is_some() {
        true
    } else {
        false
    }
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
