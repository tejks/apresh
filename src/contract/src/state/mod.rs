mod carriers;
mod customers;
mod shipments;

pub use carriers::Carriers;
pub use customers::Customers;
pub use shipments::Shipments;

use std::cell::RefCell;

thread_local! {
    pub static CUSTOMERS: RefCell<Customers> = Default::default();
    pub static SHIPMENT_COUNTER: RefCell<u64> = Default::default();
    pub static SHIPMENTS: RefCell<Shipments> = Default::default();
    pub static CARRIERS: RefCell<Carriers> = Default::default();
}
