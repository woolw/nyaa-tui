use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use datamodel::{App, NyaaEntry};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::{
    env,
    fs::{self, File},
    io::{self, BufReader},
    path::PathBuf,
    process::Command,
};

pub mod app;
pub mod datamodel;
pub mod scraper;
pub mod tui;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // check if aria2 is installed
    if !Command::new("which")
        .arg("aria2c")
        .output()
        .unwrap()
        .stderr
        .is_empty()
    {
        println!("aria2 not found");
        return Ok(());
    }

    // check if any arguments were passed
    let args: Vec<String> = env::args().collect();
    let mut download_dir: Option<String> = None;
    for arg in args.iter() {
        match arg.as_str() {
            "--info" => {
                print!(
                    "nyaa-tui version: {} \nauthor: {}",
                    env!("CARGO_PKG_VERSION"),
                    env!("CARGO_PKG_AUTHORS")
                );
                return Ok(());
            }
            arg if arg.starts_with("--dir=") => download_dir = Some(arg.to_string()),
            _ => {}
        }
    }

    // create PathBuf for saving / loading
    let data_dir = match dirs::data_dir() {
        Some(mut val) => {
            val.push("nyaa-tui");
            val.push("download_list.json");
            val
        }
        None => {
            panic!("no data_dir")
        }
    };

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // load save and create app with it
    let app = match load_list(&data_dir) {
        Some(saves) => App::new(saves).await,
        None => App::new(vec![]).await,
    };

    // start tui
    let res = app.run(&mut terminal).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // start downloading if there are any in the list
    match res {
        Ok(opt) => match opt.exit_condition {
            datamodel::ExitCondition::Quit => {}
            datamodel::ExitCondition::SaveList => save_list(opt.download_entries.items, data_dir),
            datamodel::ExitCondition::Download => {
                download_entries(opt.download_entries.items, download_dir)
            }
        },
        Err(err) => println!("{err:?}"),
    }

    Ok(())
}

fn download_entries(downloads: Vec<NyaaEntry>, download_dir: Option<String>) {
    let mut command = Command::new("aria2c");
    let mut args_vec: Vec<String> = vec!["--seed-time=0".to_string(), "-Z".to_string()];

    if let Some(val) = download_dir {
        args_vec.push(val.to_string())
    }

    for download in downloads.iter() {
        if !download.download_links.magnetic.is_empty() {
            args_vec.push(download.download_links.magnetic.to_string());
        } else if !download.download_links.torrent.is_empty() {
            args_vec.push(format!(
                "https://nyaa.si{}",
                download.download_links.torrent
            ));
        }
    }

    if args_vec.len() > 2 {
        let mut process = command.args(args_vec).spawn().expect("process failed");

        process.wait().unwrap();
    }
}

fn load_list(data_dir: &PathBuf) -> Option<Vec<NyaaEntry>> {
    match File::open(data_dir) {
        Ok(file) => Some(serde_json::from_reader(BufReader::new(file)).unwrap()),
        Err(_) => None,
    }
}

fn save_list(downloads: Vec<NyaaEntry>, data_dir: PathBuf) {
    if fs::create_dir_all(data_dir.parent().unwrap()).is_ok() {
        let list_json = serde_json::to_string_pretty(&downloads).unwrap();
        fs::write(data_dir, list_json).unwrap();
    }
}
