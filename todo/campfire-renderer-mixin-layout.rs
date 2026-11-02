// Not ported yet: the Java @Inject(HEAD) on CampfireRenderer.submit (CampfireRendererMixin.cinematic$layout).
// Why it's off: No Horizon function mapped for CampfireRenderer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for CampfireRenderer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   CampfireRenderer.submit -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/CampfireRendererMixin.java:16:
// | @Inject(
// |    method = "submit(Lnet/minecraft/client/renderer/blockentity/state/CampfireRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V",
// |    at = @At("HEAD"),
// |    cancellable = true
// | )
// | private void cinematic$layout(CampfireRenderState fire, PoseStack pose, SubmitNodeCollector collector, CameraRenderState camera, CallbackInfo ci) {
// |    if (StationDisplayLayouts.campfire(fire, pose, collector)) {
// |       ci.cancel();
// |    }
// | }
