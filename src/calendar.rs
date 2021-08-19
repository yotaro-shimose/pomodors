extern crate google_calendar3 as calendar3;
extern crate yup_oauth2 as oauth2;
use anyhow::{anyhow, Result};
use calendar3::api::{CalendarListEntry, Event};
use calendar3::CalendarHub;
use hyper;
use hyper_rustls;
use oauth2::InstalledFlowAuthenticator;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;
use whiteread::parse_line;
const CONF_PATH: &str = "/root/.pomodors/config";

pub async fn get_hub(path: &Path) -> Result<calendar3::CalendarHub> {
    let secret = oauth2::read_application_secret(path).await?;
    let auth = InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk("/root/.pomodors/tokencache.json")
    .build()
    .await?;

    let hub = CalendarHub::new(
        hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()),
        auth,
    );
    Ok(hub)
}

async fn get_calendars(hub: &calendar3::CalendarHub) -> Result<Vec<CalendarListEntry>> {
    let items = hub
        .calendar_list()
        .list()
        .show_hidden(false)
        .show_deleted(true)
        .max_results(10)
        .doit()
        .await?
        .1
        .items;
    match items {
        Some(items) => Ok(items),
        None => Err(anyhow!("No calendar is available")),
    }
}

trait InfoString {
    fn as_info_string(&self) -> String;
}

impl InfoString for CalendarListEntry {
    fn as_info_string(&self) -> String {
        match &self.summary {
            Some(summary) => summary.clone(),
            None => "Untitled Calendar".to_string(),
        }
    }
}

/// Returns calendar id specified by user
pub fn ask_calendar(calendar_list: &[CalendarListEntry]) -> String {
    loop {
        println!("Choose calendar number from the following options");
        println!(
            "{}",
            calendar_list
                .into_iter()
                .enumerate()
                .map(|(idx, entry)| format!("{}, {}", idx, entry.as_info_string()))
                .collect::<Vec<String>>()
                .join("\n")
        );

        let ret = parse_line::<usize>();
        match ret {
            Ok(idx) => break calendar_list[idx].id.as_ref().unwrap().clone(),
            Err(_) => {
                println!("Parse Error: enter non-negative integer\n")
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    calendar_id: Option<String>,
}

fn read_config() -> Result<Config> {
    let path = Path::new(CONF_PATH);
    let parent = path.parent().unwrap();
    if !parent.exists() {
        create_dir_all(parent)?;
    }
    let mut file = if path.exists() {
        File::open(path)?
    } else {
        File::create(path)?
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let ans = serde_json::from_str::<Config>(contents.as_str())?;
    Ok(ans)
}

fn write_config(config: &Config) -> Result<()> {
    let path = Path::new(CONF_PATH);
    let mut file = File::create(path)?;
    Ok(file.write_all(serde_json::to_string(config)?.as_bytes())?)
}

async fn ask_calendar_id(hub: &CalendarHub) -> String {
    let calendar_list = get_calendars(&hub).await.unwrap();
    let calendar_id = ask_calendar(&calendar_list);
    let config = Config {
        calendar_id: Some(calendar_id.clone()),
    };
    write_config(&config).unwrap();
    calendar_id
}

pub async fn get_calendar_id(hub: &CalendarHub) -> String {
    match read_config() {
        Ok(config) => match config.calendar_id {
            Some(calendar_id) => calendar_id,
            None => ask_calendar_id(&hub).await,
        },
        Err(_) => ask_calendar_id(&hub).await,
    }
}

pub async fn insert_event(hub: &CalendarHub, calendar_id: &str, event: Event) -> Result<()> {
    hub.events().insert(event, &calendar_id).doit().await?;
    Ok(())
}
