use clap::builder::{styling::AnsiColor, Styles};

pub fn get_styles() -> Styles {
    Styles::styled()
        .usage(AnsiColor::BrightRed.on_default())
        .header(AnsiColor::BrightGreen.on_default())
        .literal(AnsiColor::Cyan.on_default())
        .invalid(AnsiColor::Red.on_default())
        .error(AnsiColor::Red.on_default())
        .valid(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default())
}
