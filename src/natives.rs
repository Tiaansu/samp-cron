use std::str::FromStr;

use crate::internals::{get_job_remaining_time, insert_job, ArgumentTypes};
use crate::plugin::JobInfo;
use croner::Cron;
use log::error;
use samp::error::AmxError;
use samp::{native, prelude::*};

impl super::SampCron<'static> {
    #[native(raw, name = "cron_new")]
    pub fn cron_new(
        &mut self,
        self_amx: &'static Amx,
        mut args: samp::args::Args,
    ) -> AmxResult<i32> {
        let cron_pattern = args
            .next::<AmxString>()
            .ok_or(AmxError::Params)?
            .to_string();
        let callback_name = args
            .next::<AmxString>()
            .ok_or(AmxError::Params)?
            .to_string();
        let mut format: Vec<u8> = Vec::new();

        if Cron::from_str(&cron_pattern).is_err() {
            error!("Invalid CRON expression: {}", cron_pattern);
            return Ok(0);
        }

        if args.count() > 2 {
            if let Some(specifiers) = args.next::<AmxString>() {
                format = specifiers.to_bytes();
            }
        }

        if !format.is_empty() && format.len() != args.count() - 3 {
            error!(
                "The argument count mismatch expected: {} provided: {}.",
                format.len(),
                args.count() - 3
            );
            return Ok(0);
        }

        let mut optional_args: Vec<ArgumentTypes> = Vec::new();

        for specifiers in format {
            match specifiers {
                b'd' | b'i' | b'f' => {
                    optional_args.push(ArgumentTypes::Primitive(
                        *args.next::<Ref<i32>>().ok_or(AmxError::Params)?,
                    ));
                }
                b's' => {
                    let argument: Ref<i32> = args.next().ok_or(AmxError::Params)?;
                    let amx_str = AmxString::from_raw(self_amx, argument.address())?;
                    optional_args.push(ArgumentTypes::String(amx_str.to_bytes()));
                }
                _ => {
                    error!("Unknown specifier type: {}", specifiers);
                    return Ok(0);
                }
            }
        }

        let raw = self_amx.ident();
        let pattern: Cron = cron_pattern.parse().unwrap();

        let callback = Box::new(move || {
            if let Some(amx) = samp::amx::get(raw) {
                let allocator = amx.allocator();

                for param in optional_args.iter().rev() {
                    match param {
                        ArgumentTypes::Primitive(x) => {
                            if amx.push(x).is_err() {
                                error!("Cannot execute callback {:?} [1]", callback_name);
                            }
                        }
                        ArgumentTypes::String(data) => {
                            let buf = allocator.allot_buffer(data.len() + 1).unwrap();
                            let amx_str = unsafe { AmxString::new(buf, data) };
                            if amx.push(amx_str).is_err() {
                                error!("Cannot execute callback {:?} [2]", callback_name);
                            }
                        }
                    }
                }

                if let Ok(index) = amx.find_public(&callback_name) {
                    if amx.exec(index).is_err() {
                        error!("Cannot execute callback {:?}", callback_name);
                    }
                }
            } else {
                error!("AMX not found when firing callback {:?}", callback_name);
            }
        });

        let id = insert_job();

        self.job_infos.push(JobInfo {
            id,
            schedule: pattern,
            last_tick: None,
            callback,
        });

        Ok(id as i32)
    }

    #[native(name = "cron_is_valid")]
    pub fn cron_is_valid(&mut self, _: &Amx, index: i32) -> AmxResult<bool> {
        Ok(self.job_infos.iter().any(|j| j.id == index as usize))
    }

    #[native(name = "cron_delete")]
    pub fn cron_delete(&mut self, _: &Amx, value: i32) -> AmxResult<bool> {
        let id = value as usize;
        if let Some(index) = self.job_infos.iter().position(|j| j.id == id) {
            self.job_infos.remove(index);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[native(name = "cron_get_remaining_time")]
    pub fn cron_get_remaining_time(&mut self, _: &Amx, index: usize) -> AmxResult<i32> {
        Ok(get_job_remaining_time(self, index))
    }
}
