use std::os::unix::process;
use std::process::ExitStatus;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

// GPIO uses BCM pin numbering. BCM GPIO 2 is tied to physical pin 3.
const GPIO_LED: u8 = 2;

//uses libcamera-jpeg to take a picture named the current time
pub fn take_picture(value : &String ) -> ExitStatus {

use std::process::Command;

let mut binding = Command::new("libcamera-jpeg");
binding.current_dir("/home/newuser/remote");
return binding.arg("-o")
        .arg(value).arg("-t").arg("2000").arg("-q").arg("100").arg("--width").arg("1040").arg("--height").arg("1480")//
        .status().expect("failed");
}


//creates a string containging local time .jpg example : 2024-01-02T12:12.jpg
fn get_date_time() -> String {

use chrono::Local;
let time = Local::now().to_rfc3339();
let time =&time[0 ..16];
let time = {time.to_owned()+".jpg"}.to_string();
        time
//println!("TIME IS :{:#}",time );
}

fn actuate (val : bool){
    let gpio = Gpio::new().unwrap();
    let mut pin = gpio.get(GPIO_LED).unwrap().into_output();
    match val {
       true =>  pin.set_high(),
       false => pin.set_low(), 
    }
    let MOTOR_RUN_DURATION = 30;
    thread::sleep(Duration::from_secs(MOTOR_RUN_DURATION));
    pin.set_low();

}

//here it calls for the models binairy to process the image 
fn process_image (namie : &String ) -> bool {
let num = rand::random();
num
        

}
fn main() {

loop{
let time=get_date_time();        
take_picture(&time);
let act =process_image(&time);
actuate(act);        
thread::sleep(Duration::from_secs(1800));
    }

      
}
