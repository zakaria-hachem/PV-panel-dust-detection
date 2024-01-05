fn main() {

use std::process::Command;

let mut binding = Command::new("libcamera-jpeg");
binding.current_dir("/home/newuser/remote");
let _capture_img = binding.arg("-o")
        .arg("test.jpg").arg("-t").arg("2000").arg("-q").arg("100").arg("--width").arg("640").arg("--height").arg("480")//
        .status().expect("failed");



}
