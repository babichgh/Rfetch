//! Time since boot.

use crate::fetch::Context;
use crate::fetch::util::duration;
use sysinfo::System;

pub fn uptime(_ctx: &Context) -> Option<String> {
    let secs = System::uptime();
    if secs == 0 {
        // Distinguish "just booted" from "unknown"; both render fine but the
        // former is the honest reading on a freshly started machine.
        return Some("0m".to_string());
    }
    Some(duration(secs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uptime_looks_like_a_duration() {
        let ctx = Context::collect(&[crate::fetch::Module::Uptime]);
        let v = uptime(&ctx).unwrap();
        assert!(v.ends_with('m') || v.ends_with('h'), "got {v}");
    }

    #[test]
    fn never_renders_empty() {
        let ctx = Context::collect(&[crate::fetch::Module::Uptime]);
        assert!(!uptime(&ctx).unwrap().is_empty());
    }
}
