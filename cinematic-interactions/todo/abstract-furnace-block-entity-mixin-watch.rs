// Not ported yet: the Java @Inject(RETURN) on AbstractFurnaceBlockEntity.serverTick (AbstractFurnaceBlockEntityMixin.cinematic$watch).
// Why it's off: No Horizon function mapped for AbstractFurnaceBlockEntity.serverTick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for AbstractFurnaceBlockEntity.serverTick: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   AbstractFurnaceBlockEntity.serverTick -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/AbstractFurnaceBlockEntityMixin.java:29:
// | @Inject(method = "serverTick", at = @At("RETURN"))
// | private static void cinematic$watch(ServerLevel level, BlockPos pos, BlockState state, AbstractFurnaceBlockEntity furnace, CallbackInfo ci) {
// |    FurnaceWatch.afterTick(level, pos, state, furnace);
// | }
