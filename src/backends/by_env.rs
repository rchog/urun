use super::CompletionBackend;
use std::env;
use std::path;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
/// Most basic completion backend.
/// Plan: - parse env var PATH
///       - list of all contents in found directories
///       - collect all results corresponding to an executable file
///      (- cache these)
///       - keep any files whose prefix matches input -- POSSIBLY also any where
///         levenshtein distance is 1 and ones with different case?
///       - sort alphabetically, then by length diff from input:
///     > net
///     : netcap
///     : netcat
///     : netctl
///     : netstat
///     : networkctl
///     : netctl-auto
pub struct Completions {
    pub completions: Vec<String>,
    time: SystemTime,
    dirs: Vec<path::PathBuf>,
    candidates: Option<Vec<String>>,
}

impl Completions {
    pub fn new() -> Completions {
        Self {
            completions: Vec::new(),
            time: SystemTime::now(),
            dirs: match env::var("PATH") {
                Ok(val) => val
                    .split(":")
                    .map(|dir| path::PathBuf::from(dir.to_string()))
                    .collect(),
                Err(_) => vec![],
            },
            candidates: None,
        }
    }

    #[cfg(test)]
    fn debounce(&mut self) -> bool {
        return false;
    }

    #[cfg(not(test))]
    /// Determine whether or not to abort early
    fn debounce(&mut self) -> bool {
        let now = SystemTime::now();
        if let Ok(diff) = now.duration_since(self.time) {
            if diff.as_secs() < 1 {
                return true;
            } else {
                return false;
            }
        } else {
            self.time = now;
            return true;
        }
    }
}

impl CompletionBackend for Completions {
    fn generate(&mut self, input: &str) {
        if self.debounce() {
            return;
        }

        // just keep the whole mess in memory for now
        if let None = self.candidates {
            // TODO: fill self.candidates
        }
        if let Some(candidates) = &self.candidates {
            self.completions = candidates
                .iter()
                .filter_map(|item| -> Option<String> {
                    if item.starts_with(input) {
                        Some(item.clone())
                    } else {
                        None
                    }
                })
                .collect();
        }
        self.completions.clear();

        self.time = SystemTime::now();
    }

    fn all(&self) -> &[String] {
        self.completions.as_slice()
    }
    fn n(&self, n: usize) -> &[String] {
        if n > self.completions.len() {
            self.completions.as_slice()
        } else {
            &self.completions.as_slice()[0..n - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn env_vars_can_be_read_test() {
        let be = Completions::new();
        assert!(be.dirs.len() > 0);
        assert!(be
            .dirs
            .iter()
            // This whole thing doesn't AFAIK make any sense on Windows so this
            // should be universally true. (Macos???)
            .find(|&p| { p.to_str() == Some("/usr/bin") })
            .is_some());
    }
}
