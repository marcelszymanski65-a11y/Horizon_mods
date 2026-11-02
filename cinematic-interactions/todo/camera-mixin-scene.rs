// Not ported yet: the Java @Inject(TAIL) on Camera.alignWithEntity (CameraMixin.cinematic$scene).
// Why it's off: `horizon_client_render::camera::Camera::align_with_entity` has a closure in its body: horizon-mixins doesn't rewrite such bodies with RETURN. A HEAD hook on it, or a RETURN on a function it calls, may work instead.
// horizon-mixins kind: RETURN on `horizon_client_render::camera::Camera::align_with_entity`
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Camera.alignWithEntity -> horizon_client_render::camera::Camera::align_with_entity (suggested, high)
//   Camera.calculateFov -> horizon_client_render::camera::Camera::calculate_fov (suggested, high)
//   Camera.position -> (not mapped yet)
//   Camera.rotation -> (not mapped yet)
//   Camera.xRot -> (not mapped yet)
//   Camera.yRot -> (not mapped yet)
//   Minecraft.getCameraEntity -> (not mapped yet)
//   Minecraft.getInstance -> `self`. Hooks are methods on Game: self is what Minecraft.getInstance() gave.
//   LocalPlayer.getXRot -> (not mapped yet)
//   LocalPlayer.getYRot -> (not mapped yet)
//   LocalPlayer.isPassenger -> (not mapped yet)
//   LocalPlayer.xRotO -> (not mapped yet)
//   LocalPlayer.yRotO -> (not mapped yet)
//   Mth.lerp -> horizon_world::stepped_interpolation::lerp (suggested, high)
//   Mth.rotLerp -> horizon_world::stepped_interpolation::rot_lerp (suggested, high)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/CameraMixin.java:34:
// | @Inject(method = "alignWithEntity", at = @At("TAIL"))
// | private void cinematic$scene(float partialTick, CallbackInfo ci) {
// |    Camera self = (Camera)this;
// |    float yaw = self.yRot();
// |    float pitch = self.xRot();
// |    if (Scene.active() && Minecraft.getInstance().getCameraEntity() instanceof LocalPlayer player && !player.isPassenger()) {
// |       yaw = Mth.rotLerp(partialTick, player.yRotO, player.getYRot());
// |       pitch = Mth.lerp(partialTick, player.xRotO, player.getXRot());
// |    }
// |
// |    CameraRig.Pose pose = CameraRig.override(self.position(), yaw, pitch);
// |    if (pose != null) {
// |       this.setRotation(pose.yaw(), pose.pitch());
// |       self.rotation().rotateZ((float)Math.toRadians(-pose.roll()));
// |       this.up.set(0.0F, 1.0F, 0.0F).rotate(self.rotation());
// |       this.left.set(-1.0F, 0.0F, 0.0F).rotate(self.rotation());
// |       this.setPosition(pose.pos());
// |    }
// | }
