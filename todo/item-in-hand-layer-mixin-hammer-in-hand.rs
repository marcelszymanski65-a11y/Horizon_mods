// Not ported yet: the Java @Inject(HEAD) on ItemInHandLayer.submitArmWithItem (ItemInHandLayerMixin.cinematic$hammerInHand).
// Why it's off: No Horizon function mapped for ItemInHandLayer.submitArmWithItem (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ItemInHandLayer.submitArmWithItem: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ItemInHandLayer.submitArmWithItem -> (not mapped yet)
//   AvatarRenderState.id -> (not mapped yet)
//   HumanoidArm.RIGHT -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/ItemInHandLayerMixin.java:19:
// | @Inject(method = "submitArmWithItem", at = @At("HEAD"), cancellable = true)
// | private void cinematic$hammerInHand(
// |    ArmedEntityRenderState entity,
// |    ItemStackRenderState item,
// |    ItemStack stack,
// |    HumanoidArm arm,
// |    PoseStack pose,
// |    SubmitNodeCollector features,
// |    int light,
// |    CallbackInfo ci
// | ) {
// |    if (arm == HumanoidArm.RIGHT && (entity instanceof AvatarRenderState player ? PropLayer.hammer(player.id) : 0.0) > 0.001) {
// |       ci.cancel();
// |    }
// | }
