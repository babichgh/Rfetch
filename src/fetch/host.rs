//! Host identity: `user@hostname`.

use crate::fetch::Context;
use sysinfo::System;

pub fn host(_ctx: &Context) -> Option<String> {
    let hostname = System::host_name().filter(|h| !h.trim().is_empty());
    let user = std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .ok()
        .filter(|u| !u.trim().is_empty());

    match (user, hostname) {
        (Some(u), Some(h)) => Some(format!("{u}@{h}")),
        (None, Some(h)) => Some(h),
        (Some(u), None) => Some(u),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_something_on_this_host() {
        let ctx = Context::collect(&[crate::fetch::Module::Host]);
        let v = host(&ctx);
        assert!(v.is_some(), "host should always resolve");
        assert!(!v.unwrap().is_empty());
    }

    #[test]
    fn value_has_no_leading_or_trailing_space() {
        let ctx = Context::collect(&[crate::fetch::Module::Host]);
        let v = host(&ctx).unwrap();
        assert_eq!(v.trim(), v);
    }
}
