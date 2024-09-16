# naturalearthdata

A thin wrapper to download the data from [Natural Earth Data](https://www.naturalearthdata.com/).

At present, the crate offers basic functionality, mainly focusing on downloading data from the Natural Earth website.

- It utilizes [GDAL Virtual File Systems](https://gdal.org/en/latest/user/virtual_file_systems.html) to handle the downloading process.
- It uses the [GeoArrow](https://crates.io/crates/geoarrow) to convert the data into [RecordBatchReader](https://arrow.apache.org/docs/dev/r/reference/RecordBatchReader.html) format.

### Data that can be downloaded

[Cultural Vectors](https://www.naturalearthdata.com/downloads/10m-cultural-vectors/) and [Physical Vectors](https://www.naturalearthdata.com/downloads/10m-physical-vectors/) are available for download at three scales:

- Large (1:10m)
- Medium (1:50m)
- Small (1:110m)

## Usage

```rust
use naturalearthdata::Scale;
use naturalearthdata::{download::ne_download, CulturalType};

fn main() {
    // Download the Admin 0 Countries at large scale (1:10m)
    // https://www.naturalearthdata.com/downloads/10m-cultural-vectors/
    let mut reader = ne_download(CulturalType::Admin0Countries, Scale::Large).unwrap();

    // Read the schema
    let schema = reader.schema().unwrap();
    println!("{:?}", schema);

    // Read the forth column of the table
    for batch_result in reader.take().unwrap() {
        match batch_result {
            Ok(batch) => {
                let value = batch.column(4);
                println!("{:?}", value);
            }
            Err(e) => eprintln!("Error reading batch: {:?}", e),
        }
    }
}

```

To run the example in the `examples` directory, use the following command:

```bash
cargo run --example=main
```
