// Not ported yet: the Java @Inject(RETURN) on Hud.extractEffects (MixinOptionalInGameHud.postRenderStatusEffectOverlay).
// Why it's off: No Horizon function mapped for Hud.extractEffects (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for Hud.extractEffects: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Hud.extractCrosshair -> (not mapped yet)
//   Hud.extractEffects -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinOptionalInGameHud.java:21:
// | @Inject(at = @At("RETURN"), method = "extractEffects")
// | public void postRenderStatusEffectOverlay(GuiGraphicsExtractor guiGraphics, DeltaTracker deltaTracker, CallbackInfo info) {
// |    XaeroMinimapCore.onRenderStatusEffectOverlayPost(guiGraphics);
// | }
