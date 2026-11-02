// Not ported yet: the Java @Inject(HEAD) on GuiRenderer.close (MixinOptionalGuiRenderer.onClose).
// Why it's off: No Horizon function mapped for GuiRenderer.close (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for GuiRenderer.close: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Window.getGuiScale -> (not mapped yet)
//   Minecraft.getInstance -> `self`. Hooks are methods on Game: self is what Minecraft.getInstance() gave.
//   Minecraft.getWindow -> (not mapped yet)
//   GuiRenderer.close -> (not mapped yet)
//   GuiRenderer.preparePictureInPictureState -> (not mapped yet)
//   PictureInPictureRenderState.getClass -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinOptionalGuiRenderer.java:50:
// | @Inject(method = "close", at = @At("HEAD"))
// | public void onClose(CallbackInfo ci) {
// |    if (this.xaeroMinimapRenderer != null) {
// |       this.xaeroMinimapRenderer.close();
// |    }
// |
// |    if (this.xaeroMinimapEnlargedRenderer != null) {
// |       this.xaeroMinimapEnlargedRenderer.close();
// |    }
// | }
