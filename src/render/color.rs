//! Key/value colours for the info column.
//!
//! Logo art carries its own palette; see `crate::logo`.

use owo_colors::Rgb;

/// Global colour settings for one run.
#[derive(Clone, Debug)]
pub struct Style {
    /// False when stdout is not a TTY, `NO_COLOR` is set, or `--no-color`.
    pub colors: bool,
    pub key: Rgb,
    pub value: Rgb,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            colors: true,
            // Fedora blue for keys, bright white for values.
            key: Rgb(0x60, 0x6e, 0xb4),
            value: Rgb(0xff, 0xff, 0xff),
        }
    }
}

impl Style {
    /// Wraps `text` in `color` when colours are enabled, else returns it as is.
    pub fn paint(&self, text: &str, color: Rgb) -> String {
        if !self.colors {
            return text.to_string();
        }
        format!(
            "\x1b[38;2;{};{};{}m{text}\x1b[0m",
            color.0, color.1, color.2
        )
    }

    /// Bold variant, used for the `OS` key so the header line stands out.
    pub fn paint_bold(&self, text: &str, color: Rgb) -> String {
        if !self.colors {
            return text.to_string();
        }
        format!(
            "\x1b[1;38;2;{};{};{}m{text}\x1b[0m",
            color.0, color.1, color.2
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_style_emits_no_escapes() {
        let s = Style {
            colors: false,
            ..Default::default()
        };
        assert_eq!(s.paint("x", Rgb(1, 2, 3)), "x");
        assert_eq!(s.paint_bold("x", Rgb(1, 2, 3)), "x");
    }

    #[test]
    fn enabled_style_emits_truecolor() {
        let s = Style::default();
        assert_eq!(s.paint("x", Rgb(1, 2, 3)), "\x1b[38;2;1;2;3mx\x1b[0m");
    }
}
