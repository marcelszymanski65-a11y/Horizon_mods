fn waystones_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::official_waystones as mod_rt;
    mods.push(crate::game::native_hooks::NativeMod {
        id: mod_rt::ID.into(),
        title: format!("{} {}", "Waystones", mod_rt::VERSION),
        description: mod_rt::DESCRIPTION.into(),
        keys: vec![
            ["recipe".into(), "Show recipes".into(), "key.keyboard.r".into()],
            ["uses".into(), "Show uses".into(), "key.keyboard.u".into()],
        ],
    });
}
