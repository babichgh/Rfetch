//! Disk usage for the primary mount points.

use crate::fetch::Context;
use crate::fetch::util::bytes;

/// Renders the largest non-removable, writable filesystems first.
///
/// Returns `None` when there is nothing worth reporting (e.g. a container with
/// no mounted volumes), so the line is simply omitted.
pub fn disk(ctx: &Context) -> Option<String> {
    let disks = crate::fetch::disks(ctx);

    let mut rows: Vec<(u64, u64, String)> = disks
        .list()
        .iter()
        .filter(|d| !d.is_removable() && !d.is_read_only())
        .filter_map(|d| {
            let total = d.total_space();
            if total == 0 {
                return None;
            }
            let used = total.saturating_sub(d.available_space());
            let pct = (used as f64 / total as f64) * 100.0;
            let label = d.mount_point().display().to_string();
            let row = format!(
                "{label}: {} / {} ({:.0}%)",
                bytes(used),
                bytes(total),
                pct.clamp(0.0, 100.0)
            );
            Some((total, used, row))
        })
        .collect();

    if rows.is_empty() {
        return None;
    }

    rows.sort_by(|a, b| {
        b.0.cmp(&a.0)
            .then_with(|| b.1.cmp(&a.1))
            .then_with(|| a.2.cmp(&b.2))
    });

    // `/` and `/home` on the same volume are two mount points of one disk.
    // Keep the shallowest path and drop the rest.
    let mut seen: Vec<(u64, u64)> = Vec::new();
    let mut lines = Vec::new();
    for (total, used, row) in rows {
        let key = (total, used);
        if seen.contains(&key) {
            continue;
        }
        seen.push(key);
        lines.push(row);
        if lines.len() == 2 {
            break;
        }
    }

    Some(lines.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fetch::Module;

    #[test]
    fn renders_or_omits_cleanly() {
        let ctx = Context::collect(&[Module::Disk]);
        if let Some(v) = disk(&ctx) {
            assert!(v.contains('/'), "got {v}");
            assert!(v.contains('%'), "got {v}");
        }
    }

    #[test]
    fn limits_output_to_two_entries() {
        let ctx = Context::collect(&[Module::Disk]);
        if let Some(v) = disk(&ctx) {
            assert!(v.split(", ").count() <= 2, "got {v}");
        }
    }
}
