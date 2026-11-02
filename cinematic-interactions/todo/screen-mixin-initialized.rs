// Not ported yet: the Java @Inject(TAIL) on Screen.init (ScreenMixin.cinematic$initialized).
// Why it's off: No Horizon function mapped for Screen.init (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for Screen.init: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Screen.init -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/ScreenMixin.java:13:
// | @Inject(method = "init(II)V", at = @At("TAIL"))
// | private void cinematic$initialized(int width, int height, CallbackInfo ci) {
// |    Showcase.onScreen((Screen)this);
// |    ClientOnlySmoke.onScreen((Screen)this);
// | }
