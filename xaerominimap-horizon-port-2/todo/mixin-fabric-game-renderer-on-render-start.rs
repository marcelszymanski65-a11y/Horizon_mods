// Not ported yet: the Java @Inject(HEAD) on GameRenderer.extract (MixinFabricGameRenderer.onRenderStart).
// Why it's off: No Horizon function mapped for GameRenderer.extract (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for GameRenderer.extract: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GameRenderer.extract -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinFabricGameRenderer.java:20:
// | @Inject(at = @At("HEAD"), method = "extract")
// | public void onRenderStart(DeltaTracker deltaTracker, boolean boolean_1, CallbackInfo info) {
// |    if (XaeroMinimapCore.isModLoaded()) {
// |       ClientEvents fmlEvents = HudMod.INSTANCE.getEvents();
// |       if (fmlEvents != null) {
// |          fmlEvents.handleRenderTickStart();
// |       }
// |    }
// | }
