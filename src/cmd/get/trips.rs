use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;
use strum::VariantNames;

use crate::app::trip::TripService;
use crate::io::Format;
use crate::{external, io};

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(short, long, parse(from_os_str), default_value = "hibou.db")]
    database: PathBuf,
    #[clap(short, long, default_value = "csv", possible_values(Format::VARIANTS))]
    format: Format,
}

pub fn run(op: &Opts) -> Result<()> {
    let gtfs = external::gtfsdb::init(&op.database)?;
    let trips = TripService::new(gtfs).fetch()?;
    io::write(&trips, &op.format)?;
    Ok(())
}
