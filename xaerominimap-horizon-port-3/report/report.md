# Horizon port report: Xaero's Minimap

`xaerominimap` 26.6.2 · xaerominimap-fabric-26.3-26.6.2.jar · 678 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 3 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 2 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 41 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 36 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 0 | Horizon has nothing this can attach to yet. |

## Entry points

- `client`: xaero.minimap.XaeroMinimapFabric
- `server`: xaero.minimap.XaeroMinimapFabric
- `modmenu`: xaero.common.XaeroHudModMenu

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| KeyMappingHelper.registerKeyMapping | `PlatformContextLoaderClientOnlyFabric` | hook `key` | auto | Mojang-named variant of KeyBindingHelper. |
| HudElementRegistry.attachElementAfter | `ModClientEventsFabric` | hook `hud` | auto |  |

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| mixin into AbstractClientPlayer with no injectors (interfaces or @Shadow only) | `MixinAbstractClientPlayerEntity` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on ChatListener.handleDisguisedChatMessage | `MixinChatListener.onHandleDisguisedChatMessag` | HEAD | needs-mapping | No Horizon function known for ChatListener.handleDisguisedChatMessage: run `match`. |
| @Inject(INVOKE) on ChatListener.handleSystemMessage at Lnet/minecraft/client/gui/components/ChatComponent;addServerSystemMessage(Lnet/minecraft/network/chat/Component;)V | `MixinChatListener.onHandleSystemChat` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for ChatListener.handleSystemMessage: run `match`. The call it hooks (ChatComponent.addServerSystemMessage) isn't mapped to a Rust function yet. |
| mixin into ClientboundSectionBlocksUpdatePacket with no injectors (interfaces or @Shadow only) | `MixinChunkDeltaUpdateS2CPacket` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(INVOKE) on ClientPacketListener.handleChunkBlocksUpdate at Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V | `MixinClientPlayNetworkHandler.onOnChunkDeltaUpdate` | AFTER_CALL | needs-mapping | No Horizon function known for ClientPacketListener.handleChunkBlocksUpdate: run `match`. The call it hooks (PacketUtils.ensureRunningOnSameThread) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on ClientPacketListener.handleLevelChunkWithLight at Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V | `MixinClientPlayNetworkHandler.onHandleLevelChunkWithLight` | AFTER_CALL | needs-mapping | No Horizon function known for ClientPacketListener.handleLevelChunkWithLight: run `match`. The call it hooks (PacketUtils.ensureRunningOnSameThread) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on ClientPacketListener.handleLightUpdatePacket at Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V | `MixinClientPlayNetworkHandler.onHandleLightUpdatePacket` | AFTER_CALL | needs-mapping | No Horizon function known for ClientPacketListener.handleLightUpdatePacket: run `match`. The call it hooks (PacketUtils.ensureRunningOnSameThread) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on ClientPacketListener.queueLightRemoval | `MixinClientPlayNetworkHandler.onQueueLightRemoval` | HEAD | needs-mapping | No Horizon function known for ClientPacketListener.queueLightRemoval: run `match`. |
| @Inject(INVOKE) on ClientPacketListener.handleBlockUpdate at Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V | `MixinClientPlayNetworkHandler.onOnBlockUpdate` | AFTER_CALL | needs-mapping | No Horizon function known for ClientPacketListener.handleBlockUpdate: run `match`. The call it hooks (PacketUtils.ensureRunningOnSameThread) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on ClientPacketListener.handleSetSpawn at Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V | `MixinClientPlayNetworkHandler.onOnPlayerSpawnPosition` | AFTER_CALL | needs-mapping | No Horizon function known for ClientPacketListener.handleSetSpawn: run `match`. The call it hooks (PacketUtils.ensureRunningOnSameThread) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on ClientPacketListener.handleLogin at Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V | `MixinClientPlayNetworkHandler.onOnGameJoin` | AFTER_CALL | needs-mapping | No Horizon function known for ClientPacketListener.handleLogin: run `match`. The call it hooks (PacketUtils.ensureRunningOnSameThread) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on ClientPacketListener.close | `MixinClientPlayNetworkHandler.onClose` | HEAD | needs-mapping | No Horizon function known for ClientPacketListener.close: run `match`. |
| @Inject(HEAD) on ClientPacketListener.sendCommand | `MixinClientPlayNetworkHandler.onSendCommand` | HEAD | needs-mapping | No Horizon function known for ClientPacketListener.sendCommand: run `match`. |
| @Inject(HEAD) on ClientPacketListener.sendUnattendedCommand | `MixinClientPlayNetworkHandler.onSendUnattendedCommand` | HEAD | needs-mapping | No Horizon function known for ClientPacketListener.sendUnattendedCommand: run `match`. |
| @Inject(HEAD) on LocalPlayer.respawn | `MixinClientPlayerEntity.onTickStart` | HEAD | needs-mapping | No Horizon function known for LocalPlayer.respawn: run `match`. |
| mixin into ClientLevel with no injectors (interfaces or @Shadow only) | `MixinClientWorld` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on BossHealthOverlay.extractBar | `MixinFabricBossBarHud.onRenderBossBar` | HEAD | needs-mapping | No Horizon function known for BossHealthOverlay.extractBar: run `match`. |
| @Inject(INVOKE) on ChatListener.showMessageToPlayer at Lnet/minecraft/client/gui/components/ChatComponent;addPlayerMessage(Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/MessageSignature;Lnet/minecraft/client/multiplayer/chat/GuiMessageTag;)V | `MixinFabricChatListener.onShowMessageToPlayer` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for ChatListener.showMessageToPlayer: run `match`. The call it hooks (ChatComponent.addPlayerMessage) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on ClientPacketListener.sendChat | `MixinFabricClientPacketListener.onSendChat` | HEAD | needs-mapping | No Horizon function known for ClientPacketListener.sendChat: run `match`. |
| @Inject(HEAD) on GameRenderer.extract | `MixinFabricGameRenderer.onRenderStart` | HEAD | needs-mapping | No Horizon function known for GameRenderer.extract: run `match`. |
| @Inject(RETURN) on Gui.extractRenderState | `MixinFabricGui.onRenderGuiEnd` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for Gui.extractRenderState: run `match`. |
| @Inject(HEAD) on Gui.setScreen | `MixinFabricGui.onOpenScreen` | HEAD | needs-mapping | No Horizon function known for Gui.setScreen: run `match`. |
| @ModifyReturnValue(RETURN) on KeyMapping.isDown | `MixinFabricKeyMapping.onIsDown` | RETURN on `self.native_mapping(name) for vanilla keys; track key hook presses for the mod's own` | needs-translation | Horizon has no function to inject into here, only the helper `self.native_mapping(name) for vanilla keys; track key hook presses for the mod's own`. Target mapping is approximate. |
| @Inject(HEAD) on Minecraft.tick | `MixinFabricMinecraftClient.onTickStart` | HEAD on hook `tick` | auto |  |
| @Inject(HEAD) on Minecraft.disconnect | `MixinFabricMinecraftClient.onDisconnect` | HEAD | needs-mapping | No Horizon function known for Minecraft.disconnect: run `match`. |
| @Inject(HEAD) on Minecraft.setLevel | `MixinFabricMinecraftClient.onJoinWorld` | HEAD | needs-mapping | No Horizon function known for Minecraft.setLevel: run `match`. |
| @Inject(HEAD) on MinecraftServer.tickServer | `MixinFabricMinecraftServer.onTick` | HEAD | needs-mapping | No Horizon function known for MinecraftServer.tickServer: run `match`. |
| @Inject(HEAD) on Player.tick | `MixinFabricPlayerEntity.onTickStart` | HEAD | needs-mapping | No Horizon function known for Player.tick: run `match`. |
| @Inject(TAIL) on PlayerList.placeNewPlayer | `MixinFabricPlayerList.onPlaceNewPlayer` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for PlayerList.placeNewPlayer: run `match`. |
| @Inject(RETURN) on TextureAtlas.upload | `MixinFabricSpriteAtlasTexture.onUpload` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for TextureAtlas.upload: run `match`. |
| @ModifyArg(INVOKE) on GameRenderer.renderLevel at Lnet/minecraft/client/renderer/ProjectionMatrixBuffer;getBuffer(Lorg/joml/Matrix4f;)Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice; | `MixinGameRenderer.renderLevel` | MODIFY_ARG | needs-mapping | No Horizon function known for GameRenderer.renderLevel: run `match`. The call it hooks (ProjectionMatrixBuffer.getBuffer) isn't mapped to a Rust function yet. |
| mixin into GuiGraphicsExtractor with no injectors (interfaces or @Shadow only) | `MixinGuiGraphics` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on GuiRenderer.render | `MixinGuiRenderer.onRenderPre` | HEAD | needs-mapping | No Horizon function known for GuiRenderer.render: run `match`. |
| @Inject(RETURN) on GuiRenderer.render | `MixinGuiRenderer.onRenderPost` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for GuiRenderer.render: run `match`. |
| @Inject(RETURN) on LevelStorageAccess.deleteLevel | `MixinLevelStorageAccess.onDeleteLevel` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for LevelStorageAccess.deleteLevel: run `match`. |
| mixin into LocalPlayer with no injectors (interfaces or @Shadow only) | `MixinLocalPlayer` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into Minecraft with no injectors (interfaces or @Shadow only) | `MixinMinecraftClient` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into MinecraftServer with no injectors (interfaces or @Shadow only) | `MixinMinecraftServer` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on Model.renderToBuffer | `MixinModel.onRender` | HEAD | needs-mapping | No Horizon function known for Model.renderToBuffer: run `match`. |
| @ModifyVariable(HEAD) on ModelFeatureRenderer.prepareModel | `MixinModelFeatureRenderer.onPrepareModel` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for ModelFeatureRenderer.prepareModel: run `match`. |
| @Inject(HEAD) on ModelPart.compile | `MixinModelPart.onRender` | HEAD | needs-mapping | No Horizon function known for ModelPart.compile: run `match`. |
| @ModifyVariable(HEAD) on GuiRenderer.preparePictureInPictureState | `MixinOptionalGuiRenderer.onPreparePictureInPictureState` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for GuiRenderer.preparePictureInPictureState: run `match`. |
| @Inject(HEAD) on GuiRenderer.close | `MixinOptionalGuiRenderer.onClose` | HEAD | needs-mapping | No Horizon function known for GuiRenderer.close: run `match`. |
| @Inject(HEAD) on Hud.extractCrosshair | `MixinOptionalInGameHud.onRenderCrosshair` | HEAD | needs-mapping | No Horizon function known for Hud.extractCrosshair: run `match`. |
| @Inject(RETURN) on Hud.extractEffects | `MixinOptionalInGameHud.postRenderStatusEffectOverlay` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for Hud.extractEffects: run `match`. |
| @Inject(HEAD) on Hud.extractEffects | `MixinOptionalInGameHud.onRenderStatusEffectOverlay` | HEAD | needs-mapping | No Horizon function known for Hud.extractEffects: run `match`. |
| @Inject(HEAD) on LevelRenderer.render | `MixinOptionalLevelRenderer.onRenderLevel` | HEAD | needs-mapping | No Horizon function known for LevelRenderer.render: run `match`. |
| mixin into PictureInPictureRenderer with no injectors (interfaces or @Shadow only) | `MixinPictureInPictureRenderer` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on PlayerList.sendLevelInfo | `MixinPlayerList.onSendWorldInfo` | HEAD | needs-mapping | No Horizon function known for PlayerList.sendLevelInfo: run `match`. |
| mixin into ServerPlayer with no injectors (interfaces or @Shadow only) | `MixinServerPlayer` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Accessor on SheetedDecalTextureGenerator.textureScale | `MixinSheetedDecalTextureGeneratorAccessor.getTextureScale` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for SheetedDecalTextureGenerator.textureScale: run `match`. |
| mixin into LevelChunk with no injectors (interfaces or @Shadow only) | `MixinWorldChunk` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @ModifyVariable(HEAD) on PlayerWaystoneManager.deactivateWaystone | `MixinPlayerWaystoneManager.onDeactivation` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for PlayerWaystoneManager.deactivateWaystone: run `match`. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends MobEffect | `MinimapStatusEffect` | needs-translation | No mapping for this superclass yet. |
| extends TextureTarget | `ImprovedFramebuffer` | needs-translation | No mapping for this superclass yet. |
| extends SimpleTexture | `MinimapTexture` | needs-translation | No mapping for this superclass yet. |
| extends ConfirmScreen | `GuiClearSet` | needs-translation | No mapping for this superclass yet. |
| extends ConfirmScreen | `GuiDeleteSet` | needs-translation | No mapping for this superclass yet. |
| extends Entry | `GuiInfoDisplayEdit$SelectionList$Entry` | needs-translation | No mapping for this superclass yet. |
| extends ObjectSelectionList | `GuiInfoDisplayEdit$SelectionList` | needs-translation | No mapping for this superclass yet. |
| extends ConfirmScreen | `GuiReset` | needs-translation | No mapping for this superclass yet. |
| extends Entry | `GuiWaypoints$List$WaypointEntry` | needs-translation | No mapping for this superclass yet. |
| extends ObjectSelectionList | `GuiWaypoints$List` | needs-translation | No mapping for this superclass yet. |
| extends EditBox | `GuiWorldTpCommand$1` | needs-translation | No mapping for this superclass yet. |
| extends EditBox | `GuiWorldTpCommand$2` | needs-translation | No mapping for this superclass yet. |
| extends Plain | `MyBigButton` | needs-translation | No mapping for this superclass yet. |
| extends Plain | `MySuperTinyButton` | needs-translation | No mapping for this superclass yet. |
| extends Plain | `TooltipButton` | needs-translation | No mapping for this superclass yet. |
| extends ConfirmScreen | `GuiCategoryEditor$1` | needs-translation | No mapping for this superclass yet. |
| extends Entry | `GuiCategoryEditor$SettingRowList$Entry` | needs-translation | No mapping for this superclass yet. |
| extends ObjectSelectionList | `GuiCategoryEditor$SettingRowList` | needs-translation | No mapping for this superclass yet. |
| extends Plain | `EditorButton` | needs-translation | No mapping for this superclass yet. |
| extends AbstractOptionSliderButton | `EditorSlider` | needs-translation | No mapping for this superclass yet. |
| extends EditBox | `EditorTextField` | needs-translation | No mapping for this superclass yet. |
| extends AbstractSliderButton | `XaeroSliderWidget` | needs-translation | No mapping for this superclass yet. |
| extends Plain | `InfoDisplayCycleButton` | needs-translation | No mapping for this superclass yet. |
| extends PictureInPictureRenderer | `MinimapPipRenderer` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `WaypointTeleport$1` | needs-translation | No general screen hook; simple option pages can use the settings hook. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| registers blocks or items in code | `PlatformContextLoaderCommonFabric` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| 84 asset files, 0 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

15 of 808 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/client/KeyMapping#<init>` | `self.native_mod_key(MOD_ID, name, default)` |
| `net/minecraft/client/KeyMapping#consumeClick` | `key == self.native_mod_key(..) && pressed, in the key hook` |
| `net/minecraft/client/KeyMapping#isDown` | `self.native_mapping(name) for vanilla keys; track key hook presses for the mod's own` |
| `net/minecraft/client/Minecraft#font` | `(implicit)` |
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/Minecraft#tick` | hook `tick` |
| `net/minecraft/client/gui/Font#width` | `self.native_text_width(text)` |
| `net/minecraft/core/Registry#registerForHolder` | content |
| `net/minecraft/core/registries/BuiltInRegistries#ITEM` | content |
| `net/minecraft/world/entity/Entity#getX` | `self.native_player()?.x` |
| `net/minecraft/world/entity/Entity#getXRot` | `self.native_player()?.pitch` |
| `net/minecraft/world/entity/Entity#getY` | `self.native_player()?.y` |
| `net/minecraft/world/entity/Entity#getYRot` | `self.native_player()?.yaw` |
| `net/minecraft/world/entity/Entity#getZ` | `self.native_player()?.z` |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/mojang/blaze3d/ProjectionType#ORTHOGRAPHIC` | field | 4 |
| `com/mojang/blaze3d/pipeline/RenderTarget#getColorTextureView` | method | 3 |
| `com/mojang/blaze3d/pipeline/RenderTarget#getDepthTextureView` | method | 3 |
| `com/mojang/blaze3d/pipeline/RenderTarget#hasDepth` | method | 3 |
| `com/mojang/blaze3d/pipeline/RenderTarget#height` | field | 1 |
| `com/mojang/blaze3d/pipeline/RenderTarget#width` | field | 1 |
| `com/mojang/blaze3d/pipeline/TextureTarget#<init>` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants#UNKNOWN` | field | 2 |
| `com/mojang/blaze3d/platform/InputConstants$Key#getValue` | method | 2 |
| `com/mojang/blaze3d/platform/InputConstants$Type#KEYBOARD` | field | 2 |
| `com/mojang/blaze3d/platform/InputConstants$Type#MOUSE` | field | 1 |
| `com/mojang/blaze3d/platform/Lighting#setupFor` | method | 6 |
| `com/mojang/blaze3d/platform/Lighting$Entry#ITEMS_3D` | field | 2 |
| `com/mojang/blaze3d/platform/Lighting$Entry#ITEMS_FLAT` | field | 6 |
| `com/mojang/blaze3d/platform/NativeImage#fillRect` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getGuiScale` | method | 5 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledHeight` | method | 3 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledWidth` | method | 3 |
| `com/mojang/blaze3d/platform/Window#getHeight` | method | 6 |
| `com/mojang/blaze3d/platform/Window#getScreenHeight` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getScreenWidth` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getWidth` | method | 6 |
| `com/mojang/blaze3d/systems/RenderSystem#bindDefaultUniforms` | method | 2 |
| `com/mojang/blaze3d/systems/RenderSystem#getDevice` | method | 9 |
| `com/mojang/blaze3d/systems/RenderSystem#getModelViewStack` | method | 4 |
| `com/mojang/blaze3d/systems/RenderSystem#getProjectionMatrixBuffer` | method | 3 |
| `com/mojang/blaze3d/systems/RenderSystem#getProjectionType` | method | 3 |
| `com/mojang/blaze3d/systems/RenderSystem#getSamplerCache` | method | 2 |
| `com/mojang/blaze3d/systems/RenderSystem#getSequentialBuffer` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#setProjectionMatrix` | method | 5 |
| `com/mojang/blaze3d/systems/RenderSystem$AutoStorageIndexBuffer#getBuffer` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem$AutoStorageIndexBuffer#type` | method | 1 |
| `com/mojang/blaze3d/systems/SamplerCache#getClampToEdge` | method | 1 |
| `com/mojang/blaze3d/vertex/BufferBuilder#<init>` | method | 1 |
| `com/mojang/blaze3d/vertex/BufferBuilder#addVertex` | method | 2 |
| `com/mojang/blaze3d/vertex/BufferBuilder#build` | method | 3 |
| `com/mojang/blaze3d/vertex/ByteBufferBuilder#<init>` | method | 1 |
| `com/mojang/blaze3d/vertex/DefaultVertexFormat#POSITION` | field | 1 |
| `com/mojang/blaze3d/vertex/DefaultVertexFormat#POSITION_TEX` | field | 2 |
| `com/mojang/blaze3d/vertex/MeshData#close` | method | 1 |
| `com/mojang/blaze3d/vertex/MeshData#drawState` | method | 1 |
| `com/mojang/blaze3d/vertex/MeshData#indexBuffer` | method | 1 |
| `com/mojang/blaze3d/vertex/MeshData#vertexBuffer` | method | 1 |
| `com/mojang/blaze3d/vertex/MeshData$DrawState#indexCount` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#<init>` | method | 3 |
| `com/mojang/blaze3d/vertex/PoseStack#last` | method | 16 |
| `com/mojang/blaze3d/vertex/PoseStack#popPose` | method | 12 |
| `com/mojang/blaze3d/vertex/PoseStack#pushPose` | method | 12 |
| `com/mojang/blaze3d/vertex/PoseStack#scale` | method | 12 |
| `com/mojang/blaze3d/vertex/PoseStack#setIdentity` | method | 2 |
| `com/mojang/blaze3d/vertex/PoseStack#translate` | method | 20 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#<init>` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#normal` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#pose` | method | 14 |
| `com/mojang/blaze3d/vertex/SheetedDecalTextureGenerator#<init>` | method | 1 |
| `com/mojang/blaze3d/vertex/SheetedDecalTextureGenerator#textureScale` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#addVertex` | method | 5 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setColor` | method | 6 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setLineWidth` | method | 2 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setNormal` | method | 2 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setOverlay` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setUv` | method | 5 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setUv1` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setUv2` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setUv3` | method | 1 |
| `net/blay09/mods/waystones/core/PlayerWaystoneManager#deactivateWaystone` | method | 1 |
| `net/minecraft/ChatFormatting#AQUA` | field | 1 |
| `net/minecraft/ChatFormatting#DARK_GRAY` | field | 2 |
| `net/minecraft/ChatFormatting#DARK_GREEN` | field | 1 |
| `net/minecraft/ChatFormatting#GRAY` | field | 1 |
| `net/minecraft/ChatFormatting#GREEN` | field | 1 |
| `net/minecraft/ChatFormatting#RED` | field | 3 |
| `net/minecraft/ChatFormatting#UNDERLINE` | field | 1 |
| `net/minecraft/ChatFormatting#WHITE` | field | 1 |
| `net/minecraft/ChatFormatting#YELLOW` | field | 6 |
| `net/minecraft/IdentifierException#getMessage` | method | 1 |
| `net/minecraft/client/Camera#forwardVector` | method | 3 |
| `net/minecraft/client/Camera#position` | method | 3 |
| `net/minecraft/client/Camera#xRot` | method | 1 |
| `net/minecraft/client/Camera#yRot` | method | 2 |
| `net/minecraft/client/ClientClockManager#getInstance` | method | 1 |
| `net/minecraft/client/DeltaTracker#getGameTimeDeltaPartialTick` | method | 3 |
| `net/minecraft/client/KeyMapping$Category#register` | method | 1 |
| `net/minecraft/client/Minecraft#debugEntries` | field | 1 |
| `net/minecraft/client/Minecraft#disconnect` | method | 1 |
| `net/minecraft/client/Minecraft#gameMode` | field | 1 |
| `net/minecraft/client/Minecraft#gameRenderer` | field | 15 |
| `net/minecraft/client/Minecraft#getAtlasManager` | method | 2 |
| `net/minecraft/client/Minecraft#getBlockColors` | method | 1 |
| `net/minecraft/client/Minecraft#getCameraEntity` | method | 17 |
| `net/minecraft/client/Minecraft#getConnection` | method | 3 |
| `net/minecraft/client/Minecraft#getDeltaTracker` | method | 2 |
| `net/minecraft/client/Minecraft#getEntityRenderDispatcher` | method | 3 |
| `net/minecraft/client/Minecraft#getItemModelResolver` | method | 1 |
| `net/minecraft/client/Minecraft#getModelManager` | method | 1 |
| `net/minecraft/client/Minecraft#getResourceManager` | method | 2 |
| `net/minecraft/client/Minecraft#getSingleplayerServer` | method | 6 |
| `net/minecraft/client/Minecraft#getSoundManager` | method | 3 |
| `net/minecraft/client/Minecraft#getTextureManager` | method | 7 |
| `net/minecraft/client/Minecraft#getWindow` | method | 11 |
| `net/minecraft/client/Minecraft#gui` | field | 32 |
| `net/minecraft/client/Minecraft#isEnforceUnicode` | method | 3 |
| `net/minecraft/client/Minecraft#isPaused` | method | 1 |
| `net/minecraft/client/Minecraft#isSameThread` | method | 2 |
| `net/minecraft/client/Minecraft#level` | field | 22 |
| `net/minecraft/client/Minecraft#mouseHandler` | field | 1 |
| `net/minecraft/client/Minecraft#options` | field | 5 |
| `net/minecraft/client/Minecraft#setLevel` | method | 1 |
| `net/minecraft/client/Minecraft#submit` | method | 1 |
| `net/minecraft/client/MouseHandler#xpos` | method | 1 |
| `net/minecraft/client/MouseHandler#ypos` | method | 1 |
| `net/minecraft/client/OptionInstance#get` | method | 3 |
| `net/minecraft/client/Options#chatVisibility` | method | 1 |
| `net/minecraft/client/Options#fov` | method | 1 |
| `net/minecraft/client/Options#keyPlayerList` | field | 1 |
| `net/minecraft/client/Options#renderDistance` | method | 1 |
| `net/minecraft/client/color/block/BlockColors#getTintSource` | method | 1 |
| `net/minecraft/client/color/block/BlockTintSource#colorInWorld` | method | 1 |
| `net/minecraft/client/gui/Font$DisplayMode#NORMAL` | field | 2 |
| `net/minecraft/client/gui/Gui#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/Gui#hud` | field | 5 |
| `net/minecraft/client/gui/Gui#overlay` | method | 2 |
| `net/minecraft/client/gui/Gui#screen` | method | 15 |
| `net/minecraft/client/gui/Gui#setScreen` | method | 23 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blit` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#centeredText` | method | 18 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fill` | method | 4 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#horizontalLine` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#pose` | method | 4 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#text` | method | 7 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#verticalLine` | method | 1 |
| `net/minecraft/client/gui/Hud#extractCrosshair` | method | 1 |
| `net/minecraft/client/gui/Hud#extractEffects` | method | 2 |
| `net/minecraft/client/gui/Hud#getChat` | method | 3 |
| `net/minecraft/client/gui/Hud#isHidden` | method | 2 |
| `net/minecraft/client/gui/components/AbstractButton#onPress` | method | 1 |
| `net/minecraft/client/gui/components/AbstractOptionSliderButton#<init>` | method | 1 |
| `net/minecraft/client/gui/components/AbstractOptionSliderButton#keyPressed` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSliderButton#<init>` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#active` | field | 3 |
| `net/minecraft/client/gui/components/AbstractWidget#charTyped` | method | 2 |
| `net/minecraft/client/gui/components/AbstractWidget#extractRenderState` | method | 2 |
| `net/minecraft/client/gui/components/AbstractWidget#getHeight` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#getMessage` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#getX` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#getY` | method | 2 |
| `net/minecraft/client/gui/components/AbstractWidget#isFocused` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#isMouseOver` | method | 2 |
| `net/minecraft/client/gui/components/AbstractWidget#keyPressed` | method | 2 |
| `net/minecraft/client/gui/components/AbstractWidget#keyReleased` | method | 2 |
| `net/minecraft/client/gui/components/AbstractWidget#mouseClicked` | method | 2 |
| `net/minecraft/client/gui/components/AbstractWidget#mouseDragged` | method | 2 |
| `net/minecraft/client/gui/components/AbstractWidget#mouseMoved` | method | 2 |
| `net/minecraft/client/gui/components/AbstractWidget#mouseReleased` | method | 2 |
| `net/minecraft/client/gui/components/AbstractWidget#mouseScrolled` | method | 2 |
| `net/minecraft/client/gui/components/AbstractWidget#setFocused` | method | 3 |
| `net/minecraft/client/gui/components/AbstractWidget#setMessage` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#setX` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#setY` | method | 2 |
| `net/minecraft/client/gui/components/AbstractWidget#updateNarration` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#visible` | field | 1 |
| `net/minecraft/client/gui/components/BossHealthOverlay#extractBar` | method | 1 |
| `net/minecraft/client/gui/components/Button#active` | field | 5 |
| `net/minecraft/client/gui/components/Button#builder` | method | 6 |
| `net/minecraft/client/gui/components/Button#onClick` | method | 3 |
| `net/minecraft/client/gui/components/Button#setMessage` | method | 3 |
| `net/minecraft/client/gui/components/Button$Builder#bounds` | method | 6 |
| `net/minecraft/client/gui/components/Button$Builder#build` | method | 6 |
| `net/minecraft/client/gui/components/Button$Plain#<init>` | method | 5 |
| `net/minecraft/client/gui/components/ChatComponent#addClientSystemMessage` | method | 2 |
| `net/minecraft/client/gui/components/ChatComponent#addPlayerMessage` | method | 1 |
| `net/minecraft/client/gui/components/ChatComponent#addRecentChat` | method | 1 |
| `net/minecraft/client/gui/components/ChatComponent#addServerSystemMessage` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#<init>` | method | 7 |
| `net/minecraft/client/gui/components/EditBox#active` | field | 2 |
| `net/minecraft/client/gui/components/EditBox#addFormatter` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#createNarrationMessage` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#deleteChars` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#deleteWords` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#extractRenderState` | method | 4 |
| `net/minecraft/client/gui/components/EditBox#extractWidgetRenderState` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#getValue` | method | 6 |
| `net/minecraft/client/gui/components/EditBox#insertText` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#isFocused` | method | 3 |
| `net/minecraft/client/gui/components/EditBox#mouseClicked` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#moveCursorTo` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#moveCursorToEnd` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#moveCursorToStart` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setCursorPosition` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setEditable` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setFocused` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#setHighlightPos` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setMaxLength` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setResponder` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#setTextColor` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setValue` | method | 5 |
| `net/minecraft/client/gui/components/ObjectSelectionList#<init>` | method | 3 |
| `net/minecraft/client/gui/components/ObjectSelectionList#charTyped` | method | 1 |
| `net/minecraft/client/gui/components/ObjectSelectionList#extractItem` | method | 1 |
| `net/minecraft/client/gui/components/ObjectSelectionList#extractWidgetRenderState` | method | 3 |
| `net/minecraft/client/gui/components/ObjectSelectionList#keyReleased` | method | 1 |
| `net/minecraft/client/gui/components/ObjectSelectionList#mouseClicked` | method | 2 |
| `net/minecraft/client/gui/components/ObjectSelectionList#mouseMoved` | method | 1 |
| `net/minecraft/client/gui/components/ObjectSelectionList#nextFocusPath` | method | 2 |
| `net/minecraft/client/gui/components/ObjectSelectionList#setFocused` | method | 2 |
| `net/minecraft/client/gui/components/ObjectSelectionList#setSelected` | method | 3 |
| `net/minecraft/client/gui/components/ObjectSelectionList#updateWidgetNarration` | method | 2 |
| `net/minecraft/client/gui/components/ObjectSelectionList$Entry#<init>` | method | 3 |
| `net/minecraft/client/gui/components/ObjectSelectionList$Entry#charTyped` | method | 2 |
| `net/minecraft/client/gui/components/ObjectSelectionList$Entry#keyPressed` | method | 2 |
| `net/minecraft/client/gui/components/ObjectSelectionList$Entry#keyReleased` | method | 2 |
| `net/minecraft/client/gui/components/ObjectSelectionList$Entry#mouseClicked` | method | 1 |
| `net/minecraft/client/gui/components/ObjectSelectionList$Entry#mouseDragged` | method | 2 |
| `net/minecraft/client/gui/components/ObjectSelectionList$Entry#mouseMoved` | method | 1 |
| `net/minecraft/client/gui/components/ObjectSelectionList$Entry#mouseReleased` | method | 2 |
| `net/minecraft/client/gui/components/ObjectSelectionList$Entry#mouseScrolled` | method | 2 |
| `net/minecraft/client/gui/components/ObjectSelectionList$Entry#setFocused` | method | 1 |
| `net/minecraft/client/gui/components/ObjectSelectionList$Entry#updateNarration` | method | 1 |
| `net/minecraft/client/gui/components/debug/DebugScreenEntryList#isOverlayVisible` | method | 1 |
| `net/minecraft/client/gui/narration/NarratedElementType#USAGE` | field | 2 |
| `net/minecraft/client/gui/narration/NarrationElementOutput#add` | method | 2 |
| `net/minecraft/client/gui/navigation/FocusNavigationEvent$ArrowNavigation#direction` | method | 2 |
| `net/minecraft/client/gui/navigation/ScreenDirection#LEFT` | field | 2 |
| `net/minecraft/client/gui/navigation/ScreenDirection#RIGHT` | field | 2 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#<init>` | method | 1 |
| `net/minecraft/client/gui/render/GuiRenderer#close` | method | 1 |
| `net/minecraft/client/gui/render/GuiRenderer#preparePictureInPictureState` | method | 1 |
| `net/minecraft/client/gui/render/GuiRenderer#render` | method | 2 |
| `net/minecraft/client/gui/render/pip/PictureInPictureRenderer#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/ConfirmScreen#<init>` | method | 9 |
| `net/minecraft/client/gui/screens/ConfirmScreen#keyPressed` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#extractRenderStateWithTooltipAndSubtitles` | method | 2 |
| `net/minecraft/client/gui/screens/Screen#init` | method | 2 |
| `net/minecraft/client/gui/screens/Screen#resize` | method | 2 |
| `net/minecraft/client/input/KeyEvent#<init>` | method | 1 |
| `net/minecraft/client/input/KeyEvent#key` | method | 13 |
| `net/minecraft/client/input/MouseButtonEvent#<init>` | method | 5 |
| `net/minecraft/client/input/MouseButtonEvent#button` | method | 3 |
| `net/minecraft/client/input/MouseButtonEvent#buttonInfo` | method | 1 |
| `net/minecraft/client/input/MouseButtonEvent#x` | method | 5 |
| `net/minecraft/client/input/MouseButtonEvent#y` | method | 5 |
| `net/minecraft/client/input/MouseButtonInfo#<init>` | method | 4 |
| `net/minecraft/client/model/EntityModel#root` | method | 1 |
| `net/minecraft/client/model/EntityModel#setupAnim` | method | 1 |
| `net/minecraft/client/model/HeadedModel#getHead` | method | 1 |
| `net/minecraft/client/model/HumanoidModel#head` | field | 1 |
| `net/minecraft/client/model/Model#renderToBuffer` | method | 1 |
| `net/minecraft/client/model/Model#renderType` | method | 1 |
| `net/minecraft/client/model/Model#root` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#compile` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#getChild` | method | 2 |
| `net/minecraft/client/model/geom/ModelPart#getInitialPose` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#render` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#setPos` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#setRotation` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#skipDraw` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#visible` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#x` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#xRot` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#y` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#yRot` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#z` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#zRot` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart$Cube#maxX` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart$Cube#maxY` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart$Cube#maxZ` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart$Cube#minX` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart$Cube#minY` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart$Cube#minZ` | field | 2 |
| `net/minecraft/client/model/geom/PartPose#xRot` | method | 1 |
| `net/minecraft/client/model/geom/PartPose#yRot` | method | 1 |
| `net/minecraft/client/model/geom/PartPose#zRot` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#NETHER` | field | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#OVERWORLD` | field | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#cardinalLighting` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#clockManager` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#dimension` | method | 12 |
| `net/minecraft/client/multiplayer/ClientLevel#dimensionType` | method | 13 |
| `net/minecraft/client/multiplayer/ClientLevel#dimensionTypeRegistration` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#endFlashState` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#entitiesForRendering` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#getBiome` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getBlockEntity` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getBlockState` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getBrightness` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getChunk` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#getDifficulty` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getFluidState` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getHeight` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getLightEngine` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getMaxY` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getMinY` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#getOverworldClockTime` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getPlayerByUUID` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#isRaining` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#isThundering` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#registryAccess` | method | 3 |
| `net/minecraft/client/multiplayer/ClientPacketListener#close` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#getOnlinePlayers` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#getPlayerInfo` | method | 2 |
| `net/minecraft/client/multiplayer/ClientPacketListener#getServerData` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#handleBlockUpdate` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#handleChunkBlocksUpdate` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#handleLevelChunkWithLight` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#handleLightUpdatePacket` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#handleLogin` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#handleSetSpawn` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#levels` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#queueLightRemoval` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#sendChat` | method | 3 |
| `net/minecraft/client/multiplayer/ClientPacketListener#sendCommand` | method | 2 |
| `net/minecraft/client/multiplayer/ClientPacketListener#sendUnattendedCommand` | method | 1 |
| `net/minecraft/client/multiplayer/MultiPlayerGameMode#canHurtPlayer` | method | 1 |
| `net/minecraft/client/multiplayer/PlayerInfo#getProfile` | method | 3 |
| `net/minecraft/client/multiplayer/PlayerInfo#getSkin` | method | 1 |
| `net/minecraft/client/multiplayer/PlayerInfo#showHat` | method | 1 |
| `net/minecraft/client/multiplayer/ServerData#ip` | field | 1 |
| `net/minecraft/client/multiplayer/ServerData#isRealm` | method | 1 |
| `net/minecraft/client/multiplayer/chat/ChatListener#handleDisguisedChatMessage` | method | 1 |
| `net/minecraft/client/multiplayer/chat/ChatListener#handleSystemMessage` | method | 1 |
| `net/minecraft/client/multiplayer/chat/ChatListener#showMessageToPlayer` | method | 1 |
| `net/minecraft/client/player/AbstractClientPlayer#getSkin` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#connection` | field | 4 |
| `net/minecraft/client/player/LocalPlayer#respawn` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#extract` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#featureRenderDispatcher` | method | 2 |
| `net/minecraft/client/renderer/GameRenderer#lighting` | method | 6 |
| `net/minecraft/client/renderer/GameRenderer#lightmap` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#mainCamera` | method | 6 |
| `net/minecraft/client/renderer/GameRenderer#mainRenderTarget` | method | 2 |
| `net/minecraft/client/renderer/GameRenderer#overlayTexture` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#renderLevel` | method | 1 |
| `net/minecraft/client/renderer/LevelRenderer#render` | method | 1 |
| `net/minecraft/client/renderer/Projection#<init>` | method | 3 |
| `net/minecraft/client/renderer/Projection#height` | method | 1 |
| `net/minecraft/client/renderer/Projection#setSize` | method | 1 |
| `net/minecraft/client/renderer/Projection#setupOrtho` | method | 3 |
| `net/minecraft/client/renderer/Projection#width` | method | 1 |
| `net/minecraft/client/renderer/ProjectionMatrixBuffer#<init>` | method | 3 |
| `net/minecraft/client/renderer/ProjectionMatrixBuffer#getBuffer` | method | 5 |
| `net/minecraft/client/renderer/RenderPipelines#GUI` | field | 1 |
| `net/minecraft/client/renderer/RenderPipelines#GUI_TEXTURED` | field | 2 |
| `net/minecraft/client/renderer/SpriteCoordinateExpander#mapping` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeStorage#<init>` | method | 2 |
| `net/minecraft/client/renderer/SubmitNodeStorage#getSubmitsPerOrder` | method | 1 |
| `net/minecraft/client/renderer/block/BlockModelRenderState#clear` | method | 1 |
| `net/minecraft/client/renderer/block/BlockStateModelSet#get` | method | 1 |
| `net/minecraft/client/renderer/block/FluidModel#tintSource` | method | 1 |
| `net/minecraft/client/renderer/block/FluidStateModelSet#get` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderDispatcher#camera` | field | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderDispatcher#getRenderer` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderDispatcher#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderer#createRenderState` | method | 1 |
| `net/minecraft/client/renderer/entity/LivingEntityRenderer#getModel` | method | 1 |
| `net/minecraft/client/renderer/entity/LivingEntityRenderer#getTextureLocation` | method | 3 |
| `net/minecraft/client/renderer/entity/state/ArmedEntityRenderState#swingAnimation` | field | 1 |
| `net/minecraft/client/renderer/entity/state/HumanoidRenderState#isCrouching` | field | 1 |
| `net/minecraft/client/renderer/entity/state/HumanoidRenderState#isFallFlying` | field | 1 |
| `net/minecraft/client/renderer/entity/state/HumanoidRenderState#isVisuallySwimming` | field | 1 |
| `net/minecraft/client/renderer/entity/state/HumanoidRenderState#swimAmount` | field | 1 |
| `net/minecraft/client/renderer/entity/state/HumanoidRenderState#ticksUsingItem` | field | 1 |
| `net/minecraft/client/renderer/entity/state/LivingEntityRenderState#ageInTicks` | field | 1 |
| `net/minecraft/client/renderer/entity/state/LivingEntityRenderState#bodyRot` | field | 1 |
| `net/minecraft/client/renderer/entity/state/LivingEntityRenderState#walkAnimationSpeed` | field | 1 |
| `net/minecraft/client/renderer/entity/state/LivingEntityRenderState#xRot` | field | 1 |
| `net/minecraft/client/renderer/entity/state/LivingEntityRenderState#yRot` | field | 1 |
| `net/minecraft/client/renderer/entity/state/SlimeRenderState#squish` | field | 1 |
| `net/minecraft/client/renderer/entity/state/SulfurCubeRenderState#containedBlock` | field | 1 |
| `net/minecraft/client/renderer/feature/FeatureRenderDispatcher#prepareFrame` | method | 2 |
| `net/minecraft/client/renderer/feature/FeatureRenderDispatcher#renderAllFeatures` | method | 2 |
| `net/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame#close` | method | 2 |
| `net/minecraft/client/renderer/feature/ModelFeatureRenderer#prepareModel` | method | 1 |
| `net/minecraft/client/renderer/feature/ModelFeatureRenderer$Submit#renderType` | method | 1 |
| `net/minecraft/client/renderer/item/ItemModelResolver#updateForTopItem` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#<init>` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#submit` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#usesBlockLight` | method | 1 |
| `net/minecraft/client/renderer/rendertype/LayeringTransform#NO_LAYERING` | field | 1 |
| `net/minecraft/client/renderer/rendertype/LayeringTransform#VIEW_OFFSET_Z_LAYERING_FORWARD` | field | 1 |
| `net/minecraft/client/renderer/rendertype/PreparedRenderType$Texture#<init>` | method | 6 |
| `net/minecraft/client/renderer/rendertype/PreparedRenderType$Texture#name` | method | 1 |
| `net/minecraft/client/renderer/rendertype/PreparedRenderType$Texture#sampler` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup#builder` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup#prepareTextures` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#affectsCrumbling` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#setLayeringTransform` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#setTextureTransform` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#sortOnUpload` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#useOverlay` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#withTexture` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderType#affectsCrumbling` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderType#format` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderType#pipeline` | method | 2 |
| `net/minecraft/client/renderer/rendertype/RenderType#primitiveTopology` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderType#sortOnUpload` | method | 1 |
| `net/minecraft/client/renderer/rendertype/TextureTransform#DEFAULT_TEXTURING` | field | 1 |
| `net/minecraft/client/renderer/state/gui/GuiRenderState#addPicturesInPictureState` | method | 1 |
| `net/minecraft/client/renderer/state/gui/pip/PictureInPictureRenderState#getClass` | method | 2 |
| `net/minecraft/client/renderer/state/level/CameraRenderState#<init>` | method | 1 |
| `net/minecraft/client/renderer/state/level/CameraRenderState#viewRotationMatrix` | field | 1 |
| `net/minecraft/client/renderer/texture/AbstractTexture#getSampler` | method | 1 |
| `net/minecraft/client/renderer/texture/AbstractTexture#getTextureView` | method | 4 |
| `net/minecraft/client/renderer/texture/DynamicTexture#<init>` | method | 1 |
| `net/minecraft/client/renderer/texture/DynamicTexture#getPixels` | method | 1 |
| `net/minecraft/client/renderer/texture/DynamicTexture#getTextureView` | method | 1 |
| `net/minecraft/client/renderer/texture/DynamicTexture#upload` | method | 1 |
| `net/minecraft/client/renderer/texture/MissingTextureAtlasSprite#getLocation` | method | 1 |
| `net/minecraft/client/renderer/texture/OverlayTexture#NO_OVERLAY` | field | 2 |
| `net/minecraft/client/renderer/texture/OverlayTexture#getTextureView` | method | 1 |
| `net/minecraft/client/renderer/texture/SimpleTexture#<init>` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlas#getSprite` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlas#upload` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#atlasLocation` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU0` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU1` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV0` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV1` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureManager#getTexture` | method | 5 |
| `net/minecraft/client/renderer/texture/TextureManager#register` | method | 1 |
| `net/minecraft/client/renderer/texture/UvMapping#wrap` | method | 1 |
| `net/minecraft/client/resources/DefaultPlayerSkin#get` | method | 1 |
| `net/minecraft/client/resources/language/I18n#get` | method | 23 |
| `net/minecraft/client/resources/model/ModelManager#getBlockStateModelSet` | method | 1 |
| `net/minecraft/client/resources/model/ModelManager#getFluidStateModelSet` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#materialInfo` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#sprite` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#tintIndex` | method | 1 |
| `net/minecraft/client/resources/model/sprite/AtlasManager#getAtlasOrThrow` | method | 2 |
| `net/minecraft/client/resources/sounds/SimpleSoundInstance#forUI` | method | 3 |
| `net/minecraft/client/server/IntegratedServer#getLevel` | method | 1 |
| `net/minecraft/client/server/IntegratedServer#getWorldPath` | method | 1 |
| `net/minecraft/client/sounds/SoundManager#play` | method | 3 |
| `net/minecraft/core/BlockPos#<init>` | method | 1 |
| `net/minecraft/core/BlockPos#ZERO` | field | 2 |
| `net/minecraft/core/BlockPos#getX` | method | 6 |
| `net/minecraft/core/BlockPos#getY` | method | 5 |
| `net/minecraft/core/BlockPos#getZ` | method | 6 |
| `net/minecraft/core/BlockPos$MutableBlockPos#<init>` | method | 3 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getX` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getY` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getZ` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#set` | method | 3 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setY` | method | 1 |
| `net/minecraft/core/ClientAsset$Texture#texturePath` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#get` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#getKey` | method | 2 |
| `net/minecraft/core/DefaultedRegistry#getOptional` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#getValue` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#keySet` | method | 1 |
| `net/minecraft/core/Direction#UP` | field | 1 |
| `net/minecraft/core/Direction#values` | method | 1 |
| `net/minecraft/core/Direction$Axis#Y` | field | 1 |
| `net/minecraft/core/Holder#is` | method | 1 |
| `net/minecraft/core/Holder#unwrapKey` | method | 1 |
| `net/minecraft/core/Holder#value` | method | 5 |
| `net/minecraft/core/Holder$Reference#isBound` | method | 1 |
| `net/minecraft/core/Holder$Reference#value` | method | 1 |
| `net/minecraft/core/HolderSet#forEach` | method | 1 |
| `net/minecraft/core/NonNullList#get` | method | 1 |
| `net/minecraft/core/NonNullList#size` | method | 1 |
| `net/minecraft/core/Registry#entrySet` | method | 1 |
| `net/minecraft/core/Registry#getKey` | method | 2 |
| `net/minecraft/core/Registry#getValue` | method | 1 |
| `net/minecraft/core/Registry#iterator` | method | 1 |
| `net/minecraft/core/RegistryAccess#lookup` | method | 1 |
| `net/minecraft/core/RegistryAccess#lookupOrThrow` | method | 3 |
| `net/minecraft/core/SectionPos#getX` | method | 1 |
| `net/minecraft/core/SectionPos#getZ` | method | 1 |
| `net/minecraft/core/UUIDUtil#uuidFromIntArray` | method | 1 |
| `net/minecraft/core/UUIDUtil#uuidToIntArray` | method | 1 |
| `net/minecraft/core/component/DataComponents#TRIM` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#ENTITY_TYPE` | field | 2 |
| `net/minecraft/core/registries/BuiltInRegistries#MOB_EFFECT` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#VILLAGER_PROFESSION` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#VILLAGER_TYPE` | field | 1 |
| `net/minecraft/core/registries/Registries#BIOME` | field | 1 |
| `net/minecraft/core/registries/Registries#BLOCK` | field | 2 |
| `net/minecraft/core/registries/Registries#DIMENSION` | field | 4 |
| `net/minecraft/core/registries/Registries#DIMENSION_TYPE` | field | 1 |
| `net/minecraft/data/AtlasIds#BLOCKS` | field | 2 |
| `net/minecraft/locale/Language#getInstance` | method | 1 |
| `net/minecraft/nbt/ByteTag#byteValue` | method | 1 |
| `net/minecraft/nbt/CompoundTag#<init>` | method | 4 |
| `net/minecraft/nbt/CompoundTag#contains` | method | 1 |
| `net/minecraft/nbt/CompoundTag#get` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getBoolean` | method | 2 |
| `net/minecraft/nbt/CompoundTag#getBooleanOr` | method | 2 |
| `net/minecraft/nbt/CompoundTag#getCompoundOrEmpty` | method | 2 |
| `net/minecraft/nbt/CompoundTag#getDouble` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getIntArray` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getIntOr` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getListOrEmpty` | method | 3 |
| `net/minecraft/nbt/CompoundTag#getLongArray` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getString` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getStringOr` | method | 3 |
| `net/minecraft/nbt/CompoundTag#isEmpty` | method | 1 |
| `net/minecraft/nbt/CompoundTag#keySet` | method | 1 |
| `net/minecraft/nbt/CompoundTag#put` | method | 3 |
| `net/minecraft/nbt/CompoundTag#putBoolean` | method | 4 |
| `net/minecraft/nbt/CompoundTag#putDouble` | method | 2 |
| `net/minecraft/nbt/CompoundTag#putInt` | method | 2 |
| `net/minecraft/nbt/CompoundTag#putIntArray` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putLongArray` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putString` | method | 4 |
| `net/minecraft/nbt/DoubleTag#doubleValue` | method | 1 |
| `net/minecraft/nbt/IntArrayTag#<init>` | method | 1 |
| `net/minecraft/nbt/IntTag#intValue` | method | 1 |
| `net/minecraft/nbt/ListTag#<init>` | method | 3 |
| `net/minecraft/nbt/ListTag#add` | method | 3 |
| `net/minecraft/nbt/ListTag#getStringOr` | method | 1 |
| `net/minecraft/nbt/ListTag#isEmpty` | method | 1 |
| `net/minecraft/nbt/ListTag#iterator` | method | 2 |
| `net/minecraft/nbt/ListTag#size` | method | 1 |
| `net/minecraft/nbt/NbtAccounter#unlimitedHeap` | method | 1 |
| `net/minecraft/nbt/StringTag#valueOf` | method | 2 |
| `net/minecraft/nbt/Tag#asString` | method | 1 |
| `net/minecraft/nbt/Tag#getClass` | method | 1 |
| `net/minecraft/network/FriendlyByteBuf#readInt` | method | 2 |
| `net/minecraft/network/FriendlyByteBuf#readNbt` | method | 2 |
| `net/minecraft/network/FriendlyByteBuf#writeInt` | method | 2 |
| `net/minecraft/network/FriendlyByteBuf#writeNbt` | method | 2 |
| `net/minecraft/network/chat/ClickEvent$RunCommand#<init>` | method | 2 |
| `net/minecraft/network/chat/CommonComponents#optionNameValue` | method | 2 |
| `net/minecraft/network/chat/Component#copy` | method | 6 |
| `net/minecraft/network/chat/Component#getContents` | method | 1 |
| `net/minecraft/network/chat/Component#getString` | method | 7 |
| `net/minecraft/network/chat/Component#literal` | method | 36 |
| `net/minecraft/network/chat/Component#translatable` | method | 77 |
| `net/minecraft/network/chat/Component#visit` | method | 1 |
| `net/minecraft/network/chat/HoverEvent$ShowText#<init>` | method | 2 |
| `net/minecraft/network/chat/MutableComponent#getSiblings` | method | 6 |
| `net/minecraft/network/chat/MutableComponent#getString` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#getStyle` | method | 2 |
| `net/minecraft/network/chat/MutableComponent#setStyle` | method | 2 |
| `net/minecraft/network/chat/MutableComponent#withColor` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#withStyle` | method | 14 |
| `net/minecraft/network/chat/Style#EMPTY` | field | 6 |
| `net/minecraft/network/chat/Style#applyFormat` | method | 1 |
| `net/minecraft/network/chat/Style#withClickEvent` | method | 2 |
| `net/minecraft/network/chat/Style#withColor` | method | 6 |
| `net/minecraft/network/chat/Style#withHoverEvent` | method | 2 |
| `net/minecraft/network/chat/TextColor#fromLegacyFormat` | method | 2 |
| `net/minecraft/network/chat/contents/PlainTextContents#text` | method | 1 |
| `net/minecraft/network/protocol/PacketUtils#ensureRunningOnSameThread` | method | 6 |
| `net/minecraft/network/protocol/game/ClientboundBlockUpdatePacket#getPos` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundForgetLevelChunkPacket#pos` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundLevelChunkWithLightPacket#x` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundLevelChunkWithLightPacket#z` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundLightUpdatePacket#x` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundLightUpdatePacket#z` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundSetDefaultSpawnPositionPacket#respawnData` | method | 1 |
| `net/minecraft/network/syncher/SynchedEntityData#get` | method | 1 |
| `net/minecraft/resources/Identifier#equals` | method | 3 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 17 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 7 |
| `net/minecraft/resources/Identifier#getPath` | method | 11 |
| `net/minecraft/resources/Identifier#isAllowedInIdentifier` | method | 1 |
| `net/minecraft/resources/Identifier#parse` | method | 9 |
| `net/minecraft/resources/Identifier#toString` | method | 9 |
| `net/minecraft/resources/ResourceKey#create` | method | 4 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 16 |
| `net/minecraft/server/MinecraftServer#getLevel` | method | 1 |
| `net/minecraft/server/MinecraftServer#getPlayerList` | method | 1 |
| `net/minecraft/server/MinecraftServer#getWorldPath` | method | 1 |
| `net/minecraft/server/MinecraftServer#tickServer` | method | 1 |
| `net/minecraft/server/level/ServerLevel#dimensionType` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getSeed` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getServer` | method | 3 |
| `net/minecraft/server/level/ServerPlayer#connection` | field | 1 |
| `net/minecraft/server/level/ServerPlayer#getUUID` | method | 3 |
| `net/minecraft/server/level/ServerPlayer#level` | method | 3 |
| `net/minecraft/server/network/ServerGamePacketListenerImpl#disconnect` | method | 1 |
| `net/minecraft/server/packs/resources/Resource#open` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceManager#getResource` | method | 1 |
| `net/minecraft/server/players/PlayerList#getPlayers` | method | 1 |
| `net/minecraft/server/players/PlayerList#placeNewPlayer` | method | 1 |
| `net/minecraft/server/players/PlayerList#sendLevelInfo` | method | 1 |
| `net/minecraft/sounds/SoundEvents#UI_BUTTON_CLICK` | field | 3 |
| `net/minecraft/sounds/SoundSource#HOSTILE` | field | 1 |
| `net/minecraft/tags/BlockTags#FLOWERS` | field | 1 |
| `net/minecraft/tags/BlockTags#LEAVES` | field | 1 |
| `net/minecraft/tags/ItemTags#TRIMMABLE_ARMOR` | field | 1 |
| `net/minecraft/util/ARGB#alpha` | method | 2 |
| `net/minecraft/util/FormattedCharSink#accept` | method | 1 |
| `net/minecraft/util/Mth#clamp` | method | 5 |
| `net/minecraft/util/Mth#floor` | method | 1 |
| `net/minecraft/util/Mth#lerp` | method | 2 |
| `net/minecraft/util/Mth#positiveModulo` | method | 1 |
| `net/minecraft/util/Mth#wrapDegrees` | method | 1 |
| `net/minecraft/util/RandomSource#create` | method | 1 |
| `net/minecraft/util/RandomSource#nextInt` | method | 1 |
| `net/minecraft/util/Util#make` | method | 1 |
| `net/minecraft/util/Util#memoize` | method | 1 |
| `net/minecraft/world/Difficulty#PEACEFUL` | field | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem#builder` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem#getValue` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem#invalidateTickCache` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem$Builder#addConstantLayer` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem$Builder#addTimelineLayer` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem$Builder#build` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributes#SKY_LIGHT_FACTOR` | field | 1 |
| `net/minecraft/world/effect/MobEffect#<init>` | method | 1 |
| `net/minecraft/world/effect/MobEffect#isBeneficial` | method | 1 |
| `net/minecraft/world/effect/MobEffectCategory#BENEFICIAL` | field | 1 |
| `net/minecraft/world/effect/MobEffectCategory#HARMFUL` | field | 2 |
| `net/minecraft/world/effect/MobEffectCategory#NEUTRAL` | field | 1 |
| `net/minecraft/world/effect/MobEffectInstance#getEffect` | method | 1 |
| `net/minecraft/world/effect/MobEffectInstance#showIcon` | method | 1 |
| `net/minecraft/world/entity/Entity#blockPosition` | method | 1 |
| `net/minecraft/world/entity/Entity#getCustomName` | method | 1 |
| `net/minecraft/world/entity/Entity#getName` | method | 1 |
| `net/minecraft/world/entity/Entity#getPickResult` | method | 1 |
| `net/minecraft/world/entity/Entity#getScoreboardName` | method | 2 |
| `net/minecraft/world/entity/Entity#getSoundSource` | method | 1 |
| `net/minecraft/world/entity/Entity#getTeam` | method | 4 |
| `net/minecraft/world/entity/Entity#getType` | method | 6 |
| `net/minecraft/world/entity/Entity#getUUID` | method | 2 |
| `net/minecraft/world/entity/Entity#getViewYRot` | method | 1 |
| `net/minecraft/world/entity/Entity#hasCustomName` | method | 2 |
| `net/minecraft/world/entity/Entity#isInvisibleTo` | method | 1 |
| `net/minecraft/world/entity/Entity#isShiftKeyDown` | method | 2 |
| `net/minecraft/world/entity/Entity#level` | method | 5 |
| `net/minecraft/world/entity/Entity#position` | method | 1 |
| `net/minecraft/world/entity/Entity#tickCount` | field | 1 |
| `net/minecraft/world/entity/Entity#xOld` | field | 1 |
| `net/minecraft/world/entity/Entity#yOld` | field | 1 |
| `net/minecraft/world/entity/Entity#zOld` | field | 1 |
| `net/minecraft/world/entity/EntityReference#getUUID` | method | 1 |
| `net/minecraft/world/entity/EntityType#getKey` | method | 9 |
| `net/minecraft/world/entity/EntityTypes#ARMOR_STAND` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#CUSHION` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#GLOW_ITEM_FRAME` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#ITEM_FRAME` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlot#BODY` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlot#HEAD` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#getItemBySlot` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isBaby` | method | 2 |
| `net/minecraft/world/entity/OwnableEntity#getOwnerReference` | method | 1 |
| `net/minecraft/world/entity/TamableAnimal#isTame` | method | 1 |
| `net/minecraft/world/entity/animal/equine/AbstractHorse#isTamed` | method | 1 |
| `net/minecraft/world/entity/animal/equine/Horse#getMarkings` | method | 1 |
| `net/minecraft/world/entity/animal/equine/Llama#getBodyArmorItem` | method | 1 |
| `net/minecraft/world/entity/animal/equine/Llama#isTraderLlama` | method | 1 |
| `net/minecraft/world/entity/animal/equine/Markings#BLACK_DOTS` | field | 1 |
| `net/minecraft/world/entity/animal/equine/Markings#NONE` | field | 1 |
| `net/minecraft/world/entity/animal/equine/Markings#WHITE` | field | 1 |
| `net/minecraft/world/entity/animal/equine/Markings#WHITE_DOTS` | field | 1 |
| `net/minecraft/world/entity/animal/equine/Markings#WHITE_FIELD` | field | 1 |
| `net/minecraft/world/entity/animal/fish/TropicalFish#getBaseColor` | method | 1 |
| `net/minecraft/world/entity/animal/fish/TropicalFish#getPattern` | method | 1 |
| `net/minecraft/world/entity/animal/fish/TropicalFish#getPatternColor` | method | 1 |
| `net/minecraft/world/entity/animal/fox/Fox#getEntityData` | method | 1 |
| `net/minecraft/world/entity/animal/golem/IronGolem#getCrackiness` | method | 1 |
| `net/minecraft/world/entity/animal/pig/Pig#isSaddled` | method | 1 |
| `net/minecraft/world/entity/decoration/Cushion#getColor` | method | 1 |
| `net/minecraft/world/entity/decoration/ItemFrame#getItem` | method | 2 |
| `net/minecraft/world/entity/item/ItemEntity#getItem` | method | 3 |
| `net/minecraft/world/entity/monster/Enderman#isCreepy` | method | 1 |
| `net/minecraft/world/entity/monster/Strider#isSaddled` | method | 1 |
| `net/minecraft/world/entity/monster/piglin/Piglin#isBaby` | method | 1 |
| `net/minecraft/world/entity/npc/villager/VillagerData#level` | method | 1 |
| `net/minecraft/world/entity/npc/villager/VillagerData#profession` | method | 1 |
| `net/minecraft/world/entity/npc/villager/VillagerData#type` | method | 1 |
| `net/minecraft/world/entity/npc/villager/VillagerDataHolder#getVillagerData` | method | 1 |
| `net/minecraft/world/entity/player/ChatVisiblity#HIDDEN` | field | 1 |
| `net/minecraft/world/entity/player/Inventory#getContainerSize` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#getItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#getActiveEffects` | method | 1 |
| `net/minecraft/world/entity/player/Player#getCustomName` | method | 1 |
| `net/minecraft/world/entity/player/Player#getGameProfile` | method | 1 |
| `net/minecraft/world/entity/player/Player#getInventory` | method | 1 |
| `net/minecraft/world/entity/player/Player#getTeam` | method | 2 |
| `net/minecraft/world/entity/player/Player#getUUID` | method | 7 |
| `net/minecraft/world/entity/player/Player#getX` | method | 2 |
| `net/minecraft/world/entity/player/Player#getY` | method | 2 |
| `net/minecraft/world/entity/player/Player#getZ` | method | 2 |
| `net/minecraft/world/entity/player/Player#hasEffect` | method | 1 |
| `net/minecraft/world/entity/player/Player#level` | method | 4 |
| `net/minecraft/world/entity/player/Player#tick` | method | 1 |
| `net/minecraft/world/entity/player/PlayerSkin#body` | method | 1 |
| `net/minecraft/world/item/DyeColor#BLACK` | field | 1 |
| `net/minecraft/world/item/DyeColor#BLUE` | field | 1 |
| `net/minecraft/world/item/DyeColor#BROWN` | field | 2 |
| `net/minecraft/world/item/DyeColor#CYAN` | field | 1 |
| `net/minecraft/world/item/DyeColor#GRAY` | field | 1 |
| `net/minecraft/world/item/DyeColor#GREEN` | field | 1 |
| `net/minecraft/world/item/DyeColor#LIGHT_BLUE` | field | 2 |
| `net/minecraft/world/item/DyeColor#LIGHT_GRAY` | field | 1 |
| `net/minecraft/world/item/DyeColor#LIME` | field | 2 |
| `net/minecraft/world/item/DyeColor#MAGENTA` | field | 2 |
| `net/minecraft/world/item/DyeColor#ORANGE` | field | 1 |
| `net/minecraft/world/item/DyeColor#PINK` | field | 2 |
| `net/minecraft/world/item/DyeColor#PURPLE` | field | 1 |
| `net/minecraft/world/item/DyeColor#RED` | field | 1 |
| `net/minecraft/world/item/DyeColor#YELLOW` | field | 1 |
| `net/minecraft/world/item/DyeColor#getTextColor` | method | 1 |
| `net/minecraft/world/item/DyeColor#ordinal` | method | 2 |
| `net/minecraft/world/item/DyeColor#toString` | method | 1 |
| `net/minecraft/world/item/DyeColor#values` | method | 1 |
| `net/minecraft/world/item/Item#getDefaultInstance` | method | 1 |
| `net/minecraft/world/item/ItemDisplayContext#GUI` | field | 1 |
| `net/minecraft/world/item/ItemStack#<init>` | method | 1 |
| `net/minecraft/world/item/ItemStack#EMPTY` | field | 1 |
| `net/minecraft/world/item/ItemStack#get` | method | 1 |
| `net/minecraft/world/item/ItemStack#getItem` | method | 5 |
| `net/minecraft/world/item/ItemStack#getItemName` | method | 1 |
| `net/minecraft/world/item/ItemStack#has` | method | 1 |
| `net/minecraft/world/item/ItemStack#is` | method | 1 |
| `net/minecraft/world/item/ItemStack#isEmpty` | method | 1 |
| `net/minecraft/world/item/Items#AIR` | field | 2 |
| `net/minecraft/world/item/equipment/trim/ArmorTrim#material` | method | 1 |
| `net/minecraft/world/item/equipment/trim/ArmorTrim#pattern` | method | 1 |
| `net/minecraft/world/level/ChunkPos#<init>` | method | 1 |
| `net/minecraft/world/level/ChunkPos#pack` | method | 1 |
| `net/minecraft/world/level/ChunkPos#x` | method | 2 |
| `net/minecraft/world/level/ChunkPos#z` | method | 2 |
| `net/minecraft/world/level/ColorResolver#getColor` | method | 1 |
| `net/minecraft/world/level/Level#END` | field | 3 |
| `net/minecraft/world/level/Level#NETHER` | field | 6 |
| `net/minecraft/world/level/Level#OVERWORLD` | field | 5 |
| `net/minecraft/world/level/Level#dimension` | method | 6 |
| `net/minecraft/world/level/Level#dimensionType` | method | 1 |
| `net/minecraft/world/level/Level#getBiome` | method | 1 |
| `net/minecraft/world/level/Level#getBlockEntity` | method | 1 |
| `net/minecraft/world/level/Level#getBlockState` | method | 1 |
| `net/minecraft/world/level/Level#getBrightness` | method | 2 |
| `net/minecraft/world/level/Level#getChunk` | method | 2 |
| `net/minecraft/world/level/Level#getMaxY` | method | 1 |
| `net/minecraft/world/level/Level#getMinY` | method | 1 |
| `net/minecraft/world/level/Level#getServer` | method | 1 |
| `net/minecraft/world/level/Level#registryAccess` | method | 2 |
| `net/minecraft/world/level/LightLayer#BLOCK` | field | 3 |
| `net/minecraft/world/level/LightLayer#SKY` | field | 3 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 1 |
| `net/minecraft/world/level/block/Block#getId` | method | 1 |
| `net/minecraft/world/level/block/Blocks#AIR` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BARRIER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GLASS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GLASS_PANE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#REDSTONE_TORCH` | field | 1 |
| `net/minecraft/world/level/block/Blocks#REDSTONE_WIRE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SHORT_GRASS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#TORCH` | field | 1 |
| `net/minecraft/world/level/block/RenderShape#INVISIBLE` | field | 1 |
| `net/minecraft/world/level/block/state/BlockState#getBlock` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#getFluidState` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#getLightDampening` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getLightEmission` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getMapColor` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getPistonPushReaction` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getRenderShape` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getShape` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#is` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#isAir` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkAccess#getHeight` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkAccess#getSections` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getBlockState` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getHeight` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getLevel` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getMinY` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getPos` | method | 2 |
| `net/minecraft/world/level/chunk/LevelChunk#getSections` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunkSection#hasOnlyAir` | method | 1 |
| `net/minecraft/world/level/chunk/status/ChunkStatus#FULL` | field | 1 |
| `net/minecraft/world/level/dimension/BuiltinDimensionTypes#END` | field | 1 |
| `net/minecraft/world/level/dimension/BuiltinDimensionTypes#NETHER` | field | 2 |
| `net/minecraft/world/level/dimension/BuiltinDimensionTypes#OVERWORLD` | field | 2 |
| `net/minecraft/world/level/dimension/DimensionType#ambientLight` | method | 2 |
| `net/minecraft/world/level/dimension/DimensionType#attributes` | method | 1 |
| `net/minecraft/world/level/dimension/DimensionType#coordinateScale` | method | 11 |
| `net/minecraft/world/level/dimension/DimensionType#hasSkyLight` | method | 1 |
| `net/minecraft/world/level/dimension/DimensionType#timelines` | method | 1 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#WORLD_SURFACE` | field | 1 |
| `net/minecraft/world/level/levelgen/WorldgenRandom#seedSlimeChunk` | method | 1 |
| `net/minecraft/world/level/material/FluidState#createLegacyBlock` | method | 1 |
| `net/minecraft/world/level/material/FluidState#isEmpty` | method | 1 |
| `net/minecraft/world/level/material/MapColor#col` | field | 1 |
| `net/minecraft/world/level/material/PushReaction#POPPED` | field | 1 |
| `net/minecraft/world/level/storage/LevelData$RespawnData#pos` | method | 1 |
| `net/minecraft/world/level/storage/LevelResource#LEVEL_DATA_FILE` | field | 1 |
| `net/minecraft/world/level/storage/LevelResource#ROOT` | field | 2 |
| `net/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess#deleteLevel` | method | 1 |
| `net/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess#getLevelPath` | method | 1 |
| `net/minecraft/world/phys/Vec3#<init>` | method | 4 |
| `net/minecraft/world/phys/Vec3#x` | field | 10 |
| `net/minecraft/world/phys/Vec3#y` | field | 6 |
| `net/minecraft/world/phys/Vec3#z` | field | 10 |
| `net/minecraft/world/phys/shapes/VoxelShape#max` | method | 1 |
| `net/minecraft/world/scores/PlayerTeam#getFormattedName` | method | 1 |
| `net/minecraft/world/scores/Team#getColor` | method | 1 |
| `net/minecraft/world/scores/TeamColor#WHITE` | field | 1 |
| `net/minecraft/world/scores/TeamColor#rgb` | method | 1 |

</details>

