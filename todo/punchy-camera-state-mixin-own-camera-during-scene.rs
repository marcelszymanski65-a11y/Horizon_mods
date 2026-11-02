// Not ported yet: the Java @Inject(HEAD) on CameraAnimationState.isActive (PunchyCameraStateMixin.cinematic$ownCameraDuringScene).
// Why it's off: No Horizon function mapped for CameraAnimationState.isActive (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for CameraAnimationState.isActive: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   CameraAnimationState.isActive -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/compat/PunchyCameraStateMixin.java:13:
// | @Inject(method = "isActive", at = @At("HEAD"), cancellable = true, require = 0)
// | private static void cinematic$ownCameraDuringScene(CallbackInfoReturnable<Boolean> cir) {
// |    if (Scene.active()) {
// |       cir.setReturnValue(false);
// |    }
// | }
