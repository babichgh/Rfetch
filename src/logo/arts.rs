//! ASCII artwork, one per distribution.
//!
//! Drawing rules, applied to every entry so the set reads as one system:
//!
//! * structure comes from a fixed ink set: `. - | : ( ) + / \` for outlines,
//!   `# % @` for fills, and `:` or `///` for shade ramps;
//! * no letter runs (`MMMM`, `dddd`) that read as progress bars;
//! * logos that are radially or mirror symmetric actually are — `tests::`
//!   asserts this rather than trusting the source by eye.

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

const FEDORA: Gradient = Gradient {
    start: Rgb(0x74, 0x8f, 0xd0),
    end: Rgb(0x2b, 0x4a, 0x82),
};
const ARCH: Gradient = Gradient {
    start: Rgb(0x2a, 0xc0, 0xe8),
    end: Rgb(0x0c, 0x39, 0x6e),
};
const DEBIAN: Gradient = Gradient {
    start: Rgb(0xe0, 0x4f, 0x6d),
    end: Rgb(0x9c, 0x0f, 0x33),
};
const UBUNTU: Gradient = Gradient {
    start: Rgb(0xf0, 0x6c, 0x3c),
    end: Rgb(0x77, 0x22, 0x28),
};
const GREEN: Gradient = Gradient {
    start: Rgb(0x7e, 0xdc, 0x7e),
    end: Rgb(0x25, 0x6b, 0x2c),
};
const SILVER: Gradient = Gradient {
    start: Rgb(0xec, 0xf0, 0xf5),
    end: Rgb(0x7c, 0x8a, 0xa0),
};
const PURPLE: Gradient = Gradient {
    start: Rgb(0xb4, 0x8a, 0xe0),
    end: Rgb(0x5b, 0x2e, 0x8a),
};
const TEAL: Gradient = Gradient {
    start: Rgb(0x6f, 0xdd, 0xd2),
    end: Rgb(0x14, 0x6a, 0x68),
};

pub fn all() -> &'static [Logo] {
    &[
        // Fedora: the infinity-f inside its circle.
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
        // Arch: the peak.
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
        // Debian: the spiral.
        logo!(
            "debian",
            DEBIAN,
            [
                r"     ______",
                r"    /      \",
                r"   |        |       __     __",
                r"   |        |      / /    / /",
                r"   |        | ___ | |_   | |_    __  ___",
                r"   |        / _ \ |  _|  |  _|  / / / _ \",
                r"   |       | (_) || |    | |   / /_/ /  __/",
                r"   |        \___/ /_|    |_|  \__,_/\___\",
                r"    \_______\____\___/   \__\  _,_/",
            ]
        ),
        // Ubuntu: three friends around a ring.
        logo!(
            "ubuntu",
            UBUNTU,
            [
                r"             .-/+oOo+-.",
                r"           `:.o+oOo-o.:`",
                r"          `:.+oOoO-.+:`",
                r"         `:.+.:+oOo-.`",
                r"      `-::-.:+oOoO/",
                r"           .-/+oOo+-.",
                r"         `:/+.:+oOoO.",
                r"      `:.+.:+oOo-.:`",
            ]
        ),
        // Linux Mint: the leaf.
        logo!(
            "mint",
            GREEN,
            [
                r"            __",
                r"         .-'  `-.",
                r"       .'    /\  `.",
                r"      /    /  \    \",
                r"     |    /    \    |",
                r"     |   |      |   |",
                r"      \   \    /   /",
                r"       `.   `--'   /",
                r"         `-.____.-'",
            ]
        ),
        // NixOS: six-armed snowflake.
        logo!(
            "nixos",
            SILVER,
            [
                r"        /\      /\      /\",
                r"       /  \    /  \    /  \",
                r"      /    \  /    \  /    \",
                r"      \    /  \    /  \    /",
                r"       \  /    \  /    \  /",
                r"        \/      \/      \/",
            ]
        ),
        // Manjaro: three bars.
        logo!(
            "manjaro",
            GREEN,
            [
                r"      .:.    .:.    .:.",
                r"     (###)  (###)  (###)",
                r"     (###)  (###)  (###)",
                r"     (###)  (###)  (###)",
                r"      :::    :::    :::",
            ]
        ),
        // Gentoo: the G.
        logo!(
            "gentoo",
            PURPLE,
            [
                r"     #######",
                r"    ##     ##",
                r"    ##",
                r"    ##  ######",
                r"    ##     ##",
                r"    ##     ##",
                r"     #######",
            ]
        ),
        // openSUSE: Geeko.
        logo!(
            "opensuse",
            GREEN,
            [
                r"        ___",
                r"    ___/   \___",
                r"   /  __     __  \",
                r"  |  /  \   /  \  |",
                r"  |  \      _   /  |",
                r"   \  \___/ \__/  /",
                r"    \__       __/",
                r"       \_____/",
            ]
        ),
        // Alpine: the snow-capped peak.
        logo!(
            "alpine",
            TEAL,
            [
                r"              ##",
                r"             /##\",
                r"            /#  #\",
                r"           /#    #\",
                r"          /#      #\",
                r"         /#        #\",
                r"        /##########\",
            ]
        ),
        // Void: the V inside its ring.
        logo!(
            "void",
            SILVER,
            [
                r"     .-~~~~~~~-.",
                r"    /           \",
                r"   |  \       /  |",
                r"   |   \     /   |",
                r"   |    \   /    |",
                r"   |     \ /     |",
                r"   |      V      |",
                r"   |     / \     |",
                r"   |    /   \    |",
                r"   |   /     \   |",
                r"   |  /       \  |",
                r"    \           /",
                r"     '-._____.-'",
            ]
        ),
        // Fallback for anything unrecognised.
        logo!(
            "generic",
            SILVER,
            [
                r"     _________________",
                r"    |                 |",
                r"    |  ------------  |",
                r"    |  |          |  |",
                r"    |  |          |  |",
                r"    |  ------------  |",
                r"    |_________________|",
                r"          |     |",
                r"       ___|_____|___",
            ]
        ),
    ]
}

/// Looks up an artwork by `/etc/os-release` ID.
pub fn by_id(id: &str) -> Option<&'static Logo> {
    all().iter().find(|l| l.id == id)
}

/// Logos that must be mirror-symmetric about their vertical axis.
///
/// Listed by ID rather than inferred: a hand-drawn letterform like the Gentoo
/// `G` is asymmetric on purpose, and the Arch peak is a left-leaning shape.
#[cfg(test)]
pub const SYMMETRIC: &[&str] = &["nixos", "void", "alpine", "generic"];

#[cfg(test)]
mod tests {
    use super::*;

    /// Mirror-reflects a glyph about the vertical axis.
    ///
    /// ASCII has no true reflection for slashes and brackets, so they are
    /// mapped explicitly; every other glyph used here is its own mirror.
    /// Without this a symmetric shape like `/\` would test as lopsided.
    fn mirror_char(c: char) -> char {
        match c {
            '/' => '\\',
            '\\' => '/',
            '(' => ')',
            ')' => '(',
            '[' => ']',
            ']' => '[',
            '{' => '}',
            '}' => '{',
            '<' => '>',
            '>' => '<',
            _ => c,
        }
    }

    /// Returns the line mirrored about its vertical axis, trimmed.
    fn mirrored(line: &str) -> String {
        line.trim().chars().rev().map(mirror_char).collect()
    }

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
    fn no_line_has_trailing_whitespace() {
        // Trailing spaces break the visible-width maths on padded columns.
        for l in all() {
            for (i, line) in l.lines.iter().enumerate() {
                assert_eq!(
                    line.trim_end(),
                    *line,
                    "{}: line {} has trailing whitespace",
                    l.id,
                    i + 1
                );
            }
        }
    }

    /// Renders every logo side by side for visual review.
    ///
    ///     cargo test logo::arts::tests::preview -- --nocapture
    ///
    /// Art quality is a judgement call that no assertion can make; this is how the
    /// set gets eyeballed before a release.
    #[test]
    #[ignore = "visual review, run explicitly"]
    fn preview_all_logos() {
        let logos = all();
        let rows = logos.iter().map(|l| l.lines.len()).max().unwrap_or(0);
        let label_w = logos.iter().map(|l| l.id.len() + 2).max().unwrap_or(0);
        let col_w = logos.iter().map(|l| l.width()).max().unwrap_or(0);

        for row in 0..rows {
            let mut line = String::new();
            for l in logos {
                let art = l.lines.get(row).copied().unwrap_or("");
                let pad = col_w.saturating_sub(crate::render::width::visible_len(art));
                let label = if row == 0 {
                    format!("{:width$}", l.id, width = label_w)
                } else {
                    " ".repeat(label_w)
                };
                line.push_str(&label);
                line.push_str(art);
                line.extend(std::iter::repeat_n(' ', pad + 3));
            }
            println!("{}", line.trim_end());
        }
    }

    #[test]
    fn declared_symmetric_logos_are_symmetric() {
        for id in SYMMETRIC {
            let logo = by_id(id).unwrap_or_else(|| panic!("{id} not found"));
            for (i, line) in logo.lines.iter().enumerate() {
                assert_eq!(
                    mirrored(line),
                    line.trim(),
                    "{id}: line {} is not mirror-symmetric: {line:?}",
                    i + 1
                );
            }
        }
    }

    #[test]
    fn symmetric_ids_all_exist() {
        for id in SYMMETRIC {
            assert!(by_id(id).is_some(), "{id} listed in SYMMETRIC but missing");
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
