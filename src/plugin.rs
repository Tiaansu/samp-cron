use std::time::{Duration, Instant};

use job_scheduler_ng::Cron;
use log::info;
use samp::amx::AmxIdent;
use samp::plugin::SampPlugin;
use samp::prelude::*;

pub struct JobInfo {
    pub id: usize,
    pub schedule: Cron,
    pub last_tick: Option<chrono::DateTime<chrono::Local>>,
    pub callback: Box<dyn Fn() + Send>,
}

pub struct SampCron<'a> {
    pub amx_list: Vec<AmxIdent>,
    pub job_infos: Vec<JobInfo>,
    pub next_tick: Instant,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl SampCron<'static> {
    pub fn new() -> Self {
        SampCron {
            amx_list: Vec::new(),
            job_infos: Vec::new(),
            next_tick: Instant::now(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl SampPlugin for SampCron<'static> {
    fn on_load(&mut self) {
        info!("Version: 0.2.0");
        self.next_tick = Instant::now();
    }

    fn on_unload(&mut self) {
        info!("unloading plugin...");
        self.job_infos.clear();
        info!("unloaded plugin.");
    }

    fn on_amx_load(&mut self, amx: &Amx) {
        self.amx_list.push(amx.ident());
    }

    fn on_amx_unload(&mut self, amx: &Amx) {
        let raw = amx.ident();
        if let Some(index) = self.amx_list.iter().position(|x| *x == raw) {
            self.amx_list.remove(index);
        }
    }

    fn process_tick(&mut self) {
        let now = Instant::now();
        if now < self.next_tick {
            return;
        }
        self.next_tick = now + Duration::from_millis(500);

        let now_chrono = chrono::Local::now();

        for job_info in self.job_infos.iter_mut() {
            match job_info.last_tick {
                None => {
                    job_info.last_tick = Some(now_chrono);
                }
                Some(last) => match job_info.schedule.find_next_occurrence(&last, false) {
                    Ok(next) => {
                        if next <= now_chrono {
                            (job_info.callback)();
                            job_info.last_tick = Some(now_chrono);
                        }
                    }
                    Err(_) => {
                        job_info.last_tick = None;
                    }
                },
            }
        }
    }
}
