use crate::SampCron;
use job_scheduler_ng::Uuid;

#[derive(Debug)]
pub enum ArgumentTypes {
    Primitive(i32),
    String(Vec<u8>),
}

static mut GLOBAL_INDEX: usize = 0;

pub fn insert_uuid(samp_cron: &mut SampCron, uuid: Uuid) -> usize {
    unsafe {
        samp_cron.schedules.push(uuid);
        GLOBAL_INDEX += 1;
        return GLOBAL_INDEX;
    }
}

pub fn get_job_remaining_time(samp_cron: &mut SampCron, job_id: usize) -> i32 {
    for job_info in &samp_cron.job_infos {
        if job_info.id == job_id {
            let now = chrono::Local::now();

            let next = job_info.schedule.find_next_occurrence(&now, true);

            if next.is_err() {
                return -1;
            }

            let remaining = (next.unwrap() - now).num_seconds() as i32;
            return remaining.max(0);
        }
    }
    -1
}
