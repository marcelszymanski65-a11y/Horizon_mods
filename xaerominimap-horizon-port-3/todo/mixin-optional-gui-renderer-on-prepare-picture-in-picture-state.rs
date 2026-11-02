// Not ported yet: the Java @ModifyVariable(HEAD) on GuiRenderer.preparePictureInPictureState (MixinOptionalGuiRenderer.onPreparePictureInPictureState).
// Why it's off: No Horizon function mapped for GuiRenderer.preparePictureInPictureState (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for GuiRenderer.preparePictureInPictureState: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Window.getGuiScale -> (not mapped yet)
//   Minecraft.getInstance -> `self`. Hooks are methods on Game: self is what Minecraft.getInstance() gave.
//   Minecraft.getWindow -> (not mapped yet)
//   GuiRenderer.close -> (not mapped yet)
//   GuiRenderer.preparePictureInPictureState -> (not mapped yet)
//   PictureInPictureRenderState.getClass -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinOptionalGuiRenderer.java:26:
// | @ModifyVariable(method = "preparePictureInPictureState", at = @At("HEAD"), index = 1)
// | public <T extends PictureInPictureRenderState> T onPreparePictureInPictureState(T state) {
// |    if (this.xaeroMinimapRenderer == null) {
// |       this.xaeroMinimapRenderer = new MinimapPipRenderer<>(MinimapPipRenderState.class);
// |    }
// |
// |    if (state.getClass() == this.xaeroMinimapRenderer.getRenderStateClass()) {
// |       this.xaeroMinimapRenderer
// |          .prepare((MinimapPipRenderState)state, this.renderState, this.featureRenderDispatcher, Minecraft.getInstance().getWindow().getGuiScale());
// |       return state;
// |    }
// |
// |    if (this.xaeroMinimapEnlargedRenderer == null) {
// |       this.xaeroMinimapEnlargedRenderer = new MinimapPipRenderer<>(MinimapPipRenderState.Enlarged.class);
// |    }
// |
// |    if (state.getClass() == this.xaeroMinimapEnlargedRenderer.getRenderStateClass()) {
// |       this.xaeroMinimapEnlargedRenderer
// |          .prepare((MinimapPipRenderState.Enlarged)state, this.renderState, this.featureRenderDispatcher, Minecraft.getInstance().getWindow().getGuiScale());
// |    }
// |
// |    return state;
// | }
