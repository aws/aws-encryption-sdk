//! format numbers into human readable abbreviations

const POWER_LETTERS: &str = "0KMGTPEZYRQ";

#[must_use]
/// Format f64 into something short and readable, like 42K or 1.5M
///
/// Returned string is at most 5 characters, including optional `-` sign
/// 0-999 are themselves
/// The other formats are 1K, 1.2K, 12K, 123K
/// Where K == 1024, and M, G, T, P, E, Z, Y, R, Q each increase by a factor of 1024
/// The number represented by the string is never less than the original number
/// e.g 1000 = "1K", 1024="1K", 1025="1.1K"
pub fn format_power2f(num: f64) -> String {
    if num < 0.0 {
        let mut ret = "-".to_string();
        ret.push_str(&format_power2_inner(-num));
        ret
    } else {
        format_power2_inner(num)
    }
}

#[must_use]
/// as `format_power2f`
pub fn format_power2(num: u64) -> String {
    #[allow(clippy::cast_precision_loss, reason = "precision loss ok")]
    format_power2f(num as f64)
}

#[must_use]
fn format_power2_inner(num: f64) -> String {
    if num < 1000.0 {
        return format!("{num:.0}");
    }
    let mut curr_exp = 1f64;
    let mut exp = POWER_LETTERS.chars();

    loop {
        let e: char = exp.next().unwrap();
        if num <= (999.0 * curr_exp) {
            if num >= (9.9 * curr_exp) {
                return format!("{:.0}{e}", (num / curr_exp).ceil());
            }
            if num > curr_exp {
                return format!("{:.1}{e}", (num * 10.0 / curr_exp).ceil() / 10.0);
            }
            return format!("1{e}");
        }
        if e == 'Q' {
            return format!("{:.0}Q", num / curr_exp);
        }
        curr_exp *= 1024.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format() {
        assert!(format_power2(0) == "0");
        assert!(format_power2(1) == "1");
        assert!(format_power2f(-1f64) == "-1");
        assert!(format_power2(999) == "999");
        assert!(format_power2(1000) == "1K");
        assert!(format_power2(1024) == "1K");
        assert!(format_power2(1025) == "1.1K");
        assert!(format_power2(10137) == "9.9K");
        assert!(format_power2(10138) == "10K");
        assert!(format_power2(97 * 1024) == "97K");
        assert!(format_power2(97 * 1024 + 1) == "98K");
        assert!(format_power2(999 * 1024) == "999K");
        assert!(format_power2(999 * 1024 + 1) == "1M");
    }
}
