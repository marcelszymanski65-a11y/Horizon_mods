// Not ported yet: the Java @Inject(HEAD) on Hud.extractEffects (MixinOptionalInGameHud.onRenderStatusEffectOverlay).
// Why it's off: No Horizon function mapped for Hud.extractEffects (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Hud.extractEffects: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Hud.extractCrosshair -> (not mapped yet)
//   Hud.extractEffects -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinOptionalInGameHud.java:26:
// | @Inject(at = @At("HEAD"), method = "extractEffects", cancellable = true)
// | public void onRenderStatusEffectOverlay(GuiGraphicsExtractor guiGraphics, DeltaTracker deltaTracker, CallbackInfo info) {
// |    if (XaeroMinimapCore.onRenderStatusEffectOverlay(guiGraphics)) {
// |       info.cancel();
// |    }
// | }
