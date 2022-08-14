use curl::easy::{Easy, SslOpt};
use std::ffi::OsStr;
pub enum WarpModes {
    On,
    Off,
}
pub enum InternetStatus {
    Some(WarpModes),
    None,
}
pub fn internet_status() -> InternetStatus {
    let (data, handle) = match get_cdn_trace() {
        Ok(value) => value,
        Err(value) => return value,
    };
    find_warp(handle, data)
}

fn find_warp(mut handle: Easy, mut data: Vec<u8>) -> InternetStatus {
    {
        let mut transfer = handle.transfer();
        let ret = transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .err()
            .is_some();

        if ret {
            return InternetStatus::None;
        }
        if transfer.perform().err().is_some() {
            return InternetStatus::None;
        }
    }
    // Convert it to `String`
    let body = String::from_utf8(data).expect("body is not valid UTF8!");
    if body.contains("warp=on") {
        InternetStatus::Some(WarpModes::On)
    } else if body.contains("warp=off") {
        InternetStatus::Some(WarpModes::Off)
    } else {
        InternetStatus::None
    }
}

fn get_cdn_trace() -> Result<(Vec<u8>, Easy), InternetStatus> {
    let data = Vec::new();
    let mut handle = Easy::new();
    let mut ssl_opts = SslOpt::new();
    ssl_opts.no_revoke(true);
    ssl_opts.allow_beast(true);
    handle.ssl_options(&ssl_opts).unwrap();
    if handle
        .url("https://www.cloudflare.com/cdn-cgi/trace")
        .is_err()
    {
        return Err(InternetStatus::None);
    }
    Ok((data, handle))
}
pub fn warpctl(mode: WarpModes) {
    let out = std::process::Command::new("warp-cli")
        .arg(mode)
        .stdout(std::process::Stdio::piped())
        .output()
        .unwrap();
    log::trace!(
        "stdout:- \n{:?}\n stderr:- \n{:?}",
        String::from_utf8(out.stdout),
        String::from_utf8(out.stderr)
    );
}
impl AsRef<OsStr> for WarpModes {
    fn as_ref(&self) -> &OsStr {
        match self {
            WarpModes::On => OsStr::new("connect"),
            WarpModes::Off => OsStr::new("disconnect"),
        }
    }
}
