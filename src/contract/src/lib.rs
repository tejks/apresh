mod models;
mod state;

use candid::Principal;
use ic_cdk::{query, update, init};
use models::{
    customer::Customer, shipment::{Shipment, ShipmentInfo, ShipmentLocation, SizeCategory}, shipment_id::ShipmentId
};
use state::{CUSTOMERS, SHIPMENTS};

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

    for i in 0..10 {
        let shipment_id = ShipmentId::new();
        let inner_shipment_id = shipment_id.into_inner();

        let (origin_label, origin_lat, origin_lng) = &locations[i % locations.len()];
        let (dest_label, dest_lat, dest_lng) = &locations[(i + 1) % locations.len()];

        let shipment = Shipment::create(
            &mut default_customer,
            inner_shipment_id,
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
