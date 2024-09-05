mod models;
mod state;
mod transfer;

use anyhow::anyhow;
use candid::Principal;
use ic_cdk::{init, query, update};
use icrc_ledger_types::icrc1::transfer::NumTokens;
use models::{
    customer::Customer,
    shipment::{Shipment, ShipmentInfo, ShipmentLocation, SizeCategory},
    shipment_id::{ShipmentId, ShipmentIdInner},
};
use state::{CARRIERS, CUSTOMERS, SHIPMENTS};
use transfer::transfer_in;

#[init]
fn init() {
    ic_cdk::print("Initializing the shipment service");

    // Create a default customer
    let mut default_customer = Customer::new(
        Principal::from_text("ryssj-xcbz7-gbw4s-p7fio-lolnx-5nr7a-yxufe-cvpfg-6iujw-2ypsz-rqe")
            .unwrap(),
        "Test".to_string(),
    );

    // Define a set of realistic coordinates for shipment locations
    let locations = vec![
        ("A", 40.7128, -74.0060),  // New York, USA
        ("B", 34.0522, -118.2437), // Los Angeles, USA
        ("C", 51.5074, -0.1278),   // London, UK
        ("D", 48.8566, 2.3522),    // Paris, France
        ("E", 35.6895, 139.6917),  // Tokyo, Japan
        ("F", -33.8688, 151.2093), // Sydney, Australia
    ];

    let names = vec![
        "John Doe",
        "Jane Doe",
        "Alice Smith",
        "Bob Smith",
        "Charlie Brown",
        "Daisy Brown",
        "Eve Green",
        "Frank Green",
        "Grace Black",
        "Harry Black",
    ];

    for i in 0..10 {
        let shipment_id = ShipmentId::new();
        let inner_shipment_id = shipment_id.into_inner();

        let (origin_label, origin_lat, origin_lng) = &locations[i % locations.len()];
        let (dest_label, dest_lat, dest_lng) = &locations[(i + 1) % locations.len()];

        let shipment = Shipment::create(
            &mut default_customer,
            inner_shipment_id,
            names[i].to_string(),
            ShipmentInfo::new(
                100u64 + i as u64,
                10u64 + i as u64,
                ShipmentLocation::new(origin_label.to_string(), *origin_lat, *origin_lng),
                ShipmentLocation::new(dest_label.to_string(), *dest_lat, *dest_lng),
                SizeCategory::Envelope,
            ),
        );

        // Insert the shipment into the SHIPMENTS collection
        SHIPMENTS.with_borrow_mut(|shipments| shipments.insert(inner_shipment_id, shipment));
    }
}

#[update(name = "buyShipment")]
async fn buy_shipment(carrier_name: String, shipment_id: ShipmentIdInner) -> Result<(), String> {
    let carrier_id = ic_cdk::caller();

    let (buy_result, amount) = CARRIERS
        .with_borrow_mut(|carriers| {
            let carrier = carriers.get_or_create(carrier_id, carrier_name);

            SHIPMENTS.with_borrow_mut(|shipments| {
                let shipment = shipments
                    .get_mut(&shipment_id)
                    .ok_or(anyhow!("Shipment not found"))?;

                Ok((shipment.buy(carrier), shipment.info().value()))
            })
        })
        .map_err(|e: anyhow::Error| e.to_string())?;

    let transfer_in_args = transfer::TransferInParams {
        amount: NumTokens::from(amount),
        from: carrier_id.into(),
        memo: None,
    };

    match buy_result {
        Ok(_) => transfer_in(transfer_in_args)
            .await
            .map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[update(name = "createShipment")]
async fn create_shipment(
    customer_name: String,
    shipment_name: String,
    shipment_info: ShipmentInfo,
) -> Result<(), String> {
    let customer_id = ic_cdk::caller();
    let amount = NumTokens::from(shipment_info.price());

    let transfer_in_args = transfer::TransferInParams {
        amount: NumTokens::from(amount),
        from: customer_id.into(),
        memo: None,
    };

    transfer_in(transfer_in_args)
        .await
        .map_err(|e| e.to_string())?;

    CUSTOMERS.with_borrow_mut(|customers| {
        let customer = customers.get_or_create(customer_name, customer_id);
        let shipment_id = ShipmentId::new();
        let inner_shipment_id = shipment_id.into_inner();
        let shipment = Shipment::create(customer, inner_shipment_id, shipment_name, shipment_info);
        SHIPMENTS.with_borrow_mut(|shipments| shipments.insert(inner_shipment_id, shipment));
    });

    Ok(())
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
