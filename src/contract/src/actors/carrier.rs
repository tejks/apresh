use super::{base::ActorBase, Actor, ActorRole};
use crate::models::shipment::ShipmentId;
use candid::Principal;
use serde::{Deserialize, Serialize};

pub type CarrierId = Principal;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Carrier {
    base: ActorBase,
}

impl Carrier {
    pub fn new(id: CarrierId, name: &str) -> Self {
        Self { base: ActorBase::new(id, name.to_string()) }
    }
}

impl Actor for Carrier {
    fn id(&self) -> Principal {
        self.base.id()
    }

    fn role(&self) -> ActorRole {
        ActorRole::Carrier
    }

    fn name(&self) -> &str {
        &self.base.name()
    }

    fn active_shipments(&self) -> &[ShipmentId] {
        &self.base.active_shipments()
    }

    fn shipments_history(&self) -> &[ShipmentId] {
        &self.base.shipments_history()
    }

    fn add_shipment(&mut self, shipment_id: ShipmentId) {
        self.base.add_shipment(shipment_id);
    }

    fn archive_shipment(&mut self, shipment_id: ShipmentId) {
        self.base.archive_shipment(shipment_id);
    }
}
