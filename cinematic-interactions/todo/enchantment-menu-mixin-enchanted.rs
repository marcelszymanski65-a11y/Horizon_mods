// Not ported yet: the Java @Inject(RETURN) on EnchantmentMenu.clickMenuButton (EnchantmentMenuMixin.cinematic$enchanted).
// Why it's off: No Horizon function mapped for EnchantmentMenu.clickMenuButton (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for EnchantmentMenu.clickMenuButton: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EnchantmentMenu.clickMenuButton -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/EnchantmentMenuMixin.java:14:
// | @Inject(method = "clickMenuButton", at = @At("RETURN"))
// | private void cinematic$enchanted(Player player, int button, CallbackInfoReturnable<Boolean> cir) {
// |    if ((Boolean)cir.getReturnValue() && player instanceof ServerPlayer serverPlayer) {
// |       EnchantActions.onEnchant(serverPlayer);
// |    }
// | }
