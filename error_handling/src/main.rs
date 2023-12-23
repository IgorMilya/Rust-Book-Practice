use std::fs::{self, File};
use std::io::{Error, ErrorKind};

use std::io::{self, Read};

fn main() {
    // let greeting_file_result = File::open("hello.txt").unwrap();
    // let greeting_file_result = File::open("hello.txt").expect("Couldn't open");

    // Same with .unwrap() method
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };


    //First way
    //let greeting_file = match greeting_file_result {
    //         Ok(file) => file,
    //         Err(err) => match err.kind() {
    //             ErrorKind::NotFound => match File::create("hello.txt") {
    //                 Ok(fc) => fc,
    //                 Err(err) => panic!("Couldn't create greeting file {:?}", err)
    //             }
    //             other_error => {
    //                 panic!("Problem opening file {:?}", other_error)
    //             }
    //         }
    //     };

    // Second way
    // let greeting_file = greeting_file_result.unwrap_or_else(|err| match err.kind() {
    //     ErrorKind::NotFound => match File::create("hello.txt") {
    //         Ok(fc) => fc,
    //         Err(err) => panic!("Couldn't create greeting file {:?}", err)
    //     }
    //     other_error => {
    //         panic!("Problem opening file {:?}", other_error)
    //     }
    // });

    // // Third way
    // let greeting_file = greeting_file_result.unwrap_or_else(|err| match err.kind() {
    //     ErrorKind::NotFound => File::create("hello.txt").unwrap_or_else(|err|
    //         panic!("Couldn't create greeting file {:?}", err)
    //     ),
    //     other_error => {
    //         panic!("Problem opening file {:?}", other_error)
    //     }
    // });

    // Fourth way
    // let greeting_file = greeting_file_result.unwrap_or_else(|err|
    //     if err.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|err|
    //             panic!("Couldn't create greeting file {:?}", err)
    //         )
    //     } else {
    //         panic!("Problem opening file {:?}", err)
    //     }
    // );

    ////////////////////////////////////////////////////////////////////////////////////////////////
    //Propagating Errors

    // let string = read_username_from_file();
    //
    // println!("{:?}", string);



}

//Propagating Errors
fn read_username_from_file() -> Result<String, io::Error> {
    // ? symbol like .unwrap() method, but instead of panic return Err(err)

    // First way
    // let mut username = String::new();
    // let mut username_file_result = File::open("hello.txt")?;
    //
    // username_file_result.read_to_string(&mut username)?;
    // Ok(username)


    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // Second way
    // let mut username = String::new();
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)

    // Third way
    fs::read_to_string("hello.txt")

}


// ErrorKind::NotFound => {}
//             ErrorKind::PermissionDenied => {}
//             ErrorKind::ConnectionRefused => {}
//             ErrorKind::ConnectionReset => {}
//             ErrorKind::HostUnreachable => {}
//             ErrorKind::NetworkUnreachable => {}
//             ErrorKind::ConnectionAborted => {}
//             ErrorKind::NotConnected => {}
//             ErrorKind::AddrInUse => {}
//             ErrorKind::AddrNotAvailable => {}
//             ErrorKind::NetworkDown => {}
//             ErrorKind::BrokenPipe => {}
//             ErrorKind::AlreadyExists => {}
//             ErrorKind::WouldBlock => {}
//             ErrorKind::NotADirectory => {}
//             ErrorKind::IsADirectory => {}
//             ErrorKind::DirectoryNotEmpty => {}
//             ErrorKind::ReadOnlyFilesystem => {}
//             ErrorKind::FilesystemLoop => {}
//             ErrorKind::StaleNetworkFileHandle => {}
//             ErrorKind::InvalidInput => {}
//             ErrorKind::InvalidData => {}
//             ErrorKind::TimedOut => {}
//             ErrorKind::WriteZero => {}
//             ErrorKind::StorageFull => {}
//             ErrorKind::NotSeekable => {}
//             ErrorKind::FilesystemQuotaExceeded => {}
//             ErrorKind::FileTooLarge => {}
//             ErrorKind::ResourceBusy => {}
//             ErrorKind::ExecutableFileBusy => {}
//             ErrorKind::Deadlock => {}
//             ErrorKind::CrossesDevices => {}
//             ErrorKind::TooManyLinks => {}
//             ErrorKind::InvalidFilename => {}
//             ErrorKind::ArgumentListTooLong => {}
//             ErrorKind::Interrupted => {}
//             ErrorKind::Unsupported => {}
//             ErrorKind::UnexpectedEof => {}
//             ErrorKind::OutOfMemory => {}
//             ErrorKind::Other => {}
//             ErrorKind::Uncategorized => {}