/*
 * Memory Controller 
 */

use std::collections::VecDeque;

pub struct Job {
    number: u32,
    status: bool,
    size: u32,
    int_frag: u32,
    page_table: Vec<(u32, u32)>
}

pub struct FrameTableEntry {
    frame_number: u32,
    status: bool,
    job_number: u32,
    page_number: u32
}

pub fn admit_job(job_list: Vec<Job>, frame_table: Vec<FrameTableEntry>, job_fifo: VecDeque<u32>,
                 num: u32, size: u32, mem_size: u32, frame_size: u32,
                 num_frames: u32, free_frames: u32) -> u32 {

    return 0;
}

pub fn remove_job(job_list: Vec<Job>, frame_table: Vec<FrameTableEntry>, job_fifo: VecDeque<u32>, num: u32) -> u32 {

    return 0;
}

pub fn suspend_job(job_list: Vec<Job>, frame_table: Vec<FrameTableEntry>, job_fifo: VecDeque<u32>, num: u32) -> u32 {

    return 0;
}

pub fn resume_job(job_list: Vec<Job>, frame_table: Vec<FrameTableEntry>, job_fifo: VecDeque<u32>, num: u32) -> u32 {

    return 0;
}

pub fn translate_addr(job_list: Vec<Job>, job_num: u32, addr: u32) -> (u32, u32) {

    return (0,0);
}

pub fn print_system(job_list: Vec<Job>, frame_table: Vec<FrameTableEntry>, job_fifo: VecDeque<u32>) {

}

pub fn exit() {

}