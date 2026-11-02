// Not ported yet: the Java @Inject(HEAD) on LevelRenderer.render (MixinOptionalLevelRenderer.onRenderLevel).
// Why it's off: No Horizon function mapped for LevelRenderer.render (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for LevelRenderer.render: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LevelRenderer.render -> (not mapped yet)
//   CameraRenderState.viewRotationMatrix -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinOptionalLevelRenderer.java:16:
// | @Inject(at = @At("HEAD"), method = "render")
// | public void onRenderLevel(
// |    GraphicsResourceAllocator resourceAllocator,
// |    boolean renderOutline,
// |    CameraRenderState cameraState,
// |    GpuBufferSlice terrainFog,
// |    Vector4f fogColor,
// |    boolean shouldRenderSky,
// |    boolean consistentDepthRequired,
// |    CallbackInfo info
// | ) {
// |    XaeroMinimapCore.onWorldModelViewMatrix(cameraState.viewRotationMatrix);
// | }
