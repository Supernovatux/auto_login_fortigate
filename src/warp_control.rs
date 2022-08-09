use curl::easy::{Easy, SslOpt};
use std::ffi::OsStr;
pub enum WarpModes {
    Start,
    Stop,
}
pub fn chck_warp() -> bool {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    let mut ssl_opts = SslOpt::new();
    ssl_opts.no_revoke(true);
    ssl_opts.allow_beast(true);
    handle.ssl_options(&ssl_opts).unwrap();
    handle
        .url("https://www.cloudflare.com/cdn-cgi/trace")
        .unwrap();
    {
        let mut transfer = handle.transfer();
        if transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .err().is_some() {
                return false;
            }
        if transfer.perform().err().is_some() {
            return false;
        }
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
    std::process::Command::new("warp-cli")
        .arg(mode)
        .output()
        .unwrap();
}
impl AsRef<OsStr> for WarpModes {
    fn as_ref(&self) -> &OsStr {
        match self {
            WarpModes::Start => OsStr::new("connect"),
            WarpModes::Stop => OsStr::new("disconnect"),
        }
    }
}
