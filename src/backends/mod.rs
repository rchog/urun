use egui::load::Result;

pub mod launcher;

#[derive(Clone, Debug)]
pub struct CompletionEntry {
    pub title: String,
    pub subtitle: String,
    pub action: String,
}

pub enum Exec {
    Exit(i32),
    Continue,
    // ...
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
    fn execute(&self, task: &CompletionEntry) -> Exec;

    /// Run/execute an arbitrary command, e.g. a shell command with arguments
    fn command(&self, command: &str) -> Exec;
}
