// Not ported yet: the Java @Inject(TAIL) on PlayerList.placeNewPlayer (MixinFabricPlayerList.onPlaceNewPlayer).
// Why it's off: No Horizon function mapped for PlayerList.placeNewPlayer (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for PlayerList.placeNewPlayer: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PlayerList.placeNewPlayer -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinFabricPlayerList.java:16:
// | @Inject(at = @At("TAIL"), method = "placeNewPlayer")
// | public void onPlaceNewPlayer(Connection connection, ServerPlayer serverPlayer, CommonListenerCookie commonListenerCookie, CallbackInfo info) {
// |    if (XaeroMinimapServerCore.isModLoaded()) {
// |       HudMod.INSTANCE.getCommonEvents().onPlayerLogIn(serverPlayer);
// |    }
// | }
