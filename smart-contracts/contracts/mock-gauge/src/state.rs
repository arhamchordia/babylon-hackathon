use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Empty, StdError, Storage, Uint128};
use cw_storage_plus::{Item, Map};

use mars_owner::Owner;

use crate::contract::GaugeResult;

pub const OWNER: Owner = Owner::new("owner");

// Destinations should function as a set, but we currently don't need to save data, hence and Empty value
pub const DESTINATIONS: Map<Destination, Empty> = Map::new("destinations");

pub type Destination = String;

pub const WEIGHTS: Weights = Weights::new("weights", "total_weight");

/// Weights mocks the actual logic of the values of the gauges weights.
/// In a non mocked version, this could be the amount of btc used to secure a chain
/// or the amount of staked babylon etc.
pub struct Weights<'a> {
    weights: Map<'a, &'a str, Weight>,
    total_weight: Item<'a, Uint128>,
}

#[cw_serde]
pub struct Weight {
    pub destination_id: String,
    pub amount: Uint128,
}

impl<'a> Weights<'a> {
    pub const fn new(map_namespace: &'a str, item_namespace: &'a str) -> Self {
        Self {
            weights: Map::new(map_namespace),
            total_weight: Item::new(item_namespace),
        }
    }

    pub fn add(&self, store: &mut dyn Storage, weight: Weight) -> GaugeResult<()> {
        let old_dest_weight = self
            .weights
            .may_load(store, &weight.destination_id)?
            .unwrap_or(Weight {
                destination_id: weight.destination_id.clone(),
                amount: Uint128::default(),
            });
        self.weights.save(store, &weight.destination_id, &weight)?;

        let old = self.total_weight.may_load(store)?.unwrap_or_default();
        self.total_weight
            .save(store, &(old + weight.amount - old_dest_weight.amount))?;

        Ok(())
    }

    pub fn get(&self, store: &dyn Storage, destination_id: &str) -> Result<Weight, StdError> {
        self.weights.load(store, destination_id)
    }

    pub fn total(&self, store: &dyn Storage) -> Result<Uint128, StdError> {
        self.total_weight.load(store)
    }
}
