use anyhow::Result;
use calendar_3::api::{Event, EventDateTime};
use chrono::{Duration, Utc};
use crossterm::terminal::ClearType;
use crossterm::{cursor, terminal, QueueableCommand};
use google_calendar3 as calendar_3;
use std::io::{stdout, Stdout, Write};
use std::{thread, time};

trait FromDatetime {
    fn from_datetime(datetime: &chrono::DateTime<Utc>) -> Self;
}

impl FromDatetime for EventDateTime {
    fn from_datetime(datetime: &chrono::DateTime<Utc>) -> Self {
        let mut ans = EventDateTime::default();
        ans.date_time = Some(datetime.to_rfc3339());
        ans
    }
}

fn create_event(name: &str, start: &chrono::DateTime<Utc>, end: &chrono::DateTime<Utc>) -> Event {
    let mut ans = Event::default();
    ans.summary = Some(name.to_string());
    ans.start = Some(EventDateTime::from_datetime(start));
    ans.end = Some(EventDateTime::from_datetime(end));
    ans
}

fn display_line(stdout: &mut Stdout, name: &str, remain: Duration, clear: bool) -> Result<()> {
    let minutes = remain.num_seconds() / 60;
    let seconds = remain.num_seconds() - minutes * 60;
    if clear {
        stdout.queue(terminal::Clear(ClearType::CurrentLine))?;
    }
    stdout.queue(cursor::SavePosition)?;
    stdout.write(
        format!(
            "TaskName: {}  Remaining: {:0>2}m {:0>2}s",
            name, minutes, seconds
        )
        .as_bytes(),
    )?;
    stdout.queue(cursor::RestorePosition)?;
    stdout.flush()?;
    Ok(())
}

pub fn start(name: &str, time_in_minutes: u64) -> Result<Event> {
    let start = chrono::Utc::now();
    let mut stdout = stdout();
    let remain = Duration::minutes(time_in_minutes as i64);
    let total_time = Duration::minutes(time_in_minutes as i64);
    display_line(&mut stdout, name, remain, false)?;
    loop {
        let now = chrono::Utc::now();
        let duration = now - start;
        let remain = total_time - duration;
        display_line(&mut stdout, name, remain, true)?;
        thread::sleep(time::Duration::from_millis(200));
        if duration >= total_time {
            println!("");
            let end = chrono::Utc::now();
            let event = create_event(name, &start, &end);
            break Ok(event);
        }
    }
}

pub fn summary_print(name: &str, times_in_minutes: i64) {
    println!(
        "You've finished {} minutes of {} task!",
        times_in_minutes, name
    );
}
