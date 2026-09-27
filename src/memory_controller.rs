/*
 * Memory Controller 
 */

use std::{collections::VecDeque};

// Memory Controller Actions
pub const ADMIT_NEW_JOB:  u32 = 10;
pub const REMOVE_JOB:     u32 = 11;
pub const SUSPEND_JOB:    u32 = 12;
pub const RESUME_JOB:     u32 = 13;
pub const TRANSLATE_ADDR: u32 = 14;
pub const PRINT:          u32 = 15;
pub const EXIT:           u32 = 16;
pub const HEADER:         u32 = 17;

// Memory controller error codes
pub const OKAY:           u32 = 0;
pub const DUPLICATE_JOB:  u32 = 20;
pub const MASSIVE_JOB:    u32 = 21;
pub const INVALID_JOB:    u32 = 22;
pub const ERROR:          u32 = 255;

// Frame and Page markers
pub const RESIDENT:      bool = true;
pub const SUSPENDED:     bool = false;
pub const FRAME_TAKEN:   bool = true;
pub const FRAME_FREE:    bool = false;


/// Struct to hold a Job for the system
/// 
/// # Fields
/// 
/// - `number` (`u32`) - Job ID number
/// - `status` (`bool`) - True = resident, FALSE = suspended
/// - `size` (`u32`) - size of job in bytes
/// - `int_frag` (`u32`) - internal fragmentation from last page
/// - `page_table` (`Vec<u32>`) - Map from pages to frames in main memory
/// 
pub struct Job {
    number: u32,
    status: bool,
    size: u32,
    int_frag: u32,
    page_table: Vec<u32>
}


/// Entry into the frame table that indicates whether the frame is taken or not.
/// If taken, describes which job and which page of that job is using it.
/// 
/// # Fields
/// 
/// - `status` (`bool`) - True = frame taken, False = frame free
/// - `job_number` (`u32`) - If taken: Job ID number associated with the frame.
///                          If free: N/A
/// - `page_number` (`u32`) - If taken: Page number associated with the frame.
///                           If free: N/A
/// 
pub struct FrameTableEntry {
    status: bool,
    job_number: u32,
    page_number: u32
}

/// Public Frame Table Entry constructor for external use
/// 
/// # Arguments
/// 
/// (See struct implementation for arguments)
/// 
/// # Returns
/// 
/// - `Self`
/// 
impl FrameTableEntry {
    pub fn new(status: bool, job_number: u32, page_number: u32)-> Self {
        FrameTableEntry { status, job_number, page_number }    
    }
}


/// Public entry point into the memory controller.
/// 
/// # Arguments
/// 
/// - `action` (`u32`) - What the caller wants to do with the memory controller.
///                      See constants defined above.
/// - `args` (`Vec<&str>`) - Stripped CLI arguments. Action-specific.
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
/// - `u32` - Status code
/// 
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
            status = remove_job(job_list, frame_table, job_fifo, args[0].parse::<u32>().unwrap(), free_frames);
        },
        SUSPEND_JOB=> 
        {
            // args[0] = Job ID
            status = suspend_job(job_list, frame_table, job_fifo, args[0].parse::<u32>().unwrap(), free_frames);
        },
        RESUME_JOB=> 
        {
            // args[0] = Job ID
            status = resume_job(job_list, frame_table, job_fifo, args[0].parse::<u32>().unwrap(), free_frames);
        },
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


/// Admits a job into memory or rejects it for being too large or a duplicate
/// 
/// # Arguments
/// 
/// - `job_list` (`&mut Vec<Job>`) - List of all jobs in the system
/// - `frame_table` (`&mut Vec<FrameTableEntry>`) - Maps taken frames to page and job numbers
/// - `job_fifo` (`&mut VecDeque<u32>`) - FIFO of job numbers admitted into main memory
/// - `num` (`u32`) - Job number to be admitted
/// - `size` (`u32`) - Size of job to be admitted
/// - `mem_size` (`u32`) - Size of main memory in bytes
/// - `frame_size` (`u32`) - Size of each page/frame in bytes
/// - `num_frames` (`u32`) - Number of frames in main memory
/// - `free_frames` (`&mut u32`) - Number of free frames in main memory
/// 
/// # Returns
/// 
/// - `u32` - Status code
/// 
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
    job_list.push(new_job);
    status = job_admission(job_list.len()-1, job_list, frame_table, job_fifo, free_frames); 

    return status;
}


/// Removes a Job from the system permanently
/// 
/// # Arguments
/// 
/// - `job_list` (`&mut Vec<Job>`) - List of all jobs in the system
/// - `frame_table` (`&mut Vec<FrameTableEntry>`) - Maps taken frames to page and job numbers
/// - `job_fifo` (`&mut VecDeque<u32>`) - FIFO of job numbers admitted into main memory
/// - `num` (`u32`) - Job number to be removed.
/// - `free_frames` (`&mut u32`) - Number of free frames in main memory
/// 
/// # Returns
/// 
/// - `u32` - Status code
/// 
fn remove_job(job_list: &mut Vec<Job>, frame_table: &mut Vec<FrameTableEntry>, job_fifo: &mut VecDeque<u32>, num: u32, free_frames: &mut u32) -> u32 {

    // Check if job # matches "num"
    // If so, update the frame table based on PT
    // remove it from the job list, job fifo
    for i in 0..job_list.len() {
        
        if let Some(job) = job_list.get_mut(i) {
            if job.number == num {

                // Update frame table for every entry in the job's page table
                for page in 0..job.page_table.len() {
                    frame_table[job.page_table[page] as usize].status = FRAME_FREE;
                    *free_frames += 1;
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


/// Suspends a job by moving it from main memory to secondary storage.
/// 
/// # Arguments
/// 
/// - `job_list` (`&mut Vec<Job>`) - List of all jobs in the system
/// - `frame_table` (`&mut Vec<FrameTableEntry>`) - Maps taken frames to page and job numbers
/// - `job_fifo` (`&mut VecDeque<u32>`) - FIFO of job numbers admitted into main memory
/// - `num` (`u32`) - Job number to be suspended.
/// - `free_frames` (`&mut u32`) - Number of free frames in main memory
/// 
/// # Returns
/// 
/// - `u32` - Status code
/// 
fn suspend_job(job_list: &mut Vec<Job>, frame_table: &mut Vec<FrameTableEntry>, job_fifo: &mut VecDeque<u32>, num: u32, free_frames: &mut u32) -> u32 {

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
                    *free_frames += 1;
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


/// Resumes a job by moving it from secondary storage to main memory.
/// 
/// # Arguments
/// 
/// - `job_list` (`&mut Vec<Job>`) - List of all jobs in the system
/// - `frame_table` (`&mut Vec<FrameTableEntry>`) - Maps taken frames to page and job numbers
/// - `job_fifo` (`&mut VecDeque<u32>`) - FIFO of job numbers admitted into main memory
/// - `num` (`u32`) - Job number to be resumed.
/// - `free_frames` (`&mut u32`) - Number of free frames in main memory
/// 
/// # Returns
/// 
/// - `u32` - Status code
/// 
fn resume_job(job_list: &mut Vec<Job>, frame_table: &mut Vec<FrameTableEntry>, job_fifo: &mut VecDeque<u32>, num: u32, free_frames: &mut u32) -> u32 {

    for i in 0..job_list.len() {

       if job_list[i].number == num {

            // Check if already resident
            if job_list[i].status == RESIDENT {
                println!("ERROR: Job {} is already running", num);
                return ERROR;
            }
            // Try to admit job
            if job_admission(i, job_list, frame_table, job_fifo, free_frames) != OKAY {
                println!("ERROR: Could not resume job {}", num);
            }

            return OKAY;
        }

    }

    println!("ERROR: Job {} cannot be resumed because it doesn't exist.", num);
    return INVALID_JOB;
}


fn translate_addr(job_list: &Vec<Job>, job_num: u32, addr: u32) -> (u32, u32) {

    return (0,0);
}


/// Prints the status of the system: frame table and per-job page tables
/// 
/// # Arguments
/// 
/// - `job_list` (`&mut Vec<Job>`) - List of all jobs in the system
/// - `frame_table` (`&mut Vec<FrameTableEntry>`) - Maps taken frames to page and job numbers
/// - `job_fifo` (`&mut VecDeque<u32>`) - FIFO of job numbers admitted into main memory
/// 
/// # Returns
/// 
/// - `u32` - Status code
/// 
/// # Examples
/// 
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


/**
 * Admits jobs into main memory. 
 * Only adds them to the resident FIFO, caller needs to make sure
 * they are in the Job list if they aren't already.
 */

/// Mechanism to actually move jobs into main memory.
/// Suspends resident jobs in FIFO order until there is sufficient room.
/// 
/// 
/// # Arguments
/// 
/// - `job_index` (`usize`) - Location in `job_list` of the job to be admitted.
/// - `job_list` (`&mut Vec<Job>`) - List of all jobs in the system
/// - `frame_table` (`&mut Vec<FrameTableEntry>`) - Maps taken frames to page and job numbers
/// - `job_fifo` (`&mut VecDeque<u32>`) - FIFO of job numbers admitted into main memory
/// - `free_frames` (`&mut u32`) - Number of free frames in main memory
/// 
/// !! Assumes job exists in `job_list` !!
/// 
/// # Returns
/// 
/// - `u32` - Status code
/// 
fn job_admission(job_index: usize, job_list: &mut Vec<Job>, frame_table: &mut Vec<FrameTableEntry>, job_fifo: &mut VecDeque<u32>,
                 free_frames: &mut u32) -> u32 {

    let mut status: u32 = OKAY;

    // Evict jobs until there is enough free space
    for job_num in 0..job_fifo.len() {

        if job_list[job_index].page_table.len() > *free_frames as usize {
            // Evict from head of FIFO
            if let Some(evict_job) = job_fifo.front() {
                suspend_job(job_list, frame_table, job_fifo, *evict_job, free_frames);  
            }
            else {
                println!("ERROR: Out of bound Job FIFO access!");
                return ERROR;
            }          
        }
        else {
            break;
        }

    }

    if let Some(ready_job) = job_list.get_mut(job_index) {

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

                        // Exit loop if all pages are loaded
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

    }
    else {
        println!("ERROR: Out of bounds job list access!");
        return ERROR;
    }

    return status;

}