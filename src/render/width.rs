//! ANSI-aware string measurement.
//!
//! Layout maths must never count escape sequences as visible characters,
//! otherwise coloured keys shift the value column to the right.

/// Returns the number of visible characters in `s`, ignoring ANSI escape
/// sequences (CSI `ESC [ ... final-byte`) and OSC sequences.
pub fn visible_len(s: &str) -> usize {
    let mut len = 0;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            skip_escape(&mut chars);
            continue;
        }
        len += 1;
    }
    len
}

/// Consumes the remainder of an escape sequence.
///
/// Handles both CSI (`ESC [ ... final`) and OSC (`ESC ] ... BEL | ESC \`),
/// plus two-byte `ESC <char>` forms. Unknown sequences degrade gracefully:
/// at most a single character is swallowed so measurement stays finite.
fn skip_escape<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>) {
    let Some(kind) = chars.peek().copied() else {
        return;
    };
    match kind {
        '[' => {
            chars.next();
            // CSI ends on a byte in the range 0x40..=0x7E.
            while let Some(&c) = chars.peek() {
                chars.next();
                if ('\x40'..='\x7e').contains(&c) {
                    break;
                }
            }
        }
        ']' => {
            chars.next();
            let mut prev = '\0';
            for c in chars.by_ref() {
                if prev == '\x1b' && c == '\\' {
                    break;
                }
                if c == '\x07' {
                    break;
                }
                prev = c;
            }
        }
        _ => {
            chars.next();
        }
    }
}

/// Right-pads `s` with spaces to `width` visible columns.
/// Never truncates: callers decide how to handle overflow.
pub fn pad_right(s: &str, width: usize) -> String {
    let len = visible_len(s);
    let mut out = String::with_capacity(s.len() + width.saturating_sub(len));
    out.push_str(s);
    out.extend(std::iter::repeat_n(' ', width.saturating_sub(len)));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_string_length() {
        assert_eq!(visible_len("hello"), 5);
        assert_eq!(visible_len(""), 0);
    }

    #[test]
    fn ignores_csi_sequences() {
        assert_eq!(visible_len("\x1b[1;31mabc\x1b[0m"), 3);
        assert_eq!(visible_len("\x1b[38;2;60;110;180mx\x1b[0m"), 1);
    }

    #[test]
    fn ignores_osc_sequences() {
        assert_eq!(visible_len("\x1b]0;title\x07ok"), 2);
        assert_eq!(visible_len("\x1b]8;;http://x\x1b\\link"), 4);
    }

    #[test]
    fn truncated_escape_does_not_hang() {
        // Unterminated CSI: must still terminate and count the rest.
        assert_eq!(visible_len("\x1b[1;3"), 0);
        assert_eq!(visible_len("a\x1b"), 1);
    }

    #[test]
    fn pad_right_uses_visible_width() {
        let colored = "\x1b[1mab\x1b[0m";
        assert_eq!(visible_len(&pad_right(colored, 5)), 5);
        assert_eq!(visible_len(&pad_right("abc", 2)), 3);
    }

    #[test]
    fn multibyte_chars_count_as_one() {
        assert_eq!(visible_len("привет"), 6);
        assert_eq!(visible_len("日本"), 2);
    }
}
