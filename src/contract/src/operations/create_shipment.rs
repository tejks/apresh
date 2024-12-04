use super::{CanisterState, StateOp};

use crate::{
    actors::{customer::CustomerId, Actor},
    models::shipment::{Shipment, ShipmentId, ShipmentInfo},
};

pub struct CreateShipmentOp<'a> {
    creator_id: CustomerId,
    creator_name: &'a str,
    hashed_secret: &'a str,
    shipment_name: &'a str,
    info: ShipmentInfo,
    timestamp: u64,
}

impl<'a> CreateShipmentOp<'a> {
    pub fn new(
        creator_id: CustomerId,
        creator_name: &'a str,
        hashed_secret: &'a str,
        shipment_name: &'a str,
        info: ShipmentInfo,
        timestamp: u64,
    ) -> Self {
        Self {
            creator_id,
            creator_name,
            hashed_secret,
            shipment_name,
            info,
            timestamp,
        }
    }
}

impl StateOp<ShipmentId> for CreateShipmentOp<'_> {
    fn apply(&self, state: &mut CanisterState) -> Result<ShipmentId, anyhow::Error> {
        let new_shipment_id = state.shipment_counter;
        state.shipment_counter += 1;

        let customer = state
            .customers
            .get_or_create(&self.creator_name, self.creator_id);

        let shipment = Shipment::create(
            self.timestamp,
            customer.id(),
            new_shipment_id,
            self.hashed_secret,
            self.shipment_name,
            self.info.clone(),
        );

        state.shipments.insert(new_shipment_id, shipment);
        customer.add_shipment(new_shipment_id);

        Ok(new_shipment_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;
    use crate::models::shipment::{ShipmentInfo, ShipmentLocation, SizeCategory};

    #[test]
    fn test_create_shipment_success() {
        let mut state = CanisterState::default();

        let creator_id = Principal::anonymous();
        let creator_name = "John Doe";
        let hashed_secret = "hashed_secret_123";
        let shipment_name = "Test Shipment";
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let info = ShipmentInfo::new(
            100,
            10,
            ShipmentLocation::new("Warsaw".to_string(), 52.23, 21.01),
            ShipmentLocation::new("Krakow".to_string(), 54.44, 18.23),
            SizeCategory::Envelope,
        );

        let op = CreateShipmentOp::new(
            creator_id,
            creator_name,
            hashed_secret,
            shipment_name,
            info.clone(),
            timestamp,
        );

        let result = op.apply(&mut state);

        assert!(result.is_ok());
        let shipment_id = result.unwrap();
        assert_eq!(shipment_id, 0);
        assert_eq!(state.shipment_counter, 1);

        let shipment = state.shipments.get(&shipment_id).unwrap();
        assert_eq!(shipment.customer_id(), creator_id);
        assert_eq!(shipment.id(), shipment_id);
        assert_eq!(shipment.name(), shipment_name);

        let customer = state.customers.get(&creator_id).unwrap();
        assert_eq!(customer.id(), creator_id);
        assert!(customer.active_shipments().contains(&shipment_id));
        assert_eq!(customer.name(), creator_name);
    }
}
