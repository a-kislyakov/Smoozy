//! Adding a server and changing any saved field remain available without paid access.

use crate::edits::WhatChanged;

#[derive(Debug)]
pub struct AddingAServerWasOpen(());

#[derive(Debug)]
pub struct ChoosingADifferentKeyWasOpen(());

pub fn adding_a_server_may_be_composed() -> Result<AddingAServerWasOpen, std::convert::Infallible> {
    Ok(AddingAServerWasOpen(()))
}

pub fn the_edit_may_be_composed(
    _changed: WhatChanged,
) -> Result<ChoosingADifferentKeyWasOpen, std::convert::Infallible> {
    Ok(ChoosingADifferentKeyWasOpen(()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_and_every_edit_are_open_without_any_policy() {
        assert!(adding_a_server_may_be_composed().is_ok());
        for changed in [WhatChanged::TheKey, WhatChanged::HostOrPort,
            WhatChanged::Username, WhatChanged::TheLabelOrTheColour,
            WhatChanged::TheFallbackFolder] {
            assert!(the_edit_may_be_composed(changed).is_ok());
        }
    }
}
