use crate::models::shipment::ShipmentId;
use candid::Principal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ActorBase {
    id: Principal,
    name: String,
    active_shipments: Vec<ShipmentId>,
    shipments_history: Vec<ShipmentId>,
}

impl ActorBase {
    pub fn new(id: Principal, name: String) -> Self {
        Self {
            id,
            name,
            active_shipments: vec![],
            shipments_history: vec![],
        }
    }

    pub fn id(&self) -> Principal {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn active_shipments(&self) -> &[ShipmentId] {
        &self.active_shipments
    }

    pub fn shipments_history(&self) -> &[ShipmentId] {
        &self.shipments_history
    }

    pub fn add_shipment(&mut self, shipment_id: ShipmentId) {
        self.active_shipments.push(shipment_id);
    }

    pub fn archive_shipment(&mut self, shipment_id: ShipmentId) {
        self.active_shipments.retain(|&x| x != shipment_id);
        self.shipments_history.push(shipment_id);
    }

    pub fn shipments_completed(&self) -> u32 {
        self.shipments_history.len() as u32
    }
}
