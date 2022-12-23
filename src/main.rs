use std::path::PathBuf;

use clap::{arg, Parser};
use datafusion::prelude::*;


// Uses snmalloc as the allocator to reduce thread synchronization around allocations
// https://arrow.apache.org/datafusion/user-guide/library.html#optimized-configuration
#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;


#[derive(Parser)]
struct Args {
    #[arg(help="Input CSV file")]
    infile: PathBuf,
    #[arg(help="Output Parquet path")]
    outfile: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Args::parse();
    let path_str = args.infile.to_str().unwrap();
    let out_str = args.outfile.to_str().unwrap();
    let ctx = SessionContext::new();
    let df = ctx.read_csv(path_str, CsvReadOptions::new()).await.expect("Failed to read data file");
    df.write_parquet(out_str, None).await.expect("Failed to write output file");

    Ok(())
}
