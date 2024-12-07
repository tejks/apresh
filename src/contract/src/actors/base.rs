use crate::models::shipment::ShipmentId;
use candid::Principal;
use serde::{Deserialize, Serialize};

/// Base struct for all actors.
/// Every actor should have these properties.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ActorBase {
    /// The principal of the actor. Every actor has a unique principal.
    id: Principal,
    /// The name of the actor. Every actor should have a name.
    name: String,
    /// The list of active shipments, active is shipment which has been created but not yet delivered or cancelled.
    active_shipments: Vec<ShipmentId>,
    /// The list of completed shipments, completed is shipment which has been delivered.
    /// But it could also be shipment which has been cancelled.
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

    pub fn get_active_shipments(&self) -> &[ShipmentId] {
        &self.active_shipments
    }

    pub fn get_shipments_history(&self) -> &[ShipmentId] {
        &self.shipments_history
    }

    pub fn add_shipment(&mut self, shipment_id: ShipmentId) {
        self.active_shipments.push(shipment_id);
    }

    // TODO: check if shipment is in active_shipments
    pub fn archive_shipment(&mut self, shipment_id: ShipmentId) {
        self.active_shipments.retain(|&x| x != shipment_id);
        self.shipments_history.push(shipment_id);
    }
}
