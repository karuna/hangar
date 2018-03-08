use std::str;
use std::process::Command;

pub fn execute(args: &Vec<String>) -> Result<String, String> {
    let mut command = Command::new("cargo");
    let output = if args.len() > 2 {
        command.args(&args[2..]).output()
    } else {
        command.arg("build").output()
    };
    match output {
        Ok(result) => {
            if !&result.stdout.is_empty() {
                let stdout = String::from_utf8_lossy(&result.stdout);
                let converted_stdout = str::replace(&stdout, "cago build", "hangar build");
                println!("{}", converted_stdout);
            }
            if !&result.stderr.is_empty() {
                let stderr = String::from_utf8_lossy(&result.stderr);
                let converted_stderr = str::replace(&stderr, "cago build", "hangar build");
                println!("{}", converted_stderr);
            }
            Ok(format!("{:?}", result))
        }
        Err(e) => Err(format!("{:?}", e)),
    }
}
