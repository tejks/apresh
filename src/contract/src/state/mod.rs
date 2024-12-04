mod carriers;
mod customers;
mod shipments;

#[macro_use]
pub(crate) mod macros;

use carriers::Carriers;
use customers::Customers;
use shipments::Shipments;

use std::cell::RefCell;

#[derive(Default)]
pub struct CanisterState {
    pub customers: Customers,
    pub carriers: Carriers,
    pub shipments: Shipments,
    pub shipment_counter: u64,
}


thread_local! {
    pub static STATE: RefCell<CanisterState> = Default::default();
}
