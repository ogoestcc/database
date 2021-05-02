use csv_lib::Reader;
use serde::Deserialize;

mod alerts;
mod users;

pub struct CSVDatabase;

impl CSVDatabase {
    pub fn get_data<D: for<'de> Deserialize<'de>, F: FnMut(&D) -> bool>(
        &self,
        file: &str,
        mut filter: F,
    ) -> Vec<D> {
        log::info!("Get users infos from {}", file);

        let mut rdr = Reader::from_path(file).unwrap();
        let iter = rdr.deserialize();

        iter.filter_map(|u| {
            match u {
                Ok(d) => if filter(&d) { Some(d) } else { None },
                Err(err) => {
                    log::error!("{:?}", err);
                    None
                }
            }
        })
        .collect()
    }
}
