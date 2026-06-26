use crate::SampCron;

#[derive(Debug)]
pub enum ArgumentTypes {
    Primitive(i32),
    String(Vec<u8>),
}

static mut GLOBAL_INDEX: usize = 0;

pub fn insert_job() -> usize {
    unsafe {
        GLOBAL_INDEX += 1;
        return GLOBAL_INDEX;
    }
}

pub fn get_job_remaining_time(samp_cron: &mut SampCron, job_id: usize) -> i32 {
    for job_info in &samp_cron.job_infos {
        if job_info.id == job_id {
            let now = chrono::Local::now();
            match job_info.schedule.find_next_occurrence(&now, false) {
                Ok(next) => {
                    let remaining = (next - now).num_seconds() as i32;
                    return remaining.max(0);
                }
                Err(_) => return -1,
            }
        }
    }
    -1
}
