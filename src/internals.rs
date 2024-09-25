use crate::SampCron;
use rcron::Uuid;

static mut GLOBAL_INDEX: usize = 0;

pub fn insert_uuid(
    samp_cron: &mut SampCron,
    uuid: Uuid
) -> usize {
    unsafe {
        samp_cron.schedules.push(uuid);
        GLOBAL_INDEX += 1;
        return GLOBAL_INDEX;
    }
}