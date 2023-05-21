use crate::model::Ficha;
use rand::prelude::*;
use std::error::Error;

mod model;
mod random_utils;

fn main() -> Result<(), Box<dyn Error>> {
    const BATCH_SIZE: usize = 500;

    println!("{}", "Ingrese la cantidad de fichas a generar: ");

    let stdin = std::io::stdin();
    let mut item_count = String::with_capacity(4);
    stdin.read_line(&mut item_count)?;

    let item_count = item_count
        .trim()
        .parse::<usize>()
        .expect("Item count is not a number!");
    let batch_count: usize = item_count / BATCH_SIZE;

    let mut rng = rand::thread_rng();
    let mut writer = csv::Writer::from_path("data.csv")?;

    for i in 0..batch_count {
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
