// Not ported yet: the Java @Inject(INVOKE) on Minecraft.renderFrame (MinecraftMixin.cinematic$frame).
// Why it's off: No Horizon function mapped for Minecraft.renderFrame (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for Minecraft.renderFrame: run `match`.
// Note: The call it hooks (GameRenderer.update) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.continueAttack -> horizon_client::first_person::Clicks::continue_attack (suggested, high)
//   Minecraft.renderFrame -> (not mapped yet)
//   Minecraft.startAttack -> horizon_client::first_person::Clicks::start_attack (suggested, high)
//   Minecraft.startUseItem -> horizon_client::first_person::Clicks::start_use_item (suggested, high)
//   Minecraft.tick -> hook `tick`. Also runs in the server's Game: check native_window().
//   GameRenderer.update -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/MinecraftMixin.java:14:
// | @Inject(
// |    method = "renderFrame",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/client/renderer/GameRenderer;update(Lnet/minecraft/client/DeltaTracker;)V")
// | )
// | private void cinematic$frame(CallbackInfo ci) {
// |    Scene.frame();
// | }
