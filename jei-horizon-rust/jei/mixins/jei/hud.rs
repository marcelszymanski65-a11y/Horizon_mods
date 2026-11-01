fn jei_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::official_jei as mod_rt;
    if !game.native_on(mod_rt::ID) || !game.native_playing() {
        return;
    }
    let (x, y, z) = game.native_player_pos();
    let fps = game.native_fps();
    let text = mod_rt::format_overlay(x, y, z, fps);
    if !text.is_empty() {
        corners.push_top_left(batch, &text, [255, 255, 255, 255]);
    }
}
