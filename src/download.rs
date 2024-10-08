use crate::types::scales::Scale;
use crate::types::traits::Downloadable;
use gdal::Dataset;
use geoarrow::error::GeoArrowError;
use geoarrow::io::gdal::read_gdal;
use geoarrow::io::RecordBatchReader;
use geoarrow::table::Table;
/// Downloads and reads a dataset from a given downloadable type and scale.
///
/// # Arguments
///
/// * `downloadable_type` - The type that implements the `Downloadable` trait, representing the dataset to be downloaded.
/// * `scale` - The scale of the data to be downloaded.
///
/// # Returns
///
/// A `Result` containing a `RecordBatchReader` if successful, or a `GeoArrowError` if an error occurs.
///
/// # Errors
///
/// This function will return an error if the URL cannot be opened or if there is an issue reading the dataset.
pub fn ne_download<T: Downloadable>(
    downloadable_type: T,
    scale: Scale,
) -> Result<RecordBatchReader, GeoArrowError> {
    let url = downloadable_type.get_url(&scale);

    let dataset = Dataset::open(url)?;

    // Get the layer (assuming there is only one layer)
    let mut layer = dataset.layer(0)?;

    // Using None will get all rows, specifying a number will create many batch with of that size
    let reader = read_gdal(&mut layer, None)?;
    let table: Table = reader.try_into()?;

    Ok(table.into())
}
