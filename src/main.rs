use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde::export::Formatter;
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize)]
struct DailyData {
    date: String,
    confirmed: u64,
    deaths: u64,
    recovered: u64,
}

impl Display for DailyData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{date:<dw$} \t confirmed: {confirmed:>width$} \t deaths: {deaths:>width$} \t recovered: {recovered:>width$}",
            date = self.date,
            dw = 20,
            confirmed = self.confirmed,
            width = 9,
            deaths = self.deaths,
            recovered = self.recovered,
        )
    }
}

#[derive(Debug, StructOpt)]
struct Opts {
    /// The country to show records for
    #[structopt(short, long)]
    country: String,

    /// The last N days worth of records to show
    #[structopt(short, long, default_value = "10")]
    num_results: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt: Opts = Opts::from_args();

    let source_url = "https://pomber.github.io/covid19/timeseries.json";
    let response: HashMap<String, Vec<DailyData>> = reqwest::blocking::get(source_url)?
        .json()?;

    let country_data = response.get(&opt.country);

    if let None = country_data {
        println!("The following countries are available:");
        response.keys().for_each(|k| {
            println!("{}", k);
        });
    } else {
        let country_data: Vec<&DailyData> = country_data
            .unwrap()
            .iter()
            .filter(|datum| {
                datum.confirmed > 0
            })
            .collect();

        match opt.num_results {
            0 => {
                country_data
                    .iter()
                    .for_each(|d| {
                        println!("{}", d);
                    })
            }
            n => {
                country_data
                    .iter()
                    .skip(country_data.len() - n)
                    .for_each(|d| {
                        println!("{}", d);
                    })
            }
        }
    }

    Ok(())
}
