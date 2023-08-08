use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

fn focus_time(minutes: u8) {
    let focus_session_time_in_seconds = minutes * 60;
    let focus_time_message: &str = "Time to focus";
    session_loop(focus_session_time_in_seconds.into(), focus_time_message);
}

fn short_break_time(minutes: u8) {
    let focus_session_time_in_seconds = minutes * 60;
    let focus_time_message: &str = "Take a break ";
    session_loop(focus_session_time_in_seconds.into(), focus_time_message);
}

fn default_pomodoro_loop() {
    let focus_session_minutes: u8 = 25;
    let short_break_session_minutes: u8 = 5;
    focus_time(focus_session_minutes);
    short_break_time(short_break_session_minutes);
    focus_time(focus_session_minutes);
    short_break_time(short_break_session_minutes);
    focus_time(focus_session_minutes);
    short_break_time(short_break_session_minutes);
    focus_time(focus_session_minutes);
}

fn session_loop(session_time_in_seconds: u16, msg: &str) -> () {
    let mut elapsed_time = session_time_in_seconds;
    for _i in 0..session_time_in_seconds {
        let minutes = elapsed_time/60;
        let seconds = elapsed_time % 60;
        print!("\r{} {:02}:{:02}", msg, minutes, seconds);
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
        default_pomodoro_loop();
    }
    println!();
}
