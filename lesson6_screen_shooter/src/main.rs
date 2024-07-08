// https://hackage.haskell.org/package/evdev#permissions and reboot for permissions for input devices
// sudo apt-get install libxcb1 libxrandr2 libdbus-1-3
#![warn(clippy::all, clippy::pedantic)]

use chrono::{DateTime, Utc};
use rdev::{grab, Event, EventType, Key};
use screenshots::Screen;
use std::{env, fs};

const TARGET_DIR: &str = "screens";
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let screen_dir = args.get(1).unwrap_or(&TARGET_DIR.to_string()).to_string();

    let mut path = env::current_dir()?;
    path.push(&screen_dir);

    fs::create_dir_all(path)?;

    if let Err(error) = grab(move |e| callback(e, &screen_dir)) {
        println!("Error: {error:?}");
    }
    Ok(())
}

fn callback(event: Event, screen_dir: &String) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(Key::PageUp) => {
            make_screenshot(screen_dir);
            None
        }
        _ => Some(event),
    }
}

fn make_screenshot(screen_dir: &String) {
    let screens = Screen::all().unwrap();
    for screen in screens {
        let image = screen.capture().unwrap();
        let now: DateTime<Utc> = Utc::now();
        image
            .save(format!(
                "{}/{}.png",
                screen_dir,
                now.format("%d-&m-&Y_%H_%M_%S_%f")
            ))
            .unwrap();
        println!("Created {now:?}")
    }
}
