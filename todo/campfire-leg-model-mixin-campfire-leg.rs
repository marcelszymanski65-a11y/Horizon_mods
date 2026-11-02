// Not ported yet: the Java @Inject(TAIL) on HumanoidModel.setupAnim (CampfireLegModelMixin.cinematic$campfireLeg).
// Why it's off: No Horizon function mapped for HumanoidModel.setupAnim (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for HumanoidModel.setupAnim: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   HumanoidModel.setupAnim -> (not mapped yet)
//   ModelPart.visible -> (not mapped yet)
//   AvatarRenderState.id -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/CampfireLegModelMixin.java:21:
// | @Inject(method = "setupAnim(Lnet/minecraft/client/renderer/entity/state/HumanoidRenderState;)V", at = @At("TAIL"))
// | private void cinematic$campfireLeg(HumanoidRenderState state, CallbackInfo ci) {
// |    if (state instanceof AvatarRenderState player) {
// |       this.rightLeg.visible = !KneeLayer.sitting(player.id);
// |    }
// | }
