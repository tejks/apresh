use std::{collections::HashMap, ops::{Deref, DerefMut}};
use crate::models::{shipment, shipment_id, carrier, customer::CustomerId};

type ShipmentsStore = HashMap<shipment_id::ShipmentIdInner, shipment::Shipment>;

#[derive(Default)]
pub struct Shipments(ShipmentsStore);

impl Deref for Shipments {
    type Target = ShipmentsStore;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Shipments {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Shipments {
    pub fn get_all_pending(&self) -> Vec<shipment::Shipment> {
        self.values()
            .filter(|shipment| *shipment.status() == shipment::ShipmentStatus::Pending)
            .cloned()
            .collect()
    }

    pub fn get_all_for_customer(&self, customer_id: &CustomerId) -> Vec<shipment::Shipment> {
        self.values()
            .filter(|shipment| shipment.customer_id() == *customer_id)
            .cloned()
            .collect()
    }

    pub fn get_all_for_shipper(&self, carrier_id: &carrier::CarrierId) -> Vec<shipment::Shipment> {
        self.values()
            .filter(|shipment| shipment.carrier_id() == Some(*carrier_id))
            .cloned()
            .collect()
    }
}
