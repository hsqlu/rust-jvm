// (Full example with detailed comments in examples/01b_quick_example.rs)
//
// This example demonstrates clap's full 'builder pattern' style of creating arguments which is
// more verbose, but allows easier editing, and at times more advanced options, or the possibility
// to generate arguments dynamically.
// extern crate clap;
// use clap::{App, Arg, SubCommand};
use std::env;

// const args: Vec<i32> = vec![];
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut vm_args: Vec<String> = Vec::new();
    let mut app_args: Vec<String> = Vec::new();
    let mut app_name = String::new();
    let mut i = 1 as usize;
    let mut app_args_started = false;
    println!("Args length: {}", args.len());
    while i < args.len() {
        let arg = &args[i];
        println!("index {}:  {}", i, arg);
        if arg.starts_with("-X") && !app_args_started {
            vm_args.push(arg.clone());
        } else if !app_args_started {
            app_name = String::from(arg);
            app_args_started = true;
        } else {
            app_args.push(arg.clone());
        }
        i = i + 1;
    }
    println!("VM args: {:?}", vm_args);
    println!("App name: {}", app_name);

    if app_name.is_empty() {
        panic!("Usage: ")
    }

    // let matches = App::new("Rust-JVM")
    //     .version("0.1")
    //     .author("Qiannan L. <qiannan.lyu@gmail.com>")
    //     .about("JVM implemented in Rust")
    //     .arg(
    //         Arg::with_name("v")
    //             .short("v")
    //             .multiple(true)
    //             .help("Sets the level of verbosity"),
    //     )
    //     .subcommand(
    //         SubCommand::with_name("test")
    //             .about("controls testing features")
    //             .version("1.3")
    //             .author("Someone E. <someone_else@other.com>")
    //             .arg(
    //                 Arg::with_name("debug")
    //                     .short("d")
    //                     .help("print debug information verbosely"),
    //             ),
    //     )
    //     .get_matches();

    // // Gets a value for config if supplied by user, or defaults to "default.conf"
    // //    let config = matches.value_of("config").unwrap_or("default.conf");
    // //    println!("Value for config: {}", config);

    // // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // // required we could have used an 'if let' to conditionally get the value)
    // //    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // // Vary the output based on how many times the user used the "verbose" flag
    // // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    // match matches.occurrences_of("v") {
    //     0 => println!("No verbose info"),
    //     1 => println!("Some verbose info"),
    //     2 => println!("Tons of verbose info"),
    //     3 | _ => println!("Don't be crazy"),
    // }

    // // You can handle information about subcommands by requesting their matches by name
    // // (as below), requesting just the name used, or both at the same time
    // if let Some(matches) = matches.subcommand_matches("test") {
    //     if matches.is_present("debug") {
    //         println!("Printing debug info...");
    //     } else {
    //         println!("Printing normally...");
    //     }
    // }

    // more program logic goes here...
}
