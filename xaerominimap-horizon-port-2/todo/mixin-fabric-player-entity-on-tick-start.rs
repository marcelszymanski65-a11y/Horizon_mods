// Not ported yet: the Java @Inject(HEAD) on Player.tick (MixinFabricPlayerEntity.onTickStart).
// Why it's off: No Horizon function mapped for Player.tick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Player.tick: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Player.tick -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinFabricPlayerEntity.java:13:
// | @Inject(at = @At("HEAD"), method = "tick")
// | public void onTickStart(CallbackInfo info) {
// |    if (XaeroMinimapServerCore.isModLoaded()) {
// |       HudMod.INSTANCE.getCommonEvents().handlePlayerTickStart((Player)this);
// |    }
// | }
