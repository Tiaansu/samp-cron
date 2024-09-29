use crate::internals::{insert_uuid, ArgumentTypes};
use log::error;
use rcron::{Job, Schedule};
use samp::error::AmxError;
use samp::{native, prelude::*};
use std::str::FromStr;

impl super::SampCron<'static> {
    #[native(raw, name = "cron_new")]
    pub fn cron_new(&mut self, self_amx: &'static Amx, mut args: samp::args::Args) -> AmxResult<i32> {
        let cron_pattern = args.next::<AmxString>().ok_or(AmxError::Params)?.to_string();
        let callback_name = args.next::<AmxString>().ok_or(AmxError::Params)?.to_string();
        let mut format: Vec<u8> = Vec::new();

        let splitted: Vec<&str> = cron_pattern.split(' ').collect();

        if splitted.len() < 6 {
            error!("Insufficient cron pattern specified. Expected 6, got {}", splitted.len());
            return Ok(0);
        }

        if Schedule::from_str(&cron_pattern).is_err() {
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

        let job = Job::new(cron_pattern.parse().unwrap(), move || {
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