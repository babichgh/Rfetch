//! Kernel name and release.

use crate::fetch::Context;
use sysinfo::System;

/// Kernel name as reported by `uname(2)`.
///
/// `System::name()` is not used on Linux: sysinfo resolves it from
/// `os-release` there, which yields the distribution name ("Fedora Linux")
/// rather than the kernel name.
#[cfg(target_os = "linux")]
fn kernel_name() -> Option<String> {
    Some("Linux".to_string())
}

#[cfg(not(target_os = "linux"))]
fn kernel_name() -> Option<String> {
    System::name()
}

/// e.g. `Linux 6.17.4-300.fc44.x86_64`.
pub fn kernel(_ctx: &Context) -> Option<String> {
    let release = System::kernel_version().filter(|k| !k.trim().is_empty());
    match (kernel_name().filter(|n| !n.trim().is_empty()), release) {
        (Some(name), Some(release)) => Some(format!("{name} {release}")),
        (None, Some(release)) => Some(release),
        (Some(name), None) => Some(name),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fetch::Module;

    #[test]
    fn includes_a_version_when_available() {
        let ctx = Context::collect(&[Module::Kernel]);
        let v = kernel(&ctx).expect("kernel should resolve");
        assert!(!v.is_empty());
        assert!(!v.contains('\n'));
    }
}
