//! Disk usage for the primary mount points.

use crate::fetch::Context;

const GIB: f64 = 1024.0 * 1024.0 * 1024.0;

fn format_usage(total: u64, available: u64, filesystem: &str) -> String {
    let free = available.min(total);
    let used = total - free;
    let pct = (used as f64 / total as f64) * 100.0;
    format!(
        "{:.1} GiB / {:.1} GiB ({pct:.0}%) - {filesystem}",
        used as f64 / GIB,
        free as f64 / GIB
    )
}

/// Reports the root filesystem, or the largest writable, non-removable disk
/// when the root mount is unavailable.
///
/// Returns `None` when there is nothing worth reporting (e.g. a container with
/// no mounted volumes), so the line is simply omitted.
pub fn disk(ctx: &Context) -> Option<String> {
    let disks = crate::fetch::disks(ctx);

    let disk = disks
        .list()
        .iter()
        .filter(|d| !d.is_removable() && !d.is_read_only())
        .filter(|d| d.total_space() > 0)
        .max_by_key(|d| {
            (
                d.mount_point() == std::path::Path::new("/"),
                d.total_space(),
            )
        })?;

    Some(format_usage(
        disk.total_space(),
        disk.available_space(),
        &disk.file_system().to_string_lossy(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fetch::Module;

    #[test]
    fn renders_or_omits_cleanly() {
        let ctx = Context::collect(&[Module::Disk]);
        if let Some(v) = disk(&ctx) {
            assert!(!v.contains("/: "), "mount point leaked: {v}");
            assert!(v.contains('%'), "got {v}");
            assert!(v.contains("GiB / "), "got {v}");
        }
    }

    #[test]
    fn outputs_only_one_filesystem() {
        let ctx = Context::collect(&[Module::Disk]);
        if let Some(v) = disk(&ctx) {
            assert!(!v.contains(", "), "got {v}");
        }
    }

    #[test]
    fn formats_used_and_free_with_filesystem() {
        assert_eq!(
            format_usage(100 * 1024 * 1024 * 1024, 40 * 1024 * 1024 * 1024, "btrfs"),
            "60.0 GiB / 40.0 GiB (60%) - btrfs"
        );
        assert_eq!(
            format_usage(2 * 1024 * 1024 * 1024, 1024 * 1024 * 1024, "exfat"),
            "1.0 GiB / 1.0 GiB (50%) - exfat"
        );
    }

    #[test]
    fn available_space_cannot_exceed_total() {
        assert_eq!(
            format_usage(1024, 2048, "ext4"),
            "0.0 GiB / 0.0 GiB (0%) - ext4"
        );
    }
}
