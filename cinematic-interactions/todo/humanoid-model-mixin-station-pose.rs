// Not ported yet: the Java @Inject(TAIL) on HumanoidModel.setupAnim (HumanoidModelMixin.cinematic$stationPose).
// Why it's off: No Horizon function mapped for HumanoidModel.setupAnim (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for HumanoidModel.setupAnim: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   HumanoidModel.setupAnim -> (not mapped yet)
//   ModelPart.xRot -> (not mapped yet)
//   ModelPart.y -> (not mapped yet)
//   ModelPart.yRot -> (not mapped yet)
//   ModelPart.z -> (not mapped yet)
//   ModelPart.zRot -> (not mapped yet)
//   AvatarRenderState.id -> (not mapped yet)
//   HumanoidRenderState.ageInTicks -> (not mapped yet)
//   Mth.cos -> (not mapped yet)
//   Mth.sin -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/HumanoidModelMixin.java:48:
// | @Inject(method = "setupAnim(Lnet/minecraft/client/renderer/entity/state/HumanoidRenderState;)V", at = @At("TAIL"))
// | private void cinematic$stationPose(HumanoidRenderState state, CallbackInfo ci) {
// |    if (state instanceof AvatarRenderState entity) {
// |       StationKind kind = ClientPayloads.atStation(entity.id);
// |       if (kind != null) {
// |          float t = state.ageInTicks * 0.09F + entity.id;
// |          double move = ClientMenus.sinceMove(entity.id);
// |          switch (kind) {
// |             case ANVIL:
// |                this.leftArm.xRot = -1.05F + Mth.sin(t + 2.1F) * 0.04F;
// |                this.leftArm.yRot = 0.28F;
// |                this.leftArm.zRot = -0.06F;
// |                boolean forging = move < 2.05;
// |                this.rightArm.xRot = forging ? AnvilStation.armPitch(move) : -0.98F + Mth.sin(t) * 0.05F;
// |                this.rightArm.yRot = forging ? -0.35F : -0.2F;
// |                this.rightArm.zRot = 0.08F;
// |                break;
// |             case CAMPFIRE:
// |                this.rightLeg.xRot = -2.0F;
// |                this.rightLeg.yRot = 0.12F;
// |                this.rightLeg.zRot = 0.05F;
// |                this.leftLeg.xRot = -1.45F;
// |                this.leftLeg.yRot = -0.22F;
// |                this.leftLeg.zRot = 0.0F;
// |                if (KneeLayer.sitting(entity.id)) {
// |                   this.rightLeg.xRot = -2.1F;
// |                   this.cinematic$leanBack();
// |                   this.rightArm.xRot = -1.5F + Mth.sin(t * 0.6F) * 0.03F;
// |                   this.rightArm.yRot = -0.3F;
// |                   this.rightArm.zRot = 0.05F;
// |                   this.leftArm.xRot = 0.4F;
// |                   this.leftArm.yRot = 0.0F;
// |                   this.leftArm.zRot = -0.25F;
// |                } else {
// |                   this.rightArm.xRot = -1.05F + Mth.sin(t * 0.6F) * 0.03F;
// |                   this.rightArm.yRot = -0.1F;
// |                   this.rightArm.zRot = 0.05F;
// |                   this.leftArm.xRot = 0.35F;
// |                   this.leftArm.yRot = 0.0F;
// |                   this.leftArm.zRot = -0.32F;
// |                }
// |                break;
// |             case LOOM:
// |                float throwing = Mth.sin(t * 2.2F) * 0.2F;
// |                this.rightArm.xRot = -1.0F + Mth.sin(t * 1.1F) * 0.04F;
// |                this.rightArm.yRot = -0.35F + throwing;
// |                this.rightArm.zRot = 0.05F;
// |                this.leftArm.xRot = -1.0F + Mth.sin(t * 1.1F + 1.3F) * 0.04F;
// |                this.leftArm.yRot = 0.35F + throwing;
// |                this.leftArm.zRot = -0.05F;
// |                break;
// |             case GRINDSTONE:
// |                float stroke = move < 2.4 ? GrindstoneStation.armSway(move) : 0.0F;
// |                this.rightArm.xRot = -1.15F + Mth.sin(t * 1.1F) * 0.03F;
// |                this.rightArm.yRot = -0.32F + stroke;
// |                this.rightArm.zRot = 0.05F;
// |                this.leftArm.xRot = -1.2F + Mth.sin(t * 1.1F + 1.3F) * 0.03F;
// |                this.leftArm.yRot = 0.3F + stroke;
// |                this.leftArm.zRot = -0.05F;
// |                break;
// |             case STONECUTTER:
// |                float[] turns = move < 1.05 ? StonecutterStation.armTurns(move) : new float[]{-0.2F, -0.12F};
// |                this.leftArm.xRot = -1.0F + Mth.sin(t + 2.1F) * 0.04F;
// |                this.leftArm.yRot = turns[0];
// |                this.leftArm.zRot = -0.06F;
// |                this.rightArm.xRot = -0.98F + Mth.sin(t) * 0.05F;
// |                this.rightArm.yRot = turns[1];
// |                this.rightArm.zRot = 0.06F;
// |                break;
// |             case FURNACE:
// |             case SMOKER:
// |             case BREWING:
// |                this.rightArm.xRot = -1.32F + Mth.sin(t * 1.3F) * 0.12F;
// |                this.rightArm.yRot = -0.12F + Mth.sin(t * 0.8F) * 0.08F;
// |                this.rightArm.zRot = 0.04F;
// |                this.leftArm.xRot = -1.6F + Mth.sin(t + 2.1F) * 0.03F;
// |                this.leftArm.yRot = 0.32F;
// |                this.leftArm.zRot = -0.05F;
// |                break;
// |             case CHISELED_BOOKSHELF:
// |                if (ShelfBooks.reading(entity.id)) {
// |                   this.rightArm.xRot = -1.05F + Mth.sin(t * 0.6F) * 0.02F;
// |                   this.rightArm.yRot = -0.42F;
// |                   this.rightArm.zRot = 0.05F;
// |                   this.leftArm.xRot = -1.05F + Mth.sin(t * 0.6F + 1.4F) * 0.02F;
// |                   this.leftArm.yRot = 0.42F;
// |                   this.leftArm.zRot = -0.05F;
// |                } else {
// |                   this.rightArm.xRot = -1.4F + Mth.sin(t * 0.9F) * 0.1F;
// |                   this.rightArm.yRot = -0.1F + Mth.sin(t * 0.55F) * 0.18F;
// |                   this.rightArm.zRot = 0.04F;
// |                   this.leftArm.xRot = -0.55F;
// |                   this.leftArm.yRot = 0.1F;
// |                   this.leftArm.zRot = -0.1F;
// |                }
// |                break;
// |             case ENCHANTING:
// |             case BEACON:
// |                this.rightArm.xRot = -1.75F + Mth.sin(t * 0.7F) * 0.06F;
// |                this.rightArm.yRot = -0.32F + Mth.sin(t * 0.5F) * 0.05F;
// |                this.rightArm.zRot = 0.18F;
// |                this.leftArm.xRot = -1.75F + Mth.sin(t * 0.7F + 1.4F) * 0.06F;
// |                this.leftArm.yRot = 0.32F;
// |                this.leftArm.zRot = -0.18F;
// |                break;
// |             default:
// |                this.rightArm.xRot = -0.98F + Mth.sin(t) * 0.07F;
// |                this.rightArm.yRot = -0.16F + Mth.sin(t * 0.7F) * 0.05F;
// |                this.rightArm.zRot = 0.06F;
// |                this.leftArm.xRot = -0.98F + Mth.sin(t + 2.1F) * 0.05F;
// |                this.leftArm.yRot = 0.16F;
// |                this.leftArm.zRot = -0.06F;
// |          }
// |       }
// |    }
// | }
