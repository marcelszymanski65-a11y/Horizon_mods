// Not ported yet: the Java @Inject(HEAD) on LevelRenderer.submitFeatures (LevelRendererMixin.cinematic$outline).
// Why it's off: No Horizon function mapped for LevelRenderer.submitFeatures (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for LevelRenderer.submitFeatures: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LevelRenderer.submitFeatures -> (not mapped yet)
//   LevelRenderState.blockOutlineRenderState -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/LevelRendererMixin.java:22:
// | @Inject(method = "submitFeatures", at = @At("HEAD"))
// | private void cinematic$outline(LevelRenderState state, SubmitNodeCollector collector, boolean outline, CallbackInfo ci) {
// |    if (Scene.active()) {
// |       state.blockOutlineRenderState = null;
// |    }
// | }
