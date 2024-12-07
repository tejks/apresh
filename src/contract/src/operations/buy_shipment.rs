use super::StateOp;
use crate::{
    actors::{
        carrier::{Carrier, CarrierId},
        Actor,
    },
    models::shipment::{ShipmentActions, ShipmentId},
    state::CanisterState,
};
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
    type Error = anyhow::Error;

    fn apply(&self, state: &mut CanisterState) -> Result<Cost, Self::Error> {
        let carrier = match state.carriers.get_mut(&self.carrier_id) {
            Some(carrier) => carrier,
            None => state.carriers.create(
                self.carrier_id,
                Carrier::new(self.carrier_id, self.carrier_name),
            ),
        };

        let shipment = state
            .shipments
            .get_mut(&self.shipment_id)
            .ok_or(anyhow!("Shipment not found"))?;

        shipment.action(ShipmentActions::Buy(self.carrier_id))?;
        carrier.add_shipment(self.shipment_id);

        Ok(shipment.info().value())
    }
}
