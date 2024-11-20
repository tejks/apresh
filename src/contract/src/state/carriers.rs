use std::{collections::HashMap, ops::{Deref, DerefMut}};
use crate::models::carrier;

type CarriersStore = HashMap<carrier::CarrierId, carrier::Carrier>;

#[derive(Default)]
pub struct Carriers(CarriersStore);

impl Deref for Carriers {
    type Target = CarriersStore;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Carriers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Carriers {
    pub fn get_or_create(
        &mut self,
        carrier_id: carrier::CarrierId,
        carrier_name: String,
    ) -> &mut carrier::Carrier {
        let carrier_exists = self.get_mut(&carrier_id).is_some();

        if !carrier_exists {
            let carrier = carrier::Carrier::new(carrier_id, carrier_name);
            self.insert(carrier_id, carrier);
        }

        self.get_mut(&carrier_id).expect("")
    }
}
