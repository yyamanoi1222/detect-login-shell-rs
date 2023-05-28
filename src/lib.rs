use std::env;

pub fn detect() -> Result<String, String> {
    let output: Result<String, String> = if cfg!(target_os = "windows") {
        Result::Ok(match env::var("COMSPEC") {
            Ok(val) => val,
            Err(_) => String::from("cmd.exe"),
        })
    } else if cfg!(target_os = "linux")
        || cfg!(target_os = "macos")
        || cfg!(target_os = "freebsd")
        || cfg!(target_os = "openbsd")
    {
        match env::var("SHELL") {
            Ok(val) => {
                match val.split('/').last() {
                    Some(shell) => Result::Ok(shell.to_string()),
                    None => Result::Err("SHELL not found".to_string())
                }
            },
            Err(_) => Result::Err("SHELL not found".to_string())
        }
    } else {
        Result::Err("Unsupported OS".to_string())
    };

    output
}
