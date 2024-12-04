use super::StateOp;
use crate::{actors::carrier::CarrierId, models::shipment::ShipmentId, state::CanisterState};
use anyhow::anyhow;

pub type Cost = u64;

pub struct BuyShipmentOp<'a> {
    carrier_id: CarrierId,
    carrier_name: &'a str,
    shipment_id: ShipmentId,
}

impl<'a> BuyShipmentOp<'a> {
    pub fn new(carrier_id: CarrierId, carrier_name: &'a str, shipment_id: ShipmentId) -> Self {
        Self {
            carrier_id,
            carrier_name,
            shipment_id,
        }
    }
}

impl<'a> StateOp<Cost> for BuyShipmentOp<'a> {
    fn apply(&self, state: &mut CanisterState) -> Result<Cost, anyhow::Error> {
        state
            .carriers
            .contains_key(&self.carrier_id)
            .then_some(self.carrier_id)
            .ok_or(anyhow!("Carrier not found"))?;

        let shipment = state
            .shipments
            .get_mut(&self.shipment_id)
            .ok_or(anyhow!("Shipment not found"))?;

        shipment.buy(self.carrier_id)?;

        Ok(shipment.info().value())
    }
}
