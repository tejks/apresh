mod actors;
mod models;
mod operations;
mod qr;
mod state;
mod transfer;
mod utils;
mod vetkd;

use candid::Principal;
use ic_cdk::{query, update};
use icrc_ledger_types::icrc1::transfer::NumTokens;
use models::{
    qrcode::QrCodeOptions,
    shipment::{Shipment, ShipmentInfo, ShipmentStatus},
};
use operations::{AddMessageOp, BuyShipmentOp, CreateShipmentOp, FinalizeShipmentOp, ReadMessageOp, StateOp};
use state::STATE;
use transfer::{transfer_in, transfer_out, TransferInParams, TransferOutParams, TransferParams};
use utils::block_anonymous;

pub use vetkd::{encrypted_ibe_decryption_key_for_caller, ibe_encryption_key};

#[update(name = "addEncryptedMessage")]
async fn add_encrypted_message(message: String, shipment_id: u64) -> Result<(), String> {
    let caller: Principal = ic_cdk::caller();

    STATE
        .with_borrow_mut(|state| AddMessageOp::new(shipment_id, &message, caller).apply(state))
        .map_err(|e| e.to_string())
}

#[update(name = "readEncryptedMessage")]
async fn read_encrypted_message(shipment_id: u64) -> Result<Option<String>, String> {
    let caller: Principal = ic_cdk::caller();

    STATE
        .with_borrow_mut(|state| ReadMessageOp::new(shipment_id, caller).apply(state))
        .map_err(|e| e.to_string())
}

#[update(name = "finalizeShipment")]
async fn finalize_shipment(shipment_id: u64, secret_key: Option<String>) -> Result<(), String> {
    let caller = ic_cdk::caller();

    let finalize_shipment_op = FinalizeShipmentOp::new(shipment_id, secret_key.as_deref(), caller);

    let finalize_shipment_result = STATE
        .with_borrow_mut(|state| finalize_shipment_op.apply(state))
        .map_err(|e: anyhow::Error| e.to_string())?;

    let transfer_out_carrier_args = TransferOutParams {
        params: TransferParams {
            amount: NumTokens::from(
                finalize_shipment_result.value() + finalize_shipment_result.price(),
            ),
            memo: None,
        },
        to: (*finalize_shipment_result.carrier_id()).into(),
    };

    let transfer_out_carrier_result = transfer_out(transfer_out_carrier_args)
        .await
        .map_err(|e| e.to_string());

    match transfer_out_carrier_result {
        Ok(_) => Ok(()),
        Err(e) => ic_cdk::trap(&e.to_string()),
    }
}

#[update(name = "buyShipment")]
async fn buy_shipment(carrier_name: String, shipment_id: u64) -> Result<(), String> {
    block_anonymous()?;

    let carrier_id = ic_cdk::caller();

    let buy_shipment_op = BuyShipmentOp::new(carrier_id, &carrier_name, shipment_id);

    let shipment_cost = STATE
        .with_borrow_mut(|state| buy_shipment_op.apply(state))
        .map_err(|e: anyhow::Error| e.to_string())?;

    let transfer_in_args = TransferInParams {
        params: TransferParams {
            amount: NumTokens::from(shipment_cost),
            memo: None,
        },
        from: carrier_id.into(),
    };

    let transfer_in_result = transfer_in(transfer_in_args)
        .await
        .map_err(|e| e.to_string());

    match transfer_in_result {
        Ok(_) => Ok(()),
        Err(e) => ic_cdk::trap(&e),
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

#[update(name = "createShipment")]
async fn create_shipment(
    customer_name: String,
    shipment_name: String,
    hashed_secret: String,
    qr_options: QrCodeOptions,
    shipment_info: ShipmentInfo,
) -> Result<(Vec<u8>, u64), String> {
    block_anonymous()?;

    let customer_id = ic_cdk::caller();
    let amount = NumTokens::from(shipment_info.price());

    let transfer_in_args = TransferInParams {
        params: TransferParams {
            amount: NumTokens::from(amount),
            memo: None,
        },
        from: customer_id.into(),
    };

    transfer_in(transfer_in_args)
        .await
        .map_err(|e| e.to_string())?;

    let created_at = ic_cdk::api::time();

    let create_shipment_op = CreateShipmentOp::new(
        customer_id,
        &customer_name,
        &hashed_secret,
        &shipment_name,
        shipment_info,
        created_at,
    );

    let shipment_id = STATE
        .with_borrow_mut(|state| create_shipment_op.apply(state))
        .map_err(|e| e.to_string())?;

    let qr_code = qr::generate(qr_options).unwrap_or_else(|err| ic_cdk::trap(&err.to_string()));

    Ok((qr_code, shipment_id))
}

#[query(name = "listPendingShipments")]
fn get_pending_shipments() -> Vec<Shipment> {
    STATE.with_borrow(|state| {
        state
            .shipments
            .values()
            .filter(|shipment| *shipment.status() == ShipmentStatus::Created)
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
            .values()
            .filter(|shipment| shipment.shipper_id() == customer_id)
            .cloned()
            .collect()
    });

    let carriers = STATE.with_borrow(|state| {
        state
            .shipments
            .values()
            .filter(|shipment| shipment.carrier_id() == Some(customer_id))
            .cloned()
            .collect()
    });

    (shippers, carriers)
}

#[query]
fn roles() -> (bool, bool) {
    let caller = ic_cdk::caller();

    let carrier = STATE.with_borrow(|state| state.carriers.get(&caller).is_some());
    let shipper = STATE.with_borrow(|state| state.shippers.get(&caller).is_some());

    (carrier, shipper)
}

#[query]
fn shipments() -> Vec<Shipment> {
    STATE.with_borrow(|state| state.shipments.values().cloned().collect())
}

ic_cdk::export_candid!();
