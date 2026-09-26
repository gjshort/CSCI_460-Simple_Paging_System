/*
 * Memory Controller 
 */

use std::collections::VecDeque;

pub const OKAY:           u32 = 0;

pub const ADMIT_NEW_JOB:  u32 = 10;
pub const REMOVE_JOB:     u32 = 11;
pub const SUSPEND_JOB:    u32 = 12;
pub const RESUME_JOB:     u32 = 13;
pub const TRANSLATE_ADDR: u32 = 14;
pub const PRINT:          u32 = 15;
pub const EXIT:           u32 = 16;
pub const HEADER:         u32 = 17;
pub const ERROR:          u32 = 255;

pub const DUPLICATE_JOB:  u32 = 20;
pub const MASSIVE_JOB:    u32 = 21;
pub const INVALID_JOB:    u32 = 22;

pub const RESIDENT:      bool = true;
pub const SUSPENDED:     bool = false;
pub const FRAME_TAKEN:   bool = true;
pub const FRAME_FREE:    bool = false;

pub struct Job {
    number: u32,
    status: bool,
    size: u32,
    int_frag: u32,
    page_table: Vec<u32>
}

pub struct FrameTableEntry {
    status: bool,
    job_number: u32,
    page_number: u32
}

impl FrameTableEntry {
    // Public constructor method
    pub fn new(status: bool, job_number: u32, page_number: u32)-> Self {
        // Direct instantiation is perfectly valid inside the defining module
        FrameTableEntry { status, job_number, page_number }
            
    }
}


/**
 * Public entry point into the memory controller
 * Control is accomplished through the 'action' arg
 */
pub fn mem_control(action: u32, args: Vec<&str>, job_list: &mut Vec<Job>, frame_table: &mut Vec<FrameTableEntry>, job_fifo: &mut VecDeque<u32>,
                   mem_size: u32, frame_size: u32, num_frames: u32, free_frames: &mut u32) -> u32 {
    
    let mut status: u32 = ERROR;

    match action {
        ADMIT_NEW_JOB=> 
        {
            // args[0] = Job ID, args[1] = Job Size (bytes)
            status = admit_job(job_list, frame_table, job_fifo, args[0].parse::<u32>().unwrap(), 
                        args[1].parse::<u32>().unwrap(), mem_size, frame_size, num_frames, free_frames);
        },
        REMOVE_JOB=> 
        {
            // args[0] = Job ID
            status = remove_job(job_list, frame_table, job_fifo, args[0].parse::<u32>().unwrap());
        },
        SUSPEND_JOB=> 
        {
            // args[0] = Job ID
            status = suspend_job(job_list, frame_table, job_fifo, args[0].parse::<u32>().unwrap());
        },
        RESUME_JOB=> {},
        TRANSLATE_ADDR=> {},
        PRINT=> 
        { 
            status = print_system(job_list, frame_table, job_fifo);
        },
        EXIT=> {},
        HEADER=>
        {
            status = OKAY;
        },
        ERROR=> {},
        _=> 
        {
            status = ERROR;
        },
    }

    return status;
}


/**
 * Admits a job into memory or rejects it for 
 * being too large or a duplicate
 */
fn admit_job(job_list: &mut Vec<Job>, frame_table: &mut Vec<FrameTableEntry>, job_fifo: &mut VecDeque<u32>,
            num: u32, size: u32, mem_size: u32, frame_size: u32, num_frames: u32, free_frames: &mut u32) -> u32 {

    let mut status: u32 = OKAY;

    // --------- Error Checking ---------
    // Does the Job ID already exist?
    for i in 0..job_list.len() {

        if let Some(job) = job_list.get(i as usize) {
            if job.number == num {
                println!("ERROR: Job {} already exists.", num);
                return DUPLICATE_JOB;
            }
        }
        else {
            println!("ERROR: Out of bounds Job list access!");
            return ERROR;
        }
    }

    // Is the job too big?
    if size > mem_size {
        println!("ERROR: Job {} is too large ({} bytes). Maximum job size is {} bytes.", num, size, mem_size);
        return MASSIVE_JOB;
    } 


    // --------- Job Creation ---------
    // Calculate pages
    let num_pages: u32 = size.div_ceil(frame_size);
    let internal_frag: u32 = (frame_size * num_pages) - size;

    // Set up page table (all frames left as 0 for now)
    let mut new_page_table = Vec::<u32>::new();
    for _i in 1..=num_pages {
        new_page_table.push(0);
    }

    let mut new_job = Job {
        number: num,
        status: SUSPENDED,
        size: size,
        int_frag: internal_frag,
        page_table: new_page_table,
    };

    // --------- Admit Job into System ---------
    status = job_admission(&mut new_job, job_list, frame_table, job_fifo, free_frames); 
    if status == OKAY {
        job_list.push(new_job); 
    }

    return status;
}


fn remove_job(job_list: &mut Vec<Job>, frame_table: &mut Vec<FrameTableEntry>, job_fifo: &mut VecDeque<u32>, num: u32) -> u32 {

    // Check if job # matches "num"
    // If so, update the frame table based on PT
    // remove it from the job list, job fifo
    for i in 0..job_list.len() {
        
        if let Some(job) = job_list.get_mut(i) {
            if job.number == num {

                // Update frame table for every entry in the job's page table
                for page in 0..job.page_table.len() {
                    frame_table[job.page_table[page] as usize].status = FRAME_FREE;
                }

                // Remove job from FIFO
                if let Some(index) = job_fifo.iter().position(|x| *x == num) {
                    job_fifo.remove(index);
                }
                else {
                    println!("ERROR: Out of bounds Job FIFO access!");
                    return ERROR;
                }

                // Remove Job from system
                job_list.remove(i);
                return OKAY;
            }
        }
        else {
            println!("ERROR: Out of bounds job list access!");
            return ERROR;
        }
    }

    println!("ERROR: Job {} cannot be removed because it doesn't exist.", num);
    return INVALID_JOB;
}


fn suspend_job(job_list: &mut Vec<Job>, frame_table: &mut Vec<FrameTableEntry>, job_fifo: &mut VecDeque<u32>, num: u32) -> u32 {

    for i in 0..job_list.len() {
        
        if let Some(job) = job_list.get_mut(i) {
            if job.number == num {

                // Check if already suspended
                if job.status == SUSPENDED {
                    println!("ERROR: Job {} is already suspended", num);
                    return ERROR;
                }

                // Update frame table for every entry in the job's page table
                for page in 0..job.page_table.len() {
                    frame_table[job.page_table[page] as usize].status = FRAME_FREE;
                }

                // Remove job from FIFO
                if let Some(index) = job_fifo.iter().position(|x| *x == num) {
                    job_fifo.remove(index);
                }
                else {
                    println!("ERROR: Out of bounds Job FIFO access!");
                    return ERROR;
                }

                // Mark job as suspended
                job_list[i].status = SUSPENDED;
                return OKAY;
            }
        }
        else {
            println!("ERROR: Out of bounds job list access!");
            return ERROR;
        }
    }

    println!("ERROR: Job {} cannot be suspended because it doesn't exist.", num);
    return INVALID_JOB;
}


fn resume_job(job_list: &mut Vec<Job>, frame_table: &mut Vec<FrameTableEntry>, job_fifo: &mut VecDeque<u32>, num: u32, free_frames: &mut u32) -> u32 {

    for i in 0..job_list.len() {
        
        if let Some(job) = job_list.get_mut(i) {
            if job.number == num {

                // Check if already resident
                if job.status == RESIDENT {
                    println!("ERROR: Job {} is already running", num);
                    return ERROR;
                }

                // Try to admit job
                if job_admission(job, job_list, frame_table, job_fifo, free_frames) != OKAY {
                    println!("ERROR: Could not resume job {}", num);
                }

                job_list.push(job); // TODO

                return OKAY;
            }
        }
        else {
            println!("ERROR: Out of bounds job list access!");
            return ERROR;
        }
    }

    println!("ERROR: Job {} cannot be resumed because it doesn't exist.", num);
    return INVALID_JOB;
}

fn translate_addr(job_list: &Vec<Job>, job_num: u32, addr: u32) -> (u32, u32) {

    return (0,0);
}

fn print_system(job_list: &Vec<Job>, frame_table: &Vec<FrameTableEntry>, job_fifo: &VecDeque<u32>) -> u32 {
    let mut status: u32 = OKAY;
    
    println!("===== MEMORY SNAPSHOT =====");

    // Print Frame Table
    let mut free_frames_list: Vec<u32> = Vec::<u32>::new();
    for frame_num in 0..frame_table.len() {
        if let Some(ft_entry) = frame_table.get(frame_num) {
            print!("Frame {}: ", frame_num + 1); // 1-based indexing
            if ft_entry.status == FRAME_TAKEN {
                println!("Job {}, page {}", ft_entry.job_number, ft_entry.page_number + 1); // 1-based indexing
            }
            else {
                println!("(free)");
                free_frames_list.push((frame_num as u32) + 1);
            }
        }
        else {
            println!("ERROR: Out of bounds frame table access!");
            status = ERROR;
        }
    }
    println!("Free frames: {:?} ({} free)", free_frames_list, free_frames_list.len());

    // Print Page Tables
    println!("\n--- Page Tables ---");

    let mut total_if: u32 = 0;
    for i in 0..job_list.len() {
        if let Some(job) = job_list.get(i) {
            print!("Job {} [", job.number);
            if job.status == RESIDENT {print!("RESIDENT] ");} else {print!("SUSPENDED] ");}
            println!("size={} pages={} int.frag={}", job.size, job.page_table.len(), job.int_frag);
            for p in 0..job.page_table.len() {
                print!("  page {} -> ", p+1);
                if job.status == RESIDENT {
                    println!("frame {}", job.page_table[p]+1);
                }
                else {
                    println!("(secondary storage)");
                }
            }

            total_if += job.int_frag;
        }
        else {
            println!("ERROR: Out of bounds job list access!");
            status = ERROR;
        }
    }

    // Print total internal frag.
    println!("\nTotal internal fragmentation (resident): {} bytes", total_if);

    return status;
}


fn exit() {

}


fn job_admission(ready_job: &mut Job, job_list: &mut Vec<Job>, frame_table: &mut Vec<FrameTableEntry>, job_fifo: &mut VecDeque<u32>,
                 free_frames: &mut u32) -> u32 {

    let mut status: u32 = OKAY;

    if ready_job.page_table.len() > *free_frames as usize {
        // Evict jobs until there is enough free space
    }

    // Load job into memory one frame at a time
    let mut page_ptr: usize = 0;
    let mut job_loaded: bool = false;

    while !job_loaded {

        // Check for first available free frames
        for frame_num in 0..frame_table.len() {
            if let Some(ft_entry) = frame_table.get_mut(frame_num as usize) {

                if ft_entry.status == FRAME_FREE {

                    // Add job's page info to FT
                    ft_entry.status = FRAME_TAKEN;
                    ft_entry.job_number = ready_job.number;
                    ft_entry.page_number = page_ptr as u32;
                    *free_frames -= 1;

                    // Update job's PT
                    ready_job.page_table[page_ptr] = frame_num as u32;
                    if page_ptr >= ready_job.page_table.len()-1 {
                        job_loaded = true;
                        break;
                    }
                    else {
                        page_ptr += 1;
                    }
                }
            }
            else {
                println!("ERROR: Out of bounds frame table access!");
                return ERROR;
            }
        }
            
    }

    // -------- Update Job Control Info ---------
    ready_job.status = RESIDENT;
    job_fifo.push_back(ready_job.number);

    return status;

}