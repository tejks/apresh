use super::{base::ActorBase, Actor, ActorRole};
use crate::models::shipment::ShipmentId;
use actor_derive::IsActor;
use candid::Principal;
use serde::{Deserialize, Serialize};

pub type CarrierId = Principal;

#[derive(Deserialize, Serialize, Debug, Clone, IsActor)]
pub struct Carrier {
    base: ActorBase,
}

impl Carrier {
    pub fn new(id: CarrierId, name: &str) -> Self {
        Self { base: ActorBase::new(id, name.to_string()) }
    }
}
