use std::collections::HashMap;

use tracing::debug;

use super::{
    replacements::{Replacement, Replacements},
    Fragment, FragmentAddress, FragmentId,
};

#[derive(Debug, Default)]
pub struct Fragments {
    by_id: HashMap<FragmentId, Fragment>,
    by_address: HashMap<FragmentAddress, FragmentId>,
    replacements: Replacements,
}

impl Fragments {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, fragment: Fragment) -> FragmentId {
        let id = fragment.id();
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

            debug!("insert {id} ({payload}) at {address}");
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

                // For the update process to work, we need to know which
                // fragments replaced which other fragments. Let's start by
                // checking for the straight-forward case here. If there's
                // already a different fragment with the same address, we know
                // that the new fragment replaces it.
                if let Some(existing) = self.by_address.get(&address).copied() {
                    if existing != new {
                        // Let's only do the update, if we new id is actually
                        // different from the existing one, i.e. we're actually
                        // replacing anything.

                        self.replacements.insert(existing, new);

                        let existing = existing.display_short();
                        let new = new.display_short();
                        debug!("Replace {existing} with {new}");
                    }
                }

                // The replacement above only catches very obvious replacements
                // like this:
                //
                // ```
                // b a -> c a
                // ```
                //
                // Here, `b` is replaced by `c`. Straight-forward to detect,
                // because they have the same address.
                //
                // However, if the replacement is not at the beginning of a
                // context, things are more complicated:
                //
                // ```
                // c b a -> e d a
                // ```
                //
                // Here, `b` is replaced by `d`. It's impossible for `c` to stay
                // there in this case, since the ID of a fragment depends on its
                // next fragment (`b`/`d`), and that has changed.
                //
                // Therefore, `c` is replaced by `e`, but that replacement is
                // more difficult to detect. They don't have the same address
                // (one has `b` as next, the other `d`).
                //
                // That's what the following code is for. It checks whether IDs
                // in the address are known to have been replaced, modifies the
                // address accordingly, then tries again.

                let replaced_by_parent = address
                    .parent
                    .and_then(|parent| self.replacements.replaced_by(parent));
                if let Some(replaced_by_parent) = replaced_by_parent {
                    address.parent = Some(replaced_by_parent);
                    continue;
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

    pub fn get(&self, id: FragmentId) -> &Fragment {
        // This shouldn't ever panic, as we currently only ever add fragments,
        // never remove them, and only ever create IDs for fragments we add.
        self.by_id.get(&id).unwrap()
    }

    pub fn take_replacements(&mut self) -> Vec<Replacement> {
        self.replacements.take()
    }
}
