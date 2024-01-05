fn main() {

use std::process::Command;

let mut binding = Command::new("libcamera-jpeg");
binding.current_dir("/home/newuser/remote");
let _capture_img = binding.arg("-o")
        .arg("test.jpg").arg("-t").arg("2000").arg("-q").arg("100").arg("--width").arg("640").arg("--height").arg("480")//
        .status().expect("failed");


//let _ = capture_img.arg("-o")
 //       .arg("~/remote/test.png").status().expect("failed");
// Execute `ls` in the current directory of the program.
//list_dir.status().expect("process failed to execute");

//println!();

// Change `ls` to execute in the root directory.
//list_dir.current_dir("/");

// And then execute `ls` again but in the root directory.
//list_dir.status().expect("process failed to execute");
}
