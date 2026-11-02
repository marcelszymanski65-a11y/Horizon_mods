// Not ported yet: the Java @Inject(HEAD) on ServerLevel.updateNeighborsAt (ServerLevelMixin.cinematic$neighbours).
// Why it's off: `horizon_world::level::Behaviours::update_neighbors_at` is generic: horizon-mixins rejects generic targets.
// horizon-mixins kind: HEAD on `horizon_world::level::Behaviours::update_neighbors_at`
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ServerLevel.updateNeighborsAt -> horizon_world::level::Behaviours::update_neighbors_at (suggested, high)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/ServerLevelMixin.java:14:
// | @Inject(method = "updateNeighborsAt(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;)V", at = @At("HEAD"))
// | private void cinematic$neighbours(BlockPos pos, Block block, CallbackInfo ci) {
// |    StationEvents.neighbourNotify((ServerLevel)this, pos);
// | }
