/*
* Main File for the Paging System Simulator
*
* Used Rust Doc string for auto generated function docs
* 
* WARNING: MINIMAL TO NO ERROR CHECKING ON BAD CLI ARGUMENTS.
* RUNTIME CRASHES ARE POSSIBLE.
* Example: user enters memory_size < frame_size
* 
*/

use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

// Memory Controller Visibility
mod memory_controller; 
use crate::memory_controller::Job;
use crate::memory_controller::FrameTableEntry;

fn main() -> Result<(), io::Error> {
    
    let mut running: bool = true;

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

        // ------- Set up Memory System -------
        // 0-based internal indexing. 1-based frame and page number outputs to terminal

        memory_size = args[1].parse::<u32>().unwrap();
        frame_size = args[2].parse::<u32>().unwrap();

        num_frames = memory_size / frame_size;
        free_frames = num_frames;
    
        println!("Memory Size: {}", memory_size);
        println!("Frame Size: {}", frame_size);
        println!("Num Frames: {}", num_frames);
        println!("Free Frames: {}", free_frames);

        // Create frame table
        for _i in 1..=num_frames {
            let new_ft_entry = FrameTableEntry::new(memory_controller::FRAME_FREE, 0, 0);
            frame_table.push(new_ft_entry);
        }

        match input_file_handle(&args[3], &mut job_list, &mut frame_table, &mut resident_jobs, memory_size, frame_size, num_frames, &mut free_frames) {
            Ok(memory_controller::OKAY) => {},
            Ok(memory_controller::EXIT) => { println!("CLOSING PROGRAM..."); running = false; },
            Err(e) => eprintln!("Error reading file: {}", e),
            _=> {},
        }
    }
    else 
    {
        println!("Invalid CLI arguments");
        running = false;
    }

    let mut cmd_input: String = String::new();
    let mut symbols: Vec<&str>;
    // Main Loop
    while running {

        io::stdout().flush()?;
        io::stdin().read_line(&mut cmd_input)?;

        symbols = cmd_input.split_whitespace().collect();
        let mut action: u32 = 0;
        let mut status: u32 = 0;

        // Parsing "print" and "exit"   
        if symbols.len() == 1 {
            action = command_parser(symbols[0].to_string(), 0);
        }
        // All other commands
        else if symbols.len() > 1 {
            let arg1 = symbols[1].parse::<i32>().unwrap();
            action = command_parser(symbols[0].to_string(), arg1);
        }

        // Call memory controller on the current command
        status = memory_controller::mem_control(action, &symbols, &mut job_list, &mut frame_table, &mut resident_jobs,
                                      memory_size, frame_size, num_frames, &mut free_frames);

        // Return "EXIT" up to caller through the Ok(status)
        if status == memory_controller::EXIT {
            running = false;
        }

        symbols.clear();    // symbols directly references cmd_input, so it must be cleared first
        cmd_input.clear();  // before Rust will allow cmd_input to be cleared.

    }

    Ok(())

}


/// Parses input file from CLI arguments. 
/// 
/// # Arguments
/// 
/// - `in_file` (`&String`) - Describe this parameter.
/// - `job_list` (`&mut Vec<Job>`) - List of all jobs in the system
/// - `frame_table` (`&mut Vec<FrameTableEntry>`) - Maps taken frames to page and job numbers
/// - `job_fifo` (`&mut VecDeque<u32>`) - FIFO of job numbers admitted into main memory
/// - `mem_size` (`u32`) - Size of the main memory in bytes
/// - `frame_size` (`u32`) - Size of each page/frame in bytes
/// - `num_frames` (`u32`) - Number of frames in main memory
/// - `free_frames` (`&mut u32`) - Number of free frames in main memory
/// 
/// # Returns
/// 
/// - `Result<(), io::Error>`
/// 
/// # Errors
/// 
/// Possible errors from Buffered Reader line fetching.
/// 
fn input_file_handle(in_file: &String, job_list: &mut Vec<Job>, frame_table: &mut Vec<FrameTableEntry>, job_fifo: &mut VecDeque<u32>,
                     mem_size: u32, frame_size: u32, num_frames: u32, free_frames: &mut u32) -> Result<u32, io::Error> {

    let file = File::open(in_file)?;
    let reader = BufReader::new(file);

    let mut status: u32 = memory_controller::OKAY;

    for line in reader.lines() {
        let mut action: u32 = 0;
        
        // Break apart a line by whitespace
        let line = line?;
        let symbols: Vec<&str> = line.split_whitespace().collect();
    
        // Parsing "print" and "exit"   
        if symbols.len() == 1 {
            action = command_parser(symbols[0].to_string(), 0);
        }
        // All other commands
        else if symbols.len() > 1 {
            // Optional header
            if symbols[0] == "Job_ID" && symbols[1] == "Size" {
                action = memory_controller::HEADER;
            }
            // All other 2+ arg commands
            else {
                let arg1 = symbols[1].parse::<i32>().unwrap();
                action = command_parser(symbols[0].to_string(), arg1);
            }
        }

        // Call memory controller on the current command
        status = memory_controller::mem_control(action, &symbols, job_list, frame_table, job_fifo, mem_size, frame_size, num_frames, free_frames);

        // Return "EXIT" up to caller through the Ok(status)
        if status == memory_controller::EXIT {
            break;
        }

    }

    Ok(status)
}


/// Maps the received command by the program to a memory controller action
/// 
/// 
/// # Arguments
/// 
/// - `command` (`String`) - Received command from CLI or input file
/// - `arg1` (`i32`) - First argument for commands requiring one or mpre.
///                    Ignored for commands not requiring arguments.
/// 
/// # Returns
/// 
/// - `u32` - Memory controller action
/// 
fn command_parser(command: String, arg1: i32) -> u32 {

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
