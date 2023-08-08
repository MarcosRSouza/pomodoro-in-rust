use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

fn focus_time_loop(session_time_in_seconds: u16) -> () {
    let mut elapsed_time = session_time_in_seconds;
    for _i in 0..session_time_in_seconds {
        let minutes = elapsed_time/60;
        let seconds = elapsed_time % 60;
        print!("\r{}:{}", minutes, seconds);
        io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
        elapsed_time -= 1;
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

    if mode == "1\n" {
        let session_time_in_seconds = 5 * 1; // TODO: change this time to actual 25 * 60 seconds
        focus_time_loop(session_time_in_seconds);
    }
    println!();
}
