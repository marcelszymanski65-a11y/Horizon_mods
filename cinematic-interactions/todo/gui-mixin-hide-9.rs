// Not ported yet: the Java @Inject(HEAD) on Hud.extractDemoOverlay (GuiMixin.cinematic$hide).
// Why it's off: No Horizon function mapped for Hud.extractDemoOverlay (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Hud.extractDemoOverlay: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Hud.extractCameraOverlays -> (not mapped yet)
//   Hud.extractCrosshair -> (not mapped yet)
//   Hud.extractDemoOverlay -> (not mapped yet)
//   Hud.extractEffects -> (not mapped yet)
//   Hud.extractHotbarAndDecorations -> (not mapped yet)
//   Hud.extractOverlayMessage -> (not mapped yet)
//   Hud.extractScoreboardSidebar -> (not mapped yet)
//   Hud.extractSelectedItemName -> (not mapped yet)
//   Hud.extractSleepOverlay -> (not mapped yet)
//   Hud.extractTitle -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/GuiMixin.java:14:
// |          "extractCrosshair",
// |          "extractHotbarAndDecorations",
// |          "extractEffects",
// |          "extractCameraOverlays",
// |          "extractOverlayMessage",
// |          "extractTitle",
// |          "extractSelectedItemName",
// |          "extractScoreboardSidebar",
// |          "extractDemoOverlay",
// |          "extractSleepOverlay"
// |    },
// |    at = @At("HEAD"),
// |    cancellable = true
// | )
// | private void cinematic$hide(CallbackInfo ci) {
// |    if (Scene.active()) {
// |       ci.cancel();
// |    }
// | }
