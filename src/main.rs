use arrow_csv::reader::Format;
use std::io::{BufReader, Seek, SeekFrom, Write};
use tempfile::tempfile;


fn main() {
    ;
    let mut csv_out = tempfile().unwrap();
    writeln!(csv_out, "3.106e+04, asdf").unwrap();
    csv_out.seek(SeekFrom::Start(0)).unwrap();
    let format = Format::default()
        .with_delimiter(',' as u8)
        .with_header(false);


    // Infer the schema of the CSV file
    let schema_inference_reader = BufReader::new(csv_out);

    let (schema, _) = format.infer_schema(
        schema_inference_reader,
        None,
    ).unwrap();
    println!("{:?}", schema);
    assert!(schema.fields.get(0).unwrap().data_type().is_floating())
}