use chrono::Local;
use notify_rust::error::Error;
use notify_rust::Notification;

fn main() -> Result<(), Error> {
    let now = Local::now();
    let curr_date_long = format!("The current date is {}", now.format("%A %-e %B %Y"));
    let curr_date_short = format!("{}", now.format("%d/%m/%y"));

    Notification::new()
        .appname("Date and Time")
        .summary(&curr_date_short)
        .body(&curr_date_long)
        .icon("calendar")
        .show()?;

    Ok(())
}
