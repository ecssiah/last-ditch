use crate::simulation::state::physics::body::contact::Contact;
use std::fmt;

#[derive(Clone, Copy, Debug, Default)]
pub struct ContactSet(u32);

impl ContactSet {
    pub const EMPTY: Self = Self(0);

    pub fn has(contact: Contact, contact_set: &ContactSet) -> bool {
        (contact_set.0 & contact as u32) != 0
    }

    pub fn add(contact: Contact, contact_set: &mut ContactSet) {
        contact_set.0 |= contact as u32;
    }

    pub fn clear(contact_set: &mut ContactSet) {
        contact_set.0 = 0;
    }
}

impl fmt::Display for ContactSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;

        for contact in Contact::ALL {
            if Self::has(*contact, self) {
                if !first {
                    f.write_str(", ")?;
                }

                f.write_str(contact.as_str())?;
                first = false;
            }
        }

        if first {
            f.write_str("none")?;
        }

        Ok(())
    }
}
