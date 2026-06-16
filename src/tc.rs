use std::process::Command;
use thiserror::Error;

const HELPER_PATH: &str = "/usr/lib/badwire/badwire-tc-helper";

#[derive(Error, Debug)]
pub enum TcError {
    #[error("Failed to execute tc: {0}")]
    Io(#[from] std::io::Error),
    #[error("tc command failed with exit code {0}: {1}")]
    Command(i32, String),
    #[error("Root privileges required. Please authenticate.")]
    AuthRequired,
}

pub fn run_tc(args: &[&str]) -> Result<String, TcError> {
    let is_root = nix::unistd::geteuid().is_root();
    if is_root {
        run_tc_direct(args)
    } else {
        run_tc_via_pkexec(args)
    }
}

fn run_tc_direct(args: &[&str]) -> Result<String, TcError> {
    let output = Command::new("tc")
        .args(args)
        .output()
        .map_err(TcError::Io)?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let code = output.status.code().unwrap_or(-1);
        Err(TcError::Command(code, stderr))
    }
}

fn run_tc_via_pkexec(args: &[&str]) -> Result<String, TcError> {
    let mut cmd = Command::new("pkexec");
    cmd.arg(HELPER_PATH);
    for arg in args {
        cmd.arg(arg);
    }

    let output = cmd.output().map_err(TcError::Io)?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let code = output.status.code().unwrap_or(-1);

        if code == 126 || code == 127 {
            Err(TcError::AuthRequired)
        } else {
            Err(TcError::Command(code, stderr))
        }
    }
}

pub fn is_valid_nonneg_number(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() {
        return true;
    }
    s.parse::<f64>().map_or(false, |v| v >= 0.0)
}

pub fn is_valid_percentage(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() {
        return false;
    }
    s.parse::<f64>().map_or(false, |v| (0.0..=100.0).contains(&v))
}

pub fn build_netem_args(
    delay: &str,
    jitter: Option<&str>,
    loss: Option<&str>,
    loss_corr: Option<&str>,
    reorder: Option<&str>,
    reorder_corr: Option<&str>,
    corrupt: Option<&str>,
    corrupt_corr: Option<&str>,
    duplicate: Option<&str>,
    duplicate_corr: Option<&str>,
) -> Result<Vec<String>, String> {
    let mut args: Vec<String> = Vec::new();

    if delay.trim().is_empty() {
        return Err("Delay must not be empty".into());
    }
    if !is_valid_nonneg_number(delay) {
        return Err(format!("Invalid delay: {}", delay));
    }
    args.push("delay".into());
    args.push(format!("{}ms", delay));

    if let Some(j) = jitter {
        if !is_valid_nonneg_number(j) {
            return Err(format!("Invalid jitter: {}", j));
        }
        args.push(format!("{}ms", j));
    }

    if let Some(l) = loss {
        add_percent_param(&mut args, "loss", l, loss_corr, true)?;
    }
    if let Some(r) = reorder {
        add_percent_param(&mut args, "reorder", r, reorder_corr, false)?;
    }
    if let Some(c) = corrupt {
        add_percent_param(&mut args, "corrupt", c, corrupt_corr, false)?;
    }
    if let Some(d) = duplicate {
        add_percent_param(&mut args, "duplicate", d, duplicate_corr, false)?;
    }

    Ok(args)
}

fn add_percent_param(
    args: &mut Vec<String>,
    name: &str,
    value: &str,
    correlation: Option<&str>,
    insert_random: bool,
) -> Result<(), String> {
    if !is_valid_percentage(value) {
        return Err(format!("Invalid {}: {} (must be 0-100)", name, value));
    }

    args.push(name.to_string());
    if insert_random {
        args.push("random".to_string());
    }
    args.push(value.to_string());

    if let Some(corr) = correlation {
        if !is_valid_percentage(corr) {
            return Err(format!(
                "Invalid {} correlation: {} (must be 0-100)",
                name, corr
            ));
        }
        args.push(corr.to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_nonneg_number() {
        assert!(is_valid_nonneg_number("0"));
        assert!(is_valid_nonneg_number("0.0"));
        assert!(is_valid_nonneg_number("100.5"));
        assert!(is_valid_nonneg_number(""));
        assert!(!is_valid_nonneg_number("-1"));
        assert!(!is_valid_nonneg_number("abc"));
    }

    #[test]
    fn test_is_valid_percentage() {
        assert!(is_valid_percentage("0"));
        assert!(is_valid_percentage("100"));
        assert!(is_valid_percentage("50.5"));
        assert!(!is_valid_percentage("101"));
        assert!(!is_valid_percentage("-1"));
        assert!(!is_valid_percentage("abc"));
    }

    #[test]
    fn test_build_netem_args_delay_only() {
        let args =
            build_netem_args("100", None, None, None, None, None, None, None, None, None).unwrap();
        assert_eq!(args, vec!["delay", "100ms"]);
    }

    #[test]
    fn test_build_netem_args_delay_with_jitter() {
        let args = build_netem_args(
            "200",
            Some("50"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(args, vec!["delay", "200ms", "50ms"]);
    }

    #[test]
    fn test_build_netem_args_loss_only() {
        let args = build_netem_args(
            "0",
            None,
            Some("10"),
            Some("20"),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(args, vec!["delay", "0ms", "loss", "random", "10", "20"]);
    }

    #[test]
    fn test_build_netem_args_full() {
        let args = build_netem_args(
            "150",
            Some("30"),
            Some("5"),
            Some("25"),
            Some("2"),
            Some("50"),
            Some("1"),
            Some("75"),
            Some("3"),
            Some("80"),
        )
        .unwrap();
        assert_eq!(
            args,
            vec![
                "delay",
                "150ms",
                "30ms",
                "loss",
                "random",
                "5",
                "25",
                "reorder",
                "2",
                "50",
                "corrupt",
                "1",
                "75",
                "duplicate",
                "3",
                "80",
            ]
        );
    }

    #[test]
    fn test_build_netem_args_invalid_delay() {
        let err =
            build_netem_args("", None, None, None, None, None, None, None, None, None).unwrap_err();
        assert!(err.contains("Delay must not be empty"));
    }

    #[test]
    fn test_build_netem_args_invalid_jitter() {
        let err = build_netem_args(
            "10",
            Some("abc"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap_err();
        assert!(err.contains("Invalid jitter"));
    }

    #[test]
    fn test_build_netem_args_invalid_loss() {
        let err = build_netem_args(
            "10",
            None,
            Some("notanumber"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap_err();
        assert!(err.contains("Invalid loss"));
    }

    #[test]
    fn test_build_netem_args_percentage_out_of_range() {
        let err = build_netem_args(
            "10",
            None,
            Some("150"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap_err();
        assert!(err.contains("Invalid loss") && err.contains("0-100"));
    }
}
