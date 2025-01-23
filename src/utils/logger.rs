pub fn init_logger() {
    let mut logger = env_logger::Builder::from_default_env();
    logger.filter(Some("cat_render"), log::LevelFilter::Warn);
    logger.init();
}