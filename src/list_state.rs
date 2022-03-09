pub(crate) enum ListState {
    NotInList,
    Entering,
    InList,
    Exiting,
}

pub(crate) fn update_list_state(state: &mut ListState, next_block_is_same: bool) {
    match (&state, next_block_is_same) {
        (ListState::NotInList, false) => {}
        (ListState::NotInList | ListState::Exiting, true) => *state = ListState::Entering,
        (ListState::Entering, true) => *state = ListState::InList,
        (ListState::InList, true) => {}
        (ListState::Entering | ListState::InList, false) => *state = ListState::Exiting,
        (ListState::Exiting, false) => *state = ListState::NotInList,
    }
}
