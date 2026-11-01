fn waystones_event(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    event: &crate::game::native_hooks::NativePanelEvent,
) {
    use crate::game::official_waystones as m;
    if mod_id != m::ID || !game.native_on(m::ID) {
        return;
    }
    crate::game::official_waystones_glue::event(game, event);
}
