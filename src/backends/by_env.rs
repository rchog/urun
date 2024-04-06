use super::CompletionBackend;
use super::CompletionEntry;
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path;
use std::time::SystemTime;
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
#[derive(Debug)]
pub struct Completions {
    pub completions: Vec<CompletionEntry>,
    time: SystemTime,
    dirs: Vec<path::PathBuf>,
    candidates: Option<Vec<CompletionEntry>>,
}

impl Completions {
    pub fn new() -> Completions {
        Self {
            completions: Vec::new(),
            time: SystemTime::now(),
            dirs: match env::var("PATH") {
                Ok(val) => {
                    let mut val = val
                        .split(":")
                        .map(|dir| path::PathBuf::from(dir.to_string()))
                        .collect::<Vec<path::PathBuf>>();
                    val.sort_unstable();
                    val.dedup();
                    val
                }
                Err(_) => vec![],
            },
            candidates: None,
        }
    }

    // probably won't need any debounce functionality until the generation is
    // eventually split off into a separate thread to cut down on input jank
    /* #[cfg(test)]
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
    } */

    // This seems bad
    fn populate_candidates(&self) -> Vec<CompletionEntry> {
        let mut entries: Vec<CompletionEntry> = vec![];
        for dir in &self.dirs {
            // so gross
            let files = fs::read_dir(dir).map_or_else(
                |_| vec![],
                |rd| {
                    rd.filter_map(|file| {
                        file.map_or(None, |file| {
                            file.metadata().map_or(None, |md| {
                                if md.is_file() && (md.permissions().mode() & 0o111) != 0 {
                                    Some(CompletionEntry {
                                        filename: file.file_name().to_string_lossy().to_string(),
                                        path: dir.to_string_lossy().to_string(),
                                        full_path: file.path().to_string_lossy().to_string(),
                                    })
                                } else {
                                    None
                                }
                            })
                        })
                    })
                    .collect()
                },
            );

            entries.append(&mut files.to_vec());
        }

        entries.sort_by(|a, b| String::cmp(&a.filename, &b.filename));

        return entries;
    }
}

impl CompletionBackend for Completions {
    fn generate(&mut self, input: &str) {
        /* if self.debounce() {
            return;
        } */
        if input.len() < 1 {
            return;
        }

        if let None = self.candidates {
            self.candidates = Some(self.populate_candidates());
        }
        if let Some(candidates) = &self.candidates {
            self.completions = candidates
                .iter()
                .filter_map(|item| {
                    if item.filename.starts_with(input) {
                        Some(item.clone())
                    } else {
                        None
                    }
                })
                .collect();
        }
        // self.completions.clear();

        self.time = SystemTime::now();
    }

    fn all(&self) -> &[CompletionEntry] {
        self.completions.as_slice()
    }
    fn n(&self, n: usize) -> &[CompletionEntry] {
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
    fn env_vars_can_be_read() {
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

    #[test]
    fn basic_tools_found() {
        let mut be = Completions::new();
        be.generate("pri");
        assert!(be
            .all()
            .iter()
            .find(|&v| { v.filename == "printf".to_string() })
            .is_some());
    }

    #[test]
    fn completions_respect_prefix() {
        let mut be = Completions::new();
        be.generate("ech");
        assert!(be
            .all()
            .iter()
            .find(|&v| { v.filename == "printf".to_string() })
            .is_none());
    }

    #[test]
    fn nonsense_not_found() {
        let mut be = Completions::new();
        be.generate("lit");
        assert!(be
            .all()
            .iter()
            .find(|&v| { v.filename == "little red riding hood".to_string() })
            .is_none());
    }

    #[test]
    fn empty_input_doesnt_generate() {
        let mut be = Completions::new();
        be.generate("");
        assert!(be
            .all()
            .iter()
            .find(|&v| { v.filename == "ls".to_string() })
            .is_none());
    }
}
