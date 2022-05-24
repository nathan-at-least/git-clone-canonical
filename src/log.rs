use crate::Result;

pub fn log_init() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .format_level(true)
        .format_module_path(false)
        .format_target(false)
        .format_timestamp(None)
        .try_init()?;
    Ok(())
}
