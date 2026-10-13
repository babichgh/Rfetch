pub mod arts;

pub use arts::Logo;

/// Picks the artwork for a system.
///
/// Falls back to `generic` when the OS ID has no dedicated artwork, so output
/// is never missing its left column.
pub fn detect(os_id: Option<&str>) -> &'static Logo {
    os_id
        .and_then(arts::by_id)
        .or_else(|| arts::by_id("generic"))
        .expect("generic logo is always present")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_by_id() {
        assert_eq!(detect(Some("fedora")).id, "fedora");
    }

    #[test]
    fn falls_back_to_generic_for_unknown_id() {
        assert_eq!(detect(Some("haiku-os")).id, "generic");
    }

    #[test]
    fn falls_back_to_generic_when_id_missing() {
        assert_eq!(detect(None).id, "generic");
    }

    #[test]
    fn detection_never_panics_on_empty_id() {
        assert_eq!(detect(Some("")).id, "generic");
    }
}
