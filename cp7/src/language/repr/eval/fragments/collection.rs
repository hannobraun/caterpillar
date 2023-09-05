use std::collections::HashMap;

use super::{
    replacements::Replacements, Fragment, FragmentAddress, FragmentId,
};

#[derive(Debug)]
pub struct Fragments {
    by_id: HashMap<FragmentId, Fragment>,
    by_address: HashMap<FragmentAddress, FragmentId>,
    by_next_fragment: HashMap<FragmentId, FragmentId>,
    replacements: Replacements,
}

impl Fragments {
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
            by_address: HashMap::new(),
            by_next_fragment: HashMap::new(),
            replacements: Replacements::new(),
        }
    }

    pub fn insert(&mut self, fragment: Fragment) -> FragmentId {
        let id = FragmentId {
            hash: fragment.hash(),
        };
        let address = fragment.address;

        if let Some(existing) = self.by_id.insert(id, fragment.clone()) {
            // A hash collision should be exceedingly unlikely, but I'm not sure
            // quite *how* unlikely. Also, I don't fully trust my code to work
            // perfectly.
            //
            // Let's make sure, just for now, there actually are no hash
            // collisions, okay?
            assert_eq!(existing, fragment);
        } else {
            let id = id.display_short();
            let payload = fragment.payload.display_short();
            let address = address.display_short();

            eprintln!("insert {id} ({payload}) at {address}");
        }

        if let Some(next) = address.next {
            self.by_next_fragment.insert(next, id);
        }

        {
            if let Some(existing) = self.by_address.get(&address).copied() {
                // This is a bit too simplistic to detect changes of more than
                // one syntax fragment. It will do for now, but to make this
                // more general, we will eventually have to modify the address
                // by looking at the already detected replacements.

                if existing != id {
                    self.replacements.insert(existing, id);

                    let existing = existing.display_short();
                    let id = id.display_short();
                    eprintln!("Replace {existing} with {id}");
                }
            }
        }

        self.by_address.insert(address, id);

        id
    }

    pub fn get(&self, id: FragmentId) -> Fragment {
        // This shouldn't ever panic, as we currently only ever add fragments,
        // never remove them, and only ever create IDs for fragments we add.
        self.by_id.get(&id).cloned().unwrap()
    }

    pub fn take_replacements(&mut self) -> Vec<(FragmentId, FragmentId)> {
        self.replacements.take()
    }
}
