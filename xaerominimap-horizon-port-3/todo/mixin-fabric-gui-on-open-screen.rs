// Not ported yet: the Java @Inject(HEAD) on Gui.setScreen (MixinFabricGui.onOpenScreen).
// Why it's off: No Horizon function mapped for Gui.setScreen (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Gui.setScreen: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.gui -> (not mapped yet)
//   Gui.extractRenderState -> (not mapped yet)
//   Gui.overlay -> (not mapped yet)
//   Gui.screen -> (not mapped yet)
//   Gui.setScreen -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinFabricGui.java:33:
// | @Inject(at = @At("HEAD"), method = "setScreen", cancellable = true)
// | public void onOpenScreen(Screen screen_1, CallbackInfo info) {
// |    if (XaeroMinimapCore.isModLoaded()) {
// |       Screen resultScreen = HudMod.INSTANCE.getEvents().handleGuiOpen(screen_1);
// |       if (screen_1 != resultScreen) {
// |          ((Minecraft)this).gui.setScreen(resultScreen);
// |          info.cancel();
// |       }
// |    }
// | }
