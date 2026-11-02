// Not ported yet: the Java @Inject(INVOKE) on GameRenderer.extract (SceneExtractionMixin.cinematic$extract).
// Why it's off: No Horizon function mapped for GameRenderer.extract (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for GameRenderer.extract: run `match`.
// Note: The call it hooks (Gui.extractRenderState) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.getInstance -> `self`. Hooks are methods on Game: self is what Minecraft.getInstance() gave.
//   Minecraft.isGameLoadFinished -> (not mapped yet)
//   Minecraft.level -> (not mapped yet)
//   Gui.extractRenderState -> (not mapped yet)
//   GameRenderer.extract -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/SceneExtractionMixin.java:19:
// | @Inject(method = "extract", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/Gui;extractRenderState(Lnet/minecraft/client/DeltaTracker;ZZ)V"))
// | private void cinematic$extract(DeltaTracker tracker, boolean advance, CallbackInfo ci) {
// |    Minecraft mc = Minecraft.getInstance();
// |    if (mc.isGameLoadFinished() && advance && mc.level != null) {
// |       SceneRenderer.extract();
// |    }
// | }
