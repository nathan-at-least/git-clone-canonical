pub fn log_init() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
}
