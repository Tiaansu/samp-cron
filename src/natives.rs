use crate::internals::insert_uuid;
use log::error;
use rcron::{Job, Schedule};
use samp::error::AmxError;
use samp::{native, prelude::*};
use std::str::FromStr;

impl super::SampCron<'static> {
    #[native(raw, name = "cron_new")]
    pub fn cron_new(&mut self, amx: &'static Amx, mut args: samp::args::Args) -> AmxResult<i32> {
        let cron_pattern = args.next::<AmxString>().ok_or(AmxError::Params)?.to_string();
        let callback_name = args.next::<AmxString>().ok_or(AmxError::Params)?.to_string();

        if Schedule::from_str(&cron_pattern).is_err() {
            error!("Invalid CRON expression: {}", cron_pattern);
            return Ok(0);
        }
        
        let public_index = amx.find_public(&callback_name);

        if public_index.is_err() {
            error!("Invalid public: {}", callback_name);
            return Ok(0);
        }

        let job = Job::new(cron_pattern.parse().unwrap(), move || {
            if amx.exec(public_index.as_ref().unwrap().to_owned()).is_err() {
                error!("Cannot execute callback: {:?}", callback_name);
            }
        });
        let uuid = self.scheduler.add(job);
        let id = insert_uuid(self, uuid);

        Ok(id.as_cell())
    }

    #[native(name = "cron_is_valid")]
    pub fn cron_is_valid(&mut self, _: &Amx, index: i32) -> AmxResult<bool> {
        Ok(self.schedules.get(index as usize - 1).is_some())
    }

    #[native(name = "cron_delete")]
    pub fn cron_delete(&mut self, _: &Amx, value: i32) -> AmxResult<bool> {
        let index = value as usize - 1;
        if let Some(job_id) = self.schedules.get(index).cloned() {
            self.schedules.remove(index);
            Ok(self.scheduler.remove(job_id))
        } else {
            Ok(false)
        }
    }
}