use std::collections::HashMap;

use super::{
    replacements::{Replacement, Replacements},
    Fragment, FragmentAddress, FragmentId,
};

#[derive(Debug)]
pub struct Fragments {
    by_id: HashMap<FragmentId, Fragment>,
    by_address: HashMap<FragmentAddress, FragmentId>,
    replacements: Replacements,
}

impl Fragments {
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
            by_address: HashMap::new(),
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
            // Only show the `insert` log message if we've actually inserted
            // something new. This reduces log spam.

            let id = id.display_short();
            let payload = fragment.payload.display_short();
            let address = address.display_short();

            eprintln!("insert {id} ({payload}) at {address}");
        }

        {
            let new = id;
            let mut address = address;

            let mut i = 0;
            loop {
                i += 1;
                if i == 128 {
                    // I guess we could reach this number of iterations with big
                    // programs, so the number we panic at should probably
                    // depend on the number of fragments. But this will do for
                    // now.
                    panic!("Possibly endless loop when replacing fragments");
                }

                if let Some(existing) = self.by_address.get(&address).copied() {
                    if existing != new {
                        // Let's only do the update, if we new id is actually
                        // different from the existing one, i.e. we're actually
                        // replacing anything.

                        self.replacements.insert(existing, new);

                        let existing = existing.display_short();
                        let new = new.display_short();
                        eprintln!("Replace {existing} with {new}");
                    }
                }

                let replaced_by_next = address
                    .next
                    .and_then(|next| self.replacements.replaced_by(next));
                if let Some(replaced_by_next) = replaced_by_next {
                    address.next = Some(replaced_by_next);
                    continue;
                }

                break;
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

    pub fn take_replacements(&mut self) -> Vec<Replacement> {
        self.replacements.take()
    }
}
