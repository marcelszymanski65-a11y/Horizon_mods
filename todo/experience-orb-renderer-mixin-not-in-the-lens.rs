// Not ported yet: the Java @Inject(HEAD) on ExperienceOrbRenderer.submit (ExperienceOrbRendererMixin.cinematic$notInTheLens).
// Why it's off: No Horizon function mapped for ExperienceOrbRenderer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ExperienceOrbRenderer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Camera.position -> (not mapped yet)
//   Minecraft.gameRenderer -> (not mapped yet)
//   Minecraft.getInstance -> `self`. Hooks are methods on Game: self is what Minecraft.getInstance() gave.
//   GameRenderer.mainCamera -> (not mapped yet)
//   ExperienceOrbRenderer.submit -> (not mapped yet)
//   ExperienceOrbRenderState.x -> (not mapped yet)
//   ExperienceOrbRenderState.y -> (not mapped yet)
//   ExperienceOrbRenderState.z -> (not mapped yet)
//   Vec3.<init> -> (not mapped yet)
//   Vec3.distanceToSqr -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/ExperienceOrbRendererMixin.java:20:
// | @Inject(
// |    method = "submit(Lnet/minecraft/client/renderer/entity/state/ExperienceOrbRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V",
// |    at = @At("HEAD"),
// |    cancellable = true
// | )
// | private void cinematic$notInTheLens(ExperienceOrbRenderState orb, PoseStack pose, SubmitNodeCollector features, CameraRenderState camera, CallbackInfo ci) {
// |    if (Scene.active() && new Vec3(orb.x, orb.y, orb.z).distanceToSqr(Minecraft.getInstance().gameRenderer.mainCamera().position()) < 1.0) {
// |       ci.cancel();
// |    }
// | }
