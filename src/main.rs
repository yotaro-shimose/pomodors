use anyhow::Result;
use pomodors::calendar::{self, get_calendar_id};
use pomodors::timer;
use std::path::Path;
use structopt::StructOpt;

const SECRET_PATH: &str = "/root/.pomodors/client_secret.json";

#[derive(StructOpt, Debug)]
struct Args {
    /// task name
    taskname: String,
    /// time to work with this task in minutes
    #[structopt(default_value = "30")]
    time_in_minutes: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::from_args();
    let hub = calendar::get_hub(&Path::new(SECRET_PATH)).await?;
    let calendar_id = get_calendar_id(&hub).await;
    let event = timer::start(&args.taskname, args.time_in_minutes)?;
    calendar::insert_event(&hub, &calendar_id, event).await?;
    Ok(())
}
