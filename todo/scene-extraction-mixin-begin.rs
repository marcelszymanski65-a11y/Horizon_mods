// Not ported yet: the Java @Inject(HEAD) on GameRenderer.extract (SceneExtractionMixin.cinematic$begin).
// Why it's off: No Horizon function mapped for GameRenderer.extract (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for GameRenderer.extract: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.getInstance -> `self`. Hooks are methods on Game: self is what Minecraft.getInstance() gave.
//   Minecraft.isGameLoadFinished -> (not mapped yet)
//   Minecraft.level -> (not mapped yet)
//   Gui.extractRenderState -> (not mapped yet)
//   GameRenderer.extract -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/SceneExtractionMixin.java:14:
// | @Inject(method = "extract", at = @At("HEAD"))
// | private void cinematic$begin(DeltaTracker tracker, boolean advance, CallbackInfo ci) {
// |    SceneRenderer.beginExtraction();
// | }
