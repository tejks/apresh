mod models;
mod qr;
mod state;
mod transfer;
mod vetkd;
mod vetkd_types;

use anyhow::anyhow;
use candid::Principal;
use ic_cdk::{init, query, update};
use icrc_ledger_types::icrc1::transfer::NumTokens;
use models::{
    customer::Customer,
    qrcode::QrCodeOptions,
    shipment::{Shipment, ShipmentInfo, ShipmentLocation, SizeCategory},
    shipment_id::{ShipmentId, ShipmentIdInner},
};
use state::{CARRIERS, CUSTOMERS, SHIPMENTS};
use transfer::transfer_in;

fn check_anonymous(caller: Principal) -> Result<(), String> {
    if caller == Principal::anonymous() {
        return Err("Cannot be called anonymously".to_string());
    }

    Ok(())
}

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
    let locations = [
        ("A", 40.7128, -74.0060),  // New York, USA
        ("B", 34.0522, -118.2437), // Los Angeles, USA
        ("C", 51.5074, -0.1278),   // London, UK
        ("D", 48.8566, 2.3522),    // Paris, France
        ("E", 35.6895, 139.6917),  // Tokyo, Japan
        ("F", -33.8688, 151.2093), // Sydney, Australia
    ];

    let names: [&str; 10] = [
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

    for (i, name) in names.iter().enumerate() {
        let shipment_id = ShipmentId::new();
        let inner_shipment_id = shipment_id.into_inner();

        let (origin_label, origin_lat, origin_lng) = &locations[i % locations.len()];
        let (dest_label, dest_lat, dest_lng) = &locations[(i + 1) % locations.len()];

        let shipment = Shipment::create(
            &mut default_customer,
            inner_shipment_id,
            "hashed_secret".to_string(),
            name.to_string(),
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

#[update(name = "addEncryptedMessage")]
async fn add_encrypted_message(
    message: String,
    shipment_id: ShipmentIdInner,
) -> Result<(), String> {
    let caller: Principal = ic_cdk::caller();

    SHIPMENTS.with_borrow_mut(|shipments| {
        let Some(shipment) = shipments.get_mut(&shipment_id) else {
            return Err("Shipment not found".to_string());
        };

        if shipment.carrier_id().is_some_and(|v| v != caller) {
            return Err("Only the carrier can add an encrypted message".to_string());
        }

        shipment.add_encrypted_message(message);
        Ok(())
    })
}

#[update(name = "readEncryptedMessage")]
async fn read_encrypted_message(shipment_id: ShipmentIdInner) -> Result<Option<String>, String> {
    let caller: Principal = ic_cdk::caller();

    SHIPMENTS.with_borrow(|shipments| {
        if let Some(shipment) = shipments.get(&shipment_id) {
            if shipment.customer_id() == caller {
                Ok(shipment.encrypted_message())
            } else {
                Err("Only the customer can read an encrypted message".to_string())
            }
        } else {
            Err("Shipment not found".to_string())
        }
    })
}

#[update(name = "finalizeShipment")]
async fn finalize_shipment(
    shipment_id: ShipmentIdInner,
    secret_key: Option<String>,
) -> Result<(), String> {
    let caller = ic_cdk::caller();
    let (finalize_result, carrier, value, price) = SHIPMENTS
        .with_borrow_mut(|shipments| {
            let shipment = shipments
                .get_mut(&shipment_id)
                .ok_or(anyhow!("Shipment not found"))?;

            CUSTOMERS.with_borrow_mut(|customers| {
                let customer = customers
                    .get_mut(&shipment.customer_id())
                    .ok_or(anyhow!("Customer not found"))?;

                CARRIERS.with_borrow_mut(|carriers| {
                    let carrier = carriers
                        .get_mut(&shipment.carrier_id().ok_or(anyhow!("Carrier not set"))?)
                        .ok_or(anyhow!("Carrier not found"))?;

                    Ok((
                        shipment.finalize(carrier, customer, secret_key, caller),
                        carrier.id(),
                        shipment.info().value(),
                        shipment.info().price(),
                    ))
                })
            })
        })
        .map_err(|e: anyhow::Error| e.to_string())?;

    let transfer_out_carrier_args = transfer::TransferOutParams {
        amount: NumTokens::from(value + price),
        to: carrier.into(),
        memo: None,
    };

    if let Err(e) = finalize_result {
        return Err(e.to_string());
    }

    // if let Err(e) = transfer::transfer_out(transfer_out_carrier_args).await {
    //     ic_cdk::trap(&e.to_string())
    // }

    ic_cdk::print(format!("Shipment finalized: {:?}", shipment_id).as_str());

    Ok(())
}

#[update(name = "buyShipment")]
async fn buy_shipment(carrier_name: String, shipment_id: ShipmentIdInner) -> Result<(), String> {
    let carrier_id = ic_cdk::caller();
    check_anonymous(carrier_id)?;

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

    let _transfer_in_args = transfer::TransferInParams {
        amount: NumTokens::from(amount),
        from: carrier_id.into(),
        memo: None,
    };

    if let Err(e) = buy_result {
        return Err(e.to_string());
    }

    ic_cdk::print(format!("Shipment bought: {:?}", shipment_id).as_str());

    Ok(())
}

#[query(name = "generateQr")]
async fn generate_qr(link: String, size: usize) -> Result<Vec<u8>, String> {
    qr::generate(QrCodeOptions {
        gradient: false,
        link,
        size,
        transparent: false,
    })
    .map_err(|e| e.to_string())
}

#[update(name = "createShipment")]
async fn create_shipment(
    customer_name: String,
    shipment_name: String,
    hashed_secret: String,
    qr_options: QrCodeOptions,
    shipment_info: ShipmentInfo,
) -> Result<(Vec<u8>, ShipmentIdInner), String> {
    ic_cdk::print("Creating a shipment");

    let customer_id = ic_cdk::caller();
    check_anonymous(customer_id)?;

    // let amount = NumTokens::from(shipment_info.price());

    // let transfer_in_args = transfer::TransferInParams {
    //     amount: NumTokens::from(amount),
    //     from: customer_id.into(),
    //     memo: None,
    // };

    // transfer_in(transfer_in_args)
    //     .await
    //     .map_err(|e| e.to_string())?;

    let shipment_id = CUSTOMERS.with_borrow_mut(|customers| {
        let customer = customers.get_or_create(customer_name, customer_id);
        let shipment_id = ShipmentId::new();
        let inner_shipment_id = shipment_id.into_inner();
        let shipment = Shipment::create(
            customer,
            inner_shipment_id,
            hashed_secret,
            shipment_name,
            shipment_info,
        );
        SHIPMENTS.with_borrow_mut(|shipments| shipments.insert(inner_shipment_id, shipment));
        inner_shipment_id
    });

    let qr_code = qr::generate(qr_options).unwrap_or_else(|err| ic_cdk::trap(&err.to_string()));

    ic_cdk::print(format!("Shipment created: {:?}", shipment_id).as_str());
    Ok((qr_code, shipment_id))
}

#[query(name = "listPendingShipments")]
fn get_pending_shipments() -> Vec<Shipment> {
    SHIPMENTS.with_borrow(|shipments| shipments.get_all_pending())
}

#[query(name = "listUserShipments")]
fn get_user_shipments() -> (Vec<Shipment>, Vec<Shipment>) {
    let customer_id = ic_cdk::caller();

    let shippers = SHIPMENTS.with_borrow(|shipments| shipments.get_all_for_shipper(&customer_id));
    let customers = SHIPMENTS.with_borrow(|shipments| shipments.get_all_for_customer(&customer_id));
    (shippers, customers)
}

#[query]
fn roles() -> (bool, bool) {
    let carrier = CARRIERS.with_borrow(|carriers| carriers.contains_key(&ic_cdk::caller()));
    let customer = CUSTOMERS.with_borrow(|customers| customers.contains_key(&ic_cdk::caller()));

    (carrier, customer)
}

#[query]
fn shipments() -> Vec<Shipment> {
    SHIPMENTS.with_borrow(|shipments| shipments.values().cloned().collect())
}

ic_cdk::export_candid!();
