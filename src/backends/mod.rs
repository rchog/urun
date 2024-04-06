use egui::load::Result;

pub mod by_env;

#[derive(Clone, Debug)]
pub struct CompletionEntry {
    pub filename: String,
    pub full_path: String,
    pub path: String,
}

#[allow(unused)]
pub enum UError {
    Unknown,
    Stderr(String),
}

pub trait CompletionBackend {
    /// [generate(&mut self, input: &str)] should update the list of completions
    /// on call. Will be called willy-nilly, potentially multiple times a second
    /// so context- and cost-appropriate debounce should be implemented here.
    /// Sorting might fit best here as well.
    fn generate(&mut self, _: &str);
    /// [all(&self)] should return all the currently stored completion cand-
    /// idates. Called a *lot*, so should be reasonably cheap.
    fn all(&self) -> &[CompletionEntry];
    /// [n(&self, n: usize)] should return the first [n] results or as many as
    /// possible if [n] is higher than the found completions. Not currently
    /// used, but probably will be in future.
    fn n(&self, _: usize) -> &[CompletionEntry];
    /// The number of elements all() would return
    fn len(&self) -> usize;
    /// Run/execute the indicated item in backend-dependant way. A "normal" sys-
    /// tem application launcher should exit the urun process here as well.
    /// The return type is subject to change as I think of fancier ways to use
    /// this a. la Telescope
    fn execute(&self, task: &CompletionEntry) -> Result<String, UError>;
    /// Run/execute an arbitrary command, e.g. a shell command with arguments
    fn command(&self, command: &str) -> Result<String, UError>;
}

// nonsense generator for working on UI before getting a proper backend working.
#[allow(unused)]
pub mod dev {
    use super::CompletionBackend;
    use super::CompletionEntry;
    use super::UError;
    use std::time::{SystemTime, UNIX_EPOCH};
    const NONSENSE: [&'static str; 30] = [
        "suck",
        "tacit",
        "buzz",
        "meddle",
        "cabbage",
        "throat",
        "yell",
        "brush",
        "stuff",
        "fall",
        "pretty",
        "rabid",
        "flight",
        "round",
        "lavish",
        "typical",
        "hard",
        "acidic",
        "screeching",
        "absorbed",
        "squeak",
        "flawless",
        "color",
        "lunch",
        "memorize",
        "abject",
        "history",
        "messy",
        "stale",
        "hideous",
    ];

    pub struct Completions {
        pub completions: Vec<CompletionEntry>,
        time: SystemTime,
    }

    impl Completions {
        pub fn new() -> Completions {
            Self {
                completions: Vec::new(),
                time: SystemTime::now(),
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
        fn generate(&mut self, _input: &str) {
            if self.debounce() {
                return;
            }

            self.completions.clear();
            for _i in 0..10 {
                let index = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .subsec_nanos()
                    % 30;
                self.completions.push(CompletionEntry {
                    filename: NONSENSE[index as usize].to_string(),
                    path: NONSENSE[index as usize].to_string(),
                    full_path: NONSENSE[index as usize].to_string(),
                });
            }
            self.time = SystemTime::now();
        }
        fn all(&self) -> &[CompletionEntry] {
            self.completions.as_slice()
        }
        fn n(&self, n: usize) -> &[CompletionEntry] {
            &self.completions.as_slice()[0..n - 1]
        }
        fn len(&self) -> usize {
            self.completions.len()
        }

        fn execute(&self, task: &CompletionEntry) -> Result<String, UError> {
            println!("Execute task tried: {}", task.full_path);

            return Err(UError::Unknown);
        }
        fn command(&self, cmd: &str) -> Result<String, UError> {
            println!("Command tried: {}", cmd);

            return Err(UError::Unknown);
        }
    }

    #[test]
    fn dev_completion_gen_test() {
        let mut cm = Completions::new();
        cm.generate("hello");
        println!("dev::Completions test");
        println!("=====================");
        for c in &cm.completions {
            println!("Completion found: {}", c.filename);
        }
        assert_eq!(&cm.completions.len(), &10)
    }
}
