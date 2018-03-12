pub static TEXT: &'static str = "#!/bin/sh
watchexec --exts rs,toml,sql,tera --restart \"cargo build --features clippy && RUST_BACKTRACE=1 cargo run\"
";
