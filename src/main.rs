use crate::model::Ficha;
use rand::prelude::*;
use std::error::Error;

mod model;

fn main() -> Result<(), Box<dyn Error>> {
    const BATCH_SIZE: usize = 500;
    const BATCH_COUNT: usize = 250_000 / BATCH_SIZE;

    let mut rng = rand::thread_rng();
    let mut writer = csv::Writer::from_path("data.csv")?;

    for _ in 0..BATCH_COUNT {
        (0..BATCH_SIZE)
            .map(|_| rng.gen())
            .for_each(|f: Ficha| writer.serialize(f).unwrap());
        writer.flush()?;
    }

    Ok(())
}
