// Not ported yet: the Java @Inject(HEAD) on ClientPacketListener.sendUnattendedCommand (MixinClientPlayNetworkHandler.onSendUnattendedCommand).
// Why it's off: No Horizon function mapped for ClientPacketListener.sendUnattendedCommand (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ClientPacketListener.sendUnattendedCommand: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientPacketListener.close -> (not mapped yet)
//   ClientPacketListener.handleBlockUpdate -> (not mapped yet)
//   ClientPacketListener.handleChunkBlocksUpdate -> (not mapped yet)
//   ClientPacketListener.handleLevelChunkWithLight -> (not mapped yet)
//   ClientPacketListener.handleLightUpdatePacket -> (not mapped yet)
//   ClientPacketListener.handleLogin -> (not mapped yet)
//   ClientPacketListener.handleSetSpawn -> (not mapped yet)
//   ClientPacketListener.queueLightRemoval -> (not mapped yet)
//   ClientPacketListener.sendCommand -> (not mapped yet)
//   ClientPacketListener.sendUnattendedCommand -> (not mapped yet)
//   PacketUtils.ensureRunningOnSameThread -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinClientPlayNetworkHandler.java:125:
// | @Inject(at = @At("HEAD"), method = "sendUnattendedCommand(Ljava/lang/String;Lnet/minecraft/client/gui/screens/Screen;)V", cancellable = true)
// | public void onSendUnattendedCommand(String string_1, Screen screen, CallbackInfo info) {
// |    if (XaeroMinimapCore.isModLoaded()) {
// |       if (HudMod.INSTANCE.getEvents().handleClientSendChatEvent(string_1)) {
// |          info.cancel();
// |       }
// |    }
// | }
