extern crate notify;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{collections::LinkedList, env};

use colored::Colorize;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
fn main() {
    let mut command = Command::new("bash");
    let mut watch_location = env::current_dir().unwrap();

    if let Ok(cmd) = env::var("RUN_SCRIPT") {
        let args: LinkedList<_> = cmd.split_whitespace().collect();
        command.args(args);
    } else {
        command.arg("./run.sh");
    }
    if let Ok(loc) = env::var("WATCH_LOCATION") {
        watch_location = PathBuf::from(loc);
    }

    if let Ok(loc) = env::var("RUN_LOCATION") {
        command.current_dir(loc);
    }

    let (sender, receiver) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(sender, Duration::from_millis(10)).unwrap();
    dbg!(&command);

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(watch_location, RecursiveMode::Recursive)
        .unwrap();

    loop {
        match receiver.recv() {
            Ok(DebouncedEvent::Create(event))
            | Ok(DebouncedEvent::Remove(event))
            | Ok(DebouncedEvent::Write(event)) => {
                dbg!(event);
                if let Err(_) = command
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
                {
                    println!("{}", String::from("failed to run command").red().bold());
                }
            }
            Err(e) => println!("watch error: {:?}", e),
            _ => {}
        }
    }
}
