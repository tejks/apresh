use super::{customer::Customer, shipment_id::ShipmentIdInner};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, CandidType)]
pub enum SizeCategory {
    Envelope,
    Parcel {
        max_width: u64,
        max_height: u64,
        max_depth: u64,
    },
}

#[derive(Deserialize, Serialize, Debug, Clone, CandidType)]
pub struct ShipmentLocation {
    street: String,
    lat: f64,
    lng: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone, CandidType)]
pub struct ShipmentInfo {
    value: u64,
    price: u64,
    source: ShipmentLocation,
    destination: ShipmentLocation,
    size_category: SizeCategory,
}

#[derive(Deserialize, Serialize, Debug, Clone, CandidType, PartialEq, Eq)]
pub enum ShipmentStatus {
    Pending,
    InTransit,
    Delivered,
    Cancelled,
}

#[derive(Deserialize, Serialize, Debug, Clone, CandidType)]
pub struct Shipment {
    id: ShipmentIdInner,
    info: ShipmentInfo,
    status: ShipmentStatus,
    carrier: Option<Principal>,
    customer: Principal,
    created_at: u64,
}

impl Shipment {
    pub fn create(creator: &mut Customer, id: ShipmentIdInner, info: ShipmentInfo) -> Self {
        let created_at = ic_cdk::api::time();

        creator.add_shipment(id);

        let shipment = Self {
            id,
            info,
            status: ShipmentStatus::Pending,
            carrier: None,
            customer: creator.id(),
            created_at,
        };

        shipment
    }

    pub fn status(&self) -> &ShipmentStatus {
        &self.status
    }

    pub fn customer_id(&self) -> Principal {
        self.customer
    }
}
