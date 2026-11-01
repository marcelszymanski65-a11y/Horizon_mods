fn jei_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::official_jei as mod_rt;
    mods.push(crate::game::native_hooks::NativeMod {
        id: mod_rt::ID.into(),
        title: format!("{} {}", "Just Enough Items", mod_rt::VERSION),
        description: mod_rt::DESCRIPTION.into(),
        keys: vec![
            ["recipe".into(), "Show recipes".into(), "key.keyboard.r".into()],
            ["uses".into(), "Show uses".into(), "key.keyboard.u".into()],
        ],
    });
}
