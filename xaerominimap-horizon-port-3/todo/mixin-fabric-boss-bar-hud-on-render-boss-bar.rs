// Not ported yet: the Java @Inject(HEAD) on BossHealthOverlay.extractBar (MixinFabricBossBarHud.onRenderBossBar).
// Why it's off: No Horizon function mapped for BossHealthOverlay.extractBar (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for BossHealthOverlay.extractBar: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BossHealthOverlay.extractBar -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinFabricBossBarHud.java:14:
// | @Inject(at = @At("HEAD"), method = "extractBar")
// | public void onRenderBossBar(GuiGraphicsExtractor guiGraphics, int x, int y, BossEvent bossBar, CallbackInfo info) {
// |    XaeroMinimapCore.onBossHealthRender(y + 19);
// | }
