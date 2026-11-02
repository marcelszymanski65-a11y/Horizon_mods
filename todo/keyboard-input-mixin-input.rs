// Not ported yet: the Java @Inject(TAIL) on KeyboardInput.tick (KeyboardInputMixin.cinematic$input).
// Why it's off: No Horizon function mapped for KeyboardInput.tick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for KeyboardInput.tick: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   KeyboardInput.tick -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/KeyboardInputMixin.java:13:
// | @Inject(method = "tick", at = @At("TAIL"))
// | private void cinematic$input(CallbackInfo ci) {
// |    Scene.movementInput((ClientInput)this);
// | }
