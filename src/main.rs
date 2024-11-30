use std::process::Command;
use std::net::TcpStream;
use std::net::Shutdown;
use std::process::Stdio;

fn main() {

    let mut chrome_driver = Command::new("./chromedriver")
        .spawn()
        .expect("Failed to start chromedriver");

    //let mut child = chrome_driver.stdout.expect("Failed to get chromedriver stdout");
    let output = chrome_driver.wait_with_output().expect("err");
    let output = String::from_utf8(output.stdout.as_slice().to_vec()).expect("sdf");

    if let Ok(stream) = TcpStream::connect("127.0.0.1:9515") {
        println!("Connected to chromedriver");
        stream.shutdown(Shutdown::Both).expect("Failed to shutdown the TCP stream");
    } else {
        println!("Failed to connect to chromedriver");
    }

    chrome_driver.kill().expect("Failed to stop chromedriver");

    println!("Exited");

}
