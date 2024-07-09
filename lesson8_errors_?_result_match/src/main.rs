use std::{
    env::current_dir,
    fs::{self, create_dir_all},
    io::{Error as IOError, ErrorKind},
    path::PathBuf,
};

// if input output -> std::io::Result<()>

// any dynamic error
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let current_path = current_dir();

    // let mut target = match current_path {
    //     Ok(path) => path,
    //     Err(e) => panic!("Can't get current path: {e:?}"),
    // };

    // error propogation:
    let mut target = current_dir()?;

    target.push(" ");

    // match error handling
    match fs::create_dir_all(&target) {
        Ok(()) => println!("Created {target:?}"),
        Err(e) => match e.kind() {
            ErrorKind::InvalidData => {
                panic!("Couldn't create path: {e:?}")
            }
            ErrorKind::AlreadyExists => {
                panic!("Couldn't create path: {e:?}")
            }
            _ => {
                panic!("Couldn't create path: {e:?}")
            }
        },
    }

    // unwrap error handling -> ok and panic on error
    create_dir_all(&target).unwrap();

    // unwrap_or_else error handling -> ok and callback on error
    create_dir_all(&target).unwrap_or_else(|e| panic!("Couldn't create path: {e:?}"));

    // expect error handling -> ok and panic with message on error
    create_dir_all(&target).expect("Couldn't create path");

    // if all ok -> ok, or panic on error
    Ok(())
}

// fn create_dir_in(target: &PathBuf) -> Result<String, IOError> {
//     match create_dir_all(target) {
//         Ok(_) => Ok(format!("{}", target.to_string_lossy())),
//         Err(e) => Err(e),
//     }
// }

fn create_dir_in(target: &PathBuf) -> Result<(), IOError> {
    create_dir_all(target)?;
    Ok(())
}

fn get_cur_path() -> Result<PathBuf, IOError> {
    // if ok -> unwrap and assign
    let cur_dir = current_dir()?;
    Ok(cur_dir)
}
