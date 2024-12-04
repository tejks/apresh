use crate::actors::{carrier::CarrierId, customer::CustomerId};
use crate::impl_deref;
use crate::models::shipment::{Shipment, ShipmentStatus};
use std::collections::HashMap;

type ShipmentsStore = HashMap<u64, Shipment>;

#[derive(Default)]
pub struct Shipments(ShipmentsStore);

impl_deref!(Shipments, ShipmentsStore);

impl Shipments {
    pub fn list_pending(&self) -> Vec<&Shipment> {
        self.values()
            .filter(|shipment| *shipment.status() == ShipmentStatus::Pending)
            .collect()
    }

    pub fn list_for_customer(&self, customer_id: &CustomerId) -> Vec<&Shipment> {
        self.values()
            .filter(|shipment| shipment.customer_id() == *customer_id)
            .collect()
    }

    pub fn list_for_shipper(&self, carrier_id: &CarrierId) -> Vec<&Shipment> {
        self.values()
            .filter(|shipment| shipment.carrier_id() == Some(*carrier_id))
            .collect()
    }
}
