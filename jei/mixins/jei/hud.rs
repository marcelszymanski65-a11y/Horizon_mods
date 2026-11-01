fn jei_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::official_jei as m;
    use horizon_client::extras::Corner;
    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::TopLeft) {
        return;
    }
    let text = match game.native_player() {
        Some(p) => m::coords_text(p.x, p.y, p.z, p.yaw),
        None => {
            let (fps, _, _) = game.native_frame_rate();
            m::status_text(fps)
        }
    };
    if text.is_empty() {
        return;
    }
    let w = game.native_text_width(&text) + 4.0;
    let (x, y) = corners.place(Corner::TopLeft, w, 12.0);
    game.native_fill(batch, [x, y, w, 12.0], m::BACKGROUND);
    game.native_text(batch, &text, [x + 2.0, y + 2.0], m::INK, true);
}
