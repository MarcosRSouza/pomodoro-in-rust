use std::io;
use std::thread::sleep;
use std::time::Duration;

fn focus_time_loop(session_time_in_seconds: u16) -> () {
    for i in 0..session_time_in_seconds {
        println!("Looping every second: {}", i);
        sleep(Duration::from_secs(1));
    }
}

fn main() {
    println!("Pomodoro App with Rust");

    println!("Choose the configuration mode:");
    println!("[1]: 25/5 sessions (default)");
    println!("[2]: custom");

    let mut mode = String::new();

    io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read line");

    println!("Choosed mode: {mode}");

    if mode == "1\n" {
        let session_time_in_seconds = 5 * 1; // TODO: change this time to actual 25 * 60 seconds
        focus_time_loop(session_time_in_seconds);
    }
}
