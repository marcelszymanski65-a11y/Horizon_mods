// Not ported yet: the Java @Inject(HEAD) on Model.renderToBuffer (MixinModel.onRender).
// Why it's off: No Horizon function mapped for Model.renderToBuffer (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Model.renderToBuffer: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Model.renderToBuffer -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinModel.java:14:
// | @Inject(at = @At("HEAD"), method = "renderToBuffer")
// | public void onRender(PoseStack matrices, VertexConsumer vertices, int light, int overlay, int color, CallbackInfo info) {
// |    XaeroMinimapCore.onEntityIconsModelRenderDetection((Model)this, vertices, color);
// | }
