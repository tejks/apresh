use crate::models::shipment::ShipmentId;
use candid::Principal;

pub mod base;
pub mod carrier;
pub mod shipper;

/// The role of the actor.
/// Every actor has a role. Right now there are only two roles: shipper and carrier. In the future there could be more roles i.e. forwarder.
pub enum ActorRole {
    /// The shipper actor.
    Shipper,
    /// The carrier actor.
    Carrier,
}

/// Trait for all actors.
pub trait Actor {
    /// The principal of the actor.
    fn id(&self) -> Principal;
    /// The name of the actor.
    fn name(&self) -> &str;
    /// The role of the actor.
    fn role(&self) -> ActorRole;
    /// Add a shipment to the actor.
    fn add_shipment(&mut self, shipment_id: ShipmentId);
    /// Archive a shipment.
    fn archive_shipment(&mut self, shipment_id: ShipmentId);
    /// The list of active/pending shipments.
    fn get_active_shipments(&self) -> &[ShipmentId];
    /// The list of completed/cancelled shipments.
    fn get_shipments_history(&self) -> &[ShipmentId];
}
