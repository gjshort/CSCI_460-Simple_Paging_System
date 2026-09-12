/*
* Main File for the Paging System Simulator
*/

use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Memory Controller Visibility
mod memory_controller; 
use crate::memory_controller::Job;
use crate::memory_controller::FrameTableEntry;

const ADMIT_NEW_JOB: u8 = 0;
const REMOVE_JOB: u8 = 1;
const SUSPEND_JOB: u8 = 2;
const RESUME_JOB: u8 = 3;
const TRANSLATE_ADDR: u8 = 4;
const PRINT: u8 = 5;
const EXIT: u8 = 6;
const ERROR: u8 = 255;

fn main() {
    
    // Paging System Variables
    let mut memory_size: u32 = 0;
    let mut frame_size:  u32 = 0;
    let mut num_frames:  u32 = 0;
    let mut free_frames: u32 = 0;

    // Paging System Data Structures
    let mut job_list: Vec<Job> = Vec::new();
    let mut frame_table: Vec<FrameTableEntry> = Vec::new();
    let mut resident_jobs: VecDeque<u32> = VecDeque::new();

    // Bring in CLI arguments
    let args: Vec<String> = env::args().collect();
    if args.len() == 4 
    {
        memory_size = args[1].parse::<u32>().unwrap();
        frame_size = args[2].parse::<u32>().unwrap();

        num_frames = memory_size / frame_size;
        free_frames = num_frames;
    
        println!("Memory Size: {}", memory_size);
        println!("Frame Size: {}", frame_size);
        println!("Num Frames: {}", num_frames);
        println!("Free Frames: {}", free_frames);

        match input_file_handle(&args[3]) {
            Ok(_) => {},
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    }
    else 
    {
        println!("Invalid CLI arguments");
        return;
    }

    let mut running: bool = true;
    let mut cmd_input: String = String::new();

    // Main Loop
    while running {
       
    }

}


fn input_file_handle(in_file: &String) -> Result<(), io::Error> {

    let file = File::open(in_file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}

fn command_parser(command: String, arg1: i32) -> u8 {

    if command == "J" {
        if arg1 > 0 {
            return ADMIT_NEW_JOB;
        }
        else if arg1 == 0 {
            return REMOVE_JOB;
        }
        else if arg1 == -1 {
            return SUSPEND_JOB;
        }
        else if arg1 == -2 {
            return RESUME_JOB;
        }
        else {
            return ERROR;
        }
    }
    else if command == "translate" {
        return TRANSLATE_ADDR;
    }
    else if command == "print" {
        return PRINT;
    }
    else if command == "exit" {
        return EXIT;
    }
    else {
        return ERROR;
    }

}
