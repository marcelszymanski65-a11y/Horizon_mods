// Not ported yet: the Java @ModifyVariable(HEAD) on PlayerWaystoneManager.deactivateWaystone (MixinPlayerWaystoneManager.onDeactivation).
// Why it's off: No Horizon function mapped for PlayerWaystoneManager.deactivateWaystone (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for PlayerWaystoneManager.deactivateWaystone: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PlayerWaystoneManager.deactivateWaystone -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/waystones/MixinPlayerWaystoneManager.java:12:
// | @ModifyVariable(method = "deactivateWaystone", remap = false, index = 1, at = @At("HEAD"))
// | private static Waystone onDeactivation(Waystone waystone) {
// |    if (HudMod.INSTANCE.getSupportMods() != null) {
// |       HudMod.INSTANCE.getSupportMods().getSupportWaystones().onWaystoneDeactivation(waystone);
// |    }
// |
// |    return waystone;
// | }
