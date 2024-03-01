use std::collections::HashMap;

use tracing::debug;

use super::{
    replacements::{Replacement, Replacements},
    Fragment, FragmentAddress, FragmentId,
};

#[derive(Clone, Debug, Default)]
pub struct Fragments {
    by_id: HashMap<FragmentId, Fragment>,
    by_address: HashMap<FragmentAddress, FragmentId>,
    replacements: Replacements,
}

impl Fragments {
    pub fn new() -> Self {
        Self::default()
    }

    /// # Insert fragment, detecting replaced fragments
    ///
    /// If this fragment replaced another one, that information is stored, and
    /// can later be retrieved via [`Fragments::take_replacements`].
    ///
    /// ## Algorithm
    ///
    /// The insertion itself is straight-forward: The fragment is inserted into
    /// a map, indexed by its ID (and also a second map, indexed by its
    /// address). Done.
    ///
    /// Detecting replacements is non-trivial, however. Let's consider some
    /// examples, starting with a simple one:
    ///
    /// ``` ignore
    /// b a -> c a
    /// ```
    ///
    /// Here, the letters are placeholders for fragments, while `->` separates
    /// the old version of the example code (on the left) from the new version
    /// (on the right).
    ///
    /// Here, `c` replaces `b`, which is rather straight-forward to detect. They
    /// have the same `parent` (as both are in the same context), and they are
    /// both followed by the same `next` element, `a`. As a result, they have
    /// the same address. Hence, checking if there already was a fragment at the
    /// the new fragment's address is the first step in detecting a replacement.
    ///
    /// But by itself, this is not enough. Let's consider this slightly more
    /// complicated case:
    ///
    /// ``` ignore
    /// c b a -> e d a
    /// ```
    ///
    /// Here, `b` is replaced by `d`. Even if `c` stays the same on a syntactic
    /// level, it has a new `next` fragment, and thus must become a new fragment
    /// itself. Hence, `c` is replaced by `e`.
    ///
    /// The replacement `b` -> `d` is again trivially detected by the address
    /// lookup step. But this won't detect the replacement `c` -> `e`. Their
    /// `next` fragments differ (they are `b` and `d`, respectively), meaning
    /// they have different addresses.
    ///
    /// So what can we do? Since syntax is analyzed (and hence, fragments are
    /// created) right-to-left, the replacement `b` -> `d` is already known by
    /// the point we consider the replacement `c` -> `e`. We can use this
    /// knowledge to update the address of `e`. After that update, we can find
    /// the replacement using another address lookup.
    ///
    /// Let's take a closer look at what happens, step by step. Since all the
    /// fragments in this example have the same parent, we'll ignore that part
    /// of the address, for the sake of simplicity. In the implementation, the
    /// `parent` is handled exactly the same as the `next` fragment.
    ///
    /// 1. Insert `a` at address `next: none`
    ///    1. no fragment already at address `next: none`
    ///    2. no `next` fragment available
    /// 2. Insert `b` at address `next: a`
    ///    1. no fragment already at address `next: a`
    ///    2. no known replacements of `a`
    /// 3. Insert `c` at address `next: b`
    ///    1. no fragment already at address `next: b`
    ///    2. no known replacements of `b`
    ///
    /// At this point, our initial script (`c b a`) has been fully analyzed and
    /// is running. Now the user makes a change to the original code, resulting
    /// in more fragments being inserted.
    ///
    /// 4. Insert `a` at address `next: none`
    ///    1. fragment `a` at address `next: none` is identical; no replacement
    ///    2. no `next` fragment available
    /// 5. Insert `d` at address `next: a`
    ///    1. fragment `b` at address `next: a`; note replacement `b` -> `d`
    ///    2. no known replacements of `a`
    /// 6. Insert `e` at address `next: d`
    ///    1. no fragment already at address `next: d`
    ///    2. `d` is known to replace `b`; substitute address to `next: b`
    ///    3. fragment `c` at address `next: b`; note replacement `c` -> `e`
    ///
    /// And that's it! We've detected both replacements (`b` -> `d` and `c` ->
    /// `e`). This scales up to more complicated cases, requiring more
    /// substitutions of the address each time.
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
            assert_eq!(
                existing, fragment,
                "Inserting fragment caused hash collision"
            );
        } else {
            // Reduce log spam by only showing the `insert` log message if we've
            // actually inserted a new fragment.

            let id = id.display_short();
            let payload = fragment.payload.display_short();
            let address = address.display_short();

            debug!("Insert {id} ({payload}) at {address}");
        }

        // We've done the actual inserting, but that's the easy part. What
        // follows is the core of this method: Check whether the inserted
        // fragment has replaced another. This is required information for the
        // code update that happens after the insertion.
        {
            let new = id;
            let mut address = address;

            let mut i = 0;
            loop {
                // Let's make sure that a bug in this algorithm doesn't cause it
                // to hang mysteriously.
                i += 1;
                if i == 128 {
                    // I guess we could reach this number of iterations with big
                    // programs, so the number we panic at should probably
                    // depend on the number of fragments. But this will do for
                    // now.
                    panic!("Possibly endless loop while replacing fragments");
                }

                // Look up the address, to find any fragments that this one
                // replaces. This might be the actual address of the inserted
                // fragment, or one substituted in an earlier iteration of the
                // loop.
                if let Some(existing) = self.by_address.get(&address).copied() {
                    if existing != new {
                        // Let's only do the update, if the new id is actually
                        // different from the existing one, i.e. we're actually
                        // replacing something.

                        self.replacements.insert(existing, new);

                        let existing = existing.display_short();
                        let new = new.display_short();
                        debug!("Replace {existing} with {new}");

                        break;
                    }
                }

                // No straight replacement found. Let's see if we need to
                // substitute the address. If substitutions are found, try
                // again.

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

                // No replacement found, and no substitutions found either.
                // We're done!

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
