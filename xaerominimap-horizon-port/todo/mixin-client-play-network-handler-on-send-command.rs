// Not ported yet: the Java @Inject(HEAD) on ClientPacketListener.sendCommand (MixinClientPlayNetworkHandler.onSendCommand).
// Why it's off: No Horizon function mapped for ClientPacketListener.sendCommand (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ClientPacketListener.sendCommand: run `match`.
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
// Decompiled Java, xaero/common/mixin/MixinClientPlayNetworkHandler.java:118:
// | @Inject(at = @At("HEAD"), method = "sendCommand(Ljava/lang/String;)V", cancellable = true)
// | public void onSendCommand(String string_1, CallbackInfo info) {
// |    if (XaeroMinimapCore.onLocalPlayerCommand(string_1)) {
// |       info.cancel();
// |    }
// | }
