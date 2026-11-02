// Not ported yet: the Java @Inject(HEAD) on ModelPart.compile (MixinModelPart.onRender).
// Why it's off: No Horizon function mapped for ModelPart.compile (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ModelPart.compile: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ModelPart.compile -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinModelPart.java:14:
// | @Inject(at = @At("HEAD"), method = "compile(Lcom/mojang/blaze3d/vertex/PoseStack$Pose;Lcom/mojang/blaze3d/vertex/VertexConsumer;III)V")
// | public void onRender(Pose matrices, VertexConsumer vertexConsumer, int light, int overlay, int color, CallbackInfo info) {
// |    XaeroMinimapCore.onEntityIconsModelPartRenderDetection((ModelPart)this, color);
// | }
