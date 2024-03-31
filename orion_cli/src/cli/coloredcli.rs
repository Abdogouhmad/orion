use clap::builder::{styling::AnsiColor, Styles};

pub fn get_styles() -> Styles {
    Styles::styled()
        .usage(AnsiColor::Magenta.on_default())
        .header(AnsiColor::Magenta.on_default())
        .literal(AnsiColor::Green.on_default())
        .invalid(AnsiColor::Red.on_default())
        .error(AnsiColor::Red.on_default())
        .valid(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Blue.on_default())
}
