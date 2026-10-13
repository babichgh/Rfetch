//! OS name and version, read from `/etc/os-release`.

use crate::fetch::Context;

/// Parsed subset of `os-release(5)`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OsRelease {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pretty_name: Option<String>,
    pub version_id: Option<String>,
    pub variant: Option<String>,
}

impl OsRelease {
    /// Reads `OS_RELEASE_PATH`, returning `None` when it is absent.
    pub fn detect() -> Option<Self> {
        let raw = std::fs::read_to_string(OS_RELEASE_PATH).ok()?;
        Some(parse(&raw))
    }

    /// Distribution version, e.g. `44`.
    pub fn version(&self) -> Option<&str> {
        self.version_id.as_deref()
    }
}

#[cfg(target_os = "linux")]
const OS_RELEASE_PATH: &str = "/etc/os-release";

#[cfg(target_os = "macos")]
const OS_RELEASE_PATH: &str = "/System/Library/CoreServices/SystemVersion.plist";

/// Parses `KEY=value` lines, ignoring comments and blank lines.
///
/// Malformed lines (no `=`) are skipped rather than aborting the parse.
pub fn parse(raw: &str) -> OsRelease {
    let mut out = OsRelease::default();
    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let value = unquote(value.trim());
        match key.trim() {
            "ID" => out.id = Some(value),
            "NAME" => out.name = Some(value),
            "PRETTY_NAME" => out.pretty_name = Some(value),
            "VERSION_ID" => out.version_id = Some(value),
            "VARIANT" => out.variant = Some(value),
            _ => {}
        }
    }
    out
}

/// Strips one layer of matching single or double quotes.
fn unquote(v: &str) -> String {
    let bytes = v.as_bytes();
    if bytes.len() >= 2
        && ((bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[bytes.len() - 1] == b'\''))
    {
        return v[1..v.len() - 1].to_string();
    }
    v.to_string()
}

/// Builds the `OS` value, e.g. `Fedora Linux 44 (KDE Plasma Desktop Edition)`.
pub fn os(ctx: &Context) -> Option<String> {
    if let Some(rel) = crate::fetch::os_release(ctx) {
        if let Some(pretty) = &rel.pretty_name {
            return Some(pretty.clone());
        }
        let mut s = rel.name.clone().or_else(|| rel.id.clone())?;
        if let Some(v) = rel.version() {
            s.push(' ');
            s.push_str(v);
        }
        return Some(s);
    }
    // Fall back to whatever the OS itself reports.
    let name = sysinfo::System::name()?;
    Some(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
NAME="Fedora Linux"
VERSION="44 (KDE Plasma Desktop Edition)"
ID=fedora
VERSION_ID=44
PRETTY_NAME="Fedora Linux 44 (KDE Plasma Desktop Edition)"
VARIANT="KDE Plasma Desktop Edition"
# a comment

ANSICOLOR="0;38;2;60;110;180"
"#;

    #[test]
    fn parses_standard_fields() {
        let r = parse(SAMPLE);
        assert_eq!(r.id.as_deref(), Some("fedora"));
        assert_eq!(r.name.as_deref(), Some("Fedora Linux"));
        assert_eq!(r.version(), Some("44"));
        assert_eq!(
            r.pretty_name.as_deref(),
            Some("Fedora Linux 44 (KDE Plasma Desktop Edition)")
        );
        assert_eq!(r.variant.as_deref(), Some("KDE Plasma Desktop Edition"));
    }

    #[test]
    fn skips_comments_and_blank_lines() {
        let r = parse("# only a comment\n\n   \nID=arch\n");
        assert_eq!(r.id.as_deref(), Some("arch"));
        assert!(r.name.is_none());
    }

    #[test]
    fn skips_malformed_lines() {
        let r = parse("garbage_without_equals\nID=void\n");
        assert_eq!(r.id.as_deref(), Some("void"));
    }

    #[test]
    fn empty_input_yields_default() {
        assert_eq!(parse(""), OsRelease::default());
    }

    #[test]
    fn strips_double_quotes() {
        assert_eq!(unquote("\"x\""), "x");
        assert_eq!(unquote("'x'"), "x");
        assert_eq!(unquote("x"), "x");
        assert_eq!(unquote("\"x"), "\"x");
        assert_eq!(unquote("\""), "\"");
    }

    #[test]
    fn single_quote_literal_is_preserved() {
        let r = parse("VARIANT='KDE Plasma'\n");
        assert_eq!(r.variant.as_deref(), Some("KDE Plasma"));
    }

    #[test]
    fn value_with_equals_inside_is_kept() {
        let r = parse("PRETTY_NAME=\"a=b\"\n");
        assert_eq!(r.pretty_name.as_deref(), Some("a=b"));
    }
}
