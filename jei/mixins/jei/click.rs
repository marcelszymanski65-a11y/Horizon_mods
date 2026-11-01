fn jei_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::official_jei as mod_rt;
    if mod_id != mod_rt::ID {
        return;
    }
    let mut settings = mod_rt::settings(|key| game.native_setting(mod_rt::ID, key));
    settings.click(button);
    mod_rt::set_settings(settings);
    for (key, value) in settings.stored() {
        game.native_set_setting(mod_rt::ID, key, &value);
    }
}
