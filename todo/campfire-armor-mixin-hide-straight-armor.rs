// Not ported yet: the Java @Inject(RETURN) on HumanoidArmorLayer.getArmorModel (CampfireArmorMixin.cinematic$hideStraightArmor).
// Why it's off: No Horizon function mapped for HumanoidArmorLayer.getArmorModel (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for HumanoidArmorLayer.getArmorModel: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   HumanoidModel.rightLeg -> (not mapped yet)
//   ModelPart.visible -> (not mapped yet)
//   HumanoidArmorLayer.getArmorModel -> (not mapped yet)
//   AvatarRenderState.id -> (not mapped yet)
//   EquipmentSlot.FEET -> (not mapped yet)
//   EquipmentSlot.LEGS -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/CampfireArmorMixin.java:16:
// | @Inject(method = "getArmorModel", at = @At("RETURN"))
// | private void cinematic$hideStraightArmor(HumanoidRenderState state, EquipmentSlot slot, CallbackInfoReturnable<HumanoidModel<?>> cir) {
// |    if ((slot == EquipmentSlot.LEGS || slot == EquipmentSlot.FEET) && state instanceof AvatarRenderState player) {
// |       ((HumanoidModel)cir.getReturnValue()).rightLeg.visible = !KneeLayer.sitting(player.id);
// |    }
// | }
