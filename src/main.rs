use anyhow::Result;
use pomodors::calendar::{self, get_calendar_id};
use pomodors::path;
use pomodors::timer;
use structopt::StructOpt;
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
    let secret_path = path::get_secret_path();
    let hub = calendar::get_hub(&secret_path).await?;
    let calendar_id = get_calendar_id(&hub).await;
    let event = timer::start(&args.taskname, args.time_in_minutes)?;
    calendar::insert_event(&hub, &calendar_id, event).await?;
    Ok(())
}
