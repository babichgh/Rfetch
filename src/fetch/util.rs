//! Formatting helpers shared by the modules.

/// Formats a byte count with binary units, e.g. `15.4 GiB`.
pub fn bytes(n: u64) -> String {
    const UNITS: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
    let mut value = n as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{n} B")
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

/// Formats seconds as a compact duration, e.g. `3d 4h 12m`.
///
/// Always returns at least `0m` so the value is never empty.
pub fn duration(total_secs: u64) -> String {
    let days = total_secs / 86_400;
    let hours = (total_secs % 86_400) / 3_600;
    let mins = (total_secs % 3_600) / 60;

    let mut out = String::new();
    if days > 0 {
        out.push_str(&format!("{days}d "));
    }
    if days > 0 || hours > 0 {
        out.push_str(&format!("{hours}h "));
    }
    out.push_str(&format!("{mins}m"));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_scales_to_units() {
        assert_eq!(bytes(0), "0 B");
        assert_eq!(bytes(512), "512 B");
        assert_eq!(bytes(1024), "1.0 KiB");
        assert_eq!(bytes(1024 * 1024), "1.0 MiB");
        assert_eq!(bytes(3 * 1024 * 1024 * 1024), "3.0 GiB");
    }

    #[test]
    fn bytes_avoids_overflow_for_huge_values() {
        assert_eq!(bytes(u64::MAX), "16.0 EiB");
    }

    #[test]
    fn duration_formats_days_and_hours() {
        assert_eq!(duration(0), "0m");
        assert_eq!(duration(60), "1m");
        assert_eq!(duration(3_600), "1h 0m");
        assert_eq!(duration(86_400 + 3_600 + 120), "1d 1h 2m");
    }
}
