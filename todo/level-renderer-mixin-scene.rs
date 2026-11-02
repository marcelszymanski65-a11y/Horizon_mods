// Not ported yet: the Java @Inject(TAIL) on LevelRenderer.submitFeatures (LevelRendererMixin.cinematic$scene).
// Why it's off: No Horizon function mapped for LevelRenderer.submitFeatures (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for LevelRenderer.submitFeatures: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LevelRenderer.submitFeatures -> (not mapped yet)
//   LevelRenderState.blockOutlineRenderState -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/LevelRendererMixin.java:29:
// | @Inject(method = "submitFeatures", at = @At("TAIL"))
// | private void cinematic$scene(LevelRenderState state, SubmitNodeCollector collector, boolean outline, CallbackInfo ci) {
// |    SceneRenderer.append(state, this.submitNodeStorage);
// | }
