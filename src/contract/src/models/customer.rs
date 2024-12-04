use super::shipment::ShipmentId;
use candid::Principal;
use serde::{Deserialize, Serialize};

pub type CustomerId = Principal;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Customer {
    id: CustomerId,
    name: String,
    shipments_sent: u32,
    shipments: Vec<ShipmentId>,
}

impl Customer {
    pub fn new(id: CustomerId, name: String) -> Self {
        Self {
            id,
            name,
            shipments: vec![],
            shipments_sent: 0,
        }
    }

    pub fn add_shipment(&mut self, shipment_id: ShipmentId) {
        self.shipments.push(shipment_id);
    }

    pub fn finalize_shipment(&mut self, shipment_id: ShipmentId) {
        self.shipments.retain(|&x| x != shipment_id);
        self.shipments_sent += 1;
    }

    pub fn id(&self) -> Principal {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn shipments(&self) -> &[ShipmentId] {
        &self.shipments
    }
}
