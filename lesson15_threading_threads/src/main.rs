#![warn(clippy::all, clippy::pedantic)]

use pulldown_cmark::{Options, Parser};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

enum ProcessorMessage {
    Success(String),
    Error(String),
}

fn all_md_files() -> std::io::Result<Vec<PathBuf>> {
    let mut md_files_path = env::current_dir()?;

    //open /md
    md_files_path.push("md");

    //read files
    let entries = fs::read_dir(md_files_path)?;

    let mut paths = Vec::new();

    //try to get paths to files
    for entry in entries {
        paths.push(entry?.path());
    }

    Ok(paths)
}

fn convert_parallel(
    files: Vec<PathBuf>,
    tx: mpsc::Sender<ProcessorMessage>,
) -> Vec<thread::JoinHandle<()>> {
    files
        .into_iter() // iterator
        .enumerate() // we want to get index of files
        .map(|(index, file)| {
            // process per file
            let new_tx = tx.clone();

            thread::spawn(move || {
                let contents = match fs::read_to_string(&file) {
                    Ok(contents) => contents,
                    Err(e) => {
                        new_tx
                            .send(ProcessorMessage::Error(format!(
                                "Failed to read file {file:?}: {e}"
                            )))
                            .unwrap();
                        return;
                    }
                };

                let parser = Parser::new_ext(&contents, Options::empty());
                let mut html_output = String::new();
                // create html
                pulldown_cmark::html::push_html(&mut html_output, parser);

                //default filename
                let def = format!("file{index}");
                let current_file_name = file
                    .file_name() // file name
                    .unwrap_or(OsStr::new(&def)) // file name or os String with default value
                    .to_str() // create &str
                    .unwrap();

                // current dir
                let mut html_file = env::current_dir().expect("Can't read current dir!");
                // dir/html
                html_file.push("html");
                // dir/html/filename
                html_file.push(current_file_name);
                // dir/html/filename.html
                html_file.set_extension("html");

                // create file
                let mut output = fs::File::create(html_file).expect("Can't create HTML file");

                // write all files
                output
                    .write_all(html_output.as_bytes())
                    .expect("Can't write to HTML file!");

                // send status ok to rc
                new_tx
                    .send(ProcessorMessage::Success(format!(
                        "File {file:?} is processed!"
                    )))
                    .expect("Failed to send message");
            })
        })
        .collect() // -> vec
}

fn main() {
    match all_md_files() {
        // if files
        Ok(files) => {
            // create sender and reciever
            let (tx, rx) = mpsc::channel();

            // process per file
            let processors = convert_parallel(files, tx);

            for (index, handle) in processors.into_iter().enumerate() {
                // wait thread
                match handle.join() {
                    // thread successfully ended
                    Ok(()) => println!("Processor {index} is finished!"),
                    // thread downed
                    Err(e) => {
                        if let Some(s) = e.downcast_ref::<String>() {
                            println!("Thread {index} panicked: {s:?}");
                        } else {
                            println!("Unknown error when processing a thread {index}");
                        }
                    }
                }
            }

            for received in rx {
                match received {
                    ProcessorMessage::Success(msg) => println!("Message incoming: {msg}"),
                    ProcessorMessage::Error(e) => println!("Error incoming: {e}"),
                }
            }
        }
        //cant get files
        Err(error) => println!("There was an error when collecting MD files: {error:?}"),
    }
}
