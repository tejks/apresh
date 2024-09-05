mod models;
mod state;
mod transfer;

use ic_cdk::{query, update};
use icrc_ledger_types::icrc1::transfer::NumTokens;
use models::{
    shipment::{Shipment, ShipmentInfo},
    shipment_id::ShipmentId,
};
use state::{CUSTOMERS, SHIPMENTS};
use transfer::transfer_in;

#[update(name = "createShipment")]
async fn create_shipment(customer_name: String, shipment_info: ShipmentInfo) -> Result<(), String> {
    let customer_id = ic_cdk::caller();
    let amount = NumTokens::from(shipment_info.price());

    CUSTOMERS.with_borrow_mut(|customers| {
        let customer = customers.get_or_create(customer_name, customer_id);
        let shipment_id = ShipmentId::new();
        let inner_shipment_id = shipment_id.into_inner();
        let shipment = Shipment::create(customer, inner_shipment_id, shipment_info);
        SHIPMENTS.with_borrow_mut(|shipments| shipments.insert(inner_shipment_id, shipment));
    });

    let transfer_in_args = transfer::TransferInParams {
        amount: NumTokens::from(amount),
        from: customer_id.into(),
        memo: None,
    };

    transfer_in(transfer_in_args).await.map_err(|e| e.to_string())
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
