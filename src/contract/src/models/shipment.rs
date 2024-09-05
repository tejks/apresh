use super::{carrier::Carrier, customer::Customer, shipment_id::ShipmentIdInner};
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

impl ShipmentLocation {
    pub fn new(street: String, lat: f64, lng: f64) -> Self {
        Self { street, lat, lng }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, CandidType)]
pub struct ShipmentInfo {
    value: u64,
    price: u64,
    source: ShipmentLocation,
    destination: ShipmentLocation,
    size_category: SizeCategory,
}

impl ShipmentInfo {
    pub fn price(&self) -> u64 {
        self.price
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn new(
        value: u64,
        price: u64,
        source: ShipmentLocation,
        destination: ShipmentLocation,
        size_category: SizeCategory,
    ) -> Self {
        Self {
            value,
            price,
            source,
            destination,
            size_category,
        }
    }
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
    name: String,
    info: ShipmentInfo,
    status: ShipmentStatus,
    carrier: Option<Principal>,
    customer: Principal,
    created_at: u64,
}

impl Shipment {
    pub fn create(
        creator: &mut Customer,
        id: ShipmentIdInner,
        name: String,
        info: ShipmentInfo,
    ) -> Self {
        let created_at = ic_cdk::api::time();

        creator.add_shipment(id);

        let shipment = Self {
            id,
            info,
            name,
            status: ShipmentStatus::Pending,
            carrier: None,
            customer: creator.id(),
            created_at,
        };

        shipment
    }

    pub fn buy(&mut self, carrier: &mut Carrier) -> anyhow::Result<()> {
        if self.status != ShipmentStatus::Pending {
            return Err(anyhow::anyhow!("shipment is not pending"));
        }

        self.carrier = Some(carrier.id());
        self.status = ShipmentStatus::InTransit;
        carrier.add_shipment(self.id());

        Ok(())
    }

    pub fn status(&self) -> &ShipmentStatus {
        &self.status
    }

    pub fn customer_id(&self) -> Principal {
        self.customer
    }

    pub fn carrier_id(&self) -> Option<Principal> {
        self.carrier
    }

    pub fn id(&self) -> ShipmentIdInner {
        self.id
    }

    pub fn info(&self) -> &ShipmentInfo {
        &self.info
    }
}
