/// Standard interface that all system information modules must implement.
/// This trait ensures uniform execution and data retrieval across the entire tool.
pub trait Module: Send + Sync {
    /// Returns the user-facing name of the module (e.g., "OS", "CPU", "Memory")
    fn name(&self) -> &str;

    /// Executes the detection logic and returns a list of formatted fields to display.
    /// Example: [("Model", "AMD Ryzen 9"), ("Cores", "12")]
    fn fetch(&self) -> Vec<(String, String)>;
}
