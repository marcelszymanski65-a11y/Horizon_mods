// Not ported yet: the Java @Inject(RETURN) on BrewingStandBlockEntity.serverTick (BrewingStandBlockEntityMixin.cinematic$watch).
// Why it's off: No Horizon function mapped for BrewingStandBlockEntity.serverTick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for BrewingStandBlockEntity.serverTick: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BaseContainerBlockEntity.<init> -> (not mapped yet)
//   BrewingStandBlockEntity.serverTick -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/BrewingStandBlockEntityMixin.java:35:
// | @Inject(method = "serverTick", at = @At("RETURN"))
// | private static void cinematic$watch(ServerLevel level, BlockPos pos, BlockState state, BrewingStandBlockEntity stand, CallbackInfo ci) {
// |    BrewingWatch.afterTick(level, pos, state, stand);
// | }
