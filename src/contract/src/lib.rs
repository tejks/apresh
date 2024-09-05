mod models;
mod state;

use ic_cdk::{query, update};
use models::{
    shipment::{Shipment, ShipmentInfo},
    shipment_id::ShipmentId,
};
use state::{CUSTOMERS, SHIPMENTS};

#[update(name = "createShipment")]
fn create_shipment(customer_name: String, shipment_info: ShipmentInfo) {
    let customer_id = ic_cdk::caller();

    CUSTOMERS.with_borrow_mut(|customers| {
        let customer = customers.get_or_create(customer_name, customer_id);
        let shipment_id = ShipmentId::new();
        let inner_shipment_id = shipment_id.into_inner();
        let shipment = Shipment::create(customer, inner_shipment_id, shipment_info);

        SHIPMENTS.with_borrow_mut(|shipments| shipments.insert(inner_shipment_id, shipment));
    });
}

#[query(name = "listPendingShipments")]
fn get_pending_shipments() -> Vec<Shipment> {
    SHIPMENTS.with_borrow(|shipments| shipments.get_all_pending())
}

#[query(name = "listUserShipments")]
fn get_user_shipments() -> Vec<Shipment> {
    let customer_id = ic_cdk::caller();

    SHIPMENTS.with_borrow(|shipments| shipments.get_all_for_customer(&customer_id))
}

#[cfg(test)]
mod tests {

    #[test]
    fn list_shipments() {}
}

ic_cdk::export_candid!();
