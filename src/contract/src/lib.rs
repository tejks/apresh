mod models;
mod qr;
mod state;
mod transfer;
mod utils;
mod vetkd;
mod actors;

use anyhow::anyhow;
use candid::Principal;
use ic_cdk::{query, update};
use icrc_ledger_types::icrc1::transfer::NumTokens;
use models::{
    qrcode::QrCodeOptions,
    shipment::{Shipment, ShipmentInfo},
};
use state::STATE;
use transfer::{transfer_in, transfer_out, TransferInParams, TransferOutParams, TransferParams};
use utils::block_anonymous;

pub use vetkd::{encrypted_ibe_decryption_key_for_caller, ibe_encryption_key};

// #[init]
// fn init() {
//     ic_cdk::print("Initializing the shipment service");

//     // Create a default customer
//     let mut default_customer = Customer::new(
//         Principal::from_text("ryssj-xcbz7-gbw4s-p7fio-lolnx-5nr7a-yxufe-cvpfg-6iujw-2ypsz-rqe")
//             .unwrap(),
//         "Test".to_string(),
//     );

//     // Define a set of realistic coordinates for shipment locations
//     let locations = vec![
//         ("A", 40.7128, -74.0060),  // New York, USA
//         ("B", 34.0522, -118.2437), // Los Angeles, USA
//         ("C", 51.5074, -0.1278),   // London, UK
//         ("D", 48.8566, 2.3522),    // Paris, France
//         ("E", 35.6895, 139.6917),  // Tokyo, Japan
//         ("F", -33.8688, 151.2093), // Sydney, Australia
//     ];

//     let names = vec![
//         "John Doe",
//         "Jane Doe",
//         "Alice Smith",
//         "Bob Smith",
//         "Charlie Brown",
//         "Daisy Brown",
//         "Eve Green",
//         "Frank Green",
//         "Grace Black",
//         "Harry Black",
//     ];

//     for i in 0..10 {
//         let shipment_id = ShipmentId::new();
//         let inner_shipment_id = shipment_id.into_inner();

//         let (origin_label, origin_lat, origin_lng) = &locations[i % locations.len()];
//         let (dest_label, dest_lat, dest_lng) = &locations[(i + 1) % locations.len()];

//         let shipment = Shipment::create(
//             &mut default_customer,
//             inner_shipment_id,
//             "hashed_secret".to_string(),
//             names[i].to_string(),
//             ShipmentInfo::new(
//                 100u64 + i as u64,
//                 10u64 + i as u64,
//                 ShipmentLocation::new(origin_label.to_string(), *origin_lat, *origin_lng),
//                 ShipmentLocation::new(dest_label.to_string(), *dest_lat, *dest_lng),
//                 SizeCategory::Envelope,
//             ),
//         );

//         // Insert the shipment into the SHIPMENTS collection
//         STATE.with_borrow_mut(|state| state.shipments.insert(inner_shipment_id, shipment));
//     }
// }

#[update(name = "addEncryptedMessage")]
async fn add_encrypted_message(
    message: String,
    shipment_id: u64,
) -> Result<(), String> {
    let caller: Principal = ic_cdk::caller();

    STATE.with_borrow_mut(|state| {
        if let Some(shipment) = state.shipments.get_mut(&shipment_id) {
            if shipment.carrier_id().is_some_and(|v| v == caller) {
                Ok(shipment.add_encrypted_message(message))
            } else {
                Err("Only the carrier can add an encrypted message".to_string())
            }
        } else {
            Err("Shipment not found".to_string())
        }
    })
}

#[update(name = "readEncryptedMessage")]
async fn read_encrypted_message(shipment_id: u64) -> Result<Option<String>, String> {
    let caller: Principal = ic_cdk::caller();

    STATE.with_borrow(|state| {
        if let Some(shipment) = state.shipments.get(&shipment_id) {
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
    shipment_id: u64,
    secret_key: Option<String>,
) -> Result<(), String> {
    let caller = ic_cdk::caller();
    let (finalize_result, carrier, value, price) = STATE
        .with_borrow_mut(|state| {
            let shipment = state
                .shipments
                .get_mut(&shipment_id)
                .ok_or(anyhow!("Shipment not found"))?;

            let customer = state
                .customers
                .get_mut(&shipment.customer_id())
                .ok_or(anyhow!("Customer not found"))?;

            let carrier = state
                .carriers
                .get_mut(&shipment.carrier_id().ok_or(anyhow!("Carrier not set"))?)
                .ok_or(anyhow!("Carrier not found"))?;

            Ok((
                shipment.finalize(carrier, customer, secret_key, caller),
                carrier.id(),
                shipment.info().value(),
                shipment.info().price(),
            ))
        })
        .map_err(|e: anyhow::Error| e.to_string())?;

    let transfer_out_carrier_args = TransferOutParams {
        params: TransferParams {
            amount: NumTokens::from(value + price),
            memo: None,
        },
        to: carrier.into(),
    };

    match finalize_result {
        Ok(_) => Ok(transfer_out(transfer_out_carrier_args)
            .await
            .unwrap_or_else(|err| ic_cdk::trap(&err.to_string()))),
        Err(e) => Err(e.to_string()),
    }
}

#[update(name = "buyShipment")]
async fn buy_shipment(carrier_name: String, shipment_id: u64) -> Result<(), String> {
    block_anonymous()?;

    let carrier_id = ic_cdk::caller();

    let (buy_result, amount) = STATE
        .with_borrow_mut(|state| {
            let carrier = state.carriers.get_or_create(carrier_id, carrier_name);

            let shipment = state
                .shipments
                .get_mut(&shipment_id)
                .ok_or(anyhow!("Shipment not found"))?;

            Ok((shipment.buy(carrier), shipment.info().value()))
        })
        .map_err(|e: anyhow::Error| e.to_string())?;

    let transfer_in_args = TransferInParams {
        params: TransferParams {
            amount: NumTokens::from(amount),
            memo: None,
        },
        from: carrier_id.into(),
    };

    match buy_result {
        Ok(_) => Ok(transfer_in(transfer_in_args)
            .await
            .unwrap_or_else(|err| ic_cdk::trap(&err.to_string()))),
        Err(e) => Err(e.to_string()),
    }
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

// #[update(name = "createShipment")]
// async fn create_shipment(
//     customer_name: String,
//     shipment_name: String,
//     hashed_secret: String,
//     qr_options: QrCodeOptions,
//     shipment_info: ShipmentInfo,
// ) -> Result<(Vec<u8>, u64), String> {
//     block_anonymous()?;

//     let customer_id = ic_cdk::caller();
//     let amount = NumTokens::from(shipment_info.price());

//     let transfer_in_args = TransferInParams {
//         params: TransferParams {
//             amount: NumTokens::from(amount),
//             memo: None,
//         },
//         from: customer_id.into(),
//     };

//     transfer_in(transfer_in_args)
//         .await
//         .map_err(|e| e.to_string())?;

//     let shipment_id = STATE.with_borrow_mut(|state| {
//         let customer = state.customers.get_or_create(customer_name, customer_id);
//         let shipment_id = ShipmentId::new();
//         let inner_shipment_id = shipment_id.into_inner();
//         let shipment = Shipment::create(
//             customer,
//             inner_shipment_id,
//             hashed_secret,
//             shipment_name,
//             shipment_info,
//         );
//         state.shipments.insert(inner_shipment_id, shipment);
//         inner_shipment_id
//     });

//     let qr_code = qr::generate(qr_options).unwrap_or_else(|err| ic_cdk::trap(&err.to_string()));

//     Ok((qr_code, shipment_id))
// }

#[query(name = "listPendingShipments")]
fn get_pending_shipments() -> Vec<Shipment> {
    STATE.with_borrow(|state| {
        state
            .shipments
            .list_pending()
            .into_iter()
            .cloned()
            .collect()
    })
}

#[query(name = "listUserShipments")]
fn get_user_shipments() -> (Vec<Shipment>, Vec<Shipment>) {
    let customer_id = ic_cdk::caller();

    let shippers = STATE.with_borrow(|state| {
        state
            .shipments
            .list_for_shipper(&customer_id)
            .into_iter()
            .cloned()
            .collect()
    });

    let customers = STATE.with_borrow(|state| {
        state
            .shipments
            .list_for_customer(&customer_id)
            .into_iter()
            .cloned()
            .collect()
    });

    (shippers, customers)
}

#[query]
fn roles() -> (bool, bool) {
    let carrier = STATE.with_borrow(|state| state.carriers.contains_key(&ic_cdk::caller()));
    let customer = STATE.with_borrow(|state| state.customers.contains_key(&ic_cdk::caller()));

    (carrier, customer)
}

#[query]
fn shipments() -> Vec<Shipment> {
    STATE.with_borrow(|state| state.shipments.values().cloned().collect())
}

ic_cdk::export_candid!();
