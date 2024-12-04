use candid::Principal;
use crate::models::shipment::ShipmentId;

pub mod customer;
pub mod carrier;
pub mod base;

pub enum ActorRole {
    Customer,
    Carrier,
}

pub trait Actor {
  fn id(&self) -> Principal;
  fn name(&self) -> &str;
  fn role(&self) -> ActorRole;
  fn add_shipment(&mut self, shipment_id: ShipmentId);
  fn active_shipments(&self) -> &[ShipmentId];
  fn shipments_history(&self) -> &[ShipmentId];
  fn archive_shipment(&mut self, shipment_id: ShipmentId);
}
