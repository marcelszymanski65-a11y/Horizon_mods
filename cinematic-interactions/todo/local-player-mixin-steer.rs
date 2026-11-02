// Not ported yet: the Java @Inject(TAIL) on LocalPlayer.tick (LocalPlayerMixin.cinematic$steer).
// Why it's off: No Horizon function mapped for LocalPlayer.tick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for LocalPlayer.tick: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LocalPlayer.tick -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/LocalPlayerMixin.java:12:
// | @Inject(method = "tick", at = @At("TAIL"))
// | private void cinematic$steer(CallbackInfo ci) {
// |    Scene.steer((LocalPlayer)this);
// | }
