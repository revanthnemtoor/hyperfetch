pub trait Module: Send + Sync {
    /// Returns the name of the module (e.g., "OS", "CPU", "Memory")
    fn name(&self) -> &'static str;

    /// Fetches the data and returns a list of formatted fields to display
    fn fetch(&self) -> Vec<(String, String)>;
}
