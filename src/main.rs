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

        match input_file_handle(&args[3], &job_list, &frame_table, &resident_jobs, memory_size, frame_size, num_frames, free_frames) {
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


fn input_file_handle(in_file: &String, job_list: &Vec<Job>, frame_table: &Vec<FrameTableEntry>, job_fifo: &VecDeque<u32>,
                     mem_size: u32, frame_size: u32, num_frames: u32, free_frames: u32) -> Result<(), io::Error> {

    let file = File::open(in_file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut action: u8 = 0;
        
        // Break apart a line by whitespace
        let line = line?;
        let symbols: Vec<&str> = line.split_whitespace().collect();

        // Parsing "print" and "exit"   
        if symbols.len() == 1 {
            action = command_parser(symbols[0].to_string(), 0);
        }
        // Parsing all other commands
        else if symbols.len() > 1 {
            let arg1 = symbols[1].parse::<i32>().unwrap();
            action = command_parser(symbols[0].to_string(), arg1);
        }

        memory_controller::mem_control(action, symbols, job_list, frame_table, job_fifo, mem_size, frame_size, num_frames, free_frames);
    }

    Ok(())
}

fn command_parser(command: String, arg1: i32) -> u8 {

    if command.parse::<u32>().is_ok() {
        if arg1 > 0 {
            return memory_controller::ADMIT_NEW_JOB;
        }
        else if arg1 == 0 {
            return memory_controller::REMOVE_JOB;
        }
        else if arg1 == -1 {
            return memory_controller::SUSPEND_JOB;
        }
        else if arg1 == -2 {
            return memory_controller::RESUME_JOB;
        }
        else {
            return memory_controller::ERROR;
        }
    }
    else if command == "translate" {
        return memory_controller::TRANSLATE_ADDR;
    }
    else if command == "print" {
        return memory_controller::PRINT;
    }
    else if command == "exit" {
        return memory_controller::EXIT;
    }
    else {
        return memory_controller::ERROR;
    }

}
