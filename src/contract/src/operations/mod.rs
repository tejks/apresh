use crate::state::CanisterState;

mod buy_shipment;
mod create_shipment;
mod finalize_shipment;

pub use buy_shipment::BuyShipmentOp;
pub use create_shipment::CreateShipmentOp;
pub use finalize_shipment::{FinalizeShipmentOp, FinalizeShipmentResult};

pub trait StateOp<R> {
    fn apply(&self, state: &mut CanisterState) -> Result<R, anyhow::Error>;
}
