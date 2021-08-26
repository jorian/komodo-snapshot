use crate::error::SnapshotError;
use komodo_rpc::{Auth, Client, RpcApi};
use std::default::Default;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub chain: String,
    pub addresses: Vec<Address>,
    pub amount_in_snapshot: f64,
}

impl Snapshot {
    pub fn builder<'a>() -> SnapshotBuilder<'a> {
        Default::default()
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotBuilder<'a> {
    chain: String,
    threshold: f64,
    number_of_addresses: Option<u32>,
    excluded_addresses: Option<Vec<&'a str>>,
    csv_location: Option<PathBuf>,
}

impl<'a> SnapshotBuilder<'a> {
    pub fn on_chain(&mut self, chain: &str) -> &mut Self {
        self.chain = String::from(chain);
        self
    }

    pub fn using_threshold(&mut self, threshold: f64) -> &mut Self {
        self.threshold = threshold;
        self
    }

    pub fn include_top_n_addresses(&mut self, n: u32) -> &mut Self {
        self.number_of_addresses = Some(n);
        self
    }

    //
    pub fn exclude_addresses(&mut self, addresses: Vec<&'a str>) -> &mut Self {
        self.excluded_addresses = Some(addresses);
        self
    }

    pub fn store_in_csv(&mut self, path: &str) -> &mut Self {
        self.csv_location = Some(PathBuf::from(path));
        self
    }

    /// Takes a Snapshot and builds the resulting Snapshot struct.
    /// Here is where the threshold is applied and excluded addresses are removed, if any.
    pub fn take(&self) -> Result<Snapshot, SnapshotError> {
        let client = Client::new(&self.chain, Auth::ConfigFile).unwrap();
        // todo handle any error, after adding error handling
        let mut snapshot = match self.number_of_addresses {
            Some(max) => client.get_snapshot(Some(format!("{}", max))),
            None => client.get_snapshot(None),
        }?;

        if snapshot.addresses.is_empty() {
            // return Err(ErrorKind::EmptySnapshot.into());
            panic!("Empty snapshot!")
        }

        dbg!(&snapshot.addresses.len());

        if self.threshold > 0.0 {
            snapshot.addresses = snapshot
                .addresses
                .drain_filter(|saddress| saddress.amount >= self.threshold)
                .collect::<Vec<_>>();
        }

        // first, remove any predefined excluded addresses from the snapshotted address vec
        // then, map each address and its corresponding amount to an Address struct.
        let addresses = snapshot
            .addresses
            .iter()
            .filter(|address| {
                let excluded_addresses = self.excluded_addresses.clone();
                match excluded_addresses {
                    Some(vec) => !vec.contains(&address.addr.as_str()),
                    None => return true,
                }
            })
            .map(|address| Address {
                addr: address.addr.clone(),
                amount: address.amount,
            })
            .collect::<Vec<_>>();

        if let Some(path) = &self.csv_location {
            let mut writer = csv::Writer::from_path(&path)?;
            writer.write_record(&["address", "amount"]);
            for address in &addresses {
                writer.write_record(&[&address.addr, &address.amount.to_string()]);
            }
        }

        Ok(Snapshot {
            chain: self.chain.clone(),
            addresses,
            amount_in_snapshot: snapshot.total,
        })
    }
}

impl Default for SnapshotBuilder<'_> {
    fn default() -> Self {
        SnapshotBuilder {
            chain: String::from("KMD"),
            threshold: 0.0,
            number_of_addresses: None,
            excluded_addresses: None,
            csv_location: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Address {
    pub addr: String,
    pub amount: f64,
}

#[cfg(test)]
mod tests {
    use crate::snapshot::Snapshot;
    #[test]
    fn it_works() {
        let snapshot = Snapshot::builder()
            .on_chain("KMD")
            .using_threshold(1.0)
            .store_in_csv("./output.csv")
            .take();

        dbg!(snapshot.unwrap().addresses.len());
    }
}
