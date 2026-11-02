// Not ported yet: the Java @Inject(HEAD) on LocalPlayer.respawn (MixinClientPlayerEntity.onTickStart).
// Why it's off: No Horizon function mapped for LocalPlayer.respawn (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for LocalPlayer.respawn: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LocalPlayer.respawn -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinClientPlayerEntity.java:13:
// | @Inject(at = @At("HEAD"), method = "respawn")
// | public void onTickStart(CallbackInfo info) {
// |    XaeroMinimapCore.beforeRespawn((Player)this);
// | }
