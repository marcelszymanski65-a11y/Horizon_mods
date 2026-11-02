// Not ported yet: the Java @Inject(TAIL) on PlayerChunkSender.sendChunk (PlayerChunkSenderMixin.cinematic$sent).
// Why it's off: No Horizon function mapped for PlayerChunkSender.sendChunk (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for PlayerChunkSender.sendChunk: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PlayerChunkSender.sendChunk -> (not mapped yet)
//   ServerGamePacketListenerImpl.player -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/PlayerChunkSenderMixin.java:15:
// | @Inject(method = "sendChunk", at = @At("TAIL"))
// | private static void cinematic$sent(ServerGamePacketListenerImpl handler, ServerLevel level, LevelChunk chunk, CallbackInfo ci) {
// |    StationEvents.chunkSent(handler.player, level, chunk);
// | }
