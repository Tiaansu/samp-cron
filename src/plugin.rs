use std::time::{Duration, Instant};

use job_scheduler_ng::{Cron, JobScheduler, Uuid, MIN_DURATION};
use log::info;
use samp::amx::AmxIdent;
use samp::plugin::SampPlugin;
use samp::prelude::*;

pub struct JobInfo {
    pub id: usize,
    pub schedule: Cron,
}

pub struct SampCron<'a> {
    pub amx_list: Vec<AmxIdent>,
    pub scheduler: JobScheduler<'a>,
    pub schedules: Vec<Uuid>,
    pub next_tick: Instant,
    pub job_infos: Vec<JobInfo>,
}

impl SampPlugin for SampCron<'static> {
    fn on_load(&mut self) {
        info!("Version: 0.2.0");
        self.next_tick = Instant::now();

        let local_tz = chrono::Local::now();
        self.scheduler.set_timezone(*local_tz.offset());
    }

    fn on_unload(&mut self) {
        info!("unloading plugin...");
        for schedule in self.schedules.iter() {
            self.scheduler.remove(schedule.clone());
        }
        info!("unloaded plugin.");
    }

    fn on_amx_load(&mut self, amx: &Amx) {
        self.amx_list.push(amx.ident());
    }

    fn on_amx_unload(&mut self, amx: &Amx) {
        let raw = amx.ident();
        let index = self.amx_list.iter().position(|x| *x == raw).unwrap();
        self.amx_list.remove(index);
    }

    fn process_tick(&mut self) {
        let now = Instant::now();

        if now >= self.next_tick {
            let local_tz = chrono::Local::now();
            self.scheduler.set_timezone(*local_tz.offset());

            self.scheduler.tick();
            self.next_tick = now + Duration::from_millis(MIN_DURATION);
        }
    }
}
