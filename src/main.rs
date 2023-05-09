use crate::model::Ficha;
use rand::prelude::*;
use std::error::Error;

mod model;

fn main() -> Result<(), Box<dyn Error>> {
    const BATCH_SIZE: usize = 500;
    const BATCH_COUNT: usize = 250_000 / BATCH_SIZE;

    let mut rng = rand::thread_rng();
    let mut writer = csv::Writer::from_path("data.csv")?;

    for i in 0..BATCH_COUNT {
        (1..=BATCH_SIZE)
            .map(|j| (j, rng.gen::<Ficha>()))
            .for_each(|(j, f)| {
                let nombres = f.nombres.clone();
                let ith_number = i * BATCH_SIZE + j;
                if let Err(e) = writer.serialize(f) {
                    println!("({}) Error serializing {} \n{:?}", ith_number, nombres, e);
                }
            });
        writer.flush()?;
    }

    Ok(())
}
