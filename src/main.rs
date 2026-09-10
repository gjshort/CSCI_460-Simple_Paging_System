/*
* Main File for the Paging System Simulator
*/

// Memory Controller Visibility
mod memory_controller; 
use crate::memory_controller::Job;
use crate::memory_controller::FrameTableEntry;

use std::collections::VecDeque;

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


}
