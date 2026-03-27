use std::env;
use std::path::Path;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_os == "windows" {
        let mut res = winres::WindowsResource::new();
        let icon_path = "assets\\icons\\icon_test_examer.ico";
        if Path::new(icon_path).exists() {
            res.set_icon(icon_path);
            if let Err(e) = res.compile() {
                eprintln!("Error compiling resources: {}", e);
                std::process::exit(1);
            }
        }
        else {
            println!("cargo:warning=Warning ico file did not find: {}", icon_path);

        }
    }
}