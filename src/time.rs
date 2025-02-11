// file to set all chrono based functionality

use chrono::Local;

fn get_today_chrono() -> String {
    Local::now().format("%d-%m-%Y").to_string()
}

// Example usage
fn main() {
    println!("{}", get_today_chrono());
}
