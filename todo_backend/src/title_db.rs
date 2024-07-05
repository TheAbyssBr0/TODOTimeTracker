use chrono::Datelike;

pub fn get_title() -> String {
    let now = chrono::Local::now();
    let last_monday = now - chrono::Duration::days(
        now
        .weekday()
        .num_days_from_monday() as i64
    );
    last_monday.format("Week of %d %b '%y").to_string()
}