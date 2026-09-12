/*
* Main File for the Paging System Simulator
*/

use std::collections::VecDeque;
use std::env;

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
    }
    else 
    {
        println!("Invalid CLI arguments");
    }

}
