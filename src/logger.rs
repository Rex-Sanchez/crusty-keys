use std::{fs::OpenOptions, io::Write, path::PathBuf};

pub(crate) fn log<S: Into<String>>(msg: S) {
    let msg = msg.into();
    println!("{msg}");

    if let Some(home) = std::env::home_dir() {
        let log = home.join(PathBuf::from(".config/crusty-keys/log.log"));

        if let Ok(v) = std::fs::exists(&log)
            && v
        {
            if let Ok(mut file) = OpenOptions::new().append(true).open(&log) {
                let _ = writeln!(file, "{msg}");
            }else if let Ok(mut log) = std::fs::File::open(log){
                let _ = log.write_all(msg.as_bytes());
            }
        } else if let Ok(mut file) = std::fs::File::create_new(log) {
            let _ = file.write_all(msg.as_bytes());
        }
    }
}
