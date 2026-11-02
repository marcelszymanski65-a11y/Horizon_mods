// Not ported yet: the Java @Inject(RETURN) on GuiRenderer.render (MixinGuiRenderer.onRenderPost).
// Why it's off: No Horizon function mapped for GuiRenderer.render (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for GuiRenderer.render: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GuiRenderer.render -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinGuiRenderer.java:36:
// | @Inject(method = "render", at = @At("RETURN"))
// | public void onRenderPost(CallbackInfo ci) {
// |    XaeroMinimapCore.afterIngameGuiRender();
// | }
