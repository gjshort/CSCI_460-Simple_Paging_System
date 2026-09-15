/*
 * Memory Controller 
 */

use std::collections::VecDeque;

pub const ADMIT_NEW_JOB: u8 = 0;
pub const REMOVE_JOB: u8 = 1;
pub const SUSPEND_JOB: u8 = 2;
pub const RESUME_JOB: u8 = 3;
pub const TRANSLATE_ADDR: u8 = 4;
pub const PRINT: u8 = 5;
pub const EXIT: u8 = 6;
pub const ERROR: u8 = 255;

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

pub fn mem_control(action: u8, args: Vec<&str>, job_list: &Vec<Job>, frame_table: &Vec<FrameTableEntry>, job_fifo: &VecDeque<u32>,
                   mem_size: u32, frame_size: u32, num_frames: u32, free_frames: u32) -> u32 {
    
    match action {
        ADMIT_NEW_JOB=> 
        {
            // args[0] = Job ID, args[1] = Job Size (bytes)
            admit_job(job_list, frame_table, job_fifo, args[0].parse::<u32>().unwrap(), 
                args[1].parse::<u32>().unwrap(), mem_size, frame_size, num_frames, free_frames);
        },
        REMOVE_JOB=> 
        {
            // args[0] = Job ID
            remove_job(job_list, frame_table, job_fifo, args[0].parse::<u32>().unwrap());
        },
        SUSPEND_JOB=> {},
        RESUME_JOB=> {},
        TRANSLATE_ADDR=> {},
        PRINT=> {},
        EXIT=> {},
        ERROR=> {},
        _=> {},
    }

    return 0;
}

pub fn admit_job(job_list: &Vec<Job>, frame_table: &Vec<FrameTableEntry>, job_fifo: &VecDeque<u32>,
                 num: u32, size: u32, mem_size: u32, frame_size: u32,
                 num_frames: u32, free_frames: u32) -> u32 {

    println!("Admit Job | ID: {} | Size: {} Bytes", num, size);
    
    return 0;
}

pub fn remove_job(job_list: &Vec<Job>, frame_table: &Vec<FrameTableEntry>, job_fifo: &VecDeque<u32>, num: u32) -> u32 {

    println!("Remove Job | ID: {}", num);
    return 0;
}

pub fn suspend_job(job_list: &Vec<Job>, frame_table: &Vec<FrameTableEntry>, job_fifo: &VecDeque<u32>, num: u32) -> u32 {

    return 0;
}

pub fn resume_job(job_list: &Vec<Job>, frame_table: &Vec<FrameTableEntry>, job_fifo: &VecDeque<u32>, num: u32) -> u32 {

    return 0;
}

pub fn translate_addr(job_list: &Vec<Job>, job_num: u32, addr: u32) -> (u32, u32) {

    return (0,0);
}

pub fn print_system(job_list: &Vec<Job>, frame_table: &Vec<FrameTableEntry>, job_fifo: &VecDeque<u32>) {

}

pub fn exit() {

}