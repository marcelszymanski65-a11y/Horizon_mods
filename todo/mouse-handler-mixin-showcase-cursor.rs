// Not ported yet: the Java @Inject(HEAD) on MouseHandler.onMove (MouseHandlerMixin.cinematic$showcaseCursor).
// Why it's off: No Horizon function mapped for MouseHandler.onMove (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for MouseHandler.onMove: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.getInstance -> `self`. Hooks are methods on Game: self is what Minecraft.getInstance() gave.
//   Minecraft.gui -> (not mapped yet)
//   MouseHandler.onMove -> (not mapped yet)
//   MouseHandler.onScroll -> hook `wheel`. While playing; set claimed to keep the steps from the hotbar.
//   MouseHandler.turnPlayer -> (not mapped yet)
//   Gui.screen -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/MouseHandlerMixin.java:14:
// | @Inject(method = "onMove", at = @At("HEAD"), cancellable = true)
// | private void cinematic$showcaseCursor(CallbackInfo ci) {
// |    if (Showcase.ENABLED) {
// |       ci.cancel();
// |    }
// | }
