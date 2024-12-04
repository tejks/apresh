use candid::Principal;
use serde::{Deserialize, Serialize};

use crate::models::shipment::ShipmentId;

use super::{base::ActorBase, Actor, ActorRole};

pub type CustomerId = Principal;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Customer {
    base: ActorBase,
}

impl Customer {
    pub fn new(id: CustomerId, name: String) -> Self {
        Self { base: ActorBase::new(id, name) }
    }
}

impl Actor for Customer {
    
    fn id(&self) -> Principal {
        self.base.id()
    }

    fn role(&self) -> ActorRole {
        ActorRole::Customer
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn active_shipments(&self) -> &[ShipmentId] {
        &self.base.active_shipments()
    }

    fn shipments_history(&self) -> &[ShipmentId] {
        &self.base.shipments_history()
    }

    fn archive_shipment(&mut self, shipment_id: ShipmentId) {
        self.base.archive_shipment(shipment_id);
    }

    fn add_shipment(&mut self, shipment_id: ShipmentId) {
        self.base.add_shipment(shipment_id);
    }
}
