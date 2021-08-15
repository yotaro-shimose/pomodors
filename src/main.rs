use anyhow::Result;
use chrono::Duration;
use crossterm::terminal::ClearType;
use crossterm::{cursor, terminal, QueueableCommand};
use std::io::{stdout, Stdout, Write};
use std::{thread, time};
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
struct Args {
    /// task name
    taskname: String,
    /// time to work with this task in minutes
    #[structopt(default_value = "30")]
    time_in_minutes: i64,
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

fn start(name: &str, time_in_minutes: i64) -> Result<()> {
    let start = chrono::Utc::now();
    let mut stdout = stdout();
    let remain = Duration::minutes(time_in_minutes);
    let total_time = Duration::minutes(time_in_minutes);
    display_line(&mut stdout, name, remain, false)?;
    loop {
        let now = chrono::Utc::now();
        let duration = now - start;
        let remain = total_time - duration;
        display_line(&mut stdout, name, remain, true)?;
        thread::sleep(time::Duration::from_millis(200));
        if duration >= total_time {
            println!("");
            break Ok(());
        }
    }
}

fn summary_print(name: &str, times_in_minutes: i64) {
    println!(
        "You've finished {} minutes of {} task!",
        times_in_minutes, name
    );
}

fn main() {
    let args = Args::from_args();
    let ret = start(&args.taskname, args.time_in_minutes);
    match ret {
        Ok(()) => summary_print(&args.taskname, args.time_in_minutes),
        Err(e) => println!("{}", e),
    }
}
