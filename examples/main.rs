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
