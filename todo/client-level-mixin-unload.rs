// Not ported yet: the Java @Inject(HEAD) on ClientLevel.disconnect (ClientLevelMixin.cinematic$unload).
// Why it's off: No Horizon function mapped for ClientLevel.disconnect (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ClientLevel.disconnect: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientLevel.disconnect -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/ClientLevelMixin.java:12:
// | @Inject(method = "disconnect", at = @At("HEAD"))
// | private void cinematic$unload(CallbackInfo ci) {
// |    CinematicClient.clear();
// | }
