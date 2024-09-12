use crate::types::scales::Scale;

/// A trait for types that can be downloaded.
///
/// Types implementing this trait should provide a method to get the URL
/// from which they can be downloaded.
pub trait Downloadable {
    /// Returns the URL from which the type can be downloaded.
    ///
    /// # Arguments
    ///
    /// * `scale` - The scale of the data to be downloaded.
    ///
    /// # Returns
    ///
    /// A string slice that holds the URL.
    fn get_url(&self, scale: &Scale) -> String;
}
