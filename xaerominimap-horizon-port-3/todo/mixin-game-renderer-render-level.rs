// Not ported yet: the Java @ModifyArg(INVOKE) on GameRenderer.renderLevel (MixinGameRenderer.renderLevel).
// Why it's off: No Horizon function mapped for GameRenderer.renderLevel (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for GameRenderer.renderLevel: run `match`.
// Note: The call it hooks (ProjectionMatrixBuffer.getBuffer) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GameRenderer.renderLevel -> (not mapped yet)
//   ProjectionMatrixBuffer.getBuffer -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinGameRenderer.java:12:
// | @ModifyArg(
// |    method = "renderLevel",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/ProjectionMatrixBuffer;getBuffer(Lorg/joml/Matrix4f;)Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;"
// |    )
// | )
// | public Matrix4f renderLevel(Matrix4f perspectiveMatrix) {
// |    XaeroMinimapCore.onProjectionMatrix(perspectiveMatrix);
// |    return perspectiveMatrix;
// | }
