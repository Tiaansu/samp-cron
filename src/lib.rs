mod plugin;
mod internals;
mod natives;

use plugin::SampCron;
use rcron::JobScheduler;
use samp::initialize_plugin;

initialize_plugin!(
    natives: [
        SampCron::cron_new,
        SampCron::cron_delete,
        SampCron::cron_is_valid
    ],
    {
        samp::plugin::enable_process_tick();
        let samp_logger = samp::plugin::logger()
            .level(log::LevelFilter::Info);

        let _ = fern::Dispatch::new()
            .format(|callback, message, record| {
                callback.finish(format_args!("[SampCron] [{}]: {}", record.level().to_string().to_lowercase(), message))
            })
            .chain(samp_logger)
            .apply();

        SampCron {
            amx_list:Vec::new(),
            scheduler:JobScheduler::new(),
            schedules: Vec::new()
        }
    }
);
