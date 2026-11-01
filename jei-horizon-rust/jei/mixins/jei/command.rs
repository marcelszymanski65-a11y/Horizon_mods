fn jei_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::official_jei as mod_rt;
    if *ran || !game.native_on(mod_rt::ID) {
        return;
    }
    if mod_rt::handle_command(command) {
        *ran = true;
    }
}
