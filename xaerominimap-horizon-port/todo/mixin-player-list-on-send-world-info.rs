// Not ported yet: the Java @Inject(HEAD) on PlayerList.sendLevelInfo (MixinPlayerList.onSendWorldInfo).
// Why it's off: No Horizon function mapped for PlayerList.sendLevelInfo (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for PlayerList.sendLevelInfo: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PlayerList.sendLevelInfo -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinPlayerList.java:14:
// | @Inject(at = @At("HEAD"), method = "sendLevelInfo")
// | public void onSendWorldInfo(ServerPlayer player, ServerLevel world, CallbackInfo info) {
// |    XaeroMinimapServerCore.onServerWorldInfo(player);
// | }
