use log::info;
use rcron::{JobScheduler, Uuid};
use samp::amx::AmxIdent;
use samp::plugin::SampPlugin;
use samp::prelude::*;

pub struct SampCron<'a> {
    pub amx_list: Vec<AmxIdent>,
    pub scheduler: JobScheduler<'a>,
    pub schedules: Vec<Uuid>,
}

impl SampPlugin for SampCron<'static> {
    fn on_load(&mut self) {
        info!("Version: 0.1.0");
    }

    fn on_unload(&mut self) {
        info!("unloading plugin...");
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
        self.scheduler.tick();
    }
}