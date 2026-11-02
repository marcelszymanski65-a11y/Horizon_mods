// Not ported yet: the Java @Inject(RETURN) on ServerPlayer.openMenu (ServerPlayerMixin.cinematic$openedMenu).
// Why it's off: No Horizon function mapped for ServerPlayer.openMenu (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for ServerPlayer.openMenu: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ServerPlayer.containerMenu -> (not mapped yet)
//   ServerPlayer.openMenu -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/ServerPlayerMixin.java:15:
// | @Inject(method = "openMenu", at = @At("RETURN"))
// | private void cinematic$openedMenu(MenuProvider provider, CallbackInfoReturnable<OptionalInt> cir) {
// |    ServerPlayer self = (ServerPlayer)this;
// |    if (((OptionalInt)cir.getReturnValue()).isPresent() && self.containerMenu instanceof CraftingMenu menu) {
// |       TableActions.intoWindow(self, menu);
// |    }
// | }
