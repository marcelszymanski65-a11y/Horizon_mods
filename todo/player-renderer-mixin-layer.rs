// Not ported yet: the Java @Inject(TAIL) on AvatarRenderer.<init> (PlayerRendererMixin.cinematic$layer).
// Why it's off: No Horizon function mapped for AvatarRenderer.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for AvatarRenderer.<init>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Context.getEquipmentRenderer -> (not mapped yet)
//   AvatarRenderer.<init> -> (not mapped yet)
//   AvatarRenderer.addLayer -> (not mapped yet)
//   AvatarRenderer.getRenderOffset -> (not mapped yet)
//   AvatarRenderState.id -> (not mapped yet)
//   Vec3.add -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/PlayerRendererMixin.java:19:
// | @Inject(method = "<init>", at = @At("TAIL"))
// | private void cinematic$layer(Context context, boolean slim, CallbackInfo ci) {
// |    ((AvatarRenderer)this).addLayer(new PropLayer((AvatarRenderer)this));
// |    ((AvatarRenderer)this).addLayer(new KneeLayer((AvatarRenderer)this, context.getEquipmentRenderer()));
// | }
