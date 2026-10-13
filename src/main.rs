mod cli;
mod fetch;
mod logo;
mod render;

use clap::Parser;
use cli::Cli;
use fetch::Context;
use render::color::Style;

/// Escapes a string for JSON output.
fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn main() {
    let cli = Cli::parse();

    let style = Style {
        colors: cli.colors_enabled(),
        ..Style::default()
    };

    let modules = cli.selected_modules();
    let ctx = Context::collect(&modules);
    let entries = ctx.entries(&modules);

    // Scripted output: no logo, no colour, one field per line.
    if cli.pipe {
        let sep = cli.layout().separator;
        for e in &entries {
            println!("{}{sep} {}", e.key, e.value);
        }
        return;
    }

    if cli.json {
        let fields = entries
            .iter()
            .map(|e| {
                format!(
                    "\"{}\":\"{}\"",
                    json_escape(&e.key.to_lowercase().replace(' ', "_")),
                    json_escape(&e.value)
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        println!("{{{fields}}}");
        return;
    }

    let logo = if cli.logo_off {
        None
    } else {
        let id = cli.logo.as_deref().or_else(|| ctx.os_id());
        match id {
            Some("off") => None,
            Some(id) => Some(logo::detect(Some(id))),
            None => Some(logo::detect(None)),
        }
    };

    print!(
        "{}",
        cli.layout()
            .render(logo, &entries, &style, cli.term_width())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_quotes_and_backslashes() {
        assert_eq!(json_escape(r#"a"b\c"#), r#"a\"b\\c"#);
    }

    #[test]
    fn escapes_control_characters() {
        assert_eq!(json_escape("a\nb"), "a\\nb");
        assert_eq!(json_escape("a\tb"), "a\\tb");
        assert_eq!(json_escape("\u{1}"), "\\u0001");
    }

    #[test]
    fn leaves_ordinary_text_untouched() {
        assert_eq!(json_escape("Fedora Linux 44"), "Fedora Linux 44");
    }

    #[test]
    fn escapes_cyrillic_unchanged() {
        assert_eq!(json_escape("привет"), "привет");
    }
}
