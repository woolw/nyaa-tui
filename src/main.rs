use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use datamodel::{App, NyaaEntry};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::{
    env,
    io::{self},
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

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new().await;

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
        Ok(opt) => match opt {
            Some(downloads) => download_entries(downloads, download_dir),
            None => {}
        },
        Err(err) => println!("{err:?}"),
    }

    Ok(())
}

fn download_entries(downloads: Vec<NyaaEntry>, download_dir: Option<String>) {
    let mut command = Command::new("aria2c");
    let mut args_vec: Vec<String> = vec!["--seed-time=0".to_string(), "-Z".to_string()];

    match download_dir {
        Some(val) => args_vec.push(val.to_string()),
        None => {}
    }

    for download in downloads.iter() {
        if !download.download_links.magnetic.is_empty() {
            args_vec.push(format!("{}", download.download_links.magnetic));
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
