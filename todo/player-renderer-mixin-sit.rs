// Not ported yet: the Java @Inject(RETURN) on AvatarRenderer.getRenderOffset (PlayerRendererMixin.cinematic$sit).
// Why it's off: No Horizon function mapped for AvatarRenderer.getRenderOffset (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for AvatarRenderer.getRenderOffset: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Context.getEquipmentRenderer -> (not mapped yet)
//   AvatarRenderer.<init> -> (not mapped yet)
//   AvatarRenderer.addLayer -> (not mapped yet)
//   AvatarRenderer.getRenderOffset -> (not mapped yet)
//   AvatarRenderState.id -> (not mapped yet)
//   Vec3.add -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/PlayerRendererMixin.java:25:
// | @Inject(
// |    method = "getRenderOffset(Lnet/minecraft/client/renderer/entity/state/AvatarRenderState;)Lnet/minecraft/world/phys/Vec3;",
// |    at = @At("RETURN"),
// |    cancellable = true
// | )
// | private void cinematic$sit(AvatarRenderState player, CallbackInfoReturnable<Vec3> cir) {
// |    if (ClientPayloads.atStation(player.id) == StationKind.CAMPFIRE) {
// |       cir.setReturnValue(((Vec3)cir.getReturnValue()).add(0.0, -0.62, 0.0));
// |    }
// | }
