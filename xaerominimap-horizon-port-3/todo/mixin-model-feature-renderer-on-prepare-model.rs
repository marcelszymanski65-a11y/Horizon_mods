// Not ported yet: the Java @ModifyVariable(HEAD) on ModelFeatureRenderer.prepareModel (MixinModelFeatureRenderer.onPrepareModel).
// Why it's off: No Horizon function mapped for ModelFeatureRenderer.prepareModel (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for ModelFeatureRenderer.prepareModel: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ModelFeatureRenderer.prepareModel -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinModelFeatureRenderer.java:12:
// | @ModifyVariable(index = 1, method = "prepareModel", at = @At("HEAD"))
// | public Submit<?> onPrepareModel(Submit<?> submit) {
// |    XaeroMinimapCore.onEntityIconsModelPrepareDetection(submit);
// |    return submit;
// | }
