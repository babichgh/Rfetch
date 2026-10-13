//! Memory usage.

use crate::fetch::Context;
use crate::fetch::util::bytes;

/// e.g. `15.4 GiB / 31.3 GiB (49%)`.
pub fn ram(ctx: &Context) -> Option<String> {
    let system = crate::fetch::system(ctx);
    let total = system.total_memory();
    if total == 0 {
        return None;
    }

    let used = system.used_memory();
    let pct = (used as f64 / total as f64) * 100.0;

    Some(format!(
        "{} / {} ({:.0}%)",
        bytes(used),
        bytes(total),
        pct.clamp(0.0, 100.0)
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fetch::Module;

    #[test]
    fn formats_used_over_total() {
        let ctx = Context::collect(&[Module::Ram]);
        let v = ram(&ctx).expect("ram should resolve");
        let parts: Vec<&str> = v.split(" / ").collect();
        assert_eq!(parts.len(), 2, "expected `used / total`, got {v}");
        assert_ne!(parts[0], parts[1]);
        assert!(v.contains('%'));
    }

    #[test]
    fn percentage_is_within_bounds() {
        let ctx = Context::collect(&[Module::Ram]);
        let v = ram(&ctx).unwrap();
        let pct: f64 = v
            .rsplit('(')
            .next()
            .unwrap()
            .trim_end_matches("%)")
            .parse()
            .unwrap();
        assert!((0.0..=100.0).contains(&pct), "got {pct}");
    }
}
