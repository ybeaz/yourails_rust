use chrono::Local;

// %Y   → year
// %m   → month
// %d   → day
// %H   → hour
// %M   → minute
// %S   → second
// %3f  → milliseconds
pub fn current_datetime_string() -> String {
    Local::now().format("%Y-%m-%d-%H-%M-%S").to_string()
    // Utc::now().format("%Y-%m-%d-%H-%M-%S").to_string()
}
