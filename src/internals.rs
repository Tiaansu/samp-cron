use crate::SampCron;
use rcron::Uuid;

#[derive(Debug)]
pub enum ArgumentTypes {
    Primitive(i32),
    String(Vec<u8>),
}

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