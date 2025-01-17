use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;

use crate::app::gtfs::GtfsService;
use crate::external;

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(parse(from_os_str))]
    gtfs_dir: PathBuf,
    #[clap(short, long, parse(from_os_str), default_value = "hibou.db")]
    database: PathBuf,
}

pub fn run(op: &Opts) -> Result<()> {
    let gtfs_csv = external::gtfscsv::init(op.gtfs_dir.clone())?;
    let gtfs_db = external::gtfsdb::init(&op.database)?;

    let mut service = GtfsService::new(gtfs_db, gtfs_csv);

    service.drop_tables()?;
    service.create_tables()?;
    service.insert_tables(&op.gtfs_dir)?;

    Ok(())
}
