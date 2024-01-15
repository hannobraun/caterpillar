use std::collections::BTreeMap;

use crate::{
    module::function::UserDefinedFunction,
    repr::eval::fragments::{FragmentId, FragmentPayload, Fragments},
    value::ValuePayload,
};

#[derive(Debug, Default)]
pub struct Functions(pub BTreeMap<String, UserDefinedFunction>);

impl Functions {
    pub fn replace(
        &mut self,
        old: FragmentId,
        new: FragmentId,
        fragments: &Fragments,
    ) {
        let mut renames = Vec::new();

        for (old_name, UserDefinedFunction { name, body, .. }) in
            self.0.iter_mut()
        {
            if name.fragment == Some(old) {
                let fragment = fragments.get(new);

                // This struct can contain either user-defined functions or
                // tests. Functions have symbols as names, while tests have text
                // names.
                let FragmentPayload::Value(
                    ValuePayload::Symbol(new_name)
                    | ValuePayload::Text(new_name),
                ) = &fragment.payload
                else {
                    // If the new fragment is not a symbol, then it's not
                    // supposed to be a function name. Not sure if we can
                    // handle this any smarter.
                    continue;
                };

                name.value = new_name.clone();
                name.fragment = Some(new);

                renames.push((old_name.clone(), new_name.clone()));
            }
            if body.start == old {
                body.start = new;
            }
        }

        for (old, new) in renames {
            let function = self.0.remove(&old).expect(
                "Found `old` in the map; expecting it to still be there",
            );
            self.0.insert(new, function);
        }
    }
}
