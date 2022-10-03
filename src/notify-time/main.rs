use chrono::Local;
use notify_rust::error::Error;
use notify_rust::Notification;

fn main() -> Result<(), Error> {
    let now = Local::now().format("%R");
    let curr_time_long = format!("The current time is {}", now);
    let curr_time_short = format!("{}", now);

    Notification::new()
        .appname("Date and Time")
        .summary(&curr_time_short)
        .body(&curr_time_long)
        .icon("preferences-system-time")
        .show()?;

    Ok(())
}
