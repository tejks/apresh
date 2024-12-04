use crate::impl_deref;
use crate::models::customer::{self, Customer, CustomerId};
use std::collections::HashMap;

type CustomersStore = HashMap<customer::CustomerId, customer::Customer>;

#[derive(Default)]
pub struct Customers(CustomersStore);

impl_deref!(Customers, CustomersStore);

impl Customers {
    pub fn get_or_create(
        &mut self,
        customer_name: String,
        customer_id: CustomerId,
    ) -> &mut Customer {
        let customer_exists = self.get_mut(&customer_id).is_some();

        if !customer_exists {
            let customer = Customer::new(customer_id, customer_name);
            self.insert(customer_id, customer);
        }

        self.get_mut(&customer_id).expect("")
    }
}
