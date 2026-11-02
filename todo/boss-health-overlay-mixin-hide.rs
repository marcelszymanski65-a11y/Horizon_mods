// Not ported yet: the Java @Inject(HEAD) on BossHealthOverlay.extractRenderState (BossHealthOverlayMixin.cinematic$hide).
// Why it's off: No Horizon function mapped for BossHealthOverlay.extractRenderState (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for BossHealthOverlay.extractRenderState: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BossHealthOverlay.extractRenderState -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/BossHealthOverlayMixin.java:12:
// | @Inject(method = "extractRenderState", at = @At("HEAD"), cancellable = true)
// | private void cinematic$hide(CallbackInfo ci) {
// |    if (Scene.active()) {
// |       ci.cancel();
// |    }
// | }
