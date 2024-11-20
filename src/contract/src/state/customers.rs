use std::{collections::HashMap, ops::{Deref, DerefMut}};
use crate::models::customer::{self, Customer, CustomerId};

type CustomersStore = HashMap<customer::CustomerId, customer::Customer>;

#[derive(Default)]
pub struct Customers(CustomersStore);

impl Deref for Customers {
    type Target = CustomersStore;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Customers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
