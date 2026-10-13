//! Colour palette and gradient maths for logos.

use owo_colors::Rgb;

/// A two-colour gradient used to paint logos.
#[derive(Clone, Copy, Debug)]
pub struct Gradient {
    pub start: Rgb,
    pub end: Rgb,
}

impl Gradient {
    /// Linearly interpolates `t` in `0.0..=1.0` between the two stops.
    pub fn sample(&self, t: f32) -> Rgb {
        let t = t.clamp(0.0, 1.0);
        Rgb(
            lerp(self.start.0, self.end.0, t),
            lerp(self.start.1, self.end.1, t),
            lerp(self.start.2, self.end.2, t),
        )
    }
}

fn lerp(a: u8, b: u8, t: f32) -> u8 {
    let v = a as f32 + (b as f32 - a as f32) * t;
    v.round().clamp(0.0, 255.0) as u8
}

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
    fn gradient_endpoints_are_exact() {
        let g = Gradient {
            start: Rgb(0, 0, 0),
            end: Rgb(255, 128, 10),
        };
        assert_eq!(g.sample(0.0), Rgb(0, 0, 0));
        assert_eq!(g.sample(1.0), Rgb(255, 128, 10));
    }

    #[test]
    fn gradient_clamps_out_of_range() {
        let g = Gradient {
            start: Rgb(10, 10, 10),
            end: Rgb(20, 20, 20),
        };
        assert_eq!(g.sample(-1.0), Rgb(10, 10, 10));
        assert_eq!(g.sample(5.0), Rgb(20, 20, 20));
    }

    #[test]
    fn gradient_midpoint_is_between() {
        let g = Gradient {
            start: Rgb(0, 0, 0),
            end: Rgb(100, 100, 100),
        };
        assert_eq!(g.sample(0.5), Rgb(50, 50, 50));
    }

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
