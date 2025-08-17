use std::{io::Write, path::PathBuf};

pub(crate) fn log<S: Into<String>>(msg: S) {
    let msg = msg.into();
    if let Some(home) = std::env::home_dir() {
        let log = home.join(PathBuf::from(".config/crusty-keys/log.log"));

        if let Ok(v) = std::fs::exists(&log)
            && v
        {
            let _ = std::fs::write(log, msg.as_bytes());
        } else if let Ok(mut file) = std::fs::File::create_new(log) {
            let _ = file.write_all(msg.as_bytes());
        }
    }
}
