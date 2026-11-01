fn cinematic_event(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    event: &crate::game::native_hooks::NativePanelEvent,
) {
    use crate::game::official_cinematic as m;
    if mod_id != m::ID || !game.native_on(m::ID) {
        return;
    }
    crate::game::official_cinematic_glue::event(game, event);
}
