#[macro_export]
macro_rules! colprintln {
    ($fmt:expr $(, $args:tt)*) => {{
        let formatted_str = format!($fmt $(, $args)*);
        // Call the color_tags function to add color codes
        let colored_str = $crate::macros::colprint::color_tags(&formatted_str);
        println!("{}", colored_str);
    }};
}

#[macro_export]
macro_rules! eclprintln {
    ($fmt:expr $(, $args:tt)*) => {{
        let formatted_str = format!($fmt $(, $args)*);
        // Call the replace_color_tags function to add color codes
        let colored_str = $crate::macros::colprint::color_tags(&formatted_str);
        eprintln!("{}", colored_str);
    }};
}

/// Replaces custom color tags (like `<r>`, `<g>`) with ANSI color codes
pub fn color_tags(input: &str) -> String {
    let mut output = input.to_string();
    let colors = vec![
        ("<b>", "\x1b[1;30m"),  // Bold black
        ("<r>", "\x1b[1;31m"),  // Bold red
        ("<g>", "\x1b[1;32m"),  // Bold green
        ("<y>", "\x1b[1;33m"),  // Bold yellow
        ("<bl>", "\x1b[1;34m"), // Bold blue
        ("<m>", "\x1b[1;35m"),  // Bold magenta
        ("<c>", "\x1b[1;36m"),  // Bold cyan
        ("<w>", "\x1b[1;37m"),  // Bold white
        ("</>", "\x1b[0m"),     // Reset
    ];

    for (tag, code) in colors {
        output = output.replace(tag, code);
    }

    output
}
