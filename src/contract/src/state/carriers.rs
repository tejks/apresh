use std::collections::HashMap;
use crate::impl_deref;
use crate::actors::carrier::{Carrier, CarrierId};

type CarriersStore = HashMap<CarrierId, Carrier>;

#[derive(Default)]
pub struct Carriers(CarriersStore);

impl_deref!(Carriers, CarriersStore);

impl Carriers {
    pub fn get_or_create<'a>(
        &mut self,
        carrier_id: CarrierId,
        carrier_name: &'a str,
    ) -> &mut Carrier {
        let carrier_exists = self.get_mut(&carrier_id).is_some();

        if !carrier_exists {
            let carrier = Carrier::new(carrier_id, carrier_name);
            self.insert(carrier_id, carrier);
        }

        self.get_mut(&carrier_id).expect("")
    }
}
