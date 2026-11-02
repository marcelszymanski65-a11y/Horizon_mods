// Not ported yet: the Java @Inject(HEAD) on Gui.setScreen (GuiScreenMixin.cinematic$screen).
// Why it's off: No Horizon function mapped for Gui.setScreen (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Gui.setScreen: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Gui.setScreen -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/GuiScreenMixin.java:13:
// | @Inject(method = "setScreen", at = @At("HEAD"), cancellable = true)
// | private void cinematic$screen(Screen next, CallbackInfo ci) {
// |    if (CinematicClient.screenOpening(next)) {
// |       ci.cancel();
// |    }
// | }
