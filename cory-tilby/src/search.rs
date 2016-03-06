use repository;
use std::process::{Command, Output};

#[cfg(target_os = "macos")]
pub fn for_relevant(needle: &str) -> Output {

    let output = Command::new("mdfind")
        .arg("-onlyin")
        .arg(repository::TILBY)
        .arg(&needle).output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });

    {
        let stdout = String::from_utf8_lossy(&output.stdout);

        let split = stdout.split("\n");
        for s in split {
          println!("{}", s)
        }
    }
    // println!("status: {}", output.status);
    // println!("stdout: \n{}", String::from_utf8_lossy(&output.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    return output;
}
