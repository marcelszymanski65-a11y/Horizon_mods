// Not ported yet: the Java @Inject(HEAD) on GuiRenderer.render (MixinGuiRenderer.onRenderPre).
// Why it's off: No Horizon function mapped for GuiRenderer.render (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for GuiRenderer.render: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GuiRenderer.render -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinGuiRenderer.java:31:
// | @Inject(method = "render", at = @At("HEAD"))
// | public void onRenderPre(CallbackInfo ci) {
// |    XaeroMinimapCore.beforeIngameGuiRender();
// | }
