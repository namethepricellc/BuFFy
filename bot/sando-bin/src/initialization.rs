use anyhow::Result;
use colored::Colorize;
use fern::colors::{Color, ColoredLevelConfig};
use indoc::indoc;
use log::LevelFilter;

pub fn print_banner() {
    let banner = indoc! {
r#"

 _____   ___   _   _ ______  _____         ______  _____
/  ___| / _ \ | \ | ||  _  \|  _  |        | ___ \/  ___|
\ `--. / /_\ \|  \| || | | || | | | ______ | |_/ /\ `--.
 `--. \|  _  || . ` || | | || | | ||______||    /  `--. \
/\__/ /| | | || |\  || |/ / \ \_/ /        | |\ \ /\__/ /
\____/ \_| |_/\_| \_/|___/   \___/         \_| \_|\____/

______ __   __     _____       ______  _   _  _____ _____  ___   _       ___   _   _
| ___ \\ \ / / _  |  _  |      | ___ \| | | ||  __/|  __/ / _ \ | |     / _ \ | | | |
| |_/ / \ V / (_) | |/' |__  __| |_/ /| | | || |__ | |__ / /_\ \| |    / | | \| |_| |
| ___ \  \ /      |  /| |\ \/ /| ___ \| | | ||  _/ |  _/ |  _  || |    | | | ||  _  |
| |_/ /  | |   _  \ |_/ / >  < | |_/ /\ \_/ /| |   | |   | | | || |____\ \_/ /| | | |
\____/   \_/  (_)  \___/ /_/\_\\____/  \___/ |_|   |_|   |_| |_|\_____/ \___/ |_| |_|
"#};

    log::info!("{}", format!("{}", banner.green().bold()));
}

pub fn setup_logger() -> Result<()> {
    let colors = ColoredLevelConfig {
        trace: Color::Cyan,
        debug: Color::Magenta,
        info: Color::Green,
        warn: Color::Red,
        error: Color::BrightRed,
        ..ColoredLevelConfig::new()
    };

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Local::now().format("[%H:%M:%S]"),
                colors.color(record.level()),
                message
            ))
        })
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .level(log::LevelFilter::Error)
        .level_for("rusty_sando", LevelFilter::Info)
        .level_for("strategy", LevelFilter::Info)
        .level_for("artemis_core", LevelFilter::Info)
        .apply()?;

    Ok(())
}
