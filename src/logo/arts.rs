//! ASCII artwork, one per distribution.

use crate::render::color::Gradient;
use owo_colors::Rgb;

pub struct Logo {
    pub id: &'static str,
    pub lines: &'static [&'static str],
    pub gradient: Gradient,
}

impl Logo {
    /// Widest visible line of the artwork.
    pub fn width(&self) -> usize {
        self.lines
            .iter()
            .map(|l| crate::render::width::visible_len(l))
            .max()
            .unwrap_or(0)
    }

    /// Returns the artwork with a per-line gradient applied.
    pub fn paint(&self, style: &crate::render::color::Style) -> Vec<String> {
        let n = self.lines.len();
        self.lines
            .iter()
            .enumerate()
            .map(|(i, line)| {
                // Single-line art gets the start colour rather than a
                // degenerate gradient, so it stays visible.
                let t = if n <= 1 {
                    0.0
                } else {
                    i as f32 / (n - 1) as f32
                };
                style.paint(line, self.gradient.sample(t))
            })
            .collect()
    }
}

macro_rules! logo {
    ($id:literal, $grad:expr, [$($line:literal),* $(,)?]) => {
        Logo { id: $id, lines: &[$($line),*], gradient: $grad }
    };
}

/// Blue-to-navy gradient shared by the Fedora family.
const FEDORA: Gradient = Gradient {
    start: Rgb(0x60, 0x6e, 0xb4),
    end: Rgb(0x29, 0x41, 0x72),
};

/// Cyan-to-blue, Arch's signature.
const ARCH: Gradient = Gradient {
    start: Rgb(0x18, 0xa3, 0xdb),
    end: Rgb(0x0c, 0x39, 0x6e),
};

/// Debian red.
const DEBIAN: Gradient = Gradient {
    start: Rgb(0xd7, 0x07, 0x51),
    end: Rgb(0xa8, 0x11, 0x2d),
};

/// Ubuntu aubergine.
const UBUNTU: Gradient = Gradient {
    start: Rgb(0xe9, 0x54, 0x20),
    end: Rgb(0x77, 0x22, 0x28),
};

const GREEN: Gradient = Gradient {
    start: Rgb(0x6b, 0xd4, 0x6b),
    end: Rgb(0x2f, 0x7a, 0x2f),
};

const PURPLE: Gradient = Gradient {
    start: Rgb(0xa8, 0x79, 0xd6),
    end: Rgb(0x5e, 0x35, 0x8a),
};

const TEAL: Gradient = Gradient {
    start: Rgb(0x4d, 0xd0, 0xc4),
    end: Rgb(0x1d, 0x77, 0x74),
};

const SILVER: Gradient = Gradient {
    start: Rgb(0xe8, 0xec, 0xf1),
    end: Rgb(0x8f, 0x9a, 0xad),
};

pub fn all() -> &'static [Logo] {
    &[
        logo!(
            "fedora",
            FEDORA,
            [
                r"        ,.=:^!^!t3Z3z.,",
                r"     :tt:::tt333EE3",
                r"    Et:::ztt33EEEL",
                r"   ;tt:::tt333EE7",
                r"  :Et:::zt333EEQ.",
                r"  it::::tt333EEF",
                r#" ;3=*^```"*4E7"#,
                r" :tt:::tt333EE7",
                r"  ;Ett:::zt333EEQ.",
                r"   ;Et:::ztt333EEF",
                r"     :tt:::tt333EE7",
            ]
        ),
        logo!(
            "arch",
            ARCH,
            [
                r"                   -`",
                r"                  .o+`",
                r"                 `ooo/",
                r"                `+oooo:",
                r"               `+oooooo:",
                r"               -+oooooo+:",
                r"             `/:-:++oooo+:",
                r"            `/++++/+++++++:",
                r"           `/++++++++++++++:",
                r"         `/+++ooooooooooooo/`",
                r"        ./ooosssso++osssssso+`",
                r"       .oossssso-` ``*/ossssss+`",
                r"      -osssssso.       `:ssssssso.",
                r"     :osssssss/         osssso+++.",
                r"    /ossssssss/         `:sssso+++-`",
                r"   `ossssso+/:-`          `:-/+osssso+`",
                r"  `osssso+:-`                `:-/+oso:`",
                r" `+sso+:-`                       `:/+/:`",
                r".++/+:-`",
            ]
        ),
        logo!(
            "debian",
            DEBIAN,
            [
                r"  ______           __",
                r" /      \         / /",
                r"|        |       / / ",
                r"|        |      / /  ",
                r"|        | ___ / /   ",
                r"|       _/| _ \/ /    ",
                r"|      | | | | \ \ \ \",
                r"|     | | |_| |  \ \ \",
                r" \____|_|\__\_|   \_\ \_\",
            ]
        ),
        logo!(
            "ubuntu",
            UBUNTU,
            [
                r"             .-/+oOo+-.",
                r"           `:.o+oOo-o.:`",
                r"          `:.+oOoO-.+:`",
                r"         `:.+.:+oOo-.`",
                r"      `-::-.:+oOoO/`",
                r"           .-/+oOo+-.",
                r"         `:/+.:+oOoO.",
                r"      `:.+.:+oOo-.:`",
            ]
        ),
        logo!(
            "mint",
            GREEN,
            [
                r"             ...-:::::-...",
                r"         .-MMMMMMMMMMMMM-.",
                r"      `:MMMMMMMMMMMMMMMMMM:`",
                r"    :MMMMMMMMMMMMMMMMMMMM:",
                r"   -MMMMMMMMMMMMMMMMMMMM-",
                r"  :MMMMMdddddddddMMMMMMM:",
                r"  MMMMdddddddddddddMMMMM",
                r"  MMMMdddddddddddddMMMMM",
                r"  :MMMMMdddddddddMMMMMMM:",
                r"   -MMMMMMMMMMMMMMMMMMMM-",
                r"    :MMMMMMMMMMMMMMMMMMMM:",
                r"      `:MMMMMMMMMMMMMMMMMM:`",
                r"         .-MMMMMMMMMMMMM-.",
                r"             ...-:::::-...",
            ]
        ),
        logo!(
            "nixos",
            SILVER,
            [
                r"                    ..",
                r"                   ....",
                r"                 ....",
                r"               ....",
                r"        ........",
                r"       ..........::",
                r"     ..........::::",
                r"   :::::::::::::::::",
                r"    ::::::::::::::::",
            ]
        ),
        logo!(
            "manjaro",
            GREEN,
            [
                r"                   .-.",
                r"                  : odd",
                r"                 o+++:",
                r"               .+oooo+:",
                r"             :+oooooo+:",
                r"           -+oooooooo+-`",
                r"         `/::-:-:-:-.:/`",
                r"         .`/:-:-:-:-.`",
            ]
        ),
        logo!(
            "gentoo",
            PURPLE,
            [
                r"                        ",
                r"           /\           ",
                r"          /  \          ",
                r"         /    \         ",
                r"        /      \        ",
                r"       /        \       ",
                r"      /__________\      ",
            ]
        ),
        logo!(
            "opensuse",
            GREEN,
            [
                r"        ,.=:^!^!t3Z3z.,",
                r"     :tt:::tt333EE3",
                r"    Et:::ztt33EEEL",
                r"   ;tt:::tt333EE7",
                r"  :Et:::zt333EEQ.",
                r"  it::::tt333EEF",
                r#" ;3=*^```"*4E7"#,
                r" :tt:::tt333EE7",
                r"  ;Ett:::zt333EEQ.",
                r"   ;Et:::ztt333EEF",
                r"     :tt:::tt333EE7",
            ]
        ),
        logo!(
            "alpine",
            TEAL,
            [
                r"      /\        ",
                r"     /  \       ",
                r"    /    \      ",
                r"   /      \     ",
                r"  /   /\   \    ",
                r" /___/  \___\   ",
            ]
        ),
        logo!(
            "void",
            SILVER,
            [
                r"        .:://::.       ",
                r"    .:///+++++///:.    ",
                r"   /+++:.     .:+++/:  ",
                r"  :+++.         .+++:  ",
                r"  :+++.         .+++:  ",
                r"   /+++:.     .:+++/:  ",
                r"    .:///+++++///:.    ",
                r"        .:://::.       ",
            ]
        ),
        logo!(
            "generic",
            SILVER,
            [
                r"    ______________",
                r"   /              \ ",
                r"  |   __________  |",
                r"  |  |          | |",
                r"  |  |          | |",
                r"  |  |__________| |",
                r"   \______________/",
            ]
        ),
    ]
}

/// Looks up an artwork by `/etc/os-release` ID.
pub fn by_id(id: &str) -> Option<&'static Logo> {
    all().iter().find(|l| l.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_finds_known_id() {
        assert_eq!(by_id("fedora").map(|l| l.id), Some("fedora"));
        assert_eq!(by_id("arch").map(|l| l.id), Some("arch"));
    }

    #[test]
    fn lookup_returns_none_for_unknown() {
        assert!(by_id("definitely-not-a-distro").is_none());
    }

    #[test]
    fn logo_width_matches_longest_line() {
        let l = by_id("fedora").unwrap();
        let expected = l.lines.iter().map(|s| s.chars().count()).max().unwrap();
        assert_eq!(l.width(), expected);
    }

    #[test]
    fn every_logo_has_lines_and_unique_id() {
        let logos = all();
        assert!(!logos.is_empty());
        for l in logos {
            assert!(!l.lines.is_empty(), "{} has no lines", l.id);
            assert!(l.width() > 0, "{} is empty", l.id);
        }
        for (i, a) in logos.iter().enumerate() {
            for b in &logos[i + 1..] {
                assert_ne!(a.id, b.id, "duplicate logo id {}", a.id);
            }
        }
    }

    #[test]
    fn paint_returns_one_line_per_art_line() {
        let l = by_id("fedora").unwrap();
        let style = crate::render::color::Style::default();
        assert_eq!(l.paint(&style).len(), l.lines.len());
    }

    #[test]
    fn paint_is_plain_without_color() {
        let l = by_id("fedora").unwrap();
        let style = crate::render::color::Style {
            colors: false,
            ..Default::default()
        };
        for (painted, raw) in l.paint(&style).iter().zip(l.lines) {
            assert_eq!(painted, raw);
        }
    }
}
