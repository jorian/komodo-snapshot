#![feature(drain_filter)]

use komodo_rpc::{Auth, Client, RpcApi};
use std::default::Default;

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

    /// Takes a Snapshot and builds the resulting Snapshot struct.
    /// Here is where the threshold is applied and excluded addresses are removed, if any.
    pub fn take(&self) -> Snapshot {
        let client = Client::new(&self.chain, Auth::ConfigFile).unwrap();
        // todo handle any error, after adding error handling
        let mut snapshot = match self.number_of_addresses {
            Some(max) => client.get_snapshot(Some(format!("{}", max))),
            None => client.get_snapshot(None),
        }
        .unwrap();

        if snapshot.addresses.is_empty() {
            // return Err(ErrorKind::EmptySnapshot.into());
            panic!("Empty snapshot!")
        }

        if self.threshold > 0.0 {
            snapshot.addresses = snapshot
                .addresses
                .drain_filter(|saddress| saddress.amount > self.threshold)
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

        Snapshot {
            chain: self.chain.clone(),
            addresses,
            amount_in_snapshot: snapshot.total,
        }
    }
}

impl Default for SnapshotBuilder<'_> {
    fn default() -> Self {
        SnapshotBuilder {
            chain: String::from("KMD"),
            threshold: 0.0,
            number_of_addresses: None,
            excluded_addresses: None,
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
    use crate::Snapshot;
    #[test]
    fn it_works() {
        let snapshot = Snapshot::builder()
            .on_chain("KMD")
            .include_top_n_addresses(10)
            .take();
    }
}
