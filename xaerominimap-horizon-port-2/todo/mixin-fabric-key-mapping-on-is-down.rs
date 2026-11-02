// Not ported yet: the Java @ModifyReturnValue(RETURN) on KeyMapping.isDown (MixinFabricKeyMapping.onIsDown).
// Why it's off: Mapped to a helper call, not a function to inject into.
// horizon-mixins kind: RETURN
// Note: Horizon has no function to inject into here, only the helper `self.native_mapping(name) for vanilla keys; track key hook presses for the mod's own`.
// Note: Target mapping is approximate.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   KeyMapping.isDown -> `self.native_mapping(name) for vanilla keys; track key hook presses for the mod's own`
//
// Decompiled Java, xaero/common/mixin/MixinFabricKeyMapping.java:12:
// | @ModifyReturnValue(method = "isDown", at = @At("RETURN"))
// | public boolean onIsDown(boolean original) throws Exception {
// |    if (!(this instanceof ToggleKeyMapping)) {
// |       return original;
// |    } else {
// |       return XaeroMinimapCore.onToggleKeyIsDown((ToggleKeyMapping)this) ? true : original;
// |    }
// | }
