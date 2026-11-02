// Not ported yet: the Java @Inject(HEAD) on MinecraftServer.tickServer (MixinFabricMinecraftServer.onTick).
// Why it's off: No Horizon function mapped for MinecraftServer.tickServer (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for MinecraftServer.tickServer: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MinecraftServer.tickServer -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinFabricMinecraftServer.java:14:
// | @Inject(at = @At("HEAD"), method = "tickServer")
// | public void onTick(BooleanSupplier shouldKeepTicking, CallbackInfo info) {
// |    if (this instanceof DedicatedServer) {
// |       if (HudMod.INSTANCE != null) {
// |          HudMod.INSTANCE.tryLoadLaterServer();
// |       }
// |    }
// | }
