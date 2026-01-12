/*

[dependencies]
log = "0.4"
env_logger = "0.8"

*/

fn main() {
    // bu olmadan log çalışmıyor
    // hata'da vermiyor
    env_logger::init();

    /*
    bunlar logging implementations

    env_logger
    simple_logger
    simplelog
    pretty_env_logger
    stderrlog
    flexi_logger

    ben env_logger kullandım
         */

    log::debug!("DEBUG");
    log::error!("ERROR");
    log::info!("INFO");
    log::trace!("TRACE");
    log::warn!("WARN");

    /*
    pub enum Level {
        Error,
        Warn,
        Info,
        Debug,
        Trace,
    } */

    log::log!(log::Level::Error, "{}", "LOG");
}
