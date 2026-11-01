fn jei_settings(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::official_jei as mod_rt;
    if mod_id != mod_rt::ID {
        return;
    }
    let game = &*game;
    let settings = mod_rt::settings(|key| game.native_setting(mod_rt::ID, key));
    buttons.extend(settings.labels());
}
