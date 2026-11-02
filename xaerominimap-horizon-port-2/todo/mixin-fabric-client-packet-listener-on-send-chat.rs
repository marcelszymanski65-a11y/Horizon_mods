// Not ported yet: the Java @Inject(HEAD) on ClientPacketListener.sendChat (MixinFabricClientPacketListener.onSendChat).
// Why it's off: No Horizon function mapped for ClientPacketListener.sendChat (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ClientPacketListener.sendChat: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientPacketListener.sendChat -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinFabricClientPacketListener.java:13:
// | @Inject(at = @At("HEAD"), method = "sendChat(Ljava/lang/String;)V", cancellable = true)
// | public void onSendChat(String string_1, CallbackInfo info) {
// |    if (XaeroMinimapCore.isModLoaded()) {
// |       if (HudMod.INSTANCE.getEvents().handleClientSendChatEvent(string_1)) {
// |          info.cancel();
// |       }
// |    }
// | }
