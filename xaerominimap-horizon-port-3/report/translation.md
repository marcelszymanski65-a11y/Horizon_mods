# Java translated to Rust

Translated 128 of 3756 methods (1 more is done another way in Horizon) and 198 of 572 static fields to Rust (46 of those call the game, through Horizon's helpers); 0 of 3 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 3078 | `MultiplyAnimationHelper.tick(0)`: it uses MultiplyAnimationHelper.lastTick, which isn't translated (its value reads other changing statics) |
| Other Java this translator doesn't handle yet | 389 | `IBufferSource.getXaero_lastRenderType(0)`: it has no body |
| Uses Minecraft classes (needs Horizon's game API) | 75 | `XaeroMinimapCore.onHandleLevelChunkWithLight(1)`: it uses Minecraft's `ClientboundLevelChunkWithLightPacket` |
| Uses other libraries | 51 | `LegacyCommonConfigIO.load(0)`: it uses `ConfigProfile` (xaero.lib.common.config.profile) |
| Uses Java collections other than List (Map, Set...) | 9 | `SupportWaystones.removeWaypoint(4)`: it uses `UUID` (java.util) |
| Shares one object between names (Java references) | 8 | `GuiWaypointSets.GuiWaypointSets(2)`: it stores a second reference to an object (Rust would need it copied or shared) |
| Calls the game where Horizon has no helper yet | 5 | `Misc.getMouseX(2)`: it uses Minecraft's Minecraft.mouseHandler (no Horizon helper for it yet) |
| Uses try/catch | 5 | `XaeroMinimapCore.cleanupCurrentSession(0)`: a try/finally |
| Uses files (java.nio / java.io) | 4 | `PixelBuffers.glMapBuffer(4)`: it uses `ByteBuffer` (java.nio) |
| Uses null | 3 | `InfoDisplayIO.getInfoDisplayLine(2)`: `null` (Rust has no null: this needs an Option) |
| Done another way in Horizon | 1 | `XaeroHudModMenu.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) |

## Close, not the same

Where the Rust does a little less than the Java, because Horizon doesn't have something:

- CategoryConstants.: the text gui.xaero_off isn't in the mod's en_us.json, so the key itself shows
- CategoryConstants.: the text gui.xaero_on isn't in the mod's en_us.json, so the key itself shows
- GuiEditMode.: the text gui.no isn't in the mod's en_us.json, so the key itself shows
- GuiEditMode.: the text gui.yes isn't in the mod's en_us.json, so the key itself shows
- MinimapConfigConstants.: the text gui.xaero_off isn't in the mod's en_us.json, so the key itself shows
- SelectionList.: the text narration.selection.usage isn't in the mod's en_us.json, so the key itself shows
- SettingRowList.: the text narration.selection.usage isn't in the mod's en_us.json, so the key itself shows

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| xaero/common/events/ModClientEventsFabric.java:13 | `hud` | not translated: a handler that isn't a lambda or a Class::method reference |
| xaero/common/mixin/MixinClientPlayNetworkHandler.java:101 | `join` | not translated: it calls XaeroMinimapCore.onPlayNetHandler, which isn't translated |
| xaero/minimap/XaeroMinimapFabric.java:13 | `init` | not translated: a catch block that uses its exception |

## Every class


### `MultiplyAnimationHelper`

- [x] `MultiplyAnimationHelper.currentTick` (xaero/common/anim/MultiplyAnimationHelper.java:4)
- [ ] `MultiplyAnimationHelper.lastTick`: its value reads other changing statics (xaero/common/anim/MultiplyAnimationHelper.java:5)
- [x] `MultiplyAnimationHelper.STEP_TIME` (xaero/common/anim/MultiplyAnimationHelper.java:6)
- [ ] `MultiplyAnimationHelper.tick(0)`: it uses MultiplyAnimationHelper.lastTick, which isn't translated (its value reads other changing statics) (xaero/common/anim/MultiplyAnimationHelper.java:8)
- [ ] `MultiplyAnimationHelper.animate(2)`: it uses MultiplyAnimationHelper.lastTick, which isn't translated (its value reads other changing statics) (xaero/common/anim/MultiplyAnimationHelper.java:13)

### `BlockStateShortShapeCache`: it uses IXaeroMinimap, which isn't translated (it's an interface)

- [ ] `BlockStateShortShapeCache.BlockStateShortShapeCache(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/cache/BlockStateShortShapeCache.java:23)
- [ ] `BlockStateShortShapeCache.isShort(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/cache/BlockStateShortShapeCache.java:29)
- [ ] `BlockStateShortShapeCache.reset(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/cache/BlockStateShortShapeCache.java:67)

### `PlaceholderBlockGetter`: it implements Minecraft's `BlockGetter`

- [ ] `PlaceholderBlockGetter.setPlaceholderState(1)`: its class isn't translated (it implements Minecraft's `BlockGetter`) (xaero/common/cache/placeholder/PlaceholderBlockGetter.java:12)
- [ ] `PlaceholderBlockGetter.getBlockEntity(1)`: its class isn't translated (it implements Minecraft's `BlockGetter`) (xaero/common/cache/placeholder/PlaceholderBlockGetter.java:16)
- [ ] `PlaceholderBlockGetter.getBlockState(1)`: its class isn't translated (it implements Minecraft's `BlockGetter`) (xaero/common/cache/placeholder/PlaceholderBlockGetter.java:20)
- [ ] `PlaceholderBlockGetter.getFluidState(1)`: its class isn't translated (it implements Minecraft's `BlockGetter`) (xaero/common/cache/placeholder/PlaceholderBlockGetter.java:24)
- [ ] `PlaceholderBlockGetter.getHeight(0)`: its class isn't translated (it implements Minecraft's `BlockGetter`) (xaero/common/cache/placeholder/PlaceholderBlockGetter.java:28)
- [ ] `PlaceholderBlockGetter.getMinY(0)`: its class isn't translated (it implements Minecraft's `BlockGetter`) (xaero/common/cache/placeholder/PlaceholderBlockGetter.java:32)

### `LegacyCommonConfigInit`

- [ ] `LegacyCommonConfigInit.init(2)`: it uses IXaeroMinimap, which isn't translated (it's an interface) (xaero/common/config/LegacyCommonConfigInit.java:9)

### `LegacyCommonConfigIO`

- [x] `LegacyCommonConfigIO.LegacyCommonConfigIO(1)` (xaero/common/config/LegacyCommonConfigIO.java:24)
- [ ] `LegacyCommonConfigIO.load(0)`: it uses `ConfigProfile` (xaero.lib.common.config.profile) (xaero/common/config/LegacyCommonConfigIO.java:28)
- [ ] `LegacyCommonConfigIO.readLine(1)`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`) (xaero/common/config/LegacyCommonConfigIO.java:66)
- [x] `LegacyCommonConfigIO.shouldEnableEveryoneTracksEveryone(0)` (xaero/common/config/LegacyCommonConfigIO.java:98)

### `IBufferSource`: it's an interface

- [ ] `IBufferSource.getXaero_lastRenderType(0)`: it has no body (xaero/common/core/IBufferSource.java:6)
- [ ] `IBufferSource.setXaero_lastRenderType(1)`: it has no body (xaero/common/core/IBufferSource.java:8)

### `IGuiGraphics`: it's an interface

- [ ] `IGuiGraphics.xaero_mm_getGuiRenderState(0)`: it has no body (xaero/common/core/IGuiGraphics.java:6)

### `IGuiRenderer`: it's an interface

- [ ] `IGuiRenderer.xaero_mm_getGuiProjection(0)`: it has no body (xaero/common/core/IGuiRenderer.java:7)
- [ ] `IGuiRenderer.xaero_mm_getGuiProjectionMatrixBuffer(0)`: it has no body (xaero/common/core/IGuiRenderer.java:9)

### `IPictureInPictureRenderer`: it's an interface

- [ ] `IPictureInPictureRenderer.xaero_mm_getTexture(0)`: it has no body (xaero/common/core/IPictureInPictureRenderer.java:7)
- [ ] `IPictureInPictureRenderer.xaero_mm_getTextureView(0)`: it has no body (xaero/common/core/IPictureInPictureRenderer.java:9)
- [ ] `IPictureInPictureRenderer.xaero_mm_getDepthTextureView(0)`: it has no body (xaero/common/core/IPictureInPictureRenderer.java:11)
- [ ] `IPictureInPictureRenderer.xaero_mm_prepareTexturesAndProjection(3)`: it has no body (xaero/common/core/IPictureInPictureRenderer.java:13)

### `IXaeroMinimapClientPlayNetHandler`: it's an interface

- [ ] `IXaeroMinimapClientPlayNetHandler.getXaero_minimapSession(0)`: it has no body (xaero/common/core/IXaeroMinimapClientPlayNetHandler.java:6)
- [ ] `IXaeroMinimapClientPlayNetHandler.setXaero_minimapSession(1)`: it has no body (xaero/common/core/IXaeroMinimapClientPlayNetHandler.java:8)

### `IXaeroMinimapGameMessageS2CPacket`: it's an interface

- [ ] `IXaeroMinimapGameMessageS2CPacket.setMessage(1)`: it has no body (xaero/common/core/IXaeroMinimapGameMessageS2CPacket.java:6)

### `IXaeroMinimapMinecraftClient`: it's an interface

- [ ] `IXaeroMinimapMinecraftClient.getXaeroMinimap_fps(0)`: it has no body (xaero/common/core/IXaeroMinimapMinecraftClient.java:4)

### `IXaeroMinimapSMultiBlockChangePacket`: it's an interface

- [ ] `IXaeroMinimapSMultiBlockChangePacket.xaero_mm_getSectionPos(0)`: it has no body (xaero/common/core/IXaeroMinimapSMultiBlockChangePacket.java:6)

### `XaeroMinimapCore`

- [ ] `XaeroMinimapCore.modMain`: it uses IXaeroMinimap, which isn't translated (it's an interface) (xaero/common/core/XaeroMinimapCore.java:46)
- [ ] `XaeroMinimapCore.chunkCleanField`: it uses `Field` (java.lang.reflect) (xaero/common/core/XaeroMinimapCore.java:47)
- [ ] `XaeroMinimapCore.currentSession`: it uses XaeroMinimapSession, which isn't translated (it extends `HudSession` (xaero.hud)) (xaero/common/core/XaeroMinimapCore.java:48)
- [x] `XaeroMinimapCore.waypointsProjection` (xaero/common/core/XaeroMinimapCore.java:49)
- [x] `XaeroMinimapCore.waypointModelView` (xaero/common/core/XaeroMinimapCore.java:50)
- [ ] `XaeroMinimapCore.ensureField(0)`: it uses XaeroMinimapCore.chunkCleanField, which isn't translated (it uses `Field` (java.lang.reflect)) (xaero/common/core/XaeroMinimapCore.java:52)
- [ ] `XaeroMinimapCore.chunkUpdateCallback(2)`: it calls XaeroMinimapCore.ensureField, which isn't translated (xaero/common/core/XaeroMinimapCore.java:62)
- [ ] `XaeroMinimapCore.onHandleLevelChunkWithLight(1)`: it uses Minecraft's `ClientboundLevelChunkWithLightPacket` (xaero/common/core/XaeroMinimapCore.java:81)
- [ ] `XaeroMinimapCore.onHandleLightUpdatePacket(1)`: it uses Minecraft's `ClientboundLightUpdatePacket` (xaero/common/core/XaeroMinimapCore.java:85)
- [ ] `XaeroMinimapCore.onQueueLightRemoval(1)`: it uses Minecraft's `ClientboundForgetLevelChunkPacket` (xaero/common/core/XaeroMinimapCore.java:89)
- [ ] `XaeroMinimapCore.onBlockChange(1)`: it uses Minecraft's `ClientboundBlockUpdatePacket` (xaero/common/core/XaeroMinimapCore.java:93)
- [ ] `XaeroMinimapCore.onMultiBlockChange(1)`: it uses Minecraft's `ClientboundSectionBlocksUpdatePacket` (xaero/common/core/XaeroMinimapCore.java:97)
- [ ] `XaeroMinimapCore.onPlayNetHandler(2)`: it uses Minecraft's `ClientPacketListener` (xaero/common/core/XaeroMinimapCore.java:102)
- [ ] `XaeroMinimapCore.cleanupCurrentSession(0)`: a try/finally (xaero/common/core/XaeroMinimapCore.java:136)
- [ ] `XaeroMinimapCore.onPlayNetHandlerCleanup(1)`: it uses Minecraft's `ClientPacketListener` (xaero/common/core/XaeroMinimapCore.java:146)
- [ ] `XaeroMinimapCore.beforeRespawn(1)`: it calls XaeroMinimapCore.isModLoaded, which isn't translated (xaero/common/core/XaeroMinimapCore.java:170)
- [x] `XaeroMinimapCore.onProjectionMatrix(1)` (xaero/common/core/XaeroMinimapCore.java:181)
- [x] `XaeroMinimapCore.onWorldModelViewMatrix(1)` (xaero/common/core/XaeroMinimapCore.java:186)
- [x] `XaeroMinimapCore.onRenderLevelMatrices(2)` (xaero/common/core/XaeroMinimapCore.java:191)
- [ ] `XaeroMinimapCore.beforeIngameGuiRender(0)`: it calls XaeroMinimapCore.isModLoaded, which isn't translated (xaero/common/core/XaeroMinimapCore.java:196)
- [ ] `XaeroMinimapCore.afterIngameGuiRender(0)`: it calls XaeroMinimapCore.isModLoaded, which isn't translated (xaero/common/core/XaeroMinimapCore.java:202)
- [ ] `XaeroMinimapCore.onRenderStatusEffectOverlayPost(1)`: it calls XaeroMinimapCore.isModLoaded, which isn't translated (xaero/common/core/XaeroMinimapCore.java:208)
- [ ] `XaeroMinimapCore.onBossHealthRender(1)`: it calls XaeroMinimapCore.isModLoaded, which isn't translated (xaero/common/core/XaeroMinimapCore.java:217)
- [ ] `XaeroMinimapCore.onEntityIconsModelPrepareDetection(1)`: it uses Minecraft's `Submit` (xaero/common/core/XaeroMinimapCore.java:227)
- [ ] `XaeroMinimapCore.onEntityIconsModelRenderDetection(3)`: it uses Minecraft's `Model` (xaero/common/core/XaeroMinimapCore.java:233)
- [ ] `XaeroMinimapCore.onEntityIconsModelPartRenderDetection(2)`: it uses Minecraft's `ModelPart` (xaero/common/core/XaeroMinimapCore.java:239)
- [ ] `XaeroMinimapCore.onDeleteWorld(1)`: it uses Minecraft's `LevelStorageAccess` (xaero/common/core/XaeroMinimapCore.java:245)
- [ ] `XaeroMinimapCore.onSpawn(1)`: it uses Minecraft's `ClientboundSetDefaultSpawnPositionPacket` (xaero/common/core/XaeroMinimapCore.java:263)
- [ ] `XaeroMinimapCore.onLocalPlayerCommand(1)`: it calls XaeroMinimapCore.isModLoaded, which isn't translated (xaero/common/core/XaeroMinimapCore.java:269)
- [ ] `XaeroMinimapCore.onSystemChat(1)`: it calls XaeroMinimapCore.isModLoaded, which isn't translated (xaero/common/core/XaeroMinimapCore.java:273)
- [ ] `XaeroMinimapCore.onHandleDisguisedChatMessage(2)`: it uses Minecraft's `Bound` (xaero/common/core/XaeroMinimapCore.java:277)
- [ ] `XaeroMinimapCore.isModLoaded(0)`: it uses XaeroMinimapCore.modMain, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/core/XaeroMinimapCore.java:281)
- [ ] `XaeroMinimapCore.onRenderStatusEffectOverlay(1)`: it calls XaeroMinimapCore.isModLoaded, which isn't translated (xaero/common/core/XaeroMinimapCore.java:285)
- [ ] `XaeroMinimapCore.onRenderCrosshair(1)`: it calls XaeroMinimapCore.isModLoaded, which isn't translated (xaero/common/core/XaeroMinimapCore.java:289)
- [ ] `XaeroMinimapCore.handleRenderModOverlay(2)`: it calls XaeroMinimapCore.isModLoaded, which isn't translated (xaero/common/core/XaeroMinimapCore.java:293)
- [ ] `XaeroMinimapCore.onBufferSourceGetBuffer(2)`: it uses IBufferSource, which isn't translated (it's an interface) (xaero/common/core/XaeroMinimapCore.java:299)
- [ ] `XaeroMinimapCore.onToggleKeyIsDown(1)`: it uses Minecraft's `ToggleKeyMapping` (xaero/common/core/XaeroMinimapCore.java:305)

### `XaeroMinimapCoreFabric`


### `Effects`

- [ ] `Effects.NO_MINIMAP_UNHELD`: it uses Minecraft's `MobEffect` (xaero/common/effect/Effects.java:8)
- [ ] `Effects.NO_MINIMAP_HARMFUL_UNHELD`: it uses Minecraft's `MobEffect` (xaero/common/effect/Effects.java:9)
- [ ] `Effects.NO_RADAR_UNHELD`: it uses Minecraft's `MobEffect` (xaero/common/effect/Effects.java:10)
- [ ] `Effects.NO_RADAR_HARMFUL_UNHELD`: it uses Minecraft's `MobEffect` (xaero/common/effect/Effects.java:11)
- [ ] `Effects.NO_WAYPOINTS_UNHELD`: it uses Minecraft's `MobEffect` (xaero/common/effect/Effects.java:12)
- [ ] `Effects.NO_WAYPOINTS_HARMFUL_UNHELD`: it uses Minecraft's `MobEffect` (xaero/common/effect/Effects.java:13)
- [ ] `Effects.NO_CAVE_MAPS_UNHELD`: it uses Minecraft's `MobEffect` (xaero/common/effect/Effects.java:14)
- [ ] `Effects.NO_CAVE_MAPS_HARMFUL_UNHELD`: it uses Minecraft's `MobEffect` (xaero/common/effect/Effects.java:15)
- [ ] `Effects.NO_MINIMAP`: it uses Minecraft's `Holder` (xaero/common/effect/Effects.java:16)
- [ ] `Effects.NO_MINIMAP_HARMFUL`: it uses Minecraft's `Holder` (xaero/common/effect/Effects.java:17)
- [ ] `Effects.NO_RADAR`: it uses Minecraft's `Holder` (xaero/common/effect/Effects.java:18)
- [ ] `Effects.NO_RADAR_HARMFUL`: it uses Minecraft's `Holder` (xaero/common/effect/Effects.java:19)
- [ ] `Effects.NO_WAYPOINTS`: it uses Minecraft's `Holder` (xaero/common/effect/Effects.java:20)
- [ ] `Effects.NO_WAYPOINTS_HARMFUL`: it uses Minecraft's `Holder` (xaero/common/effect/Effects.java:21)
- [ ] `Effects.NO_CAVE_MAPS`: it uses Minecraft's `Holder` (xaero/common/effect/Effects.java:22)
- [ ] `Effects.NO_CAVE_MAPS_HARMFUL`: it uses Minecraft's `Holder` (xaero/common/effect/Effects.java:23)
- [ ] `Effects.init(0)`: it uses Effects.NO_MINIMAP, which isn't translated (it uses Minecraft's `Holder`) (xaero/common/effect/Effects.java:25)

### `EffectsRegister`

- [ ] `EffectsRegister.registerEffects(1)`: it uses MinimapStatusEffect, which isn't translated (it extends Minecraft's `MobEffect`) (xaero/common/effect/EffectsRegister.java:8)

### `MinimapStatusEffect`: it extends Minecraft's `MobEffect`

- [ ] `MinimapStatusEffect.MinimapStatusEffect(3)`: its class isn't translated (it extends Minecraft's `MobEffect`) (xaero/common/effect/MinimapStatusEffect.java:10)
- [ ] `MinimapStatusEffect.setRegistryName(1)`: its class isn't translated (it extends Minecraft's `MobEffect`) (xaero/common/effect/MinimapStatusEffect.java:16)
- [ ] `MinimapStatusEffect.getRegistryName(0)`: its class isn't translated (it extends Minecraft's `MobEffect`) (xaero/common/effect/MinimapStatusEffect.java:20)

### `NoCaveMapsEffect`: it extends `MinimapStatusEffect`

- [ ] `NoCaveMapsEffect.NoCaveMapsEffect(1)`: its class isn't translated (it extends `MinimapStatusEffect`) (xaero/common/effect/NoCaveMapsEffect.java:6)

### `NoMinimapEffect`: it extends `MinimapStatusEffect`

- [ ] `NoMinimapEffect.NoMinimapEffect(1)`: its class isn't translated (it extends `MinimapStatusEffect`) (xaero/common/effect/NoMinimapEffect.java:6)

### `NoRadarEffect`: it extends `MinimapStatusEffect`

- [ ] `NoRadarEffect.NoRadarEffect(1)`: its class isn't translated (it extends `MinimapStatusEffect`) (xaero/common/effect/NoRadarEffect.java:6)

### `NoWaypointsEffect`: it extends `MinimapStatusEffect`

- [ ] `NoWaypointsEffect.NoWaypointsEffect(1)`: its class isn't translated (it extends `MinimapStatusEffect`) (xaero/common/effect/NoWaypointsEffect.java:6)

### `ClientEvents`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)

- [ ] `ClientEvents.ClientEvents(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:61)
- [ ] `ClientEvents.handleGuiOpen(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:65)
- [ ] `ClientEvents.handleRenderGameOverlayEventPre(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:111)
- [ ] `ClientEvents.handleRenderGameOverlayEventPost(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:137)
- [ ] `ClientEvents.handleClientSendChatEvent(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:145)
- [ ] `ClientEvents.handleClientPlayerChatReceivedEvent(3)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:160)
- [ ] `ClientEvents.handleClientSystemChatReceivedEvent(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:166)
- [ ] `ClientEvents.handleChatMessage(2)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:198)
- [ ] `ClientEvents.handleDrawScreenEventPost(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:213)
- [ ] `ClientEvents.handlePlayerSetSpawnEvent(2)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:219)
- [ ] `ClientEvents.getLastGuiOpen(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:231)
- [ ] `ClientEvents.worldUnload(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:235)
- [ ] `ClientEvents.handleClientTickStart(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:245)
- [ ] `ClientEvents.handlePlayerTickStart(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:259)
- [ ] `ClientEvents.handleRenderTickStart(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:279)
- [ ] `ClientEvents.handleRenderStatusEffectOverlay(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:302)
- [ ] `ClientEvents.handleRenderCrosshairOverlay(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:306)
- [ ] `ClientEvents.handleForceToggleKeyMapping(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:314)
- [ ] `ClientEvents.handleTextureStitchEventPost(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:318)
- [ ] `ClientEvents.handleTextureStitchEventPost_onReset(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEvents.java:335)

### `ClientEventsFabric`: it extends `ClientEvents`

- [ ] `ClientEventsFabric.ClientEventsFabric(1)`: its class isn't translated (it extends `ClientEvents`) (xaero/common/events/ClientEventsFabric.java:6)
- [ ] `ClientEventsFabric.handleTextureStitchEventPost_onReset(0)`: its class isn't translated (it extends `ClientEvents`) (xaero/common/events/ClientEventsFabric.java:10)

### `ClientEventsListener`

- [ ] `ClientEventsListener.playerTickPost(1)`: it uses HudSession, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEventsListener.java:8)
- [ ] `ClientEventsListener.clientTickPost(1)`: it uses HudSession, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/ClientEventsListener.java:11)
- [ ] `ClientEventsListener.handleRenderStatusEffectOverlay(1)`: it uses Minecraft's `GuiGraphicsExtractor` (xaero/common/events/ClientEventsListener.java:14)
- [ ] `ClientEventsListener.handleForceToggleKeyMapping(1)`: it uses Minecraft's `ToggleKeyMapping` (xaero/common/events/ClientEventsListener.java:18)

### `CommonEvents`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)

- [ ] `CommonEvents.CommonEvents(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/CommonEvents.java:29)
- [ ] `CommonEvents.onPlayerClone(3)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/CommonEvents.java:33)
- [ ] `CommonEvents.onServerStarting(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/CommonEvents.java:39)
- [ ] `CommonEvents.onServerStopped(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/CommonEvents.java:45)
- [ ] `CommonEvents.onPlayerLogIn(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/CommonEvents.java:48)
- [ ] `CommonEvents.onPlayerWorldJoin(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/CommonEvents.java:56)
- [ ] `CommonEvents.handlePlayerTickStart(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/common/events/CommonEvents.java:84)

### `CommonEventsFabric`: it extends `CommonEvents`

- [ ] `CommonEventsFabric.CommonEventsFabric(1)`: its class isn't translated (it extends `CommonEvents`) (xaero/common/events/CommonEventsFabric.java:8)
- [ ] `CommonEventsFabric.register(0)`: its class isn't translated (it extends `CommonEvents`) (xaero/common/events/CommonEventsFabric.java:12)

### `ModClientEvents`: it uses IXaeroMinimap, which isn't translated (it's an interface)

- [ ] `ModClientEvents.ModClientEvents(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/events/ModClientEvents.java:15)
- [ ] `ModClientEvents.handleRenderModOverlay(2)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/events/ModClientEvents.java:19)

### `ModClientEventsFabric`: it extends `ModClientEvents`

- [ ] `ModClientEventsFabric.ModClientEventsFabric(1)`: its class isn't translated (it extends `ModClientEvents`) (xaero/common/events/ModClientEventsFabric.java:9)
- [ ] `ModClientEventsFabric.register(0)`: its class isn't translated (it extends `ModClientEvents`) (xaero/common/events/ModClientEventsFabric.java:13)

### `ModCommonEvents`: it uses IXaeroMinimap, which isn't translated (it's an interface)

- [ ] `ModCommonEvents.ModCommonEvents(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/events/ModCommonEvents.java:8)

### `ModCommonEventsFabric`: it extends `ModCommonEvents`

- [ ] `ModCommonEventsFabric.ModCommonEventsFabric(1)`: its class isn't translated (it extends `ModCommonEvents`) (xaero/common/events/ModCommonEventsFabric.java:6)

### `SilentException`: it extends `Exception`

- [x] `SilentException.serialVersionUID` (xaero/common/exception/SilentException.java:4)
- [ ] `SilentException.SilentException(1)`: its class isn't translated (it extends `Exception`) (xaero/common/exception/SilentException.java:6)

### `SimpleBackup`

- [ ] `SimpleBackup.moveToBackup(1)`: it calls SimpleBackup.moveToBackup, which isn't translated (xaero/common/file/SimpleBackup.java:14)
- [ ] `SimpleBackup.moveToBackup(2)`: a catch block that uses its exception (xaero/common/file/SimpleBackup.java:18)
- [ ] `SimpleBackup.copyDirectoryWithContents(4)`: its body couldn't be read: an anonymous class (xaero/common/file/SimpleBackup.java:36)

### `CustomRenderTypes`

- [ ] `CustomRenderTypes.GUI_NEAREST`: it uses Minecraft's `RenderType` (xaero/common/graphics/CustomRenderTypes.java:24)
- [ ] `CustomRenderTypes.GUI_NEAREST_PRE`: it uses Minecraft's `RenderType` (xaero/common/graphics/CustomRenderTypes.java:25)
- [ ] `CustomRenderTypes.GUI_BILINEAR`: it uses Minecraft's `RenderType` (xaero/common/graphics/CustomRenderTypes.java:26)
- [ ] `CustomRenderTypes.GUI_BILINEAR_PRE`: it uses Minecraft's `RenderType` (xaero/common/graphics/CustomRenderTypes.java:27)
- [ ] `CustomRenderTypes.GUI_BILINEAR_NO_DEPTH`: it uses Minecraft's `RenderType` (xaero/common/graphics/CustomRenderTypes.java:28)
- [ ] `CustomRenderTypes.COLORED_WAYPOINTS_BGS`: it uses Minecraft's `RenderType` (xaero/common/graphics/CustomRenderTypes.java:29)
- [ ] `CustomRenderTypes.MAP_CHUNK_OVERLAY`: it uses Minecraft's `RenderType` (xaero/common/graphics/CustomRenderTypes.java:33)
- [ ] `CustomRenderTypes.MAP_LINES`: it uses Minecraft's `RenderType` (xaero/common/graphics/CustomRenderTypes.java:36)
- [ ] `CustomRenderTypes.MINIMAP_WORLD_MAP`: it uses Minecraft's `RenderType` (xaero/common/graphics/CustomRenderTypes.java:37)
- [ ] `CustomRenderTypes.MINIMAP_WORLD_MAP_ZOOM`: it uses Minecraft's `RenderType` (xaero/common/graphics/CustomRenderTypes.java:38)
- [ ] `CustomRenderTypes.MINIMAP`: it uses Minecraft's `RenderType` (xaero/common/graphics/CustomRenderTypes.java:39)
- [ ] `CustomRenderTypes.MINIMAP_ZOOM`: it uses Minecraft's `RenderType` (xaero/common/graphics/CustomRenderTypes.java:40)
- [ ] `CustomRenderTypes.RADAR_NAME_BGS`: it uses Minecraft's `RenderType` (xaero/common/graphics/CustomRenderTypes.java:41)
- [ ] `CustomRenderTypes.DEPTH_CLEAR`: it uses Minecraft's `RenderType` (xaero/common/graphics/CustomRenderTypes.java:45)
- [ ] `CustomRenderTypes.entityIconRenderType(7)`: it uses Minecraft's `RenderType` (xaero/common/graphics/CustomRenderTypes.java:47)
- [ ] `CustomRenderTypes.applyFixedOrder(0)`: it uses `XaeroBufferProvider` (xaero.lib.client.graphics) (xaero/common/graphics/CustomRenderTypes.java:76)

### `ImprovedFramebuffer`: it extends Minecraft's `TextureTarget`

- [ ] `ImprovedFramebuffer.MAIN_RENDER_TARGET_FIELD`: it uses `Field` (java.lang.reflect) (xaero/common/graphics/ImprovedFramebuffer.java:21)
- [ ] `ImprovedFramebuffer.mainRenderTargetBackup`: it uses Minecraft's `RenderTarget` (xaero/common/graphics/ImprovedFramebuffer.java:24)
- [ ] `ImprovedFramebuffer.outputColorTextureOverrideBU`: it uses Minecraft's `GpuTextureView` (xaero/common/graphics/ImprovedFramebuffer.java:25)
- [ ] `ImprovedFramebuffer.outputDepthTextureOverrideBU`: it uses Minecraft's `GpuTextureView` (xaero/common/graphics/ImprovedFramebuffer.java:26)
- [ ] `ImprovedFramebuffer.ImprovedFramebuffer(4)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/common/graphics/ImprovedFramebuffer.java:30)
- [ ] `ImprovedFramebuffer.bindDefaultFramebuffer(1)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/common/graphics/ImprovedFramebuffer.java:38)
- [ ] `ImprovedFramebuffer.generateMipmaps(0)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/common/graphics/ImprovedFramebuffer.java:42)
- [ ] `ImprovedFramebuffer.forceAsMainRenderTarget(0)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/common/graphics/ImprovedFramebuffer.java:48)
- [x] `ImprovedFramebuffer.restoreMainRenderTarget(0)` (xaero/common/graphics/ImprovedFramebuffer.java:60)
- [ ] `ImprovedFramebuffer.bindAsMainTarget(1)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/common/graphics/ImprovedFramebuffer.java:74)
- [ ] `ImprovedFramebuffer.setColorTexture(2)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/common/graphics/ImprovedFramebuffer.java:78)
- [ ] `ImprovedFramebuffer.setColorTexture(1)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/common/graphics/ImprovedFramebuffer.java:94)
- [ ] `ImprovedFramebuffer.setDepthTexture(2)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/common/graphics/ImprovedFramebuffer.java:98)
- [ ] `ImprovedFramebuffer.getSampler(0)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/common/graphics/ImprovedFramebuffer.java:103)
- [ ] `ImprovedFramebuffer.setSampler(1)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/common/graphics/ImprovedFramebuffer.java:107)
- [ ] `ImprovedFramebuffer.closeColorTexture(0)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/common/graphics/ImprovedFramebuffer.java:116)
- [ ] `ImprovedFramebuffer.closeDepthTexture(0)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/common/graphics/ImprovedFramebuffer.java:121)
- [ ] `ImprovedFramebuffer.getTas(0)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/common/graphics/ImprovedFramebuffer.java:126)
- [ ] `ImprovedFramebuffer.getFormat(0)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/common/graphics/ImprovedFramebuffer.java:130)

### `MinimapTexture`: it extends Minecraft's `SimpleTexture`

- [ ] `MinimapTexture.loadIfNeeded(0)`: its class isn't translated (it extends Minecraft's `SimpleTexture`) (xaero/common/graphics/MinimapTexture.java:27)
- [ ] `MinimapTexture.MinimapTexture(1)`: its class isn't translated (it extends Minecraft's `SimpleTexture`) (xaero/common/graphics/MinimapTexture.java:34)
- [ ] `MinimapTexture.load(1)`: its class isn't translated (it extends Minecraft's `SimpleTexture`) (xaero/common/graphics/MinimapTexture.java:38)
- [ ] `MinimapTexture.getTextureAndView(0)`: its class isn't translated (it extends Minecraft's `SimpleTexture`) (xaero/common/graphics/MinimapTexture.java:50)
- [ ] `MinimapTexture.getTas(0)`: its class isn't translated (it extends Minecraft's `SimpleTexture`) (xaero/common/graphics/MinimapTexture.java:54)

### `PixelBuffers`

- [x] `PixelBuffers.buffersType` (xaero/common/graphics/PixelBuffers.java:8)
- [ ] `PixelBuffers.glGenBuffers(0)`: it calls `GL15` (org.lwjgl.opengl).glGenBuffers (xaero/common/graphics/PixelBuffers.java:10)
- [ ] `PixelBuffers.glBindBuffer(2)`: it calls `GL15` (org.lwjgl.opengl).glBindBuffer (xaero/common/graphics/PixelBuffers.java:19)
- [ ] `PixelBuffers.glBufferData(3)`: it calls `GL15` (org.lwjgl.opengl).glBufferData (xaero/common/graphics/PixelBuffers.java:26)
- [ ] `PixelBuffers.glMapBuffer(4)`: it uses `ByteBuffer` (java.nio) (xaero/common/graphics/PixelBuffers.java:33)
- [ ] `PixelBuffers.glUnmapBuffer(1)`: it calls `GL15` (org.lwjgl.opengl).glUnmapBuffer (xaero/common/graphics/PixelBuffers.java:42)
- [ ] `PixelBuffers.glDeleteBuffers(1)`: it's overloaded with the same number of parameters (xaero/common/graphics/PixelBuffers.java:51)
- [ ] `PixelBuffers.glDeleteBuffers(1)`: it's overloaded with the same number of parameters (xaero/common/graphics/PixelBuffers.java:58)
- [ ] `PixelBuffers.glMapBuffer(2)`: it uses `ByteBuffer` (java.nio) (xaero/common/graphics/PixelBuffers.java:65)

### `MultiTextureRenderTypeRenderer`: it uses Minecraft's `ByteBufferBuilder`

- [x] `MultiTextureRenderTypeRenderer.RENDER_PASS_NAME` (xaero/common/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java:31)
- [ ] `MultiTextureRenderTypeRenderer.MultiTextureRenderTypeRenderer(0)`: its class isn't translated (it uses Minecraft's `ByteBufferBuilder`) (xaero/common/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java:44)
- [ ] `MultiTextureRenderTypeRenderer.init(3)`: its class isn't translated (it uses Minecraft's `ByteBufferBuilder`) (xaero/common/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java:47)
- [ ] `MultiTextureRenderTypeRenderer.draw(0)`: its class isn't translated (it uses Minecraft's `ByteBufferBuilder`) (xaero/common/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java:59)
- [ ] `MultiTextureRenderTypeRenderer.uploadImmediateVertexBuffer(1)`: its class isn't translated (it uses Minecraft's `ByteBufferBuilder`) (xaero/common/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java:176)
- [ ] `MultiTextureRenderTypeRenderer.endBuffer(1)`: its class isn't translated (it uses Minecraft's `ByteBufferBuilder`) (xaero/common/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java:187)
- [ ] `MultiTextureRenderTypeRenderer.begin(1)`: its class isn't translated (it uses Minecraft's `ByteBufferBuilder`) (xaero/common/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java:191)

### `MultiTextureRenderTypeRenderer.DrawCall`: it uses Minecraft's `GpuBufferSlice`

- [ ] `MultiTextureRenderTypeRenderer.DrawCall.DrawCall(5)`: its class isn't translated (it uses Minecraft's `GpuBufferSlice`) (xaero/common/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java:220)

### `MultiTextureRenderTypeRendererProvider`: it uses `Deque` (java.util)

- [ ] `MultiTextureRenderTypeRendererProvider.MultiTextureRenderTypeRendererProvider(1)`: its class isn't translated (it uses `Deque` (java.util)) (xaero/common/graphics/renderer/multitexture/MultiTextureRenderTypeRendererProvider.java:14)
- [ ] `MultiTextureRenderTypeRendererProvider.getRenderer(1)`: its class isn't translated (it uses `Deque` (java.util)) (xaero/common/graphics/renderer/multitexture/MultiTextureRenderTypeRendererProvider.java:22)
- [ ] `MultiTextureRenderTypeRendererProvider.getRenderer(2)`: its class isn't translated (it uses `Deque` (java.util)) (xaero/common/graphics/renderer/multitexture/MultiTextureRenderTypeRendererProvider.java:26)
- [ ] `MultiTextureRenderTypeRendererProvider.getRenderer(2)`: its class isn't translated (it uses `Deque` (java.util)) (xaero/common/graphics/renderer/multitexture/MultiTextureRenderTypeRendererProvider.java:30)
- [ ] `MultiTextureRenderTypeRendererProvider.getRenderer(3)`: its class isn't translated (it uses `Deque` (java.util)) (xaero/common/graphics/renderer/multitexture/MultiTextureRenderTypeRendererProvider.java:34)
- [ ] `MultiTextureRenderTypeRendererProvider.draw(1)`: its class isn't translated (it uses `Deque` (java.util)) (xaero/common/graphics/renderer/multitexture/MultiTextureRenderTypeRendererProvider.java:45)

### `CachedComponentSupplier`: it uses `Object`

- [ ] `CachedComponentSupplier.CachedComponentSupplier(1)`: its class isn't translated (it uses `Object`) (xaero/common/gui/component/CachedComponentSupplier.java:15)
- [ ] `CachedComponentSupplier.get(1)`: its class isn't translated (it uses `Object`) (xaero/common/gui/component/CachedComponentSupplier.java:19)

### `GuiAddWaypoint`: it extends `ScreenBase` (xaero.lib.client.gui)

- [ ] `GuiAddWaypoint.VISIBILITY_TYPE_TOOLTIP`: it uses `Tooltip` (xaero.lib.client.gui.widget) (xaero/common/gui/GuiAddWaypoint.java:51)
- [ ] `GuiAddWaypoint.TYPE_TOOLTIP`: it uses `Tooltip` (xaero.lib.client.gui.widget) (xaero/common/gui/GuiAddWaypoint.java:52)
- [ ] `GuiAddWaypoint.GuiAddWaypoint(15)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:112)
- [ ] `GuiAddWaypoint.GuiAddWaypoint(9)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:177)
- [ ] `GuiAddWaypoint.GuiAddWaypoint(7)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:207)
- [ ] `GuiAddWaypoint.fillFormWaypoint(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:219)
- [ ] `GuiAddWaypoint.getDimDiv(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:235)
- [ ] `GuiAddWaypoint.getAutomaticX(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:240)
- [ ] `GuiAddWaypoint.getAutomaticYInput(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:245)
- [ ] `GuiAddWaypoint.getAutomaticZ(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:254)
- [ ] `GuiAddWaypoint.fillFormAutomatic(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:259)
- [ ] `GuiAddWaypoint.createForms(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:267)
- [ ] `GuiAddWaypoint.resetCurrentForm(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:287)
- [ ] `GuiAddWaypoint.updateMutual(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:300)
- [ ] `GuiAddWaypoint.confirmMutual(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:372)
- [ ] `GuiAddWaypoint.differentValues(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:417)
- [ ] `GuiAddWaypoint.createColorOptions(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:434)
- [ ] `GuiAddWaypoint.getMutualEditConfig(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:453)
- [ ] `GuiAddWaypoint.setMutualEditConfig(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:458)
- [ ] `GuiAddWaypoint.init(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:463)
- [ ] `GuiAddWaypoint.setInitialFocus(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:808)
- [ ] `GuiAddWaypoint.applyEditBoxResponder(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:811)
- [ ] `GuiAddWaypoint.createSetsDropdown(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:820)
- [ ] `GuiAddWaypoint.createContainersDropdown(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:833)
- [ ] `GuiAddWaypoint.createWorldsDropdown(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:846)
- [ ] `GuiAddWaypoint.getDisableButtonText(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:859)
- [ ] `GuiAddWaypoint.getCurrent(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:871)
- [ ] `GuiAddWaypoint.keyPressed(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:875)
- [ ] `GuiAddWaypoint.charTyped(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:892)
- [ ] `GuiAddWaypoint.mouseClicked(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:904)
- [ ] `GuiAddWaypoint.mouseReleased(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:914)
- [ ] `GuiAddWaypoint.preType(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:924)
- [ ] `GuiAddWaypoint.postType(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:930)
- [ ] `GuiAddWaypoint.setFocused(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:946)
- [ ] `GuiAddWaypoint.canConfirm(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:956)
- [ ] `GuiAddWaypoint.updateConfirmButton(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:961)
- [ ] `GuiAddWaypoint.handleCoordinateInputSpaces(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:967)
- [ ] `GuiAddWaypoint.checkFields(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:984)
- [ ] `GuiAddWaypoint.tick(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:1025)
- [ ] `GuiAddWaypoint.extractBackground(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:1031)
- [ ] `GuiAddWaypoint.extractRenderState(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:1036)
- [ ] `GuiAddWaypoint.renderPreDropdown(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:1041)
- [ ] `GuiAddWaypoint.onSelected(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiAddWaypoint.java:1124)

### `GuiChoosePreset`: it extends `GuiSettings` (xaero.lib.client.gui)

- [ ] `GuiChoosePreset.GuiChoosePreset(3)`: its class isn't translated (it extends `GuiSettings` (xaero.lib.client.gui)) (xaero/common/gui/GuiChoosePreset.java:13)
- [ ] `GuiChoosePreset.goBack(0)`: its class isn't translated (it extends `GuiSettings` (xaero.lib.client.gui)) (xaero/common/gui/GuiChoosePreset.java:24)

### `GuiClearSet`: it extends Minecraft's `ConfirmScreen`

- [ ] `GuiClearSet.GuiClearSet(7)`: its class isn't translated (it extends Minecraft's `ConfirmScreen`) (xaero/common/gui/GuiClearSet.java:17)
- [ ] `GuiClearSet.confirmClearSet(7)`: it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)) (xaero/common/gui/GuiClearSet.java:25)

### `GuiDeleteSet`: it extends Minecraft's `ConfirmScreen`

- [ ] `GuiDeleteSet.GuiDeleteSet(7)`: its class isn't translated (it extends Minecraft's `ConfirmScreen`) (xaero/common/gui/GuiDeleteSet.java:17)
- [ ] `GuiDeleteSet.confirmDeleteSet(7)`: it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)) (xaero/common/gui/GuiDeleteSet.java:25)

### `GuiDropdownHelper`: it's generic

- [ ] `GuiDropdownHelper.getCurrentKey(0)`: its class isn't translated (it's generic) (xaero/common/gui/GuiDropdownHelper.java:9)
- [ ] `GuiDropdownHelper.getCurrentName(0)`: its class isn't translated (it's generic) (xaero/common/gui/GuiDropdownHelper.java:13)

### `GuiEditMode`: it extends `ScreenBase` (xaero.lib.client.gui)

- [x] `GuiEditMode.CENTERED_COMPONENT` in game_side.rs (xaero/common/gui/GuiEditMode.java:23)
- [x] `GuiEditMode.FLIPPED_COMPONENT` in game_side.rs (xaero/common/gui/GuiEditMode.java:24)
- [x] `GuiEditMode.TRUE_COMPONENT` in game_side.rs (xaero/common/gui/GuiEditMode.java:25)
- [x] `GuiEditMode.FALSE_COMPONENT` in game_side.rs (xaero/common/gui/GuiEditMode.java:26)
- [x] `GuiEditMode.PRESS_C_COMPONENT` in game_side.rs (xaero/common/gui/GuiEditMode.java:27)
- [x] `GuiEditMode.PRESS_F_COMPONENT` in game_side.rs (xaero/common/gui/GuiEditMode.java:28)
- [x] `GuiEditMode.NOT_INGAME` in game_side.rs (xaero/common/gui/GuiEditMode.java:29)
- [ ] `GuiEditMode.GuiEditMode(5)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiEditMode.java:42)
- [ ] `GuiEditMode.init(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiEditMode.java:49)
- [ ] `GuiEditMode.confirm(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiEditMode.java:85)
- [ ] `GuiEditMode.cancel(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiEditMode.java:105)
- [ ] `GuiEditMode.extractBackground(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiEditMode.java:119)
- [ ] `GuiEditMode.applyPushes(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiEditMode.java:128)
- [ ] `GuiEditMode.extractRenderState(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiEditMode.java:142)
- [ ] `GuiEditMode.renderModuleBox(4)`: it's generic (xaero/common/gui/GuiEditMode.java:169)
- [ ] `GuiEditMode.getHoveredModule(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiEditMode.java:182)
- [ ] `GuiEditMode.isHovered(3)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiEditMode.java:201)
- [ ] `GuiEditMode.mouseClicked(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiEditMode.java:208)
- [ ] `GuiEditMode.mouseReleased(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiEditMode.java:228)
- [ ] `GuiEditMode.keyPressed(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiEditMode.java:233)
- [ ] `GuiEditMode.handleDraggedModule(3)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiEditMode.java:263)
- [ ] `GuiEditMode.getTooltipText(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiEditMode.java:291)

### `GuiEntityRadarCategoryEditor`: it extends `GuiCategoryEditor` (xaero.hud.category.ui)

- [x] `GuiEntityRadarCategoryEditor.SERVER_ENFORCED_COMPONENT` in game_side.rs (xaero/common/gui/GuiEntityRadarCategoryEditor.java:19)
- [ ] `GuiEntityRadarCategoryEditor.GuiEntityRadarCategoryEditor(5)`: its class isn't translated (it extends `GuiCategoryEditor` (xaero.hud.category.ui)) (xaero/common/gui/GuiEntityRadarCategoryEditor.java:26)
- [ ] `GuiEntityRadarCategoryEditor.constructEditorData(1)`: its class isn't translated (it extends `GuiCategoryEditor` (xaero.hud.category.ui)) (xaero/common/gui/GuiEntityRadarCategoryEditor.java:41)
- [ ] `GuiEntityRadarCategoryEditor.constructDefaultData(1)`: its class isn't translated (it extends `GuiCategoryEditor` (xaero.hud.category.ui)) (xaero/common/gui/GuiEntityRadarCategoryEditor.java:50)
- [ ] `GuiEntityRadarCategoryEditor.onConfigConfirmed(1)`: its class isn't translated (it extends `GuiCategoryEditor` (xaero.hud.category.ui)) (xaero/common/gui/GuiEntityRadarCategoryEditor.java:60)
- [ ] `GuiEntityRadarCategoryEditor.extractRenderState(4)`: its class isn't translated (it extends `GuiCategoryEditor` (xaero.hud.category.ui)) (xaero/common/gui/GuiEntityRadarCategoryEditor.java:65)

### `GuiEntityRadarSettings`: it extends `GuiMinimapSettings`

- [ ] `GuiEntityRadarSettings.GuiEntityRadarSettings(4)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiEntityRadarSettings.java:18)

### `GuiHelper`: it uses IXaeroMinimap, which isn't translated (it's an interface)

- [ ] `GuiHelper.GuiHelper(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/gui/GuiHelper.java:15)
- [ ] `GuiHelper.getMinimapSettingsFromScreen(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/gui/GuiHelper.java:19)
- [ ] `GuiHelper.openMinimapSettingsFromScreen(2)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/gui/GuiHelper.java:25)
- [ ] `GuiHelper.getMainSettingsScreen(1)`: it has no body (xaero/common/gui/GuiHelper.java:30)
- [ ] `GuiHelper.openMainSettingsFromScreen(1)`: it has no body (xaero/common/gui/GuiHelper.java:32)

### `GuiInfoDisplayEdit`: it extends `ScreenBase` (xaero.lib.client.gui)

- [x] `GuiInfoDisplayEdit.FRAME_TOP_SIZE` (xaero/common/gui/GuiInfoDisplayEdit.java:40)
- [x] `GuiInfoDisplayEdit.FRAME_BOTTOM_SIZE` (xaero/common/gui/GuiInfoDisplayEdit.java:41)
- [x] `GuiInfoDisplayEdit.SELECTION_ITEM_HEIGHT` (xaero/common/gui/GuiInfoDisplayEdit.java:42)
- [x] `GuiInfoDisplayEdit.HELP_COMPONENT` in game_side.rs (xaero/common/gui/GuiInfoDisplayEdit.java:43)
- [x] `GuiInfoDisplayEdit.SERVER_ENFORCED_COMPONENT` in game_side.rs (xaero/common/gui/GuiInfoDisplayEdit.java:44)
- [ ] `GuiInfoDisplayEdit.GuiInfoDisplayEdit(5)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiInfoDisplayEdit.java:59)
- [ ] `GuiInfoDisplayEdit.init(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiInfoDisplayEdit.java:77)
- [ ] `GuiInfoDisplayEdit.createEntryFor(1)`: it's generic (xaero/common/gui/GuiInfoDisplayEdit.java:95)
- [ ] `GuiInfoDisplayEdit.refreshEntry(1)`: it's generic (xaero/common/gui/GuiInfoDisplayEdit.java:109)
- [ ] `GuiInfoDisplayEdit.onExit(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiInfoDisplayEdit.java:114)
- [ ] `GuiInfoDisplayEdit.saveToConfigProfile(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiInfoDisplayEdit.java:121)
- [ ] `GuiInfoDisplayEdit.encodeState(1)`: it's generic (xaero/common/gui/GuiInfoDisplayEdit.java:135)
- [ ] `GuiInfoDisplayEdit.createColorOptions(3)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiInfoDisplayEdit.java:139)
- [ ] `GuiInfoDisplayEdit.addSubElements(1)`: it's generic (xaero/common/gui/GuiInfoDisplayEdit.java:157)
- [ ] `GuiInfoDisplayEdit.mouseReleased(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiInfoDisplayEdit.java:237)
- [ ] `GuiInfoDisplayEdit.setFocused(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiInfoDisplayEdit.java:245)
- [ ] `GuiInfoDisplayEdit.extractBackground(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiInfoDisplayEdit.java:249)

### `GuiInfoDisplayEdit.MoveableEntry`: it's generic

- [ ] `GuiInfoDisplayEdit.MoveableEntry.MoveableEntry(1)`: its class isn't translated (it's generic) (xaero/common/gui/GuiInfoDisplayEdit.java:269)
- [ ] `GuiInfoDisplayEdit.MoveableEntry.addSubElement(1)`: its class isn't translated (it's generic) (xaero/common/gui/GuiInfoDisplayEdit.java:274)
- [ ] `GuiInfoDisplayEdit.MoveableEntry.getState(0)`: its class isn't translated (it's generic) (xaero/common/gui/GuiInfoDisplayEdit.java:278)
- [ ] `GuiInfoDisplayEdit.MoveableEntry.setState(1)`: its class isn't translated (it's generic) (xaero/common/gui/GuiInfoDisplayEdit.java:282)
- [ ] `GuiInfoDisplayEdit.MoveableEntry.clearSubElements(0)`: its class isn't translated (it's generic) (xaero/common/gui/GuiInfoDisplayEdit.java:286)

### `GuiInfoDisplayEdit.SelectionList`: it extends Minecraft's `ObjectSelectionList`

- [x] `GuiInfoDisplayEdit.SelectionList.USAGE_NARRATION` in game_side.rs (xaero/common/gui/GuiInfoDisplayEdit.java:292)
- [x] `GuiInfoDisplayEdit.SelectionList.LEFT_RIGHT_USAGE` in game_side.rs (xaero/common/gui/GuiInfoDisplayEdit.java:293)
- [ ] `GuiInfoDisplayEdit.SelectionList.SelectionList(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiInfoDisplayEdit.java:301)
- [ ] `GuiInfoDisplayEdit.SelectionList.isFocused(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiInfoDisplayEdit.java:311)
- [ ] `GuiInfoDisplayEdit.SelectionList.setFocused(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiInfoDisplayEdit.java:315)
- [ ] `GuiInfoDisplayEdit.SelectionList.setSelected(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiInfoDisplayEdit.java:336)
- [ ] `GuiInfoDisplayEdit.SelectionList.updateWidgetNarration(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiInfoDisplayEdit.java:340)
- [ ] `GuiInfoDisplayEdit.SelectionList.createEntries(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiInfoDisplayEdit.java:347)
- [ ] `GuiInfoDisplayEdit.SelectionList.releaseDrag(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiInfoDisplayEdit.java:354)
- [ ] `GuiInfoDisplayEdit.SelectionList.scrollBarX(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiInfoDisplayEdit.java:359)
- [ ] `GuiInfoDisplayEdit.SelectionList.getRowWidth(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiInfoDisplayEdit.java:363)
- [ ] `GuiInfoDisplayEdit.SelectionList.extractWidgetRenderState(4)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiInfoDisplayEdit.java:367)
- [ ] `GuiInfoDisplayEdit.SelectionList.nextFocusPath(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiInfoDisplayEdit.java:392)

### `GuiInfoDisplayEdit.SelectionList.Entry`: it extends `Entry`

- [ ] `GuiInfoDisplayEdit.SelectionList.Entry.Entry(1)`: its class isn't translated (it extends `Entry`) (xaero/common/gui/GuiInfoDisplayEdit.java:408)
- [ ] `GuiInfoDisplayEdit.SelectionList.Entry.renderNonInteractable(3)`: its class isn't translated (it extends `Entry`) (xaero/common/gui/GuiInfoDisplayEdit.java:412)
- [ ] `GuiInfoDisplayEdit.SelectionList.Entry.getMoveable(0)`: its class isn't translated (it extends `Entry`) (xaero/common/gui/GuiInfoDisplayEdit.java:418)
- [ ] `GuiInfoDisplayEdit.SelectionList.Entry.extractContent(5)`: its class isn't translated (it extends `Entry`) (xaero/common/gui/GuiInfoDisplayEdit.java:423)
- [ ] `GuiInfoDisplayEdit.SelectionList.Entry.mouseClicked(2)`: its class isn't translated (it extends `Entry`) (xaero/common/gui/GuiInfoDisplayEdit.java:445)
- [ ] `GuiInfoDisplayEdit.SelectionList.Entry.mouseReleased(1)`: its class isn't translated (it extends `Entry`) (xaero/common/gui/GuiInfoDisplayEdit.java:479)
- [ ] `GuiInfoDisplayEdit.SelectionList.Entry.mouseMoved(2)`: its class isn't translated (it extends `Entry`) (xaero/common/gui/GuiInfoDisplayEdit.java:489)
- [ ] `GuiInfoDisplayEdit.SelectionList.Entry.mouseDragged(3)`: its class isn't translated (it extends `Entry`) (xaero/common/gui/GuiInfoDisplayEdit.java:503)
- [ ] `GuiInfoDisplayEdit.SelectionList.Entry.mouseScrolled(4)`: its class isn't translated (it extends `Entry`) (xaero/common/gui/GuiInfoDisplayEdit.java:519)
- [ ] `GuiInfoDisplayEdit.SelectionList.Entry.keyPressed(1)`: its class isn't translated (it extends `Entry`) (xaero/common/gui/GuiInfoDisplayEdit.java:531)
- [ ] `GuiInfoDisplayEdit.SelectionList.Entry.keyReleased(1)`: its class isn't translated (it extends `Entry`) (xaero/common/gui/GuiInfoDisplayEdit.java:563)
- [ ] `GuiInfoDisplayEdit.SelectionList.Entry.charTyped(1)`: its class isn't translated (it extends `Entry`) (xaero/common/gui/GuiInfoDisplayEdit.java:570)
- [ ] `GuiInfoDisplayEdit.SelectionList.Entry.updateNarration(1)`: its class isn't translated (it extends `Entry`) (xaero/common/gui/GuiInfoDisplayEdit.java:577)
- [ ] `GuiInfoDisplayEdit.SelectionList.Entry.getNarration(0)`: its class isn't translated (it extends `Entry`) (xaero/common/gui/GuiInfoDisplayEdit.java:597)

### `GuiInstructions`: it extends `ScreenBase` (xaero.lib.client.gui)

- [ ] `GuiInstructions.GuiInstructions(3)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiInstructions.java:12)
- [ ] `GuiInstructions.init(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiInstructions.java:16)
- [ ] `GuiInstructions.extractBackground(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiInstructions.java:25)

### `GuiLightOverlay`: it extends `GuiMinimapSettings`

- [ ] `GuiLightOverlay.GuiLightOverlay(4)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiLightOverlay.java:11)

### `GuiMinimapBlockMapSettings`: it extends `GuiMinimapSettings`

- [ ] `GuiMinimapBlockMapSettings.GuiMinimapBlockMapSettings(4)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiMinimapBlockMapSettings.java:19)

### `GuiMinimapInfoSettings`: it extends `GuiMinimapSettings`

- [ ] `GuiMinimapInfoSettings.GuiMinimapInfoSettings(4)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiMinimapInfoSettings.java:12)

### `GuiMinimapMain`: it extends `GuiMinimapSettings`

- [ ] `GuiMinimapMain.GuiMinimapMain(1)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiMinimapMain.java:30)
- [ ] `GuiMinimapMain.GuiMinimapMain(5)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiMinimapMain.java:34)
- [ ] `GuiMinimapMain.resetConfirmResult(1)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiMinimapMain.java:118)
- [ ] `GuiMinimapMain.updateSearchableEntries(0)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiMinimapMain.java:127)
- [ ] `GuiMinimapMain.init(0)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiMinimapMain.java:151)

### `GuiMinimapMiscSettings`: it extends `GuiMinimapSettings`

- [ ] `GuiMinimapMiscSettings.GuiMinimapMiscSettings(3)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiMinimapMiscSettings.java:11)

### `GuiMinimapOverlaysSettings`: it extends `GuiMinimapSettings`

- [ ] `GuiMinimapOverlaysSettings.GuiMinimapOverlaysSettings(4)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiMinimapOverlaysSettings.java:15)

### `GuiMinimapSettings`: it extends `EditConfigScreen` (xaero.lib.client.gui.config)

- [ ] `GuiMinimapSettings.GuiMinimapSettings(4)`: its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)) (xaero/common/gui/GuiMinimapSettings.java:19)
- [ ] `GuiMinimapSettings.onEditedProfileSwitch(0)`: its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)) (xaero/common/gui/GuiMinimapSettings.java:29)
- [ ] `GuiMinimapSettings.extractRenderState(4)`: its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)) (xaero/common/gui/GuiMinimapSettings.java:44)
- [ ] `GuiMinimapSettings.handleChanges(0)`: its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)) (xaero/common/gui/GuiMinimapSettings.java:53)
- [ ] `GuiMinimapSettings.saveEditedCategoryIfNeeded(0)`: its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)) (xaero/common/gui/GuiMinimapSettings.java:58)
- [ ] `GuiMinimapSettings.keyPressed(1)`: its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)) (xaero/common/gui/GuiMinimapSettings.java:66)

### `GuiMinimapViewSettings`: it extends `GuiMinimapSettings`

- [ ] `GuiMinimapViewSettings.GuiMinimapViewSettings(4)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiMinimapViewSettings.java:11)

### `GuiMisc`

- [x] `GuiMisc.wpDistanceFormatPrecision` (xaero/common/gui/GuiMisc.java:7)
- [ ] `GuiMisc.wpDistanceFormat`: it uses `DecimalFormat` (java.text) (xaero/common/gui/GuiMisc.java:8)
- [ ] `GuiMisc.getFormat(1)`: it uses `DecimalFormat` (java.text) (xaero/common/gui/GuiMisc.java:10)

### `GuiNewSet`: it extends `ScreenBase` (xaero.lib.client.gui)

- [ ] `GuiNewSet.GuiNewSet(5)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiNewSet.java:27)
- [ ] `GuiNewSet.init(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiNewSet.java:35)
- [ ] `GuiNewSet.setInitialFocus(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiNewSet.java:66)
- [ ] `GuiNewSet.canConfirm(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiNewSet.java:69)
- [ ] `GuiNewSet.updateConfirmButton(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiNewSet.java:73)
- [ ] `GuiNewSet.keyPressed(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiNewSet.java:77)
- [ ] `GuiNewSet.tick(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiNewSet.java:87)
- [ ] `GuiNewSet.extractBackground(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiNewSet.java:91)

### `GuiReset`: it extends Minecraft's `ConfirmScreen`

- [ ] `GuiReset.GuiReset(3)`: its class isn't translated (it extends Minecraft's `ConfirmScreen`) (xaero/common/gui/GuiReset.java:8)

### `GuiSlimeSeed`: it extends `GuiMinimapSettings`

- [ ] `GuiSlimeSeed.GuiSlimeSeed(5)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiSlimeSeed.java:26)
- [ ] `GuiSlimeSeed.init(0)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiSlimeSeed.java:36)
- [ ] `GuiSlimeSeed.extractRenderState(4)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiSlimeSeed.java:43)
- [ ] `GuiSlimeSeed.tick(0)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiSlimeSeed.java:50)
- [ ] `GuiSlimeSeed.keyPressed(1)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiSlimeSeed.java:53)
- [ ] `GuiSlimeSeed.charTyped(1)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiSlimeSeed.java:64)
- [ ] `GuiSlimeSeed.updateSlimeSeed(0)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiSlimeSeed.java:70)

### `GuiTransfer`: it extends `ScreenBase` (xaero.lib.client.gui)

- [ ] `GuiTransfer.GuiTransfer(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiTransfer.java:38)
- [ ] `GuiTransfer.init(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiTransfer.java:56)
- [ ] `GuiTransfer.createWorlds1DD(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiTransfer.java:73)
- [ ] `GuiTransfer.createWorlds2DD(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiTransfer.java:86)
- [ ] `GuiTransfer.createContainers1DD(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiTransfer.java:99)
- [ ] `GuiTransfer.createContainers2DD(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiTransfer.java:112)
- [ ] `GuiTransfer.openParent(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiTransfer.java:125)
- [ ] `GuiTransfer.transfer(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiTransfer.java:133)
- [ ] `GuiTransfer.renderPreDropdown(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiTransfer.java:171)
- [ ] `GuiTransfer.onSelected(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiTransfer.java:178)

### `GuiWaypointContainers`: it extends `GuiDropdownHelper`

- [ ] `GuiWaypointContainers.GuiWaypointContainers(4)`: its class isn't translated (it extends `GuiDropdownHelper`) (xaero/common/gui/GuiWaypointContainers.java:15)

### `GuiWaypoints`: it extends `ScreenBase` (xaero.lib.client.gui)

- [x] `GuiWaypoints.FRAME_TOP_SIZE` (xaero/common/gui/GuiWaypoints.java:53)
- [x] `GuiWaypoints.FRAME_BOTTOM_SIZE` (xaero/common/gui/GuiWaypoints.java:54)
- [x] `GuiWaypoints.distanceDivided` (xaero/common/gui/GuiWaypoints.java:55)
- [x] `GuiWaypoints.showingThirdPartyDeleted` (xaero/common/gui/GuiWaypoints.java:56)
- [ ] `GuiWaypoints.GuiWaypoints(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:91)
- [ ] `GuiWaypoints.init(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:114)
- [ ] `GuiWaypoints.setDisabled(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:291)
- [ ] `GuiWaypoints.createSetsDropdown(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:299)
- [ ] `GuiWaypoints.createContainersDropdown(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:312)
- [ ] `GuiWaypoints.createWorldsDropdown(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:325)
- [ ] `GuiWaypoints.getSelectedWaypointsList(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:338)
- [ ] `GuiWaypoints.allWaypointsAre(2)`: it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/gui/GuiWaypoints.java:342)
- [ ] `GuiWaypoints.shouldDeleteSet(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:355)
- [ ] `GuiWaypoints.undrag(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:359)
- [ ] `GuiWaypoints.mouseClicked(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:366)
- [ ] `GuiWaypoints.mouseReleased(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:404)
- [ ] `GuiWaypoints.keyReleased(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:421)
- [ ] `GuiWaypoints.extractBackground(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:448)
- [ ] `GuiWaypoints.extractRenderState(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:453)
- [ ] `GuiWaypoints.renderHoveredWaypointDistance(3)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:479)
- [ ] `GuiWaypoints.renderPreDropdown(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:516)
- [ ] `GuiWaypoints.updateButtons(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:545)
- [ ] `GuiWaypoints.isAddEditEnabled(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:563)
- [ ] `GuiWaypoints.isSomethingSelected(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:567)
- [ ] `GuiWaypoints.isOneSelected(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:571)
- [ ] `GuiWaypoints.clearSelection(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:575)
- [ ] `GuiWaypoints.canTeleport(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:579)
- [ ] `GuiWaypoints.onSelected(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:597)
- [ ] `GuiWaypoints.updateSortedList(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:659)
- [ ] `GuiWaypoints.getSortObject(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:733)
- [ ] `GuiWaypoints.isFiltering(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:747)
- [ ] `GuiWaypoints.getWaypointFilterMatch(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:751)
- [ ] `GuiWaypoints.onFilterTyped(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:778)
- [ ] `GuiWaypoints.keyPressed(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:789)
- [ ] `GuiWaypoints.clearFilter(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:805)
- [ ] `GuiWaypoints.selectWaypoints(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypoints.java:817)

### `GuiWaypoints.List`: it extends Minecraft's `ObjectSelectionList`

- [ ] `GuiWaypoints.List.List(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiWaypoints.java:840)
- [ ] `GuiWaypoints.List.getThirdPartyCount(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiWaypoints.java:845)
- [ ] `GuiWaypoints.List.getThirdPartyWaypoint(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiWaypoints.java:849)
- [ ] `GuiWaypoints.List.getWaypointCount(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiWaypoints.java:853)
- [ ] `GuiWaypoints.List.getWaypoint(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiWaypoints.java:858)
- [ ] `GuiWaypoints.List.entriesCanBeSelected(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiWaypoints.java:867)
- [ ] `GuiWaypoints.List.isSelectedItem(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiWaypoints.java:871)
- [ ] `GuiWaypoints.List.createEntries(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiWaypoints.java:875)
- [ ] `GuiWaypoints.List.extractWidgetRenderState(4)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiWaypoints.java:885)
- [ ] `GuiWaypoints.List.extractItem(5)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiWaypoints.java:895)
- [ ] `GuiWaypoints.List.isFocused(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiWaypoints.java:904)
- [ ] `GuiWaypoints.List.setSelected(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiWaypoints.java:908)
- [ ] `GuiWaypoints.List.getItemHeight(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiWaypoints.java:938)
- [ ] `GuiWaypoints.List.drawWaypointSlot(4)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiWaypoints.java:942)
- [ ] `GuiWaypoints.List.getEntryAt(2)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiWaypoints.java:973)
- [ ] `GuiWaypoints.List.ensureVisible(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiWaypoints.java:978)
- [ ] `GuiWaypoints.List.recreateEntries(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiWaypoints.java:982)
- [ ] `GuiWaypoints.List.mouseClicked(2)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/common/gui/GuiWaypoints.java:986)

### `GuiWaypoints.List.WaypointEntry`: it extends Minecraft's `Entry`

- [ ] `GuiWaypoints.List.WaypointEntry.WaypointEntry(1)`: its class isn't translated (it extends Minecraft's `Entry`) (xaero/common/gui/GuiWaypoints.java:997)
- [ ] `GuiWaypoints.List.WaypointEntry.extractContent(5)`: its class isn't translated (it extends Minecraft's `Entry`) (xaero/common/gui/GuiWaypoints.java:1001)
- [ ] `GuiWaypoints.List.WaypointEntry.mouseClicked(2)`: its class isn't translated (it extends Minecraft's `Entry`) (xaero/common/gui/GuiWaypoints.java:1008)
- [ ] `GuiWaypoints.List.WaypointEntry.getNarration(0)`: its class isn't translated (it extends Minecraft's `Entry`) (xaero/common/gui/GuiWaypoints.java:1012)

### `GuiWaypointSets`

- [ ] `GuiWaypointSets.GuiWaypointSets(2)`: it stores a second reference to an object (Rust would need it copied or shared) (xaero/common/gui/GuiWaypointSets.java:14)
- [ ] `GuiWaypointSets.GuiWaypointSets(3)`: it uses `KeySortableByOther` (xaero.lib.common.util) (xaero/common/gui/GuiWaypointSets.java:18)
- [x] `GuiWaypointSets.getCurrentSet(0)` (xaero/common/gui/GuiWaypointSets.java:42)
- [x] `GuiWaypointSets.getCurrentSetKey(0)` (xaero/common/gui/GuiWaypointSets.java:46)
- [x] `GuiWaypointSets.setCurrentSet(1)` (xaero/common/gui/GuiWaypointSets.java:50)
- [x] `GuiWaypointSets.getOptions(0)` (xaero/common/gui/GuiWaypointSets.java:54)

### `GuiWaypointSettings`: it extends `GuiMinimapSettings`

- [ ] `GuiWaypointSettings.GuiWaypointSettings(4)`: its class isn't translated (it extends `GuiMinimapSettings`) (xaero/common/gui/GuiWaypointSettings.java:11)

### `GuiWaypointsOptions`: it extends `ScreenBase` (xaero.lib.client.gui)

- [ ] `GuiWaypointsOptions.GuiWaypointsOptions(6)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypointsOptions.java:62)
- [ ] `GuiWaypointsOptions.init(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypointsOptions.java:75)
- [ ] `GuiWaypointsOptions.getConfigButtonName(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypointsOptions.java:133)
- [ ] `GuiWaypointsOptions.onMultiWorldButton(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypointsOptions.java:156)
- [ ] `GuiWaypointsOptions.onTeleportationButton(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypointsOptions.java:165)
- [ ] `GuiWaypointsOptions.onSortButton(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypointsOptions.java:173)
- [ ] `GuiWaypointsOptions.onSortReverseButton(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypointsOptions.java:186)
- [ ] `GuiWaypointsOptions.onShowThirdPartyDeleted(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypointsOptions.java:199)
- [ ] `GuiWaypointsOptions.getThirdPartyDeletedMessage(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypointsOptions.java:210)
- [ ] `GuiWaypointsOptions.mouseClicked(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypointsOptions.java:217)
- [ ] `GuiWaypointsOptions.actionPerformed(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypointsOptions.java:227)
- [ ] `GuiWaypointsOptions.confirmResult(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypointsOptions.java:337)
- [ ] `GuiWaypointsOptions.multiplyWaypoints(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypointsOptions.java:527)
- [ ] `GuiWaypointsOptions.isDifferentRootContainer(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypointsOptions.java:542)
- [ ] `GuiWaypointsOptions.isDifferentSubWorld(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypointsOptions.java:547)
- [ ] `GuiWaypointsOptions.extractBackground(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypointsOptions.java:551)
- [ ] `GuiWaypointsOptions.extractRenderState(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypointsOptions.java:556)
- [ ] `GuiWaypointsOptions.isInGameUi(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWaypointsOptions.java:563)

### `GuiWaypointWorlds`: it extends `GuiDropdownHelper`

- [ ] `GuiWaypointWorlds.GuiWaypointWorlds(4)`: its class isn't translated (it extends `GuiDropdownHelper`) (xaero/common/gui/GuiWaypointWorlds.java:18)
- [ ] `GuiWaypointWorlds.addWorlds(7)`: its class isn't translated (it extends `GuiDropdownHelper`) (xaero/common/gui/GuiWaypointWorlds.java:67)

### `GuiWorldTpCommand`: it extends `ScreenBase` (xaero.lib.client.gui)

- [ ] `GuiWorldTpCommand.GuiWorldTpCommand(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWorldTpCommand.java:29)
- [ ] `GuiWorldTpCommand.init(0)`: its body couldn't be read: an anonymous class (xaero/common/gui/GuiWorldTpCommand.java:46)
- [ ] `GuiWorldTpCommand.extractBackground(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWorldTpCommand.java:153)
- [ ] `GuiWorldTpCommand.extractRenderState(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWorldTpCommand.java:165)
- [ ] `GuiWorldTpCommand.tick(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWorldTpCommand.java:189)
- [ ] `GuiWorldTpCommand.keyPressed(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWorldTpCommand.java:199)
- [ ] `GuiWorldTpCommand.isInGameUi(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/common/gui/GuiWorldTpCommand.java:212)

### `HudPresetSettingEntry`: it implements `ISettingEntry` (xaero.lib.client.gui)

- [ ] `HudPresetSettingEntry.HudPresetSettingEntry(2)`: its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)) (xaero/common/gui/HudPresetSettingEntry.java:16)
- [ ] `HudPresetSettingEntry.getStringForSearch(0)`: its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)) (xaero/common/gui/HudPresetSettingEntry.java:28)
- [ ] `HudPresetSettingEntry.createWidget(3)`: its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)) (xaero/common/gui/HudPresetSettingEntry.java:32)

### `IXaeroConfirmScreenCallback`: it's an interface

- [ ] `IXaeroConfirmScreenCallback.accept(1)`: it has no body (xaero/common/gui/IXaeroConfirmScreenCallback.java:4)

### `IXaeroNarratableWidget`: it's an interface

- [ ] `IXaeroNarratableWidget.createNarrationMessage(0)`: it has no body (xaero/common/gui/IXaeroNarratableWidget.java:6)

### `MyBigButton`: it extends Minecraft's `Plain`

- [ ] `MyBigButton.MyBigButton(8)`: its class isn't translated (it extends Minecraft's `Plain`) (xaero/common/gui/MyBigButton.java:10)
- [ ] `MyBigButton.MyBigButton(5)`: its class isn't translated (it extends Minecraft's `Plain`) (xaero/common/gui/MyBigButton.java:15)
- [ ] `MyBigButton.getId(0)`: its class isn't translated (it extends Minecraft's `Plain`) (xaero/common/gui/MyBigButton.java:20)

### `MySuperTinyButton`: it extends Minecraft's `Plain`

- [ ] `MySuperTinyButton.MySuperTinyButton(6)`: its class isn't translated (it extends Minecraft's `Plain`) (xaero/common/gui/MySuperTinyButton.java:8)
- [ ] `MySuperTinyButton.MySuperTinyButton(4)`: its class isn't translated (it extends Minecraft's `Plain`) (xaero/common/gui/MySuperTinyButton.java:12)

### `ScreenSwitchSettingEntry`: it implements `ISettingEntry` (xaero.lib.client.gui)

- [ ] `ScreenSwitchSettingEntry.ScreenSwitchSettingEntry(4)`: its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)) (xaero/common/gui/ScreenSwitchSettingEntry.java:22)
- [ ] `ScreenSwitchSettingEntry.ScreenSwitchSettingEntry(4)`: its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)) (xaero/common/gui/ScreenSwitchSettingEntry.java:26)
- [ ] `ScreenSwitchSettingEntry.ScreenSwitchSettingEntry(5)`: its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)) (xaero/common/gui/ScreenSwitchSettingEntry.java:32)
- [ ] `ScreenSwitchSettingEntry.ScreenSwitchSettingEntry(5)`: its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)) (xaero/common/gui/ScreenSwitchSettingEntry.java:38)
- [ ] `ScreenSwitchSettingEntry.getStringForSearch(0)`: its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)) (xaero/common/gui/ScreenSwitchSettingEntry.java:52)
- [ ] `ScreenSwitchSettingEntry.createWidget(3)`: its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)) (xaero/common/gui/ScreenSwitchSettingEntry.java:62)
- [ ] `ScreenSwitchSettingEntry.getScreenFactory(0)`: its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)) (xaero/common/gui/ScreenSwitchSettingEntry.java:78)

### `TooltipButton`: it extends Minecraft's `Plain`

- [ ] `TooltipButton.TooltipButton(7)`: its class isn't translated (it extends Minecraft's `Plain`) (xaero/common/gui/TooltipButton.java:13)
- [ ] `TooltipButton.getXaero_tooltip(0)`: its class isn't translated (it extends Minecraft's `Plain`) (xaero/common/gui/TooltipButton.java:18)

### `WaypointEditForm`: it uses WaypointVisibilityType, which isn't translated (its constants carry values)

- [ ] `WaypointEditForm.getName(0)`: its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)) (xaero/common/gui/WaypointEditForm.java:31)
- [ ] `WaypointEditForm.getxText(0)`: its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)) (xaero/common/gui/WaypointEditForm.java:35)
- [ ] `WaypointEditForm.getyText(0)`: its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)) (xaero/common/gui/WaypointEditForm.java:39)
- [ ] `WaypointEditForm.getzText(0)`: its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)) (xaero/common/gui/WaypointEditForm.java:43)
- [ ] `WaypointEditForm.getYawText(0)`: its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)) (xaero/common/gui/WaypointEditForm.java:47)
- [ ] `WaypointEditForm.getInitial(0)`: its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)) (xaero/common/gui/WaypointEditForm.java:51)
- [ ] `WaypointEditForm.getVisibilityType(0)`: its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)) (xaero/common/gui/WaypointEditForm.java:55)
- [ ] `WaypointEditForm.getDisabledOrTemporary(0)`: its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)) (xaero/common/gui/WaypointEditForm.java:59)
- [ ] `WaypointEditForm.getColor(0)`: its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)) (xaero/common/gui/WaypointEditForm.java:63)
- [ ] `WaypointEditForm.isThirdParty(0)`: its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)) (xaero/common/gui/WaypointEditForm.java:67)

### `HudClientOnlyBase`

- [ ] `HudClientOnlyBase.preInit(2)`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudClientOnlyBase.java:12)
- [ ] `HudClientOnlyBase.preLoadLater(1)`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudClientOnlyBase.java:20)
- [x] `HudClientOnlyBase.loadLaterRender(0)` (xaero/common/HudClientOnlyBase.java:25)

### `HudCommonBase`

- [ ] `HudCommonBase.setup(1)`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudCommonBase.java:9)
- [ ] `HudCommonBase.preLoadLater(1)`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudCommonBase.java:16)

### `HudMod`: it implements `IXaeroMinimap`

- [x] `HudMod.LOGGER` (xaero/common/HudMod.java:56)
- [x] `HudMod.FAIRPLAY` (xaero/common/HudMod.java:57)
- [x] `HudMod.MINIMAP_ID` (xaero/common/HudMod.java:58)
- [ ] `HudMod.INSTANCE`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:59)
- [x] `HudMod.versionID_minecraft` (xaero/common/HudMod.java:64)
- [ ] `HudMod.isLoadedClient(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:104)
- [ ] `HudMod.isLoadedServer(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:109)
- [ ] `HudMod.HudMod(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:114)
- [ ] `HudMod.createPlatformContext(0)`: it has no body (xaero/common/HudMod.java:155)
- [ ] `HudMod.getCommonConfigFileName(0)`: it has no body (xaero/common/HudMod.java:157)
- [ ] `HudMod.createModSettings(0)`: it has no body (xaero/common/HudMod.java:159)
- [ ] `HudMod.createControlsRegister(0)`: it has no body (xaero/common/HudMod.java:161)
- [ ] `HudMod.createGuiHelper(0)`: it has no body (xaero/common/HudMod.java:163)
- [ ] `HudMod.getOldConfigFileName(0)`: it has no body (xaero/common/HudMod.java:165)
- [ ] `HudMod.createClientOnly(0)`: it has no body (xaero/common/HudMod.java:167)
- [ ] `HudMod.getModName(0)`: it has no body (xaero/common/HudMod.java:169)
- [ ] `HudMod.getLogger(0)`: it has no body (xaero/common/HudMod.java:171)
- [ ] `HudMod.createForgeEventHandlerListener(0)`: it has no body (xaero/common/HudMod.java:173)
- [ ] `HudMod.createConfigCommonRegistryHandler(0)`: it has no body (xaero/common/HudMod.java:175)
- [ ] `HudMod.createConfigClientRegistryHandlerSupplier(0)`: it has no body (xaero/common/HudMod.java:177)
- [ ] `HudMod.loadClient(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:179)
- [ ] `HudMod.loadLater(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:277)
- [ ] `HudMod.loadLaterClientRender(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:320)
- [ ] `HudMod.loadServer(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:336)
- [ ] `HudMod.loadCommon(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:344)
- [ ] `HudMod.loadLaterServer(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:352)
- [ ] `HudMod.getConfigFile(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:360)
- [ ] `HudMod.getModJAR(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:365)
- [ ] `HudMod.getSettings(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:370)
- [ ] `HudMod.setSettings(1)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:375)
- [ ] `HudMod.resetSettings(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:380)
- [ ] `HudMod.isOutdated(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:384)
- [ ] `HudMod.setOutdated(1)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:389)
- [ ] `HudMod.getMessage(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:394)
- [ ] `HudMod.setMessage(1)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:399)
- [ ] `HudMod.getLatestVersion(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:404)
- [ ] `HudMod.setLatestVersion(1)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:409)
- [ ] `HudMod.getNewestUpdateID(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:414)
- [ ] `HudMod.setNewestUpdateID(1)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:419)
- [ ] `HudMod.getPatreon(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:424)
- [ ] `HudMod.getWaypointsFile(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:429)
- [ ] `HudMod.getMinimapFolder(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:434)
- [ ] `HudMod.getSupportMods(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:439)
- [ ] `HudMod.getModEvents(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:444)
- [ ] `HudMod.getGuiHelper(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:449)
- [ ] `HudMod.getFieldValidators(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:454)
- [ ] `HudMod.getControlsRegister(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:459)
- [ ] `HudMod.getEvents(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:464)
- [ ] `HudMod.setLatestVersionMD5(1)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:469)
- [ ] `HudMod.getLatestVersionMD5(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:474)
- [ ] `HudMod.isStandalone(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:479)
- [ ] `HudMod.getEntityRadarCategoryManager(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:484)
- [ ] `HudMod.isFairPlay(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:489)
- [ ] `HudMod.getTrackedPlayerRenderer(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:499)
- [ ] `HudMod.getRenderedPlayerTrackerManager(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:504)
- [ ] `HudMod.getServerPlayerTickHandler(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:509)
- [ ] `HudMod.setServerPlayerTickHandler(1)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:514)
- [ ] `HudMod.getMessageHandler(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:519)
- [ ] `HudMod.getCommonEvents(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:524)
- [ ] `HudMod.getSupportServerMods(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:529)
- [ ] `HudMod.setCommonConfigIO(1)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:534)
- [ ] `HudMod.getCommonConfigIO(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:539)
- [ ] `HudMod.getClientEventsListener(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:544)
- [ ] `HudMod.getPlatformContext(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:549)
- [ ] `HudMod.ensureControlsRegister(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:554)
- [ ] `HudMod.getModClientEvents(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:561)
- [ ] `HudMod.getModCommonEvents(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:566)
- [ ] `HudMod.tryLoadLater(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:571)
- [ ] `HudMod.tryLoadLaterServer(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:575)
- [ ] `HudMod.getVersionID(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:579)
- [ ] `HudMod.getHud(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:584)
- [ ] `HudMod.getHudRenderer(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:589)
- [ ] `HudMod.getHudIO(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:594)
- [ ] `HudMod.getMinimap(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:599)
- [ ] `HudMod.isFirstStageLoaded(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:604)
- [ ] `HudMod.getKeyMappingControllers(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:609)
- [ ] `HudMod.getConfigSubFolder(0)`: it has no body (xaero/common/HudMod.java:613)
- [ ] `HudMod.getDefaultConfigsSubFolder(0)`: it has no body (xaero/common/HudMod.java:615)
- [ ] `HudMod.getHudConfigs(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:617)
- [ ] `HudMod.getInfoDisplaysIO(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:621)
- [ ] `HudMod.getEntityRadarCategorySerializers(0)`: its class isn't translated (it implements `IXaeroMinimap`) (xaero/common/HudMod.java:625)

### `XaeroIcon`: it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)

- [ ] `XaeroIcon.XaeroIcon(3)`: it stores a second reference to an object (Rust would need it copied or shared) (xaero/common/icon/XaeroIcon.java:8)
- [ ] `XaeroIcon.getTextureAtlas(0)`: it stores a second reference to an object (Rust would need it copied or shared) (xaero/common/icon/XaeroIcon.java:14)
- [ ] `XaeroIcon.getOffsetX(0)`: its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)) (xaero/common/icon/XaeroIcon.java:18)
- [ ] `XaeroIcon.getOffsetY(0)`: its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)) (xaero/common/icon/XaeroIcon.java:22)

### `XaeroIconAtlas`: it uses Minecraft's `GpuTexture`

- [ ] `XaeroIconAtlas.XaeroIconAtlas(4)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/common/icon/XaeroIconAtlas.java:19)
- [ ] `XaeroIconAtlas.getTextureId(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/common/icon/XaeroIconAtlas.java:28)
- [ ] `XaeroIconAtlas.getTextureView(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/common/icon/XaeroIconAtlas.java:32)
- [ ] `XaeroIconAtlas.close(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/common/icon/XaeroIconAtlas.java:36)
- [ ] `XaeroIconAtlas.getWidth(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/common/icon/XaeroIconAtlas.java:41)
- [ ] `XaeroIconAtlas.getCurrentIndex(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/common/icon/XaeroIconAtlas.java:45)
- [ ] `XaeroIconAtlas.isFull(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/common/icon/XaeroIconAtlas.java:49)
- [ ] `XaeroIconAtlas.createIcon(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/common/icon/XaeroIconAtlas.java:53)

### `XaeroIconAtlas.Builder`: it uses Minecraft's `GpuTexture`

- [ ] `XaeroIconAtlas.Builder.Builder(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/common/icon/XaeroIconAtlas.java:69)
- [ ] `XaeroIconAtlas.Builder.setDefault(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/common/icon/XaeroIconAtlas.java:72)
- [ ] `XaeroIconAtlas.Builder.setPreparedTexture(1)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/common/icon/XaeroIconAtlas.java:77)
- [ ] `XaeroIconAtlas.Builder.setWidth(1)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/common/icon/XaeroIconAtlas.java:82)
- [ ] `XaeroIconAtlas.Builder.setIconWidth(1)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/common/icon/XaeroIconAtlas.java:87)
- [ ] `XaeroIconAtlas.Builder.createGlTexture(1)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/common/icon/XaeroIconAtlas.java:92)
- [ ] `XaeroIconAtlas.Builder.build(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/common/icon/XaeroIconAtlas.java:102)
- [ ] `XaeroIconAtlas.Builder.begin(0)`: it uses Builder, which isn't translated (it uses Minecraft's `GpuTexture`) (xaero/common/icon/XaeroIconAtlas.java:121)

### `XaeroIconAtlasManager`: it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)

- [ ] `XaeroIconAtlasManager.XaeroIconAtlasManager(3)`: its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)) (xaero/common/icon/XaeroIconAtlasManager.java:11)
- [ ] `XaeroIconAtlasManager.clearAtlases(0)`: its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)) (xaero/common/icon/XaeroIconAtlasManager.java:18)
- [ ] `XaeroIconAtlasManager.getCurrentAtlas(0)`: its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)) (xaero/common/icon/XaeroIconAtlasManager.java:27)
- [ ] `XaeroIconAtlasManager.getAtlasTextureSize(0)`: its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)) (xaero/common/icon/XaeroIconAtlasManager.java:36)

### `IXaeroMinimap`: it's an interface

- [ ] `IXaeroMinimap.getVersionID(0)`: it has no body (xaero/common/IXaeroMinimap.java:35)
- [ ] `IXaeroMinimap.getFileLayoutID(0)`: it has no body (xaero/common/IXaeroMinimap.java:37)
- [ ] `IXaeroMinimap.getConfigFile(0)`: it has no body (xaero/common/IXaeroMinimap.java:39)
- [ ] `IXaeroMinimap.getModJAR(0)`: it has no body (xaero/common/IXaeroMinimap.java:41)
- [ ] `IXaeroMinimap.getSettings(0)`: it has no body (xaero/common/IXaeroMinimap.java:43)
- [ ] `IXaeroMinimap.setSettings(1)`: it has no body (xaero/common/IXaeroMinimap.java:45)
- [ ] `IXaeroMinimap.isOutdated(0)`: it has no body (xaero/common/IXaeroMinimap.java:47)
- [ ] `IXaeroMinimap.setOutdated(1)`: it has no body (xaero/common/IXaeroMinimap.java:49)
- [ ] `IXaeroMinimap.getMessage(0)`: it has no body (xaero/common/IXaeroMinimap.java:51)
- [ ] `IXaeroMinimap.setMessage(1)`: it has no body (xaero/common/IXaeroMinimap.java:53)
- [ ] `IXaeroMinimap.getLatestVersion(0)`: it has no body (xaero/common/IXaeroMinimap.java:55)
- [ ] `IXaeroMinimap.getLatestVersionMD5(0)`: it has no body (xaero/common/IXaeroMinimap.java:57)
- [ ] `IXaeroMinimap.setLatestVersion(1)`: it has no body (xaero/common/IXaeroMinimap.java:59)
- [ ] `IXaeroMinimap.setLatestVersionMD5(1)`: it has no body (xaero/common/IXaeroMinimap.java:61)
- [ ] `IXaeroMinimap.getNewestUpdateID(0)`: it has no body (xaero/common/IXaeroMinimap.java:63)
- [ ] `IXaeroMinimap.setNewestUpdateID(1)`: it has no body (xaero/common/IXaeroMinimap.java:65)
- [ ] `IXaeroMinimap.getPatreon(0)`: it has no body (xaero/common/IXaeroMinimap.java:67)
- [ ] `IXaeroMinimap.getVersionsURL(0)`: it has no body (xaero/common/IXaeroMinimap.java:69)
- [ ] `IXaeroMinimap.resetSettings(0)`: it has no body (xaero/common/IXaeroMinimap.java:71)
- [ ] `IXaeroMinimap.getUpdateLink(0)`: it has no body (xaero/common/IXaeroMinimap.java:73)
- [ ] `IXaeroMinimap.getSettingsKey(0)`: it has no body (xaero/common/IXaeroMinimap.java:75)
- [ ] `IXaeroMinimap.getServerSettingsKey(0)`: it has no body (xaero/common/IXaeroMinimap.java:77)
- [ ] `IXaeroMinimap.getWaypointsFile(0)`: it has no body (xaero/common/IXaeroMinimap.java:79)
- [ ] `IXaeroMinimap.getMinimapFolder(0)`: it has no body (xaero/common/IXaeroMinimap.java:81)
- [ ] `IXaeroMinimap.createSession(0)`: it has no body (xaero/common/IXaeroMinimap.java:83)
- [ ] `IXaeroMinimap.getSupportMods(0)`: it has no body (xaero/common/IXaeroMinimap.java:85)
- [ ] `IXaeroMinimap.getSupportServerMods(0)`: it has no body (xaero/common/IXaeroMinimap.java:87)
- [ ] `IXaeroMinimap.getGuiHelper(0)`: it has no body (xaero/common/IXaeroMinimap.java:89)
- [ ] `IXaeroMinimap.getFieldValidators(0)`: it has no body (xaero/common/IXaeroMinimap.java:91)
- [ ] `IXaeroMinimap.getControlsRegister(0)`: it has no body (xaero/common/IXaeroMinimap.java:93)
- [ ] `IXaeroMinimap.getEvents(0)`: it has no body (xaero/common/IXaeroMinimap.java:95)
- [ ] `IXaeroMinimap.tryLoadLater(0)`: it has no body (xaero/common/IXaeroMinimap.java:97)
- [ ] `IXaeroMinimap.tryLoadLaterServer(0)`: it has no body (xaero/common/IXaeroMinimap.java:99)
- [ ] `IXaeroMinimap.getModEvents(0)`: it has no body (xaero/common/IXaeroMinimap.java:101)
- [ ] `IXaeroMinimap.isStandalone(0)`: it has no body (xaero/common/IXaeroMinimap.java:103)
- [ ] `IXaeroMinimap.getEntityRadarCategoryManager(0)`: it has no body (xaero/common/IXaeroMinimap.java:105)
- [ ] `IXaeroMinimap.isFairPlay(0)`: it has no body (xaero/common/IXaeroMinimap.java:107)
- [ ] `IXaeroMinimap.getTrackedPlayerRenderer(0)`: it has no body (xaero/common/IXaeroMinimap.java:109)
- [ ] `IXaeroMinimap.getRenderedPlayerTrackerManager(0)`: it has no body (xaero/common/IXaeroMinimap.java:111)
- [ ] `IXaeroMinimap.getServerPlayerTickHandler(0)`: it has no body (xaero/common/IXaeroMinimap.java:113)
- [ ] `IXaeroMinimap.setServerPlayerTickHandler(1)`: it has no body (xaero/common/IXaeroMinimap.java:115)
- [ ] `IXaeroMinimap.getMessageHandler(0)`: it has no body (xaero/common/IXaeroMinimap.java:117)
- [ ] `IXaeroMinimap.getCommonEvents(0)`: it has no body (xaero/common/IXaeroMinimap.java:119)
- [ ] `IXaeroMinimap.setCommonConfigIO(1)`: it has no body (xaero/common/IXaeroMinimap.java:121)
- [ ] `IXaeroMinimap.getCommonConfigIO(0)`: it has no body (xaero/common/IXaeroMinimap.java:123)
- [ ] `IXaeroMinimap.getClientEventsListener(0)`: it has no body (xaero/common/IXaeroMinimap.java:125)
- [ ] `IXaeroMinimap.getPlatformContext(0)`: it has no body (xaero/common/IXaeroMinimap.java:127)
- [ ] `IXaeroMinimap.ensureControlsRegister(0)`: it has no body (xaero/common/IXaeroMinimap.java:129)
- [ ] `IXaeroMinimap.getModClientEvents(0)`: it has no body (xaero/common/IXaeroMinimap.java:131)
- [ ] `IXaeroMinimap.getModCommonEvents(0)`: it has no body (xaero/common/IXaeroMinimap.java:133)
- [ ] `IXaeroMinimap.getModId(0)`: it has no body (xaero/common/IXaeroMinimap.java:135)
- [ ] `IXaeroMinimap.isLoadedClient(0)`: it has no body (xaero/common/IXaeroMinimap.java:137)
- [ ] `IXaeroMinimap.isLoadedServer(0)`: it has no body (xaero/common/IXaeroMinimap.java:139)
- [ ] `IXaeroMinimap.getHud(0)`: it has no body (xaero/common/IXaeroMinimap.java:141)
- [ ] `IXaeroMinimap.getHudRenderer(0)`: it has no body (xaero/common/IXaeroMinimap.java:143)
- [ ] `IXaeroMinimap.getHudIO(0)`: it has no body (xaero/common/IXaeroMinimap.java:145)
- [ ] `IXaeroMinimap.getMinimap(0)`: it has no body (xaero/common/IXaeroMinimap.java:147)
- [ ] `IXaeroMinimap.isFirstStageLoaded(0)`: it has no body (xaero/common/IXaeroMinimap.java:149)

### `AbstractHighlighter`

- [x] `AbstractHighlighter.AbstractHighlighter(1)` (xaero/common/minimap/highlight/AbstractHighlighter.java:11)
- [ ] `AbstractHighlighter.regionHasHighlights(3)`: it has no body (xaero/common/minimap/highlight/AbstractHighlighter.java:15)
- [ ] `AbstractHighlighter.chunkIsHighlit(3)`: it has no body (xaero/common/minimap/highlight/AbstractHighlighter.java:17)
- [ ] `AbstractHighlighter.getChunkHighlitColor(3)`: it has no body (xaero/common/minimap/highlight/AbstractHighlighter.java:19)
- [ ] `AbstractHighlighter.addBlockHighlightTooltips(5)`: it has no body (xaero/common/minimap/highlight/AbstractHighlighter.java:21)
- [x] `AbstractHighlighter.setResult(3)` (xaero/common/minimap/highlight/AbstractHighlighter.java:23)
- [x] `AbstractHighlighter.getBlend(2)` (xaero/common/minimap/highlight/AbstractHighlighter.java:27)
- [x] `AbstractHighlighter.isCoveringOutsideDiscovered(0)` (xaero/common/minimap/highlight/AbstractHighlighter.java:47)

### `ChunkHighlighter`: it extends `AbstractHighlighter`

- [ ] `ChunkHighlighter.ChunkHighlighter(1)`: its class isn't translated (it extends `AbstractHighlighter`) (xaero/common/minimap/highlight/ChunkHighlighter.java:8)
- [ ] `ChunkHighlighter.getColors(3)`: it has no body (xaero/common/minimap/highlight/ChunkHighlighter.java:12)
- [ ] `ChunkHighlighter.getChunkHighlitColor(3)`: its class isn't translated (it extends `AbstractHighlighter`) (xaero/common/minimap/highlight/ChunkHighlighter.java:14)
- [ ] `ChunkHighlighter.getSideBlend(3)`: its class isn't translated (it extends `AbstractHighlighter`) (xaero/common/minimap/highlight/ChunkHighlighter.java:49)
- [ ] `ChunkHighlighter.addBlockHighlightTooltips(5)`: its class isn't translated (it extends `AbstractHighlighter`) (xaero/common/minimap/highlight/ChunkHighlighter.java:53)
- [ ] `ChunkHighlighter.addChunkHighlightTooltips(5)`: it has no body (xaero/common/minimap/highlight/ChunkHighlighter.java:60)

### `DimensionHighlighterHandler`: it uses Minecraft's `ResourceKey`

- [ ] `DimensionHighlighterHandler.DimensionHighlighterHandler(3)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/common/minimap/highlight/DimensionHighlighterHandler.java:14)
- [ ] `DimensionHighlighterHandler.shouldApplyRegionHighlights(3)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/common/minimap/highlight/DimensionHighlighterHandler.java:20)
- [ ] `DimensionHighlighterHandler.shouldApplyTileChunkHighlights(5)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/common/minimap/highlight/DimensionHighlighterHandler.java:32)
- [ ] `DimensionHighlighterHandler.shouldApplyTileChunkHighlights(6)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/common/minimap/highlight/DimensionHighlighterHandler.java:45)
- [ ] `DimensionHighlighterHandler.shouldApplyTileChunkHighlightsHelp(6)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/common/minimap/highlight/DimensionHighlighterHandler.java:53)
- [ ] `DimensionHighlighterHandler.applyChunkHighlightColors(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/common/minimap/highlight/DimensionHighlighterHandler.java:74)
- [ ] `DimensionHighlighterHandler.requestRefresh(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/common/minimap/highlight/DimensionHighlighterHandler.java:122)
- [ ] `DimensionHighlighterHandler.requestRefresh(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/common/minimap/highlight/DimensionHighlighterHandler.java:136)
- [ ] `DimensionHighlighterHandler.addBlockHighlightTooltips(5)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/common/minimap/highlight/DimensionHighlighterHandler.java:140)
- [ ] `DimensionHighlighterHandler.getVersion(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/common/minimap/highlight/DimensionHighlighterHandler.java:158)

### `HighlighterRegistry`

- [ ] `HighlighterRegistry.register(1)`: it stores a second reference to an object (Rust would need it copied or shared) (xaero/common/minimap/highlight/HighlighterRegistry.java:10)
- [x] `HighlighterRegistry.end(0)` (xaero/common/minimap/highlight/HighlighterRegistry.java:14)
- [x] `HighlighterRegistry.getHighlighters(0)` (xaero/common/minimap/highlight/HighlighterRegistry.java:18)

### `TestHighlighter`: it extends `ChunkHighlighter`

- [ ] `TestHighlighter.TestHighlighter(0)`: its class isn't translated (it extends `ChunkHighlighter`) (xaero/common/minimap/highlight/TestHighlighter.java:9)
- [ ] `TestHighlighter.regionHasHighlights(3)`: its class isn't translated (it extends `ChunkHighlighter`) (xaero/common/minimap/highlight/TestHighlighter.java:13)
- [ ] `TestHighlighter.getColors(3)`: its class isn't translated (it extends `ChunkHighlighter`) (xaero/common/minimap/highlight/TestHighlighter.java:18)
- [ ] `TestHighlighter.chunkIsHighlit(3)`: its class isn't translated (it extends `ChunkHighlighter`) (xaero/common/minimap/highlight/TestHighlighter.java:34)
- [ ] `TestHighlighter.addChunkHighlightTooltips(5)`: its class isn't translated (it extends `ChunkHighlighter`) (xaero/common/minimap/highlight/TestHighlighter.java:39)

### `IXaeroMinimapClientWorld`: it's an interface

- [ ] `IXaeroMinimapClientWorld.getXaero_minimapData(0)`: it has no body (xaero/common/minimap/mcworld/IXaeroMinimapClientWorld.java:4)
- [ ] `IXaeroMinimapClientWorld.setXaero_minimapData(1)`: it has no body (xaero/common/minimap/mcworld/IXaeroMinimapClientWorld.java:6)

### `MinimapClientWorldData`: it uses Minecraft's `EnvironmentAttributeSystem`

- [ ] `MinimapClientWorldData.MinimapClientWorldData(1)`: its class isn't translated (it uses Minecraft's `EnvironmentAttributeSystem`) (xaero/common/minimap/mcworld/MinimapClientWorldData.java:20)
- [ ] `MinimapClientWorldData.setServerModNetworkVersion(1)`: its class isn't translated (it uses Minecraft's `EnvironmentAttributeSystem`) (xaero/common/minimap/mcworld/MinimapClientWorldData.java:39)
- [ ] `MinimapClientWorldData.getServerModNetworkVersion(0)`: its class isn't translated (it uses Minecraft's `EnvironmentAttributeSystem`) (xaero/common/minimap/mcworld/MinimapClientWorldData.java:43)
- [ ] `MinimapClientWorldData.setSyncedRules(1)`: its class isn't translated (it uses Minecraft's `EnvironmentAttributeSystem`) (xaero/common/minimap/mcworld/MinimapClientWorldData.java:47)
- [ ] `MinimapClientWorldData.getSyncedRules(0)`: its class isn't translated (it uses Minecraft's `EnvironmentAttributeSystem`) (xaero/common/minimap/mcworld/MinimapClientWorldData.java:51)
- [ ] `MinimapClientWorldData.getAttributeSystem(0)`: its class isn't translated (it uses Minecraft's `EnvironmentAttributeSystem`) (xaero/common/minimap/mcworld/MinimapClientWorldData.java:59)

### `MinimapClientWorldDataHelper`

- [ ] `MinimapClientWorldDataHelper.getCurrentWorldData(0)`: it uses MinimapClientWorldData, which isn't translated (it uses Minecraft's `EnvironmentAttributeSystem`) (xaero/common/minimap/mcworld/MinimapClientWorldDataHelper.java:7)
- [ ] `MinimapClientWorldDataHelper.getWorldData(1)`: it uses MinimapClientWorldData, which isn't translated (it uses Minecraft's `EnvironmentAttributeSystem`) (xaero/common/minimap/mcworld/MinimapClientWorldDataHelper.java:11)

### `MinimapProcessor`: it uses IXaeroMinimap, which isn't translated (it's an interface)

- [x] `MinimapProcessor.DEBUG` (xaero/common/minimap/MinimapProcessor.java:43)
- [x] `MinimapProcessor.FRAME` (xaero/common/minimap/MinimapProcessor.java:44)
- [ ] `MinimapProcessor.MinimapProcessor(5)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:66)
- [ ] `MinimapProcessor.getMinimapSize(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:86)
- [ ] `MinimapProcessor.getMinimapBufferSize(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:91)
- [ ] `MinimapProcessor.isEnlargedMap(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:100)
- [ ] `MinimapProcessor.setEnlargedMap(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:104)
- [ ] `MinimapProcessor.getMinimapZoom(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:108)
- [ ] `MinimapProcessor.isCaveModeDisplayed(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:112)
- [ ] `MinimapProcessor.getTargetZoom(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:116)
- [ ] `MinimapProcessor.instantZoom(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:137)
- [ ] `MinimapProcessor.updateZoom(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:141)
- [ ] `MinimapProcessor.getMinimapWriter(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:153)
- [ ] `MinimapProcessor.canUseFrameBuffer(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:157)
- [ ] `MinimapProcessor.getFBOBufferSize(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:161)
- [ ] `MinimapProcessor.onClientTick(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:165)
- [ ] `MinimapProcessor.onPlayerTick(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:177)
- [ ] `MinimapProcessor.checkFBO(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:180)
- [ ] `MinimapProcessor.onRender(12)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:197)
- [ ] `MinimapProcessor.getRenderer(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:238)
- [ ] `MinimapProcessor.hasMinimapItem(1)`: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/common/minimap/MinimapProcessor.java:242)
- [ ] `MinimapProcessor.isToResetImage(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:252)
- [ ] `MinimapProcessor.setToResetImage(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:256)
- [ ] `MinimapProcessor.getRadarSession(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:260)
- [ ] `MinimapProcessor.cleanup(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:264)
- [ ] `MinimapProcessor.isManualCaveMode(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:268)
- [ ] `MinimapProcessor.toggleManualCaveMode(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:274)
- [ ] `MinimapProcessor.getMinimap(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:278)
- [ ] `MinimapProcessor.getNoMinimapMessageReceived(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:282)
- [ ] `MinimapProcessor.setNoMinimapMessageReceived(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:286)
- [ ] `MinimapProcessor.getForcedFairPlay(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:290)
- [ ] `MinimapProcessor.setFairPlayOnlyMessageReceived(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:294)
- [ ] `MinimapProcessor.getSyncedTrackedPlayerManager(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:298)
- [ ] `MinimapProcessor.serverHasMod(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:302)
- [ ] `MinimapProcessor.setServerModNetworkVersion(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:307)
- [ ] `MinimapProcessor.getServerModNetworkVersion(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:314)
- [ ] `MinimapProcessor.getLastMapDimensionScale(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:319)
- [ ] `MinimapProcessor.setLastMapDimensionScale(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:323)
- [ ] `MinimapProcessor.getLastMapDimension(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:327)
- [ ] `MinimapProcessor.setLastMapDimension(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:331)
- [ ] `MinimapProcessor.getSession(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:335)
- [ ] `MinimapProcessor.updateMinimapItem(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:339)
- [ ] `MinimapProcessor.getMinimapItem(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:365)
- [ ] `MinimapProcessor.setConsideringNetherFairPlayMessage(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:369)
- [ ] `MinimapProcessor.isConsideringNetherFairPlayMessage(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:373)
- [ ] `MinimapProcessor.getRenderState(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:377)
- [ ] `MinimapProcessor.getEnlargedRenderState(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/MinimapProcessor.java:381)

### `MinimapChunk`: it uses `GpuTextureAndView` (xaero.lib.client.graphics)

- [x] `MinimapChunk.SIZE_TILES` (xaero/common/minimap/region/MinimapChunk.java:16)
- [x] `MinimapChunk.INT_BUFFER_SIZE` (xaero/common/minimap/region/MinimapChunk.java:17)
- [x] `MinimapChunk.LIGHT_LEVELS` (xaero/common/minimap/region/MinimapChunk.java:18)
- [ ] `MinimapChunk.MinimapChunk(2)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:32)
- [ ] `MinimapChunk.reset(2)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:42)
- [ ] `MinimapChunk.recycleTiles(0)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:68)
- [ ] `MinimapChunk.getLevelToRefresh(1)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:83)
- [ ] `MinimapChunk.bindTexture(1)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:104)
- [ ] `MinimapChunk.updateBuffers(1)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:132)
- [ ] `MinimapChunk.putColour(8)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:173)
- [ ] `MinimapChunk.copyBuffer(2)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:178)
- [ ] `MinimapChunk.getLevelsBuffered(0)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:190)
- [ ] `MinimapChunk.isHasSomething(0)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:194)
- [ ] `MinimapChunk.setHasSomething(1)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:198)
- [ ] `MinimapChunk.getX(0)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:202)
- [ ] `MinimapChunk.getZ(0)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:206)
- [ ] `MinimapChunk.getGlTexture(1)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:210)
- [ ] `MinimapChunk.setGlTexture(2)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:214)
- [ ] `MinimapChunk.getTile(2)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:218)
- [ ] `MinimapChunk.setTile(3)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:222)
- [ ] `MinimapChunk.isChanged(0)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:226)
- [ ] `MinimapChunk.setChanged(1)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:230)
- [ ] `MinimapChunk.setLevelsBuffered(1)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:234)
- [ ] `MinimapChunk.isBlockTextureUpload(0)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:238)
- [ ] `MinimapChunk.setBlockTextureUpload(1)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:242)
- [ ] `MinimapChunk.isRefreshRequired(1)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:246)
- [ ] `MinimapChunk.setRefreshRequired(2)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:250)
- [ ] `MinimapChunk.getBuffer(1)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:254)
- [ ] `MinimapChunk.cleanup(1)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/common/minimap/region/MinimapChunk.java:258)

### `MinimapTile`

- [x] `MinimapTile.recycled` (xaero/common/minimap/region/MinimapTile.java:10)
- [ ] `MinimapTile.getANewTile(4)`: List.remove on a list kept in a changing static (xaero/common/minimap/region/MinimapTile.java:32)
- [ ] `MinimapTile.MinimapTile(4)`: it uses ModSettings, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/region/MinimapTile.java:42)
- [ ] `MinimapTile.create(4)`: it uses ModSettings, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/region/MinimapTile.java:46)
- [ ] `MinimapTile.recycle(0)`: `this` used as a value (xaero/common/minimap/region/MinimapTile.java:56)
- [ ] `MinimapTile.isSlimeChunk(4)`: it uses ModSettings, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/region/MinimapTile.java:62)
- [x] `MinimapTile.isWasTransfered(0)` (xaero/common/minimap/region/MinimapTile.java:75)
- [x] `MinimapTile.setWasTransfered(1)` (xaero/common/minimap/region/MinimapTile.java:79)
- [x] `MinimapTile.isChunkGrid(0)` (xaero/common/minimap/region/MinimapTile.java:83)
- [x] `MinimapTile.isSlimeChunk(0)` (xaero/common/minimap/region/MinimapTile.java:87)
- [x] `MinimapTile.getRed(3)` (xaero/common/minimap/region/MinimapTile.java:91)
- [x] `MinimapTile.getGreen(3)` (xaero/common/minimap/region/MinimapTile.java:95)
- [x] `MinimapTile.getBlue(3)` (xaero/common/minimap/region/MinimapTile.java:99)
- [x] `MinimapTile.setRGB(6)` (xaero/common/minimap/region/MinimapTile.java:103)
- [x] `MinimapTile.isSuccess(0)` (xaero/common/minimap/region/MinimapTile.java:109)
- [x] `MinimapTile.setSuccess(1)` (xaero/common/minimap/region/MinimapTile.java:113)
- [x] `MinimapTile.getX(0)` (xaero/common/minimap/region/MinimapTile.java:117)
- [x] `MinimapTile.getZ(0)` (xaero/common/minimap/region/MinimapTile.java:121)
- [x] `MinimapTile.getHeight(2)` (xaero/common/minimap/region/MinimapTile.java:125)
- [x] `MinimapTile.setHeight(3)` (xaero/common/minimap/region/MinimapTile.java:129)
- [x] `MinimapTile.pixelChanged(7)` (xaero/common/minimap/region/MinimapTile.java:133)
- [x] `MinimapTile.setCode(7)` (xaero/common/minimap/region/MinimapTile.java:141)
- [x] `MinimapTile.setHasSomething(1)` (xaero/common/minimap/region/MinimapTile.java:149)
- [x] `MinimapTile.isHasSomething(0)` (xaero/common/minimap/region/MinimapTile.java:153)
- [x] `MinimapTile.setHasTerrain(1)` (xaero/common/minimap/region/MinimapTile.java:157)
- [x] `MinimapTile.hasTerrain(0)` (xaero/common/minimap/region/MinimapTile.java:161)
- [x] `MinimapTile.getHighlightVersion(0)` (xaero/common/minimap/region/MinimapTile.java:165)
- [x] `MinimapTile.setHighlightVersion(1)` (xaero/common/minimap/region/MinimapTile.java:169)
- [x] `MinimapTile.getHighlights(0)` (xaero/common/minimap/region/MinimapTile.java:173)
- [x] `MinimapTile.setHighlights(1)` (xaero/common/minimap/region/MinimapTile.java:177)

### `MinimapFBORenderer`: it extends `MinimapRenderer`

- [ ] `MinimapFBORenderer.MinimapFBORenderer(6)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapFBORenderer.java:72)
- [ ] `MinimapFBORenderer.loadFrameBuffer(1)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapFBORenderer.java:78)
- [ ] `MinimapFBORenderer.restoreDefaultTarget(0)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapFBORenderer.java:106)
- [ ] `MinimapFBORenderer.renderChunks(19)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapFBORenderer.java:110)
- [ ] `MinimapFBORenderer.renderChunksToFBO(16)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapFBORenderer.java:161)
- [ ] `MinimapFBORenderer.deleteFramebuffers(0)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapFBORenderer.java:396)
- [ ] `MinimapFBORenderer.isLoadedFBO(0)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapFBORenderer.java:404)
- [ ] `MinimapFBORenderer.setLoadedFBO(1)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapFBORenderer.java:408)
- [ ] `MinimapFBORenderer.isTriedFBO(0)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapFBORenderer.java:412)
- [ ] `MinimapFBORenderer.assumeUsingFBO(0)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapFBORenderer.java:416)
- [ ] `MinimapFBORenderer.resetEntityIcons(0)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapFBORenderer.java:421)
- [ ] `MinimapFBORenderer.resetEntityIconsResources(0)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapFBORenderer.java:427)
- [ ] `MinimapFBORenderer.onRadarIconModelPrepareDetection(1)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapFBORenderer.java:433)
- [ ] `MinimapFBORenderer.onRadarIconModelRenderTrace(3)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapFBORenderer.java:437)
- [ ] `MinimapFBORenderer.onEntityIconModelPartRenderTrace(2)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapFBORenderer.java:441)
- [ ] `MinimapFBORenderer.renderMainEntityDot(3)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapFBORenderer.java:445)
- [ ] `MinimapFBORenderer.getTas(0)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapFBORenderer.java:454)
- [ ] `MinimapFBORenderer.getEntityRadarRenderer(0)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapFBORenderer.java:459)

### `MinimapRenderer`: another class has the same name, in a different package

- [x] `MinimapRenderer.black` (xaero/common/minimap/render/MinimapRenderer.java:56)
- [x] `MinimapRenderer.slime` (xaero/common/minimap/render/MinimapRenderer.java:57)
- [ ] `MinimapRenderer.MinimapRenderer(6)`: its class isn't translated (another class has the same name, in a different package) (xaero/common/minimap/render/MinimapRenderer.java:69)
- [ ] `MinimapRenderer.getRenderAngle(1)`: its class isn't translated (another class has the same name, in a different package) (xaero/common/minimap/render/MinimapRenderer.java:82)
- [ ] `MinimapRenderer.getActualAngle(0)`: its class isn't translated (another class has the same name, in a different package) (xaero/common/minimap/render/MinimapRenderer.java:86)
- [ ] `MinimapRenderer.renderChunks(19)`: it has no body (xaero/common/minimap/render/MinimapRenderer.java:91)
- [ ] `MinimapRenderer.renderMinimap(11)`: its class isn't translated (another class has the same name, in a different package) (xaero/common/minimap/render/MinimapRenderer.java:113)
- [ ] `MinimapRenderer.renderOutsidePip(10)`: its class isn't translated (another class has the same name, in a different package) (xaero/common/minimap/render/MinimapRenderer.java:527)
- [ ] `MinimapRenderer.renderCompass(11)`: its class isn't translated (another class has the same name, in a different package) (xaero/common/minimap/render/MinimapRenderer.java:553)
- [ ] `MinimapRenderer.drawArrow(10)`: its class isn't translated (another class has the same name, in a different package) (xaero/common/minimap/render/MinimapRenderer.java:600)
- [ ] `MinimapRenderer.getZoom(0)`: its class isn't translated (another class has the same name, in a different package) (xaero/common/minimap/render/MinimapRenderer.java:618)
- [ ] `MinimapRenderer.setZoom(1)`: its class isn't translated (another class has the same name, in a different package) (xaero/common/minimap/render/MinimapRenderer.java:622)
- [ ] `MinimapRenderer.getSunBrightness(2)`: its class isn't translated (another class has the same name, in a different package) (xaero/common/minimap/render/MinimapRenderer.java:626)
- [ ] `MinimapRenderer.getHelper(0)`: its class isn't translated (another class has the same name, in a different package) (xaero/common/minimap/render/MinimapRenderer.java:641)
- [ ] `MinimapRenderer.getTas(0)`: it has no body (xaero/common/minimap/render/MinimapRenderer.java:645)

### `MinimapRendererHelper`: it uses Minecraft's `ProjectionMatrixBuffer`

- [ ] `MinimapRendererHelper.drawMyTexturedModalRect(11)`: its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`) (xaero/common/minimap/render/MinimapRendererHelper.java:26)
- [ ] `MinimapRendererHelper.drawMyTexturedModalRect(12)`: its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`) (xaero/common/minimap/render/MinimapRendererHelper.java:42)
- [ ] `MinimapRendererHelper.drawIconOutline(11)`: its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`) (xaero/common/minimap/render/MinimapRendererHelper.java:59)
- [ ] `MinimapRendererHelper.drawTexturedElipseInsideRectangle(11)`: its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`) (xaero/common/minimap/render/MinimapRendererHelper.java:75)
- [ ] `MinimapRendererHelper.drawTexturedElipseInsideRectangle(12)`: its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`) (xaero/common/minimap/render/MinimapRendererHelper.java:91)
- [ ] `MinimapRendererHelper.drawTexturedElipseInsideRectangleFrame(18)`: its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`) (xaero/common/minimap/render/MinimapRendererHelper.java:143)
- [ ] `MinimapRendererHelper.addTexturedRectToExistingBuffer(8)`: its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`) (xaero/common/minimap/render/MinimapRendererHelper.java:217)
- [ ] `MinimapRendererHelper.drawMyColoredRect(6)`: its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`) (xaero/common/minimap/render/MinimapRendererHelper.java:229)
- [ ] `MinimapRendererHelper.addColoredLineToExistingBuffer(11)`: its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`) (xaero/common/minimap/render/MinimapRendererHelper.java:241)
- [ ] `MinimapRendererHelper.drawMyColoredRect(6)`: its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`) (xaero/common/minimap/render/MinimapRendererHelper.java:248)
- [ ] `MinimapRendererHelper.bindTextureBuffer(5)`: its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`) (xaero/common/minimap/render/MinimapRendererHelper.java:252)
- [ ] `MinimapRendererHelper.putColor(7)`: its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`) (xaero/common/minimap/render/MinimapRendererHelper.java:256)
- [ ] `MinimapRendererHelper.gridOverlay(5)`: its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`) (xaero/common/minimap/render/MinimapRendererHelper.java:260)
- [ ] `MinimapRendererHelper.slimeOverlay(4)`: its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`) (xaero/common/minimap/render/MinimapRendererHelper.java:266)
- [ ] `MinimapRendererHelper.defaultOrtho(1)`: its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`) (xaero/common/minimap/render/MinimapRendererHelper.java:272)
- [x] `MinimapRendererHelper.restoreDefaultShaderBlendState(0)` (xaero/common/minimap/render/MinimapRendererHelper.java:285)

### `MinimapSafeModeRenderer`: it extends `MinimapRenderer`

- [x] `MinimapSafeModeRenderer.mapTextures` (xaero/common/minimap/render/MinimapSafeModeRenderer.java:43)
- [ ] `MinimapSafeModeRenderer.MinimapSafeModeRenderer(6)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapSafeModeRenderer.java:49)
- [ ] `MinimapSafeModeRenderer.updateMapFrameSafeMode(14)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapSafeModeRenderer.java:55)
- [ ] `MinimapSafeModeRenderer.renderEntityListSafeMode(18)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapSafeModeRenderer.java:199)
- [ ] `MinimapSafeModeRenderer.renderEntityDotSafeMode(18)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapSafeModeRenderer.java:246)
- [ ] `MinimapSafeModeRenderer.renderChunks(19)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapSafeModeRenderer.java:318)
- [ ] `MinimapSafeModeRenderer.getLoadedBlockColor(8)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapSafeModeRenderer.java:357)
- [ ] `MinimapSafeModeRenderer.chunkOverlay(8)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapSafeModeRenderer.java:398)
- [ ] `MinimapSafeModeRenderer.getTas(0)`: its class isn't translated (it extends `MinimapRenderer`) (xaero/common/minimap/render/MinimapSafeModeRenderer.java:410)

### `EntityIconDefinitions`

- [ ] `EntityIconDefinitions.getVariant(3)`: it's generic (xaero/common/minimap/render/radar/EntityIconDefinitions.java:10)
- [ ] `EntityIconDefinitions.buildVariantIdString(3)`: it uses Minecraft's `EntityRenderer` (xaero/common/minimap/render/radar/EntityIconDefinitions.java:14)
- [ ] `EntityIconDefinitions.getVariantString(2)`: it uses Minecraft's `EntityRenderer` (xaero/common/minimap/render/radar/EntityIconDefinitions.java:17)

### `Waypoint`: it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)

- [x] `Waypoint.ONEOFF_DESTINATION_SAFE_FOR` (xaero/common/minimap/waypoints/Waypoint.java:14)
- [x] `Waypoint.ONEOFF_DESTINATION_REMOVE_DISTANCE` (xaero/common/minimap/waypoints/Waypoint.java:15)
- [ ] `Waypoint.RENDER_SORTING_POS`: it uses Minecraft's `Vec3` (xaero/common/minimap/waypoints/Waypoint.java:16)
- [ ] `Waypoint.Waypoint(6)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:30)
- [ ] `Waypoint.Waypoint(7)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:35)
- [ ] `Waypoint.Waypoint(8)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:40)
- [ ] `Waypoint.Waypoint(9)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:45)
- [ ] `Waypoint.Waypoint(6)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:51)
- [ ] `Waypoint.Waypoint(7)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:55)
- [ ] `Waypoint.Waypoint(8)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:59)
- [ ] `Waypoint.Waypoint(9)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:63)
- [ ] `Waypoint.getX(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:83)
- [ ] `Waypoint.setX(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:87)
- [ ] `Waypoint.getX(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:91)
- [ ] `Waypoint.getY(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:95)
- [ ] `Waypoint.setY(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:99)
- [ ] `Waypoint.getZ(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:103)
- [ ] `Waypoint.setZ(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:107)
- [ ] `Waypoint.getZ(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:111)
- [ ] `Waypoint.getName(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:115)
- [ ] `Waypoint.getLocalizedName(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:119)
- [ ] `Waypoint.getInitials(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:123)
- [ ] `Waypoint.getNameSafe(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:130)
- [ ] `Waypoint.setName(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:134)
- [ ] `Waypoint.getSymbol(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:138)
- [ ] `Waypoint.setSymbol(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:143)
- [ ] `Waypoint.getSymbolSafe(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:148)
- [ ] `Waypoint.getColor(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:153)
- [ ] `Waypoint.getActualColor(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:158)
- [ ] `Waypoint.setColor(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:163)
- [ ] `Waypoint.getWaypointColor(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:169)
- [ ] `Waypoint.isGlobal(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:176)
- [ ] `Waypoint.getVisibilityType(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:180)
- [ ] `Waypoint.getVisibility(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:185)
- [ ] `Waypoint.setVisibilityType(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:192)
- [ ] `Waypoint.setVisibility(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:197)
- [ ] `Waypoint.isDisabled(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:204)
- [ ] `Waypoint.getDisabled(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:208)
- [ ] `Waypoint.setDisabled(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:215)
- [ ] `Waypoint.setDisabled(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:219)
- [ ] `Waypoint.getWaypointType(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:225)
- [ ] `Waypoint.setType(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:230)
- [ ] `Waypoint.getPurpose(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:235)
- [ ] `Waypoint.setPurpose(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:239)
- [ ] `Waypoint.isRotation(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:246)
- [ ] `Waypoint.setRotation(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:250)
- [ ] `Waypoint.getYaw(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:254)
- [ ] `Waypoint.setYaw(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:258)
- [ ] `Waypoint.isTemporary(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:262)
- [ ] `Waypoint.setTemporary(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:266)
- [ ] `Waypoint.isYIncluded(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:271)
- [ ] `Waypoint.setYIncluded(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:275)
- [ ] `Waypoint.getCreatedAt(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:279)
- [ ] `Waypoint.isOneoffDestination(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:283)
- [ ] `Waypoint.setOneoffDestination(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:288)
- [ ] `Waypoint.isDestination(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:299)
- [ ] `Waypoint.getDistanceSq(3)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:303)
- [x] `Waypoint.getStringFromStringSafe(2)` (xaero/common/minimap/waypoints/Waypoint.java:310)
- [ ] `Waypoint.isServerWaypoint(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:314)
- [ ] `Waypoint.getComparisonName(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:319)
- [ ] `Waypoint.getComparisonDistance(2)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:328)
- [ ] `Waypoint.getComparisonAngleCos(2)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:336)
- [ ] `Waypoint.getRenderSortingDistanceSquared(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:346)
- [ ] `Waypoint.compareTo(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:353)
- [ ] `Waypoint.setThirdPartyOrigin(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:364)
- [ ] `Waypoint.getThirdPartyOrigin(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:369)
- [ ] `Waypoint.isThirdParty(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:373)
- [ ] `Waypoint.isEffectivelyDeleted(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:377)
- [ ] `Waypoint.getThirdPartyRenderOverride(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:381)
- [ ] `Waypoint.setThirdPartyRenderOverride(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:385)
- [ ] `Waypoint.isThirdPartyDeleted(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:389)
- [ ] `Waypoint.setThirdPartyDeleted(1)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:394)
- [ ] `Waypoint.getRenderInfoEditDest(0)`: its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/common/minimap/waypoints/Waypoint.java:403)

### `WaypointUtil`

- [x] `WaypointUtil.getAddedMinimapIconFrame(1)` (xaero/common/minimap/waypoints/WaypointUtil.java:4)
- [x] `WaypointUtil.getAddedMinimapIconFrame(2)` (xaero/common/minimap/waypoints/WaypointUtil.java:8)

### `BiomeBlendCalculator`: it implements Minecraft's `BlockAndTintGetter`

- [ ] `BiomeBlendCalculator.prepare(2)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/common/minimap/write/biome/BiomeBlendCalculator.java:24)
- [ ] `BiomeBlendCalculator.getBlockEntity(1)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/common/minimap/write/biome/BiomeBlendCalculator.java:35)
- [ ] `BiomeBlendCalculator.getBlockState(1)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/common/minimap/write/biome/BiomeBlendCalculator.java:39)
- [ ] `BiomeBlendCalculator.getFluidState(1)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/common/minimap/write/biome/BiomeBlendCalculator.java:43)
- [ ] `BiomeBlendCalculator.getLightEngine(0)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/common/minimap/write/biome/BiomeBlendCalculator.java:47)
- [ ] `BiomeBlendCalculator.getHeight(0)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/common/minimap/write/biome/BiomeBlendCalculator.java:51)
- [ ] `BiomeBlendCalculator.getMinY(0)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/common/minimap/write/biome/BiomeBlendCalculator.java:55)
- [ ] `BiomeBlendCalculator.cardinalLighting(0)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/common/minimap/write/biome/BiomeBlendCalculator.java:59)
- [ ] `BiomeBlendCalculator.getBlockTint(2)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/common/minimap/write/biome/BiomeBlendCalculator.java:63)

### `MinimapWriter`: it uses IXaeroMinimap, which isn't translated (it's an interface)

- [x] `MinimapWriter.VOID_COLOR` (xaero/common/minimap/write/MinimapWriter.java:84)
- [x] `MinimapWriter.DEFAULT_AMBIENT_LIGHT` (xaero/common/minimap/write/MinimapWriter.java:85)
- [x] `MinimapWriter.DEFAULT_AMBIENT_LIGHT_COLORED` (xaero/common/minimap/write/MinimapWriter.java:86)
- [x] `MinimapWriter.DEFAULT_AMBIENT_LIGHT_WHITE` (xaero/common/minimap/write/MinimapWriter.java:87)
- [x] `MinimapWriter.DEFAULT_MAX_DIRECT_LIGHT` (xaero/common/minimap/write/MinimapWriter.java:88)
- [x] `MinimapWriter.GLOWING_MAX_DIRECT_LIGHT` (xaero/common/minimap/write/MinimapWriter.java:89)
- [x] `MinimapWriter.dimensionsToIgnore` (xaero/common/minimap/write/MinimapWriter.java:90)
- [x] `MinimapWriter.UPDATE_EVERY_RUNS` (xaero/common/minimap/write/MinimapWriter.java:91)
- [x] `MinimapWriter.MAXIMUM_OVERLAYS` (xaero/common/minimap/write/MinimapWriter.java:92)
- [x] `MinimapWriter.SUN_MINIMUM` (xaero/common/minimap/write/MinimapWriter.java:93)
- [x] `MinimapWriter.NO_Y_VALUE` (xaero/common/minimap/write/MinimapWriter.java:94)
- [x] `MinimapWriter.MAX_TRANSPARENCY_BLEND_DEPTH` (xaero/common/minimap/write/MinimapWriter.java:95)
- [ ] `MinimapWriter.MinimapWriter(4)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:236)
- [ ] `MinimapWriter.getBlockStateLightEmission(3)`: it has no body (xaero/common/minimap/write/MinimapWriter.java:288)
- [ ] `MinimapWriter.setupDimensionHighlightHandler(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:290)
- [ ] `MinimapWriter.updateTimeDebug(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:294)
- [ ] `MinimapWriter.onRender(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:323)
- [ ] `MinimapWriter.beforeWriting(4)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:488)
- [ ] `MinimapWriter.writeChunk(7)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:515)
- [ ] `MinimapWriter.writeTile(16)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:802)
- [ ] `MinimapWriter.loadBlockColor(64)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:1078)
- [ ] `MinimapWriter.unpackFramedBlocks(3)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:1621)
- [ ] `MinimapWriter.findBlock(21)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:1641)
- [ ] `MinimapWriter.findBlockHelp(22)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:1778)
- [ ] `MinimapWriter.calculateBlockColors(15)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:1913)
- [ ] `MinimapWriter.isTransparent(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:1963)
- [ ] `MinimapWriter.isGlowing(3)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:1967)
- [ ] `MinimapWriter.applyTransparentLayer(10)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:1984)
- [ ] `MinimapWriter.applyTransparentLayer(5)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2038)
- [ ] `MinimapWriter.getBlockTint(3)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2050)
- [ ] `MinimapWriter.getQuads(5)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2071)
- [ ] `MinimapWriter.getParticleIcon(5)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2076)
- [ ] `MinimapWriter.loadBlockColourFromTexture(5)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2081)
- [ ] `MinimapWriter.addBlockColorMultipliers(4)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2119)
- [ ] `MinimapWriter.ignoreWorld(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2145)
- [ ] `MinimapWriter.getCaving(4)`: its body couldn't be read: a labelled statement (xaero/common/minimap/write/MinimapWriter.java:2155)
- [ ] `MinimapWriter.getSectionBasedHeight(2)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2281)
- [ ] `MinimapWriter.furtherZoomout(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2315)
- [ ] `MinimapWriter.getLoadSide(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2319)
- [ ] `MinimapWriter.getUpdateRadiusInChunks(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2323)
- [ ] `MinimapWriter.getMapCoord(2)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2327)
- [ ] `MinimapWriter.getLoadedCaving(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2331)
- [ ] `MinimapWriter.notEmptyColor(3)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2335)
- [ ] `MinimapWriter.getFixedSkyLightBlockBrightness(3)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2339)
- [ ] `MinimapWriter.getBlockBrightness(4)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2343)
- [ ] `MinimapWriter.getLoadingMapChunkX(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2353)
- [ ] `MinimapWriter.getLoadingMapChunkZ(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2357)
- [ ] `MinimapWriter.getLoadingSideInChunks(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2361)
- [ ] `MinimapWriter.getLoadedBlocks(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2365)
- [ ] `MinimapWriter.getLoadedMapChunkZ(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2369)
- [ ] `MinimapWriter.getLoadedMapChunkX(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2373)
- [ ] `MinimapWriter.getLoadedLevels(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2377)
- [ ] `MinimapWriter.setClearBlockColours(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2381)
- [ ] `MinimapWriter.cleanup(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2385)
- [ ] `MinimapWriter.resetShortBlocks(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2398)
- [ ] `MinimapWriter.getDimensionHighlightHandler(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2402)
- [ ] `MinimapWriter.getLoadedSideInChunks(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2406)
- [ ] `MinimapWriter.isLoadedNonWorldMap(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2410)
- [ ] `MinimapWriter.onResourceReload(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/minimap/write/MinimapWriter.java:2414)

### `MinimapWriterFabric`: it extends `MinimapWriter`

- [ ] `MinimapWriterFabric.MinimapWriterFabric(4)`: its class isn't translated (it extends `MinimapWriter`) (xaero/common/minimap/write/MinimapWriterFabric.java:12)
- [ ] `MinimapWriterFabric.getBlockStateLightEmission(3)`: its class isn't translated (it extends `MinimapWriter`) (xaero/common/minimap/write/MinimapWriterFabric.java:18)

### `MinimapWriterHelper`

- [x] `MinimapWriterHelper.getGlowingColour(4)` (xaero/common/minimap/write/MinimapWriterHelper.java:4)

### `CachedFunction`: it's generic

- [ ] `CachedFunction.CachedFunction(1)`: its class isn't translated (it's generic) (xaero/common/misc/CachedFunction.java:12)
- [ ] `CachedFunction.apply(1)`: its class isn't translated (it's generic) (xaero/common/misc/CachedFunction.java:16)
- [ ] `CachedFunction.clear(0)`: its class isn't translated (it's generic) (xaero/common/misc/CachedFunction.java:32)

### `Internet`

- [ ] `Internet.cipher`: it uses `Cipher` (javax.crypto) (xaero/common/misc/Internet.java:23)
- [ ] `Internet.checkModVersion(1)`: it uses IXaeroMinimap, which isn't translated (it's an interface) (xaero/common/misc/Internet.java:25)

### `ListFactory`: it's an interface

- [ ] `ListFactory.get(0)`: it has no body (xaero/common/misc/ListFactory.java:6)

### `MapFactory`: it's an interface

- [ ] `MapFactory.get(0)`: it has no body (xaero/common/misc/MapFactory.java:6)

### `Misc`

- [ ] `Misc.getMouseX(2)`: it uses Minecraft's Minecraft.mouseHandler (no Horizon helper for it yet) (xaero/common/misc/Misc.java:17)
- [ ] `Misc.getMouseY(2)`: it uses Minecraft's Minecraft.mouseHandler (no Horizon helper for it yet) (xaero/common/misc/Misc.java:21)
- [ ] `Misc.getChunkPosAsLong(1)`: it uses Minecraft's `LevelChunk` (xaero/common/misc/Misc.java:25)
- [ ] `Misc.hasItem(2)`: it calls Minecraft's Inventory.getContainerSize (no Horizon helper for it yet) (xaero/common/misc/Misc.java:29)
- [ ] `Misc.hasItem(3)`: it uses Minecraft's `NonNullList` (xaero/common/misc/Misc.java:47)
- [ ] `Misc.getFixedDisplayName(1)`: it uses Minecraft's `Entity` (xaero/common/misc/Misc.java:57)
- [ ] `Misc.hasEffect(2)`: it uses Minecraft's `Holder` (xaero/common/misc/Misc.java:66)
- [ ] `Misc.hasEffect(1)`: it uses Minecraft's `Holder` (xaero/common/misc/Misc.java:70)
- [ ] `Misc.isValidResourceLocationString(1)`: it calls Minecraft's `Identifier`.isAllowedInIdentifier (xaero/common/misc/Misc.java:74)

### `OptimizedMath`

- [x] `OptimizedMath.XP` (xaero/common/misc/OptimizedMath.java:10)
- [x] `OptimizedMath.YP` (xaero/common/misc/OptimizedMath.java:11)
- [x] `OptimizedMath.ZP` (xaero/common/misc/OptimizedMath.java:12)
- [x] `OptimizedMath.myFloor(1)` (xaero/common/misc/OptimizedMath.java:14)
- [ ] `OptimizedMath.rotatePose(3)`: it uses Minecraft's `Pose` (xaero/common/misc/OptimizedMath.java:23)
- [ ] `OptimizedMath.rotateMatrix(3)`: JOML's Matrix4f.rotate with 2 values (xaero/common/misc/OptimizedMath.java:29)

### `MixinPlayer`


### `MixinWaystoneStorage`

- [ ] `MixinAbstractClientPlayerEntity`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:7)
- [ ] `MixinChatListener`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:13)
- [ ] `MixinChunkDeltaUpdateS2CPacket`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:10)
- [ ] `MixinClientPlayerEntity`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:12)
- [ ] `MixinClientPlayNetworkHandler`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:23)
- [ ] `MixinClientWorld`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:9)
- [ ] `MixinFabricBossBarHud`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:13)
- [ ] `MixinFabricChatListener`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:17)
- [ ] `MixinFabricClientPacketListener`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:12)
- [ ] `MixinFabricGameRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:16)
- [ ] `MixinFabricGui`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:17)
- [ ] `MixinFabricKeyMapping`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:11)
- [ ] `MixinFabricMinecraftClient`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:15)
- [ ] `MixinFabricMinecraftServer`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:13)
- [ ] `MixinFabricPlayerEntity`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:12)
- [ ] `MixinFabricPlayerList`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:15)
- [ ] `MixinFabricSpriteAtlasTexture`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:13)
- [ ] `MixinGameRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:11)
- [ ] `MixinGuiGraphics`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:10)
- [ ] `MixinGuiRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:15)
- [ ] `MixinLevelStorageAccess`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:11)
- [ ] `MixinLocalPlayer`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:7)
- [ ] `MixinMinecraftClient`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:9)
- [ ] `MixinMinecraftServer`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:9)
- [ ] `MixinModel`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:13)
- [ ] `MixinModelFeatureRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:11)
- [ ] `MixinModelPart`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:13)
- [ ] `MixinOptionalGuiRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:18)
- [ ] `MixinOptionalInGameHud`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:13)
- [ ] `MixinOptionalLevelRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:15)
- [ ] `MixinPictureInPictureRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:11)
- [ ] `MixinPlayerList`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:13)
- [ ] `MixinServerPlayer`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:9)
- [ ] `MixinSheetedDecalTextureGeneratorAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:8)
- [ ] `MixinWorldChunk`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:7)
- [ ] `MixinPlayerWaystoneManager`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java:11)

### `ClientClaimChangeListener`: it implements `IClaimsManagerListenerAPI` (xaero.pac.common.claims.tracker.api)

- [ ] `ClientClaimChangeListener.onWholeRegionChange(3)`: its class isn't translated (it implements `IClaimsManagerListenerAPI` (xaero.pac.common.claims.tracker.api)) (xaero/common/mods/pac/ClientClaimChangeListener.java:11)
- [ ] `ClientClaimChangeListener.onChunkChange(4)`: its class isn't translated (it implements `IClaimsManagerListenerAPI` (xaero.pac.common.claims.tracker.api)) (xaero/common/mods/pac/ClientClaimChangeListener.java:26)
- [ ] `ClientClaimChangeListener.onDimensionChange(1)`: its class isn't translated (it implements `IClaimsManagerListenerAPI` (xaero.pac.common.claims.tracker.api)) (xaero/common/mods/pac/ClientClaimChangeListener.java:41)

### `ClaimsHighlighter`: it extends `ChunkHighlighter` (xaero.common.minimap.highlight)

- [ ] `ClaimsHighlighter.ClaimsHighlighter(2)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.common.minimap.highlight)) (xaero/common/mods/pac/highlight/ClaimsHighlighter.java:43)
- [ ] `ClaimsHighlighter.regionHasHighlights(3)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.common.minimap.highlight)) (xaero/common/mods/pac/highlight/ClaimsHighlighter.java:50)
- [ ] `ClaimsHighlighter.getColors(3)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.common.minimap.highlight)) (xaero/common/mods/pac/highlight/ClaimsHighlighter.java:56)
- [ ] `ClaimsHighlighter.getChunkHighlitColor(3)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.common.minimap.highlight)) (xaero/common/mods/pac/highlight/ClaimsHighlighter.java:88)
- [ ] `ClaimsHighlighter.applyAnchorPixel(4)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.common.minimap.highlight)) (xaero/common/mods/pac/highlight/ClaimsHighlighter.java:117)
- [ ] `ClaimsHighlighter.chunkIsHighlit(3)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.common.minimap.highlight)) (xaero/common/mods/pac/highlight/ClaimsHighlighter.java:123)
- [ ] `ClaimsHighlighter.addChunkHighlightTooltips(5)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.common.minimap.highlight)) (xaero/common/mods/pac/highlight/ClaimsHighlighter.java:128)
- [ ] `ClaimsHighlighter.getClaimsCustomName(2)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.common.minimap.highlight)) (xaero/common/mods/pac/highlight/ClaimsHighlighter.java:174)
- [ ] `ClaimsHighlighter.getClaimsColor(2)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.common.minimap.highlight)) (xaero/common/mods/pac/highlight/ClaimsHighlighter.java:178)

### `OPACPlayerTrackerSystem`: it implements `IRenderedPlayerTracker` (xaero.hud.minimap.player.tracker.system)

- [ ] `OPACPlayerTrackerSystem.OPACPlayerTrackerSystem(1)`: its class isn't translated (it implements `IRenderedPlayerTracker` (xaero.hud.minimap.player.tracker.system)) (xaero/common/mods/pac/party/OPACPlayerTrackerSystem.java:13)
- [ ] `OPACPlayerTrackerSystem.getReader(0)`: its class isn't translated (it implements `IRenderedPlayerTracker` (xaero.hud.minimap.player.tracker.system)) (xaero/common/mods/pac/party/OPACPlayerTrackerSystem.java:18)
- [ ] `OPACPlayerTrackerSystem.getTrackedPlayerIterator(0)`: its class isn't translated (it implements `IRenderedPlayerTracker` (xaero.hud.minimap.player.tracker.system)) (xaero/common/mods/pac/party/OPACPlayerTrackerSystem.java:23)

### `OPACTrackedPlayerReader`: it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)

- [ ] `OPACTrackedPlayerReader.getId(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)) (xaero/common/mods/pac/party/OPACTrackedPlayerReader.java:16)
- [ ] `OPACTrackedPlayerReader.getX(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)) (xaero/common/mods/pac/party/OPACTrackedPlayerReader.java:20)
- [ ] `OPACTrackedPlayerReader.getY(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)) (xaero/common/mods/pac/party/OPACTrackedPlayerReader.java:24)
- [ ] `OPACTrackedPlayerReader.getZ(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)) (xaero/common/mods/pac/party/OPACTrackedPlayerReader.java:28)
- [ ] `OPACTrackedPlayerReader.getDimension(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)) (xaero/common/mods/pac/party/OPACTrackedPlayerReader.java:32)

### `SupportOpenPartiesAndClaims`: it uses IXaeroMinimap, which isn't translated (it's an interface)

- [ ] `SupportOpenPartiesAndClaims.SupportOpenPartiesAndClaims(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/pac/SupportOpenPartiesAndClaims.java:27)
- [ ] `SupportOpenPartiesAndClaims.register(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/pac/SupportOpenPartiesAndClaims.java:35)
- [ ] `SupportOpenPartiesAndClaims.claimAt(3)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/pac/SupportOpenPartiesAndClaims.java:40)
- [ ] `SupportOpenPartiesAndClaims.onMapRender(8)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/pac/SupportOpenPartiesAndClaims.java:44)
- [ ] `SupportOpenPartiesAndClaims.isFromParty(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/pac/SupportOpenPartiesAndClaims.java:56)
- [ ] `SupportOpenPartiesAndClaims.registerHighlighters(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/pac/SupportOpenPartiesAndClaims.java:61)
- [ ] `SupportOpenPartiesAndClaims.getModMain(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/pac/SupportOpenPartiesAndClaims.java:65)
- [ ] `SupportOpenPartiesAndClaims.getAllyIterator(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/pac/SupportOpenPartiesAndClaims.java:69)

### `SupportAmecs`

- [ ] `SupportAmecs.SupportAmecs(1)`: it uses `Logger` (org.apache.logging.log4j) (xaero/common/mods/SupportAmecs.java:9)
- [ ] `SupportAmecs.modifiersArePressed(1)`: it uses Minecraft's `KeyMapping` (xaero/common/mods/SupportAmecs.java:12)

### `SupportFramedBlocks`: it uses `Class`

- [ ] `SupportFramedBlocks.SupportFramedBlocks(0)`: its class isn't translated (it uses `Class`) (xaero/common/mods/SupportFramedBlocks.java:26)
- [ ] `SupportFramedBlocks.onWorldChange(0)`: its class isn't translated (it uses `Class`) (xaero/common/mods/SupportFramedBlocks.java:76)
- [ ] `SupportFramedBlocks.findFramedBlocks(2)`: its class isn't translated (it uses `Class`) (xaero/common/mods/SupportFramedBlocks.java:80)
- [ ] `SupportFramedBlocks.isFrameBlock(3)`: its class isn't translated (it uses `Class`) (xaero/common/mods/SupportFramedBlocks.java:96)
- [ ] `SupportFramedBlocks.unpackFramedBlock(4)`: its class isn't translated (it uses `Class`) (xaero/common/mods/SupportFramedBlocks.java:105)

### `SupportIris`

- [x] `SupportIris.SupportIris(0)` (xaero/common/mods/SupportIris.java:4)

### `SupportMods`: it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))

- [ ] `SupportMods.ensureCheckedLater(0)`: its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))) (xaero/common/mods/SupportMods.java:27)
- [ ] `SupportMods.worldmap(0)`: its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))) (xaero/common/mods/SupportMods.java:33)
- [ ] `SupportMods.pac(0)`: its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))) (xaero/common/mods/SupportMods.java:37)
- [ ] `SupportMods.shouldUseWorldMapChunks(0)`: its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))) (xaero/common/mods/SupportMods.java:42)
- [ ] `SupportMods.shouldUseWorldMapCaveChunks(0)`: its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))) (xaero/common/mods/SupportMods.java:47)
- [ ] `SupportMods.framedBlocks(0)`: its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))) (xaero/common/mods/SupportMods.java:51)
- [ ] `SupportMods.waystones(0)`: its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))) (xaero/common/mods/SupportMods.java:55)
- [ ] `SupportMods.getSupportWaystones(0)`: its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))) (xaero/common/mods/SupportMods.java:59)
- [ ] `SupportMods.checkForMinimapDuplicates(1)`: a try/catch (ClassNotFoundException) (xaero/common/mods/SupportMods.java:63)
- [ ] `SupportMods.SupportMods(1)`: its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))) (xaero/common/mods/SupportMods.java:71)
- [ ] `SupportMods.checkLater(0)`: its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))) (xaero/common/mods/SupportMods.java:134)
- [ ] `SupportMods.registerClientEvents(0)`: its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))) (xaero/common/mods/SupportMods.java:146)
- [ ] `SupportMods.onClientTick(0)`: its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))) (xaero/common/mods/SupportMods.java:152)
- [ ] `SupportMods.onMinimapSessionStarted(1)`: its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))) (xaero/common/mods/SupportMods.java:155)

### `SupportModsFabric`: it extends `SupportMods`

- [ ] `SupportModsFabric.SupportModsFabric(1)`: its class isn't translated (it extends `SupportMods`) (xaero/common/mods/SupportModsFabric.java:11)
- [ ] `SupportModsFabric.amecs(0)`: its class isn't translated (it extends `SupportMods`) (xaero/common/mods/SupportModsFabric.java:21)
- [ ] `SupportModsFabric.wraithWaystones(0)`: its class isn't translated (it extends `SupportMods`) (xaero/common/mods/SupportModsFabric.java:25)
- [ ] `SupportModsFabric.registerClientEvents(0)`: its class isn't translated (it extends `SupportMods`) (xaero/common/mods/SupportModsFabric.java:29)
- [ ] `SupportModsFabric.onClientTick(0)`: its class isn't translated (it extends `SupportMods`) (xaero/common/mods/SupportModsFabric.java:34)
- [ ] `SupportModsFabric.onMinimapSessionStarted(1)`: its class isn't translated (it extends `SupportMods`) (xaero/common/mods/SupportModsFabric.java:39)

### `SupportWraithWaystones`


### `SupportXaeroWorldmap`: it uses IXaeroMinimap, which isn't translated (it's an interface)

- [x] `SupportXaeroWorldmap.WORLDMAP_COMPATIBILITY_VERSION` (xaero/common/mods/SupportXaeroWorldmap.java:58)
- [x] `SupportXaeroWorldmap.MINIMAP_MW` (xaero/common/mods/SupportXaeroWorldmap.java:59)
- [ ] `SupportXaeroWorldmap.seedsUsed`: it uses `MapTileChunk` (xaero.map.region) (xaero/common/mods/SupportXaeroWorldmap.java:61)
- [x] `SupportXaeroWorldmap.black` (xaero/common/mods/SupportXaeroWorldmap.java:62)
- [x] `SupportXaeroWorldmap.slime` (xaero/common/mods/SupportXaeroWorldmap.java:63)
- [ ] `SupportXaeroWorldmap.SupportXaeroWorldmap(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:71)
- [ ] `SupportXaeroWorldmap.drawMinimap(14)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:84)
- [ ] `SupportXaeroWorldmap.renderChunks(23)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:185)
- [ ] `SupportXaeroWorldmap.bumpLoadedRegion(2)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:245)
- [ ] `SupportXaeroWorldmap.renderSlimeChunks(7)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:251)
- [ ] `SupportXaeroWorldmap.getWorldMapWaypoints(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:275)
- [ ] `SupportXaeroWorldmap.getWorldMapColours(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:280)
- [ ] `SupportXaeroWorldmap.getWorldMapFlowers(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:285)
- [ ] `SupportXaeroWorldmap.getWorldMapTerrainDepth(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:290)
- [ ] `SupportXaeroWorldmap.getWorldMapTerrainSlopes(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:295)
- [ ] `SupportXaeroWorldmap.getWorldMapBiomeColorsVanillaMode(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:300)
- [ ] `SupportXaeroWorldmap.getWorldMapIgnoreHeightmaps(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:305)
- [ ] `SupportXaeroWorldmap.tryToGetMultiworldId(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:315)
- [ ] `SupportXaeroWorldmap.getMultiworldIdUnsynced(2)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:323)
- [ ] `SupportXaeroWorldmap.getPotentialMultiworldIds(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:330)
- [ ] `SupportXaeroWorldmap.getMultiworldIds(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:339)
- [ ] `SupportXaeroWorldmap.tryToGetMultiworldName(2)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:348)
- [ ] `SupportXaeroWorldmap.getMultiworldNameUnsynced(3)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:356)
- [ ] `SupportXaeroWorldmap.openSettings(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:361)
- [ ] `SupportXaeroWorldmap.getSettingsScreen(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:366)
- [ ] `SupportXaeroWorldmap.getSettingsScreen(2)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:371)
- [ ] `SupportXaeroWorldmap.getMinimapBrightness(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:375)
- [ ] `SupportXaeroWorldmap.prepareMapTexturedRect(11)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:386)
- [ ] `SupportXaeroWorldmap.getAdjustHeightForCarpetLikeBlocks(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:418)
- [ ] `SupportXaeroWorldmap.registerHighlighters(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:423)
- [ ] `SupportXaeroWorldmap.createRadarRenderWrapper(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:427)
- [ ] `SupportXaeroWorldmap.worldMapIsRenderingRadar(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:431)
- [ ] `SupportXaeroWorldmap.getPartialYTeleport(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:436)
- [ ] `SupportXaeroWorldmap.isStainedGlassDisplayed(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:441)
- [ ] `SupportXaeroWorldmap.isMultiplayerMap(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:446)
- [ ] `SupportXaeroWorldmap.getManualCaveStart(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:456)
- [ ] `SupportXaeroWorldmap.hasEnabledCaveLayers(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:463)
- [ ] `SupportXaeroWorldmap.getCaveModeType(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:467)
- [ ] `SupportXaeroWorldmap.getWorldMapScreenForOption(2)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:490)
- [ ] `SupportXaeroWorldmap.getCaveModeDepth(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:510)
- [ ] `SupportXaeroWorldmap.isLegibleCaveMaps(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:515)
- [ ] `SupportXaeroWorldmap.getBiomeBlending(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:520)
- [ ] `SupportXaeroWorldmap.confirmPlayerRadarRender(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:525)
- [ ] `SupportXaeroWorldmap.getDisplayClaims(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:531)
- [ ] `SupportXaeroWorldmap.getClaimsBorderOpacity(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:536)
- [ ] `SupportXaeroWorldmap.getClaimsFillOpacity(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:541)
- [ ] `SupportXaeroWorldmap.toggleChunkClaims(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:546)
- [ ] `SupportXaeroWorldmap.caveLayersAreUsable(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:550)
- [ ] `SupportXaeroWorldmap.shouldPreventAutoCaveMode(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:577)
- [ ] `SupportXaeroWorldmap.getMapDimensionScale(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:588)
- [ ] `SupportXaeroWorldmap.getMapDimension(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/mods/SupportXaeroWorldmap.java:605)

### `WorldMapHighlighter`: it extends `AbstractHighlighter` (xaero.common.minimap.highlight)

- [ ] `WorldMapHighlighter.WorldMapHighlighter(1)`: its class isn't translated (it extends `AbstractHighlighter` (xaero.common.minimap.highlight)) (xaero/common/mods/WorldMapHighlighter.java:15)
- [ ] `WorldMapHighlighter.regionHasHighlights(3)`: its class isn't translated (it extends `AbstractHighlighter` (xaero.common.minimap.highlight)) (xaero/common/mods/WorldMapHighlighter.java:21)
- [ ] `WorldMapHighlighter.chunkIsHighlit(3)`: its class isn't translated (it extends `AbstractHighlighter` (xaero.common.minimap.highlight)) (xaero/common/mods/WorldMapHighlighter.java:26)
- [ ] `WorldMapHighlighter.getChunkHighlitColor(3)`: its class isn't translated (it extends `AbstractHighlighter` (xaero.common.minimap.highlight)) (xaero/common/mods/WorldMapHighlighter.java:31)
- [ ] `WorldMapHighlighter.addBlockHighlightTooltips(5)`: its class isn't translated (it extends `AbstractHighlighter` (xaero.common.minimap.highlight)) (xaero/common/mods/WorldMapHighlighter.java:36)
- [ ] `WorldMapHighlighter.isCoveringOutsideDiscovered(0)`: its class isn't translated (it extends `AbstractHighlighter` (xaero.common.minimap.highlight)) (xaero/common/mods/WorldMapHighlighter.java:46)

### `PlatformContext`

- [ ] `PlatformContext.createClientEvents(1)`: it has no body (xaero/common/PlatformContext.java:15)
- [ ] `PlatformContext.createCommonEvents(1)`: it has no body (xaero/common/PlatformContext.java:17)
- [ ] `PlatformContext.getLoaderClientOnly(0)`: it has no body (xaero/common/PlatformContext.java:19)
- [ ] `PlatformContext.getLoaderCommon(0)`: it has no body (xaero/common/PlatformContext.java:21)
- [ ] `PlatformContext.createModClientEvents(1)`: it has no body (xaero/common/PlatformContext.java:23)
- [ ] `PlatformContext.createSupportMods(1)`: it has no body (xaero/common/PlatformContext.java:25)
- [ ] `PlatformContext.createModCommonEvents(1)`: it has no body (xaero/common/PlatformContext.java:27)
- [ ] `PlatformContext.createMinimapWriter(4)`: it has no body (xaero/common/PlatformContext.java:29)
- [ ] `PlatformContext.getModInfoVersion(0)`: it has no body (xaero/common/PlatformContext.java:31)
- [ ] `PlatformContext.createSupportServerMods(1)`: it has no body (xaero/common/PlatformContext.java:33)

### `PlatformContextFabric`: it extends `PlatformContext`

- [ ] `PlatformContextFabric.PlatformContextFabric(1)`: its class isn't translated (it extends `PlatformContext`) (xaero/common/PlatformContextFabric.java:31)
- [ ] `PlatformContextFabric.postLoadClient(0)`: its class isn't translated (it extends `PlatformContext`) (xaero/common/PlatformContextFabric.java:35)
- [ ] `PlatformContextFabric.postLoadCommon(0)`: its class isn't translated (it extends `PlatformContext`) (xaero/common/PlatformContextFabric.java:39)
- [ ] `PlatformContextFabric.postLoadServer(0)`: its class isn't translated (it extends `PlatformContext`) (xaero/common/PlatformContextFabric.java:42)
- [ ] `PlatformContextFabric.preTryLoadLater(0)`: its class isn't translated (it extends `PlatformContext`) (xaero/common/PlatformContextFabric.java:46)
- [ ] `PlatformContextFabric.preTryLoadLaterServer(0)`: its class isn't translated (it extends `PlatformContext`) (xaero/common/PlatformContextFabric.java:63)
- [ ] `PlatformContextFabric.createClientEvents(1)`: its class isn't translated (it extends `PlatformContext`) (xaero/common/PlatformContextFabric.java:80)
- [ ] `PlatformContextFabric.createCommonEvents(1)`: its class isn't translated (it extends `PlatformContext`) (xaero/common/PlatformContextFabric.java:85)
- [ ] `PlatformContextFabric.getLoaderClientOnly(0)`: its class isn't translated (it extends `PlatformContext`) (xaero/common/PlatformContextFabric.java:90)
- [ ] `PlatformContextFabric.getLoaderCommon(0)`: its class isn't translated (it extends `PlatformContext`) (xaero/common/PlatformContextFabric.java:98)
- [ ] `PlatformContextFabric.createModClientEvents(1)`: its class isn't translated (it extends `PlatformContext`) (xaero/common/PlatformContextFabric.java:106)
- [ ] `PlatformContextFabric.createModCommonEvents(1)`: its class isn't translated (it extends `PlatformContext`) (xaero/common/PlatformContextFabric.java:111)
- [ ] `PlatformContextFabric.createSupportMods(1)`: its class isn't translated (it extends `PlatformContext`) (xaero/common/PlatformContextFabric.java:116)
- [ ] `PlatformContextFabric.createSupportServerMods(1)`: its class isn't translated (it extends `PlatformContext`) (xaero/common/PlatformContextFabric.java:121)
- [ ] `PlatformContextFabric.createMinimapWriter(4)`: its class isn't translated (it extends `PlatformContext`) (xaero/common/PlatformContextFabric.java:126)
- [ ] `PlatformContextFabric.getModInfoVersion(0)`: its class isn't translated (it extends `PlatformContext`) (xaero/common/PlatformContextFabric.java:133)
- [ ] `PlatformContextFabric.get(0)`: it uses PlatformContextFabric, which isn't translated (it extends `PlatformContext`) (xaero/common/PlatformContextFabric.java:139)

### `PlatformContextLoaderClientOnly`

- [ ] `PlatformContextLoaderClientOnly.preInit(2)`: it has no body (xaero/common/PlatformContextLoaderClientOnly.java:4)

### `PlatformContextLoaderClientOnlyFabric`: it extends `PlatformContextLoaderClientOnly`

- [ ] `PlatformContextLoaderClientOnlyFabric.preInit(2)`: its class isn't translated (it extends `PlatformContextLoaderClientOnly`) (xaero/common/PlatformContextLoaderClientOnlyFabric.java:10)

### `PlatformContextLoaderCommon`

- [ ] `PlatformContextLoaderCommon.setup(1)`: it has no body (xaero/common/PlatformContextLoaderCommon.java:4)

### `PlatformContextLoaderCommonFabric`: it extends `PlatformContextLoaderCommon`

- [ ] `PlatformContextLoaderCommonFabric.setup(1)`: its class isn't translated (it extends `PlatformContextLoaderCommon`) (xaero/common/PlatformContextLoaderCommonFabric.java:12)

### `Services`

- [ ] `Services.PLATFORM`: it uses IPlatformHelper, which isn't translated (it's an interface) (xaero/common/platform/Services.java:8)
- [ ] `Services.load(1)`: it's generic (xaero/common/platform/Services.java:10)

### `FabricPlatformHelper`: it implements `IPlatformHelper`

- [ ] `FabricPlatformHelper.getPlatformName(0)`: its class isn't translated (it implements `IPlatformHelper`) (xaero/common/platform/services/FabricPlatformHelper.java:15)
- [ ] `FabricPlatformHelper.isModLoaded(1)`: its class isn't translated (it implements `IPlatformHelper`) (xaero/common/platform/services/FabricPlatformHelper.java:20)
- [ ] `FabricPlatformHelper.isDevelopmentEnvironment(0)`: its class isn't translated (it implements `IPlatformHelper`) (xaero/common/platform/services/FabricPlatformHelper.java:25)
- [ ] `FabricPlatformHelper.isDedicatedServer(0)`: its class isn't translated (it implements `IPlatformHelper`) (xaero/common/platform/services/FabricPlatformHelper.java:30)
- [ ] `FabricPlatformHelper.getGameDir(0)`: its class isn't translated (it implements `IPlatformHelper`) (xaero/common/platform/services/FabricPlatformHelper.java:35)
- [ ] `FabricPlatformHelper.getConfigDir(0)`: its class isn't translated (it implements `IPlatformHelper`) (xaero/common/platform/services/FabricPlatformHelper.java:40)
- [ ] `FabricPlatformHelper.getModFile(1)`: its class isn't translated (it implements `IPlatformHelper`) (xaero/common/platform/services/FabricPlatformHelper.java:45)

### `IPlatformHelper`: it's an interface

- [ ] `IPlatformHelper.getPlatformName(0)`: it has no body (xaero/common/platform/services/IPlatformHelper.java:6)
- [ ] `IPlatformHelper.isModLoaded(1)`: it has no body (xaero/common/platform/services/IPlatformHelper.java:8)
- [ ] `IPlatformHelper.checkModForMixin(1)`: its class isn't translated (it's an interface) (xaero/common/platform/services/IPlatformHelper.java:10)
- [ ] `IPlatformHelper.isDevelopmentEnvironment(0)`: it has no body (xaero/common/platform/services/IPlatformHelper.java:14)
- [ ] `IPlatformHelper.getEnvironmentName(0)`: its class isn't translated (it's an interface) (xaero/common/platform/services/IPlatformHelper.java:16)
- [ ] `IPlatformHelper.isDedicatedServer(0)`: it has no body (xaero/common/platform/services/IPlatformHelper.java:20)
- [ ] `IPlatformHelper.getGameDir(0)`: it has no body (xaero/common/platform/services/IPlatformHelper.java:22)
- [ ] `IPlatformHelper.getConfigDir(0)`: it has no body (xaero/common/platform/services/IPlatformHelper.java:24)
- [ ] `IPlatformHelper.getModFile(1)`: it has no body (xaero/common/platform/services/IPlatformHelper.java:26)

### `MapPool`: it's generic

- [ ] `MapPool.MapPool(1)`: its class isn't translated (it's generic) (xaero/common/pool/MapPool.java:10)
- [ ] `MapPool.get(1)`: its class isn't translated (it's generic) (xaero/common/pool/MapPool.java:15)
- [ ] `MapPool.addToPool(1)`: its class isn't translated (it's generic) (xaero/common/pool/MapPool.java:31)
- [ ] `MapPool.takeFromPool(0)`: its class isn't translated (it's generic) (xaero/common/pool/MapPool.java:42)
- [ ] `MapPool.size(0)`: its class isn't translated (it's generic) (xaero/common/pool/MapPool.java:46)
- [ ] `MapPool.construct(1)`: it has no body (xaero/common/pool/MapPool.java:50)

### `PoolUnit`: it's an interface

- [ ] `PoolUnit.create(1)`: it has no body (xaero/common/pool/PoolUnit.java:4)

### `XaeroMinimapServerCore`

- [ ] `XaeroMinimapServerCore.modMain`: it uses IXaeroMinimap, which isn't translated (it's an interface) (xaero/common/server/core/XaeroMinimapServerCore.java:8)
- [ ] `XaeroMinimapServerCore.onServerWorldInfo(1)`: it calls XaeroMinimapServerCore.isModLoaded, which isn't translated (xaero/common/server/core/XaeroMinimapServerCore.java:10)
- [ ] `XaeroMinimapServerCore.isModLoaded(0)`: it uses XaeroMinimapServerCore.modMain, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/server/core/XaeroMinimapServerCore.java:16)

### `ServerEvents`: it uses XaeroMinimapServer, which isn't translated (it uses `Logger` (org.apache.logging.log4j))

- [ ] `ServerEvents.ServerEvents(1)`: it stores a second reference to an object (Rust would need it copied or shared) (xaero/common/server/events/ServerEvents.java:8)
- [ ] `ServerEvents.register(0)`: its class isn't translated (it uses XaeroMinimapServer, which isn't translated (it uses `Logger` (org.apache.logging.log4j))) (xaero/common/server/events/ServerEvents.java:12)

### `IMinecraftServer`: it's an interface

- [ ] `IMinecraftServer.getXaeroMinimapServerData(0)`: it has no body (xaero/common/server/IMinecraftServer.java:4)
- [ ] `IMinecraftServer.setXaeroMinimapServerData(1)`: it has no body (xaero/common/server/IMinecraftServer.java:6)

### `LevelMapProperties`

- [ ] `LevelMapProperties.write(1)`: it's overloaded with the same number of parameters (xaero/common/server/level/LevelMapProperties.java:13)
- [ ] `LevelMapProperties.read(1)`: it's overloaded with the same number of parameters (xaero/common/server/level/LevelMapProperties.java:17)
- [x] `LevelMapProperties.isUsable(0)` (xaero/common/server/level/LevelMapProperties.java:30)
- [x] `LevelMapProperties.setUsable(1)` (xaero/common/server/level/LevelMapProperties.java:34)
- [x] `LevelMapProperties.getId(0)` (xaero/common/server/level/LevelMapProperties.java:38)
- [ ] `LevelMapProperties.read(1)`: it's overloaded with the same number of parameters (xaero/common/server/level/LevelMapProperties.java:42)
- [ ] `LevelMapProperties.write(1)`: it's overloaded with the same number of parameters (xaero/common/server/level/LevelMapProperties.java:48)

### `LevelMapPropertiesIO`

- [x] `LevelMapPropertiesIO.FILE_NAME` (xaero/common/server/level/LevelMapPropertiesIO.java:16)
- [ ] `LevelMapPropertiesIO.load(2)`: a try/finally (xaero/common/server/level/LevelMapPropertiesIO.java:18)
- [ ] `LevelMapPropertiesIO.save(2)`: a try-with-resources over something other than a file reader or writer (xaero/common/server/level/LevelMapPropertiesIO.java:33)

### `MinecraftServerData`: a Map or Set keyed by a Path

- [ ] `MinecraftServerData.MinecraftServerData(3)`: its class isn't translated (a Map or Set keyed by a Path) (xaero/common/server/MinecraftServerData.java:23)
- [ ] `MinecraftServerData.getLevelProperties(1)`: its class isn't translated (a Map or Set keyed by a Path) (xaero/common/server/MinecraftServerData.java:31)
- [ ] `MinecraftServerData.getSyncedPlayerTrackerSystemManager(0)`: its class isn't translated (a Map or Set keyed by a Path) (xaero/common/server/MinecraftServerData.java:59)
- [ ] `MinecraftServerData.getSyncedPlayerTracker(0)`: its class isn't translated (a Map or Set keyed by a Path) (xaero/common/server/MinecraftServerData.java:63)
- [ ] `MinecraftServerData.get(1)`: it uses MinecraftServerData, which isn't translated (a Map or Set keyed by a Path) (xaero/common/server/MinecraftServerData.java:67)
- [ ] `MinecraftServerData.getModMain(0)`: its class isn't translated (a Map or Set keyed by a Path) (xaero/common/server/MinecraftServerData.java:71)

### `MineraftServerDataInitializer`

- [ ] `MineraftServerDataInitializer.init(2)`: it uses Minecraft's `MinecraftServer` (xaero/common/server/MineraftServerDataInitializer.java:9)

### `ArgonautsSyncedPlayerTrackerSystem`: it implements `ISyncedPlayerTrackerSystem` (xaero.common.server.radar.tracker)

- [ ] `ArgonautsSyncedPlayerTrackerSystem.getTrackingLevel(2)`: its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.common.server.radar.tracker)) (xaero/common/server/mods/argonauts/ArgonautsSyncedPlayerTrackerSystem.java:11)
- [ ] `ArgonautsSyncedPlayerTrackerSystem.isPartySystem(0)`: its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.common.server.radar.tracker)) (xaero/common/server/mods/argonauts/ArgonautsSyncedPlayerTrackerSystem.java:18)
- [ ] `ArgonautsSyncedPlayerTrackerSystem.getPartyTrackingLevel(2)`: its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.common.server.radar.tracker)) (xaero/common/server/mods/argonauts/ArgonautsSyncedPlayerTrackerSystem.java:23)
- [ ] `ArgonautsSyncedPlayerTrackerSystem.getGuildTrackingLevel(2)`: its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.common.server.radar.tracker)) (xaero/common/server/mods/argonauts/ArgonautsSyncedPlayerTrackerSystem.java:33)

### `SupportArgonautsServer`: it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)

- [ ] `SupportArgonautsServer.getSyncedPlayerTrackerSystem(0)`: its class isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)) (xaero/common/server/mods/argonauts/SupportArgonautsServer.java:8)

### `FTBTeamsSyncedPlayerTrackerSystem`: it implements `ISyncedPlayerTrackerSystem` (xaero.common.server.radar.tracker)

- [ ] `FTBTeamsSyncedPlayerTrackerSystem.getTrackingLevel(2)`: its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.common.server.radar.tracker)) (xaero/common/server/mods/ftbteams/FTBTeamsSyncedPlayerTrackerSystem.java:10)
- [ ] `FTBTeamsSyncedPlayerTrackerSystem.isPartySystem(0)`: its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.common.server.radar.tracker)) (xaero/common/server/mods/ftbteams/FTBTeamsSyncedPlayerTrackerSystem.java:31)

### `SupportFTBTeamsServer`: it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)

- [ ] `SupportFTBTeamsServer.getSyncedPlayerTrackerSystem(0)`: its class isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)) (xaero/common/server/mods/ftbteams/SupportFTBTeamsServer.java:8)

### `ServerPlayerOpacData`


### `SupportOPACServer`

- [ ] `SupportOPACServer.isPositionSyncAllowed(3)`: it uses ServerPlayerData, which isn't translated (it uses `UUID` (java.util)) (xaero/common/server/mods/opac/SupportOPACServer.java:11)
- [ ] `SupportOPACServer.getReceiveLocationsFromMutualAlliesConfigValue(1)`: it uses Minecraft's `ServerPlayer` (xaero/common/server/mods/opac/SupportOPACServer.java:24)
- [ ] `SupportOPACServer.getReceiveLocationsFromPartyConfigValue(1)`: it uses Minecraft's `ServerPlayer` (xaero/common/server/mods/opac/SupportOPACServer.java:30)
- [ ] `SupportOPACServer.updateShareLocationConfigValues(2)`: it uses Minecraft's `ServerPlayer` (xaero/common/server/mods/opac/SupportOPACServer.java:36)
- [ ] `SupportOPACServer.getPlayerOpacData(1)`: it uses `Object` (xaero/common/server/mods/opac/SupportOPACServer.java:44)

### `SupportServerMods`: it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))

- [ ] `SupportServerMods.check(1)`: its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))) (xaero/common/server/mods/SupportServerMods.java:14)
- [ ] `SupportServerMods.hasFtbTeams(0)`: its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))) (xaero/common/server/mods/SupportServerMods.java:40)
- [ ] `SupportServerMods.getFtbTeams(0)`: its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))) (xaero/common/server/mods/SupportServerMods.java:44)
- [ ] `SupportServerMods.hasArgonauts(0)`: its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))) (xaero/common/server/mods/SupportServerMods.java:48)
- [ ] `SupportServerMods.getArgonauts(0)`: its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))) (xaero/common/server/mods/SupportServerMods.java:52)
- [ ] `SupportServerMods.hasOpac(0)`: its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))) (xaero/common/server/mods/SupportServerMods.java:56)
- [ ] `SupportServerMods.getOpac(0)`: its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))) (xaero/common/server/mods/SupportServerMods.java:60)
- [ ] `SupportServerMods.hasWorldmap(0)`: its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))) (xaero/common/server/mods/SupportServerMods.java:64)
- [ ] `SupportServerMods.getWorldmap(0)`: its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))) (xaero/common/server/mods/SupportServerMods.java:68)
- [ ] `SupportServerMods.registerEvents(0)`: its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))) (xaero/common/server/mods/SupportServerMods.java:72)

### `SupportServerModsFabric`: it extends `SupportServerMods`


### `SupportWorldMapServer`

- [x] `SupportWorldMapServer.supportsTrackedPlayers(0)` (xaero/common/server/mods/SupportWorldMapServer.java:8)

### `IServerPlayer`: it's an interface

- [ ] `IServerPlayer.getXaeroMinimapPlayerData(0)`: it has no body (xaero/common/server/player/IServerPlayer.java:4)
- [ ] `IServerPlayer.setXaeroMinimapPlayerData(1)`: it has no body (xaero/common/server/player/IServerPlayer.java:6)

### `ServerPlayerData`: it uses `UUID` (java.util)

- [ ] `ServerPlayerData.ServerPlayerData(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/player/ServerPlayerData.java:17)
- [ ] `ServerPlayerData.getLastSyncedData(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/player/ServerPlayerData.java:21)
- [ ] `ServerPlayerData.ensureLastSyncedData(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/player/ServerPlayerData.java:25)
- [ ] `ServerPlayerData.getCurrentlySyncedPlayers(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/player/ServerPlayerData.java:33)
- [ ] `ServerPlayerData.ensureCurrentlySyncedPlayers(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/player/ServerPlayerData.java:37)
- [ ] `ServerPlayerData.getLastTrackedPlayerSync(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/player/ServerPlayerData.java:45)
- [ ] `ServerPlayerData.setLastTrackedPlayerSync(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/player/ServerPlayerData.java:49)
- [ ] `ServerPlayerData.get(1)`: it uses ServerPlayerData, which isn't translated (it uses `UUID` (java.util)) (xaero/common/server/player/ServerPlayerData.java:53)
- [ ] `ServerPlayerData.hasMod(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/player/ServerPlayerData.java:62)
- [ ] `ServerPlayerData.setClientModNetworkVersion(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/player/ServerPlayerData.java:66)
- [ ] `ServerPlayerData.getClientModNetworkVersion(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/player/ServerPlayerData.java:70)
- [ ] `ServerPlayerData.setOpacData(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/player/ServerPlayerData.java:74)
- [ ] `ServerPlayerData.getOpacData(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/player/ServerPlayerData.java:78)

### `ServerPlayerTickHandler`

- [ ] `ServerPlayerTickHandler.tick(1)`: it uses Minecraft's `ServerPlayer` (xaero/common/server/player/ServerPlayerTickHandler.java:8)

### `ISyncedPlayerTrackerSystem`: it's an interface

- [ ] `ISyncedPlayerTrackerSystem.getTrackingLevel(2)`: it has no body (xaero/common/server/radar/tracker/ISyncedPlayerTrackerSystem.java:6)
- [ ] `ISyncedPlayerTrackerSystem.isPartySystem(0)`: it has no body (xaero/common/server/radar/tracker/ISyncedPlayerTrackerSystem.java:8)

### `SyncedPlayerTracker`: it uses IXaeroMinimap, which isn't translated (it's an interface)

- [ ] `SyncedPlayerTracker.SyncedPlayerTracker(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/server/radar/tracker/SyncedPlayerTracker.java:21)
- [ ] `SyncedPlayerTracker.onTick(4)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/server/radar/tracker/SyncedPlayerTracker.java:25)
- [ ] `SyncedPlayerTracker.sendRemovePacket(3)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/server/radar/tracker/SyncedPlayerTracker.java:104)
- [ ] `SyncedPlayerTracker.sendTrackedPlayerPacket(3)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/server/radar/tracker/SyncedPlayerTracker.java:110)

### `SyncedPlayerTrackerSystemManager`: it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)

- [ ] `SyncedPlayerTrackerSystemManager.register(2)`: its class isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)) (xaero/common/server/radar/tracker/SyncedPlayerTrackerSystemManager.java:10)
- [ ] `SyncedPlayerTrackerSystemManager.getSystems(0)`: its class isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)) (xaero/common/server/radar/tracker/SyncedPlayerTrackerSystemManager.java:19)

### `SyncedTrackedPlayer`: it uses `UUID` (java.util)

- [ ] `SyncedTrackedPlayer.SyncedTrackedPlayer(5)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/radar/tracker/SyncedTrackedPlayer.java:15)
- [ ] `SyncedTrackedPlayer.setPos(3)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/radar/tracker/SyncedTrackedPlayer.java:23)
- [ ] `SyncedTrackedPlayer.setDimension(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/radar/tracker/SyncedTrackedPlayer.java:30)
- [ ] `SyncedTrackedPlayer.getId(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/radar/tracker/SyncedTrackedPlayer.java:35)
- [ ] `SyncedTrackedPlayer.getX(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/radar/tracker/SyncedTrackedPlayer.java:39)
- [ ] `SyncedTrackedPlayer.getY(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/radar/tracker/SyncedTrackedPlayer.java:43)
- [ ] `SyncedTrackedPlayer.getZ(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/radar/tracker/SyncedTrackedPlayer.java:47)
- [ ] `SyncedTrackedPlayer.getDimensionKey(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/radar/tracker/SyncedTrackedPlayer.java:51)
- [ ] `SyncedTrackedPlayer.matchesEnough(2)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/radar/tracker/SyncedTrackedPlayer.java:55)
- [ ] `SyncedTrackedPlayer.update(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/radar/tracker/SyncedTrackedPlayer.java:62)
- [ ] `SyncedTrackedPlayer.copyFrom(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/common/server/radar/tracker/SyncedTrackedPlayer.java:66)

### `XaeroMinimapServer`: it uses `Logger` (org.apache.logging.log4j)

- [x] `XaeroMinimapServer.SERVER_COMPATIBILITY` (xaero/common/server/XaeroMinimapServer.java:9)
- [ ] `XaeroMinimapServer.XaeroMinimapServer(1)`: its class isn't translated (it uses `Logger` (org.apache.logging.log4j)) (xaero/common/server/XaeroMinimapServer.java:14)
- [ ] `XaeroMinimapServer.load(0)`: its class isn't translated (it uses `Logger` (org.apache.logging.log4j)) (xaero/common/server/XaeroMinimapServer.java:18)
- [ ] `XaeroMinimapServer.loadLater(0)`: its class isn't translated (it uses `Logger` (org.apache.logging.log4j)) (xaero/common/server/XaeroMinimapServer.java:23)
- [ ] `XaeroMinimapServer.getModMain(0)`: its class isn't translated (it uses `Logger` (org.apache.logging.log4j)) (xaero/common/server/XaeroMinimapServer.java:26)
- [ ] `XaeroMinimapServer.getServerEvents(0)`: its class isn't translated (it uses `Logger` (org.apache.logging.log4j)) (xaero/common/server/XaeroMinimapServer.java:30)

### `ModSettings`: it uses IXaeroMinimap, which isn't translated (it's an interface)

- [x] `ModSettings.defaultSettings` (xaero/common/settings/ModSettings.java:38)
- [x] `ModSettings.format` (xaero/common/settings/ModSettings.java:39)
- [x] `ModSettings.serverSettings` (xaero/common/settings/ModSettings.java:45)
- [ ] `ModSettings.legacyServerSlimeSeeds`: it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)) (xaero/common/settings/ModSettings.java:46)
- [x] `ModSettings.SHOW_LIGHT_LEVEL_NAMES` (xaero/common/settings/ModSettings.java:47)
- [x] `ModSettings.OLD_MINIMAP_SIZES` (xaero/common/settings/ModSettings.java:50)
- [ ] `ModSettings.ModSettings(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:59)
- [ ] `ModSettings.isKeyRepeat(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:68)
- [ ] `ModSettings.getMinimap(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:73)
- [ ] `ModSettings.setSlimeChunksSeed(2)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:78)
- [ ] `ModSettings.getSlimeChunksSeed(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:93)
- [ ] `ModSettings.getLegacySlimeChunksSeed(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:104)
- [ ] `ModSettings.removeLegacySlimeChunksSeed(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:109)
- [ ] `ModSettings.getSlimeChunks(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:114)
- [ ] `ModSettings.waypointsGUI(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:119)
- [ ] `ModSettings.getMinimapScale(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:126)
- [ ] `ModSettings.getUIScale(3)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:130)
- [ ] `ModSettings.getAutoUIScale(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:134)
- [ ] `ModSettings.getMaxWaypointsDistance(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:138)
- [ ] `ModSettings.assumeUsingFBO(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:142)
- [ ] `ModSettings.isIgnoreHeightmaps(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:146)
- [ ] `ModSettings.writeSettings(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:155)
- [ ] `ModSettings.saveSettings(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:158)
- [ ] `ModSettings.readSetting(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:185)
- [ ] `ModSettings.loadDefaultSettings(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:497)
- [ ] `ModSettings.loadSettings(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:510)
- [ ] `ModSettings.loadSettingsFile(2)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:525)
- [ ] `ModSettings.getTranslation(1)`: it uses `Object` (xaero/common/settings/ModSettings.java:580)
- [ ] `ModSettings.refreshScreen(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:584)
- [ ] `ModSettings.minimapDisabled(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:589)
- [ ] `ModSettings.caveMapsDisabled(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:593)
- [ ] `ModSettings.showWaypointsDisabled(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:606)
- [ ] `ModSettings.deathpointsDisabled(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:610)
- [ ] `ModSettings.resetServerSettings(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:614)
- [x] `ModSettings.setServerSettings(0)` (xaero/common/settings/ModSettings.java:618)
- [ ] `ModSettings.canEditIngameSettings(0)`: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/common/settings/ModSettings.java:621)
- [ ] `ModSettings.getEntityRadarBackwardsCompatibilityConfig(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:626)
- [ ] `ModSettings.resetEntityRadarBackwardsCompatibilityConfig(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:630)
- [ ] `ModSettings.foundOldRadarSettings(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:635)
- [ ] `ModSettings.getLoadedWaypointLines(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:639)
- [ ] `ModSettings.removeLoadedWaypointLines(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/common/settings/ModSettings.java:643)

### `FieldValidatorHolder`: it uses WaypointCoordinateFieldValidator, which isn't translated (it extends `NumericFieldValidator`)

- [ ] `FieldValidatorHolder.FieldValidatorHolder(2)`: its class isn't translated (it uses WaypointCoordinateFieldValidator, which isn't translated (it extends `NumericFieldValidator`)) (xaero/common/validator/FieldValidatorHolder.java:7)
- [ ] `FieldValidatorHolder.getNumericFieldValidator(0)`: its class isn't translated (it uses WaypointCoordinateFieldValidator, which isn't translated (it extends `NumericFieldValidator`)) (xaero/common/validator/FieldValidatorHolder.java:12)
- [ ] `FieldValidatorHolder.getWpCoordFieldValidator(0)`: its class isn't translated (it uses WaypointCoordinateFieldValidator, which isn't translated (it extends `NumericFieldValidator`)) (xaero/common/validator/FieldValidatorHolder.java:16)

### `NumericFieldValidator`

- [x] `NumericFieldValidator.charIsValid(2)` (xaero/common/validator/NumericFieldValidator.java:8)
- [ ] `NumericFieldValidator.validate(1)`: it uses Minecraft's `EditBox` (xaero/common/validator/NumericFieldValidator.java:12)
- [ ] `NumericFieldValidator.checkNumberFormat(1)`: it calls `Integer`.parseInt (xaero/common/validator/NumericFieldValidator.java:32)

### `WaypointCoordinateFieldValidator`: it extends `NumericFieldValidator`

- [ ] `WaypointCoordinateFieldValidator.charIsValid(2)`: its class isn't translated (it extends `NumericFieldValidator`) (xaero/common/validator/WaypointCoordinateFieldValidator.java:4)
- [ ] `WaypointCoordinateFieldValidator.checkNumberFormat(1)`: its class isn't translated (it extends `NumericFieldValidator`) (xaero/common/validator/WaypointCoordinateFieldValidator.java:9)

### `XaeroHudModMenu`: it implements `ModMenuApi` (com.terraformersmc.modmenu.api)

- [ ] `XaeroHudModMenu.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) (xaero/common/XaeroHudModMenu.java:8)

### `XaeroMinimapSession`: it extends `HudSession` (xaero.hud)

- [ ] `XaeroMinimapSession.XaeroMinimapSession(1)`: its class isn't translated (it extends `HudSession` (xaero.hud)) (xaero/common/XaeroMinimapSession.java:12)
- [ ] `XaeroMinimapSession.init(1)`: its class isn't translated (it extends `HudSession` (xaero.hud)) (xaero/common/XaeroMinimapSession.java:16)
- [ ] `XaeroMinimapSession.cleanup(0)`: its class isn't translated (it extends `HudSession` (xaero.hud)) (xaero/common/XaeroMinimapSession.java:21)
- [ ] `XaeroMinimapSession.getMinimapProcessor(0)`: its class isn't translated (it extends `HudSession` (xaero.hud)) (xaero/common/XaeroMinimapSession.java:26)
- [ ] `XaeroMinimapSession.getCurrentSession(0)`: it uses XaeroMinimapSession, which isn't translated (it extends `HudSession` (xaero.hud)) (xaero/common/XaeroMinimapSession.java:30)
- [ ] `XaeroMinimapSession.getForPlayer(1)`: it uses XaeroMinimapSession, which isn't translated (it extends `HudSession` (xaero.hud)) (xaero/common/XaeroMinimapSession.java:34)
- [ ] `XaeroMinimapSession.getModMain(0)`: its class isn't translated (it extends `HudSession` (xaero.hud)) (xaero/common/XaeroMinimapSession.java:38)

### `FilterObjectCategory`: it's generic

- [ ] `FilterObjectCategory.FilterObjectCategory(12)`: its class isn't translated (it's generic) (xaero/hud/category/FilterObjectCategory.java:27)
- [ ] `FilterObjectCategory.getBaseRule(0)`: its class isn't translated (it's generic) (xaero/hud/category/FilterObjectCategory.java:51)
- [ ] `FilterObjectCategory.getIncludeList(1)`: it's generic (xaero/hud/category/FilterObjectCategory.java:55)
- [ ] `FilterObjectCategory.getExcludeList(1)`: it's generic (xaero/hud/category/FilterObjectCategory.java:59)
- [ ] `FilterObjectCategory.getIncludeLists(0)`: its class isn't translated (it's generic) (xaero/hud/category/FilterObjectCategory.java:63)
- [ ] `FilterObjectCategory.getExcludeLists(0)`: its class isn't translated (it's generic) (xaero/hud/category/FilterObjectCategory.java:67)
- [ ] `FilterObjectCategory.getExcludeMode(0)`: its class isn't translated (it's generic) (xaero/hud/category/FilterObjectCategory.java:71)
- [ ] `FilterObjectCategory.getIncludeInSuperCategory(0)`: its class isn't translated (it's generic) (xaero/hud/category/FilterObjectCategory.java:75)

### `FilterObjectCategory.Builder`: it's generic

- [ ] `FilterObjectCategory.Builder.Builder(3)`: its class isn't translated (it's generic) (xaero/hud/category/FilterObjectCategory.java:87)
- [ ] `FilterObjectCategory.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/FilterObjectCategory.java:98)
- [ ] `FilterObjectCategory.Builder.getIncludeListBuilder(1)`: it's generic (xaero/hud/category/FilterObjectCategory.java:108)
- [ ] `FilterObjectCategory.Builder.getExcludeListBuilder(1)`: it's generic (xaero/hud/category/FilterObjectCategory.java:112)
- [ ] `FilterObjectCategory.Builder.setBaseRule(1)`: its class isn't translated (it's generic) (xaero/hud/category/FilterObjectCategory.java:116)
- [ ] `FilterObjectCategory.Builder.setExcludeMode(1)`: its class isn't translated (it's generic) (xaero/hud/category/FilterObjectCategory.java:121)
- [ ] `FilterObjectCategory.Builder.setIncludeInSuperCategory(1)`: its class isn't translated (it's generic) (xaero/hud/category/FilterObjectCategory.java:126)
- [ ] `FilterObjectCategory.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/FilterObjectCategory.java:131)
- [ ] `FilterObjectCategory.Builder.buildUnchecked(1)`: its class isn't translated (it's generic) (xaero/hud/category/FilterObjectCategory.java:139)
- [ ] `FilterObjectCategory.Builder.buildUncheckedFilter(5)`: it has no body (xaero/hud/category/FilterObjectCategory.java:160)

### `ObjectCategory`: it's generic

- [ ] `ObjectCategory.ObjectCategory(5)`: its class isn't translated (it's generic) (xaero/hud/category/ObjectCategory.java:21)
- [ ] `ObjectCategory.getName(0)`: its class isn't translated (it's generic) (xaero/hud/category/ObjectCategory.java:35)
- [ ] `ObjectCategory.getSuperCategory(0)`: its class isn't translated (it's generic) (xaero/hud/category/ObjectCategory.java:39)
- [ ] `ObjectCategory.setSuperCategory(1)`: its class isn't translated (it's generic) (xaero/hud/category/ObjectCategory.java:43)
- [ ] `ObjectCategory.getDirectSubCategoryIterator(0)`: its class isn't translated (it's generic) (xaero/hud/category/ObjectCategory.java:47)
- [ ] `ObjectCategory.getSettingValue(1)`: it's generic (xaero/hud/category/ObjectCategory.java:51)
- [ ] `ObjectCategory.setSettingValue(2)`: it's generic (xaero/hud/category/ObjectCategory.java:60)
- [ ] `ObjectCategory.getSettingOverridesIterator(0)`: its class isn't translated (it's generic) (xaero/hud/category/ObjectCategory.java:64)
- [ ] `ObjectCategory.getProtection(0)`: its class isn't translated (it's generic) (xaero/hud/category/ObjectCategory.java:68)

### `ObjectCategory.Builder`: it's generic

- [ ] `ObjectCategory.Builder.Builder(2)`: its class isn't translated (it's generic) (xaero/hud/category/ObjectCategory.java:82)
- [ ] `ObjectCategory.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ObjectCategory.java:89)
- [ ] `ObjectCategory.Builder.setName(1)`: its class isn't translated (it's generic) (xaero/hud/category/ObjectCategory.java:98)
- [ ] `ObjectCategory.Builder.setSuperCategory(1)`: its class isn't translated (it's generic) (xaero/hud/category/ObjectCategory.java:103)
- [ ] `ObjectCategory.Builder.addSubCategoryBuilder(1)`: its class isn't translated (it's generic) (xaero/hud/category/ObjectCategory.java:108)
- [ ] `ObjectCategory.Builder.setSettingValue(2)`: it's generic (xaero/hud/category/ObjectCategory.java:113)
- [ ] `ObjectCategory.Builder.setProtection(1)`: its class isn't translated (it's generic) (xaero/hud/category/ObjectCategory.java:118)
- [ ] `ObjectCategory.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ObjectCategory.java:123)
- [ ] `ObjectCategory.Builder.buildSubCategories(0)`: its class isn't translated (it's generic) (xaero/hud/category/ObjectCategory.java:133)
- [ ] `ObjectCategory.Builder.buildUnchecked(1)`: it has no body (xaero/hud/category/ObjectCategory.java:137)

### `ExcludeListMode`


### `ObjectCategoryExcludeList`: it's generic

- [ ] `ObjectCategoryExcludeList.ObjectCategoryExcludeList(4)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryExcludeList.java:13)
- [ ] `ObjectCategoryExcludeList.isFollowedBy(2)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryExcludeList.java:20)
- [ ] `ObjectCategoryExcludeList.getExcludeMode(0)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryExcludeList.java:26)

### `ObjectCategoryExcludeList.Builder`: it's generic

- [ ] `ObjectCategoryExcludeList.Builder.Builder(2)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryExcludeList.java:33)
- [ ] `ObjectCategoryExcludeList.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryExcludeList.java:37)
- [ ] `ObjectCategoryExcludeList.Builder.setExcludeMode(1)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryExcludeList.java:43)
- [ ] `ObjectCategoryExcludeList.Builder.build(3)`: it's generic (xaero/hud/category/rule/ObjectCategoryExcludeList.java:48)
- [ ] `ObjectCategoryExcludeList.Builder.build(1)`: it's generic (xaero/hud/category/rule/ObjectCategoryExcludeList.java:56)
- [ ] `ObjectCategoryExcludeList.Builder.buildInternally(1)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryExcludeList.java:60)
- [ ] `ObjectCategoryExcludeList.Builder.begin(2)`: it's generic (xaero/hud/category/rule/ObjectCategoryExcludeList.java:64)

### `ObjectCategoryHardRule`: it's generic

- [ ] `ObjectCategoryHardRule.ObjectCategoryHardRule(3)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryHardRule.java:11)
- [ ] `ObjectCategoryHardRule.isFollowedBy(2)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryHardRule.java:17)

### `ObjectCategoryHardRule.Builder`: it's generic

- [ ] `ObjectCategoryHardRule.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryHardRule.java:27)
- [ ] `ObjectCategoryHardRule.Builder.setName(1)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryHardRule.java:34)
- [ ] `ObjectCategoryHardRule.Builder.setPredicate(1)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryHardRule.java:39)
- [ ] `ObjectCategoryHardRule.Builder.setReversed(1)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryHardRule.java:44)
- [ ] `ObjectCategoryHardRule.Builder.build(2)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryHardRule.java:49)

### `ObjectCategoryHardRule.Predicate`: it's an interface

- [ ] `ObjectCategoryHardRule.Predicate.test(2)`: it has no body (xaero/hud/category/rule/ObjectCategoryHardRule.java:62)

### `ObjectCategoryIncludeList`: it's generic

- [ ] `ObjectCategoryIncludeList.ObjectCategoryIncludeList(3)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryIncludeList.java:11)
- [ ] `ObjectCategoryIncludeList.isFollowedBy(2)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryIncludeList.java:15)

### `ObjectCategoryIncludeList.Builder`: it's generic

- [ ] `ObjectCategoryIncludeList.Builder.Builder(2)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryIncludeList.java:21)
- [ ] `ObjectCategoryIncludeList.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryIncludeList.java:25)
- [ ] `ObjectCategoryIncludeList.Builder.begin(2)`: it's generic (xaero/hud/category/rule/ObjectCategoryIncludeList.java:30)
- [ ] `ObjectCategoryIncludeList.Builder.build(3)`: it's generic (xaero/hud/category/rule/ObjectCategoryIncludeList.java:34)
- [ ] `ObjectCategoryIncludeList.Builder.build(1)`: it's generic (xaero/hud/category/rule/ObjectCategoryIncludeList.java:42)
- [ ] `ObjectCategoryIncludeList.Builder.buildInternally(1)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryIncludeList.java:46)

### `ObjectCategoryListRule`: it's generic

- [ ] `ObjectCategoryListRule.ObjectCategoryListRule(4)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryListRule.java:18)
- [ ] `ObjectCategoryListRule.inList(2)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryListRule.java:25)
- [ ] `ObjectCategoryListRule.iterator(0)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryListRule.java:34)
- [ ] `ObjectCategoryListRule.getStringValidator(0)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryListRule.java:39)
- [ ] `ObjectCategoryListRule.getType(0)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryListRule.java:43)

### `ObjectCategoryListRule.Builder`: it's generic

- [ ] `ObjectCategoryListRule.Builder.Builder(2)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryListRule.java:52)
- [ ] `ObjectCategoryListRule.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryListRule.java:57)
- [ ] `ObjectCategoryListRule.Builder.addListElement(1)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryListRule.java:62)
- [ ] `ObjectCategoryListRule.Builder.getType(0)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryListRule.java:67)
- [ ] `ObjectCategoryListRule.Builder.build(3)`: it's generic (xaero/hud/category/rule/ObjectCategoryListRule.java:71)
- [ ] `ObjectCategoryListRule.Builder.buildInternally(1)`: it has no body (xaero/hud/category/rule/ObjectCategoryListRule.java:105)

### `ObjectCategoryListRuleType`: it's generic

- [ ] `ObjectCategoryListRuleType.ObjectCategoryListRuleType(9)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryListRuleType.java:19)
- [ ] `ObjectCategoryListRuleType.getId(0)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryListRuleType.java:41)
- [ ] `ObjectCategoryListRuleType.getGetter(0)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryListRuleType.java:45)
- [ ] `ObjectCategoryListRuleType.getAllElementSupplier(0)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryListRuleType.java:49)
- [ ] `ObjectCategoryListRuleType.getElementResolver(0)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryListRuleType.java:53)
- [ ] `ObjectCategoryListRuleType.getStringFixer(0)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryListRuleType.java:57)
- [ ] `ObjectCategoryListRuleType.getStringValidator(0)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryListRuleType.java:61)
- [ ] `ObjectCategoryListRuleType.getSerializer(0)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryListRuleType.java:65)

### `ObjectCategoryRule`: it's generic

- [ ] `ObjectCategoryRule.ObjectCategoryRule(1)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryRule.java:8)
- [ ] `ObjectCategoryRule.getName(0)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryRule.java:12)
- [ ] `ObjectCategoryRule.isFollowedBy(2)`: it has no body (xaero/hud/category/rule/ObjectCategoryRule.java:16)
- [ ] `ObjectCategoryRule.toString(0)`: its class isn't translated (it's generic) (xaero/hud/category/rule/ObjectCategoryRule.java:18)

### `ObjectCategoryRuleResolver`

- [x] `ObjectCategoryRuleResolver.ObjectCategoryRuleResolver(0)` (xaero/hud/category/rule/resolver/ObjectCategoryRuleResolver.java:11)
- [ ] `ObjectCategoryRuleResolver.resolve(3)`: it's generic (xaero/hud/category/rule/resolver/ObjectCategoryRuleResolver.java:14)
- [ ] `ObjectCategoryRuleResolver.followsRules(3)`: it's generic (xaero/hud/category/rule/resolver/ObjectCategoryRuleResolver.java:32)

### `ObjectCategoryRuleResolver.Builder`

- [x] `ObjectCategoryRuleResolver.Builder.Builder(0)` (xaero/hud/category/rule/resolver/ObjectCategoryRuleResolver.java:61)
- [ ] `ObjectCategoryRuleResolver.Builder.setDefault(0)`: `this` used as a value (xaero/hud/category/rule/resolver/ObjectCategoryRuleResolver.java:64)
- [x] `ObjectCategoryRuleResolver.Builder.build(0)` (xaero/hud/category/rule/resolver/ObjectCategoryRuleResolver.java:68)
- [ ] `ObjectCategoryRuleResolver.Builder.begin(0)`: it calls Builder.setDefault, which isn't translated (xaero/hud/category/rule/resolver/ObjectCategoryRuleResolver.java:72)

### `FilterObjectCategoryData`: it's generic

- [ ] `FilterObjectCategoryData.FilterObjectCategoryData(9)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryData.java:19)
- [ ] `FilterObjectCategoryData.getIncludeListIterator(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryData.java:38)
- [ ] `FilterObjectCategoryData.getExcludeListIterator(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryData.java:42)
- [ ] `FilterObjectCategoryData.getHardInclude(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryData.java:46)
- [ ] `FilterObjectCategoryData.getExcludeMode(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryData.java:50)
- [ ] `FilterObjectCategoryData.getIncludeListInSuperCategory(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryData.java:54)

### `FilterObjectCategoryData.Builder`: it's generic

- [ ] `FilterObjectCategoryData.Builder.Builder(2)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryData.java:66)
- [ ] `FilterObjectCategoryData.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryData.java:72)
- [ ] `FilterObjectCategoryData.Builder.setHardInclude(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryData.java:82)
- [ ] `FilterObjectCategoryData.Builder.setExcludeMode(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryData.java:86)
- [ ] `FilterObjectCategoryData.Builder.addToIncludeList(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryData.java:90)
- [ ] `FilterObjectCategoryData.Builder.addToExcludeList(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryData.java:95)
- [ ] `FilterObjectCategoryData.Builder.setIncludeListInSuperCategory(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryData.java:100)
- [ ] `FilterObjectCategoryData.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryData.java:105)

### `FilterObjectCategoryDataNbtSerializer`: it's generic

- [ ] `FilterObjectCategoryDataNbtSerializer.FilterObjectCategoryDataNbtSerializer(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryDataNbtSerializer.java:12)
- [ ] `FilterObjectCategoryDataNbtSerializer.serialize(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryDataNbtSerializer.java:16)
- [ ] `FilterObjectCategoryDataNbtSerializer.getConfiguredBuilder(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryDataNbtSerializer.java:30)

### `FilterObjectCategoryDataNbtSerializer.Builder`: it's generic

- [ ] `FilterObjectCategoryDataNbtSerializer.Builder.Builder(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryDataNbtSerializer.java:49)
- [ ] `FilterObjectCategoryDataNbtSerializer.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryDataNbtSerializer.java:52)

### `FilterObjectCategoryDataNbtSerializer.FinalBuilder`: it's generic

- [ ] `FilterObjectCategoryDataNbtSerializer.FinalBuilder.FinalBuilder(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryDataNbtSerializer.java:59)
- [ ] `FilterObjectCategoryDataNbtSerializer.FinalBuilder.buildInternally(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/FilterObjectCategoryDataNbtSerializer.java:62)
- [ ] `FilterObjectCategoryDataNbtSerializer.FinalBuilder.begin(0)`: it's generic (xaero/hud/category/serialization/data/FilterObjectCategoryDataNbtSerializer.java:66)

### `ObjectCategoryData`: it's generic

- [ ] `ObjectCategoryData.ObjectCategoryData(4)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryData.java:17)
- [ ] `ObjectCategoryData.getName(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryData.java:24)
- [ ] `ObjectCategoryData.getSettingOverrideIterator(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryData.java:28)
- [ ] `ObjectCategoryData.getSubCategoryIterator(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryData.java:32)
- [ ] `ObjectCategoryData.getProtection(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryData.java:36)

### `ObjectCategoryData.Builder`: it's generic

- [ ] `ObjectCategoryData.Builder.Builder(2)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryData.java:48)
- [ ] `ObjectCategoryData.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryData.java:54)
- [ ] `ObjectCategoryData.Builder.setName(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryData.java:61)
- [ ] `ObjectCategoryData.Builder.setSettingOverride(2)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryData.java:66)
- [ ] `ObjectCategoryData.Builder.addSubCategoryBuilder(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryData.java:71)
- [ ] `ObjectCategoryData.Builder.setProtection(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryData.java:76)
- [ ] `ObjectCategoryData.Builder.buildSubCategories(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryData.java:81)
- [ ] `ObjectCategoryData.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryData.java:85)
- [ ] `ObjectCategoryData.Builder.buildInternally(1)`: it has no body (xaero/hud/category/serialization/data/ObjectCategoryData.java:93)

### `ObjectCategoryDataGsonSerializer`: it's generic

- [ ] `ObjectCategoryDataGsonSerializer.ObjectCategoryDataGsonSerializer(2)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryDataGsonSerializer.java:10)
- [ ] `ObjectCategoryDataGsonSerializer.serialize(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryDataGsonSerializer.java:15)
- [ ] `ObjectCategoryDataGsonSerializer.deserialize(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryDataGsonSerializer.java:19)

### `ObjectCategoryDataGsonSerializer.Builder`: it's generic

- [ ] `ObjectCategoryDataGsonSerializer.Builder.Builder(2)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryDataGsonSerializer.java:27)
- [ ] `ObjectCategoryDataGsonSerializer.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryDataGsonSerializer.java:32)
- [ ] `ObjectCategoryDataGsonSerializer.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryDataGsonSerializer.java:37)
- [ ] `ObjectCategoryDataGsonSerializer.Builder.begin(2)`: it's generic (xaero/hud/category/serialization/data/ObjectCategoryDataGsonSerializer.java:41)

### `ObjectCategoryDataNbtSerializer`: it's generic

- [ ] `ObjectCategoryDataNbtSerializer.ObjectCategoryDataNbtSerializer(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryDataNbtSerializer.java:15)
- [ ] `ObjectCategoryDataNbtSerializer.serialize(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryDataNbtSerializer.java:19)
- [ ] `ObjectCategoryDataNbtSerializer.deserialize(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryDataNbtSerializer.java:49)
- [ ] `ObjectCategoryDataNbtSerializer.getConfiguredBuilder(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryDataNbtSerializer.java:53)

### `ObjectCategoryDataNbtSerializer.Builder`: it's generic

- [ ] `ObjectCategoryDataNbtSerializer.Builder.Builder(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryDataNbtSerializer.java:85)
- [ ] `ObjectCategoryDataNbtSerializer.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryDataNbtSerializer.java:88)
- [ ] `ObjectCategoryDataNbtSerializer.Builder.setBuilderSupplier(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryDataNbtSerializer.java:93)
- [ ] `ObjectCategoryDataNbtSerializer.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryDataNbtSerializer.java:98)
- [ ] `ObjectCategoryDataNbtSerializer.Builder.buildInternally(0)`: it has no body (xaero/hud/category/serialization/data/ObjectCategoryDataNbtSerializer.java:106)

### `ObjectCategoryDataNbtSerializer.FinalBuilder`: it's generic

- [ ] `ObjectCategoryDataNbtSerializer.FinalBuilder.FinalBuilder(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryDataNbtSerializer.java:111)
- [ ] `ObjectCategoryDataNbtSerializer.FinalBuilder.buildInternally(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryDataNbtSerializer.java:114)
- [ ] `ObjectCategoryDataNbtSerializer.FinalBuilder.begin(0)`: it's generic (xaero/hud/category/serialization/data/ObjectCategoryDataNbtSerializer.java:119)

### `ObjectCategoryDataSerializer`: it's generic

- [ ] `ObjectCategoryDataSerializer.ObjectCategoryDataSerializer(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryDataSerializer.java:4)
- [ ] `ObjectCategoryDataSerializer.serialize(1)`: it has no body (xaero/hud/category/serialization/data/ObjectCategoryDataSerializer.java:7)
- [ ] `ObjectCategoryDataSerializer.deserialize(1)`: it has no body (xaero/hud/category/serialization/data/ObjectCategoryDataSerializer.java:9)

### `ObjectCategoryDataSerializer.Builder`: it's generic

- [ ] `ObjectCategoryDataSerializer.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/data/ObjectCategoryDataSerializer.java:12)
- [ ] `ObjectCategoryDataSerializer.Builder.build(0)`: it has no body (xaero/hud/category/serialization/data/ObjectCategoryDataSerializer.java:16)

### `FilterObjectCategorySerializationHandler`: it's generic

- [ ] `FilterObjectCategorySerializationHandler.FilterObjectCategorySerializationHandler(8)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/FilterObjectCategorySerializationHandler.java:23)
- [ ] `FilterObjectCategorySerializationHandler.getConfiguredDataBuilderForCategory(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/FilterObjectCategorySerializationHandler.java:40)
- [ ] `FilterObjectCategorySerializationHandler.getListRulePrefix(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/FilterObjectCategorySerializationHandler.java:60)
- [ ] `FilterObjectCategorySerializationHandler.getConfiguredCategoryBuilderForData(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/FilterObjectCategorySerializationHandler.java:64)
- [ ] `FilterObjectCategorySerializationHandler.handleListRuleSerializedElement(2)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/FilterObjectCategorySerializationHandler.java:79)
- [ ] `FilterObjectCategorySerializationHandler.handleListRuleSerializedElement(5)`: it's generic (xaero/hud/category/serialization/FilterObjectCategorySerializationHandler.java:85)

### `FilterObjectCategorySerializationHandler.Builder`: it's generic

- [ ] `FilterObjectCategorySerializationHandler.Builder.Builder(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/FilterObjectCategorySerializationHandler.java:112)
- [ ] `FilterObjectCategorySerializationHandler.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/FilterObjectCategorySerializationHandler.java:116)
- [ ] `FilterObjectCategorySerializationHandler.Builder.setDefaultListRuleType(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/FilterObjectCategorySerializationHandler.java:125)
- [ ] `FilterObjectCategorySerializationHandler.Builder.setHardRuleGetter(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/FilterObjectCategorySerializationHandler.java:130)
- [ ] `FilterObjectCategorySerializationHandler.Builder.setListRuleTypeGetter(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/FilterObjectCategorySerializationHandler.java:135)
- [ ] `FilterObjectCategorySerializationHandler.Builder.setListRuleTypePrefixSeparator(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/FilterObjectCategorySerializationHandler.java:140)
- [ ] `FilterObjectCategorySerializationHandler.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/FilterObjectCategorySerializationHandler.java:145)

### `ObjectCategorySerializationHandler`: it's generic

- [ ] `ObjectCategorySerializationHandler.ObjectCategorySerializationHandler(4)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/ObjectCategorySerializationHandler.java:17)
- [ ] `ObjectCategorySerializationHandler.serialize(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/ObjectCategorySerializationHandler.java:29)
- [ ] `ObjectCategorySerializationHandler.convertToData(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/ObjectCategorySerializationHandler.java:34)
- [ ] `ObjectCategorySerializationHandler.getConfiguredDataBuilderForCategory(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/ObjectCategorySerializationHandler.java:39)
- [ ] `ObjectCategorySerializationHandler.convertFromData(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/ObjectCategorySerializationHandler.java:48)
- [ ] `ObjectCategorySerializationHandler.deserialize(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/ObjectCategorySerializationHandler.java:53)
- [ ] `ObjectCategorySerializationHandler.getConfiguredCategoryBuilderForData(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/ObjectCategorySerializationHandler.java:59)
- [ ] `ObjectCategorySerializationHandler.setSettingValue(3)`: it's generic (xaero/hud/category/serialization/ObjectCategorySerializationHandler.java:75)

### `ObjectCategorySerializationHandler.Builder`: it's generic

- [ ] `ObjectCategorySerializationHandler.Builder.Builder(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/ObjectCategorySerializationHandler.java:86)
- [ ] `ObjectCategorySerializationHandler.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/ObjectCategorySerializationHandler.java:90)
- [ ] `ObjectCategorySerializationHandler.Builder.setDataBuilderFactory(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/ObjectCategorySerializationHandler.java:97)
- [ ] `ObjectCategorySerializationHandler.Builder.setObjectCategoryBuilderFactory(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/ObjectCategorySerializationHandler.java:102)
- [ ] `ObjectCategorySerializationHandler.Builder.setSettingTypeGetter(1)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/ObjectCategorySerializationHandler.java:107)
- [ ] `ObjectCategorySerializationHandler.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/serialization/ObjectCategorySerializationHandler.java:112)
- [ ] `ObjectCategorySerializationHandler.Builder.buildInternally(0)`: it has no body (xaero/hud/category/serialization/ObjectCategorySerializationHandler.java:120)

### `ObjectCategoryDefaultSettingsSetter`: it uses ObjectCategorySetting, which isn't translated (it's generic)

- [ ] `ObjectCategoryDefaultSettingsSetter.ObjectCategoryDefaultSettingsSetter(1)`: its class isn't translated (it uses ObjectCategorySetting, which isn't translated (it's generic)) (xaero/hud/category/setting/ObjectCategoryDefaultSettingsSetter.java:10)
- [ ] `ObjectCategoryDefaultSettingsSetter.setDefaultsFor(2)`: its class isn't translated (it uses ObjectCategorySetting, which isn't translated (it's generic)) (xaero/hud/category/setting/ObjectCategoryDefaultSettingsSetter.java:14)
- [ ] `ObjectCategoryDefaultSettingsSetter.setForSetting(2)`: it's generic (xaero/hud/category/setting/ObjectCategoryDefaultSettingsSetter.java:27)

### `ObjectCategoryDefaultSettingsSetter.Builder`: it uses ObjectCategorySetting, which isn't translated (it's generic)

- [ ] `ObjectCategoryDefaultSettingsSetter.Builder.Builder(0)`: its class isn't translated (it uses ObjectCategorySetting, which isn't translated (it's generic)) (xaero/hud/category/setting/ObjectCategoryDefaultSettingsSetter.java:34)
- [ ] `ObjectCategoryDefaultSettingsSetter.Builder.setDefault(0)`: its class isn't translated (it uses ObjectCategorySetting, which isn't translated (it's generic)) (xaero/hud/category/setting/ObjectCategoryDefaultSettingsSetter.java:37)
- [ ] `ObjectCategoryDefaultSettingsSetter.Builder.setSettings(1)`: its class isn't translated (it uses ObjectCategorySetting, which isn't translated (it's generic)) (xaero/hud/category/setting/ObjectCategoryDefaultSettingsSetter.java:42)
- [ ] `ObjectCategoryDefaultSettingsSetter.Builder.build(0)`: its class isn't translated (it uses ObjectCategorySetting, which isn't translated (it's generic)) (xaero/hud/category/setting/ObjectCategoryDefaultSettingsSetter.java:47)
- [ ] `ObjectCategoryDefaultSettingsSetter.Builder.begin(0)`: it uses Builder, which isn't translated (it uses ObjectCategorySetting, which isn't translated (it's generic)) (xaero/hud/category/setting/ObjectCategoryDefaultSettingsSetter.java:55)

### `ObjectCategorySetting`: it's generic

- [ ] `ObjectCategorySetting.ObjectCategorySetting(10)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:24)
- [ ] `ObjectCategorySetting.getSettingUIType(0)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:48)
- [ ] `ObjectCategorySetting.getId(0)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:52)
- [ ] `ObjectCategorySetting.getDisplayName(0)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:56)
- [ ] `ObjectCategorySetting.getDefaultValue(0)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:60)
- [ ] `ObjectCategorySetting.getTooltip(0)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:64)
- [ ] `ObjectCategorySetting.getUiFirstOption(0)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:68)
- [ ] `ObjectCategorySetting.getUiLastOption(0)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:72)
- [ ] `ObjectCategorySetting.getIndexReader(0)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:76)
- [ ] `ObjectCategorySetting.getIndexWriter(0)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:80)
- [ ] `ObjectCategorySetting.getWidgetValueNameProvider(0)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:84)

### `ObjectCategorySetting.Builder`: it's generic

- [ ] `ObjectCategorySetting.Builder.Builder(0)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:100)
- [ ] `ObjectCategorySetting.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:103)
- [ ] `ObjectCategorySetting.Builder.setId(1)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:116)
- [ ] `ObjectCategorySetting.Builder.setDisplayName(1)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:121)
- [ ] `ObjectCategorySetting.Builder.setDefaultValue(1)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:126)
- [ ] `ObjectCategorySetting.Builder.setSettingUIType(1)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:131)
- [ ] `ObjectCategorySetting.Builder.setTooltip(1)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:136)
- [ ] `ObjectCategorySetting.Builder.setUiFirstOption(1)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:141)
- [ ] `ObjectCategorySetting.Builder.setUiLastOption(1)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:146)
- [ ] `ObjectCategorySetting.Builder.setIndexReader(1)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:151)
- [ ] `ObjectCategorySetting.Builder.setIndexWriter(1)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:156)
- [ ] `ObjectCategorySetting.Builder.setUiValueNameProvider(1)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:161)
- [ ] `ObjectCategorySetting.Builder.build(2)`: its class isn't translated (it's generic) (xaero/hud/category/setting/ObjectCategorySetting.java:166)
- [ ] `ObjectCategorySetting.Builder.begin(0)`: it's generic (xaero/hud/category/setting/ObjectCategorySetting.java:196)

### `EditorCategoryNodeConverter`: it's generic

- [ ] `EditorCategoryNodeConverter.EditorCategoryNodeConverter(2)`: its class isn't translated (it's generic) (xaero/hud/category/ui/EditorCategoryNodeConverter.java:15)
- [ ] `EditorCategoryNodeConverter.convert(2)`: its class isn't translated (it's generic) (xaero/hud/category/ui/EditorCategoryNodeConverter.java:20)
- [ ] `EditorCategoryNodeConverter.getConfiguredBuilder(2)`: its class isn't translated (it's generic) (xaero/hud/category/ui/EditorCategoryNodeConverter.java:24)
- [ ] `EditorCategoryNodeConverter.setSettingValue(3)`: it's generic (xaero/hud/category/ui/EditorCategoryNodeConverter.java:35)
- [ ] `EditorCategoryNodeConverter.convert(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/EditorCategoryNodeConverter.java:39)
- [ ] `EditorCategoryNodeConverter.getConfiguredBuilder(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/EditorCategoryNodeConverter.java:43)
- [ ] `EditorCategoryNodeConverter.setSettingValue(3)`: it's generic (xaero/hud/category/ui/EditorCategoryNodeConverter.java:52)

### `EditorCategoryNodeConverter.Builder`: it's generic

- [ ] `EditorCategoryNodeConverter.Builder.Builder(2)`: its class isn't translated (it's generic) (xaero/hud/category/ui/EditorCategoryNodeConverter.java:61)
- [ ] `EditorCategoryNodeConverter.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/EditorCategoryNodeConverter.java:66)
- [ ] `EditorCategoryNodeConverter.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/EditorCategoryNodeConverter.java:70)
- [ ] `EditorCategoryNodeConverter.Builder.buildInternally(0)`: it has no body (xaero/hud/category/ui/EditorCategoryNodeConverter.java:74)

### `EditorFilterCategoryNodeConverter`: it's generic

- [ ] `EditorFilterCategoryNodeConverter.EditorFilterCategoryNodeConverter(6)`: its class isn't translated (it's generic) (xaero/hud/category/ui/EditorFilterCategoryNodeConverter.java:25)
- [ ] `EditorFilterCategoryNodeConverter.getConfiguredBuilder(2)`: its class isn't translated (it's generic) (xaero/hud/category/ui/EditorFilterCategoryNodeConverter.java:40)
- [ ] `EditorFilterCategoryNodeConverter.getListRulePrefix(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/EditorFilterCategoryNodeConverter.java:63)
- [ ] `EditorFilterCategoryNodeConverter.getConfiguredBuilder(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/EditorFilterCategoryNodeConverter.java:67)

### `EditorFilterCategoryNodeConverter.Builder`: it's generic

- [ ] `EditorFilterCategoryNodeConverter.Builder.Builder(2)`: its class isn't translated (it's generic) (xaero/hud/category/ui/EditorFilterCategoryNodeConverter.java:105)
- [ ] `EditorFilterCategoryNodeConverter.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/EditorFilterCategoryNodeConverter.java:109)
- [ ] `EditorFilterCategoryNodeConverter.Builder.setDefaultListRuleType(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/EditorFilterCategoryNodeConverter.java:117)
- [ ] `EditorFilterCategoryNodeConverter.Builder.setListRuleTypeGetter(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/EditorFilterCategoryNodeConverter.java:122)
- [ ] `EditorFilterCategoryNodeConverter.Builder.setListRuleTypePrefixSeparator(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/EditorFilterCategoryNodeConverter.java:127)
- [ ] `EditorFilterCategoryNodeConverter.Builder.setInputRuleTypeStringValidator(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/EditorFilterCategoryNodeConverter.java:132)
- [ ] `EditorFilterCategoryNodeConverter.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/EditorFilterCategoryNodeConverter.java:137)
- [ ] `EditorFilterCategoryNodeConverter.Builder.buildInternally(0)`: it has no body (xaero/hud/category/ui/EditorFilterCategoryNodeConverter.java:146)

### `ConnectionLineType`


### `EditorListEntry`: it holds an object of its own class (a linked structure)

- [ ] `EditorListEntry.EditorListEntry(7)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:31)
- [ ] `EditorListEntry.onSelected(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:45)
- [ ] `EditorListEntry.mouseClicked(6)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:63)
- [ ] `EditorListEntry.confirmSelection(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:92)
- [ ] `EditorListEntry.mouseReleased(4)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:96)
- [ ] `EditorListEntry.mouseScrolled(4)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:106)
- [ ] `EditorListEntry.mouseDragged(6)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:118)
- [ ] `EditorListEntry.mouseMoved(2)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:128)
- [ ] `EditorListEntry.keyPressed(2)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:131)
- [ ] `EditorListEntry.keyReleased(1)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:150)
- [ ] `EditorListEntry.charTyped(1)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:162)
- [ ] `EditorListEntry.tick(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:171)
- [ ] `EditorListEntry.getSubNarration(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:179)
- [ ] `EditorListEntry.getHoveredNarration(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:183)
- [ ] `EditorListEntry.getSelectedNarration(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:187)
- [ ] `EditorListEntry.getTooltipSupplier(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:196)
- [ ] `EditorListEntry.getMessage(0)`: it has no body (xaero/hud/category/ui/entry/EditorListEntry.java:200)
- [ ] `EditorListEntry.getNarrationMessage(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:202)
- [ ] `EditorListEntry.getNarration(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:206)
- [ ] `EditorListEntry.getHoverNarration(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:221)
- [ ] `EditorListEntry.preRender(3)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:225)
- [ ] `EditorListEntry.render(13)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:235)
- [ ] `EditorListEntry.postRender(1)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:283)
- [ ] `EditorListEntry.isHoveredOver(2)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:288)
- [ ] `EditorListEntry.selectAction(0)`: it has no body (xaero/hud/category/ui/entry/EditorListEntry.java:295)
- [ ] `EditorListEntry.setFocused(1)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:297)
- [ ] `EditorListEntry.unhoverRecursively(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:300)
- [ ] `EditorListEntry.moveFocus(1)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:307)
- [ ] `EditorListEntry.moveFocus(2)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:317)
- [ ] `EditorListEntry.unfocusRecursively(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:353)
- [ ] `EditorListEntry.focusFirstRecursively(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:364)
- [ ] `EditorListEntry.focusLastRecursively(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:372)
- [ ] `EditorListEntry.withSubEntry(1)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:380)
- [ ] `EditorListEntry.getEntryRelativeX(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:385)
- [ ] `EditorListEntry.getEntryRelativeY(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/category/ui/entry/EditorListEntry.java:389)

### `EditorListEntryCategory`: it's generic

- [ ] `EditorListEntryCategory.HELP_TOOLTIP`: it uses `Tooltip` (xaero.lib.client.gui.widget) (xaero/hud/category/ui/entry/EditorListEntryCategory.java:12)
- [ ] `EditorListEntryCategory.PROTECTED_TOOLTIP`: it uses `Tooltip` (xaero.lib.client.gui.widget) (xaero/hud/category/ui/entry/EditorListEntryCategory.java:13)
- [ ] `EditorListEntryCategory.UP_TOOLTIP`: it uses `Tooltip` (xaero.lib.client.gui.widget) (xaero/hud/category/ui/entry/EditorListEntryCategory.java:14)
- [ ] `EditorListEntryCategory.DOWN_TOOLTIP`: it uses `Tooltip` (xaero.lib.client.gui.widget) (xaero/hud/category/ui/entry/EditorListEntryCategory.java:15)
- [ ] `EditorListEntryCategory.EditorListEntryCategory(8)`: its class isn't translated (it's generic) (xaero/hud/category/ui/entry/EditorListEntryCategory.java:17)
- [ ] `EditorListEntryCategory.getCategoryNameEntryFactory(4)`: its class isn't translated (it's generic) (xaero/hud/category/ui/entry/EditorListEntryCategory.java:68)
- [ ] `EditorListEntryCategory.getPasteEntryFactory(4)`: its class isn't translated (it's generic) (xaero/hud/category/ui/entry/EditorListEntryCategory.java:83)
- [ ] `EditorListEntryCategory.getPasteTooltip(2)`: its class isn't translated (it's generic) (xaero/hud/category/ui/entry/EditorListEntryCategory.java:96)
- [ ] `EditorListEntryCategory.getDuplicateEntryFactory(3)`: its class isn't translated (it's generic) (xaero/hud/category/ui/entry/EditorListEntryCategory.java:109)
- [ ] `EditorListEntryCategory.getProtectedEntryFactory(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/entry/EditorListEntryCategory.java:130)
- [ ] `EditorListEntryCategory.getCutEntryFactory(3)`: its class isn't translated (it's generic) (xaero/hud/category/ui/entry/EditorListEntryCategory.java:136)
- [ ] `EditorListEntryCategory.getPriorityEntryFactory(3)`: its class isn't translated (it's generic) (xaero/hud/category/ui/entry/EditorListEntryCategory.java:147)
- [ ] `EditorListEntryCategory.getMessage(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/entry/EditorListEntryCategory.java:165)

### `EditorListEntryDeletableListElement`: it extends `EditorListRootEntry`

- [ ] `EditorListEntryDeletableListElement.DELETE_TOOLTIP`: it uses `Tooltip` (xaero.lib.client.gui.widget) (xaero/hud/category/ui/entry/EditorListEntryDeletableListElement.java:15)
- [ ] `EditorListEntryDeletableListElement.EditorListEntryDeletableListElement(9)`: its class isn't translated (it extends `EditorListRootEntry`) (xaero/hud/category/ui/entry/EditorListEntryDeletableListElement.java:17)
- [ ] `EditorListEntryDeletableListElement.keyPressed(2)`: its class isn't translated (it extends `EditorListRootEntry`) (xaero/hud/category/ui/entry/EditorListEntryDeletableListElement.java:53)
- [ ] `EditorListEntryDeletableListElement.getMessage(0)`: its class isn't translated (it extends `EditorListRootEntry`) (xaero/hud/category/ui/entry/EditorListEntryDeletableListElement.java:67)

### `EditorListEntryExpandingOption`: it's generic

- [ ] `EditorListEntryExpandingOption.EditorListEntryExpandingOption(9)`: its class isn't translated (it's generic) (xaero/hud/category/ui/entry/EditorListEntryExpandingOption.java:12)
- [ ] `EditorListEntryExpandingOption.selectAction(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/entry/EditorListEntryExpandingOption.java:27)

### `EditorListEntryExpandingOptions`: it extends `EditorListEntryWidget`

- [ ] `EditorListEntryExpandingOptions.EditorListEntryExpandingOptions(10)`: its class isn't translated (it extends `EditorListEntryWidget`) (xaero/hud/category/ui/entry/EditorListEntryExpandingOptions.java:10)

### `EditorListEntryTextWithAction`: it extends `EditorListEntryWithIconAndText`

- [ ] `EditorListEntryTextWithAction.EditorListEntryTextWithAction(9)`: its class isn't translated (it extends `EditorListEntryWithIconAndText`) (xaero/hud/category/ui/entry/EditorListEntryTextWithAction.java:14)
- [ ] `EditorListEntryTextWithAction.EditorListEntryTextWithAction(10)`: its class isn't translated (it extends `EditorListEntryWithIconAndText`) (xaero/hud/category/ui/entry/EditorListEntryTextWithAction.java:28)
- [ ] `EditorListEntryTextWithAction.selectAction(0)`: its class isn't translated (it extends `EditorListEntryWithIconAndText`) (xaero/hud/category/ui/entry/EditorListEntryTextWithAction.java:44)

### `EditorListEntryWidget`: it extends `EditorListEntryWithRootReference`

- [ ] `EditorListEntryWidget.EditorListEntryWidget(9)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWidget.java:27)
- [ ] `EditorListEntryWidget.mouseClicked(6)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWidget.java:42)
- [ ] `EditorListEntryWidget.mouseReleased(4)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWidget.java:68)
- [ ] `EditorListEntryWidget.mouseDragged(6)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWidget.java:79)
- [ ] `EditorListEntryWidget.mouseScrolled(4)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWidget.java:86)
- [ ] `EditorListEntryWidget.mouseMoved(2)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWidget.java:93)
- [ ] `EditorListEntryWidget.keyPressed(2)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWidget.java:99)
- [ ] `EditorListEntryWidget.keyReleased(1)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWidget.java:104)
- [ ] `EditorListEntryWidget.charTyped(1)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWidget.java:109)
- [ ] `EditorListEntryWidget.tick(0)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWidget.java:114)
- [ ] `EditorListEntryWidget.getNarration(0)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWidget.java:119)
- [ ] `EditorListEntryWidget.getHoverNarration(0)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWidget.java:124)
- [ ] `EditorListEntryWidget.getMessage(0)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWidget.java:129)
- [ ] `EditorListEntryWidget.getNarrationMessage(0)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWidget.java:134)
- [ ] `EditorListEntryWidget.render(13)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWidget.java:145)
- [ ] `EditorListEntryWidget.setFocused(1)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWidget.java:192)
- [ ] `EditorListEntryWidget.selectAction(0)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWidget.java:201)

### `EditorListEntryWithIcon`: it extends `EditorListEntryWithRootReference`

- [ ] `EditorListEntryWithIcon.EditorListEntryWithIcon(12)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWithIcon.java:18)
- [ ] `EditorListEntryWithIcon.getIconX(0)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWithIcon.java:39)
- [ ] `EditorListEntryWithIcon.getIconY(0)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWithIcon.java:43)
- [ ] `EditorListEntryWithIcon.getIconW(0)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWithIcon.java:47)
- [ ] `EditorListEntryWithIcon.getIconH(0)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWithIcon.java:51)
- [ ] `EditorListEntryWithIcon.render(13)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWithIcon.java:55)
- [ ] `EditorListEntryWithIcon.selectAction(0)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWithIcon.java:90)
- [ ] `EditorListEntryWithIcon.getMessage(0)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListEntryWithIcon.java:95)

### `EditorListEntryWithIconAndText`: it extends `EditorListEntryWithIcon`

- [ ] `EditorListEntryWithIconAndText.EditorListEntryWithIconAndText(9)`: its class isn't translated (it extends `EditorListEntryWithIcon`) (xaero/hud/category/ui/entry/EditorListEntryWithIconAndText.java:15)
- [ ] `EditorListEntryWithIconAndText.setColor(1)`: its class isn't translated (it extends `EditorListEntryWithIcon`) (xaero/hud/category/ui/entry/EditorListEntryWithIconAndText.java:32)
- [ ] `EditorListEntryWithIconAndText.setHoverColor(1)`: its class isn't translated (it extends `EditorListEntryWithIcon`) (xaero/hud/category/ui/entry/EditorListEntryWithIconAndText.java:36)
- [ ] `EditorListEntryWithIconAndText.getColor(0)`: its class isn't translated (it extends `EditorListEntryWithIcon`) (xaero/hud/category/ui/entry/EditorListEntryWithIconAndText.java:40)
- [ ] `EditorListEntryWithIconAndText.getHoverColor(0)`: its class isn't translated (it extends `EditorListEntryWithIcon`) (xaero/hud/category/ui/entry/EditorListEntryWithIconAndText.java:44)
- [ ] `EditorListEntryWithIconAndText.render(13)`: its class isn't translated (it extends `EditorListEntryWithIcon`) (xaero/hud/category/ui/entry/EditorListEntryWithIconAndText.java:48)
- [ ] `EditorListEntryWithIconAndText.getMessage(0)`: its class isn't translated (it extends `EditorListEntryWithIcon`) (xaero/hud/category/ui/entry/EditorListEntryWithIconAndText.java:84)

### `EditorListEntryWithRootReference`: it extends `EditorListEntry`

- [ ] `EditorListEntryWithRootReference.EditorListEntryWithRootReference(8)`: its class isn't translated (it extends `EditorListEntry`) (xaero/hud/category/ui/entry/EditorListEntryWithRootReference.java:10)

### `EditorListEntryWrapper`: it extends `EditorListRootEntry`

- [ ] `EditorListEntryWrapper.EditorListEntryWrapper(6)`: its class isn't translated (it extends `EditorListRootEntry`) (xaero/hud/category/ui/entry/EditorListEntryWrapper.java:10)
- [ ] `EditorListEntryWrapper.EditorListEntryWrapper(7)`: its class isn't translated (it extends `EditorListRootEntry`) (xaero/hud/category/ui/entry/EditorListEntryWrapper.java:21)
- [ ] `EditorListEntryWrapper.getMessage(0)`: its class isn't translated (it extends `EditorListRootEntry`) (xaero/hud/category/ui/entry/EditorListEntryWrapper.java:35)

### `EditorListRootEntry`: it extends `EditorListEntry`

- [ ] `EditorListRootEntry.EditorListRootEntry(5)`: its class isn't translated (it extends `EditorListEntry`) (xaero/hud/category/ui/entry/EditorListRootEntry.java:15)
- [ ] `EditorListRootEntry.addHelpElement(1)`: its class isn't translated (it extends `EditorListEntry`) (xaero/hud/category/ui/entry/EditorListRootEntry.java:23)
- [ ] `EditorListRootEntry.render(13)`: its class isn't translated (it extends `EditorListEntry`) (xaero/hud/category/ui/entry/EditorListRootEntry.java:33)
- [ ] `EditorListRootEntry.withSubEntry(1)`: its class isn't translated (it extends `EditorListEntry`) (xaero/hud/category/ui/entry/EditorListRootEntry.java:95)
- [ ] `EditorListRootEntry.selectAction(0)`: its class isn't translated (it extends `EditorListEntry`) (xaero/hud/category/ui/entry/EditorListRootEntry.java:100)

### `EditorListRootEntry.CenteredEntryFactory`: it's an interface

- [ ] `EditorListRootEntry.CenteredEntryFactory.get(5)`: it has no body (xaero/hud/category/ui/entry/EditorListRootEntry.java:107)

### `EditorListRootEntryFactory`: it's an interface

- [ ] `EditorListRootEntryFactory.get(7)`: it has no body (xaero/hud/category/ui/entry/EditorListRootEntryFactory.java:7)

### `EditorListTextButtonEntry`: it extends `EditorListEntryWithRootReference`

- [ ] `EditorListTextButtonEntry.EditorListTextButtonEntry(11)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListTextButtonEntry.java:18)
- [ ] `EditorListTextButtonEntry.render(13)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListTextButtonEntry.java:48)
- [ ] `EditorListTextButtonEntry.selectAction(0)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListTextButtonEntry.java:84)
- [ ] `EditorListTextButtonEntry.getMessage(0)`: its class isn't translated (it extends `EditorListEntryWithRootReference`) (xaero/hud/category/ui/entry/EditorListTextButtonEntry.java:89)

### `EditorButton`: it extends Minecraft's `Plain`

- [ ] `EditorButton.EditorButton(6)`: its class isn't translated (it extends Minecraft's `Plain`) (xaero/hud/category/ui/entry/widget/EditorButton.java:18)
- [ ] `EditorButton.EditorButton(7)`: its class isn't translated (it extends Minecraft's `Plain`) (xaero/hud/category/ui/entry/widget/EditorButton.java:22)
- [ ] `EditorButton.updateMessage(0)`: its class isn't translated (it extends Minecraft's `Plain`) (xaero/hud/category/ui/entry/widget/EditorButton.java:39)
- [ ] `EditorButton.createNarrationMessage(0)`: its class isn't translated (it extends Minecraft's `Plain`) (xaero/hud/category/ui/entry/widget/EditorButton.java:43)

### `EditorButton.PressActionWithContext`: it implements Minecraft's `OnPress`

- [ ] `EditorButton.PressActionWithContext.onPress(1)`: its class isn't translated (it implements Minecraft's `OnPress`) (xaero/hud/category/ui/entry/widget/EditorButton.java:49)
- [ ] `EditorButton.PressActionWithContext.onPress(3)`: it has no body (xaero/hud/category/ui/entry/widget/EditorButton.java:53)

### `EditorButtonIteration`: it extends `EditorButton`

- [ ] `EditorButtonIteration.EditorButtonIteration(9)`: its class isn't translated (it extends `EditorButton`) (xaero/hud/category/ui/entry/widget/EditorButtonIteration.java:15)
- [ ] `EditorButtonIteration.toggle(0)`: its class isn't translated (it extends `EditorButton`) (xaero/hud/category/ui/entry/widget/EditorButtonIteration.java:33)
- [ ] `EditorButtonIteration.iterate(1)`: its class isn't translated (it extends `EditorButton`) (xaero/hud/category/ui/entry/widget/EditorButtonIteration.java:37)
- [ ] `EditorButtonIteration.putCurrentIndexInRange(0)`: its class isn't translated (it extends `EditorButton`) (xaero/hud/category/ui/entry/widget/EditorButtonIteration.java:44)
- [ ] `EditorButtonIteration.mouseScrolled(4)`: its class isn't translated (it extends `EditorButton`) (xaero/hud/category/ui/entry/widget/EditorButtonIteration.java:55)

### `EditorSlider`: it extends Minecraft's `AbstractOptionSliderButton`

- [ ] `EditorSlider.EditorSlider(7)`: its class isn't translated (it extends Minecraft's `AbstractOptionSliderButton`) (xaero/hud/category/ui/entry/widget/EditorSlider.java:21)
- [ ] `EditorSlider.keyPressed(1)`: its class isn't translated (it extends Minecraft's `AbstractOptionSliderButton`) (xaero/hud/category/ui/entry/widget/EditorSlider.java:40)
- [ ] `EditorSlider.manualOptionChange(1)`: its class isn't translated (it extends Minecraft's `AbstractOptionSliderButton`) (xaero/hud/category/ui/entry/widget/EditorSlider.java:52)
- [ ] `EditorSlider.createNarrationMessage(0)`: its class isn't translated (it extends Minecraft's `AbstractOptionSliderButton`) (xaero/hud/category/ui/entry/widget/EditorSlider.java:64)
- [ ] `EditorSlider.applyValue(0)`: its class isn't translated (it extends Minecraft's `AbstractOptionSliderButton`) (xaero/hud/category/ui/entry/widget/EditorSlider.java:69)
- [ ] `EditorSlider.updateMessage(0)`: its class isn't translated (it extends Minecraft's `AbstractOptionSliderButton`) (xaero/hud/category/ui/entry/widget/EditorSlider.java:74)
- [ ] `EditorSlider.toSliderValue(1)`: its class isn't translated (it extends Minecraft's `AbstractOptionSliderButton`) (xaero/hud/category/ui/entry/widget/EditorSlider.java:83)
- [ ] `EditorSlider.toValue(1)`: its class isn't translated (it extends Minecraft's `AbstractOptionSliderButton`) (xaero/hud/category/ui/entry/widget/EditorSlider.java:87)
- [ ] `EditorSlider.clamp(1)`: its class isn't translated (it extends Minecraft's `AbstractOptionSliderButton`) (xaero/hud/category/ui/entry/widget/EditorSlider.java:91)

### `EditorTextField`: it extends Minecraft's `EditBox`

- [ ] `EditorTextField.EditorTextField(11)`: its class isn't translated (it extends Minecraft's `EditBox`) (xaero/hud/category/ui/entry/widget/EditorTextField.java:21)
- [ ] `EditorTextField.updateColorForValue(1)`: its class isn't translated (it extends Minecraft's `EditBox`) (xaero/hud/category/ui/entry/widget/EditorTextField.java:51)
- [ ] `EditorTextField.setCursorPosition(1)`: its class isn't translated (it extends Minecraft's `EditBox`) (xaero/hud/category/ui/entry/widget/EditorTextField.java:55)
- [ ] `EditorTextField.setHighlightPos(1)`: its class isn't translated (it extends Minecraft's `EditBox`) (xaero/hud/category/ui/entry/widget/EditorTextField.java:59)
- [ ] `EditorTextField.createNarrationMessage(0)`: its class isn't translated (it extends Minecraft's `EditBox`) (xaero/hud/category/ui/entry/widget/EditorTextField.java:64)
- [ ] `EditorTextField.extractWidgetRenderState(4)`: its class isn't translated (it extends Minecraft's `EditBox`) (xaero/hud/category/ui/entry/widget/EditorTextField.java:69)

### `EditorTextField.UpdatedValueConsumer`: it's an interface

- [ ] `EditorTextField.UpdatedValueConsumer.accept(4)`: it has no body (xaero/hud/category/ui/entry/widget/EditorTextField.java:82)

### `GuiCategoryEditor`: it's generic

- [x] `GuiCategoryEditor.READ_ONLY_COMPONENT` in game_side.rs (xaero/hud/category/ui/GuiCategoryEditor.java:36)
- [x] `GuiCategoryEditor.FRAME_TOP_SIZE` (xaero/hud/category/ui/GuiCategoryEditor.java:37)
- [x] `GuiCategoryEditor.FRAME_BOTTOM_SIZE` (xaero/hud/category/ui/GuiCategoryEditor.java:38)
- [x] `GuiCategoryEditor.ROW_HEIGHT` (xaero/hud/category/ui/GuiCategoryEditor.java:39)
- [x] `GuiCategoryEditor.ROW_WIDTH` (xaero/hud/category/ui/GuiCategoryEditor.java:40)
- [ ] `GuiCategoryEditor.GuiCategoryEditor(6)`: its class isn't translated (it's generic) (xaero/hud/category/ui/GuiCategoryEditor.java:49)
- [ ] `GuiCategoryEditor.constructEditorData(1)`: it has no body (xaero/hud/category/ui/GuiCategoryEditor.java:64)
- [ ] `GuiCategoryEditor.constructDefaultData(1)`: it has no body (xaero/hud/category/ui/GuiCategoryEditor.java:66)
- [ ] `GuiCategoryEditor.onConfigConfirmed(1)`: it has no body (xaero/hud/category/ui/GuiCategoryEditor.java:68)
- [ ] `GuiCategoryEditor.init(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/GuiCategoryEditor.java:70)
- [ ] `GuiCategoryEditor.confirm(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/GuiCategoryEditor.java:114)
- [ ] `GuiCategoryEditor.onExit(1)`: its body couldn't be read: an anonymous class (xaero/hud/category/ui/GuiCategoryEditor.java:121)
- [ ] `GuiCategoryEditor.cancel(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/GuiCategoryEditor.java:139)
- [ ] `GuiCategoryEditor.extractBackground(4)`: its class isn't translated (it's generic) (xaero/hud/category/ui/GuiCategoryEditor.java:143)
- [ ] `GuiCategoryEditor.extractRenderState(4)`: its class isn't translated (it's generic) (xaero/hud/category/ui/GuiCategoryEditor.java:149)
- [ ] `GuiCategoryEditor.keyPressed(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/GuiCategoryEditor.java:168)
- [ ] `GuiCategoryEditor.tick(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/GuiCategoryEditor.java:172)
- [ ] `GuiCategoryEditor.getRowList(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/GuiCategoryEditor.java:177)

### `GuiCategoryEditor.SettingRowList`: it extends Minecraft's `ObjectSelectionList`

- [x] `GuiCategoryEditor.SettingRowList.USAGE_NARRATION` in game_side.rs (xaero/hud/category/ui/GuiCategoryEditor.java:187)
- [x] `GuiCategoryEditor.SettingRowList.LEFT_RIGHT_USAGE` in game_side.rs (xaero/hud/category/ui/GuiCategoryEditor.java:188)
- [ ] `GuiCategoryEditor.SettingRowList.SettingRowList(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:190)
- [ ] `GuiCategoryEditor.SettingRowList.entriesCanBeSelected(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:197)
- [ ] `GuiCategoryEditor.SettingRowList.hasCut(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:201)
- [ ] `GuiCategoryEditor.SettingRowList.getCut(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:214)
- [ ] `GuiCategoryEditor.SettingRowList.isCut(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:218)
- [ ] `GuiCategoryEditor.SettingRowList.setCutCategory(2)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:222)
- [ ] `GuiCategoryEditor.SettingRowList.pasteTo(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:227)
- [ ] `GuiCategoryEditor.SettingRowList.isFocused(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:242)
- [ ] `GuiCategoryEditor.SettingRowList.setLastExpandedData(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:246)
- [ ] `GuiCategoryEditor.SettingRowList.restoreScrollAfterUpdate(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:250)
- [ ] `GuiCategoryEditor.SettingRowList.updateEntries(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:254)
- [ ] `GuiCategoryEditor.SettingRowList.addEntriesForExpanded(2)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:269)
- [ ] `GuiCategoryEditor.SettingRowList.mouseClicked(2)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:316)
- [ ] `GuiCategoryEditor.SettingRowList.mouseMoved(2)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:324)
- [ ] `GuiCategoryEditor.SettingRowList.keyReleased(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:332)
- [ ] `GuiCategoryEditor.SettingRowList.charTyped(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:336)
- [ ] `GuiCategoryEditor.SettingRowList.tick(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:347)
- [ ] `GuiCategoryEditor.SettingRowList.confirmSelection(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:353)
- [ ] `GuiCategoryEditor.SettingRowList.setFocused(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:363)
- [ ] `GuiCategoryEditor.SettingRowList.setSelected(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:387)
- [ ] `GuiCategoryEditor.SettingRowList.getRowWidth(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:391)
- [ ] `GuiCategoryEditor.SettingRowList.scrollBarX(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:395)
- [ ] `GuiCategoryEditor.SettingRowList.narrateSelection(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:399)
- [ ] `GuiCategoryEditor.SettingRowList.updateWidgetNarration(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:403)
- [ ] `GuiCategoryEditor.SettingRowList.extractWidgetRenderState(4)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:410)
- [ ] `GuiCategoryEditor.SettingRowList.nextFocusPath(1)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:415)
- [ ] `GuiCategoryEditor.SettingRowList.getDataConverter(0)`: its class isn't translated (it extends Minecraft's `ObjectSelectionList`) (xaero/hud/category/ui/GuiCategoryEditor.java:424)

### `GuiCategoryEditor.SettingRowList.Entry`: it extends `Entry`

- [ ] `GuiCategoryEditor.SettingRowList.Entry.Entry(2)`: its class isn't translated (it extends `Entry`) (xaero/hud/category/ui/GuiCategoryEditor.java:434)
- [ ] `GuiCategoryEditor.SettingRowList.Entry.extractContent(5)`: its class isn't translated (it extends `Entry`) (xaero/hud/category/ui/GuiCategoryEditor.java:439)
- [ ] `GuiCategoryEditor.SettingRowList.Entry.mouseClicked(2)`: its class isn't translated (it extends `Entry`) (xaero/hud/category/ui/GuiCategoryEditor.java:472)
- [ ] `GuiCategoryEditor.SettingRowList.Entry.mouseReleased(1)`: its class isn't translated (it extends `Entry`) (xaero/hud/category/ui/GuiCategoryEditor.java:480)
- [ ] `GuiCategoryEditor.SettingRowList.Entry.mouseScrolled(4)`: its class isn't translated (it extends `Entry`) (xaero/hud/category/ui/GuiCategoryEditor.java:487)
- [ ] `GuiCategoryEditor.SettingRowList.Entry.mouseDragged(3)`: its class isn't translated (it extends `Entry`) (xaero/hud/category/ui/GuiCategoryEditor.java:493)
- [ ] `GuiCategoryEditor.SettingRowList.Entry.keyPressed(1)`: its class isn't translated (it extends `Entry`) (xaero/hud/category/ui/GuiCategoryEditor.java:501)
- [ ] `GuiCategoryEditor.SettingRowList.Entry.keyReleased(1)`: its class isn't translated (it extends `Entry`) (xaero/hud/category/ui/GuiCategoryEditor.java:505)
- [ ] `GuiCategoryEditor.SettingRowList.Entry.charTyped(1)`: its class isn't translated (it extends `Entry`) (xaero/hud/category/ui/GuiCategoryEditor.java:509)
- [ ] `GuiCategoryEditor.SettingRowList.Entry.setFocused(1)`: its class isn't translated (it extends `Entry`) (xaero/hud/category/ui/GuiCategoryEditor.java:513)
- [ ] `GuiCategoryEditor.SettingRowList.Entry.tick(0)`: its class isn't translated (it extends `Entry`) (xaero/hud/category/ui/GuiCategoryEditor.java:518)
- [ ] `GuiCategoryEditor.SettingRowList.Entry.getNarration(0)`: its class isn't translated (it extends `Entry`) (xaero/hud/category/ui/GuiCategoryEditor.java:522)

### `EditorAdderNode`: it extends `EditorNode`

- [ ] `EditorAdderNode.EditorAdderNode(6)`: its class isn't translated (it extends `EditorNode`) (xaero/hud/category/ui/node/EditorAdderNode.java:23)
- [ ] `EditorAdderNode.isConfirmed(0)`: its class isn't translated (it extends `EditorNode`) (xaero/hud/category/ui/node/EditorAdderNode.java:37)
- [ ] `EditorAdderNode.setExpanded(1)`: its class isn't translated (it extends `EditorNode`) (xaero/hud/category/ui/node/EditorAdderNode.java:41)
- [ ] `EditorAdderNode.reset(0)`: its class isn't translated (it extends `EditorNode`) (xaero/hud/category/ui/node/EditorAdderNode.java:49)
- [ ] `EditorAdderNode.getNameField(0)`: its class isn't translated (it extends `EditorNode`) (xaero/hud/category/ui/node/EditorAdderNode.java:54)
- [ ] `EditorAdderNode.getDisplayName(0)`: its class isn't translated (it extends `EditorNode`) (xaero/hud/category/ui/node/EditorAdderNode.java:58)
- [ ] `EditorAdderNode.getSubNodes(0)`: its class isn't translated (it extends `EditorNode`) (xaero/hud/category/ui/node/EditorAdderNode.java:63)

### `EditorAdderNode.Builder`: it extends `Builder`

- [ ] `EditorAdderNode.Builder.Builder(1)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/EditorAdderNode.java:73)
- [ ] `EditorAdderNode.Builder.setDefault(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/EditorAdderNode.java:78)
- [ ] `EditorAdderNode.Builder.getCenteredEntryFactory(4)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/EditorAdderNode.java:93)
- [ ] `EditorAdderNode.Builder.setDisplayName(1)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/EditorAdderNode.java:107)
- [ ] `EditorAdderNode.Builder.getNameFieldBuilder(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/EditorAdderNode.java:112)
- [ ] `EditorAdderNode.Builder.getConfirmButtonBuilder(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/EditorAdderNode.java:116)
- [ ] `EditorAdderNode.Builder.begin(1)`: it uses Builder, which isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/EditorAdderNode.java:120)
- [ ] `EditorAdderNode.Builder.build(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/EditorAdderNode.java:124)
- [ ] `EditorAdderNode.Builder.buildInternally(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/EditorAdderNode.java:132)

### `EditorCategoryNode`: it's generic

- [ ] `EditorCategoryNode.EditorCategoryNode(8)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:35)
- [ ] `EditorCategoryNode.getSettingsNode(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:52)
- [ ] `EditorCategoryNode.getSubCategories(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:56)
- [ ] `EditorCategoryNode.getName(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:60)
- [ ] `EditorCategoryNode.getDisplayName(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:64)
- [ ] `EditorCategoryNode.getAdderHandler(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:69)
- [ ] `EditorCategoryNode.getDeletionHandler(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:79)
- [ ] `EditorCategoryNode.getMoveAction(3)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:92)
- [ ] `EditorCategoryNode.getDuplicateAction(2)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:115)
- [ ] `EditorCategoryNode.getCutAction(2)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:144)
- [ ] `EditorCategoryNode.getPasteAction(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:153)
- [ ] `EditorCategoryNode.getSubNodes(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:161)
- [ ] `EditorCategoryNode.removeProtectionRecursive(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:172)

### `EditorCategoryNode.Builder`: it's generic

- [ ] `EditorCategoryNode.Builder.Builder(2)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:191)
- [ ] `EditorCategoryNode.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:203)
- [ ] `EditorCategoryNode.Builder.mainEntryFactory(7)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:220)
- [ ] `EditorCategoryNode.Builder.setNewCategorySupplier(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:242)
- [ ] `EditorCategoryNode.Builder.setSubIndex(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:247)
- [ ] `EditorCategoryNode.Builder.setName(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:252)
- [ ] `EditorCategoryNode.Builder.getSettingDataBuilder(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:257)
- [ ] `EditorCategoryNode.Builder.addSubCategoryBuilder(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:261)
- [ ] `EditorCategoryNode.Builder.buildSubCategories(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:267)
- [ ] `EditorCategoryNode.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorCategoryNode.java:271)

### `EditorFilterCategoryNode`: it's generic

- [ ] `EditorFilterCategoryNode.EditorFilterCategoryNode(8)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterCategoryNode.java:14)

### `EditorFilterCategoryNode.Builder`: it's generic

- [ ] `EditorFilterCategoryNode.Builder.Builder(2)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterCategoryNode.java:32)
- [ ] `EditorFilterCategoryNode.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterCategoryNode.java:36)
- [ ] `EditorFilterCategoryNode.Builder.setListRuleTypePrefixSeparator(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterCategoryNode.java:43)
- [ ] `EditorFilterCategoryNode.Builder.setInputRuleTypeStringValidator(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterCategoryNode.java:48)
- [ ] `EditorFilterCategoryNode.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterCategoryNode.java:53)

### `EditorFilterSettingsNode`: it's generic

- [ ] `EditorFilterSettingsNode.EditorFilterSettingsNode(14)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterSettingsNode.java:29)
- [ ] `EditorFilterSettingsNode.getBaseRule(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterSettingsNode.java:53)
- [ ] `EditorFilterSettingsNode.getIncludeList(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterSettingsNode.java:57)
- [ ] `EditorFilterSettingsNode.getExcludeList(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterSettingsNode.java:61)
- [ ] `EditorFilterSettingsNode.getSubNodes(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterSettingsNode.java:65)

### `EditorFilterSettingsNode.Builder`: it's generic

- [ ] `EditorFilterSettingsNode.Builder.Builder(5)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterSettingsNode.java:83)
- [ ] `EditorFilterSettingsNode.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterSettingsNode.java:97)
- [ ] `EditorFilterSettingsNode.Builder.setBaseRule(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterSettingsNode.java:117)
- [ ] `EditorFilterSettingsNode.Builder.setListRuleTypePrefixSeparator(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterSettingsNode.java:121)
- [ ] `EditorFilterSettingsNode.Builder.setInputRuleTypeStringValidator(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterSettingsNode.java:126)
- [ ] `EditorFilterSettingsNode.Builder.getIncludeListBuilder(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterSettingsNode.java:131)
- [ ] `EditorFilterSettingsNode.Builder.getExcludeListBuilder(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterSettingsNode.java:135)
- [ ] `EditorFilterSettingsNode.Builder.buildIncludeList(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterSettingsNode.java:139)
- [ ] `EditorFilterSettingsNode.Builder.buildExcludeList(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterSettingsNode.java:143)
- [ ] `EditorFilterSettingsNode.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterSettingsNode.java:147)

### `EditorFilterSettingsNode.FinalBuilder`: it's generic

- [ ] `EditorFilterSettingsNode.FinalBuilder.FinalBuilder(5)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterSettingsNode.java:164)
- [ ] `EditorFilterSettingsNode.FinalBuilder.buildInternally(2)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorFilterSettingsNode.java:174)
- [ ] `EditorFilterSettingsNode.FinalBuilder.begin(5)`: it's generic (xaero/hud/category/ui/node/EditorFilterSettingsNode.java:195)

### `EditorNode`: it uses EditorListRootEntryFactory, which isn't translated (it's an interface)

- [ ] `EditorNode.EditorNode(3)`: its class isn't translated (it uses EditorListRootEntryFactory, which isn't translated (it's an interface)) (xaero/hud/category/ui/node/EditorNode.java:24)
- [ ] `EditorNode.isMovable(0)`: its class isn't translated (it uses EditorListRootEntryFactory, which isn't translated (it's an interface)) (xaero/hud/category/ui/node/EditorNode.java:30)
- [ ] `EditorNode.isExpanded(0)`: its class isn't translated (it uses EditorListRootEntryFactory, which isn't translated (it's an interface)) (xaero/hud/category/ui/node/EditorNode.java:34)
- [ ] `EditorNode.setExpanded(1)`: its class isn't translated (it uses EditorListRootEntryFactory, which isn't translated (it's an interface)) (xaero/hud/category/ui/node/EditorNode.java:38)
- [ ] `EditorNode.getListEntryFactory(0)`: its class isn't translated (it uses EditorListRootEntryFactory, which isn't translated (it's an interface)) (xaero/hud/category/ui/node/EditorNode.java:53)
- [ ] `EditorNode.getTooltipSupplier(1)`: its class isn't translated (it uses EditorListRootEntryFactory, which isn't translated (it's an interface)) (xaero/hud/category/ui/node/EditorNode.java:57)
- [ ] `EditorNode.getExpandAction(1)`: its class isn't translated (it uses EditorListRootEntryFactory, which isn't translated (it's an interface)) (xaero/hud/category/ui/node/EditorNode.java:61)
- [ ] `EditorNode.getDisplayName(0)`: it has no body (xaero/hud/category/ui/node/EditorNode.java:81)
- [ ] `EditorNode.getSubNodes(0)`: it has no body (xaero/hud/category/ui/node/EditorNode.java:83)

### `EditorNode.Builder`: it's generic

- [ ] `EditorNode.Builder.Builder(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorNode.java:91)
- [ ] `EditorNode.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorNode.java:94)
- [ ] `EditorNode.Builder.mainEntryFactory(7)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorNode.java:101)
- [ ] `EditorNode.Builder.getCenteredEntryFactory(4)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorNode.java:113)
- [ ] `EditorNode.Builder.setMovable(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorNode.java:121)
- [ ] `EditorNode.Builder.setListEntryFactory(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorNode.java:126)
- [ ] `EditorNode.Builder.setTooltipSupplier(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorNode.java:131)
- [ ] `EditorNode.Builder.setTooltipInfoSupplier(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorNode.java:136)
- [ ] `EditorNode.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorNode.java:143)
- [ ] `EditorNode.Builder.buildInternally(0)`: it has no body (xaero/hud/category/ui/node/EditorNode.java:151)

### `EditorSettingsNode`: it's generic

- [ ] `EditorSettingsNode.EditorSettingsNode(11)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSettingsNode.java:41)
- [ ] `EditorSettingsNode.getSettings(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSettingsNode.java:65)
- [ ] `EditorSettingsNode.getSettingData(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSettingsNode.java:69)
- [ ] `EditorSettingsNode.isRootSettings(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSettingsNode.java:73)
- [ ] `EditorSettingsNode.isToBeDeleted(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSettingsNode.java:77)
- [ ] `EditorSettingsNode.setToBeDeleted(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSettingsNode.java:81)
- [ ] `EditorSettingsNode.getProtection(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSettingsNode.java:85)
- [ ] `EditorSettingsNode.setProtected(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSettingsNode.java:89)
- [ ] `EditorSettingsNode.getSubNodes(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSettingsNode.java:93)
- [ ] `EditorSettingsNode.getDisplayName(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSettingsNode.java:106)
- [ ] `EditorSettingsNode.getNameOption(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSettingsNode.java:111)

### `EditorSettingsNode.Builder`: it's generic

- [ ] `EditorSettingsNode.Builder.Builder(4)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSettingsNode.java:128)
- [ ] `EditorSettingsNode.Builder.addSetting(2)`: it's generic (xaero/hud/category/ui/node/EditorSettingsNode.java:147)
- [ ] `EditorSettingsNode.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSettingsNode.java:153)
- [ ] `EditorSettingsNode.Builder.getCenteredEntryFactory(4)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSettingsNode.java:226)
- [ ] `EditorSettingsNode.Builder.setSettingValue(2)`: it's generic (xaero/hud/category/ui/node/EditorSettingsNode.java:236)
- [ ] `EditorSettingsNode.Builder.setRootSettings(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSettingsNode.java:242)
- [ ] `EditorSettingsNode.Builder.setProtection(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSettingsNode.java:247)
- [ ] `EditorSettingsNode.Builder.getNameOptionBuilder(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSettingsNode.java:252)
- [ ] `EditorSettingsNode.Builder.getDeleteButtonBuilder(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSettingsNode.java:256)
- [ ] `EditorSettingsNode.Builder.buildInternally(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSettingsNode.java:260)
- [ ] `EditorSettingsNode.Builder.buildInternally(2)`: it has no body (xaero/hud/category/ui/node/EditorSettingsNode.java:284)

### `EditorSettingsNode.FinalBuilder`: it extends `Builder`

- [ ] `EditorSettingsNode.FinalBuilder.FinalBuilder(4)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/EditorSettingsNode.java:288)
- [ ] `EditorSettingsNode.FinalBuilder.buildInternally(2)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/EditorSettingsNode.java:297)

### `EditorSimpleDeletableWrapperNode`: it's generic

- [ ] `EditorSimpleDeletableWrapperNode.EditorSimpleDeletableWrapperNode(5)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleDeletableWrapperNode.java:14)
- [ ] `EditorSimpleDeletableWrapperNode.getDeletionCallback(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleDeletableWrapperNode.java:25)

### `EditorSimpleDeletableWrapperNode.Builder`: it's generic

- [ ] `EditorSimpleDeletableWrapperNode.Builder.Builder(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleDeletableWrapperNode.java:32)
- [ ] `EditorSimpleDeletableWrapperNode.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleDeletableWrapperNode.java:35)
- [ ] `EditorSimpleDeletableWrapperNode.Builder.mainEntryFactory(7)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleDeletableWrapperNode.java:41)
- [ ] `EditorSimpleDeletableWrapperNode.Builder.setDeletionCallback(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleDeletableWrapperNode.java:57)
- [ ] `EditorSimpleDeletableWrapperNode.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleDeletableWrapperNode.java:62)
- [ ] `EditorSimpleDeletableWrapperNode.Builder.buildInternally(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleDeletableWrapperNode.java:70)
- [ ] `EditorSimpleDeletableWrapperNode.Builder.begin(0)`: it's generic (xaero/hud/category/ui/node/EditorSimpleDeletableWrapperNode.java:75)

### `EditorSimpleDeletableWrapperNode.DeletionCallback`: it's an interface

- [ ] `EditorSimpleDeletableWrapperNode.DeletionCallback.delete(3)`: it has no body (xaero/hud/category/ui/node/EditorSimpleDeletableWrapperNode.java:81)

### `EditorSimpleWrapperNode`: it's generic

- [ ] `EditorSimpleWrapperNode.EditorSimpleWrapperNode(4)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleWrapperNode.java:13)
- [ ] `EditorSimpleWrapperNode.getElement(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleWrapperNode.java:20)
- [ ] `EditorSimpleWrapperNode.setElement(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleWrapperNode.java:24)
- [ ] `EditorSimpleWrapperNode.getSubNodes(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleWrapperNode.java:28)
- [ ] `EditorSimpleWrapperNode.getDisplayName(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleWrapperNode.java:33)
- [ ] `EditorSimpleWrapperNode.equals(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleWrapperNode.java:38)
- [ ] `EditorSimpleWrapperNode.compareTo(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleWrapperNode.java:43)

### `EditorSimpleWrapperNode.Builder`: it's generic

- [ ] `EditorSimpleWrapperNode.Builder.Builder(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleWrapperNode.java:56)
- [ ] `EditorSimpleWrapperNode.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleWrapperNode.java:59)
- [ ] `EditorSimpleWrapperNode.Builder.setElement(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleWrapperNode.java:65)
- [ ] `EditorSimpleWrapperNode.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleWrapperNode.java:70)

### `EditorSimpleWrapperNode.FinalBuilder`: it's generic

- [ ] `EditorSimpleWrapperNode.FinalBuilder.begin(0)`: it's generic (xaero/hud/category/ui/node/EditorSimpleWrapperNode.java:80)
- [ ] `EditorSimpleWrapperNode.FinalBuilder.buildInternally(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/EditorSimpleWrapperNode.java:84)

### `EditorCompactBooleanOptionsNode`: it extends `EditorCompactOptionsNode`

- [ ] `EditorCompactBooleanOptionsNode.EditorCompactBooleanOptionsNode(9)`: its class isn't translated (it extends `EditorCompactOptionsNode`) (xaero/hud/category/ui/node/options/EditorCompactBooleanOptionsNode.java:13)
- [ ] `EditorCompactBooleanOptionsNode.getIndexReader(0)`: its class isn't translated (it extends `EditorCompactOptionsNode`) (xaero/hud/category/ui/node/options/EditorCompactBooleanOptionsNode.java:30)

### `EditorCompactBooleanOptionsNode.Builder`: it extends `Builder`

- [ ] `EditorCompactBooleanOptionsNode.Builder.Builder(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/EditorCompactBooleanOptionsNode.java:43)
- [ ] `EditorCompactBooleanOptionsNode.Builder.setDefault(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/EditorCompactBooleanOptionsNode.java:46)
- [ ] `EditorCompactBooleanOptionsNode.Builder.getTrueOptionBuilder(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/EditorCompactBooleanOptionsNode.java:54)
- [ ] `EditorCompactBooleanOptionsNode.Builder.getFalseOptionBuilder(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/EditorCompactBooleanOptionsNode.java:58)
- [ ] `EditorCompactBooleanOptionsNode.Builder.build(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/EditorCompactBooleanOptionsNode.java:62)
- [ ] `EditorCompactBooleanOptionsNode.Builder.buildInternally(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/EditorCompactBooleanOptionsNode.java:72)
- [ ] `EditorCompactBooleanOptionsNode.Builder.begin(0)`: it uses Builder, which isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/EditorCompactBooleanOptionsNode.java:88)

### `EditorCompactOptionsNode`: it's generic

- [ ] `EditorCompactOptionsNode.EditorCompactOptionsNode(7)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorCompactOptionsNode.java:23)
- [ ] `EditorCompactOptionsNode.getCurrentIndex(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorCompactOptionsNode.java:37)
- [ ] `EditorCompactOptionsNode.getOptionCount(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorCompactOptionsNode.java:41)
- [ ] `EditorCompactOptionsNode.getUpdatedIndexConsumer(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorCompactOptionsNode.java:45)
- [ ] `EditorCompactOptionsNode.getIndexReader(0)`: it has no body (xaero/hud/category/ui/node/options/EditorCompactOptionsNode.java:56)
- [ ] `EditorCompactOptionsNode.getSubNodes(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorCompactOptionsNode.java:58)

### `EditorCompactOptionsNode.Builder`: it's generic

- [ ] `EditorCompactOptionsNode.Builder.Builder(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorCompactOptionsNode.java:66)
- [ ] `EditorCompactOptionsNode.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorCompactOptionsNode.java:69)
- [ ] `EditorCompactOptionsNode.Builder.setSlider(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorCompactOptionsNode.java:75)
- [ ] `EditorCompactOptionsNode.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorCompactOptionsNode.java:80)
- [ ] `EditorCompactOptionsNode.Builder.getMainEntryFactory(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorCompactOptionsNode.java:86)
- [ ] `EditorCompactOptionsNode.Builder.getCenteredEntryFactory(5)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorCompactOptionsNode.java:92)
- [ ] `EditorCompactOptionsNode.Builder.getEntryWidget(5)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorCompactOptionsNode.java:103)
- [ ] `EditorCompactOptionsNode.Builder.buildInternally(0)`: it has no body (xaero/hud/category/ui/node/options/EditorCompactOptionsNode.java:129)

### `EditorExpandingOptionsNode`: it's generic

- [ ] `EditorExpandingOptionsNode.EditorExpandingOptionsNode(7)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorExpandingOptionsNode.java:18)
- [ ] `EditorExpandingOptionsNode.onSelected(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorExpandingOptionsNode.java:32)
- [ ] `EditorExpandingOptionsNode.getSubNodes(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorExpandingOptionsNode.java:38)
- [ ] `EditorExpandingOptionsNode.getDisplayName(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorExpandingOptionsNode.java:43)

### `EditorExpandingOptionsNode.Builder`: it's generic

- [ ] `EditorExpandingOptionsNode.Builder.Builder(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorExpandingOptionsNode.java:52)
- [ ] `EditorExpandingOptionsNode.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorExpandingOptionsNode.java:57)
- [ ] `EditorExpandingOptionsNode.Builder.getCenteredEntryFactory(4)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorExpandingOptionsNode.java:63)
- [ ] `EditorExpandingOptionsNode.Builder.addOptionBuilderFor(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorExpandingOptionsNode.java:81)
- [ ] `EditorExpandingOptionsNode.Builder.addOptionBuilder(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorExpandingOptionsNode.java:86)
- [ ] `EditorExpandingOptionsNode.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorExpandingOptionsNode.java:91)
- [ ] `EditorExpandingOptionsNode.Builder.buildInternally(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorExpandingOptionsNode.java:99)
- [ ] `EditorExpandingOptionsNode.Builder.buildInternally(2)`: it has no body (xaero/hud/category/ui/node/options/EditorExpandingOptionsNode.java:121)

### `EditorExpandingOptionsNode.FinalBuilder`: it's generic

- [ ] `EditorExpandingOptionsNode.FinalBuilder.FinalBuilder(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorExpandingOptionsNode.java:125)
- [ ] `EditorExpandingOptionsNode.FinalBuilder.begin(1)`: it's generic (xaero/hud/category/ui/node/options/EditorExpandingOptionsNode.java:129)
- [ ] `EditorExpandingOptionsNode.FinalBuilder.buildInternally(2)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorExpandingOptionsNode.java:133)

### `EditorOptionNode`: it's generic

- [ ] `EditorOptionNode.EditorOptionNode(5)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionNode.java:17)
- [ ] `EditorOptionNode.getValue(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionNode.java:25)
- [ ] `EditorOptionNode.getDisplayName(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionNode.java:29)
- [ ] `EditorOptionNode.getSubNodes(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionNode.java:34)

### `EditorOptionNode.Builder`: it's generic

- [ ] `EditorOptionNode.Builder.Builder(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionNode.java:43)
- [ ] `EditorOptionNode.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionNode.java:46)
- [ ] `EditorOptionNode.Builder.getCenteredEntryFactory(4)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionNode.java:53)
- [ ] `EditorOptionNode.Builder.setValue(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionNode.java:63)
- [ ] `EditorOptionNode.Builder.setDisplayName(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionNode.java:68)
- [ ] `EditorOptionNode.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionNode.java:73)
- [ ] `EditorOptionNode.Builder.begin(0)`: it's generic (xaero/hud/category/ui/node/options/EditorOptionNode.java:81)
- [ ] `EditorOptionNode.Builder.buildInternally(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionNode.java:85)

### `EditorOptionsNode`: it's generic

- [ ] `EditorOptionsNode.EditorOptionsNode(5)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionsNode.java:17)
- [ ] `EditorOptionsNode.getCurrentValue(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionsNode.java:29)
- [ ] `EditorOptionsNode.setCurrentValue(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionsNode.java:33)
- [ ] `EditorOptionsNode.getMessageSupplier(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionsNode.java:37)
- [ ] `EditorOptionsNode.getIsActiveSupplier(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionsNode.java:47)
- [ ] `EditorOptionsNode.getDisplayName(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionsNode.java:51)

### `EditorOptionsNode.Builder`: it's generic

- [ ] `EditorOptionsNode.Builder.Builder(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionsNode.java:62)
- [ ] `EditorOptionsNode.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionsNode.java:65)
- [ ] `EditorOptionsNode.Builder.setCurrentValue(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionsNode.java:73)
- [ ] `EditorOptionsNode.Builder.setDisplayName(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionsNode.java:78)
- [ ] `EditorOptionsNode.Builder.setIsActiveSupplier(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionsNode.java:83)
- [ ] `EditorOptionsNode.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/EditorOptionsNode.java:88)
- [ ] `EditorOptionsNode.Builder.buildInternally(0)`: it has no body (xaero/hud/category/ui/node/options/EditorOptionsNode.java:96)

### `EditorOptionsNode.IOptionsNodeIsActiveSupplier`: it's an interface

- [ ] `EditorOptionsNode.IOptionsNodeIsActiveSupplier.get(2)`: it has no body (xaero/hud/category/ui/node/options/EditorOptionsNode.java:101)

### `EditorSimpleButtonNode`: it extends `EditorNode` (xaero.hud.category.ui.node)

- [ ] `EditorSimpleButtonNode.EditorSimpleButtonNode(7)`: its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)) (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java:22)
- [ ] `EditorSimpleButtonNode.getMessageSupplier(2)`: its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)) (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java:38)
- [ ] `EditorSimpleButtonNode.getIsActiveSupplier(3)`: its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)) (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java:42)
- [ ] `EditorSimpleButtonNode.getDisplayName(0)`: its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)) (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java:46)
- [ ] `EditorSimpleButtonNode.getPressAction(0)`: its body couldn't be read: an anonymous class (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java:51)
- [ ] `EditorSimpleButtonNode.getSubNodes(0)`: its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)) (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java:66)

### `EditorSimpleButtonNode.Builder`: it extends `Builder`

- [ ] `EditorSimpleButtonNode.Builder.Builder(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java:77)
- [ ] `EditorSimpleButtonNode.Builder.setDefault(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java:80)
- [ ] `EditorSimpleButtonNode.Builder.getCenteredEntryFactory(4)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java:89)
- [ ] `EditorSimpleButtonNode.Builder.setDisplayName(1)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java:102)
- [ ] `EditorSimpleButtonNode.Builder.setCallback(1)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java:107)
- [ ] `EditorSimpleButtonNode.Builder.setMessageSupplier(1)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java:112)
- [ ] `EditorSimpleButtonNode.Builder.setIsActiveSupplier(1)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java:117)
- [ ] `EditorSimpleButtonNode.Builder.build(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java:122)
- [ ] `EditorSimpleButtonNode.Builder.buildInternally(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java:130)
- [ ] `EditorSimpleButtonNode.Builder.begin(0)`: it uses Builder, which isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java:137)

### `EditorSimpleButtonNode.ISimpleButtonCallback`: it's an interface

- [ ] `EditorSimpleButtonNode.ISimpleButtonCallback.onButtonPress(3)`: it has no body (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java:144)

### `EditorSimpleButtonNode.ISimpleButtonIsActiveSupplier`: it's an interface

- [ ] `EditorSimpleButtonNode.ISimpleButtonIsActiveSupplier.get(3)`: it has no body (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java:149)

### `EditorSimpleButtonNode.ISimpleButtonMessageSupplier`: it's an interface

- [ ] `EditorSimpleButtonNode.ISimpleButtonMessageSupplier.get(2)`: it has no body (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java:154)

### `EditorCompactListOptionsNode`: it's generic

- [ ] `EditorCompactListOptionsNode.EditorCompactListOptionsNode(7)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/list/EditorCompactListOptionsNode.java:18)
- [ ] `EditorCompactListOptionsNode.getIndexReader(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/list/EditorCompactListOptionsNode.java:32)

### `EditorCompactListOptionsNode.Builder`: it's generic

- [ ] `EditorCompactListOptionsNode.Builder.Builder(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/list/EditorCompactListOptionsNode.java:45)
- [ ] `EditorCompactListOptionsNode.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/list/EditorCompactListOptionsNode.java:50)
- [ ] `EditorCompactListOptionsNode.Builder.addOptionBuilder(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/list/EditorCompactListOptionsNode.java:55)
- [ ] `EditorCompactListOptionsNode.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/list/EditorCompactListOptionsNode.java:60)
- [ ] `EditorCompactListOptionsNode.Builder.buildInternally(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/list/EditorCompactListOptionsNode.java:68)
- [ ] `EditorCompactListOptionsNode.Builder.begin(1)`: it's generic (xaero/hud/category/ui/node/options/list/EditorCompactListOptionsNode.java:91)

### `EditorCompactRangeNode`: it's generic

- [ ] `EditorCompactRangeNode.EditorCompactRangeNode(12)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java:20)
- [ ] `EditorCompactRangeNode.setCurrentValue(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java:43)
- [ ] `EditorCompactRangeNode.getCurrentRangeValue(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java:50)
- [ ] `EditorCompactRangeNode.getIndexReader(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java:54)

### `EditorCompactRangeNode.Builder`: it's generic

- [ ] `EditorCompactRangeNode.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java:83)
- [ ] `EditorCompactRangeNode.Builder.setCurrentRangeValue(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java:94)
- [ ] `EditorCompactRangeNode.Builder.setMinNumber(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java:99)
- [ ] `EditorCompactRangeNode.Builder.setMaxNumber(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java:104)
- [ ] `EditorCompactRangeNode.Builder.setNumberReader(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java:109)
- [ ] `EditorCompactRangeNode.Builder.setNumberWriter(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java:114)
- [ ] `EditorCompactRangeNode.Builder.setValueNamer(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java:119)
- [ ] `EditorCompactRangeNode.Builder.setHasNullOption(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java:124)
- [ ] `EditorCompactRangeNode.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java:129)
- [ ] `EditorCompactRangeNode.Builder.buildInternally(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java:137)
- [ ] `EditorCompactRangeNode.Builder.buildInternally(3)`: it has no body (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java:152)

### `EditorCompactRangeNode.FinalBuilder`: it's generic

- [ ] `EditorCompactRangeNode.FinalBuilder.FinalBuilder(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java:156)
- [ ] `EditorCompactRangeNode.FinalBuilder.buildInternally(3)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java:159)
- [ ] `EditorCompactRangeNode.FinalBuilder.begin(0)`: it's generic (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java:177)

### `EditorExpandingRangeNode`: it's generic

- [ ] `EditorExpandingRangeNode.EditorExpandingRangeNode(9)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java:20)
- [ ] `EditorExpandingRangeNode.onSelected(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java:36)
- [ ] `EditorExpandingRangeNode.getCurrentRangeValue(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java:46)

### `EditorExpandingRangeNode.Builder`: it's generic

- [ ] `EditorExpandingRangeNode.Builder.Builder(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java:59)
- [ ] `EditorExpandingRangeNode.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java:63)
- [ ] `EditorExpandingRangeNode.Builder.setCurrentRangeValue(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java:74)
- [ ] `EditorExpandingRangeNode.Builder.setMinNumber(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java:79)
- [ ] `EditorExpandingRangeNode.Builder.setMaxNumber(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java:84)
- [ ] `EditorExpandingRangeNode.Builder.setNumberReader(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java:89)
- [ ] `EditorExpandingRangeNode.Builder.setNumberWriter(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java:94)
- [ ] `EditorExpandingRangeNode.Builder.setValueNamer(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java:99)
- [ ] `EditorExpandingRangeNode.Builder.setHasNullOption(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java:104)
- [ ] `EditorExpandingRangeNode.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java:109)
- [ ] `EditorExpandingRangeNode.Builder.buildInternally(2)`: it has no body (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java:136)

### `EditorExpandingRangeNode.FinalBuilder`: it's generic

- [ ] `EditorExpandingRangeNode.FinalBuilder.FinalBuilder(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java:140)
- [ ] `EditorExpandingRangeNode.FinalBuilder.buildInternally(2)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java:144)
- [ ] `EditorExpandingRangeNode.FinalBuilder.begin(1)`: it's generic (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java:159)

### `EditorCompactSettingNode`: it's generic

- [ ] `EditorCompactSettingNode.EditorCompactSettingNode(14)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorCompactSettingNode.java:17)
- [ ] `EditorCompactSettingNode.getSetting(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorCompactSettingNode.java:51)
- [ ] `EditorCompactSettingNode.getSettingValue(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorCompactSettingNode.java:56)
- [ ] `EditorCompactSettingNode.isRootSettings(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorCompactSettingNode.java:61)
- [ ] `EditorCompactSettingNode.getValueName(2)`: it's generic (xaero/hud/category/ui/node/options/range/setting/EditorCompactSettingNode.java:66)

### `EditorCompactSettingNode.Builder`: it's generic

- [ ] `EditorCompactSettingNode.Builder.Builder(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorCompactSettingNode.java:76)
- [ ] `EditorCompactSettingNode.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorCompactSettingNode.java:79)
- [ ] `EditorCompactSettingNode.Builder.setSetting(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorCompactSettingNode.java:85)
- [ ] `EditorCompactSettingNode.Builder.setSettingValue(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorCompactSettingNode.java:106)
- [ ] `EditorCompactSettingNode.Builder.setRootSettings(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorCompactSettingNode.java:111)
- [ ] `EditorCompactSettingNode.Builder.setSlider(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorCompactSettingNode.java:117)
- [ ] `EditorCompactSettingNode.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorCompactSettingNode.java:121)
- [ ] `EditorCompactSettingNode.Builder.begin(0)`: it's generic (xaero/hud/category/ui/node/options/range/setting/EditorCompactSettingNode.java:133)
- [ ] `EditorCompactSettingNode.Builder.buildInternally(3)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorCompactSettingNode.java:137)

### `EditorExpandingSettingNode`: it's generic

- [ ] `EditorExpandingSettingNode.EditorExpandingSettingNode(11)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorExpandingSettingNode.java:19)
- [ ] `EditorExpandingSettingNode.getSetting(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorExpandingSettingNode.java:37)
- [ ] `EditorExpandingSettingNode.getSettingValue(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorExpandingSettingNode.java:42)
- [ ] `EditorExpandingSettingNode.isRootSettings(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorExpandingSettingNode.java:47)

### `EditorExpandingSettingNode.Builder`: it's generic

- [ ] `EditorExpandingSettingNode.Builder.Builder(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorExpandingSettingNode.java:58)
- [ ] `EditorExpandingSettingNode.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorExpandingSettingNode.java:62)
- [ ] `EditorExpandingSettingNode.Builder.setSetting(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorExpandingSettingNode.java:68)
- [ ] `EditorExpandingSettingNode.Builder.setSettingValue(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorExpandingSettingNode.java:89)
- [ ] `EditorExpandingSettingNode.Builder.setRootSettings(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorExpandingSettingNode.java:94)
- [ ] `EditorExpandingSettingNode.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorExpandingSettingNode.java:100)
- [ ] `EditorExpandingSettingNode.Builder.buildInternally(2)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/options/range/setting/EditorExpandingSettingNode.java:113)
- [ ] `EditorExpandingSettingNode.Builder.begin(1)`: it's generic (xaero/hud/category/ui/node/options/range/setting/EditorExpandingSettingNode.java:129)

### `IEditorSettingNode`: it's an interface

- [ ] `IEditorSettingNode.getSetting(0)`: it has no body (xaero/hud/category/ui/node/options/range/setting/IEditorSettingNode.java:6)
- [ ] `IEditorSettingNode.getSettingValue(0)`: it has no body (xaero/hud/category/ui/node/options/range/setting/IEditorSettingNode.java:8)
- [ ] `IEditorSettingNode.isRootSettings(0)`: it has no body (xaero/hud/category/ui/node/options/range/setting/IEditorSettingNode.java:10)

### `IEditorSettingNodeBuilder`: it's an interface

- [ ] `IEditorSettingNodeBuilder.setSetting(1)`: it has no body (xaero/hud/category/ui/node/options/range/setting/IEditorSettingNodeBuilder.java:7)
- [ ] `IEditorSettingNodeBuilder.setSettingValue(1)`: it has no body (xaero/hud/category/ui/node/options/range/setting/IEditorSettingNodeBuilder.java:9)
- [ ] `IEditorSettingNodeBuilder.setRootSettings(1)`: it has no body (xaero/hud/category/ui/node/options/range/setting/IEditorSettingNodeBuilder.java:11)
- [ ] `IEditorSettingNodeBuilder.build(0)`: it has no body (xaero/hud/category/ui/node/options/range/setting/IEditorSettingNodeBuilder.java:13)

### `EditorTextFieldOptionsNode`: it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)

- [ ] `EditorTextFieldOptionsNode.EditorTextFieldOptionsNode(13)`: its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:33)
- [ ] `EditorTextFieldOptionsNode.resetInput(1)`: its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:57)
- [ ] `EditorTextFieldOptionsNode.setCurrentValue(1)`: its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:63)
- [ ] `EditorTextFieldOptionsNode.getCurrentValue(0)`: its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:67)
- [ ] `EditorTextFieldOptionsNode.getInput(0)`: its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:72)
- [ ] `EditorTextFieldOptionsNode.getResult(0)`: its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:76)
- [ ] `EditorTextFieldOptionsNode.getCursorPos(0)`: its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:80)
- [ ] `EditorTextFieldOptionsNode.getHighlightPos(0)`: its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:84)
- [ ] `EditorTextFieldOptionsNode.getMaxLength(0)`: its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:88)
- [ ] `EditorTextFieldOptionsNode.getDisplayName(0)`: its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:92)
- [ ] `EditorTextFieldOptionsNode.setExpanded(1)`: its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:97)
- [ ] `EditorTextFieldOptionsNode.onSelected(1)`: its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:107)
- [ ] `EditorTextFieldOptionsNode.getUpdatedValueConsumer(0)`: its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:114)
- [ ] `EditorTextFieldOptionsNode.getSubNodes(0)`: its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:157)

### `EditorTextFieldOptionsNode.Builder`: it extends `Builder`

- [ ] `EditorTextFieldOptionsNode.Builder.Builder(1)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:171)
- [ ] `EditorTextFieldOptionsNode.Builder.setDefault(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:176)
- [ ] `EditorTextFieldOptionsNode.Builder.getCenteredEntryFactory(4)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:189)
- [ ] `EditorTextFieldOptionsNode.Builder.setInput(1)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:216)
- [ ] `EditorTextFieldOptionsNode.Builder.setInputStringValidator(1)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:221)
- [ ] `EditorTextFieldOptionsNode.Builder.needsInputStringValidator(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:226)
- [ ] `EditorTextFieldOptionsNode.Builder.setAllowCustomInput(1)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:230)
- [ ] `EditorTextFieldOptionsNode.Builder.setAutoConfirm(1)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:235)
- [ ] `EditorTextFieldOptionsNode.Builder.setMaxLength(1)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:240)
- [ ] `EditorTextFieldOptionsNode.Builder.setCurrentValue(1)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:245)
- [ ] `EditorTextFieldOptionsNode.Builder.build(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:249)
- [ ] `EditorTextFieldOptionsNode.Builder.buildInternally(2)`: its class isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:257)
- [ ] `EditorTextFieldOptionsNode.Builder.begin(1)`: it uses Builder, which isn't translated (it extends `Builder`) (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java:275)

### `TextFieldSuggestionsResolver`: it uses ListFactory, which isn't translated (it's an interface)

- [ ] `TextFieldSuggestionsResolver.TextFieldSuggestionsResolver(1)`: its class isn't translated (it uses ListFactory, which isn't translated (it's an interface)) (xaero/hud/category/ui/node/options/text/TextFieldSuggestionsResolver.java:11)
- [ ] `TextFieldSuggestionsResolver.getSuggestions(2)`: its class isn't translated (it uses ListFactory, which isn't translated (it's an interface)) (xaero/hud/category/ui/node/options/text/TextFieldSuggestionsResolver.java:15)

### `TextFieldSuggestionsResolver.Builder`: it uses ListFactory, which isn't translated (it's an interface)

- [ ] `TextFieldSuggestionsResolver.Builder.Builder(1)`: its class isn't translated (it uses ListFactory, which isn't translated (it's an interface)) (xaero/hud/category/ui/node/options/text/TextFieldSuggestionsResolver.java:31)
- [ ] `TextFieldSuggestionsResolver.Builder.setDefault(0)`: its class isn't translated (it uses ListFactory, which isn't translated (it's an interface)) (xaero/hud/category/ui/node/options/text/TextFieldSuggestionsResolver.java:35)
- [ ] `TextFieldSuggestionsResolver.Builder.build(0)`: its class isn't translated (it uses ListFactory, which isn't translated (it's an interface)) (xaero/hud/category/ui/node/options/text/TextFieldSuggestionsResolver.java:39)
- [ ] `TextFieldSuggestionsResolver.Builder.begin(1)`: it uses Builder, which isn't translated (it uses ListFactory, which isn't translated (it's an interface)) (xaero/hud/category/ui/node/options/text/TextFieldSuggestionsResolver.java:47)

### `EditorExcludeListNode`: it extends `EditorListNode`

- [ ] `EditorExcludeListNode.EditorExcludeListNode(10)`: its class isn't translated (it extends `EditorListNode`) (xaero/hud/category/ui/node/rule/EditorExcludeListNode.java:19)
- [ ] `EditorExcludeListNode.getExcludeMode(0)`: its class isn't translated (it extends `EditorListNode`) (xaero/hud/category/ui/node/rule/EditorExcludeListNode.java:35)
- [ ] `EditorExcludeListNode.getDisplayName(0)`: its class isn't translated (it extends `EditorListNode`) (xaero/hud/category/ui/node/rule/EditorExcludeListNode.java:39)
- [ ] `EditorExcludeListNode.getSubNodes(0)`: its class isn't translated (it extends `EditorListNode`) (xaero/hud/category/ui/node/rule/EditorExcludeListNode.java:44)

### `EditorExcludeListNode.Builder`: it's generic

- [ ] `EditorExcludeListNode.Builder.Builder(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorExcludeListNode.java:54)
- [ ] `EditorExcludeListNode.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorExcludeListNode.java:59)
- [ ] `EditorExcludeListNode.Builder.setExcludeMode(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorExcludeListNode.java:70)
- [ ] `EditorExcludeListNode.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorExcludeListNode.java:75)
- [ ] `EditorExcludeListNode.Builder.buildInternally(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorExcludeListNode.java:79)
- [ ] `EditorExcludeListNode.Builder.begin(1)`: it's generic (xaero/hud/category/ui/node/rule/EditorExcludeListNode.java:94)

### `EditorIncludeListNode`: it extends `EditorListNode`

- [ ] `EditorIncludeListNode.EditorIncludeListNode(10)`: its class isn't translated (it extends `EditorListNode`) (xaero/hud/category/ui/node/rule/EditorIncludeListNode.java:17)
- [ ] `EditorIncludeListNode.getDisplayName(0)`: its class isn't translated (it extends `EditorListNode`) (xaero/hud/category/ui/node/rule/EditorIncludeListNode.java:33)
- [ ] `EditorIncludeListNode.getSubNodes(0)`: its class isn't translated (it extends `EditorListNode`) (xaero/hud/category/ui/node/rule/EditorIncludeListNode.java:38)
- [ ] `EditorIncludeListNode.getIncludeInSuper(0)`: its class isn't translated (it extends `EditorListNode`) (xaero/hud/category/ui/node/rule/EditorIncludeListNode.java:45)

### `EditorIncludeListNode.Builder`: it's generic

- [ ] `EditorIncludeListNode.Builder.Builder(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorIncludeListNode.java:52)
- [ ] `EditorIncludeListNode.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorIncludeListNode.java:56)
- [ ] `EditorIncludeListNode.Builder.getIncludeInSuperToggleDataBuilder(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorIncludeListNode.java:62)
- [ ] `EditorIncludeListNode.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorIncludeListNode.java:66)
- [ ] `EditorIncludeListNode.Builder.buildInternally(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorIncludeListNode.java:70)
- [ ] `EditorIncludeListNode.Builder.begin(1)`: it's generic (xaero/hud/category/ui/node/rule/EditorIncludeListNode.java:85)

### `EditorListNode`: it extends `EditorNode` (xaero.hud.category.ui.node)

- [ ] `EditorListNode.EditorListNode(9)`: its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)) (xaero/hud/category/ui/node/rule/EditorListNode.java:34)
- [ ] `EditorListNode.getList(0)`: its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)) (xaero/hud/category/ui/node/rule/EditorListNode.java:54)
- [ ] `EditorListNode.getDeletionCallback(0)`: its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)) (xaero/hud/category/ui/node/rule/EditorListNode.java:58)
- [ ] `EditorListNode.getAdderHandler(0)`: its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)) (xaero/hud/category/ui/node/rule/EditorListNode.java:62)
- [ ] `EditorListNode.getSubNodes(0)`: its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)) (xaero/hud/category/ui/node/rule/EditorListNode.java:81)

### `EditorListNode.Builder`: it's generic

- [ ] `EditorListNode.Builder.Builder(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorListNode.java:108)
- [ ] `EditorListNode.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorListNode.java:114)
- [ ] `EditorListNode.Builder.mainEntryFactory(7)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorListNode.java:136)
- [ ] `EditorListNode.Builder.getCenteredEntryFactory(4)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorListNode.java:158)
- [ ] `EditorListNode.Builder.addListElement(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorListNode.java:168)
- [ ] `EditorListNode.Builder.setDeletionCallback(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorListNode.java:173)
- [ ] `EditorListNode.Builder.getAdderBuilder(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorListNode.java:178)
- [ ] `EditorListNode.Builder.buildList(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorListNode.java:182)
- [ ] `EditorListNode.Builder.setInputRuleTypeStringValidator(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorListNode.java:190)
- [ ] `EditorListNode.Builder.setHelpTooltipSupplier(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorListNode.java:195)
- [ ] `EditorListNode.Builder.setHelpTooltipInfoSupplier(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorListNode.java:200)
- [ ] `EditorListNode.Builder.setDefaultListRuleType(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorListNode.java:204)
- [ ] `EditorListNode.Builder.setListRuleTypes(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorListNode.java:209)
- [ ] `EditorListNode.Builder.setListRuleTypePrefixSeparator(1)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorListNode.java:214)
- [ ] `EditorListNode.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/node/rule/EditorListNode.java:219)
- [ ] `EditorListNode.Builder.addSuggestionsForListRuleType(2)`: it's generic (xaero/hud/category/ui/node/rule/EditorListNode.java:266)
- [ ] `EditorListNode.Builder.buildInternally(0)`: it has no body (xaero/hud/category/ui/node/rule/EditorListNode.java:270)

### `IEditorDataTooltipSupplier`: it's an interface


### `RootCategorySettingEntry`: it's generic

- [ ] `RootCategorySettingEntry.RootCategorySettingEntry(3)`: its class isn't translated (it's generic) (xaero/hud/category/ui/RootCategorySettingEntry.java:17)
- [ ] `RootCategorySettingEntry.RootCategorySettingEntry(4)`: its class isn't translated (it's generic) (xaero/hud/category/ui/RootCategorySettingEntry.java:21)
- [ ] `RootCategorySettingEntry.RootCategorySettingEntry(5)`: its class isn't translated (it's generic) (xaero/hud/category/ui/RootCategorySettingEntry.java:27)
- [ ] `RootCategorySettingEntry.getStringForSearch(0)`: its class isn't translated (it's generic) (xaero/hud/category/ui/RootCategorySettingEntry.java:59)
- [ ] `RootCategorySettingEntry.createWidget(3)`: its class isn't translated (it's generic) (xaero/hud/category/ui/RootCategorySettingEntry.java:64)

### `EditorSettingType`

- [x] `EditorSettingType.ITERATION_BUTTON` (xaero/hud/category/ui/setting/EditorSettingType.java:4)
- [x] `EditorSettingType.SLIDER` (xaero/hud/category/ui/setting/EditorSettingType.java:5)
- [x] `EditorSettingType.EXPANDING` (xaero/hud/category/ui/setting/EditorSettingType.java:6)
- [x] `EditorSettingType.EditorSettingType(2)` (xaero/hud/category/ui/setting/EditorSettingType.java:10)
- [x] `EditorSettingType.isUsingIndices(0)` (xaero/hud/category/ui/setting/EditorSettingType.java:15)

### `SettingNodeBuilderFactory`: it's an interface

- [ ] `SettingNodeBuilderFactory.apply(1)`: it has no body (xaero/hud/category/ui/setting/SettingNodeBuilderFactory.java:8)

### `SettingNodeBuilderFactoryManager`: it uses SettingNodeBuilderFactory, which isn't translated (it's an interface)

- [ ] `SettingNodeBuilderFactoryManager.SettingNodeBuilderFactoryManager(1)`: its class isn't translated (it uses SettingNodeBuilderFactory, which isn't translated (it's an interface)) (xaero/hud/category/ui/setting/SettingNodeBuilderFactoryManager.java:14)
- [ ] `SettingNodeBuilderFactoryManager.register(2)`: its class isn't translated (it uses SettingNodeBuilderFactory, which isn't translated (it's an interface)) (xaero/hud/category/ui/setting/SettingNodeBuilderFactoryManager.java:18)
- [ ] `SettingNodeBuilderFactoryManager.get(1)`: its class isn't translated (it uses SettingNodeBuilderFactory, which isn't translated (it's an interface)) (xaero/hud/category/ui/setting/SettingNodeBuilderFactoryManager.java:26)

### `SettingNodeBuilderFactoryManager.Builder`: it uses MapFactory, which isn't translated (it's an interface)

- [ ] `SettingNodeBuilderFactoryManager.Builder.Builder(1)`: its class isn't translated (it uses MapFactory, which isn't translated (it's an interface)) (xaero/hud/category/ui/setting/SettingNodeBuilderFactoryManager.java:33)
- [ ] `SettingNodeBuilderFactoryManager.Builder.setDefault(0)`: its class isn't translated (it uses MapFactory, which isn't translated (it's an interface)) (xaero/hud/category/ui/setting/SettingNodeBuilderFactoryManager.java:37)
- [ ] `SettingNodeBuilderFactoryManager.Builder.build(0)`: its body couldn't be read: an anonymous class (xaero/hud/category/ui/setting/SettingNodeBuilderFactoryManager.java:41)
- [ ] `SettingNodeBuilderFactoryManager.Builder.begin(1)`: it uses Builder, which isn't translated (it uses MapFactory, which isn't translated (it's an interface)) (xaero/hud/category/ui/setting/SettingNodeBuilderFactoryManager.java:59)

### `CategoryConstants`

- [x] `CategoryConstants.ON` in game_side.rs (xaero/hud/category/util/CategoryConstants.java:6)
- [x] `CategoryConstants.OFF` in game_side.rs (xaero/hud/category/util/CategoryConstants.java:7)
- [x] `CategoryConstants.INHERIT` in game_side.rs (xaero/hud/category/util/CategoryConstants.java:8)
- [x] `CategoryConstants.getBooleanComponent(1)` in game_side.rs (xaero/hud/category/util/CategoryConstants.java:10)

### `HudModConfigCommonRegistryHandler`: it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler)

- [ ] `HudModConfigCommonRegistryHandler.registerPrimaryCommonOptions(1)`: its class isn't translated (it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler)) (xaero/hud/common/config/channel/register/HudModConfigCommonRegistryHandler.java:13)
- [ ] `HudModConfigCommonRegistryHandler.registerProfiledOptions(1)`: its class isn't translated (it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler)) (xaero/hud/common/config/channel/register/HudModConfigCommonRegistryHandler.java:17)
- [ ] `HudModConfigCommonRegistryHandler.registerServerOptionChangeHandlers(1)`: its class isn't translated (it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler)) (xaero/hud/common/config/channel/register/HudModConfigCommonRegistryHandler.java:21)
- [ ] `HudModConfigCommonRegistryHandler.registerOptionServerRedirectors(1)`: its class isn't translated (it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler)) (xaero/hud/common/config/channel/register/HudModConfigCommonRegistryHandler.java:25)

### `ImmediatelyFastHelper`

- [ ] `ImmediatelyFastHelper.triggerBatchingBuffersFlush(1)`: it calls `ImmediateRenderUtil` (xaero.lib.client.graphics.util).coloredRectangle (xaero/hud/compat/mods/ImmediatelyFastHelper.java:7)

### `SupportWaystones`

- [ ] `SupportWaystones.registerClientEvents(0)`: it uses `WaystonesListReceivedEvent` (net.blay09.mods.waystones.api.event).EVENT (xaero/hud/compat/mods/SupportWaystones.java:29)
- [ ] `SupportWaystones.onWaystoneDeactivation(1)`: it uses `Waystone` (net.blay09.mods.waystones.api) (xaero/hud/compat/mods/SupportWaystones.java:65)
- [ ] `SupportWaystones.addWaypoint(2)`: it uses `Waystone` (net.blay09.mods.waystones.api) (xaero/hud/compat/mods/SupportWaystones.java:77)
- [ ] `SupportWaystones.removeAllOfOrigin(2)`: it uses MinimapWorldRootContainer, which isn't translated (it extends `MinimapWorldContainer`) (xaero/hud/compat/mods/SupportWaystones.java:122)
- [ ] `SupportWaystones.removeWaypoint(4)`: it uses `UUID` (java.util) (xaero/hud/compat/mods/SupportWaystones.java:128)
- [ ] `SupportWaystones.removeWaypointFromAllOrigins(2)`: it uses `UUID` (java.util) (xaero/hud/compat/mods/SupportWaystones.java:134)
- [ ] `SupportWaystones.getWithConfigCheck(2)`: it calls `WaystoneKinds` (net.blay09.mods.waystones.api).isSharestone (xaero/hud/compat/mods/SupportWaystones.java:142)
- [x] `SupportWaystones.getDiscoveredOriginId(1)` (xaero/hud/compat/mods/SupportWaystones.java:155)
- [x] `SupportWaystones.getGlobalOriginId(1)` (xaero/hud/compat/mods/SupportWaystones.java:159)
- [ ] `SupportWaystones.getSharestoneColor(1)`: it uses WaypointColor, which isn't translated (its constants carry values) (xaero/hud/compat/mods/SupportWaystones.java:163)

### `OldSystemCompatibility`

- [ ] `OldSystemCompatibility.convertTransform(3)`: it uses ModuleSession, which isn't translated (it's generic) (xaero/hud/compat/OldSystemCompatibility.java:8)
- [ ] `OldSystemCompatibility.loadOldTransform(1)`: it calls `Integer`.parseInt (xaero/hud/compat/OldSystemCompatibility.java:19)

### `HudModConfigClientRegistryHandler`: it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler)

- [ ] `HudModConfigClientRegistryHandler.registerPrimaryClientOptions(1)`: its class isn't translated (it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler)) (xaero/hud/config/channel/register/HudModConfigClientRegistryHandler.java:14)
- [ ] `HudModConfigClientRegistryHandler.registerConfigOptionUITypes(1)`: its class isn't translated (it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler)) (xaero/hud/config/channel/register/HudModConfigClientRegistryHandler.java:18)
- [ ] `HudModConfigClientRegistryHandler.registerClientOptionChangeHandlers(1)`: its class isn't translated (it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler)) (xaero/hud/config/channel/register/HudModConfigClientRegistryHandler.java:22)
- [ ] `HudModConfigClientRegistryHandler.registerOptionClientRedirectors(1)`: its class isn't translated (it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler)) (xaero/hud/config/channel/register/HudModConfigClientRegistryHandler.java:26)

### `ControlsRegister`: it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)

- [ ] `ControlsRegister.ControlsRegister(0)`: its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)) (xaero/hud/controls/ControlsRegister.java:12)
- [ ] `ControlsRegister.registerKeybindings(2)`: its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)) (xaero/hud/controls/ControlsRegister.java:15)
- [ ] `ControlsRegister.onStage2(0)`: its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)) (xaero/hud/controls/ControlsRegister.java:19)
- [ ] `ControlsRegister.getKeyMappingControllers(0)`: its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)) (xaero/hud/controls/ControlsRegister.java:22)

### `KeyMappingFunction`

- [x] `KeyMappingFunction.KeyMappingFunction(1)` (xaero/hud/controls/key/function/KeyMappingFunction.java:6)
- [ ] `KeyMappingFunction.onPress(0)`: it has no body (xaero/hud/controls/key/function/KeyMappingFunction.java:10)
- [ ] `KeyMappingFunction.onRelease(0)`: it has no body (xaero/hud/controls/key/function/KeyMappingFunction.java:12)
- [x] `KeyMappingFunction.isHeld(0)` (xaero/hud/controls/key/function/KeyMappingFunction.java:14)

### `KeyMappingController`: it implements `Iterable`

- [ ] `KeyMappingController.KeyMappingController(2)`: its class isn't translated (it implements `Iterable`) (xaero/hud/controls/key/KeyMappingController.java:15)
- [ ] `KeyMappingController.add(1)`: its class isn't translated (it implements `Iterable`) (xaero/hud/controls/key/KeyMappingController.java:21)
- [ ] `KeyMappingController.getKeyMapping(0)`: its class isn't translated (it implements `Iterable`) (xaero/hud/controls/key/KeyMappingController.java:25)
- [ ] `KeyMappingController.getFunctions(0)`: its class isn't translated (it implements `Iterable`) (xaero/hud/controls/key/KeyMappingController.java:29)
- [ ] `KeyMappingController.isXaeroKey(0)`: its class isn't translated (it implements `Iterable`) (xaero/hud/controls/key/KeyMappingController.java:33)
- [ ] `KeyMappingController.isPressed(0)`: its class isn't translated (it implements `Iterable`) (xaero/hud/controls/key/KeyMappingController.java:37)
- [ ] `KeyMappingController.setPressed(1)`: its class isn't translated (it implements `Iterable`) (xaero/hud/controls/key/KeyMappingController.java:41)
- [ ] `KeyMappingController.iterator(0)`: its class isn't translated (it implements `Iterable`) (xaero/hud/controls/key/KeyMappingController.java:45)

### `KeyMappingControllerManager`: it implements `Iterable`

- [ ] `KeyMappingControllerManager.getController(1)`: its class isn't translated (it implements `Iterable`) (xaero/hud/controls/key/KeyMappingControllerManager.java:13)
- [ ] `KeyMappingControllerManager.registerController(2)`: its class isn't translated (it implements `Iterable`) (xaero/hud/controls/key/KeyMappingControllerManager.java:17)
- [ ] `KeyMappingControllerManager.registerController(3)`: its class isn't translated (it implements `Iterable`) (xaero/hud/controls/key/KeyMappingControllerManager.java:21)
- [ ] `KeyMappingControllerManager.registerFunction(2)`: its class isn't translated (it implements `Iterable`) (xaero/hud/controls/key/KeyMappingControllerManager.java:32)
- [ ] `KeyMappingControllerManager.iterator(0)`: its class isn't translated (it implements `Iterable`) (xaero/hud/controls/key/KeyMappingControllerManager.java:41)

### `KeyMappingTickHandler`: it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)

- [x] `KeyMappingTickHandler.DISABLE_KEY_MAPPING_OVERRIDES` (xaero/hud/controls/key/KeyMappingTickHandler.java:8)
- [ ] `KeyMappingTickHandler.KeyMappingTickHandler(1)`: its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)) (xaero/hud/controls/key/KeyMappingTickHandler.java:11)
- [ ] `KeyMappingTickHandler.tick(0)`: its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)) (xaero/hud/controls/key/KeyMappingTickHandler.java:15)
- [ ] `KeyMappingTickHandler.handlePress(1)`: its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)) (xaero/hud/controls/key/KeyMappingTickHandler.java:29)
- [ ] `KeyMappingTickHandler.handleExtraPresses(1)`: its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)) (xaero/hud/controls/key/KeyMappingTickHandler.java:44)
- [ ] `KeyMappingTickHandler.handleRelease(1)`: its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)) (xaero/hud/controls/key/KeyMappingTickHandler.java:62)
- [ ] `KeyMappingTickHandler.handlePressFunctions(2)`: its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)) (xaero/hud/controls/key/KeyMappingTickHandler.java:71)
- [ ] `KeyMappingTickHandler.handleReleaseFunctions(1)`: its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)) (xaero/hud/controls/key/KeyMappingTickHandler.java:79)
- [ ] `KeyMappingTickHandler.getUpdatedPressedValue(1)`: its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)) (xaero/hud/controls/key/KeyMappingTickHandler.java:85)
- [ ] `KeyMappingTickHandler.isPhysicallyDown(1)`: its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)) (xaero/hud/controls/key/KeyMappingTickHandler.java:104)

### `EntityUtils`

- [ ] `EntityUtils.getEntityX(2)`: it uses Minecraft's `Entity` (xaero/hud/entity/EntityUtils.java:7)
- [ ] `EntityUtils.getEntityY(2)`: it uses Minecraft's `Entity` (xaero/hud/entity/EntityUtils.java:12)
- [ ] `EntityUtils.getEntityZ(2)`: it uses Minecraft's `Entity` (xaero/hud/entity/EntityUtils.java:17)
- [ ] `EntityUtils.getEntityPos(2)`: it uses Minecraft's `Entity` (xaero/hud/entity/EntityUtils.java:22)

### `HudEventHandler`: it uses Hud, which isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic)))

- [ ] `HudEventHandler.setHud(1)`: its class isn't translated (it uses Hud, which isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic)))) (xaero/hud/event/HudEventHandler.java:10)
- [ ] `HudEventHandler.handleRenderGameOverlayEventPost(0)`: it calls XaeroMinimapSession.getCurrentSession, which isn't translated (xaero/hud/event/HudEventHandler.java:18)

### `GuiUtils`

- [ ] `GuiUtils.getMinimapScale(1)`: it uses `ClientConfigManager` (xaero.lib.client.config) (xaero/hud/gui/util/GuiUtils.java:11)
- [ ] `GuiUtils.getUIScale(3)`: it calls GuiUtils.getAutoUIScale, which isn't translated (xaero/hud/gui/util/GuiUtils.java:19)
- [ ] `GuiUtils.getAutoUIScale(0)`: it calls Minecraft's Minecraft.getWindow (no Horizon helper for it yet) (xaero/hud/gui/util/GuiUtils.java:27)
- [ ] `GuiUtils.refreshScreenBase(0)`: it uses Minecraft's `Screen` (xaero/hud/gui/util/GuiUtils.java:34)

### `XaeroSliderWidget`: it extends Minecraft's `AbstractSliderButton`

- [ ] `XaeroSliderWidget.XaeroSliderWidget(8)`: its class isn't translated (it extends Minecraft's `AbstractSliderButton`) (xaero/hud/gui/widget/XaeroSliderWidget.java:15)
- [ ] `XaeroSliderWidget.applyValue(0)`: its class isn't translated (it extends Minecraft's `AbstractSliderButton`) (xaero/hud/gui/widget/XaeroSliderWidget.java:21)
- [ ] `XaeroSliderWidget.updateMessage(0)`: its class isn't translated (it extends Minecraft's `AbstractSliderButton`) (xaero/hud/gui/widget/XaeroSliderWidget.java:25)
- [ ] `XaeroSliderWidget.setXaero_tooltip(1)`: its class isn't translated (it extends Minecraft's `AbstractSliderButton`) (xaero/hud/gui/widget/XaeroSliderWidget.java:29)
- [ ] `XaeroSliderWidget.getXaero_tooltip(0)`: its class isn't translated (it extends Minecraft's `AbstractSliderButton`) (xaero/hud/gui/widget/XaeroSliderWidget.java:33)

### `Hud`: it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))

- [ ] `Hud.Hud(6)`: it stores a second reference to an object (Rust would need it copied or shared) (xaero/hud/Hud.java:18)
- [ ] `Hud.getModuleManager(0)`: its class isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))) (xaero/hud/Hud.java:34)
- [ ] `Hud.getPushboxManager(0)`: its class isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))) (xaero/hud/Hud.java:38)
- [ ] `Hud.getPresetManager(0)`: its class isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))) (xaero/hud/Hud.java:42)
- [ ] `Hud.getEventHandler(0)`: it stores a second reference to an object (Rust would need it copied or shared) (xaero/hud/Hud.java:46)
- [ ] `Hud.getOldSystemCompatibility(0)`: its class isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))) (xaero/hud/Hud.java:50)
- [ ] `Hud.getSessionHandler(0)`: its class isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))) (xaero/hud/Hud.java:54)

### `Hud.Builder`

- [x] `Hud.Builder.Builder(0)` (xaero/hud/Hud.java:59)
- [ ] `Hud.Builder.setDefault(0)`: `this` used as a value (xaero/hud/Hud.java:62)
- [ ] `Hud.Builder.build(0)`: it creates a Hud with a constructor that isn't translated (xaero/hud/Hud.java:66)
- [ ] `Hud.Builder.begin(0)`: it calls Builder.setDefault, which isn't translated (xaero/hud/Hud.java:78)

### `HudSession`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)

- [ ] `HudSession.HudSession(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/HudSession.java:25)
- [ ] `HudSession.getSession(1)`: it's generic (xaero/hud/HudSession.java:30)
- [ ] `HudSession.init(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/HudSession.java:41)
- [ ] `HudSession.tryCleanup(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/HudSession.java:50)
- [ ] `HudSession.cleanup(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/HudSession.java:62)
- [ ] `HudSession.getCurrentSession(0)`: it uses HudSession, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/HudSession.java:68)
- [ ] `HudSession.getForPlayer(1)`: it uses HudSession, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/HudSession.java:77)
- [ ] `HudSession.getKeyMappingTickHandler(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/HudSession.java:81)
- [ ] `HudSession.getHudMod(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/HudSession.java:85)

### `HudIO`: it uses Hud, which isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic)))

- [x] `HudIO.SEPARATOR` (xaero/hud/io/HudIO.java:14)
- [x] `HudIO.MODULE_LINE_PREFIX` (xaero/hud/io/HudIO.java:15)
- [ ] `HudIO.HudIO(2)`: its class isn't translated (it uses Hud, which isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic)))) (xaero/hud/io/HudIO.java:19)
- [ ] `HudIO.save(1)`: it uses `PrintWriter` (java.io) (xaero/hud/io/HudIO.java:24)
- [ ] `HudIO.load(2)`: it uses HudModule, which isn't translated (it's generic) (xaero/hud/io/HudIO.java:68)

### `HudIO.Builder`: it uses Hud, which isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic)))

- [ ] `HudIO.Builder.Builder(0)`: its class isn't translated (it uses Hud, which isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic)))) (xaero/hud/io/HudIO.java:133)
- [ ] `HudIO.Builder.setDefault(0)`: `this` used as a value (xaero/hud/io/HudIO.java:136)
- [ ] `HudIO.Builder.setHud(1)`: `this` used as a value (xaero/hud/io/HudIO.java:141)
- [ ] `HudIO.Builder.build(0)`: its class isn't translated (it uses Hud, which isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic)))) (xaero/hud/io/HudIO.java:146)
- [ ] `HudIO.Builder.begin(0)`: it uses Builder, which isn't translated (it uses Hud, which isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic)))) (xaero/hud/io/HudIO.java:154)

### `BuiltInHudModules`

- [ ] `BuiltInHudModules.MINIMAP`: it uses HudModule, which isn't translated (it's generic) (xaero/hud/minimap/BuiltInHudModules.java:13)
- [ ] `BuiltInHudModules.addAll(1)`: it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic)) (xaero/hud/minimap/BuiltInHudModules.java:22)

### `InfoDisplayConfigData`

- [x] `InfoDisplayConfigData.InfoDisplayConfigData(3)` (xaero/hud/minimap/common/config/info/config/InfoDisplayConfigData.java:8)
- [x] `InfoDisplayConfigData.getBackgroundColor(0)` (xaero/hud/minimap/common/config/info/config/InfoDisplayConfigData.java:14)
- [x] `InfoDisplayConfigData.getTextColor(0)` (xaero/hud/minimap/common/config/info/config/InfoDisplayConfigData.java:18)
- [x] `InfoDisplayConfigData.getState(0)` (xaero/hud/minimap/common/config/info/config/InfoDisplayConfigData.java:22)

### `InfoDisplayManagerConfigData`

- [x] `InfoDisplayManagerConfigData.EMPTY` (xaero/hud/minimap/common/config/info/config/InfoDisplayManagerConfigData.java:10)
- [x] `InfoDisplayManagerConfigData.InfoDisplayManagerConfigData(2)` (xaero/hud/minimap/common/config/info/config/InfoDisplayManagerConfigData.java:14)
- [x] `InfoDisplayManagerConfigData.getFromOrder(1)` (xaero/hud/minimap/common/config/info/config/InfoDisplayManagerConfigData.java:19)
- [ ] `InfoDisplayManagerConfigData.getOrderStream(0)`: it uses `Stream` (java.util.stream) (xaero/hud/minimap/common/config/info/config/InfoDisplayManagerConfigData.java:23)
- [x] `InfoDisplayManagerConfigData.copyOrder(0)` (xaero/hud/minimap/common/config/info/config/InfoDisplayManagerConfigData.java:27)
- [x] `InfoDisplayManagerConfigData.get(1)` (xaero/hud/minimap/common/config/info/config/InfoDisplayManagerConfigData.java:31)

### `InfoDisplayManagerConfigData.Builder`

- [x] `InfoDisplayManagerConfigData.Builder.Builder(0)` (xaero/hud/minimap/common/config/info/config/InfoDisplayManagerConfigData.java:39)
- [ ] `InfoDisplayManagerConfigData.Builder.setDefault(0)`: `this` used as a value (xaero/hud/minimap/common/config/info/config/InfoDisplayManagerConfigData.java:42)
- [ ] `InfoDisplayManagerConfigData.Builder.add(2)`: `this` used as a value (xaero/hud/minimap/common/config/info/config/InfoDisplayManagerConfigData.java:48)
- [x] `InfoDisplayManagerConfigData.Builder.build(0)` (xaero/hud/minimap/common/config/info/config/InfoDisplayManagerConfigData.java:54)
- [ ] `InfoDisplayManagerConfigData.Builder.begin(0)`: it calls Builder.setDefault, which isn't translated (xaero/hud/minimap/common/config/info/config/InfoDisplayManagerConfigData.java:58)

### `MinimapConfigConstants`

- [x] `MinimapConfigConstants.DOTS_STYLES` in game_side.rs (xaero/hud/minimap/common/config/MinimapConfigConstants.java:6)
- [x] `MinimapConfigConstants.MAIN_ENTITY_TYPES` in game_side.rs (xaero/hud/minimap/common/config/MinimapConfigConstants.java:9)
- [x] `MinimapConfigConstants.ARROW_COLOR_NAMES` in game_side.rs (xaero/hud/minimap/common/config/MinimapConfigConstants.java:12)
- [x] `MinimapConfigConstants.ARROW_COLORS` (xaero/hud/minimap/common/config/MinimapConfigConstants.java:22)
- [x] `MinimapConfigConstants.AUTO_SCALE_COMPONENT` in game_side.rs (xaero/hud/minimap/common/config/MinimapConfigConstants.java:32)
- [x] `MinimapConfigConstants.MINECRAFT_SCALE_COMPONENT` in game_side.rs (xaero/hud/minimap/common/config/MinimapConfigConstants.java:33)
- [x] `MinimapConfigConstants.DEFAULT_WORLD_WAYPOINT_SCALE` (xaero/hud/minimap/common/config/MinimapConfigConstants.java:34)
- [x] `MinimapConfigConstants.WORLD_WAYPOINT_MINECRAFT_SCALE` (xaero/hud/minimap/common/config/MinimapConfigConstants.java:35)
- [x] `MinimapConfigConstants.WAYPOINT_ICON_WORLD_SCALE` (xaero/hud/minimap/common/config/MinimapConfigConstants.java:36)
- [x] `MinimapConfigConstants.MULTIPLE_WAYPOINTS_INFO_NAMES` in game_side.rs (xaero/hud/minimap/common/config/MinimapConfigConstants.java:37)
- [x] `MinimapConfigConstants.WAYPOINT_DISTANCE_NAMES` in game_side.rs (xaero/hud/minimap/common/config/MinimapConfigConstants.java:42)
- [x] `MinimapConfigConstants.UNLIMITED_COMPONENT` in game_side.rs (xaero/hud/minimap/common/config/MinimapConfigConstants.java:45)
- [x] `MinimapConfigConstants.CONVERT_DISTANCE_NEVER_COMPONENT` in game_side.rs (xaero/hud/minimap/common/config/MinimapConfigConstants.java:46)
- [x] `MinimapConfigConstants.HIDE_WORLD_NAMES_NAMES` in game_side.rs (xaero/hud/minimap/common/config/MinimapConfigConstants.java:47)
- [x] `MinimapConfigConstants.COMPASS_LOCATION_NAMES` in game_side.rs (xaero/hud/minimap/common/config/MinimapConfigConstants.java:50)
- [x] `MinimapConfigConstants.COLOR_NAMES` in game_side.rs (xaero/hud/minimap/common/config/MinimapConfigConstants.java:55)
- [x] `MinimapConfigConstants.COLORS` (xaero/hud/minimap/common/config/MinimapConfigConstants.java:73)
- [x] `MinimapConfigConstants.COLOR_CODES` (xaero/hud/minimap/common/config/MinimapConfigConstants.java:76)
- [x] `MinimapConfigConstants.BLOCK_COLOR_NAMES` in game_side.rs (xaero/hud/minimap/common/config/MinimapConfigConstants.java:77)
- [x] `MinimapConfigConstants.TERRAIN_SLOPES_NAMES` in game_side.rs (xaero/hud/minimap/common/config/MinimapConfigConstants.java:80)
- [x] `MinimapConfigConstants.MINIMAP_SHAPE_NAMES` in game_side.rs (xaero/hud/minimap/common/config/MinimapConfigConstants.java:86)
- [x] `MinimapConfigConstants.MAX_ZOOM` (xaero/hud/minimap/common/config/MinimapConfigConstants.java:89)
- [x] `MinimapConfigConstants.FRAME_NAMES` in game_side.rs (xaero/hud/minimap/common/config/MinimapConfigConstants.java:90)
- [x] `MinimapConfigConstants.PUSH_BOX_NAMES` in game_side.rs (xaero/hud/minimap/common/config/MinimapConfigConstants.java:96)
- [x] `MinimapConfigConstants.LIGHT_NAMES` in game_side.rs (xaero/hud/minimap/common/config/MinimapConfigConstants.java:99)
- [x] `MinimapConfigConstants.INFO_DISPLAY_ALIGNMENT_NAMES` in game_side.rs (xaero/hud/minimap/common/config/MinimapConfigConstants.java:106)

### `MinimapProfiledConfigOptions`

- [ ] `MinimapProfiledConfigOptions.ALL`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:30)
- [ ] `MinimapProfiledConfigOptions.DISPLAY_MINIMAP`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:31)
- [ ] `MinimapProfiledConfigOptions.MINIMAP_ITEM`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:36)
- [ ] `MinimapProfiledConfigOptions.SAFE_MODE`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:44)
- [ ] `MinimapProfiledConfigOptions.UI_SCALE`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:49)
- [ ] `MinimapProfiledConfigOptions.RADAR_CATEGORIES`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:58)
- [ ] `MinimapProfiledConfigOptions.DISPLAY_RADAR`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:69)
- [ ] `MinimapProfiledConfigOptions.RADAR_DOTS_STYLE`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:72)
- [ ] `MinimapProfiledConfigOptions.RADAR_SMOOTH_DOTS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:80)
- [ ] `MinimapProfiledConfigOptions.RADAR_NAME_SCALE`: it uses `SteppedConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:83)
- [ ] `MinimapProfiledConfigOptions.RADAR_MAIN_ENTITY`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:92)
- [ ] `MinimapProfiledConfigOptions.RADAR_MAIN_DOT_SIZE`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:100)
- [ ] `MinimapProfiledConfigOptions.RADAR_HIDE_INVISIBLE`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:107)
- [ ] `MinimapProfiledConfigOptions.ARROW_SCALE`: it uses `SteppedConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:111)
- [ ] `MinimapProfiledConfigOptions.ARROW_COLOR`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:122)
- [ ] `MinimapProfiledConfigOptions.ARROW_OPACITY`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:131)
- [ ] `MinimapProfiledConfigOptions.TRACKED_PLAYERS_ON_MINIMAP`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:138)
- [ ] `MinimapProfiledConfigOptions.TRACKED_PLAYERS_IN_WORLD`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:144)
- [ ] `MinimapProfiledConfigOptions.TRACKED_PLAYER_MINIMAP_ICON_SCALE`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:149)
- [ ] `MinimapProfiledConfigOptions.TRACKED_PLAYER_WORLD_ICON_SCALE`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:158)
- [ ] `MinimapProfiledConfigOptions.TRACKED_PLAYER_WORLD_NAME_SCALE`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:167)
- [ ] `MinimapProfiledConfigOptions.WAYPOINTS_IN_WORLD`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:176)
- [ ] `MinimapProfiledConfigOptions.WAYPOINTS_ON_MINIMAP`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:180)
- [ ] `MinimapProfiledConfigOptions.WAYPOINTS_ALL_SETS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:184)
- [ ] `MinimapProfiledConfigOptions.DEATHPOINTS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:188)
- [ ] `MinimapProfiledConfigOptions.OLD_DEATHPOINTS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:191)
- [ ] `MinimapProfiledConfigOptions.DELETE_REACHED_DEATHPOINTS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:194)
- [ ] `MinimapProfiledConfigOptions.AUTO_WAYPOINTS_ON_DEATH`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:200)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_ICON_SCALE_IN_WORLD`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:206)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_NAME_SCALE_IN_WORLD`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:215)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_CLOSE_SCALE_IN_WORLD`: it uses `SteppedConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:224)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_DISTANCE_SCALE_IN_WORLD`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:234)
- [ ] `MinimapProfiledConfigOptions.MULTIPLE_WAYPOINTS_INFO`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:243)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_DISTANCE_IN_WORLD`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:252)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_DISTANCE_IN_MENU`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:261)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_NAME_IN_WORLD`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:265)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_SHORT_DISTANCE_IN_WORLD`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:269)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_VERTICAL_POINTING_ANGLE`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:275)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_HORIZONTAL_POINTING_ANGLE`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:282)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_MAX_DISTANCE`: it uses `IndexedConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:289)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_MAX_DISTANCE_DIMENSION_SCALE`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:301)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_MIN_DISTANCE_IN_WORLD`: it uses `SteppedConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:307)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_CONVERT_DISTANCE_TO_KM_AT`: it uses `IndexedConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:317)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_DISTANCE_PRECISION`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:327)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_OPACITY_IN_WORLD`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:334)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_OPACITY_ON_MINIMAP`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:341)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_ICON_SCALE_ON_MINIMAP`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:348)
- [ ] `MinimapProfiledConfigOptions.TEMPORARY_WAYPOINTS_GLOBAL`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:356)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_DEFAULT_TELEPORT_FORMAT`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:362)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_DEFAULT_TELEPORT_ROTATION_FORMAT`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:369)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_TELEPORT_CROSS_DIMENSION`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:376)
- [ ] `MinimapProfiledConfigOptions.WAYPOINT_PARTIAL_Y_TELEPORT`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:381)
- [ ] `MinimapProfiledConfigOptions.NEW_WAYPOINTS_TO_BOTTOM`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:387)
- [ ] `MinimapProfiledConfigOptions.HIDE_WORLD_NAMES`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:392)
- [ ] `MinimapProfiledConfigOptions.COMPASS_OVER_EVERYTHING`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:400)
- [ ] `MinimapProfiledConfigOptions.HIDE_WAYPOINT_COORDINATES`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:406)
- [ ] `MinimapProfiledConfigOptions.COMPASS_LOCATION`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:410)
- [ ] `MinimapProfiledConfigOptions.COMPASS_SHADOW_COLOR`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:419)
- [ ] `MinimapProfiledConfigOptions.COMPASS_NORTH_SHADOW_COLOR`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:428)
- [ ] `MinimapProfiledConfigOptions.COMPASS_SCALE`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:441)
- [ ] `MinimapProfiledConfigOptions.BLOCK_COLORS`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:450)
- [ ] `MinimapProfiledConfigOptions.BIOMES_IN_VANILLA_COLORS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:458)
- [ ] `MinimapProfiledConfigOptions.CAVE_MODE_ALLOWED`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:462)
- [ ] `MinimapProfiledConfigOptions.AUTO_CAVE_MODE`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:467)
- [ ] `MinimapProfiledConfigOptions.CAVE_MODE_ALLOWED_DIMENSIONS`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:485)
- [ ] `MinimapProfiledConfigOptions.MANUAL_CAVE_MODE_START`: it uses `IndexedConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:498)
- [ ] `MinimapProfiledConfigOptions.CAVE_MODE_DEPTH`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:512)
- [ ] `MinimapProfiledConfigOptions.LEGIBLE_CAVE_MAPS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:519)
- [ ] `MinimapProfiledConfigOptions.CAVE_MODE_TOGGLE_TIMER`: it uses `SteppedConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:524)
- [ ] `MinimapProfiledConfigOptions.BIOME_BLENDING`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:535)
- [ ] `MinimapProfiledConfigOptions.DISPLAY_WORLD_MAP_CHUNKS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:540)
- [ ] `MinimapProfiledConfigOptions.TERRAIN_DEPTH`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:544)
- [ ] `MinimapProfiledConfigOptions.TERRAIN_SLOPES`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:547)
- [ ] `MinimapProfiledConfigOptions.DISPLAY_REDSTONE`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:555)
- [ ] `MinimapProfiledConfigOptions.DISPLAY_FLOWERS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:559)
- [ ] `MinimapProfiledConfigOptions.BLOCK_TRANSPARENCY`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:563)
- [ ] `MinimapProfiledConfigOptions.DISPLAY_STAINED_GLASS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:567)
- [ ] `MinimapProfiledConfigOptions.ADJUST_HEIGHT_FOR_SHORT_BLOCKS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:571)
- [ ] `MinimapProfiledConfigOptions.ANTI_ALIASING`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:577)
- [ ] `MinimapProfiledConfigOptions.SIZE`: it uses `IndexedConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:580)
- [ ] `MinimapProfiledConfigOptions.SHAPE`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:591)
- [ ] `MinimapProfiledConfigOptions.NORTH_LOCKED`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:599)
- [ ] `MinimapProfiledConfigOptions.ZOOM`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:602)
- [ ] `MinimapProfiledConfigOptions.FURTHER_ZOOMOUT`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:610)
- [ ] `MinimapProfiledConfigOptions.CAVE_ZOOM`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:615)
- [ ] `MinimapProfiledConfigOptions.ZOOM_ENLARGED`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:624)
- [ ] `MinimapProfiledConfigOptions.CENTERED_ENLARGED`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:632)
- [ ] `MinimapProfiledConfigOptions.KEEP_ENLARGED_UNLOCKED`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:636)
- [ ] `MinimapProfiledConfigOptions.TOGGLED_ENLARGED`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:642)
- [ ] `MinimapProfiledConfigOptions.LIGHTING`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:647)
- [ ] `MinimapProfiledConfigOptions.OPACITY`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:650)
- [ ] `MinimapProfiledConfigOptions.FRAME`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:657)
- [ ] `MinimapProfiledConfigOptions.FRAME_COLOR`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:665)
- [ ] `MinimapProfiledConfigOptions.HIDE_UNDER_SCREEN`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:674)
- [ ] `MinimapProfiledConfigOptions.HIDE_UNDER_F3`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:679)
- [ ] `MinimapProfiledConfigOptions.BOSS_HEALTH_PUSH_BOX`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:684)
- [ ] `MinimapProfiledConfigOptions.POTION_EFFECT_PUSH_BOX`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:693)
- [ ] `MinimapProfiledConfigOptions.CHUNK_GRID`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:702)
- [ ] `MinimapProfiledConfigOptions.CHUNK_GRID_LINE_WIDTH`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:712)
- [ ] `MinimapProfiledConfigOptions.SLIME_CHUNKS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:719)
- [ ] `MinimapProfiledConfigOptions.OPEN_SLIME_CHUNKS_SCREEN`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:722)
- [ ] `MinimapProfiledConfigOptions.LIGHT_OVERLAY_TYPE`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:726)
- [ ] `MinimapProfiledConfigOptions.LIGHT_OVERLAY_MAX_LIGHT`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:743)
- [ ] `MinimapProfiledConfigOptions.LIGHT_OVERLAY_MIN_LIGHT`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:750)
- [ ] `MinimapProfiledConfigOptions.LIGHT_OVERLAY_COLOR`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:757)
- [ ] `MinimapProfiledConfigOptions.OPAC_CLAIMS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:765)
- [ ] `MinimapProfiledConfigOptions.OPAC_CLAIMS_FILL_OPACITY`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:770)
- [ ] `MinimapProfiledConfigOptions.OPAC_CLAIMS_BORDER_OPACITY`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:778)
- [ ] `MinimapProfiledConfigOptions.OPAC_CURRENT_CLAIM`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:786)
- [ ] `MinimapProfiledConfigOptions.INFO_DISPLAY_BG_OPACITY`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:792)
- [ ] `MinimapProfiledConfigOptions.INFO_DISPLAY_ALIGNMENT`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:800)
- [ ] `MinimapProfiledConfigOptions.INFO_DISPLAY_CONFIG`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:808)
- [ ] `MinimapProfiledConfigOptions.RENDER_LAYER`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:818)
- [ ] `MinimapProfiledConfigOptions.UNDISCOVERED_OPACITY`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:825)
- [ ] `MinimapProfiledConfigOptions.DISCOVERED_WAYSTONE_WAYPOINTS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:833)
- [ ] `MinimapProfiledConfigOptions.OTHER_WAYSTONE_WAYPOINTS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:839)
- [ ] `MinimapProfiledConfigOptions.registerAll(1)`: it uses `ConfigOptionManager` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:845)
- [ ] `MinimapProfiledConfigOptions.getDefaultBossHealthPushBox(0)`: a try/catch (ClassNotFoundException) (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java:851)

### `MinimapConfigOptionServerRedirectors`

- [ ] `MinimapConfigOptionServerRedirectors.registerAll(1)`: it uses `OptionValueRedirectorManager` (xaero.lib.common.config.option.value.redirect) (xaero/hud/minimap/common/config/option/value/redirect/MinimapConfigOptionServerRedirectors.java:6)

### `MinimapValueIOCodecs`

- [ ] `MinimapValueIOCodecs.ENTITY_RADAR_CATEGORIES`: it uses `LargeConfigValueIOCodec` (xaero.lib.common.config.option.value.io.serialization) (xaero/hud/minimap/common/config/option/value/type/io/serialization/MinimapValueIOCodecs.java:9)
- [ ] `MinimapValueIOCodecs.INFO_DISPLAY_CONFIG`: it uses `LargeConfigValueIOCodec` (xaero.lib.common.config.option.value.io.serialization) (xaero/hud/minimap/common/config/option/value/type/io/serialization/MinimapValueIOCodecs.java:15)

### `MinimapConfigValueTypes`

- [ ] `MinimapConfigValueTypes.ENTITY_RADAR_CATEGORIES`: it uses `ConfigValueType` (xaero.lib.common.config.option.value.type) (xaero/hud/minimap/common/config/option/value/type/MinimapConfigValueTypes.java:11)
- [ ] `MinimapConfigValueTypes.INFO_DISPLAY_CONFIG`: it uses `ConfigValueType` (xaero.lib.common.config.option.value.type) (xaero/hud/minimap/common/config/option/value/type/MinimapConfigValueTypes.java:15)

### `MinimapValueSyncCodecs`

- [ ] `MinimapValueSyncCodecs.ENTITY_RADAR_CATEGORIES`: it uses `ConfigValueSyncCodec` (xaero.lib.common.config.option.value.sync.serialization) (xaero/hud/minimap/common/config/option/value/type/sync/serialization/MinimapValueSyncCodecs.java:17)
- [ ] `MinimapValueSyncCodecs.INFO_DISPLAY_CONFIG`: it uses `ConfigValueSyncCodec` (xaero.lib.common.config.option.value.sync.serialization) (xaero/hud/minimap/common/config/option/value/type/sync/serialization/MinimapValueSyncCodecs.java:21)

### `MinimapPrimaryCommonConfigOptions`

- [ ] `MinimapPrimaryCommonConfigOptions.ALL`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/primary/option/MinimapPrimaryCommonConfigOptions.java:11)
- [ ] `MinimapPrimaryCommonConfigOptions.REGISTER_EFFECTS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/primary/option/MinimapPrimaryCommonConfigOptions.java:12)
- [ ] `MinimapPrimaryCommonConfigOptions.registerAll(1)`: it uses `ConfigOptionManager` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/primary/option/MinimapPrimaryCommonConfigOptions.java:16)

### `MinimapConfigOptionServerHandlers`

- [ ] `MinimapConfigOptionServerHandlers.registerAll(1)`: it uses `HandlerBasedConfigChangeListener` (xaero.lib.common.config.listener.handler) (xaero/hud/minimap/common/config/server/listener/handler/MinimapConfigOptionServerHandlers.java:6)

### `MinimapConfigUtils`

- [ ] `MinimapConfigUtils.getUIScaleDisplayGetter(3)`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/util/MinimapConfigUtils.java:12)
- [ ] `MinimapConfigUtils.getUIScaleDisplayGetter(4)`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/util/MinimapConfigUtils.java:16)
- [ ] `MinimapConfigUtils.getUIScaleDisplayGetter(2)`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/util/MinimapConfigUtils.java:38)
- [ ] `MinimapConfigUtils.getUIScaleDisplayGetter(3)`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/common/config/util/MinimapConfigUtils.java:42)
- [ ] `MinimapConfigUtils.getAutoMinimapSizeName(0)`: it uses Services.PLATFORM, which isn't translated (it uses IPlatformHelper, which isn't translated (it's an interface)) (xaero/hud/minimap/common/config/util/MinimapConfigUtils.java:46)

### `EntityRadarCategorySerializers`: it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)

- [ ] `EntityRadarCategorySerializers.EntityRadarCategorySerializers(2)`: its class isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)) (xaero/hud/minimap/common/radar/category/EntityRadarCategorySerializers.java:13)
- [ ] `EntityRadarCategorySerializers.getGson(0)`: its class isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)) (xaero/hud/minimap/common/radar/category/EntityRadarCategorySerializers.java:21)
- [ ] `EntityRadarCategorySerializers.getNbt(0)`: its class isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)) (xaero/hud/minimap/common/radar/category/EntityRadarCategorySerializers.java:25)

### `EntityRadarCategorySerializers.Builder`

- [x] `EntityRadarCategorySerializers.Builder.Builder(0)` (xaero/hud/minimap/common/radar/category/EntityRadarCategorySerializers.java:30)
- [ ] `EntityRadarCategorySerializers.Builder.setDefault(0)`: `this` used as a value (xaero/hud/minimap/common/radar/category/EntityRadarCategorySerializers.java:33)
- [ ] `EntityRadarCategorySerializers.Builder.build(0)`: it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)) (xaero/hud/minimap/common/radar/category/EntityRadarCategorySerializers.java:37)
- [ ] `EntityRadarCategorySerializers.Builder.begin(0)`: it calls Builder.setDefault, which isn't translated (xaero/hud/minimap/common/radar/category/EntityRadarCategorySerializers.java:48)

### `CardinalDirection`: its constants carry values

- [ ] `CardinalDirection.CardinalDirection(1)`: its class isn't translated (its constants carry values) (xaero/hud/minimap/compass/render/CardinalDirection.java:13)
- [ ] `CardinalDirection.getInitials(0)`: its class isn't translated (its constants carry values) (xaero/hud/minimap/compass/render/CardinalDirection.java:17)

### `CompassRenderer`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)

- [ ] `CompassRenderer.CompassRenderer(2)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/compass/render/CompassRenderer.java:22)
- [ ] `CompassRenderer.drawCompass(11)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/compass/render/CompassRenderer.java:28)

### `MinimapConfigOptionClientHandlers`

- [ ] `MinimapConfigOptionClientHandlers.resetImage(0)`: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/config/listener/handler/MinimapConfigOptionClientHandlers.java:14)
- [ ] `MinimapConfigOptionClientHandlers.handleSafeMode(1)`: it uses `Config` (xaero.lib.common.config) (xaero/hud/minimap/config/listener/handler/MinimapConfigOptionClientHandlers.java:21)
- [ ] `MinimapConfigOptionClientHandlers.handleMinimapItem(1)`: it uses `Config` (xaero.lib.common.config) (xaero/hud/minimap/config/listener/handler/MinimapConfigOptionClientHandlers.java:26)
- [ ] `MinimapConfigOptionClientHandlers.handleEntityRadarCategories(1)`: it uses `Config` (xaero.lib.common.config) (xaero/hud/minimap/config/listener/handler/MinimapConfigOptionClientHandlers.java:33)
- [ ] `MinimapConfigOptionClientHandlers.requestClaimsRefresh(0)`: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/config/listener/handler/MinimapConfigOptionClientHandlers.java:37)
- [ ] `MinimapConfigOptionClientHandlers.handleOpacClaims(1)`: it uses `Config` (xaero.lib.common.config) (xaero/hud/minimap/config/listener/handler/MinimapConfigOptionClientHandlers.java:47)
- [ ] `MinimapConfigOptionClientHandlers.handleOpacClaimsOpacity(1)`: it uses `Config` (xaero.lib.common.config) (xaero/hud/minimap/config/listener/handler/MinimapConfigOptionClientHandlers.java:51)
- [ ] `MinimapConfigOptionClientHandlers.handleLegibleCaveMaps(1)`: it uses `Config` (xaero.lib.common.config) (xaero/hud/minimap/config/listener/handler/MinimapConfigOptionClientHandlers.java:57)
- [ ] `MinimapConfigOptionClientHandlers.handleDisplayWorldMapChunks(1)`: it uses `Config` (xaero.lib.common.config) (xaero/hud/minimap/config/listener/handler/MinimapConfigOptionClientHandlers.java:61)
- [ ] `MinimapConfigOptionClientHandlers.handleBlockTransparency(1)`: it uses `Config` (xaero.lib.common.config) (xaero/hud/minimap/config/listener/handler/MinimapConfigOptionClientHandlers.java:65)
- [ ] `MinimapConfigOptionClientHandlers.handleDisplayStainedGlass(1)`: it uses `Config` (xaero.lib.common.config) (xaero/hud/minimap/config/listener/handler/MinimapConfigOptionClientHandlers.java:69)
- [ ] `MinimapConfigOptionClientHandlers.handleInfoDisplayConfig(1)`: it uses `Config` (xaero.lib.common.config) (xaero/hud/minimap/config/listener/handler/MinimapConfigOptionClientHandlers.java:73)
- [ ] `MinimapConfigOptionClientHandlers.registerAll(1)`: it uses `ClientConfigChangeListener` (xaero.lib.client.config.listener) (xaero/hud/minimap/config/listener/handler/MinimapConfigOptionClientHandlers.java:84)

### `MinimapConfigOptionUIRegister`

- [ ] `MinimapConfigOptionUIRegister.registerAll(1)`: it uses `ConfigOptionUITypeManager` (xaero.lib.client.config.option.ui) (xaero/hud/minimap/config/option/ui/MinimapConfigOptionUIRegister.java:10)

### `MinimapConfigOptionUITypes`

- [ ] `MinimapConfigOptionUITypes.RADAR_CATEGORIES_EDITOR`: it uses `ConfigOptionUIType` (xaero.lib.client.config.option.ui.type) (xaero/hud/minimap/config/option/ui/type/MinimapConfigOptionUITypes.java:15)
- [ ] `MinimapConfigOptionUITypes.INFO_DISPLAY_CONFIG_EDITOR`: it uses `ConfigOptionUIType` (xaero.lib.client.config.option.ui.type) (xaero/hud/minimap/config/option/ui/type/MinimapConfigOptionUITypes.java:28)

### `MinimapConfigOptionClientRedirectors`

- [x] `MinimapConfigOptionClientRedirectors.LEGACY_PLUGIN_TOOLTIP` in game_side.rs (xaero/hud/minimap/config/option/value/redirect/MinimapConfigOptionClientRedirectors.java:18)
- [x] `MinimapConfigOptionClientRedirectors.FAIRPLAY_TOOLTIP` in game_side.rs (xaero/hud/minimap/config/option/value/redirect/MinimapConfigOptionClientRedirectors.java:19)
- [x] `MinimapConfigOptionClientRedirectors.SAFE_MODE_TOOLTIP` in game_side.rs (xaero/hud/minimap/config/option/value/redirect/MinimapConfigOptionClientRedirectors.java:20)
- [x] `MinimapConfigOptionClientRedirectors.EFFECT_TOOLTIP` in game_side.rs (xaero/hud/minimap/config/option/value/redirect/MinimapConfigOptionClientRedirectors.java:21)
- [x] `MinimapConfigOptionClientRedirectors.FROM_WORLDMAP_TOOLTIP` in game_side.rs (xaero/hud/minimap/config/option/value/redirect/MinimapConfigOptionClientRedirectors.java:22)
- [x] `MinimapConfigOptionClientRedirectors.HARD_WORLDMAP_TOOLTIP` in game_side.rs (xaero/hud/minimap/config/option/value/redirect/MinimapConfigOptionClientRedirectors.java:23)
- [x] `MinimapConfigOptionClientRedirectors.FROM_WORLDMAP_SCREEN_TOOLTIP` in game_side.rs (xaero/hud/minimap/config/option/value/redirect/MinimapConfigOptionClientRedirectors.java:24)
- [x] `MinimapConfigOptionClientRedirectors.WORLD_MAP_COMPONENT` in game_side.rs (xaero/hud/minimap/config/option/value/redirect/MinimapConfigOptionClientRedirectors.java:25)
- [ ] `MinimapConfigOptionClientRedirectors.registerAll(1)`: it uses `ClientOptionValueRedirectorManager` (xaero.lib.client.config.option.value.redirect) (xaero/hud/minimap/config/option/value/redirect/MinimapConfigOptionClientRedirectors.java:27)

### `MinimapPrimaryClientConfigOptions`

- [ ] `MinimapPrimaryClientConfigOptions.ALL`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/config/primary/option/MinimapPrimaryClientConfigOptions.java:15)
- [ ] `MinimapPrimaryClientConfigOptions.UPDATE_NOTIFICATIONS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/config/primary/option/MinimapPrimaryClientConfigOptions.java:16)
- [ ] `MinimapPrimaryClientConfigOptions.IGNORED_UPDATE`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/config/primary/option/MinimapPrimaryClientConfigOptions.java:20)
- [ ] `MinimapPrimaryClientConfigOptions.WRONG_WORLD_TELEPORT`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/config/primary/option/MinimapPrimaryClientConfigOptions.java:26)
- [ ] `MinimapPrimaryClientConfigOptions.DIFFERENTIATE_BY_SERVER_ADDRESS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/config/primary/option/MinimapPrimaryClientConfigOptions.java:29)
- [ ] `MinimapPrimaryClientConfigOptions.DEBUG_ENTITY_ICONS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/config/primary/option/MinimapPrimaryClientConfigOptions.java:32)
- [ ] `MinimapPrimaryClientConfigOptions.DEBUG_ENTITY_VARIANT_IDS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/config/primary/option/MinimapPrimaryClientConfigOptions.java:34)
- [ ] `MinimapPrimaryClientConfigOptions.WAYPOINT_MUTUAL_EDIT`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/hud/minimap/config/primary/option/MinimapPrimaryClientConfigOptions.java:37)
- [ ] `MinimapPrimaryClientConfigOptions.registerAll(1)`: it uses `ConfigOptionManager` (xaero.lib.common.config.option) (xaero/hud/minimap/config/primary/option/MinimapPrimaryClientConfigOptions.java:40)

### `MinimapConfigClientUtils`

- [ ] `MinimapConfigClientUtils.getUIScale(2)`: it uses `ClientConfigManager` (xaero.lib.client.config) (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:28)
- [ ] `MinimapConfigClientUtils.getUIScale(3)`: it uses `ClientConfigManager` (xaero.lib.client.config) (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:33)
- [ ] `MinimapConfigClientUtils.getUIScale(5)`: it uses `ClientConfigManager` (xaero.lib.client.config) (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:38)
- [ ] `MinimapConfigClientUtils.getWaypointsClampDepth(3)`: it uses `ClientConfigManager` (xaero.lib.client.config) (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:44)
- [ ] `MinimapConfigClientUtils.addAutoUIScaleValueToComponent(2)`: Component.getSiblings (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:52)
- [ ] `MinimapConfigClientUtils.addAutoMCScaleValueToComponent(1)`: Component.getSiblings (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:56)
- [ ] `MinimapConfigClientUtils.getAutoMinimapSize(0)`: it calls Minecraft's Minecraft.getWindow (no Horizon helper for it yet) (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:60)
- [ ] `MinimapConfigClientUtils.getEffectiveMinimapSize(0)`: it uses `ClientConfigManager` (xaero.lib.client.config) (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:67)
- [ ] `MinimapConfigClientUtils.getEffectiveNorthLocked(2)`: it uses `ClientConfigManager` (xaero.lib.client.config) (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:73)
- [ ] `MinimapConfigClientUtils.changeZoomUnchecked(2)`: it uses `ClientConfigManager` (xaero.lib.client.config) (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:87)
- [ ] `MinimapConfigClientUtils.changeZoom(1)`: it uses `ClientConfigManager` (xaero.lib.client.config) (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:94)
- [ ] `MinimapConfigClientUtils.getEffectiveSlimeChunks(1)`: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:113)
- [ ] `MinimapConfigClientUtils.getEffectiveSlimeChunksSeed(1)`: it uses MinimapWorld, which isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:132)
- [ ] `MinimapConfigClientUtils.isFairPlayForCaveMode(0)`: it uses HudMod.INSTANCE, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:141)
- [ ] `MinimapConfigClientUtils.hasNoCaveModeEffect(0)`: it calls Misc.hasEffect, which isn't translated (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:161)
- [ ] `MinimapConfigClientUtils.getEffectiveCaveModeAllowed(0)`: it uses `ClientConfigManager` (xaero.lib.client.config) (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:166)
- [ ] `MinimapConfigClientUtils.getActualZoom(1)`: it's overloaded with the same number of parameters (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:191)
- [ ] `MinimapConfigClientUtils.getActualZoom(1)`: it's overloaded with the same number of parameters (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:197)
- [ ] `MinimapConfigClientUtils.areDiscoveredWaystonesEnabled(0)`: it uses `ClientConfigManager` (xaero.lib.client.config) (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:212)
- [ ] `MinimapConfigClientUtils.areOtherWaystonesEnabled(0)`: it uses `ClientConfigManager` (xaero.lib.client.config) (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java:217)

### `AddWaypointFunction`: it extends `KeyMappingFunction` (xaero.hud.controls.key.function)

- [ ] `AddWaypointFunction.AddWaypointFunction(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/AddWaypointFunction.java:12)
- [ ] `AddWaypointFunction.onPress(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/AddWaypointFunction.java:16)
- [ ] `AddWaypointFunction.onRelease(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/AddWaypointFunction.java:36)

### `HeldEnlargeMapFunction`: it extends `KeyMappingFunction` (xaero.hud.controls.key.function)

- [ ] `HeldEnlargeMapFunction.HeldEnlargeMapFunction(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/HeldEnlargeMapFunction.java:12)
- [ ] `HeldEnlargeMapFunction.onPress(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/HeldEnlargeMapFunction.java:16)
- [ ] `HeldEnlargeMapFunction.onRelease(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/HeldEnlargeMapFunction.java:29)

### `LightOverlayFunction`: it extends `KeyMappingFunction` (xaero.hud.controls.key.function)

- [ ] `LightOverlayFunction.LightOverlayFunction(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/LightOverlayFunction.java:10)
- [ ] `LightOverlayFunction.onPress(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/LightOverlayFunction.java:14)
- [ ] `LightOverlayFunction.onRelease(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/LightOverlayFunction.java:28)

### `ManualCaveModeFunction`: it extends `KeyMappingFunction` (xaero.hud.controls.key.function)

- [ ] `ManualCaveModeFunction.ManualCaveModeFunction(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ManualCaveModeFunction.java:7)
- [ ] `ManualCaveModeFunction.onPress(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ManualCaveModeFunction.java:11)
- [ ] `ManualCaveModeFunction.onRelease(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ManualCaveModeFunction.java:16)

### `MinimapKeyMappingFunctions`

- [ ] `MinimapKeyMappingFunctions.ZOOM_IN`: it creates a ZoomFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:9)
- [ ] `MinimapKeyMappingFunctions.ZOOM_OUT`: it creates a ZoomFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:10)
- [ ] `MinimapKeyMappingFunctions.TOGGLE_RADAR`: it creates a ToggleSettingFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:11)
- [ ] `MinimapKeyMappingFunctions.TOGGLE_TRACKED_PLAYER_MAP`: it creates a ToggleSettingFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:12)
- [ ] `MinimapKeyMappingFunctions.TOGGLE_TRACKED_PLAYER_WORLD`: it creates a ToggleSettingFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:13)
- [ ] `MinimapKeyMappingFunctions.ADD_WAYPOINT`: it creates a AddWaypointFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:14)
- [ ] `MinimapKeyMappingFunctions.WAYPOINT_MENU`: it creates a WaypointMenuFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:15)
- [ ] `MinimapKeyMappingFunctions.HELD_ENLARGE_MAP`: it creates a HeldEnlargeMapFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:16)
- [ ] `MinimapKeyMappingFunctions.TOGGLED_ENLARGE_MAP`: it creates a ToggledEnlargeMapFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:17)
- [ ] `MinimapKeyMappingFunctions.TOGGLE_MAP`: it creates a ToggleMapFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:18)
- [ ] `MinimapKeyMappingFunctions.TOGGLE_WORLD_WAYPOINTS`: it creates a ToggleSettingFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:19)
- [ ] `MinimapKeyMappingFunctions.TOGGLE_MAP_WAYPOINTS`: it creates a ToggleSettingFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:20)
- [ ] `MinimapKeyMappingFunctions.TOGGLE_SLIME_CHUNKS`: it creates a ToggleSlimeChunksFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:21)
- [ ] `MinimapKeyMappingFunctions.TOGGLE_GRID`: it creates a ToggleGridFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:22)
- [ ] `MinimapKeyMappingFunctions.TEMPORARY_WAYPOINT`: it creates a TemporaryWaypointFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:23)
- [ ] `MinimapKeyMappingFunctions.SWITCH_WAYPOINT_SET`: it creates a SwitchWaypointSetFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:24)
- [ ] `MinimapKeyMappingFunctions.RENDER_ALL_SETS`: it creates a ToggleSettingFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:25)
- [ ] `MinimapKeyMappingFunctions.LIGHT_OVERLAY`: it creates a LightOverlayFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:26)
- [ ] `MinimapKeyMappingFunctions.MANUAL_CAVE_MODE`: it creates a ManualCaveModeFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:27)
- [ ] `MinimapKeyMappingFunctions.TOGGLE_OPAC_CLAIMS`: it creates a OpacClaimsFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:28)
- [ ] `MinimapKeyMappingFunctions.registerAll(1)`: it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`) (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java:30)

### `OpacClaimsFunction`: it extends `KeyMappingFunction` (xaero.hud.controls.key.function)

- [ ] `OpacClaimsFunction.OpacClaimsFunction(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/OpacClaimsFunction.java:10)
- [ ] `OpacClaimsFunction.onPress(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/OpacClaimsFunction.java:14)
- [ ] `OpacClaimsFunction.onRelease(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/OpacClaimsFunction.java:27)

### `SwitchWaypointSetFunction`: it extends `KeyMappingFunction` (xaero.hud.controls.key.function)

- [ ] `SwitchWaypointSetFunction.SwitchWaypointSetFunction(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/SwitchWaypointSetFunction.java:17)
- [ ] `SwitchWaypointSetFunction.onPress(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/SwitchWaypointSetFunction.java:21)
- [ ] `SwitchWaypointSetFunction.onRelease(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/SwitchWaypointSetFunction.java:67)

### `TemporaryWaypointFunction`: it extends `KeyMappingFunction` (xaero.hud.controls.key.function)

- [ ] `TemporaryWaypointFunction.TemporaryWaypointFunction(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/TemporaryWaypointFunction.java:12)
- [ ] `TemporaryWaypointFunction.onPress(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/TemporaryWaypointFunction.java:16)
- [ ] `TemporaryWaypointFunction.onRelease(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/TemporaryWaypointFunction.java:32)

### `ToggledEnlargeMapFunction`: it extends `KeyMappingFunction` (xaero.hud.controls.key.function)

- [ ] `ToggledEnlargeMapFunction.ToggledEnlargeMapFunction(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ToggledEnlargeMapFunction.java:10)
- [ ] `ToggledEnlargeMapFunction.onPress(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ToggledEnlargeMapFunction.java:14)
- [ ] `ToggledEnlargeMapFunction.onRelease(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ToggledEnlargeMapFunction.java:24)

### `ToggleGridFunction`: it extends `KeyMappingFunction` (xaero.hud.controls.key.function)

- [ ] `ToggleGridFunction.ToggleGridFunction(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ToggleGridFunction.java:10)
- [ ] `ToggleGridFunction.onPress(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ToggleGridFunction.java:14)
- [ ] `ToggleGridFunction.onRelease(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ToggleGridFunction.java:22)

### `ToggleMapFunction`: it extends `KeyMappingFunction` (xaero.hud.controls.key.function)

- [ ] `ToggleMapFunction.ToggleMapFunction(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ToggleMapFunction.java:12)
- [ ] `ToggleMapFunction.onPress(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ToggleMapFunction.java:16)
- [ ] `ToggleMapFunction.onRelease(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ToggleMapFunction.java:27)

### `ToggleSettingFunction`: it extends `KeyMappingFunction` (xaero.hud.controls.key.function)

- [ ] `ToggleSettingFunction.ToggleSettingFunction(1)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ToggleSettingFunction.java:13)
- [ ] `ToggleSettingFunction.onPress(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ToggleSettingFunction.java:18)
- [ ] `ToggleSettingFunction.onRelease(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ToggleSettingFunction.java:27)

### `ToggleSlimeChunksFunction`: it extends `KeyMappingFunction` (xaero.hud.controls.key.function)

- [ ] `ToggleSlimeChunksFunction.ToggleSlimeChunksFunction(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ToggleSlimeChunksFunction.java:17)
- [ ] `ToggleSlimeChunksFunction.onPress(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ToggleSlimeChunksFunction.java:21)
- [ ] `ToggleSlimeChunksFunction.onRelease(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ToggleSlimeChunksFunction.java:39)

### `WaypointMenuFunction`: it extends `KeyMappingFunction` (xaero.hud.controls.key.function)

- [ ] `WaypointMenuFunction.WaypointMenuFunction(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/WaypointMenuFunction.java:13)
- [ ] `WaypointMenuFunction.onPress(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/WaypointMenuFunction.java:17)
- [ ] `WaypointMenuFunction.onRelease(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/WaypointMenuFunction.java:28)

### `ZoomFunction`: it extends `KeyMappingFunction` (xaero.hud.controls.key.function)

- [ ] `ZoomFunction.ZoomFunction(1)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ZoomFunction.java:15)
- [ ] `ZoomFunction.onPress(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ZoomFunction.java:20)
- [ ] `ZoomFunction.onRelease(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/minimap/controls/key/function/ZoomFunction.java:34)

### `MinimapKeyMappings`

- [ ] `MinimapKeyMappings.CATEGORY`: it uses Minecraft's `Category` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:12)
- [ ] `MinimapKeyMappings.ZOOM_IN`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:13)
- [ ] `MinimapKeyMappings.ZOOM_OUT`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:14)
- [ ] `MinimapKeyMappings.ADD_WAYPOINT`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:15)
- [ ] `MinimapKeyMappings.WAYPOINT_MENU`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:16)
- [ ] `MinimapKeyMappings.ENLARGE_MAP`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:17)
- [ ] `MinimapKeyMappings.TOGGLE_MAP`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:18)
- [ ] `MinimapKeyMappings.TOGGLE_WORLD_WAYPOINTS`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:19)
- [ ] `MinimapKeyMappings.TOGGLE_MAP_WAYPOINTS`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:20)
- [ ] `MinimapKeyMappings.TOGGLE_SLIME_CHUNKS`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:21)
- [ ] `MinimapKeyMappings.TOGGLE_GRID`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:22)
- [ ] `MinimapKeyMappings.TEMPORARY_WAYPOINT`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:23)
- [ ] `MinimapKeyMappings.SWITCH_WAYPOINT_SET`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:24)
- [ ] `MinimapKeyMappings.RENDER_ALL_SETS`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:25)
- [ ] `MinimapKeyMappings.LIGHT_OVERLAY`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:26)
- [ ] `MinimapKeyMappings.TOGGLE_RADAR`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:27)
- [ ] `MinimapKeyMappings.REVERSE_ENTITY_RADAR`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:28)
- [ ] `MinimapKeyMappings.MANUAL_CAVE_MODE`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:29)
- [ ] `MinimapKeyMappings.ALTERNATIVE_LIST_PLAYERS`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:30)
- [ ] `MinimapKeyMappings.TOGGLE_TRACKED_PLAYERS_MAP`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:31)
- [ ] `MinimapKeyMappings.TOGGLE_TRACKED_PLAYERS_WORLD`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:34)
- [ ] `MinimapKeyMappings.TOGGLE_OPAC_CLAIMS`: it uses Minecraft's `KeyMapping` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:37)
- [ ] `MinimapKeyMappings.registerAll(3)`: it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`) (xaero/hud/minimap/controls/key/MinimapKeyMappings.java:39)

### `MinimapElementMapRendererHandler`: it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)

- [ ] `MinimapElementMapRendererHandler.MinimapElementMapRendererHandler(3)`: its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/element/render/map/MinimapElementMapRendererHandler.java:20)
- [ ] `MinimapElementMapRendererHandler.prepareRender(4)`: its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/element/render/map/MinimapElementMapRendererHandler.java:24)
- [ ] `MinimapElementMapRendererHandler.transformAndRenderForRenderer(10)`: it's generic (xaero/hud/minimap/element/render/map/MinimapElementMapRendererHandler.java:31)
- [ ] `MinimapElementMapRendererHandler.beforeRender(2)`: its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/element/render/map/MinimapElementMapRendererHandler.java:66)
- [ ] `MinimapElementMapRendererHandler.afterRender(2)`: its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/element/render/map/MinimapElementMapRendererHandler.java:70)

### `MinimapElementMapRendererHandler.Builder`: it extends `Builder`

- [ ] `MinimapElementMapRendererHandler.Builder.build(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/element/render/map/MinimapElementMapRendererHandler.java:75)
- [ ] `MinimapElementMapRendererHandler.Builder.buildInternally(2)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/element/render/map/MinimapElementMapRendererHandler.java:79)
- [ ] `MinimapElementMapRendererHandler.Builder.setDefault(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/element/render/map/MinimapElementMapRendererHandler.java:83)
- [ ] `MinimapElementMapRendererHandler.Builder.begin(0)`: it uses Builder, which isn't translated (it extends `Builder`) (xaero/hud/minimap/element/render/map/MinimapElementMapRendererHandler.java:88)

### `MinimapElementGraphics`: it uses `XaeroBufferProvider` (xaero.lib.client.graphics)

- [ ] `MinimapElementGraphics.MinimapElementGraphics(1)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/hud/minimap/element/render/MinimapElementGraphics.java:29)
- [ ] `MinimapElementGraphics.fill(5)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/hud/minimap/element/render/MinimapElementGraphics.java:33)
- [ ] `MinimapElementGraphics.fill(6)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/hud/minimap/element/render/MinimapElementGraphics.java:37)
- [ ] `MinimapElementGraphics.fillGradient(6)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/hud/minimap/element/render/MinimapElementGraphics.java:54)
- [ ] `MinimapElementGraphics.blit(6)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/hud/minimap/element/render/MinimapElementGraphics.java:59)
- [ ] `MinimapElementGraphics.blit(11)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/hud/minimap/element/render/MinimapElementGraphics.java:80)
- [ ] `MinimapElementGraphics.blit(11)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/hud/minimap/element/render/MinimapElementGraphics.java:103)
- [ ] `MinimapElementGraphics.drawCenteredString(5)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/hud/minimap/element/render/MinimapElementGraphics.java:111)
- [ ] `MinimapElementGraphics.drawCenteredString(5)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/hud/minimap/element/render/MinimapElementGraphics.java:115)
- [ ] `MinimapElementGraphics.drawCenteredString(5)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/hud/minimap/element/render/MinimapElementGraphics.java:119)
- [ ] `MinimapElementGraphics.drawString(5)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/hud/minimap/element/render/MinimapElementGraphics.java:123)
- [ ] `MinimapElementGraphics.drawString(6)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/hud/minimap/element/render/MinimapElementGraphics.java:127)
- [ ] `MinimapElementGraphics.drawString(5)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/hud/minimap/element/render/MinimapElementGraphics.java:135)
- [ ] `MinimapElementGraphics.drawString(6)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/hud/minimap/element/render/MinimapElementGraphics.java:139)
- [ ] `MinimapElementGraphics.drawString(5)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/hud/minimap/element/render/MinimapElementGraphics.java:147)
- [ ] `MinimapElementGraphics.drawString(6)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/hud/minimap/element/render/MinimapElementGraphics.java:151)
- [ ] `MinimapElementGraphics.getBufferSource(0)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/hud/minimap/element/render/MinimapElementGraphics.java:159)
- [ ] `MinimapElementGraphics.pose(0)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/hud/minimap/element/render/MinimapElementGraphics.java:163)
- [ ] `MinimapElementGraphics.flush(0)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/hud/minimap/element/render/MinimapElementGraphics.java:167)

### `MinimapElementReader`: it's generic

- [ ] `MinimapElementReader.isHidden(2)`: it has no body (xaero/hud/minimap/element/render/MinimapElementReader.java:6)
- [ ] `MinimapElementReader.getRenderX(3)`: it has no body (xaero/hud/minimap/element/render/MinimapElementReader.java:8)
- [ ] `MinimapElementReader.getRenderY(3)`: it has no body (xaero/hud/minimap/element/render/MinimapElementReader.java:10)
- [ ] `MinimapElementReader.getRenderZ(3)`: it has no body (xaero/hud/minimap/element/render/MinimapElementReader.java:12)
- [ ] `MinimapElementReader.getCoordinateScale(3)`: its class isn't translated (it's generic) (xaero/hud/minimap/element/render/MinimapElementReader.java:14)
- [ ] `MinimapElementReader.shouldScalePartialCoordinates(3)`: its class isn't translated (it's generic) (xaero/hud/minimap/element/render/MinimapElementReader.java:18)
- [ ] `MinimapElementReader.getInteractionBoxLeft(3)`: it has no body (xaero/hud/minimap/element/render/MinimapElementReader.java:22)
- [ ] `MinimapElementReader.getInteractionBoxRight(3)`: it has no body (xaero/hud/minimap/element/render/MinimapElementReader.java:24)
- [ ] `MinimapElementReader.getInteractionBoxTop(3)`: it has no body (xaero/hud/minimap/element/render/MinimapElementReader.java:26)
- [ ] `MinimapElementReader.getInteractionBoxBottom(3)`: it has no body (xaero/hud/minimap/element/render/MinimapElementReader.java:28)
- [ ] `MinimapElementReader.getRenderBoxLeft(3)`: it has no body (xaero/hud/minimap/element/render/MinimapElementReader.java:30)
- [ ] `MinimapElementReader.getRenderBoxRight(3)`: it has no body (xaero/hud/minimap/element/render/MinimapElementReader.java:32)
- [ ] `MinimapElementReader.getRenderBoxTop(3)`: it has no body (xaero/hud/minimap/element/render/MinimapElementReader.java:34)
- [ ] `MinimapElementReader.getRenderBoxBottom(3)`: it has no body (xaero/hud/minimap/element/render/MinimapElementReader.java:36)
- [ ] `MinimapElementReader.getLeftSideLength(2)`: it has no body (xaero/hud/minimap/element/render/MinimapElementReader.java:38)
- [ ] `MinimapElementReader.getMenuName(1)`: it has no body (xaero/hud/minimap/element/render/MinimapElementReader.java:40)
- [ ] `MinimapElementReader.getFilterName(1)`: it has no body (xaero/hud/minimap/element/render/MinimapElementReader.java:42)
- [ ] `MinimapElementReader.getMenuTextFillLeftPadding(1)`: it has no body (xaero/hud/minimap/element/render/MinimapElementReader.java:44)
- [ ] `MinimapElementReader.getRightClickTitleBackgroundColor(1)`: it has no body (xaero/hud/minimap/element/render/MinimapElementReader.java:46)
- [ ] `MinimapElementReader.shouldScaleBoxWithOptionalScale(0)`: it has no body (xaero/hud/minimap/element/render/MinimapElementReader.java:48)
- [ ] `MinimapElementReader.isInteractable(2)`: its class isn't translated (it's generic) (xaero/hud/minimap/element/render/MinimapElementReader.java:50)
- [ ] `MinimapElementReader.getBoxScale(3)`: its class isn't translated (it's generic) (xaero/hud/minimap/element/render/MinimapElementReader.java:54)
- [ ] `MinimapElementReader.isAlwaysHighlightedWhenHovered(2)`: its class isn't translated (it's generic) (xaero/hud/minimap/element/render/MinimapElementReader.java:58)

### `MinimapElementRenderer`: it's generic

- [ ] `MinimapElementRenderer.MinimapElementRenderer(3)`: its class isn't translated (it's generic) (xaero/hud/minimap/element/render/MinimapElementRenderer.java:11)
- [ ] `MinimapElementRenderer.getOrder(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/element/render/MinimapElementRenderer.java:17)
- [ ] `MinimapElementRenderer.compareTo(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/element/render/MinimapElementRenderer.java:21)
- [ ] `MinimapElementRenderer.getContext(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/element/render/MinimapElementRenderer.java:25)
- [ ] `MinimapElementRenderer.getProvider(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/element/render/MinimapElementRenderer.java:29)
- [ ] `MinimapElementRenderer.getElementReader(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/element/render/MinimapElementRenderer.java:33)
- [ ] `MinimapElementRenderer.renderElement(10)`: it has no body (xaero/hud/minimap/element/render/MinimapElementRenderer.java:37)
- [ ] `MinimapElementRenderer.preRender(3)`: it has no body (xaero/hud/minimap/element/render/MinimapElementRenderer.java:50)
- [ ] `MinimapElementRenderer.postRender(3)`: it has no body (xaero/hud/minimap/element/render/MinimapElementRenderer.java:52)
- [ ] `MinimapElementRenderer.shouldRender(1)`: it has no body (xaero/hud/minimap/element/render/MinimapElementRenderer.java:54)

### `MinimapElementRendererHandler`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)

- [ ] `MinimapElementRendererHandler.MinimapElementRendererHandler(5)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/element/render/MinimapElementRendererHandler.java:29)
- [ ] `MinimapElementRendererHandler.add(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/element/render/MinimapElementRendererHandler.java:39)
- [ ] `MinimapElementRendererHandler.render(5)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/element/render/MinimapElementRendererHandler.java:44)
- [ ] `MinimapElementRendererHandler.renderForRenderer(5)`: it's generic (xaero/hud/minimap/element/render/MinimapElementRendererHandler.java:74)
- [ ] `MinimapElementRendererHandler.transformAndRenderForRenderer(7)`: it's generic (xaero/hud/minimap/element/render/MinimapElementRendererHandler.java:108)
- [ ] `MinimapElementRendererHandler.getElementIndexDepth(2)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/element/render/MinimapElementRendererHandler.java:138)
- [ ] `MinimapElementRendererHandler.getGuiGraphics(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/element/render/MinimapElementRendererHandler.java:142)
- [ ] `MinimapElementRendererHandler.getIndexLimit(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/element/render/MinimapElementRendererHandler.java:146)
- [ ] `MinimapElementRendererHandler.transformAndRenderForRenderer(10)`: it has no body (xaero/hud/minimap/element/render/MinimapElementRendererHandler.java:150)
- [ ] `MinimapElementRendererHandler.beforeRender(2)`: it has no body (xaero/hud/minimap/element/render/MinimapElementRendererHandler.java:154)
- [ ] `MinimapElementRendererHandler.afterRender(2)`: it has no body (xaero/hud/minimap/element/render/MinimapElementRendererHandler.java:156)

### `MinimapElementRendererHandler.Builder`: it's generic

- [ ] `MinimapElementRendererHandler.Builder.Builder(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/element/render/MinimapElementRendererHandler.java:162)
- [ ] `MinimapElementRendererHandler.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/element/render/MinimapElementRendererHandler.java:165)
- [ ] `MinimapElementRendererHandler.Builder.setPoseStack(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/element/render/MinimapElementRendererHandler.java:170)
- [ ] `MinimapElementRendererHandler.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/element/render/MinimapElementRendererHandler.java:175)
- [ ] `MinimapElementRendererHandler.Builder.buildInternally(2)`: it has no body (xaero/hud/minimap/element/render/MinimapElementRendererHandler.java:183)

### `MinimapElementRenderInfo`: it uses Minecraft's `Entity`

- [ ] `MinimapElementRenderInfo.MinimapElementRenderInfo(9)`: its class isn't translated (it uses Minecraft's `Entity`) (xaero/hud/minimap/element/render/MinimapElementRenderInfo.java:26)

### `MinimapElementRenderLocation`

- [x] `MinimapElementRenderLocation.UNKNOWN` (xaero/hud/minimap/element/render/MinimapElementRenderLocation.java:4)
- [x] `MinimapElementRenderLocation.IN_MINIMAP` (xaero/hud/minimap/element/render/MinimapElementRenderLocation.java:5)
- [x] `MinimapElementRenderLocation.OVER_MINIMAP` (xaero/hud/minimap/element/render/MinimapElementRenderLocation.java:6)
- [x] `MinimapElementRenderLocation.IN_WORLD` (xaero/hud/minimap/element/render/MinimapElementRenderLocation.java:7)
- [x] `MinimapElementRenderLocation.WORLD_MAP` (xaero/hud/minimap/element/render/MinimapElementRenderLocation.java:8)
- [x] `MinimapElementRenderLocation.WORLD_MAP_MENU` (xaero/hud/minimap/element/render/MinimapElementRenderLocation.java:9)
- [x] `MinimapElementRenderLocation.MinimapElementRenderLocation(1)` (xaero/hud/minimap/element/render/MinimapElementRenderLocation.java:12)
- [x] `MinimapElementRenderLocation.getName(0)` (xaero/hud/minimap/element/render/MinimapElementRenderLocation.java:16)

### `MinimapElementRenderProvider`: it's generic

- [ ] `MinimapElementRenderProvider.begin(2)`: it has no body (xaero/hud/minimap/element/render/MinimapElementRenderProvider.java:4)
- [ ] `MinimapElementRenderProvider.hasNext(2)`: it has no body (xaero/hud/minimap/element/render/MinimapElementRenderProvider.java:6)
- [ ] `MinimapElementRenderProvider.getNext(2)`: it has no body (xaero/hud/minimap/element/render/MinimapElementRenderProvider.java:8)
- [ ] `MinimapElementRenderProvider.setupContextAndGetNext(2)`: its class isn't translated (it's generic) (xaero/hud/minimap/element/render/MinimapElementRenderProvider.java:10)
- [ ] `MinimapElementRenderProvider.end(2)`: it has no body (xaero/hud/minimap/element/render/MinimapElementRenderProvider.java:14)

### `MinimapElementOverMapRendererHandler`: it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)

- [ ] `MinimapElementOverMapRendererHandler.MinimapElementOverMapRendererHandler(4)`: its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/element/render/over/MinimapElementOverMapRendererHandler.java:26)
- [ ] `MinimapElementOverMapRendererHandler.prepareRender(9)`: its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/element/render/over/MinimapElementOverMapRendererHandler.java:33)
- [ ] `MinimapElementOverMapRendererHandler.transformAndRenderForRenderer(10)`: it's generic (xaero/hud/minimap/element/render/over/MinimapElementOverMapRendererHandler.java:45)
- [ ] `MinimapElementOverMapRendererHandler.beforeRender(2)`: its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/element/render/over/MinimapElementOverMapRendererHandler.java:82)
- [ ] `MinimapElementOverMapRendererHandler.afterRender(2)`: its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/element/render/over/MinimapElementOverMapRendererHandler.java:86)
- [x] `MinimapElementOverMapRendererHandler.translatePosition(12)` (xaero/hud/minimap/element/render/over/MinimapElementOverMapRendererHandler.java:90)

### `MinimapElementOverMapRendererHandler.Builder`: it extends `Builder`

- [ ] `MinimapElementOverMapRendererHandler.Builder.build(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/element/render/over/MinimapElementOverMapRendererHandler.java:157)
- [ ] `MinimapElementOverMapRendererHandler.Builder.buildInternally(2)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/element/render/over/MinimapElementOverMapRendererHandler.java:161)
- [ ] `MinimapElementOverMapRendererHandler.Builder.setDefault(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/element/render/over/MinimapElementOverMapRendererHandler.java:165)
- [ ] `MinimapElementOverMapRendererHandler.Builder.begin(0)`: it uses Builder, which isn't translated (it extends `Builder`) (xaero/hud/minimap/element/render/over/MinimapElementOverMapRendererHandler.java:170)

### `MinimapElementWorldRendererHandler`: it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)

- [x] `MinimapElementWorldRendererHandler.DEFAULT_SCALE` (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java:30)
- [x] `MinimapElementWorldRendererHandler.MINECRAFT_SCALE` (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java:31)
- [x] `MinimapElementWorldRendererHandler.ELEMENT_WORLD_SCALE` (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java:32)
- [ ] `MinimapElementWorldRendererHandler.MinimapElementWorldRendererHandler(5)`: its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java:49)
- [ ] `MinimapElementWorldRendererHandler.prepareRender(2)`: its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java:57)
- [ ] `MinimapElementWorldRendererHandler.render(5)`: its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java:62)
- [ ] `MinimapElementWorldRendererHandler.transformAndRenderForRenderer(10)`: it's generic (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java:70)
- [ ] `MinimapElementWorldRendererHandler.isElementHovered(6)`: it's generic (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java:138)
- [ ] `MinimapElementWorldRendererHandler.handleClosestHovered(4)`: it's generic (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java:167)
- [ ] `MinimapElementWorldRendererHandler.renderMainHighlightedElement(2)`: it's generic (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java:182)
- [ ] `MinimapElementWorldRendererHandler.beforeRender(2)`: its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java:203)
- [ ] `MinimapElementWorldRendererHandler.afterRender(2)`: its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java:214)
- [ ] `MinimapElementWorldRendererHandler.ensureOrthoProjectionCache(0)`: its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java:227)
- [ ] `MinimapElementWorldRendererHandler.getOrthoProjection(2)`: its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java:234)

### `MinimapElementWorldRendererHandler.Builder`: it extends `Builder`

- [ ] `MinimapElementWorldRendererHandler.Builder.buildInternally(2)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java:241)
- [ ] `MinimapElementWorldRendererHandler.Builder.build(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java:246)
- [ ] `MinimapElementWorldRendererHandler.Builder.setDefault(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java:250)
- [ ] `MinimapElementWorldRendererHandler.Builder.begin(0)`: it uses Builder, which isn't translated (it extends `Builder`) (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java:254)

### `BuiltInInfoDisplays`

- [ ] `BuiltInInfoDisplays.ALL`: it uses InfoDisplay, which isn't translated (it's generic) (xaero/hud/minimap/info/BuiltInInfoDisplays.java:27)
- [ ] `BuiltInInfoDisplays.COORDINATES`: it uses InfoDisplay, which isn't translated (it's generic) (xaero/hud/minimap/info/BuiltInInfoDisplays.java:28)
- [ ] `BuiltInInfoDisplays.OVERWORLD_COORDINATES`: it uses InfoDisplay, which isn't translated (it's generic) (xaero/hud/minimap/info/BuiltInInfoDisplays.java:54)
- [ ] `BuiltInInfoDisplays.CHUNK_COORDINATES`: it uses InfoDisplay, which isn't translated (it's generic) (xaero/hud/minimap/info/BuiltInInfoDisplays.java:74)
- [ ] `BuiltInInfoDisplays.ANGLES`: it uses InfoDisplay, which isn't translated (it's generic) (xaero/hud/minimap/info/BuiltInInfoDisplays.java:92)
- [ ] `BuiltInInfoDisplays.DIMENSION`: it uses InfoDisplay, which isn't translated (it's generic) (xaero/hud/minimap/info/BuiltInInfoDisplays.java:117)
- [ ] `BuiltInInfoDisplays.BIOME`: it uses InfoDisplay, which isn't translated (it's generic) (xaero/hud/minimap/info/BuiltInInfoDisplays.java:139)
- [ ] `BuiltInInfoDisplays.WEATHER`: it uses InfoDisplay, which isn't translated (it's generic) (xaero/hud/minimap/info/BuiltInInfoDisplays.java:165)
- [ ] `BuiltInInfoDisplays.LIGHT_LEVEL`: it uses InfoDisplay, which isn't translated (it's generic) (xaero/hud/minimap/info/BuiltInInfoDisplays.java:187)
- [ ] `BuiltInInfoDisplays.TIME`: it uses InfoDisplay, which isn't translated (it's generic) (xaero/hud/minimap/info/BuiltInInfoDisplays.java:230)
- [ ] `BuiltInInfoDisplays.CALENDAR`: it uses `Calendar` (java.util) (xaero/hud/minimap/info/BuiltInInfoDisplays.java:290)
- [ ] `BuiltInInfoDisplays.REAL_TIME`: it uses InfoDisplay, which isn't translated (it's generic) (xaero/hud/minimap/info/BuiltInInfoDisplays.java:291)
- [ ] `BuiltInInfoDisplays.HIGHLIGHTS`: it uses InfoDisplay, which isn't translated (it's generic) (xaero/hud/minimap/info/BuiltInInfoDisplays.java:329)
- [ ] `BuiltInInfoDisplays.LIGHT_OVERLAY_INDICATOR`: it uses InfoDisplay, which isn't translated (it's generic) (xaero/hud/minimap/info/BuiltInInfoDisplays.java:349)
- [ ] `BuiltInInfoDisplays.MANUAL_CAVE_MODE_INDICATOR`: it uses InfoDisplay, which isn't translated (it's generic) (xaero/hud/minimap/info/BuiltInInfoDisplays.java:364)
- [ ] `BuiltInInfoDisplays.CUSTOM_SUB_WORLD`: it uses InfoDisplay, which isn't translated (it's generic) (xaero/hud/minimap/info/BuiltInInfoDisplays.java:379)
- [ ] `BuiltInInfoDisplays.forEach(1)`: it uses InfoDisplay, which isn't translated (it's generic) (xaero/hud/minimap/info/BuiltInInfoDisplays.java:398)

### `InfoDisplayCommonStateCodecs`

- [ ] `InfoDisplayCommonStateCodecs.BOOLEAN`: it uses InfoDisplayStateCodec, which isn't translated (it's generic) (xaero/hud/minimap/info/codec/InfoDisplayCommonStateCodecs.java:6)
- [ ] `InfoDisplayCommonStateCodecs.INTEGER`: it uses InfoDisplayStateCodec, which isn't translated (it's generic) (xaero/hud/minimap/info/codec/InfoDisplayCommonStateCodecs.java:8)

### `InfoDisplayStateCodec`: it's generic

- [ ] `InfoDisplayStateCodec.InfoDisplayStateCodec(3)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/codec/InfoDisplayStateCodec.java:8)

### `InfoDisplayConfigClientUtils`

- [ ] `InfoDisplayConfigClientUtils.createDefaultConfig(3)`: it's generic (xaero/hud/minimap/info/config/InfoDisplayConfigClientUtils.java:14)
- [ ] `InfoDisplayConfigClientUtils.createDefaultConfig(3)`: it's generic (xaero/hud/minimap/info/config/InfoDisplayConfigClientUtils.java:23)

### `InfoDisplay`: it's generic

- [ ] `InfoDisplay.InfoDisplay(7)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:28)
- [ ] `InfoDisplay.setManager(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:46)
- [ ] `InfoDisplay.getState(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:54)
- [ ] `InfoDisplay.getEffectiveState(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:59)
- [ ] `InfoDisplay.getTextColor(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:68)
- [ ] `InfoDisplay.getBackgroundColor(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:77)
- [ ] `InfoDisplay.updateConfigCache(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:86)
- [ ] `InfoDisplay.getId(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:92)
- [ ] `InfoDisplay.getName(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:96)
- [ ] `InfoDisplay.getDefaultState(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:100)
- [ ] `InfoDisplay.getCodec(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:104)
- [ ] `InfoDisplay.createWidget(7)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:108)
- [ ] `InfoDisplay.getCompiler(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:112)
- [ ] `InfoDisplay.clearStateCache(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:116)
- [ ] `InfoDisplay.getLegacyValue(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:120)

### `InfoDisplay.Builder`: it's generic

- [ ] `InfoDisplay.Builder.Builder(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:134)
- [ ] `InfoDisplay.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:137)
- [ ] `InfoDisplay.Builder.setId(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:149)
- [ ] `InfoDisplay.Builder.setName(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:154)
- [ ] `InfoDisplay.Builder.setDefaultState(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:159)
- [ ] `InfoDisplay.Builder.setCodec(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:164)
- [ ] `InfoDisplay.Builder.setCodec(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:169)
- [ ] `InfoDisplay.Builder.setWidgetFactory(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:174)
- [ ] `InfoDisplay.Builder.setCompiler(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:179)
- [ ] `InfoDisplay.Builder.setDestination(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:184)
- [ ] `InfoDisplay.Builder.setLegacyGetter(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:189)
- [ ] `InfoDisplay.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/InfoDisplay.java:194)
- [ ] `InfoDisplay.Builder.begin(0)`: it's generic (xaero/hud/minimap/info/InfoDisplay.java:207)

### `InfoDisplay.ConfigCache`: it uses `T`

- [ ] `InfoDisplay.ConfigCache.BG_COLOR_GETTER`: a function kept in a variable or field (only parameters can be) (xaero/hud/minimap/info/InfoDisplay.java:213)
- [ ] `InfoDisplay.ConfigCache.TEXT_COLOR_GETTER`: a function kept in a variable or field (only parameters can be) (xaero/hud/minimap/info/InfoDisplay.java:214)
- [ ] `InfoDisplay.ConfigCache.ConfigCache(0)`: its class isn't translated (it uses `T`) (xaero/hud/minimap/info/InfoDisplay.java:219)
- [ ] `InfoDisplay.ConfigCache.fetchStateFromConfig(0)`: its class isn't translated (it uses `T`) (xaero/hud/minimap/info/InfoDisplay.java:222)
- [ ] `InfoDisplay.ConfigCache.fetchBackgroundColorFromConfig(0)`: its class isn't translated (it uses `T`) (xaero/hud/minimap/info/InfoDisplay.java:249)
- [ ] `InfoDisplay.ConfigCache.fetchTextColorFromConfig(0)`: its class isn't translated (it uses `T`) (xaero/hud/minimap/info/InfoDisplay.java:253)
- [ ] `InfoDisplay.ConfigCache.fetchColorFromConfig(2)`: its class isn't translated (it uses `T`) (xaero/hud/minimap/info/InfoDisplay.java:257)

### `InfoDisplayIO`

- [ ] `InfoDisplayIO.encode(1)`: it calls InfoDisplayManagerConfigData.getOrderStream, which isn't translated (xaero/hud/minimap/info/InfoDisplayIO.java:12)
- [ ] `InfoDisplayIO.getInfoDisplayLine(2)`: `null` (Rust has no null: this needs an Option) (xaero/hud/minimap/info/InfoDisplayIO.java:31)
- [ ] `InfoDisplayIO.decode(1)`: it calls InfoDisplayIO.loadInfoDisplay, which isn't translated (xaero/hud/minimap/info/InfoDisplayIO.java:38)
- [x] `InfoDisplayIO.loadInfoDisplayOrderLine(2)` (xaero/hud/minimap/info/InfoDisplayIO.java:54)
- [ ] `InfoDisplayIO.loadInfoDisplay(2)`: `null` (Rust has no null: this needs an Option) (xaero/hud/minimap/info/InfoDisplayIO.java:64)

### `InfoDisplayManager`: it uses InfoDisplay, which isn't translated (it's generic)

- [ ] `InfoDisplayManager.InfoDisplayManager(6)`: its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)) (xaero/hud/minimap/info/InfoDisplayManager.java:26)
- [ ] `InfoDisplayManager.add(1)`: its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)) (xaero/hud/minimap/info/InfoDisplayManager.java:42)
- [ ] `InfoDisplayManager.applyLocalConfig(0)`: its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)) (xaero/hud/minimap/info/InfoDisplayManager.java:49)
- [ ] `InfoDisplayManager.adaptOrder(1)`: its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)) (xaero/hud/minimap/info/InfoDisplayManager.java:62)
- [ ] `InfoDisplayManager.getLocalConfig(0)`: its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)) (xaero/hud/minimap/info/InfoDisplayManager.java:86)
- [ ] `InfoDisplayManager.getEnforcedConfig(0)`: its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)) (xaero/hud/minimap/info/InfoDisplayManager.java:90)
- [ ] `InfoDisplayManager.get(1)`: its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)) (xaero/hud/minimap/info/InfoDisplayManager.java:94)
- [ ] `InfoDisplayManager.getOrderedStream(0)`: its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)) (xaero/hud/minimap/info/InfoDisplayManager.java:98)
- [ ] `InfoDisplayManager.getStream(0)`: its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)) (xaero/hud/minimap/info/InfoDisplayManager.java:103)
- [ ] `InfoDisplayManager.getCount(0)`: its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)) (xaero/hud/minimap/info/InfoDisplayManager.java:107)
- [ ] `InfoDisplayManager.getDefaultOrder(0)`: its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)) (xaero/hud/minimap/info/InfoDisplayManager.java:111)
- [ ] `InfoDisplayManager.clearStateCache(0)`: its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)) (xaero/hud/minimap/info/InfoDisplayManager.java:115)

### `InfoDisplayManager.Builder`: a function kept in a variable or field (only parameters can be)

- [ ] `InfoDisplayManager.Builder.Builder(0)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (xaero/hud/minimap/info/InfoDisplayManager.java:124)
- [ ] `InfoDisplayManager.Builder.setDefault(0)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (xaero/hud/minimap/info/InfoDisplayManager.java:127)
- [ ] `InfoDisplayManager.Builder.setLocalConfigSupplier(1)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (xaero/hud/minimap/info/InfoDisplayManager.java:134)
- [ ] `InfoDisplayManager.Builder.setLocalConfigSetter(1)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (xaero/hud/minimap/info/InfoDisplayManager.java:139)
- [ ] `InfoDisplayManager.Builder.setEnforcedConfigSupplier(1)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (xaero/hud/minimap/info/InfoDisplayManager.java:144)
- [ ] `InfoDisplayManager.Builder.build(0)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (xaero/hud/minimap/info/InfoDisplayManager.java:149)
- [ ] `InfoDisplayManager.Builder.begin(0)`: it uses Builder, which isn't translated (a function kept in a variable or field (only parameters can be)) (xaero/hud/minimap/info/InfoDisplayManager.java:162)

### `InfoDisplays`: it uses InfoDisplayManager, which isn't translated (it uses InfoDisplay, which isn't translated (it's generic))

- [ ] `InfoDisplays.InfoDisplays(1)`: its class isn't translated (it uses InfoDisplayManager, which isn't translated (it uses InfoDisplay, which isn't translated (it's generic))) (xaero/hud/minimap/info/InfoDisplays.java:14)
- [ ] `InfoDisplays.getManager(0)`: its class isn't translated (it uses InfoDisplayManager, which isn't translated (it uses InfoDisplay, which isn't translated (it's generic))) (xaero/hud/minimap/info/InfoDisplays.java:30)
- [ ] `InfoDisplays.getIo(0)`: its class isn't translated (it uses InfoDisplayManager, which isn't translated (it uses InfoDisplay, which isn't translated (it's generic))) (xaero/hud/minimap/info/InfoDisplays.java:34)
- [ ] `InfoDisplays.getRenderer(0)`: its class isn't translated (it uses InfoDisplayManager, which isn't translated (it uses InfoDisplay, which isn't translated (it's generic))) (xaero/hud/minimap/info/InfoDisplays.java:38)
- [ ] `InfoDisplays.clearStateCache(0)`: its class isn't translated (it uses InfoDisplayManager, which isn't translated (it uses InfoDisplay, which isn't translated (it's generic))) (xaero/hud/minimap/info/InfoDisplays.java:42)

### `InfoDisplayCompiler`: it uses Minecraft's `Component`

- [ ] `InfoDisplayCompiler.InfoDisplayCompiler(1)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/hud/minimap/info/render/compile/InfoDisplayCompiler.java:16)
- [ ] `InfoDisplayCompiler.compile(4)`: it's generic (xaero/hud/minimap/info/render/compile/InfoDisplayCompiler.java:20)
- [ ] `InfoDisplayCompiler.addWords(1)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/hud/minimap/info/render/compile/InfoDisplayCompiler.java:33)
- [ ] `InfoDisplayCompiler.addLine(1)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/hud/minimap/info/render/compile/InfoDisplayCompiler.java:67)
- [ ] `InfoDisplayCompiler.addLine(1)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/hud/minimap/info/render/compile/InfoDisplayCompiler.java:75)

### `InfoDisplayCompiler.Builder`

- [x] `InfoDisplayCompiler.Builder.Builder(0)` (xaero/hud/minimap/info/render/compile/InfoDisplayCompiler.java:80)
- [ ] `InfoDisplayCompiler.Builder.setDefault(0)`: `this` used as a value (xaero/hud/minimap/info/render/compile/InfoDisplayCompiler.java:83)
- [ ] `InfoDisplayCompiler.Builder.build(0)`: it uses InfoDisplayCompiler, which isn't translated (it uses Minecraft's `Component`) (xaero/hud/minimap/info/render/compile/InfoDisplayCompiler.java:87)
- [ ] `InfoDisplayCompiler.Builder.begin(0)`: it calls Builder.setDefault, which isn't translated (xaero/hud/minimap/info/render/compile/InfoDisplayCompiler.java:91)

### `InfoDisplayOnCompile`: it's an interface

- [ ] `InfoDisplayOnCompile.onCompile(5)`: it has no body (xaero/hud/minimap/info/render/compile/InfoDisplayOnCompile.java:9)

### `InfoDisplayRenderer`: it uses InfoDisplayCompiler, which isn't translated (it uses Minecraft's `Component`)

- [x] `InfoDisplayRenderer.DEPTH_OFFSET` (xaero/hud/minimap/info/render/InfoDisplayRenderer.java:19)
- [ ] `InfoDisplayRenderer.InfoDisplayRenderer(1)`: its class isn't translated (it uses InfoDisplayCompiler, which isn't translated (it uses Minecraft's `Component`)) (xaero/hud/minimap/info/render/InfoDisplayRenderer.java:22)
- [ ] `InfoDisplayRenderer.render(9)`: its class isn't translated (it uses InfoDisplayCompiler, which isn't translated (it uses Minecraft's `Component`)) (xaero/hud/minimap/info/render/InfoDisplayRenderer.java:26)

### `InfoDisplayRenderer.Builder`

- [x] `InfoDisplayRenderer.Builder.Builder(0)` (xaero/hud/minimap/info/render/InfoDisplayRenderer.java:82)
- [ ] `InfoDisplayRenderer.Builder.setDefault(0)`: `this` used as a value (xaero/hud/minimap/info/render/InfoDisplayRenderer.java:85)
- [ ] `InfoDisplayRenderer.Builder.build(0)`: it uses InfoDisplayRenderer, which isn't translated (it uses InfoDisplayCompiler, which isn't translated (it uses Minecraft's `Component`)) (xaero/hud/minimap/info/render/InfoDisplayRenderer.java:89)
- [ ] `InfoDisplayRenderer.Builder.begin(0)`: it calls Builder.setDefault, which isn't translated (xaero/hud/minimap/info/render/InfoDisplayRenderer.java:93)

### `InfoDisplayCommonWidgetFactories`

- [ ] `InfoDisplayCommonWidgetFactories.OFF_ON`: it uses InfoDisplayCycleWidgetFactory, which isn't translated (it's generic) (xaero/hud/minimap/info/widget/InfoDisplayCommonWidgetFactories.java:7)
- [ ] `InfoDisplayCommonWidgetFactories.ALWAYS_ON`: it uses InfoDisplayWidgetFactory, which isn't translated (it's an interface) (xaero/hud/minimap/info/widget/InfoDisplayCommonWidgetFactories.java:11)

### `InfoDisplayCycleButton`: it extends Minecraft's `Plain`

- [ ] `InfoDisplayCycleButton.InfoDisplayCycleButton(7)`: its class isn't translated (it extends Minecraft's `Plain`) (xaero/hud/minimap/info/widget/InfoDisplayCycleButton.java:15)

### `InfoDisplayCycleButton.Builder`: it's generic

- [ ] `InfoDisplayCycleButton.Builder.Builder(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/widget/InfoDisplayCycleButton.java:31)
- [ ] `InfoDisplayCycleButton.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/widget/InfoDisplayCycleButton.java:34)
- [ ] `InfoDisplayCycleButton.Builder.setBounds(4)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/widget/InfoDisplayCycleButton.java:42)
- [ ] `InfoDisplayCycleButton.Builder.setValues(2)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/widget/InfoDisplayCycleButton.java:50)
- [ ] `InfoDisplayCycleButton.Builder.setEntry(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/widget/InfoDisplayCycleButton.java:64)
- [ ] `InfoDisplayCycleButton.Builder.setOnChange(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/widget/InfoDisplayCycleButton.java:69)
- [ ] `InfoDisplayCycleButton.Builder.setIncludeNull(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/widget/InfoDisplayCycleButton.java:74)
- [ ] `InfoDisplayCycleButton.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/widget/InfoDisplayCycleButton.java:79)
- [ ] `InfoDisplayCycleButton.Builder.begin(0)`: it's generic (xaero/hud/minimap/info/widget/InfoDisplayCycleButton.java:123)

### `InfoDisplayCycleWidgetFactory`: it's generic

- [ ] `InfoDisplayCycleWidgetFactory.InfoDisplayCycleWidgetFactory(2)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/widget/InfoDisplayCycleWidgetFactory.java:12)
- [ ] `InfoDisplayCycleWidgetFactory.create(7)`: its class isn't translated (it's generic) (xaero/hud/minimap/info/widget/InfoDisplayCycleWidgetFactory.java:17)

### `InfoDisplayWidgetFactory`: it's an interface

- [ ] `InfoDisplayWidgetFactory.create(7)`: it has no body (xaero/hud/minimap/info/widget/InfoDisplayWidgetFactory.java:7)

### `Minimap`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)

- [ ] `Minimap.Minimap(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/Minimap.java:32)
- [ ] `Minimap.getCrashedWith(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/Minimap.java:48)
- [ ] `Minimap.setCrashedWith(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/Minimap.java:52)
- [ ] `Minimap.checkCrashes(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/Minimap.java:58)
- [ ] `Minimap.getWaypointMapRenderer(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/Minimap.java:66)
- [ ] `Minimap.getWaypointWorldRenderer(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/Minimap.java:70)
- [ ] `Minimap.getMinimapFBORenderer(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/Minimap.java:74)
- [ ] `Minimap.getMinimapSafeModeRenderer(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/Minimap.java:78)
- [ ] `Minimap.getOverMapRendererHandler(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/Minimap.java:82)
- [ ] `Minimap.getWorldRendererHandler(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/Minimap.java:86)
- [ ] `Minimap.usingFBO(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/Minimap.java:90)
- [ ] `Minimap.getCompassRenderer(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/Minimap.java:95)
- [ ] `Minimap.getInfoDisplays(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/Minimap.java:99)
- [ ] `Minimap.getMatrixStack(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/Minimap.java:103)
- [ ] `Minimap.getModMain(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/Minimap.java:107)

### `MinimapLogs`

- [x] `MinimapLogs.LOGGER` (xaero/hud/minimap/MinimapLogs.java:7)

### `MinimapRenderer`: another class has the same name, in a different package

- [ ] `MinimapRenderer.render(4)`: its class isn't translated (another class has the same name, in a different package) (xaero/hud/minimap/module/MinimapRenderer.java:21)

### `MinimapSession`: it extends `ModuleSession` (xaero.hud.module)

- [ ] `MinimapSession.MinimapSession(3)`: its class isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/module/MinimapSession.java:41)
- [ ] `MinimapSession.prePotentialRender(0)`: its class isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/module/MinimapSession.java:85)
- [ ] `MinimapSession.close(0)`: its class isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/module/MinimapSession.java:97)
- [ ] `MinimapSession.getWidth(1)`: its class isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/module/MinimapSession.java:102)
- [ ] `MinimapSession.getHeight(1)`: its class isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/module/MinimapSession.java:109)
- [ ] `MinimapSession.getConfiguredWidth(0)`: its class isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/module/MinimapSession.java:114)
- [ ] `MinimapSession.getProcessor(0)`: its class isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/module/MinimapSession.java:118)
- [ ] `MinimapSession.getHideMinimapUnderScreen(0)`: its class isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/module/MinimapSession.java:122)
- [ ] `MinimapSession.getHideMinimapUnderF3(0)`: its class isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/module/MinimapSession.java:126)
- [ ] `MinimapSession.getMultiTextureRenderTypeRenderers(0)`: its class isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/module/MinimapSession.java:130)
- [ ] `MinimapSession.getWaypointSession(0)`: its class isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/module/MinimapSession.java:134)
- [ ] `MinimapSession.getRadarSession(0)`: its class isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/module/MinimapSession.java:138)
- [ ] `MinimapSession.getMc(0)`: its class isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/module/MinimapSession.java:142)
- [ ] `MinimapSession.getWorldManager(0)`: its class isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/module/MinimapSession.java:146)
- [ ] `MinimapSession.getWorldState(0)`: its class isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/module/MinimapSession.java:150)
- [ ] `MinimapSession.getWorldStateUpdater(0)`: its class isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/module/MinimapSession.java:154)
- [ ] `MinimapSession.getDimensionHelper(0)`: its class isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/module/MinimapSession.java:158)
- [ ] `MinimapSession.getWorldManagerIO(0)`: its class isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/module/MinimapSession.java:162)

### `PlayerTrackerIconRenderer`

- [ ] `PlayerTrackerIconRenderer.isPlayerUpsideDown(1)`: it uses Minecraft's `Player` (xaero/hud/minimap/player/tracker/PlayerTrackerIconRenderer.java:16)
- [x] `PlayerTrackerIconRenderer.isUpsideDownName(1)` (xaero/hud/minimap/player/tracker/PlayerTrackerIconRenderer.java:21)
- [ ] `PlayerTrackerIconRenderer.renderIcon(7)`: it uses Minecraft's `Minecraft` (xaero/hud/minimap/player/tracker/PlayerTrackerIconRenderer.java:25)

### `PlayerTrackerMinimapElement`: it's generic

- [ ] `PlayerTrackerMinimapElement.PlayerTrackerMinimapElement(2)`: its class isn't translated (it's generic) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElement.java:13)
- [ ] `PlayerTrackerMinimapElement.getPlayerId(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElement.java:18)
- [ ] `PlayerTrackerMinimapElement.getX(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElement.java:22)
- [ ] `PlayerTrackerMinimapElement.getY(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElement.java:26)
- [ ] `PlayerTrackerMinimapElement.getZ(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElement.java:30)
- [ ] `PlayerTrackerMinimapElement.getDimension(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElement.java:34)
- [ ] `PlayerTrackerMinimapElement.getPlayer(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElement.java:38)
- [ ] `PlayerTrackerMinimapElement.setRenderedOnRadar(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElement.java:42)
- [ ] `PlayerTrackerMinimapElement.wasRenderedOnRadar(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElement.java:46)
- [ ] `PlayerTrackerMinimapElement.getSystem(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElement.java:50)

### `PlayerTrackerMinimapElementCollector`: it uses `UUID` (java.util)

- [ ] `PlayerTrackerMinimapElementCollector.PlayerTrackerMinimapElementCollector(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementCollector.java:17)
- [ ] `PlayerTrackerMinimapElementCollector.update(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementCollector.java:21)
- [ ] `PlayerTrackerMinimapElementCollector.updateForSystem(3)`: it's generic (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementCollector.java:38)
- [ ] `PlayerTrackerMinimapElementCollector.playerExists(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementCollector.java:66)
- [ ] `PlayerTrackerMinimapElementCollector.getElements(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementCollector.java:70)
- [ ] `PlayerTrackerMinimapElementCollector.resetRenderedOnRadarFlags(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementCollector.java:74)
- [ ] `PlayerTrackerMinimapElementCollector.confirmPlayerRadarRender(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementCollector.java:80)

### `PlayerTrackerMinimapElementReader`: it extends `MinimapElementReader` (xaero.hud.minimap.element.render)

- [ ] `PlayerTrackerMinimapElementReader.isHidden(2)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:12)
- [ ] `PlayerTrackerMinimapElementReader.getRenderX(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:16)
- [ ] `PlayerTrackerMinimapElementReader.getRenderY(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:22)
- [ ] `PlayerTrackerMinimapElementReader.getRenderZ(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:28)
- [ ] `PlayerTrackerMinimapElementReader.getCoordinateScale(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:34)
- [ ] `PlayerTrackerMinimapElementReader.getInteractionBoxLeft(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:40)
- [ ] `PlayerTrackerMinimapElementReader.getInteractionBoxRight(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:44)
- [ ] `PlayerTrackerMinimapElementReader.getInteractionBoxTop(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:48)
- [ ] `PlayerTrackerMinimapElementReader.getInteractionBoxBottom(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:52)
- [ ] `PlayerTrackerMinimapElementReader.getRenderBoxLeft(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:56)
- [ ] `PlayerTrackerMinimapElementReader.getRenderBoxRight(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:60)
- [ ] `PlayerTrackerMinimapElementReader.getRenderBoxTop(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:64)
- [ ] `PlayerTrackerMinimapElementReader.getRenderBoxBottom(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:68)
- [ ] `PlayerTrackerMinimapElementReader.getLeftSideLength(2)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:72)
- [ ] `PlayerTrackerMinimapElementReader.getMenuName(1)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:77)
- [ ] `PlayerTrackerMinimapElementReader.getFilterName(1)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:82)
- [ ] `PlayerTrackerMinimapElementReader.getMenuTextFillLeftPadding(1)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:86)
- [ ] `PlayerTrackerMinimapElementReader.getRightClickTitleBackgroundColor(1)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:90)
- [ ] `PlayerTrackerMinimapElementReader.shouldScaleBoxWithOptionalScale(0)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:94)
- [ ] `PlayerTrackerMinimapElementReader.getBoxScale(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:99)
- [ ] `PlayerTrackerMinimapElementReader.isInteractable(2)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java:103)

### `PlayerTrackerMinimapElementRenderContext`: it uses Minecraft's `VertexConsumer`


### `PlayerTrackerMinimapElementRenderer`: it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)

- [ ] `PlayerTrackerMinimapElementRenderer.PlayerTrackerMinimapElementRenderer(6)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderer.java:35)
- [ ] `PlayerTrackerMinimapElementRenderer.getPlayerSkin(2)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderer.java:49)
- [ ] `PlayerTrackerMinimapElementRenderer.preRender(3)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderer.java:60)
- [ ] `PlayerTrackerMinimapElementRenderer.postRender(3)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderer.java:74)
- [ ] `PlayerTrackerMinimapElementRenderer.renderElement(10)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderer.java:81)
- [ ] `PlayerTrackerMinimapElementRenderer.shouldRender(1)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderer.java:160)
- [ ] `PlayerTrackerMinimapElementRenderer.getOrder(0)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderer.java:167)
- [ ] `PlayerTrackerMinimapElementRenderer.getCollector(0)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderer.java:172)

### `PlayerTrackerMinimapElementRenderer.Builder`: it uses IXaeroMinimap, which isn't translated (it's an interface)

- [ ] `PlayerTrackerMinimapElementRenderer.Builder.Builder(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderer.java:179)
- [ ] `PlayerTrackerMinimapElementRenderer.Builder.setDefault(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderer.java:183)
- [ ] `PlayerTrackerMinimapElementRenderer.Builder.build(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderer.java:187)
- [ ] `PlayerTrackerMinimapElementRenderer.Builder.begin(1)`: it uses Builder, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderer.java:199)

### `PlayerTrackerMinimapElementRenderProvider`: it's generic

- [ ] `PlayerTrackerMinimapElementRenderProvider.PlayerTrackerMinimapElementRenderProvider(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderProvider.java:11)
- [ ] `PlayerTrackerMinimapElementRenderProvider.begin(2)`: its class isn't translated (it's generic) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderProvider.java:15)
- [ ] `PlayerTrackerMinimapElementRenderProvider.hasNext(2)`: its class isn't translated (it's generic) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderProvider.java:20)
- [ ] `PlayerTrackerMinimapElementRenderProvider.getNext(2)`: its class isn't translated (it's generic) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderProvider.java:25)
- [ ] `PlayerTrackerMinimapElementRenderProvider.end(2)`: its class isn't translated (it's generic) (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderProvider.java:29)

### `ClientSyncedTrackedPlayerManager`: it uses `UUID` (java.util)

- [ ] `ClientSyncedTrackedPlayerManager.ClientSyncedTrackedPlayerManager(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/hud/minimap/player/tracker/synced/ClientSyncedTrackedPlayerManager.java:13)
- [ ] `ClientSyncedTrackedPlayerManager.remove(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/hud/minimap/player/tracker/synced/ClientSyncedTrackedPlayerManager.java:17)
- [ ] `ClientSyncedTrackedPlayerManager.update(5)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/hud/minimap/player/tracker/synced/ClientSyncedTrackedPlayerManager.java:21)
- [ ] `ClientSyncedTrackedPlayerManager.getPlayers(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/hud/minimap/player/tracker/synced/ClientSyncedTrackedPlayerManager.java:30)
- [ ] `ClientSyncedTrackedPlayerManager.reset(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/hud/minimap/player/tracker/synced/ClientSyncedTrackedPlayerManager.java:34)

### `ClientSyncedTrackedPlayerManager.Builder`

- [x] `ClientSyncedTrackedPlayerManager.Builder.Builder(0)` (xaero/hud/minimap/player/tracker/synced/ClientSyncedTrackedPlayerManager.java:39)
- [ ] `ClientSyncedTrackedPlayerManager.Builder.setDefault(0)`: `this` used as a value (xaero/hud/minimap/player/tracker/synced/ClientSyncedTrackedPlayerManager.java:42)
- [ ] `ClientSyncedTrackedPlayerManager.Builder.build(0)`: it uses ClientSyncedTrackedPlayerManager, which isn't translated (it uses `UUID` (java.util)) (xaero/hud/minimap/player/tracker/synced/ClientSyncedTrackedPlayerManager.java:46)
- [ ] `ClientSyncedTrackedPlayerManager.Builder.begin(0)`: it calls Builder.setDefault, which isn't translated (xaero/hud/minimap/player/tracker/synced/ClientSyncedTrackedPlayerManager.java:50)

### `SyncedRenderedPlayerTracker`: it implements `IRenderedPlayerTracker` (xaero.hud.minimap.player.tracker.system)

- [ ] `SyncedRenderedPlayerTracker.getReader(0)`: its class isn't translated (it implements `IRenderedPlayerTracker` (xaero.hud.minimap.player.tracker.system)) (xaero/hud/minimap/player/tracker/synced/SyncedRenderedPlayerTracker.java:15)
- [ ] `SyncedRenderedPlayerTracker.getTrackedPlayerIterator(0)`: its class isn't translated (it implements `IRenderedPlayerTracker` (xaero.hud.minimap.player.tracker.system)) (xaero/hud/minimap/player/tracker/synced/SyncedRenderedPlayerTracker.java:20)
- [ ] `SyncedRenderedPlayerTracker.shouldUseWorldMapTrackedPlayers(1)`: its class isn't translated (it implements `IRenderedPlayerTracker` (xaero.hud.minimap.player.tracker.system)) (xaero/hud/minimap/player/tracker/synced/SyncedRenderedPlayerTracker.java:38)

### `SyncedTrackedPlayerReader`: it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)

- [ ] `SyncedTrackedPlayerReader.getId(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)) (xaero/hud/minimap/player/tracker/synced/SyncedTrackedPlayerReader.java:10)
- [ ] `SyncedTrackedPlayerReader.getX(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)) (xaero/hud/minimap/player/tracker/synced/SyncedTrackedPlayerReader.java:14)
- [ ] `SyncedTrackedPlayerReader.getY(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)) (xaero/hud/minimap/player/tracker/synced/SyncedTrackedPlayerReader.java:18)
- [ ] `SyncedTrackedPlayerReader.getZ(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)) (xaero/hud/minimap/player/tracker/synced/SyncedTrackedPlayerReader.java:22)
- [ ] `SyncedTrackedPlayerReader.getDimension(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)) (xaero/hud/minimap/player/tracker/synced/SyncedTrackedPlayerReader.java:26)

### `IRenderedPlayerTracker`: it's an interface

- [ ] `IRenderedPlayerTracker.getReader(0)`: it has no body (xaero/hud/minimap/player/tracker/system/IRenderedPlayerTracker.java:6)
- [ ] `IRenderedPlayerTracker.getTrackedPlayerIterator(0)`: it has no body (xaero/hud/minimap/player/tracker/system/IRenderedPlayerTracker.java:8)

### `ITrackedPlayerReader`: it's an interface

- [ ] `ITrackedPlayerReader.getId(1)`: it has no body (xaero/hud/minimap/player/tracker/system/ITrackedPlayerReader.java:8)
- [ ] `ITrackedPlayerReader.getX(1)`: it has no body (xaero/hud/minimap/player/tracker/system/ITrackedPlayerReader.java:10)
- [ ] `ITrackedPlayerReader.getY(1)`: it has no body (xaero/hud/minimap/player/tracker/system/ITrackedPlayerReader.java:12)
- [ ] `ITrackedPlayerReader.getZ(1)`: it has no body (xaero/hud/minimap/player/tracker/system/ITrackedPlayerReader.java:14)
- [ ] `ITrackedPlayerReader.getDimension(1)`: it has no body (xaero/hud/minimap/player/tracker/system/ITrackedPlayerReader.java:16)

### `RenderedPlayerTrackerManager`: it uses IRenderedPlayerTracker, which isn't translated (it's an interface)

- [ ] `RenderedPlayerTrackerManager.RenderedPlayerTrackerManager(1)`: its class isn't translated (it uses IRenderedPlayerTracker, which isn't translated (it's an interface)) (xaero/hud/minimap/player/tracker/system/RenderedPlayerTrackerManager.java:10)
- [ ] `RenderedPlayerTrackerManager.register(2)`: its class isn't translated (it uses IRenderedPlayerTracker, which isn't translated (it's an interface)) (xaero/hud/minimap/player/tracker/system/RenderedPlayerTrackerManager.java:14)
- [ ] `RenderedPlayerTrackerManager.getAllSystems(0)`: its class isn't translated (it uses IRenderedPlayerTracker, which isn't translated (it's an interface)) (xaero/hud/minimap/player/tracker/system/RenderedPlayerTrackerManager.java:23)

### `RenderedPlayerTrackerManager.Builder`

- [x] `RenderedPlayerTrackerManager.Builder.Builder(0)` (xaero/hud/minimap/player/tracker/system/RenderedPlayerTrackerManager.java:28)
- [ ] `RenderedPlayerTrackerManager.Builder.setDefault(0)`: `this` used as a value (xaero/hud/minimap/player/tracker/system/RenderedPlayerTrackerManager.java:31)
- [ ] `RenderedPlayerTrackerManager.Builder.build(0)`: it uses RenderedPlayerTrackerManager, which isn't translated (it uses IRenderedPlayerTracker, which isn't translated (it's an interface)) (xaero/hud/minimap/player/tracker/system/RenderedPlayerTrackerManager.java:35)
- [ ] `RenderedPlayerTrackerManager.Builder.begin(0)`: it calls Builder.setDefault, which isn't translated (xaero/hud/minimap/player/tracker/system/RenderedPlayerTrackerManager.java:39)

### `EntityRadarBackwardsCompatibilityConfig`

- [ ] `EntityRadarBackwardsCompatibilityConfig.readSetting(1)`: it calls `Integer`.parseInt (xaero/hud/minimap/radar/category/EntityRadarBackwardsCompatibilityConfig.java:38)

### `EntityRadarCategory`: it extends `FilterObjectCategory` (xaero.hud.category)

- [ ] `EntityRadarCategory.EntityRadarCategory(12)`: its class isn't translated (it extends `FilterObjectCategory` (xaero.hud.category)) (xaero/hud/minimap/radar/category/EntityRadarCategory.java:19)

### `EntityRadarCategory.Builder`: it extends `Builder`

- [ ] `EntityRadarCategory.Builder.Builder(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/EntityRadarCategory.java:50)
- [ ] `EntityRadarCategory.Builder.setDefault(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/EntityRadarCategory.java:54)
- [ ] `EntityRadarCategory.Builder.buildUncheckedFilter(5)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/EntityRadarCategory.java:60)
- [ ] `EntityRadarCategory.Builder.begin(0)`: it uses Builder, which isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/EntityRadarCategory.java:83)

### `EntityRadarCategoryConfigIO`: it uses `ConfigOption` (xaero.lib.common.config.option)

- [ ] `EntityRadarCategoryConfigIO.EntityRadarCategoryConfigIO(2)`: its class isn't translated (it uses `ConfigOption` (xaero.lib.common.config.option)) (xaero/hud/minimap/radar/category/EntityRadarCategoryConfigIO.java:13)
- [ ] `EntityRadarCategoryConfigIO.storeRootCategory(2)`: its class isn't translated (it uses `ConfigOption` (xaero.lib.common.config.option)) (xaero/hud/minimap/radar/category/EntityRadarCategoryConfigIO.java:20)
- [ ] `EntityRadarCategoryConfigIO.loadRootCategory(1)`: its class isn't translated (it uses `ConfigOption` (xaero.lib.common.config.option)) (xaero/hud/minimap/radar/category/EntityRadarCategoryConfigIO.java:25)

### `EntityRadarCategoryConfigIO.Builder`: it uses `ConfigOption` (xaero.lib.common.config.option)

- [ ] `EntityRadarCategoryConfigIO.Builder.Builder(0)`: its class isn't translated (it uses `ConfigOption` (xaero.lib.common.config.option)) (xaero/hud/minimap/radar/category/EntityRadarCategoryConfigIO.java:34)
- [ ] `EntityRadarCategoryConfigIO.Builder.setDefault(0)`: its class isn't translated (it uses `ConfigOption` (xaero.lib.common.config.option)) (xaero/hud/minimap/radar/category/EntityRadarCategoryConfigIO.java:37)
- [ ] `EntityRadarCategoryConfigIO.Builder.setOption(1)`: its class isn't translated (it uses `ConfigOption` (xaero.lib.common.config.option)) (xaero/hud/minimap/radar/category/EntityRadarCategoryConfigIO.java:42)
- [ ] `EntityRadarCategoryConfigIO.Builder.setSerializationHandler(1)`: its class isn't translated (it uses `ConfigOption` (xaero.lib.common.config.option)) (xaero/hud/minimap/radar/category/EntityRadarCategoryConfigIO.java:47)
- [ ] `EntityRadarCategoryConfigIO.Builder.build(0)`: its class isn't translated (it uses `ConfigOption` (xaero.lib.common.config.option)) (xaero/hud/minimap/radar/category/EntityRadarCategoryConfigIO.java:52)
- [ ] `EntityRadarCategoryConfigIO.Builder.begin(0)`: it uses Builder, which isn't translated (it uses `ConfigOption` (xaero.lib.common.config.option)) (xaero/hud/minimap/radar/category/EntityRadarCategoryConfigIO.java:60)

### `EntityRadarCategoryConstants`

- [x] `EntityRadarCategoryConstants.format` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:27)
- [ ] `EntityRadarCategoryConstants.LIST_FACTORY`: it uses ListFactory, which isn't translated (it's an interface) (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:28)
- [ ] `EntityRadarCategoryConstants.MAP_FACTORY`: it uses MapFactory, which isn't translated (it's an interface) (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:29)
- [ ] `EntityRadarCategoryConstants.LEGACY_CONFIG_PATH`: it uses Services.PLATFORM, which isn't translated (it uses IPlatformHelper, which isn't translated (it's an interface)) (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:30)
- [ ] `EntityRadarCategoryConstants.LEGACY_DEFAULT_CONFIG_PATH`: it uses EntityRadarCategoryConstants.LEGACY_CONFIG_PATH, which isn't translated (it uses Services.PLATFORM, which isn't translated (it uses IPlatformHelper, which isn't translated (it's an interface))) (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:31)
- [ ] `EntityRadarCategoryConstants.DATA_BUILDER_FACTORY`: it uses Builder, which isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:34)
- [ ] `EntityRadarCategoryConstants.CATEGORY_BUILDER_FACTORY`: it uses Builder, which isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:35)
- [ ] `EntityRadarCategoryConstants.DEFAULT_LIST_SERIALIZER`: it uses Minecraft's `EntityType` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:36)
- [ ] `EntityRadarCategoryConstants.WILDCARD_TO_REGEX`: a function kept in a variable or field (only parameters can be) (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:37)
- [ ] `EntityRadarCategoryConstants.NULL_DATA`: it uses EntityRadarCategoryData, which isn't translated (it extends `FilterObjectCategoryData` (xaero.hud.category.serialization.data)) (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:38)
- [ ] `EntityRadarCategoryConstants.DEFAULT_LIST_WILDCARD_VALIDATOR`: a function kept in a variable or field (only parameters can be) (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:39)
- [ ] `EntityRadarCategoryConstants.DEFAULT_LIST_STRING_VALIDATOR`: a function kept in a variable or field (only parameters can be) (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:47)
- [ ] `EntityRadarCategoryConstants.DEFAULT_LIST_STRING_VALIDATOR_FIXER`: a function kept in a variable or field (only parameters can be) (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:50)
- [x] `EntityRadarCategoryConstants.PLAYER_NAME_LIST_ALLOWED_CHARS` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:51)
- [ ] `EntityRadarCategoryConstants.PLAYER_NAME_VALIDATOR`: a function kept in a variable or field (only parameters can be) (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:52)
- [ ] `EntityRadarCategoryConstants.PLAYER_NAME_VALIDATOR_FIXER`: a function kept in a variable or field (only parameters can be) (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:53)
- [ ] `EntityRadarCategoryConstants.PREDICATE_VALUE_ALL_SUPPLIER`: a function kept in a variable or field (only parameters can be) (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:54)
- [ ] `EntityRadarCategoryConstants.PREDICATE_VALUE_RESOLVER`: a function kept in a variable or field (only parameters can be) (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:55)
- [ ] `EntityRadarCategoryConstants.PREDICATE_VALUE_SERIALIZER`: a function kept in a variable or field (only parameters can be) (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:56)
- [ ] `EntityRadarCategoryConstants.PREDICATE_VALUE_VALIDATOR`: a function kept in a variable or field (only parameters can be) (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:57)
- [ ] `EntityRadarCategoryConstants.PREDICATE_VALUE_VALIDATOR_FIXER`: a function kept in a variable or field (only parameters can be) (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:58)
- [x] `EntityRadarCategoryConstants.CATEGORY_ROOT` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:59)
- [x] `EntityRadarCategoryConstants.CATEGORY_LIVING` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:60)
- [x] `EntityRadarCategoryConstants.CATEGORY_HOSTILE` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:61)
- [x] `EntityRadarCategoryConstants.CATEGORY_FRIENDLY` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:62)
- [x] `EntityRadarCategoryConstants.CATEGORY_HOSTILE_TAMED` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:63)
- [x] `EntityRadarCategoryConstants.CATEGORY_FRIENDLY_TAMED` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:64)
- [x] `EntityRadarCategoryConstants.CATEGORY_PLAYERS` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:65)
- [x] `EntityRadarCategoryConstants.CATEGORY_FRIENDS` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:66)
- [x] `EntityRadarCategoryConstants.CATEGORY_TRACKED` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:67)
- [x] `EntityRadarCategoryConstants.CATEGORY_SAME_TEAM` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:68)
- [x] `EntityRadarCategoryConstants.CATEGORY_OTHER_TEAMS` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:69)
- [x] `EntityRadarCategoryConstants.CATEGORY_ITEMS` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:70)
- [x] `EntityRadarCategoryConstants.CATEGORY_OTHER` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:71)
- [x] `EntityRadarCategoryConstants.HARD_NOTHING` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:72)
- [x] `EntityRadarCategoryConstants.HARD_LIVING` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:73)
- [x] `EntityRadarCategoryConstants.HARD_HOSTILE` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:74)
- [x] `EntityRadarCategoryConstants.HARD_FRIENDLY` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:75)
- [x] `EntityRadarCategoryConstants.HARD_TAMED` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:76)
- [x] `EntityRadarCategoryConstants.HARD_PLAYERS` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:77)
- [x] `EntityRadarCategoryConstants.HARD_SAME_TEAM` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:78)
- [x] `EntityRadarCategoryConstants.HARD_OTHER_TEAMS` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:79)
- [x] `EntityRadarCategoryConstants.HARD_ITEMS` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:80)
- [x] `EntityRadarCategoryConstants.HARD_ANYTHING` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:81)
- [x] `EntityRadarCategoryConstants.HARD_BABY` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:82)
- [x] `EntityRadarCategoryConstants.HARD_VANILLA` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:83)
- [x] `EntityRadarCategoryConstants.HARD_MODDED` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:84)
- [x] `EntityRadarCategoryConstants.HARD_ABOVE_GROUND` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:85)
- [x] `EntityRadarCategoryConstants.HARD_BELOW_GROUND` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:86)
- [x] `EntityRadarCategoryConstants.HARD_MY_GROUND` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:87)
- [x] `EntityRadarCategoryConstants.HARD_NOT_MY_GROUND` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:88)
- [x] `EntityRadarCategoryConstants.HARD_LIT` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:89)
- [x] `EntityRadarCategoryConstants.HARD_UNLIT` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:90)
- [x] `EntityRadarCategoryConstants.HARD_CUSTOM_NAME` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:91)
- [x] `EntityRadarCategoryConstants.HARD_NO_CUSTOM_NAME` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:92)
- [x] `EntityRadarCategoryConstants.HARD_TRACKED` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:93)
- [x] `EntityRadarCategoryConstants.HARD_IN_TEAM` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:94)
- [x] `EntityRadarCategoryConstants.HARD_TEAMLESS` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:95)
- [ ] `EntityRadarCategoryConstants.getDefaultElementResolver(3)`: it's generic (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:97)
- [x] `EntityRadarCategoryConstants.containsWildcardCharacters(1)` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:125)
- [ ] `EntityRadarCategoryConstants.createHardRuleBasedPredicateListRuleType(3)`: it uses ObjectCategoryListRuleType, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java:129)

### `EntityRadarCategoryFileIO`: it uses EntityRadarCategorySerializationHandler, which isn't translated (it extends `FilterObjectCategorySerializationHandler` (xaero.hud.category.serialization))

- [ ] `EntityRadarCategoryFileIO.EntityRadarCategoryFileIO(2)`: its class isn't translated (it uses EntityRadarCategorySerializationHandler, which isn't translated (it extends `FilterObjectCategorySerializationHandler` (xaero.hud.category.serialization))) (xaero/hud/minimap/radar/category/EntityRadarCategoryFileIO.java:21)
- [ ] `EntityRadarCategoryFileIO.saveRootCategory(1)`: its class isn't translated (it uses EntityRadarCategorySerializationHandler, which isn't translated (it extends `FilterObjectCategorySerializationHandler` (xaero.hud.category.serialization))) (xaero/hud/minimap/radar/category/EntityRadarCategoryFileIO.java:26)
- [ ] `EntityRadarCategoryFileIO.saveRootCategory(3)`: its class isn't translated (it uses EntityRadarCategorySerializationHandler, which isn't translated (it extends `FilterObjectCategorySerializationHandler` (xaero.hud.category.serialization))) (xaero/hud/minimap/radar/category/EntityRadarCategoryFileIO.java:32)
- [ ] `EntityRadarCategoryFileIO.loadRootCategory(0)`: its class isn't translated (it uses EntityRadarCategorySerializationHandler, which isn't translated (it extends `FilterObjectCategorySerializationHandler` (xaero.hud.category.serialization))) (xaero/hud/minimap/radar/category/EntityRadarCategoryFileIO.java:58)

### `EntityRadarCategoryFileIO.Builder`: it holds an object of its own class (a linked structure)

- [ ] `EntityRadarCategoryFileIO.Builder.Builder(1)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/minimap/radar/category/EntityRadarCategoryFileIO.java:83)
- [ ] `EntityRadarCategoryFileIO.Builder.setDefault(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/minimap/radar/category/EntityRadarCategoryFileIO.java:87)
- [ ] `EntityRadarCategoryFileIO.Builder.setSaveLocationPath(1)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/minimap/radar/category/EntityRadarCategoryFileIO.java:92)
- [ ] `EntityRadarCategoryFileIO.Builder.build(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/minimap/radar/category/EntityRadarCategoryFileIO.java:97)
- [ ] `EntityRadarCategoryFileIO.Builder.begin(1)`: it uses Builder, which isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/minimap/radar/category/EntityRadarCategoryFileIO.java:105)

### `EntityRadarCategoryManager`: it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))

- [ ] `EntityRadarCategoryManager.EntityRadarCategoryManager(7)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:50)
- [ ] `EntityRadarCategoryManager.init(0)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:68)
- [ ] `EntityRadarCategoryManager.fetchDefaultClientCategory(0)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:128)
- [ ] `EntityRadarCategoryManager.fetchDefaultClientCategory(1)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:132)
- [ ] `EntityRadarCategoryManager.fetchDefaultServerCategory(0)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:160)
- [ ] `EntityRadarCategoryManager.fetchDefaultServerCategory(1)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:164)
- [ ] `EntityRadarCategoryManager.updateFromConfigChange(1)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:191)
- [ ] `EntityRadarCategoryManager.getRuleResolver(0)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:219)
- [ ] `EntityRadarCategoryManager.getRootCategory(0)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:223)
- [ ] `EntityRadarCategoryManager.getSyncedRootCategory(0)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:227)
- [ ] `EntityRadarCategoryManager.getEffectiveSyncedRootCategory(0)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:231)
- [ ] `EntityRadarCategoryManager.getDefaultCategoryConfigurator(0)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:236)
- [ ] `EntityRadarCategoryManager.getSecondaryLegacyFilePath(0)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:240)
- [ ] `EntityRadarCategoryManager.loadEditedCategory(2)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:244)
- [ ] `EntityRadarCategoryManager.getEditedCategoryConfig(0)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:255)
- [ ] `EntityRadarCategoryManager.getEditedCategory(0)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:259)
- [ ] `EntityRadarCategoryManager.forgetEditedCategory(0)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:263)
- [ ] `EntityRadarCategoryManager.storeEditedCategory(1)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:268)
- [ ] `EntityRadarCategoryManager.storeEditedCategory(2)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:272)
- [ ] `EntityRadarCategoryManager.getMainIO(0)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:282)
- [ ] `EntityRadarCategoryManager.resetRootCategorySettings(1)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:286)
- [ ] `EntityRadarCategoryManager.copySettingValue(3)`: it's generic (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:309)
- [ ] `EntityRadarCategoryManager.editedCategoryNeedsSaving(0)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:313)
- [ ] `EntityRadarCategoryManager.setEditedCategoryNeedsSaving(1)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:317)

### `EntityRadarCategoryManager.Builder`: it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))

- [ ] `EntityRadarCategoryManager.Builder.Builder(0)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:329)
- [ ] `EntityRadarCategoryManager.Builder.setMainLegacyFilePath(1)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:332)
- [ ] `EntityRadarCategoryManager.Builder.setSecondaryLegacyFilePath(1)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:337)
- [ ] `EntityRadarCategoryManager.Builder.setDefaultClientFilePath(1)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:342)
- [ ] `EntityRadarCategoryManager.Builder.setDefaultServerFilePath(1)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:347)
- [ ] `EntityRadarCategoryManager.Builder.setSecondaryDefaultFolderPath(1)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:352)
- [ ] `EntityRadarCategoryManager.Builder.setSerializers(1)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:357)
- [ ] `EntityRadarCategoryManager.Builder.setDefault(0)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:362)
- [ ] `EntityRadarCategoryManager.Builder.build(0)`: its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:372)
- [ ] `EntityRadarCategoryManager.Builder.begin(0)`: it uses Builder, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))) (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java:395)

### `EntityRadarDefaultCategories`

- [x] `EntityRadarDefaultCategories.EntityRadarDefaultCategories(1)` (xaero/hud/minimap/radar/category/EntityRadarDefaultCategories.java:13)
- [ ] `EntityRadarDefaultCategories.setupDefault(1)`: it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category)) (xaero/hud/minimap/radar/category/EntityRadarDefaultCategories.java:17)

### `EntityRadarDefaultCategories.Builder`

- [x] `EntityRadarDefaultCategories.Builder.Builder(0)` (xaero/hud/minimap/radar/category/EntityRadarDefaultCategories.java:264)
- [ ] `EntityRadarDefaultCategories.Builder.setDefault(0)`: `this` used as a value (xaero/hud/minimap/radar/category/EntityRadarDefaultCategories.java:267)
- [ ] `EntityRadarDefaultCategories.Builder.setForServer(1)`: `this` used as a value (xaero/hud/minimap/radar/category/EntityRadarDefaultCategories.java:272)
- [x] `EntityRadarDefaultCategories.Builder.build(0)` (xaero/hud/minimap/radar/category/EntityRadarDefaultCategories.java:277)
- [ ] `EntityRadarDefaultCategories.Builder.begin(0)`: it calls Builder.setDefault, which isn't translated (xaero/hud/minimap/radar/category/EntityRadarDefaultCategories.java:281)

### `EntityRadarCategoryHardRules`

- [ ] `EntityRadarCategoryHardRules.HARD_RULES_LIST`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:20)
- [ ] `EntityRadarCategoryHardRules.HARD_RULES`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:21)
- [ ] `EntityRadarCategoryHardRules.IS_NOTHING`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:22)
- [ ] `EntityRadarCategoryHardRules.IS_ANYTHING`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:26)
- [ ] `EntityRadarCategoryHardRules.IS_LIVING`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:30)
- [ ] `EntityRadarCategoryHardRules.IS_PLAYER`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:34)
- [ ] `EntityRadarCategoryHardRules.IS_SAME_TEAM`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:38)
- [ ] `EntityRadarCategoryHardRules.IS_HOSTILE`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:42)
- [ ] `EntityRadarCategoryHardRules.IS_TAMED`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:46)
- [ ] `EntityRadarCategoryHardRules.IS_ITEM`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:50)
- [ ] `EntityRadarCategoryHardRules.IS_FRIENDLY`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:54)
- [ ] `EntityRadarCategoryHardRules.IS_OTHER_TEAMS`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:58)
- [ ] `EntityRadarCategoryHardRules.IS_BABY`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:62)
- [ ] `EntityRadarCategoryHardRules.IS_VANILLA`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:66)
- [ ] `EntityRadarCategoryHardRules.IS_MODDED`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:74)
- [ ] `EntityRadarCategoryHardRules.IS_ABOVE_GROUND`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:78)
- [ ] `EntityRadarCategoryHardRules.IS_BELOW_GROUND`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:82)
- [ ] `EntityRadarCategoryHardRules.IS_MY_GROUND`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:86)
- [ ] `EntityRadarCategoryHardRules.IS_NOT_MY_GROUND`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:97)
- [ ] `EntityRadarCategoryHardRules.IS_LIT`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:101)
- [ ] `EntityRadarCategoryHardRules.IS_UNLIT`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:105)
- [ ] `EntityRadarCategoryHardRules.HAS_CUSTOM_NAME`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:109)
- [ ] `EntityRadarCategoryHardRules.NO_CUSTOM_NAME`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:113)
- [ ] `EntityRadarCategoryHardRules.IS_TRACKED`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:117)
- [ ] `EntityRadarCategoryHardRules.IS_IN_TEAM`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:121)
- [ ] `EntityRadarCategoryHardRules.IS_TEAMLESS`: it uses ObjectCategoryHardRule, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java:125)

### `EntityRadarListRuleTypes`

- [ ] `EntityRadarListRuleTypes.TYPE_LIST`: it uses ObjectCategoryListRuleType, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarListRuleTypes.java:25)
- [ ] `EntityRadarListRuleTypes.TYPE_MAP`: it uses ObjectCategoryListRuleType, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarListRuleTypes.java:26)
- [ ] `EntityRadarListRuleTypes.ENTITY_TYPE`: it uses ObjectCategoryListRuleType, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarListRuleTypes.java:27)
- [ ] `EntityRadarListRuleTypes.ITEM_TYPE`: it uses ObjectCategoryListRuleType, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarListRuleTypes.java:40)
- [ ] `EntityRadarListRuleTypes.PLAYER_NAME`: it uses ObjectCategoryListRuleType, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarListRuleTypes.java:53)
- [ ] `EntityRadarListRuleTypes.CUSTOM_NAME`: it uses ObjectCategoryListRuleType, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarListRuleTypes.java:66)
- [ ] `EntityRadarListRuleTypes.LIVING`: it uses ObjectCategoryListRuleType, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarListRuleTypes.java:82)
- [ ] `EntityRadarListRuleTypes.HOSTILE`: it uses ObjectCategoryListRuleType, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarListRuleTypes.java:85)
- [ ] `EntityRadarListRuleTypes.TAMED`: it uses ObjectCategoryListRuleType, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarListRuleTypes.java:88)
- [ ] `EntityRadarListRuleTypes.SAME_TEAM`: it uses ObjectCategoryListRuleType, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarListRuleTypes.java:91)
- [ ] `EntityRadarListRuleTypes.BABY`: it uses ObjectCategoryListRuleType, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarListRuleTypes.java:94)
- [ ] `EntityRadarListRuleTypes.VANILLA`: it uses ObjectCategoryListRuleType, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarListRuleTypes.java:97)
- [ ] `EntityRadarListRuleTypes.ABOVE_GROUND`: it uses ObjectCategoryListRuleType, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarListRuleTypes.java:100)
- [ ] `EntityRadarListRuleTypes.MY_GROUND`: it uses ObjectCategoryListRuleType, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarListRuleTypes.java:103)
- [ ] `EntityRadarListRuleTypes.LIT`: it uses ObjectCategoryListRuleType, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarListRuleTypes.java:106)
- [ ] `EntityRadarListRuleTypes.HAS_CUSTOM_NAME`: it uses ObjectCategoryListRuleType, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarListRuleTypes.java:109)
- [ ] `EntityRadarListRuleTypes.IN_TEAM`: it uses ObjectCategoryListRuleType, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarListRuleTypes.java:112)
- [ ] `EntityRadarListRuleTypes.TRACKED`: it uses ObjectCategoryListRuleType, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/rule/EntityRadarListRuleTypes.java:115)

### `EntityRadarCategoryData`: it extends `FilterObjectCategoryData` (xaero.hud.category.serialization.data)

- [ ] `EntityRadarCategoryData.EntityRadarCategoryData(9)`: its class isn't translated (it extends `FilterObjectCategoryData` (xaero.hud.category.serialization.data)) (xaero/hud/minimap/radar/category/serialization/data/EntityRadarCategoryData.java:10)

### `EntityRadarCategoryData.Builder`: it extends `Builder`

- [ ] `EntityRadarCategoryData.Builder.Builder(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/serialization/data/EntityRadarCategoryData.java:25)
- [ ] `EntityRadarCategoryData.Builder.buildInternally(1)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/serialization/data/EntityRadarCategoryData.java:29)
- [ ] `EntityRadarCategoryData.Builder.begin(0)`: it uses Builder, which isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/serialization/data/EntityRadarCategoryData.java:43)

### `EntityRadarCategorySerializationHandler`: it extends `FilterObjectCategorySerializationHandler` (xaero.hud.category.serialization)

- [ ] `EntityRadarCategorySerializationHandler.EntityRadarCategorySerializationHandler(8)`: its class isn't translated (it extends `FilterObjectCategorySerializationHandler` (xaero.hud.category.serialization)) (xaero/hud/minimap/radar/category/serialization/EntityRadarCategorySerializationHandler.java:21)

### `EntityRadarCategorySerializationHandler.Builder`: it extends `Builder`

- [ ] `EntityRadarCategorySerializationHandler.Builder.Builder(1)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/serialization/EntityRadarCategorySerializationHandler.java:45)
- [ ] `EntityRadarCategorySerializationHandler.Builder.setDefault(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/serialization/EntityRadarCategorySerializationHandler.java:49)
- [ ] `EntityRadarCategorySerializationHandler.Builder.buildInternally(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/serialization/EntityRadarCategorySerializationHandler.java:60)
- [ ] `EntityRadarCategorySerializationHandler.Builder.begin(1)`: it uses Builder, which isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/serialization/EntityRadarCategorySerializationHandler.java:73)

### `EntityRadarCategorySettings`

- [x] `EntityRadarCategorySettings.ENTITY_ICONS_OPTIONS` (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:15)
- [x] `EntityRadarCategorySettings.ENTITY_NAMES_OPTIONS` (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:16)
- [x] `EntityRadarCategorySettings.RADAR_OVER_MAP_OPTIONS` (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:17)
- [ ] `EntityRadarCategorySettings.SETTINGS`: it uses ObjectCategorySetting, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:20)
- [ ] `EntityRadarCategorySettings.SETTINGS_LIST`: it uses ObjectCategorySetting, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:21)
- [ ] `EntityRadarCategorySettings.DISPLAYED`: it uses ObjectCategorySetting, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:22)
- [ ] `EntityRadarCategorySettings.COLOR`: it uses ObjectCategorySetting, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:33)
- [ ] `EntityRadarCategorySettings.ICONS`: it uses ObjectCategorySetting, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:55)
- [ ] `EntityRadarCategorySettings.NAMES`: it uses ObjectCategorySetting, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:67)
- [ ] `EntityRadarCategorySettings.DOT_SIZE`: it uses ObjectCategorySetting, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:79)
- [ ] `EntityRadarCategorySettings.ICON_SCALE`: it uses ObjectCategorySetting, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:90)
- [ ] `EntityRadarCategorySettings.HEIGHT_LIMIT`: it uses ObjectCategorySetting, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:101)
- [ ] `EntityRadarCategorySettings.HEIGHT_FADE`: it uses ObjectCategorySetting, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:113)
- [ ] `EntityRadarCategorySettings.DISPLAY_Y`: it uses ObjectCategorySetting, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:125)
- [ ] `EntityRadarCategorySettings.START_FADING_AT`: it uses ObjectCategorySetting, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:148)
- [ ] `EntityRadarCategorySettings.ENTITY_NUMBER`: it uses ObjectCategorySetting, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:160)
- [ ] `EntityRadarCategorySettings.ALWAYS_NAMETAGS`: it uses ObjectCategorySetting, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:172)
- [ ] `EntityRadarCategorySettings.ICON_NAME_FALLBACK`: it uses ObjectCategorySetting, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:184)
- [ ] `EntityRadarCategorySettings.RENDER_OVER_MINIMAP`: it uses ObjectCategorySetting, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:195)
- [ ] `EntityRadarCategorySettings.RENDER_ORDER`: it uses ObjectCategorySetting, which isn't translated (it's generic) (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java:207)

### `EditorEntityRadarCategoryNodeConverter`: it extends `EditorFilterCategoryNodeConverter` (xaero.hud.category.ui)

- [ ] `EditorEntityRadarCategoryNodeConverter.EditorEntityRadarCategoryNodeConverter(6)`: its class isn't translated (it extends `EditorFilterCategoryNodeConverter` (xaero.hud.category.ui)) (xaero/hud/minimap/radar/category/ui/EditorEntityRadarCategoryNodeConverter.java:18)

### `EditorEntityRadarCategoryNodeConverter.Builder`: it extends `Builder`

- [ ] `EditorEntityRadarCategoryNodeConverter.Builder.Builder(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/ui/EditorEntityRadarCategoryNodeConverter.java:33)
- [ ] `EditorEntityRadarCategoryNodeConverter.Builder.setDefault(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/ui/EditorEntityRadarCategoryNodeConverter.java:37)
- [ ] `EditorEntityRadarCategoryNodeConverter.Builder.buildInternally(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/ui/EditorEntityRadarCategoryNodeConverter.java:44)
- [ ] `EditorEntityRadarCategoryNodeConverter.Builder.begin(0)`: it uses Builder, which isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/ui/EditorEntityRadarCategoryNodeConverter.java:55)

### `EntityRadarCategoryUIConstants`

- [ ] `EntityRadarCategoryUIConstants.SETTING_NODE_BUILDER_FACTORIES`: it uses SettingNodeBuilderFactoryManager, which isn't translated (it uses SettingNodeBuilderFactory, which isn't translated (it's an interface)) (xaero/hud/minimap/radar/category/ui/EntityRadarCategoryUIConstants.java:7)

### `EditorEntityRadarCategoryNode`: it extends `EditorFilterCategoryNode` (xaero.hud.category.ui.node)

- [ ] `EditorEntityRadarCategoryNode.EditorEntityRadarCategoryNode(8)`: its class isn't translated (it extends `EditorFilterCategoryNode` (xaero.hud.category.ui.node)) (xaero/hud/minimap/radar/category/ui/node/EditorEntityRadarCategoryNode.java:15)

### `EditorEntityRadarCategoryNode.Builder`: it extends `Builder`

- [ ] `EditorEntityRadarCategoryNode.Builder.Builder(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/ui/node/EditorEntityRadarCategoryNode.java:30)
- [ ] `EditorEntityRadarCategoryNode.Builder.setDefault(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/ui/node/EditorEntityRadarCategoryNode.java:34)
- [ ] `EditorEntityRadarCategoryNode.Builder.buildInternally(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/ui/node/EditorEntityRadarCategoryNode.java:40)
- [ ] `EditorEntityRadarCategoryNode.Builder.begin(0)`: it uses Builder, which isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/ui/node/EditorEntityRadarCategoryNode.java:53)

### `EditorEntityRadarCategorySettingsNode`: it's generic

- [ ] `EditorEntityRadarCategorySettingsNode.EditorEntityRadarCategorySettingsNode(14)`: its class isn't translated (it's generic) (xaero/hud/minimap/radar/category/ui/node/EditorEntityRadarCategorySettingsNode.java:31)

### `EditorEntityRadarCategorySettingsNode.Builder`: it extends `Builder`

- [ ] `EditorEntityRadarCategorySettingsNode.Builder.Builder(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/ui/node/EditorEntityRadarCategorySettingsNode.java:67)
- [ ] `EditorEntityRadarCategorySettingsNode.Builder.setDefault(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/ui/node/EditorEntityRadarCategorySettingsNode.java:77)
- [ ] `EditorEntityRadarCategorySettingsNode.Builder.begin(0)`: it uses Builder, which isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/ui/node/EditorEntityRadarCategorySettingsNode.java:101)
- [ ] `EditorEntityRadarCategorySettingsNode.Builder.buildInternally(2)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/radar/category/ui/node/EditorEntityRadarCategorySettingsNode.java:105)

### `RadarColor`: its constants carry values

- [ ] `RadarColor.RadarColor(3)`: its class isn't translated (its constants carry values) (xaero/hud/minimap/radar/color/RadarColor.java:28)
- [ ] `RadarColor.getName(0)`: its class isn't translated (its constants carry values) (xaero/hud/minimap/radar/color/RadarColor.java:34)
- [ ] `RadarColor.getFormat(0)`: its class isn't translated (its constants carry values) (xaero/hud/minimap/radar/color/RadarColor.java:38)
- [ ] `RadarColor.getHex(0)`: its class isn't translated (its constants carry values) (xaero/hud/minimap/radar/color/RadarColor.java:42)
- [ ] `RadarColor.fromIndex(1)`: it uses RadarColor, which isn't translated (its constants carry values) (xaero/hud/minimap/radar/color/RadarColor.java:46)
- [ ] `RadarColor.getRandom(0)`: it uses RadarColor, which isn't translated (its constants carry values) (xaero/hud/minimap/radar/color/RadarColor.java:50)

### `RadarColorHelper`

- [ ] `RadarColorHelper.getTeamColor(1)`: it uses Minecraft's `Entity` (xaero/hud/minimap/radar/color/RadarColorHelper.java:11)
- [ ] `RadarColorHelper.getEntityColor(8)`: it uses Minecraft's `Entity` (xaero/hud/minimap/radar/color/RadarColorHelper.java:21)
- [ ] `RadarColorHelper.getRadarColorHex(3)`: it uses Minecraft's `Entity` (xaero/hud/minimap/radar/color/RadarColorHelper.java:45)
- [ ] `RadarColorHelper.getFallbackColor(1)`: it uses RadarColor, which isn't translated (its constants carry values) (xaero/hud/minimap/radar/color/RadarColorHelper.java:54)
- [ ] `RadarColorHelper.getFallbackColor(2)`: it uses RadarColor, which isn't translated (its constants carry values) (xaero/hud/minimap/radar/color/RadarColorHelper.java:58)
- [x] `RadarColorHelper.getEntityHeightFade(3)` (xaero/hud/minimap/radar/color/RadarColorHelper.java:82)

### `RadarIconArmor`: it uses Minecraft's `Item`

- [ ] `RadarIconArmor.RadarIconArmor(3)`: its class isn't translated (it uses Minecraft's `Item`) (xaero/hud/minimap/radar/icon/cache/id/armor/RadarIconArmor.java:13)
- [ ] `RadarIconArmor.toString(0)`: its class isn't translated (it uses Minecraft's `Item`) (xaero/hud/minimap/radar/icon/cache/id/armor/RadarIconArmor.java:19)
- [ ] `RadarIconArmor.equals(1)`: its class isn't translated (it uses Minecraft's `Item`) (xaero/hud/minimap/radar/icon/cache/id/armor/RadarIconArmor.java:24)
- [ ] `RadarIconArmor.hashCode(0)`: its class isn't translated (it uses Minecraft's `Item`) (xaero/hud/minimap/radar/icon/cache/id/armor/RadarIconArmor.java:36)

### `RadarIconArmorHandler`

- [ ] `RadarIconArmorHandler.getArmor(1)`: it uses RadarIconArmor, which isn't translated (it uses Minecraft's `Item`) (xaero/hud/minimap/radar/icon/cache/id/armor/RadarIconArmorHandler.java:16)

### `RadarIconKey`: it uses `Object`

- [ ] `RadarIconKey.RadarIconKey(2)`: its class isn't translated (it uses `Object`) (xaero/hud/minimap/radar/icon/cache/id/RadarIconKey.java:10)
- [ ] `RadarIconKey.getVariant(0)`: its class isn't translated (it uses `Object`) (xaero/hud/minimap/radar/icon/cache/id/RadarIconKey.java:15)
- [ ] `RadarIconKey.toString(0)`: its class isn't translated (it uses `Object`) (xaero/hud/minimap/radar/icon/cache/id/RadarIconKey.java:19)
- [ ] `RadarIconKey.equals(1)`: its class isn't translated (it uses `Object`) (xaero/hud/minimap/radar/icon/cache/id/RadarIconKey.java:24)
- [ ] `RadarIconKey.hashCode(0)`: its class isn't translated (it uses `Object`) (xaero/hud/minimap/radar/icon/cache/id/RadarIconKey.java:36)

### `CushionVariant`: it uses Minecraft's `DyeColor`

- [ ] `CushionVariant.CushionVariant(1)`: its class isn't translated (it uses Minecraft's `DyeColor`) (xaero/hud/minimap/radar/icon/cache/id/variant/CushionVariant.java:9)
- [ ] `CushionVariant.equals(1)`: its class isn't translated (it uses Minecraft's `DyeColor`) (xaero/hud/minimap/radar/icon/cache/id/variant/CushionVariant.java:13)
- [ ] `CushionVariant.hashCode(0)`: its class isn't translated (it uses Minecraft's `DyeColor`) (xaero/hud/minimap/radar/icon/cache/id/variant/CushionVariant.java:23)
- [ ] `CushionVariant.toString(0)`: its class isn't translated (it uses Minecraft's `DyeColor`) (xaero/hud/minimap/radar/icon/cache/id/variant/CushionVariant.java:28)

### `EndermanVariant`

- [x] `EndermanVariant.EndermanVariant(2)` (xaero/hud/minimap/radar/icon/cache/id/variant/EndermanVariant.java:10)
- [x] `EndermanVariant.toString(0)` (xaero/hud/minimap/radar/icon/cache/id/variant/EndermanVariant.java:15)
- [ ] `EndermanVariant.equals(1)`: it uses `Object` (xaero/hud/minimap/radar/icon/cache/id/variant/EndermanVariant.java:20)
- [ ] `EndermanVariant.hashCode(0)`: it calls `Objects` (java.util).hash (xaero/hud/minimap/radar/icon/cache/id/variant/EndermanVariant.java:32)

### `HorseVariant`: it uses Minecraft's `Markings`

- [ ] `HorseVariant.HORSE_MARKINGS`: it uses Minecraft's `Markings` (xaero/hud/minimap/radar/icon/cache/id/variant/HorseVariant.java:11)
- [ ] `HorseVariant.HorseVariant(2)`: its class isn't translated (it uses Minecraft's `Markings`) (xaero/hud/minimap/radar/icon/cache/id/variant/HorseVariant.java:21)
- [ ] `HorseVariant.toString(0)`: its class isn't translated (it uses Minecraft's `Markings`) (xaero/hud/minimap/radar/icon/cache/id/variant/HorseVariant.java:26)
- [ ] `HorseVariant.equals(1)`: its class isn't translated (it uses Minecraft's `Markings`) (xaero/hud/minimap/radar/icon/cache/id/variant/HorseVariant.java:31)
- [ ] `HorseVariant.hashCode(0)`: its class isn't translated (it uses Minecraft's `Markings`) (xaero/hud/minimap/radar/icon/cache/id/variant/HorseVariant.java:43)

### `IronGolemVariant`: it uses Minecraft's `Level`

- [ ] `IronGolemVariant.IronGolemVariant(2)`: its class isn't translated (it uses Minecraft's `Level`) (xaero/hud/minimap/radar/icon/cache/id/variant/IronGolemVariant.java:11)
- [ ] `IronGolemVariant.toString(0)`: its class isn't translated (it uses Minecraft's `Level`) (xaero/hud/minimap/radar/icon/cache/id/variant/IronGolemVariant.java:16)
- [ ] `IronGolemVariant.equals(1)`: its class isn't translated (it uses Minecraft's `Level`) (xaero/hud/minimap/radar/icon/cache/id/variant/IronGolemVariant.java:21)
- [ ] `IronGolemVariant.hashCode(0)`: its class isn't translated (it uses Minecraft's `Level`) (xaero/hud/minimap/radar/icon/cache/id/variant/IronGolemVariant.java:33)

### `LlamaVariant`: it uses Minecraft's `Item`

- [ ] `LlamaVariant.LlamaVariant(3)`: its class isn't translated (it uses Minecraft's `Item`) (xaero/hud/minimap/radar/icon/cache/id/variant/LlamaVariant.java:12)
- [ ] `LlamaVariant.toString(0)`: its class isn't translated (it uses Minecraft's `Item`) (xaero/hud/minimap/radar/icon/cache/id/variant/LlamaVariant.java:18)
- [ ] `LlamaVariant.equals(1)`: its class isn't translated (it uses Minecraft's `Item`) (xaero/hud/minimap/radar/icon/cache/id/variant/LlamaVariant.java:23)
- [ ] `LlamaVariant.hashCode(0)`: its class isn't translated (it uses Minecraft's `Item`) (xaero/hud/minimap/radar/icon/cache/id/variant/LlamaVariant.java:35)

### `RadarIconVariantHandler`

- [ ] `RadarIconVariantHandler.getEntityVariant(4)`: it's generic (xaero/hud/minimap/radar/icon/cache/id/variant/RadarIconVariantHandler.java:18)
- [ ] `RadarIconVariantHandler.getLegacyVariantId(3)`: it's generic (xaero/hud/minimap/radar/icon/cache/id/variant/RadarIconVariantHandler.java:61)

### `SaddleVariant`

- [x] `SaddleVariant.SaddleVariant(2)` (xaero/hud/minimap/radar/icon/cache/id/variant/SaddleVariant.java:10)
- [x] `SaddleVariant.toString(0)` (xaero/hud/minimap/radar/icon/cache/id/variant/SaddleVariant.java:15)
- [ ] `SaddleVariant.equals(1)`: it uses `Object` (xaero/hud/minimap/radar/icon/cache/id/variant/SaddleVariant.java:20)
- [ ] `SaddleVariant.hashCode(0)`: it calls `Objects` (java.util).hash (xaero/hud/minimap/radar/icon/cache/id/variant/SaddleVariant.java:32)

### `TamableVariant`

- [x] `TamableVariant.TamableVariant(2)` (xaero/hud/minimap/radar/icon/cache/id/variant/TamableVariant.java:10)
- [x] `TamableVariant.toString(0)` (xaero/hud/minimap/radar/icon/cache/id/variant/TamableVariant.java:15)
- [ ] `TamableVariant.equals(1)`: it uses `Object` (xaero/hud/minimap/radar/icon/cache/id/variant/TamableVariant.java:20)
- [ ] `TamableVariant.hashCode(0)`: it calls `Objects` (java.util).hash (xaero/hud/minimap/radar/icon/cache/id/variant/TamableVariant.java:32)

### `TropicalFishVariant`: it uses Minecraft's `DyeColor`

- [ ] `TropicalFishVariant.TropicalFishVariant(4)`: its class isn't translated (it uses Minecraft's `DyeColor`) (xaero/hud/minimap/radar/icon/cache/id/variant/TropicalFishVariant.java:14)
- [ ] `TropicalFishVariant.toString(0)`: its class isn't translated (it uses Minecraft's `DyeColor`) (xaero/hud/minimap/radar/icon/cache/id/variant/TropicalFishVariant.java:21)
- [ ] `TropicalFishVariant.equals(1)`: its class isn't translated (it uses Minecraft's `DyeColor`) (xaero/hud/minimap/radar/icon/cache/id/variant/TropicalFishVariant.java:26)
- [ ] `TropicalFishVariant.hashCode(0)`: its class isn't translated (it uses Minecraft's `DyeColor`) (xaero/hud/minimap/radar/icon/cache/id/variant/TropicalFishVariant.java:41)

### `VillagerVariant`

- [x] `VillagerVariant.VillagerVariant(5)` (xaero/hud/minimap/radar/icon/cache/id/variant/VillagerVariant.java:13)
- [x] `VillagerVariant.toString(0)` (xaero/hud/minimap/radar/icon/cache/id/variant/VillagerVariant.java:21)
- [ ] `VillagerVariant.equals(1)`: it uses `Object` (xaero/hud/minimap/radar/icon/cache/id/variant/VillagerVariant.java:26)
- [ ] `VillagerVariant.hashCode(0)`: it calls `Objects` (java.util).hash (xaero/hud/minimap/radar/icon/cache/id/variant/VillagerVariant.java:42)

### `RadarIconCache`: it uses Minecraft's `EntityType`

- [ ] `RadarIconCache.getEntityCache(1)`: its class isn't translated (it uses Minecraft's `EntityType`) (xaero/hud/minimap/radar/icon/cache/RadarIconCache.java:10)
- [ ] `RadarIconCache.clear(0)`: its class isn't translated (it uses Minecraft's `EntityType`) (xaero/hud/minimap/radar/icon/cache/RadarIconCache.java:19)

### `RadarIconEntityCache`: it uses Minecraft's `EntityType`

- [ ] `RadarIconEntityCache.RadarIconEntityCache(1)`: its class isn't translated (it uses Minecraft's `EntityType`) (xaero/hud/minimap/radar/icon/cache/RadarIconEntityCache.java:18)
- [ ] `RadarIconEntityCache.get(1)`: its class isn't translated (it uses Minecraft's `EntityType`) (xaero/hud/minimap/radar/icon/cache/RadarIconEntityCache.java:24)
- [ ] `RadarIconEntityCache.add(2)`: its class isn't translated (it uses Minecraft's `EntityType`) (xaero/hud/minimap/radar/icon/cache/RadarIconEntityCache.java:37)
- [ ] `RadarIconEntityCache.getVariantString(1)`: its class isn't translated (it uses Minecraft's `EntityType`) (xaero/hud/minimap/radar/icon/cache/RadarIconEntityCache.java:87)
- [ ] `RadarIconEntityCache.isInvalidVariantClass(0)`: its class isn't translated (it uses Minecraft's `EntityType`) (xaero/hud/minimap/radar/icon/cache/RadarIconEntityCache.java:97)

### `LivingEntityPoseResetter`

- [ ] `LivingEntityPoseResetter.resetValues(1)`: it uses Minecraft's `LivingEntityRenderState` (xaero/hud/minimap/radar/icon/creator/entity/LivingEntityPoseResetter.java:8)

### `RadarIconCreator`: it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)

- [x] `RadarIconCreator.PREFERRED_ATLAS_WIDTH` (xaero/hud/minimap/radar/icon/creator/RadarIconCreator.java:48)
- [x] `RadarIconCreator.ICON_WIDTH` (xaero/hud/minimap/radar/icon/creator/RadarIconCreator.java:49)
- [x] `RadarIconCreator.FAR_PLANE` (xaero/hud/minimap/radar/icon/creator/RadarIconCreator.java:50)
- [ ] `RadarIconCreator.RadarIconCreator(0)`: its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)) (xaero/hud/minimap/radar/icon/creator/RadarIconCreator.java:63)
- [ ] `RadarIconCreator.create(6)`: it's generic (xaero/hud/minimap/radar/icon/creator/RadarIconCreator.java:74)
- [ ] `RadarIconCreator.getFinalIcon(4)`: its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)) (xaero/hud/minimap/radar/icon/creator/RadarIconCreator.java:148)
- [ ] `RadarIconCreator.renderOutline(1)`: its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)) (xaero/hud/minimap/radar/icon/creator/RadarIconCreator.java:238)
- [ ] `RadarIconCreator.setupMatrices(3)`: its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)) (xaero/hud/minimap/radar/icon/creator/RadarIconCreator.java:252)
- [ ] `RadarIconCreator.restoreMatrices(1)`: its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)) (xaero/hud/minimap/radar/icon/creator/RadarIconCreator.java:264)
- [ ] `RadarIconCreator.endFormRendering(0)`: its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)) (xaero/hud/minimap/radar/icon/creator/RadarIconCreator.java:270)
- [ ] `RadarIconCreator.bindDefaultFramebuffer(1)`: its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)) (xaero/hud/minimap/radar/icon/creator/RadarIconCreator.java:274)
- [ ] `RadarIconCreator.reset(0)`: its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)) (xaero/hud/minimap/radar/icon/creator/RadarIconCreator.java:286)
- [ ] `RadarIconCreator.getRenderTracer(0)`: its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)) (xaero/hud/minimap/radar/icon/creator/RadarIconCreator.java:292)
- [ ] `RadarIconCreator.getCurrentAtlas(0)`: its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)) (xaero/hud/minimap/radar/icon/creator/RadarIconCreator.java:296)
- [ ] `RadarIconCreator.initFramebuffers(1)`: its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)) (xaero/hud/minimap/radar/icon/creator/RadarIconCreator.java:300)

### `RadarIconCreator.Parameters`: it uses `Object`

- [ ] `RadarIconCreator.Parameters.Parameters(5)`: its class isn't translated (it uses `Object`) (xaero/hud/minimap/radar/icon/creator/RadarIconCreator.java:325)

### `IRadarIconFormPrerenderer`: it's an interface

- [ ] `IRadarIconFormPrerenderer.requiresEntityModel(0)`: it has no body (xaero/hud/minimap/radar/icon/creator/render/form/IRadarIconFormPrerenderer.java:15)
- [ ] `IRadarIconFormPrerenderer.isFlipped(0)`: it has no body (xaero/hud/minimap/radar/icon/creator/render/form/IRadarIconFormPrerenderer.java:17)
- [ ] `IRadarIconFormPrerenderer.isOutlined(0)`: it has no body (xaero/hud/minimap/radar/icon/creator/render/form/IRadarIconFormPrerenderer.java:19)
- [ ] `IRadarIconFormPrerenderer.prerender(8)`: it has no body (xaero/hud/minimap/radar/icon/creator/render/form/IRadarIconFormPrerenderer.java:21)

### `RadarIconItemFormPrerenderer`: it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)

- [ ] `RadarIconItemFormPrerenderer.requiresEntityModel(0)`: its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)) (xaero/hud/minimap/radar/icon/creator/render/form/item/RadarIconItemFormPrerenderer.java:39)
- [ ] `RadarIconItemFormPrerenderer.isFlipped(0)`: its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)) (xaero/hud/minimap/radar/icon/creator/render/form/item/RadarIconItemFormPrerenderer.java:44)
- [ ] `RadarIconItemFormPrerenderer.isOutlined(0)`: its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)) (xaero/hud/minimap/radar/icon/creator/render/form/item/RadarIconItemFormPrerenderer.java:49)
- [ ] `RadarIconItemFormPrerenderer.prerender(8)`: it's generic (xaero/hud/minimap/radar/icon/creator/render/form/item/RadarIconItemFormPrerenderer.java:54)
- [ ] `RadarIconItemFormPrerenderer.getItemToRender(2)`: its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)) (xaero/hud/minimap/radar/icon/creator/render/form/item/RadarIconItemFormPrerenderer.java:137)
- [ ] `RadarIconItemFormPrerenderer.getSelfItem(1)`: its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)) (xaero/hud/minimap/radar/icon/creator/render/form/item/RadarIconItemFormPrerenderer.java:149)

### `CustomModelIconCustomPrenderer`: it extends `RenderTypeIconCustomPrerenderer`

- [ ] `CustomModelIconCustomPrenderer.getRenderedDest(1)`: its class isn't translated (it extends `RenderTypeIconCustomPrerenderer`) (xaero/hud/minimap/radar/icon/creator/render/form/model/custom/CustomModelIconCustomPrenderer.java:11)
- [ ] `CustomModelIconCustomPrenderer.getModelParts(4)`: it has no body (xaero/hud/minimap/radar/icon/creator/render/form/model/custom/CustomModelIconCustomPrenderer.java:16)

### `LayeredIconCustomPrerenderer`: it extends `RadarIconCustomPrerenderer`

- [ ] `LayeredIconCustomPrerenderer.LayeredIconCustomPrerenderer(1)`: its class isn't translated (it extends `RadarIconCustomPrerenderer`) (xaero/hud/minimap/radar/icon/creator/render/form/model/custom/LayeredIconCustomPrerenderer.java:18)
- [ ] `LayeredIconCustomPrerenderer.render(11)`: it's generic (xaero/hud/minimap/radar/icon/creator/render/form/model/custom/LayeredIconCustomPrerenderer.java:22)

### `RadarIconCustomPrerenderer`

- [ ] `RadarIconCustomPrerenderer.render(11)`: it has no body (xaero/hud/minimap/radar/icon/creator/render/form/model/custom/RadarIconCustomPrerenderer.java:16)

### `RenderTypeIconCustomPrerenderer`: it extends `RadarIconCustomPrerenderer`

- [ ] `RenderTypeIconCustomPrerenderer.render(11)`: it's generic (xaero/hud/minimap/radar/icon/creator/render/form/model/custom/RenderTypeIconCustomPrerenderer.java:19)
- [ ] `RenderTypeIconCustomPrerenderer.getModelParts(4)`: it's generic (xaero/hud/minimap/radar/icon/creator/render/form/model/custom/RenderTypeIconCustomPrerenderer.java:47)
- [ ] `RenderTypeIconCustomPrerenderer.getRenderedDest(1)`: its class isn't translated (it extends `RadarIconCustomPrerenderer`) (xaero/hud/minimap/radar/icon/creator/render/form/model/custom/RenderTypeIconCustomPrerenderer.java:53)
- [ ] `RenderTypeIconCustomPrerenderer.getRenderType(2)`: it has no body (xaero/hud/minimap/radar/icon/creator/render/form/model/custom/RenderTypeIconCustomPrerenderer.java:57)

### `ModelPartUtil`

- [ ] `ModelPartUtil.CUBES_FIELD`: it uses `Field` (java.lang.reflect) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/ModelPartUtil.java:11)
- [ ] `ModelPartUtil.CHILDREN_FIELD`: it uses `Field` (java.lang.reflect) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/ModelPartUtil.java:12)
- [ ] `ModelPartUtil.getCubes(1)`: it uses Minecraft's `Cube` (xaero/hud/minimap/radar/icon/creator/render/form/model/part/ModelPartUtil.java:14)
- [ ] `ModelPartUtil.getChildren(1)`: it uses Minecraft's `ModelPart` (xaero/hud/minimap/radar/icon/creator/render/form/model/part/ModelPartUtil.java:18)
- [ ] `ModelPartUtil.hasDirectCubes(1)`: it uses Minecraft's `ModelPart` (xaero/hud/minimap/radar/icon/creator/render/form/model/part/ModelPartUtil.java:22)
- [ ] `ModelPartUtil.hasCubes(1)`: it uses Minecraft's `ModelPart` (xaero/hud/minimap/radar/icon/creator/render/form/model/part/ModelPartUtil.java:27)
- [ ] `ModelPartUtil.getBiggestCuboid(1)`: it uses Minecraft's `Cube` (xaero/hud/minimap/radar/icon/creator/render/form/model/part/ModelPartUtil.java:43)

### `RadarIconModelPartPrerenderer`: it uses `Field` (java.lang.reflect)

- [ ] `RadarIconModelPartPrerenderer.renderPart(5)`: its class isn't translated (it uses `Field` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java:33)
- [ ] `RadarIconModelPartPrerenderer.testRenderEngineWrapperCompatibility(3)`: its class isn't translated (it uses `Field` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java:100)
- [ ] `RadarIconModelPartPrerenderer.renderDeclaredMethod(6)`: it's generic (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java:146)
- [ ] `RadarIconModelPartPrerenderer.renderPartsIterable(5)`: its class isn't translated (it uses `Field` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java:161)

### `RadarIconModelPartPrerenderer.Parameters`: it uses Minecraft's `ModelPart`

- [ ] `RadarIconModelPartPrerenderer.Parameters.Parameters(3)`: its class isn't translated (it uses Minecraft's `ModelPart`) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java:190)

### `RadarIconModelPartPrerenderer.VertexConsumerWrapper`: it implements Minecraft's `VertexConsumer`

- [ ] `RadarIconModelPartPrerenderer.VertexConsumerWrapper.prepareDetection(7)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java:207)
- [ ] `RadarIconModelPartPrerenderer.VertexConsumerWrapper.addVertex(3)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java:227)
- [ ] `RadarIconModelPartPrerenderer.VertexConsumerWrapper.setColor(4)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java:240)
- [ ] `RadarIconModelPartPrerenderer.VertexConsumerWrapper.setColor(1)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java:244)
- [ ] `RadarIconModelPartPrerenderer.VertexConsumerWrapper.setUv(2)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java:248)
- [ ] `RadarIconModelPartPrerenderer.VertexConsumerWrapper.setUv1(2)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java:252)
- [ ] `RadarIconModelPartPrerenderer.VertexConsumerWrapper.setOverlay(1)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java:256)
- [ ] `RadarIconModelPartPrerenderer.VertexConsumerWrapper.setUv2(2)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java:260)
- [ ] `RadarIconModelPartPrerenderer.VertexConsumerWrapper.setUv3(2)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java:264)
- [ ] `RadarIconModelPartPrerenderer.VertexConsumerWrapper.setNormal(3)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java:268)
- [ ] `RadarIconModelPartPrerenderer.VertexConsumerWrapper.setLineWidth(1)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java:272)
- [ ] `RadarIconModelPartPrerenderer.VertexConsumerWrapper.hasDetectedVertex(0)`: its class isn't translated (it implements Minecraft's `VertexConsumer`) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java:276)

### `ResolvedFieldModelPartRenderer`: it implements `Listener`

- [ ] `ResolvedFieldModelPartRenderer.prepare(6)`: its class isn't translated (it implements `Listener`) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/ResolvedFieldModelPartRenderer.java:21)
- [ ] `ResolvedFieldModelPartRenderer.isFieldAllowed(1)`: its class isn't translated (it implements `Listener`) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/ResolvedFieldModelPartRenderer.java:38)
- [ ] `ResolvedFieldModelPartRenderer.shouldStop(0)`: its class isn't translated (it implements `Listener`) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/ResolvedFieldModelPartRenderer.java:61)
- [ ] `ResolvedFieldModelPartRenderer.onFieldResolved(2)`: its class isn't translated (it implements `Listener`) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/ResolvedFieldModelPartRenderer.java:66)
- [ ] `ResolvedFieldModelPartRenderer.getMainPart(0)`: its class isn't translated (it implements `Listener`) (xaero/hud/minimap/radar/icon/creator/render/form/model/part/ResolvedFieldModelPartRenderer.java:88)

### `RadarIconModelFormPrerenderer`: it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)

- [ ] `RadarIconModelFormPrerenderer.requiresEntityModel(0)`: its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)) (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelFormPrerenderer.java:57)
- [ ] `RadarIconModelFormPrerenderer.isFlipped(0)`: its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)) (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelFormPrerenderer.java:62)
- [ ] `RadarIconModelFormPrerenderer.isOutlined(0)`: its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)) (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelFormPrerenderer.java:67)
- [ ] `RadarIconModelFormPrerenderer.prerender(8)`: it's generic (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelFormPrerenderer.java:72)
- [ ] `RadarIconModelFormPrerenderer.renderLayers(9)`: it's generic (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelFormPrerenderer.java:136)
- [ ] `RadarIconModelFormPrerenderer.addDefaultLayer(5)`: it's generic (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelFormPrerenderer.java:218)
- [ ] `RadarIconModelFormPrerenderer.renderLayer(8)`: it's generic (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelFormPrerenderer.java:243)
- [ ] `RadarIconModelFormPrerenderer.resetModelRotations(2)`: it's generic (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelFormPrerenderer.java:324)

### `RadarIconModelPrerenderer`: it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect))

- [x] `RadarIconModelPrerenderer.EXTRA_PART_NAMES` (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelPrerenderer.java:41)
- [ ] `RadarIconModelPrerenderer.ONE_RENDERER_ARRAY`: it uses `Object` (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelPrerenderer.java:42)
- [ ] `RadarIconModelPrerenderer.ONE_OBJECT_ARRAY`: it uses `Object` (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelPrerenderer.java:43)
- [x] `RadarIconModelPrerenderer.LIGHTMAP` (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelPrerenderer.java:44)
- [ ] `RadarIconModelPrerenderer.RadarIconModelPrerenderer(0)`: its class isn't translated (it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect))) (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelPrerenderer.java:52)
- [ ] `RadarIconModelPrerenderer.renderModel(7)`: its class isn't translated (it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect))) (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelPrerenderer.java:63)
- [ ] `RadarIconModelPrerenderer.renderAgeableListModel(4)`: its class isn't translated (it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect))) (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelPrerenderer.java:127)
- [ ] `RadarIconModelPrerenderer.renderHierarchicalModel(5)`: its class isn't translated (it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect))) (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelPrerenderer.java:138)
- [ ] `RadarIconModelPrerenderer.renderPart(4)`: its class isn't translated (it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect))) (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelPrerenderer.java:205)
- [ ] `RadarIconModelPrerenderer.searchAndRenderFields(6)`: its class isn't translated (it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect))) (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelPrerenderer.java:213)
- [ ] `RadarIconModelPrerenderer.getLayerModelVertexConsumer(4)`: its class isn't translated (it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect))) (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelPrerenderer.java:226)
- [ ] `RadarIconModelPrerenderer.resolveModelRoot(3)`: its class isn't translated (it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect))) (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelPrerenderer.java:242)
- [ ] `RadarIconModelPrerenderer.getPartPrerenderer(0)`: its class isn't translated (it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect))) (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelPrerenderer.java:260)

### `RadarIconModelPrerenderer.Parameters`: it extends `Parameters`

- [ ] `RadarIconModelPrerenderer.Parameters.Parameters(10)`: its class isn't translated (it extends `Parameters`) (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelPrerenderer.java:273)

### `RadarIconModelFieldResolver`

- [ ] `RadarIconModelFieldResolver.KEYS_PARSER`: it uses SeparatedKeysParser, which isn't translated (a function kept in a variable or field (only parameters can be)) (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java:14)
- [ ] `RadarIconModelFieldResolver.handleDeclaredField(4)`: it uses `Object` (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java:16)
- [ ] `RadarIconModelFieldResolver.searchSuperclassFields(4)`: it uses `Object` (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java:54)
- [ ] `RadarIconModelFieldResolver.handleFields(5)`: it uses `Object` (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java:68)
- [ ] `RadarIconModelFieldResolver.passesFilter(2)`: String.lastIndexOf with 1 values (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java:93)
- [ ] `RadarIconModelFieldResolver.getReferenceType(1)`: it uses FieldReferenceType, which isn't translated (it's generic) (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java:108)

### `RadarIconModelFieldResolver.FieldReferenceElementGetter`: it's an interface

- [ ] `RadarIconModelFieldResolver.FieldReferenceElementGetter.get(3)`: it has no body (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java:120)

### `RadarIconModelFieldResolver.FieldReferenceType`: it's generic

- [ ] `RadarIconModelFieldResolver.FieldReferenceType.SINGLE`: it uses FieldReferenceType, which isn't translated (it's generic) (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java:124)
- [ ] `RadarIconModelFieldResolver.FieldReferenceType.ARRAY`: it uses FieldReferenceType, which isn't translated (it's generic) (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java:130)
- [ ] `RadarIconModelFieldResolver.FieldReferenceType.COLLECTION`: it uses FieldReferenceType, which isn't translated (it's generic) (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java:133)
- [ ] `RadarIconModelFieldResolver.FieldReferenceType.MAP`: it uses FieldReferenceType, which isn't translated (it's generic) (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java:136)
- [ ] `RadarIconModelFieldResolver.FieldReferenceType.FieldReferenceType(2)`: its class isn't translated (it's generic) (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java:151)
- [ ] `RadarIconModelFieldResolver.FieldReferenceType.getArray(2)`: its class isn't translated (it's generic) (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java:156)
- [ ] `RadarIconModelFieldResolver.FieldReferenceType.getElement(3)`: its class isn't translated (it's generic) (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java:160)

### `RadarIconModelFieldResolver.Listener`: it's an interface

- [ ] `RadarIconModelFieldResolver.Listener.isFieldAllowed(1)`: it has no body (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java:166)
- [ ] `RadarIconModelFieldResolver.Listener.shouldStop(0)`: it has no body (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java:168)
- [ ] `RadarIconModelFieldResolver.Listener.onFieldResolved(2)`: it has no body (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java:170)

### `ResolvedFieldModelRootPathListener`: it implements `Listener`

- [ ] `ResolvedFieldModelRootPathListener.prepare(0)`: its class isn't translated (it implements `Listener`) (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/ResolvedFieldModelRootPathListener.java:11)
- [ ] `ResolvedFieldModelRootPathListener.isFieldAllowed(1)`: its class isn't translated (it implements `Listener`) (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/ResolvedFieldModelRootPathListener.java:17)
- [ ] `ResolvedFieldModelRootPathListener.shouldStop(0)`: its class isn't translated (it implements `Listener`) (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/ResolvedFieldModelRootPathListener.java:22)
- [ ] `ResolvedFieldModelRootPathListener.onFieldResolved(2)`: its class isn't translated (it implements `Listener`) (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/ResolvedFieldModelRootPathListener.java:27)
- [ ] `ResolvedFieldModelRootPathListener.getCurrentNode(0)`: its class isn't translated (it implements `Listener`) (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/ResolvedFieldModelRootPathListener.java:39)
- [ ] `ResolvedFieldModelRootPathListener.failed(0)`: its class isn't translated (it implements `Listener`) (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/ResolvedFieldModelRootPathListener.java:43)

### `RadarIconSpriteFormPrerenderer`: it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)

- [ ] `RadarIconSpriteFormPrerenderer.RadarIconSpriteFormPrerenderer(2)`: its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)) (xaero/hud/minimap/radar/icon/creator/render/form/sprite/RadarIconSpriteFormPrerenderer.java:31)
- [ ] `RadarIconSpriteFormPrerenderer.requiresEntityModel(0)`: its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)) (xaero/hud/minimap/radar/icon/creator/render/form/sprite/RadarIconSpriteFormPrerenderer.java:36)
- [ ] `RadarIconSpriteFormPrerenderer.isFlipped(0)`: its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)) (xaero/hud/minimap/radar/icon/creator/render/form/sprite/RadarIconSpriteFormPrerenderer.java:41)
- [ ] `RadarIconSpriteFormPrerenderer.isOutlined(0)`: its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)) (xaero/hud/minimap/radar/icon/creator/render/form/sprite/RadarIconSpriteFormPrerenderer.java:46)
- [ ] `RadarIconSpriteFormPrerenderer.prerender(8)`: it's generic (xaero/hud/minimap/radar/icon/creator/render/form/sprite/RadarIconSpriteFormPrerenderer.java:51)

### `EntityRenderTracer`: it uses Minecraft's `Entity`

- [x] `EntityRenderTracer.TRACING_MODEL_RENDERS` (xaero/hud/minimap/radar/icon/creator/render/trace/EntityRenderTracer.java:55)
- [ ] `EntityRenderTracer.EntityRenderTracer(0)`: its class isn't translated (it uses Minecraft's `Entity`) (xaero/hud/minimap/radar/icon/creator/render/trace/EntityRenderTracer.java:69)
- [ ] `EntityRenderTracer.trace(5)`: it's generic (xaero/hud/minimap/radar/icon/creator/render/trace/EntityRenderTracer.java:75)
- [ ] `EntityRenderTracer.onModelPrepare(1)`: its class isn't translated (it uses Minecraft's `Entity`) (xaero/hud/minimap/radar/icon/creator/render/trace/EntityRenderTracer.java:159)
- [ ] `EntityRenderTracer.onModelRender(3)`: its class isn't translated (it uses Minecraft's `Entity`) (xaero/hud/minimap/radar/icon/creator/render/trace/EntityRenderTracer.java:163)
- [ ] `EntityRenderTracer.fixAdditiveBlendUncached(1)`: its class isn't translated (it uses Minecraft's `Entity`) (xaero/hud/minimap/radar/icon/creator/render/trace/EntityRenderTracer.java:231)
- [ ] `EntityRenderTracer.fixAdditiveBlendUncached(3)`: its class isn't translated (it uses Minecraft's `Entity`) (xaero/hud/minimap/radar/icon/creator/render/trace/EntityRenderTracer.java:242)
- [ ] `EntityRenderTracer.onModelPartRender(2)`: its class isn't translated (it uses Minecraft's `Entity`) (xaero/hud/minimap/radar/icon/creator/render/trace/EntityRenderTracer.java:273)
- [ ] `EntityRenderTracer.getRenderState(1)`: its class isn't translated (it uses Minecraft's `Entity`) (xaero/hud/minimap/radar/icon/creator/render/trace/EntityRenderTracer.java:279)
- [ ] `EntityRenderTracer.getEntityRendererModel(1)`: it's generic (xaero/hud/minimap/radar/icon/creator/render/trace/EntityRenderTracer.java:287)
- [ ] `EntityRenderTracer.initReflection(0)`: its class isn't translated (it uses Minecraft's `Entity`) (xaero/hud/minimap/radar/icon/creator/render/trace/EntityRenderTracer.java:297)
- [ ] `EntityRenderTracer.reset(0)`: its class isn't translated (it uses Minecraft's `Entity`) (xaero/hud/minimap/radar/icon/creator/render/trace/EntityRenderTracer.java:317)

### `ModelPartRenderTrace`: it uses Minecraft's `ModelPart`

- [ ] `ModelPartRenderTrace.ModelPartRenderTrace(2)`: its class isn't translated (it uses Minecraft's `ModelPart`) (xaero/hud/minimap/radar/icon/creator/render/trace/ModelPartRenderTrace.java:9)

### `ModelRenderTrace`: it uses Minecraft's `Model`

- [ ] `ModelRenderTrace.ModelRenderTrace(11)`: its class isn't translated (it uses Minecraft's `Model`) (xaero/hud/minimap/radar/icon/creator/render/trace/ModelRenderTrace.java:28)
- [ ] `ModelRenderTrace.toString(0)`: its class isn't translated (it uses Minecraft's `Model`) (xaero/hud/minimap/radar/icon/creator/render/trace/ModelRenderTrace.java:54)
- [ ] `ModelRenderTrace.addVisibleModelPart(2)`: its class isn't translated (it uses Minecraft's `Model`) (xaero/hud/minimap/radar/icon/creator/render/trace/ModelRenderTrace.java:59)
- [ ] `ModelRenderTrace.getModelPartRenderInfo(1)`: its class isn't translated (it uses Minecraft's `Model`) (xaero/hud/minimap/radar/icon/creator/render/trace/ModelRenderTrace.java:67)
- [ ] `ModelRenderTrace.isEmpty(0)`: its class isn't translated (it uses Minecraft's `Model`) (xaero/hud/minimap/radar/icon/creator/render/trace/ModelRenderTrace.java:76)
- [ ] `ModelRenderTrace.sameVisibility(1)`: its class isn't translated (it uses Minecraft's `Model`) (xaero/hud/minimap/radar/icon/creator/render/trace/ModelRenderTrace.java:80)

### `BuiltInRadarIconDefinitions`

- [ ] `BuiltInRadarIconDefinitions.BUILD_VARIANT_ID_STRING_METHOD`: it uses `Method` (java.lang.reflect) (xaero/hud/minimap/radar/icon/definition/BuiltInRadarIconDefinitions.java:103)
- [ ] `BuiltInRadarIconDefinitions.GET_VARIANT_ID_STRING_METHOD`: it uses `Method` (java.lang.reflect) (xaero/hud/minimap/radar/icon/definition/BuiltInRadarIconDefinitions.java:104)
- [ ] `BuiltInRadarIconDefinitions.getMainModelPartFields(3)`: it uses Minecraft's `EntityRenderer` (xaero/hud/minimap/radar/icon/definition/BuiltInRadarIconDefinitions.java:106)
- [ ] `BuiltInRadarIconDefinitions.getSecondaryModelPartsFields(3)`: it uses Minecraft's `EntityRenderer` (xaero/hud/minimap/radar/icon/definition/BuiltInRadarIconDefinitions.java:234)
- [ ] `BuiltInRadarIconDefinitions.getModelRoot(1)`: it uses `Object` (xaero/hud/minimap/radar/icon/definition/BuiltInRadarIconDefinitions.java:256)
- [ ] `BuiltInRadarIconDefinitions.forceFieldCheck(1)`: it uses Minecraft's `EntityModel` (xaero/hud/minimap/radar/icon/definition/BuiltInRadarIconDefinitions.java:278)
- [ ] `BuiltInRadarIconDefinitions.prepareEntityRenderState(1)`: it uses Minecraft's `EntityRenderState` (xaero/hud/minimap/radar/icon/definition/BuiltInRadarIconDefinitions.java:282)
- [ ] `BuiltInRadarIconDefinitions.defaultTransformation(2)`: it uses Minecraft's `EntityModel` (xaero/hud/minimap/radar/icon/definition/BuiltInRadarIconDefinitions.java:291)
- [ ] `BuiltInRadarIconDefinitions.defaultPostIconModelRender(2)`: it uses Minecraft's `EntityModel` (xaero/hud/minimap/radar/icon/definition/BuiltInRadarIconDefinitions.java:333)
- [ ] `BuiltInRadarIconDefinitions.fullModelIcon(1)`: it uses Minecraft's `EntityModel` (xaero/hud/minimap/radar/icon/definition/BuiltInRadarIconDefinitions.java:336)
- [ ] `BuiltInRadarIconDefinitions.getCustomLayer(2)`: it's generic (xaero/hud/minimap/radar/icon/definition/BuiltInRadarIconDefinitions.java:355)
- [ ] `BuiltInRadarIconDefinitions.getVariant(3)`: it's generic (xaero/hud/minimap/radar/icon/definition/BuiltInRadarIconDefinitions.java:359)

### `RadarIconItemForm`: it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)

- [ ] `RadarIconItemForm.RadarIconItemForm(2)`: its class isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)) (xaero/hud/minimap/radar/icon/definition/form/item/RadarIconItemForm.java:13)
- [ ] `RadarIconItemForm.getItemKey(0)`: its class isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)) (xaero/hud/minimap/radar/icon/definition/form/item/RadarIconItemForm.java:18)
- [ ] `RadarIconItemForm.read(3)`: it uses RadarIconItemForm, which isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)) (xaero/hud/minimap/radar/icon/definition/form/item/RadarIconItemForm.java:22)

### `RadarIconModelConfig`


### `RadarIconModelForm`: it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)

- [ ] `RadarIconModelForm.RadarIconModelForm(2)`: its class isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)) (xaero/hud/minimap/radar/icon/definition/form/model/RadarIconModelForm.java:14)
- [ ] `RadarIconModelForm.getConfig(0)`: its class isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)) (xaero/hud/minimap/radar/icon/definition/form/model/RadarIconModelForm.java:19)
- [ ] `RadarIconModelForm.read(3)`: it uses RadarIconModelForm, which isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)) (xaero/hud/minimap/radar/icon/definition/form/model/RadarIconModelForm.java:23)

### `RadarIconBasicForms`

- [ ] `RadarIconBasicForms.DEFAULT_MODEL`: it uses RadarIconModelForm, which isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)) (xaero/hud/minimap/radar/icon/definition/form/RadarIconBasicForms.java:8)
- [ ] `RadarIconBasicForms.SELF_ITEM`: it uses RadarIconItemForm, which isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)) (xaero/hud/minimap/radar/icon/definition/form/RadarIconBasicForms.java:9)
- [ ] `RadarIconBasicForms.DOT`: it uses RadarIconForm, which isn't translated (it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface))) (xaero/hud/minimap/radar/icon/definition/form/RadarIconBasicForms.java:10)

### `RadarIconForm`: it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface))

- [ ] `RadarIconForm.RadarIconForm(1)`: its class isn't translated (it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface))) (xaero/hud/minimap/radar/icon/definition/form/RadarIconForm.java:11)
- [ ] `RadarIconForm.getType(0)`: its class isn't translated (it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface))) (xaero/hud/minimap/radar/icon/definition/form/RadarIconForm.java:15)
- [ ] `RadarIconForm.getPrerenderer(0)`: it uses IRadarIconFormPrerenderer, which isn't translated (it's an interface) (xaero/hud/minimap/radar/icon/definition/form/RadarIconForm.java:19)
- [ ] `RadarIconForm.getFailureResult(0)`: its class isn't translated (it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface))) (xaero/hud/minimap/radar/icon/definition/form/RadarIconForm.java:24)

### `RadarIconSpriteForm`: it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)

- [ ] `RadarIconSpriteForm.RadarIconSpriteForm(2)`: its class isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)) (xaero/hud/minimap/radar/icon/definition/form/sprite/RadarIconSpriteForm.java:11)
- [ ] `RadarIconSpriteForm.getSpriteLocation(0)`: its class isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)) (xaero/hud/minimap/radar/icon/definition/form/sprite/RadarIconSpriteForm.java:16)
- [ ] `RadarIconSpriteForm.read(3)`: it uses RadarIconSpriteForm, which isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)) (xaero/hud/minimap/radar/icon/definition/form/sprite/RadarIconSpriteForm.java:20)

### `IRadarIconFormReader`: it's an interface

- [ ] `IRadarIconFormReader.read(3)`: it has no body (xaero/hud/minimap/radar/icon/definition/form/type/IRadarIconFormReader.java:7)

### `RadarIconFormType`: it uses IRadarIconFormReader, which isn't translated (it's an interface)

- [ ] `RadarIconFormType.RadarIconFormType(4)`: its class isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)) (xaero/hud/minimap/radar/icon/definition/form/type/RadarIconFormType.java:16)
- [ ] `RadarIconFormType.getId(0)`: its class isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)) (xaero/hud/minimap/radar/icon/definition/form/type/RadarIconFormType.java:23)
- [ ] `RadarIconFormType.readForm(2)`: its class isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)) (xaero/hud/minimap/radar/icon/definition/form/type/RadarIconFormType.java:27)
- [ ] `RadarIconFormType.addTo(1)`: its class isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)) (xaero/hud/minimap/radar/icon/definition/form/type/RadarIconFormType.java:31)
- [ ] `RadarIconFormType.getPrerenderer(0)`: its class isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)) (xaero/hud/minimap/radar/icon/definition/form/type/RadarIconFormType.java:36)
- [ ] `RadarIconFormType.getFailureResult(0)`: its class isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)) (xaero/hud/minimap/radar/icon/definition/form/type/RadarIconFormType.java:41)

### `RadarIconFormTypes`

- [ ] `RadarIconFormTypes.ALL`: it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)) (xaero/hud/minimap/radar/icon/definition/form/type/RadarIconFormTypes.java:15)
- [ ] `RadarIconFormTypes.MODEL`: it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)) (xaero/hud/minimap/radar/icon/definition/form/type/RadarIconFormTypes.java:16)
- [ ] `RadarIconFormTypes.NORMAL_SPRITE`: it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)) (xaero/hud/minimap/radar/icon/definition/form/type/RadarIconFormTypes.java:20)
- [ ] `RadarIconFormTypes.OUTLINED_SPRITE`: it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)) (xaero/hud/minimap/radar/icon/definition/form/type/RadarIconFormTypes.java:24)
- [ ] `RadarIconFormTypes.OLD_SPRITE`: it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)) (xaero/hud/minimap/radar/icon/definition/form/type/RadarIconFormTypes.java:28)
- [ ] `RadarIconFormTypes.ITEM`: it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)) (xaero/hud/minimap/radar/icon/definition/form/type/RadarIconFormTypes.java:32)
- [ ] `RadarIconFormTypes.DOT`: it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)) (xaero/hud/minimap/radar/icon/definition/form/type/RadarIconFormTypes.java:34)
- [ ] `RadarIconFormTypes.readType(1)`: it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)) (xaero/hud/minimap/radar/icon/definition/form/type/RadarIconFormTypes.java:37)

### `RadarIconDefinition`: it uses `Method` (java.lang.reflect)

- [ ] `RadarIconDefinition.getVariantForm(1)`: its class isn't translated (it uses `Method` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/definition/RadarIconDefinition.java:35)
- [ ] `RadarIconDefinition.construct(1)`: its class isn't translated (it uses `Method` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/definition/RadarIconDefinition.java:39)
- [ ] `RadarIconDefinition.constructForm(1)`: its class isn't translated (it uses `Method` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/definition/RadarIconDefinition.java:82)
- [ ] `RadarIconDefinition.getVariantMethodString(0)`: its class isn't translated (it uses `Method` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/definition/RadarIconDefinition.java:88)
- [ ] `RadarIconDefinition.getVariantMethod(0)`: its class isn't translated (it uses `Method` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/definition/RadarIconDefinition.java:92)
- [ ] `RadarIconDefinition.setVariantMethod(1)`: its class isn't translated (it uses `Method` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/definition/RadarIconDefinition.java:96)
- [ ] `RadarIconDefinition.getVariantIdBuilderMethodString(0)`: its class isn't translated (it uses `Method` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/definition/RadarIconDefinition.java:100)
- [ ] `RadarIconDefinition.getVariantIdBuilderMethod(0)`: its class isn't translated (it uses `Method` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/definition/RadarIconDefinition.java:104)
- [ ] `RadarIconDefinition.setVariantIdBuilderMethod(1)`: its class isn't translated (it uses `Method` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/definition/RadarIconDefinition.java:108)
- [ ] `RadarIconDefinition.convertStringToMethod(5)`: its class isn't translated (it uses `Method` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/definition/RadarIconDefinition.java:112)
- [ ] `RadarIconDefinition.getOldVariantIdMethodString(0)`: its class isn't translated (it uses `Method` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/definition/RadarIconDefinition.java:140)
- [ ] `RadarIconDefinition.getOldVariantIdMethod(0)`: its class isn't translated (it uses `Method` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/definition/RadarIconDefinition.java:144)
- [ ] `RadarIconDefinition.setOldVariantIdMethod(1)`: its class isn't translated (it uses `Method` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/definition/RadarIconDefinition.java:148)
- [ ] `RadarIconDefinition.getModelConfig(1)`: its class isn't translated (it uses `Method` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/definition/RadarIconDefinition.java:152)
- [ ] `RadarIconDefinition.getEntityId(0)`: its class isn't translated (it uses `Method` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/definition/RadarIconDefinition.java:160)

### `RadarIconDefinitionManager`: it uses RadarIconDefinition, which isn't translated (it uses `Method` (java.lang.reflect))

- [ ] `RadarIconDefinitionManager.get(1)`: its class isn't translated (it uses RadarIconDefinition, which isn't translated (it uses `Method` (java.lang.reflect))) (xaero/hud/minimap/radar/icon/definition/RadarIconDefinitionManager.java:11)
- [ ] `RadarIconDefinitionManager.reloadResources(0)`: its class isn't translated (it uses RadarIconDefinition, which isn't translated (it uses `Method` (java.lang.reflect))) (xaero/hud/minimap/radar/icon/definition/RadarIconDefinitionManager.java:15)

### `RadarIconDefinitionReloader`

- [ ] `RadarIconDefinitionReloader.reloadResources(1)`: it uses RadarIconDefinition, which isn't translated (it uses `Method` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/definition/RadarIconDefinitionReloader.java:22)
- [ ] `RadarIconDefinitionReloader.reloadResourcesAttempt(3)`: it uses RadarIconDefinition, which isn't translated (it uses `Method` (java.lang.reflect)) (xaero/hud/minimap/radar/icon/definition/RadarIconDefinitionReloader.java:41)

### `RadarIconManager`: it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))

- [ ] `RadarIconManager.FAILED`: it uses XaeroIcon, which isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)) (xaero/hud/minimap/radar/icon/RadarIconManager.java:35)
- [ ] `RadarIconManager.DOT`: it uses XaeroIcon, which isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)) (xaero/hud/minimap/radar/icon/RadarIconManager.java:36)
- [ ] `RadarIconManager.RadarIconManager(1)`: its class isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))) (xaero/hud/minimap/radar/icon/RadarIconManager.java:45)
- [ ] `RadarIconManager.get(6)`: it's generic (xaero/hud/minimap/radar/icon/RadarIconManager.java:55)
- [ ] `RadarIconManager.get(9)`: it's generic (xaero/hud/minimap/radar/icon/RadarIconManager.java:65)
- [ ] `RadarIconManager.reset(0)`: its class isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))) (xaero/hud/minimap/radar/icon/RadarIconManager.java:132)
- [ ] `RadarIconManager.resetResources(0)`: its class isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))) (xaero/hud/minimap/radar/icon/RadarIconManager.java:138)
- [ ] `RadarIconManager.allowPrerender(0)`: its class isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))) (xaero/hud/minimap/radar/icon/RadarIconManager.java:142)
- [ ] `RadarIconManager.onModelPrepareDetection(1)`: its class isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))) (xaero/hud/minimap/radar/icon/RadarIconManager.java:146)
- [ ] `RadarIconManager.onModelRenderTrace(3)`: its class isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))) (xaero/hud/minimap/radar/icon/RadarIconManager.java:150)
- [ ] `RadarIconManager.onModelPartRenderTrace(2)`: its class isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))) (xaero/hud/minimap/radar/icon/RadarIconManager.java:154)

### `RadarSession`: it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))

- [ ] `RadarSession.RadarSession(4)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/RadarSession.java:17)
- [ ] `RadarSession.update(3)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/RadarSession.java:24)
- [ ] `RadarSession.getCategoryManager(0)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/RadarSession.java:28)
- [ ] `RadarSession.getState(0)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/RadarSession.java:32)
- [ ] `RadarSession.getStateUpdater(0)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/RadarSession.java:36)
- [ ] `RadarSession.getColorHelper(0)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/RadarSession.java:40)

### `RadarSession.Builder`: it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))

- [ ] `RadarSession.Builder.Builder(0)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/RadarSession.java:47)
- [ ] `RadarSession.Builder.setDefault(0)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/RadarSession.java:50)
- [ ] `RadarSession.Builder.setCategoryManager(1)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/RadarSession.java:55)
- [ ] `RadarSession.Builder.build(0)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/RadarSession.java:60)
- [ ] `RadarSession.Builder.begin(0)`: it uses Builder, which isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/RadarSession.java:71)

### `RadarElementReader`: it extends `MinimapElementReader` (xaero.hud.minimap.element.render)

- [ ] `RadarElementReader.getRenderX(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:10)
- [ ] `RadarElementReader.getRenderY(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:14)
- [ ] `RadarElementReader.getRenderZ(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:18)
- [ ] `RadarElementReader.isHidden(2)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:22)
- [ ] `RadarElementReader.getInteractionBoxLeft(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:26)
- [ ] `RadarElementReader.getInteractionBoxRight(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:30)
- [ ] `RadarElementReader.getInteractionBoxTop(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:34)
- [ ] `RadarElementReader.getInteractionBoxBottom(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:38)
- [ ] `RadarElementReader.getRenderBoxLeft(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:42)
- [ ] `RadarElementReader.getRenderBoxRight(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:46)
- [ ] `RadarElementReader.getRenderBoxTop(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:50)
- [ ] `RadarElementReader.getRenderBoxBottom(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:54)
- [ ] `RadarElementReader.getLeftSideLength(2)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:58)
- [ ] `RadarElementReader.getMenuName(1)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:62)
- [ ] `RadarElementReader.getFilterName(1)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:66)
- [ ] `RadarElementReader.getMenuTextFillLeftPadding(1)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:70)
- [ ] `RadarElementReader.getRightClickTitleBackgroundColor(1)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:74)
- [ ] `RadarElementReader.getBoxScale(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:78)
- [ ] `RadarElementReader.isInteractable(2)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:82)
- [ ] `RadarElementReader.shouldScaleBoxWithOptionalScale(0)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarElementReader.java:86)

### `RadarRenderContext`: it uses RadarList, which isn't translated (it implements `Comparable`)


### `RadarRenderer`: it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)

- [ ] `RadarRenderer.RadarRenderer(5)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderer.java:79)
- [ ] `RadarRenderer.preRender(3)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderer.java:88)
- [ ] `RadarRenderer.renderElement(10)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderer.java:119)
- [ ] `RadarRenderer.postRender(3)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderer.java:258)
- [ ] `RadarRenderer.renderIcon(6)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderer.java:276)
- [ ] `RadarRenderer.renderDot(9)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderer.java:305)
- [ ] `RadarRenderer.renderLabel(6)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderer.java:384)
- [ ] `RadarRenderer.setupRenderForList(1)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderer.java:428)
- [ ] `RadarRenderer.setupRenderForEntity(1)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderer.java:445)
- [ ] `RadarRenderer.confirmTrackedPlayerRadarRender(1)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderer.java:458)
- [ ] `RadarRenderer.renderSingleEntity(10)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderer.java:468)
- [ ] `RadarRenderer.shouldRender(1)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderer.java:501)

### `RadarRenderer.Builder`: it uses RadarIconManager, which isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)))

- [ ] `RadarRenderer.Builder.Builder(0)`: its class isn't translated (it uses RadarIconManager, which isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)))) (xaero/hud/minimap/radar/render/element/RadarRenderer.java:523)
- [ ] `RadarRenderer.Builder.setDefault(0)`: its class isn't translated (it uses RadarIconManager, which isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)))) (xaero/hud/minimap/radar/render/element/RadarRenderer.java:526)
- [ ] `RadarRenderer.Builder.setRadarIconManager(1)`: its class isn't translated (it uses RadarIconManager, which isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)))) (xaero/hud/minimap/radar/render/element/RadarRenderer.java:531)
- [ ] `RadarRenderer.Builder.setMinimap(1)`: its class isn't translated (it uses RadarIconManager, which isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)))) (xaero/hud/minimap/radar/render/element/RadarRenderer.java:536)
- [ ] `RadarRenderer.Builder.build(0)`: its class isn't translated (it uses RadarIconManager, which isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)))) (xaero/hud/minimap/radar/render/element/RadarRenderer.java:541)
- [ ] `RadarRenderer.Builder.begin(0)`: it uses Builder, which isn't translated (it uses RadarIconManager, which isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)))) (xaero/hud/minimap/radar/render/element/RadarRenderer.java:552)

### `RadarRenderProvider`: it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render)

- [ ] `RadarRenderProvider.SCREEN_FIELD`: it uses `Field` (java.lang.reflect) (xaero/hud/minimap/radar/render/element/RadarRenderProvider.java:21)
- [ ] `RadarRenderProvider.begin(2)`: its class isn't translated (it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderProvider.java:29)
- [ ] `RadarRenderProvider.ensureList(2)`: its body couldn't be read: a labelled statement (xaero/hud/minimap/radar/render/element/RadarRenderProvider.java:44)
- [ ] `RadarRenderProvider.shouldRenderOverMinimap(1)`: its class isn't translated (it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderProvider.java:62)
- [ ] `RadarRenderProvider.hasNext(2)`: its class isn't translated (it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderProvider.java:67)
- [ ] `RadarRenderProvider.setupContextAndGetNext(2)`: its class isn't translated (it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderProvider.java:74)
- [ ] `RadarRenderProvider.getNext(2)`: its class isn't translated (it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderProvider.java:91)
- [ ] `RadarRenderProvider.end(2)`: its class isn't translated (it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderProvider.java:97)
- [ ] `RadarRenderProvider.setupContextForList(2)`: its class isn't translated (it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderProvider.java:103)
- [ ] `RadarRenderProvider.setupContextForEntity(2)`: its class isn't translated (it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderProvider.java:113)
- [ ] `RadarRenderProvider.isUsed(0)`: its class isn't translated (it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/radar/render/element/RadarRenderProvider.java:117)

### `RadarList`: it implements `Comparable`

- [ ] `RadarList.RadarList(1)`: its class isn't translated (it implements `Comparable`) (xaero/hud/minimap/radar/state/RadarList.java:15)
- [ ] `RadarList.getClientCategory(0)`: its class isn't translated (it implements `Comparable`) (xaero/hud/minimap/radar/state/RadarList.java:19)
- [ ] `RadarList.getSyncedCategory(0)`: its class isn't translated (it implements `Comparable`) (xaero/hud/minimap/radar/state/RadarList.java:23)
- [ ] `RadarList.setClientCategory(1)`: its class isn't translated (it implements `Comparable`) (xaero/hud/minimap/radar/state/RadarList.java:27)
- [ ] `RadarList.setSyncedCategory(1)`: its class isn't translated (it implements `Comparable`) (xaero/hud/minimap/radar/state/RadarList.java:32)
- [ ] `RadarList.getEffective(1)`: it's generic (xaero/hud/minimap/radar/state/RadarList.java:37)
- [ ] `RadarList.clearEntities(0)`: its class isn't translated (it implements `Comparable`) (xaero/hud/minimap/radar/state/RadarList.java:42)
- [ ] `RadarList.add(1)`: its class isn't translated (it implements `Comparable`) (xaero/hud/minimap/radar/state/RadarList.java:46)
- [ ] `RadarList.get(1)`: its class isn't translated (it implements `Comparable`) (xaero/hud/minimap/radar/state/RadarList.java:50)
- [ ] `RadarList.size(0)`: its class isn't translated (it implements `Comparable`) (xaero/hud/minimap/radar/state/RadarList.java:54)
- [ ] `RadarList.getEntities(0)`: its class isn't translated (it implements `Comparable`) (xaero/hud/minimap/radar/state/RadarList.java:58)
- [ ] `RadarList.compareTo(1)`: its class isn't translated (it implements `Comparable`) (xaero/hud/minimap/radar/state/RadarList.java:62)

### `RadarList.Builder`

- [x] `RadarList.Builder.Builder(0)` (xaero/hud/minimap/radar/state/RadarList.java:69)
- [ ] `RadarList.Builder.setDefault(0)`: `this` used as a value (xaero/hud/minimap/radar/state/RadarList.java:72)
- [ ] `RadarList.Builder.build(0)`: it uses RadarList, which isn't translated (it implements `Comparable`) (xaero/hud/minimap/radar/state/RadarList.java:76)
- [ ] `RadarList.Builder.getDefault(0)`: it calls Builder.setDefault, which isn't translated (xaero/hud/minimap/radar/state/RadarList.java:80)

### `RadarState`: it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category))

- [ ] `RadarState.getListsReversedOrder(0)`: its class isn't translated (it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category))) (xaero/hud/minimap/radar/state/RadarState.java:13)
- [ ] `RadarState.getRadarLists(0)`: its class isn't translated (it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category))) (xaero/hud/minimap/radar/state/RadarState.java:17)
- [ ] `RadarState.getListsGeneratedForConfig(0)`: its class isn't translated (it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category))) (xaero/hud/minimap/radar/state/RadarState.java:21)
- [ ] `RadarState.getListsGeneratedForSyncedConfig(0)`: its class isn't translated (it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category))) (xaero/hud/minimap/radar/state/RadarState.java:25)
- [ ] `RadarState.getUpdatableLists(0)`: its class isn't translated (it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category))) (xaero/hud/minimap/radar/state/RadarState.java:29)
- [ ] `RadarState.setListsReversedOrder(1)`: its class isn't translated (it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category))) (xaero/hud/minimap/radar/state/RadarState.java:33)
- [ ] `RadarState.setListsGeneratedForConfig(1)`: its class isn't translated (it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category))) (xaero/hud/minimap/radar/state/RadarState.java:37)
- [ ] `RadarState.setListsGeneratedForSyncedConfig(1)`: its class isn't translated (it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category))) (xaero/hud/minimap/radar/state/RadarState.java:41)

### `RadarStateUpdater`: it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))

- [ ] `RadarStateUpdater.RadarStateUpdater(2)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/state/RadarStateUpdater.java:29)
- [ ] `RadarStateUpdater.update(3)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/state/RadarStateUpdater.java:35)
- [ ] `RadarStateUpdater.ensureCategories(4)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/state/RadarStateUpdater.java:100)
- [ ] `RadarStateUpdater.traceAddCategories(3)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/state/RadarStateUpdater.java:118)
- [ ] `RadarStateUpdater.traceAddSyncedCategories(3)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/state/RadarStateUpdater.java:129)
- [ ] `RadarStateUpdater.putOnUpdateMap(3)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/state/RadarStateUpdater.java:136)
- [ ] `RadarStateUpdater.isWorldMapRadarEnabled(0)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/state/RadarStateUpdater.java:145)
- [ ] `RadarStateUpdater.isInvisibleTo(2)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/state/RadarStateUpdater.java:149)
- [ ] `RadarStateUpdater.shouldHideForSneaking(2)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/state/RadarStateUpdater.java:153)
- [ ] `RadarStateUpdater.setLastRenderViewEntity(1)`: its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))) (xaero/hud/minimap/radar/state/RadarStateUpdater.java:162)

### `RadarUtils`

- [ ] `RadarUtils.FOX_TRUSTED_UUID_SECONDARY`: it uses Minecraft's `EntityDataAccessor` (xaero/hud/minimap/radar/util/RadarUtils.java:26)
- [ ] `RadarUtils.FOX_TRUSTED_UUID_MAIN`: it uses Minecraft's `EntityDataAccessor` (xaero/hud/minimap/radar/util/RadarUtils.java:27)
- [ ] `RadarUtils.getMaxDistance(2)`: it uses MinimapProcessor, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/hud/minimap/radar/util/RadarUtils.java:29)
- [ ] `RadarUtils.isHostileException(1)`: it uses Minecraft's `Entity` (xaero/hud/minimap/radar/util/RadarUtils.java:38)
- [ ] `RadarUtils.isTamed(2)`: it uses Minecraft's `Entity` (xaero/hud/minimap/radar/util/RadarUtils.java:42)
- [ ] `RadarUtils.isHostile(1)`: it uses Minecraft's `Entity` (xaero/hud/minimap/radar/util/RadarUtils.java:57)
- [ ] `RadarUtils.getCustomName(2)`: it uses Minecraft's `Entity` (xaero/hud/minimap/radar/util/RadarUtils.java:65)

### `MinimapPipRenderer`: it's generic

- [ ] `MinimapPipRenderer.MinimapPipRenderer(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/render/MinimapPipRenderer.java:18)
- [ ] `MinimapPipRenderer.getRenderStateClass(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/render/MinimapPipRenderer.java:22)
- [ ] `MinimapPipRenderer.prepare(4)`: its class isn't translated (it's generic) (xaero/hud/minimap/render/MinimapPipRenderer.java:26)
- [ ] `MinimapPipRenderer.renderToTexture(3)`: its class isn't translated (it's generic) (xaero/hud/minimap/render/MinimapPipRenderer.java:45)
- [ ] `MinimapPipRenderer.getTextureLabel(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/render/MinimapPipRenderer.java:68)

### `MinimapPipRenderState`: it implements Minecraft's `PictureInPictureRenderState`

- [x] `MinimapPipRenderState.HORIZONTAL_PADDING` (xaero/hud/minimap/render/MinimapPipRenderState.java:9)
- [x] `MinimapPipRenderState.VERTICAL_PADDING` (xaero/hud/minimap/render/MinimapPipRenderState.java:10)
- [ ] `MinimapPipRenderState.update(10)`: its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`) (xaero/hud/minimap/render/MinimapPipRenderState.java:25)
- [ ] `MinimapPipRenderState.getWidth(0)`: its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`) (xaero/hud/minimap/render/MinimapPipRenderState.java:43)
- [ ] `MinimapPipRenderState.getHeight(0)`: its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`) (xaero/hud/minimap/render/MinimapPipRenderState.java:47)
- [ ] `MinimapPipRenderState.getScale(0)`: its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`) (xaero/hud/minimap/render/MinimapPipRenderState.java:51)
- [ ] `MinimapPipRenderState.getMinimapScale(0)`: its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`) (xaero/hud/minimap/render/MinimapPipRenderState.java:55)
- [ ] `MinimapPipRenderState.getSize(0)`: its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`) (xaero/hud/minimap/render/MinimapPipRenderState.java:59)
- [ ] `MinimapPipRenderState.getBoxSize(0)`: its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`) (xaero/hud/minimap/render/MinimapPipRenderState.java:63)
- [ ] `MinimapPipRenderState.getPartial(0)`: its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`) (xaero/hud/minimap/render/MinimapPipRenderState.java:67)
- [ ] `MinimapPipRenderState.getCvc(0)`: its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`) (xaero/hud/minimap/render/MinimapPipRenderState.java:71)
- [ ] `MinimapPipRenderState.getScaledHorizontalPadding(0)`: its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`) (xaero/hud/minimap/render/MinimapPipRenderState.java:75)
- [ ] `MinimapPipRenderState.getScaledVerticalPadding(0)`: its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`) (xaero/hud/minimap/render/MinimapPipRenderState.java:79)
- [ ] `MinimapPipRenderState.x0(0)`: its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`) (xaero/hud/minimap/render/MinimapPipRenderState.java:83)
- [ ] `MinimapPipRenderState.y0(0)`: its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`) (xaero/hud/minimap/render/MinimapPipRenderState.java:87)
- [ ] `MinimapPipRenderState.x1(0)`: its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`) (xaero/hud/minimap/render/MinimapPipRenderState.java:91)
- [ ] `MinimapPipRenderState.y1(0)`: its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`) (xaero/hud/minimap/render/MinimapPipRenderState.java:95)
- [ ] `MinimapPipRenderState.scale(0)`: its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`) (xaero/hud/minimap/render/MinimapPipRenderState.java:99)
- [ ] `MinimapPipRenderState.scissorArea(0)`: its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`) (xaero/hud/minimap/render/MinimapPipRenderState.java:103)
- [ ] `MinimapPipRenderState.bounds(0)`: its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`) (xaero/hud/minimap/render/MinimapPipRenderState.java:108)

### `MinimapPipRenderState.Enlarged`: it extends `MinimapPipRenderState`


### `DeathpointHandler`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)

- [ ] `DeathpointHandler.DeathpointHandler(2)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/DeathpointHandler.java:22)
- [ ] `DeathpointHandler.createDeathpoint(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/DeathpointHandler.java:27)
- [ ] `DeathpointHandler.createDeathpoint(3)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/DeathpointHandler.java:66)

### `DestinationHandler`: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))

- [ ] `DestinationHandler.DestinationHandler(2)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/DestinationHandler.java:18)
- [ ] `DestinationHandler.begin(4)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/DestinationHandler.java:23)
- [ ] `DestinationHandler.handle(1)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/DestinationHandler.java:32)
- [ ] `DestinationHandler.end(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/DestinationHandler.java:52)

### `WaypointIO`

- [ ] `WaypointIO.WaypointIO(1)`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`) (xaero/hud/minimap/waypoint/io/WaypointIO.java:16)
- [x] `WaypointIO.getOldIO(0)` (xaero/hud/minimap/waypoint/io/WaypointIO.java:20)
- [ ] `WaypointIO.checkLine(2)`: `null` (Rust has no null: this needs an Option) (xaero/hud/minimap/waypoint/io/WaypointIO.java:24)
- [ ] `WaypointIO.saveWaypoints(2)`: it uses `OutputStreamWriter` (java.io) (xaero/hud/minimap/waypoint/io/WaypointIO.java:103)

### `WaypointOldIO`

- [x] `WaypointOldIO.WaypointOldIO(1)` (xaero/hud/minimap/waypoint/io/WaypointOldIO.java:24)
- [ ] `WaypointOldIO.load(1)`: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/waypoint/io/WaypointOldIO.java:28)
- [ ] `WaypointOldIO.loadOldWaypoints(1)`: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/waypoint/io/WaypointOldIO.java:33)
- [ ] `WaypointOldIO.checkLine(2)`: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/waypoint/io/WaypointOldIO.java:43)
- [ ] `WaypointOldIO.loadFromFile(2)`: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/waypoint/io/WaypointOldIO.java:110)
- [ ] `WaypointOldIO.convertToNewContainerID(2)`: it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/minimap/waypoint/io/WaypointOldIO.java:131)
- [x] `WaypointOldIO.fixOldDimensionName(1)` (xaero/hud/minimap/waypoint/io/WaypointOldIO.java:149)
- [ ] `WaypointOldIO.loadFromConfigLines(1)`: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)) (xaero/hud/minimap/waypoint/io/WaypointOldIO.java:159)

### `AbstractWaypointRenderContext`


### `AbstractWaypointRenderProvider`: it's generic

- [ ] `AbstractWaypointRenderProvider.begin(2)`: its class isn't translated (it's generic) (xaero/hud/minimap/waypoint/render/AbstractWaypointRenderProvider.java:39)
- [ ] `AbstractWaypointRenderProvider.hasNext(2)`: its class isn't translated (it's generic) (xaero/hud/minimap/waypoint/render/AbstractWaypointRenderProvider.java:60)
- [ ] `AbstractWaypointRenderProvider.getNext(2)`: its class isn't translated (it's generic) (xaero/hud/minimap/waypoint/render/AbstractWaypointRenderProvider.java:64)
- [ ] `AbstractWaypointRenderProvider.end(2)`: its class isn't translated (it's generic) (xaero/hud/minimap/waypoint/render/AbstractWaypointRenderProvider.java:70)

### `WaypointDeleter`: it uses IXaeroMinimap, which isn't translated (it's an interface)

- [ ] `WaypointDeleter.WaypointDeleter(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/hud/minimap/waypoint/render/WaypointDeleter.java:18)
- [ ] `WaypointDeleter.begin(0)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/hud/minimap/waypoint/render/WaypointDeleter.java:23)
- [ ] `WaypointDeleter.add(1)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/hud/minimap/waypoint/render/WaypointDeleter.java:27)
- [ ] `WaypointDeleter.deleteCollected(3)`: its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)) (xaero/hud/minimap/waypoint/render/WaypointDeleter.java:35)

### `WaypointMapRenderContext`: it extends `AbstractWaypointRenderContext`


### `WaypointMapRenderer`: it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)

- [ ] `WaypointMapRenderer.WaypointMapRenderer(3)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderer.java:48)
- [ ] `WaypointMapRenderer.renderElement(10)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderer.java:52)
- [ ] `WaypointMapRenderer.preRender(3)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderer.java:94)
- [ ] `WaypointMapRenderer.postRender(3)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderer.java:118)
- [ ] `WaypointMapRenderer.drawIcon(9)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderer.java:126)
- [ ] `WaypointMapRenderer.drawIconGUI(5)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderer.java:168)
- [ ] `WaypointMapRenderer.drawIcon(16)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderer.java:183)
- [ ] `WaypointMapRenderer.drawIconGUI(13)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderer.java:217)
- [ ] `WaypointMapRenderer.drawSetChange(3)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderer.java:245)
- [ ] `WaypointMapRenderer.shouldRender(1)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderer.java:271)
- [ ] `WaypointMapRenderer.getOrder(0)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderer.java:280)

### `WaypointMapRenderer.Builder`: it uses WaypointDeleter, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))

- [ ] `WaypointMapRenderer.Builder.Builder(1)`: its class isn't translated (it uses WaypointDeleter, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))) (xaero/hud/minimap/waypoint/render/WaypointMapRenderer.java:289)
- [ ] `WaypointMapRenderer.Builder.setDefault(0)`: its class isn't translated (it uses WaypointDeleter, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))) (xaero/hud/minimap/waypoint/render/WaypointMapRenderer.java:293)
- [ ] `WaypointMapRenderer.Builder.setWaypointDeleter(1)`: its class isn't translated (it uses WaypointDeleter, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))) (xaero/hud/minimap/waypoint/render/WaypointMapRenderer.java:298)
- [ ] `WaypointMapRenderer.Builder.build(0)`: its class isn't translated (it uses WaypointDeleter, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))) (xaero/hud/minimap/waypoint/render/WaypointMapRenderer.java:303)
- [ ] `WaypointMapRenderer.Builder.begin(1)`: it uses Builder, which isn't translated (it uses WaypointDeleter, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))) (xaero/hud/minimap/waypoint/render/WaypointMapRenderer.java:312)

### `WaypointMapRenderProvider`: it extends `AbstractWaypointRenderProvider`


### `WaypointMapRenderReader`: it extends `MinimapElementReader` (xaero.hud.minimap.element.render)

- [ ] `WaypointMapRenderReader.getRenderX(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:10)
- [ ] `WaypointMapRenderReader.getRenderY(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:14)
- [ ] `WaypointMapRenderReader.getRenderZ(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:18)
- [ ] `WaypointMapRenderReader.getCoordinateScale(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:22)
- [ ] `WaypointMapRenderReader.shouldScalePartialCoordinates(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:26)
- [ ] `WaypointMapRenderReader.isHidden(2)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:30)
- [ ] `WaypointMapRenderReader.getInteractionBoxLeft(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:34)
- [ ] `WaypointMapRenderReader.getInteractionBoxRight(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:38)
- [ ] `WaypointMapRenderReader.getInteractionBoxTop(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:42)
- [ ] `WaypointMapRenderReader.getInteractionBoxBottom(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:46)
- [ ] `WaypointMapRenderReader.getRenderBoxLeft(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:50)
- [ ] `WaypointMapRenderReader.getRenderBoxRight(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:54)
- [ ] `WaypointMapRenderReader.getRenderBoxTop(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:60)
- [ ] `WaypointMapRenderReader.getRenderBoxBottom(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:64)
- [ ] `WaypointMapRenderReader.getLeftSideLength(2)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:68)
- [ ] `WaypointMapRenderReader.getMenuName(1)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:72)
- [ ] `WaypointMapRenderReader.getFilterName(1)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:76)
- [ ] `WaypointMapRenderReader.getMenuTextFillLeftPadding(1)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:80)
- [ ] `WaypointMapRenderReader.getRightClickTitleBackgroundColor(1)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:84)
- [ ] `WaypointMapRenderReader.shouldScaleBoxWithOptionalScale(0)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java:88)

### `WaypointWorldRenderContext`: it extends `AbstractWaypointRenderContext` (xaero.hud.minimap.waypoint.render)


### `WaypointWorldRenderer`: it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)

- [ ] `WaypointWorldRenderer.WaypointWorldRenderer(3)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderer.java:66)
- [ ] `WaypointWorldRenderer.renderElement(10)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderer.java:72)
- [ ] `WaypointWorldRenderer.preRender(3)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderer.java:157)
- [ ] `WaypointWorldRenderer.postRender(3)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderer.java:229)
- [ ] `WaypointWorldRenderer.renderIconWithLabels(12)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderer.java:242)
- [ ] `WaypointWorldRenderer.renderIcon(5)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderer.java:277)
- [ ] `WaypointWorldRenderer.renderColorBackground(7)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderer.java:297)
- [ ] `WaypointWorldRenderer.renderTexturedIcon(9)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderer.java:305)
- [ ] `WaypointWorldRenderer.renderWaypointLabel(6)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderer.java:319)
- [ ] `WaypointWorldRenderer.getDistanceText(1)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderer.java:341)
- [ ] `WaypointWorldRenderer.shouldRender(1)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderer.java:345)
- [ ] `WaypointWorldRenderer.getOrder(0)`: its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderer.java:356)

### `WaypointWorldRenderer.Builder`

- [x] `WaypointWorldRenderer.Builder.Builder(0)` (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderer.java:362)
- [ ] `WaypointWorldRenderer.Builder.setDefault(0)`: `this` used as a value (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderer.java:365)
- [ ] `WaypointWorldRenderer.Builder.build(0)`: it uses WaypointWorldRenderer, which isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderer.java:369)
- [ ] `WaypointWorldRenderer.Builder.begin(0)`: it calls Builder.setDefault, which isn't translated (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderer.java:374)

### `WaypointWorldRenderProvider`: it extends `AbstractWaypointRenderProvider` (xaero.hud.minimap.waypoint.render)


### `WaypointWorldRenderReader`: it extends `MinimapElementReader` (xaero.hud.minimap.element.render)

- [ ] `WaypointWorldRenderReader.WaypointWorldRenderReader(1)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:13)
- [ ] `WaypointWorldRenderReader.isHidden(2)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:17)
- [ ] `WaypointWorldRenderReader.getRenderX(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:21)
- [ ] `WaypointWorldRenderReader.getRenderY(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:25)
- [ ] `WaypointWorldRenderReader.getRenderZ(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:35)
- [ ] `WaypointWorldRenderReader.getCoordinateScale(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:39)
- [ ] `WaypointWorldRenderReader.shouldScalePartialCoordinates(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:43)
- [ ] `WaypointWorldRenderReader.getInteractionBoxLeft(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:47)
- [ ] `WaypointWorldRenderReader.getInteractionBoxRight(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:51)
- [ ] `WaypointWorldRenderReader.getInteractionBoxTop(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:55)
- [ ] `WaypointWorldRenderReader.getInteractionBoxBottom(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:59)
- [ ] `WaypointWorldRenderReader.getRenderBoxLeft(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:63)
- [ ] `WaypointWorldRenderReader.getRenderBoxRight(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:67)
- [ ] `WaypointWorldRenderReader.getRenderBoxTop(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:71)
- [ ] `WaypointWorldRenderReader.getRenderBoxBottom(3)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:75)
- [ ] `WaypointWorldRenderReader.getLeftSideLength(2)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:79)
- [ ] `WaypointWorldRenderReader.getMenuName(1)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:83)
- [ ] `WaypointWorldRenderReader.getFilterName(1)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:87)
- [ ] `WaypointWorldRenderReader.getMenuTextFillLeftPadding(1)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:91)
- [ ] `WaypointWorldRenderReader.getRightClickTitleBackgroundColor(1)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:95)
- [ ] `WaypointWorldRenderReader.shouldScaleBoxWithOptionalScale(0)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:99)
- [ ] `WaypointWorldRenderReader.isInteractable(2)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:104)
- [ ] `WaypointWorldRenderReader.isAlwaysHighlightedWhenHovered(2)`: its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)) (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java:108)

### `ServerWaypointManager`: it uses ThirdPartyWaypointManager, which isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)))

- [ ] `ServerWaypointManager.ORIGIN_ID`: it uses HudMod.INSTANCE, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/server/ServerWaypointManager.java:15)
- [ ] `ServerWaypointManager.ServerWaypointManager(1)`: its class isn't translated (it uses ThirdPartyWaypointManager, which isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)))) (xaero/hud/minimap/waypoint/server/ServerWaypointManager.java:18)
- [ ] `ServerWaypointManager.clear(0)`: it uses ServerWaypointManager.ORIGIN_ID, which isn't translated (it uses HudMod.INSTANCE, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))) (xaero/hud/minimap/waypoint/server/ServerWaypointManager.java:22)
- [ ] `ServerWaypointManager.remove(1)`: it uses ServerWaypointManager.ORIGIN_ID, which isn't translated (it uses HudMod.INSTANCE, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))) (xaero/hud/minimap/waypoint/server/ServerWaypointManager.java:26)
- [ ] `ServerWaypointManager.add(2)`: it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/hud/minimap/waypoint/server/ServerWaypointManager.java:30)
- [ ] `ServerWaypointManager.getById(1)`: it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/hud/minimap/waypoint/server/ServerWaypointManager.java:34)
- [ ] `ServerWaypointManager.getBySlot(1)`: it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/hud/minimap/waypoint/server/ServerWaypointManager.java:38)
- [ ] `ServerWaypointManager.getIds(0)`: it uses `IntIterable` (it.unimi.dsi.fastutil.ints) (xaero/hud/minimap/waypoint/server/ServerWaypointManager.java:42)
- [ ] `ServerWaypointManager.addDisabled(1)`: it uses ServerWaypointManager.ORIGIN_ID, which isn't translated (it uses HudMod.INSTANCE, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))) (xaero/hud/minimap/waypoint/server/ServerWaypointManager.java:47)
- [ ] `ServerWaypointManager.isEmpty(0)`: it uses ServerWaypointManager.ORIGIN_ID, which isn't translated (it uses HudMod.INSTANCE, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))) (xaero/hud/minimap/waypoint/server/ServerWaypointManager.java:53)
- [ ] `ServerWaypointManager.getWaypoints(0)`: it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/hud/minimap/waypoint/server/ServerWaypointManager.java:57)
- [ ] `ServerWaypointManager.size(0)`: it uses ServerWaypointManager.ORIGIN_ID, which isn't translated (it uses HudMod.INSTANCE, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))) (xaero/hud/minimap/waypoint/server/ServerWaypointManager.java:61)

### `WaypointSet`: it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))

- [ ] `WaypointSet.WaypointSet(1)`: its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))) (xaero/hud/minimap/waypoint/set/WaypointSet.java:12)
- [ ] `WaypointSet.getName(0)`: its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))) (xaero/hud/minimap/waypoint/set/WaypointSet.java:17)
- [ ] `WaypointSet.getWaypoints(0)`: its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))) (xaero/hud/minimap/waypoint/set/WaypointSet.java:21)
- [ ] `WaypointSet.addTo(1)`: its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))) (xaero/hud/minimap/waypoint/set/WaypointSet.java:25)
- [ ] `WaypointSet.add(2)`: its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))) (xaero/hud/minimap/waypoint/set/WaypointSet.java:29)
- [ ] `WaypointSet.add(1)`: its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))) (xaero/hud/minimap/waypoint/set/WaypointSet.java:37)
- [ ] `WaypointSet.addAll(2)`: its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))) (xaero/hud/minimap/waypoint/set/WaypointSet.java:41)
- [ ] `WaypointSet.addAll(1)`: its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))) (xaero/hud/minimap/waypoint/set/WaypointSet.java:49)
- [ ] `WaypointSet.remove(1)`: its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))) (xaero/hud/minimap/waypoint/set/WaypointSet.java:53)
- [ ] `WaypointSet.remove(1)`: its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))) (xaero/hud/minimap/waypoint/set/WaypointSet.java:57)
- [ ] `WaypointSet.removeAll(1)`: its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))) (xaero/hud/minimap/waypoint/set/WaypointSet.java:61)
- [ ] `WaypointSet.clear(0)`: its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))) (xaero/hud/minimap/waypoint/set/WaypointSet.java:65)
- [ ] `WaypointSet.isEmpty(0)`: its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))) (xaero/hud/minimap/waypoint/set/WaypointSet.java:69)
- [ ] `WaypointSet.size(0)`: its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))) (xaero/hud/minimap/waypoint/set/WaypointSet.java:73)
- [ ] `WaypointSet.get(1)`: its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))) (xaero/hud/minimap/waypoint/set/WaypointSet.java:77)
- [ ] `WaypointSet.set(2)`: its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))) (xaero/hud/minimap/waypoint/set/WaypointSet.java:81)

### `WaypointSet.Builder`

- [x] `WaypointSet.Builder.Builder(0)` (xaero/hud/minimap/waypoint/set/WaypointSet.java:88)
- [ ] `WaypointSet.Builder.setDefault(0)`: `this` used as a value (xaero/hud/minimap/waypoint/set/WaypointSet.java:91)
- [ ] `WaypointSet.Builder.setName(1)`: `this` used as a value (xaero/hud/minimap/waypoint/set/WaypointSet.java:96)
- [ ] `WaypointSet.Builder.build(0)`: it uses WaypointSet, which isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))) (xaero/hud/minimap/waypoint/set/WaypointSet.java:101)
- [ ] `WaypointSet.Builder.begin(0)`: it calls Builder.setDefault, which isn't translated (xaero/hud/minimap/waypoint/set/WaypointSet.java:109)

### `TemporaryWaypointHandler`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)

- [ ] `TemporaryWaypointHandler.TemporaryWaypointHandler(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/TemporaryWaypointHandler.java:15)
- [ ] `TemporaryWaypointHandler.createTemporaryWaypoint(4)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/TemporaryWaypointHandler.java:19)
- [ ] `TemporaryWaypointHandler.createTemporaryWaypoint(5)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/TemporaryWaypointHandler.java:23)
- [ ] `TemporaryWaypointHandler.createTemporaryWaypoint(6)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/TemporaryWaypointHandler.java:27)

### `ThirdPartyWaypointManager`: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))

- [ ] `ThirdPartyWaypointManager.ThirdPartyWaypointManager(1)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypointManager.java:17)
- [ ] `ThirdPartyWaypointManager.clear(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypointManager.java:24)
- [ ] `ThirdPartyWaypointManager.clearOrigin(1)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypointManager.java:28)
- [ ] `ThirdPartyWaypointManager.getOriginCount(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypointManager.java:35)
- [ ] `ThirdPartyWaypointManager.get(1)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypointManager.java:39)
- [ ] `ThirdPartyWaypointManager.getByIndex(1)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypointManager.java:47)
- [ ] `ThirdPartyWaypointManager.getCount(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypointManager.java:51)
- [ ] `ThirdPartyWaypointManager.getAll(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypointManager.java:55)

### `ThirdPartyWaypoints`: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))

- [ ] `ThirdPartyWaypoints.ThirdPartyWaypoints(2)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypoints.java:28)
- [ ] `ThirdPartyWaypoints.clear(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypoints.java:41)
- [ ] `ThirdPartyWaypoints.remove(1)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypoints.java:47)
- [ ] `ThirdPartyWaypoints.add(2)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypoints.java:58)
- [ ] `ThirdPartyWaypoints.get(1)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypoints.java:82)
- [ ] `ThirdPartyWaypoints.getBySlot(1)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypoints.java:86)
- [ ] `ThirdPartyWaypoints.getIds(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypoints.java:90)
- [ ] `ThirdPartyWaypoints.addRenderInfoOverride(2)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypoints.java:94)
- [ ] `ThirdPartyWaypoints.isEmpty(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypoints.java:103)
- [ ] `ThirdPartyWaypoints.getWaypoints(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypoints.java:107)
- [ ] `ThirdPartyWaypoints.getCount(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypoints.java:111)
- [ ] `ThirdPartyWaypoints.getOriginId(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypoints.java:115)
- [ ] `ThirdPartyWaypoints.getRenderInfoOverrides(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypoints.java:119)
- [ ] `ThirdPartyWaypoints.setEnabledStateGetter(1)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypoints.java:123)
- [ ] `ThirdPartyWaypoints.isEnabled(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypoints.java:127)
- [ ] `ThirdPartyWaypoints.hasEnabledStateGetter(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypoints.java:131)
- [ ] `ThirdPartyWaypoints.getEnabledStateGetter(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypoints.java:135)

### `WaypointUtils`

- [ ] `WaypointUtils.getDistanceText(3)`: it calls GuiMisc.getFormat, which isn't translated (xaero/hud/minimap/waypoint/util/WaypointUtils.java:11)
- [ ] `WaypointUtils.getDistanceText(1)`: it uses `ClientConfigManager` (xaero.lib.client.config) (xaero/hud/minimap/waypoint/util/WaypointUtils.java:17)
- [ ] `WaypointUtils.getDistanceText(6)`: it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/hud/minimap/waypoint/util/WaypointUtils.java:24)
- [ ] `WaypointUtils.getDistanceTextForCurrentWorld(5)`: it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)) (xaero/hud/minimap/waypoint/util/WaypointUtils.java:42)

### `WaypointCollector`: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))

- [ ] `WaypointCollector.WaypointCollector(1)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/WaypointCollector.java:17)
- [ ] `WaypointCollector.collect(1)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/WaypointCollector.java:21)

### `WaypointColor`: its constants carry values

- [ ] `WaypointColor.WaypointColor(3)`: its class isn't translated (its constants carry values) (xaero/hud/minimap/waypoint/WaypointColor.java:34)
- [ ] `WaypointColor.getName(0)`: its class isn't translated (its constants carry values) (xaero/hud/minimap/waypoint/WaypointColor.java:40)
- [ ] `WaypointColor.getFormat(0)`: its class isn't translated (its constants carry values) (xaero/hud/minimap/waypoint/WaypointColor.java:44)
- [ ] `WaypointColor.getHex(0)`: its class isn't translated (its constants carry values) (xaero/hud/minimap/waypoint/WaypointColor.java:48)
- [ ] `WaypointColor.fromIndex(1)`: it uses WaypointColor, which isn't translated (its constants carry values) (xaero/hud/minimap/waypoint/WaypointColor.java:52)
- [ ] `WaypointColor.getRandom(0)`: it uses WaypointColor, which isn't translated (its constants carry values) (xaero/hud/minimap/waypoint/WaypointColor.java:56)
- [ ] `WaypointColor.fromDye(1)`: it uses WaypointColor, which isn't translated (its constants carry values) (xaero/hud/minimap/waypoint/WaypointColor.java:60)

### `WaypointPurpose`: its constants carry values

- [ ] `WaypointPurpose.WaypointPurpose(2)`: its class isn't translated (its constants carry values) (xaero/hud/minimap/waypoint/WaypointPurpose.java:12)
- [ ] `WaypointPurpose.isDeath(0)`: its class isn't translated (its constants carry values) (xaero/hud/minimap/waypoint/WaypointPurpose.java:17)
- [ ] `WaypointPurpose.isDestination(0)`: its class isn't translated (its constants carry values) (xaero/hud/minimap/waypoint/WaypointPurpose.java:21)

### `WaypointRenderInfo`: it uses WaypointColor, which isn't translated (its constants carry values)

- [ ] `WaypointRenderInfo.getInitials(0)`: its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)) (xaero/hud/minimap/waypoint/WaypointRenderInfo.java:15)
- [ ] `WaypointRenderInfo.getInitialsSafe(1)`: its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)) (xaero/hud/minimap/waypoint/WaypointRenderInfo.java:19)
- [ ] `WaypointRenderInfo.setInitials(1)`: its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)) (xaero/hud/minimap/waypoint/WaypointRenderInfo.java:23)
- [ ] `WaypointRenderInfo.getWaypointColor(0)`: its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)) (xaero/hud/minimap/waypoint/WaypointRenderInfo.java:27)
- [ ] `WaypointRenderInfo.setWaypointColor(1)`: its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)) (xaero/hud/minimap/waypoint/WaypointRenderInfo.java:31)
- [ ] `WaypointRenderInfo.getVisibility(0)`: its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)) (xaero/hud/minimap/waypoint/WaypointRenderInfo.java:36)
- [ ] `WaypointRenderInfo.setVisibility(1)`: its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)) (xaero/hud/minimap/waypoint/WaypointRenderInfo.java:40)
- [ ] `WaypointRenderInfo.getDisabled(0)`: its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)) (xaero/hud/minimap/waypoint/WaypointRenderInfo.java:44)
- [ ] `WaypointRenderInfo.setDisabled(1)`: its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)) (xaero/hud/minimap/waypoint/WaypointRenderInfo.java:48)
- [ ] `WaypointRenderInfo.isThirdPartyDeleted(0)`: its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)) (xaero/hud/minimap/waypoint/WaypointRenderInfo.java:52)
- [ ] `WaypointRenderInfo.setThirdPartyDeleted(1)`: its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)) (xaero/hud/minimap/waypoint/WaypointRenderInfo.java:56)
- [ ] `WaypointRenderInfo.nullEverythingMatching(1)`: its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)) (xaero/hud/minimap/waypoint/WaypointRenderInfo.java:60)
- [ ] `WaypointRenderInfo.clear(0)`: its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)) (xaero/hud/minimap/waypoint/WaypointRenderInfo.java:78)

### `WaypointSession`: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))

- [ ] `WaypointSession.WaypointSession(2)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/WaypointSession.java:20)
- [ ] `WaypointSession.getSharing(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/WaypointSession.java:32)
- [ ] `WaypointSession.getDeathpointHandler(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/WaypointSession.java:36)
- [ ] `WaypointSession.getTemporaryHandler(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/WaypointSession.java:40)
- [ ] `WaypointSession.getTeleport(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/WaypointSession.java:44)
- [ ] `WaypointSession.getCollector(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/WaypointSession.java:48)
- [ ] `WaypointSession.getDeleter(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/WaypointSession.java:52)
- [ ] `WaypointSession.getDestinationHandler(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/WaypointSession.java:56)
- [ ] `WaypointSession.getMc(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/WaypointSession.java:60)
- [ ] `WaypointSession.getSetChangedTime(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/WaypointSession.java:64)
- [ ] `WaypointSession.setSetChangedTime(1)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/WaypointSession.java:68)
- [ ] `WaypointSession.getSession(0)`: its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))) (xaero/hud/minimap/waypoint/WaypointSession.java:72)

### `WaypointSharingHandler`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)

- [x] `WaypointSharingHandler.WAYPOINT_OLD_SHARE_PREFIX` (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:32)
- [x] `WaypointSharingHandler.WAYPOINT_ADD_PREFIX` (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:33)
- [x] `WaypointSharingHandler.WAYPOINT_SHARE_PREFIX` (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:34)
- [x] `WaypointSharingHandler.DESTINATION_PREFIX_INTERNAL` (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:35)
- [x] `WaypointSharingHandler.DESTINATION_PREFIX_INTERNAL_HYPHEN` (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:36)
- [x] `WaypointSharingHandler.DESTINATION_PREFIX_EXTERNAL` (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:37)
- [ ] `WaypointSharingHandler.WaypointSharingHandler(2)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:44)
- [ ] `WaypointSharingHandler.shareWaypoint(3)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:49)
- [ ] `WaypointSharingHandler.onShareConfirmationResult(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:60)
- [ ] `WaypointSharingHandler.getSharedDestinationDetails(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:89)
- [ ] `WaypointSharingHandler.onWaypointReceived(2)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:116)
- [ ] `WaypointSharingHandler.getReceivedDimensionName(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:167)
- [ ] `WaypointSharingHandler.onWaypointAdd(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:193)
- [ ] `WaypointSharingHandler.getReceivedDestinationWorld(2)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:266)
- [ ] `WaypointSharingHandler.getReceivedDimId(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:369)
- [ ] `WaypointSharingHandler.removeFormatting(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:391)
- [ ] `WaypointSharingHandler.restoreFormatting(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:395)

### `WaypointSharingHandler.Builder`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)

- [ ] `WaypointSharingHandler.Builder.Builder(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:403)
- [ ] `WaypointSharingHandler.Builder.setDefault(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:406)
- [ ] `WaypointSharingHandler.Builder.setModMain(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:412)
- [ ] `WaypointSharingHandler.Builder.setSession(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:417)
- [ ] `WaypointSharingHandler.Builder.build(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:422)
- [ ] `WaypointSharingHandler.Builder.begin(0)`: it uses Builder, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointSharingHandler.java:430)

### `WaypointsSort`: its constants carry values

- [ ] `WaypointsSort.WaypointsSort(1)`: its class isn't translated (its constants carry values) (xaero/hud/minimap/waypoint/WaypointsSort.java:13)

### `WaypointTeleport`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)

- [x] `WaypointTeleport.TELEPORT_ANYWAY_COMMAND` (xaero/hud/minimap/waypoint/WaypointTeleport.java:28)
- [x] `WaypointTeleport.SLASH_TELEPORT_ANYWAY_COMMAND` (xaero/hud/minimap/waypoint/WaypointTeleport.java:29)
- [ ] `WaypointTeleport.WaypointTeleport(3)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointTeleport.java:37)
- [ ] `WaypointTeleport.canTeleport(2)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointTeleport.java:44)
- [ ] `WaypointTeleport.teleportAnyway(0)`: its body couldn't be read: an anonymous class (xaero/hud/minimap/waypoint/WaypointTeleport.java:51)
- [ ] `WaypointTeleport.teleportToWaypoint(3)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointTeleport.java:60)
- [ ] `WaypointTeleport.teleportToWaypoint(4)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointTeleport.java:64)
- [ ] `WaypointTeleport.isWorldTeleportable(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointTeleport.java:182)
- [ ] `WaypointTeleport.isTeleportationSafe(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/waypoint/WaypointTeleport.java:198)

### `WaypointVisibilityType`: its constants carry values

- [ ] `WaypointVisibilityType.WaypointVisibilityType(2)`: its class isn't translated (its constants carry values) (xaero/hud/minimap/waypoint/WaypointVisibilityType.java:14)
- [ ] `WaypointVisibilityType.getTranslation(0)`: its class isn't translated (its constants carry values) (xaero/hud/minimap/waypoint/WaypointVisibilityType.java:19)
- [ ] `WaypointVisibilityType.isGlobal(0)`: its class isn't translated (its constants carry values) (xaero/hud/minimap/waypoint/WaypointVisibilityType.java:23)

### `MinimapWorldConnectionManager`: it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))

- [ ] `MinimapWorldConnectionManager.MinimapWorldConnectionManager(1)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java:16)
- [ ] `MinimapWorldConnectionManager.addConnection(2)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java:20)
- [ ] `MinimapWorldConnectionManager.addConnection(2)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java:24)
- [ ] `MinimapWorldConnectionManager.addOneWayConnection(2)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java:29)
- [ ] `MinimapWorldConnectionManager.removeConnection(2)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java:38)
- [ ] `MinimapWorldConnectionManager.removeConnection(2)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java:42)
- [ ] `MinimapWorldConnectionManager.removeOneWayConnection(2)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java:47)
- [ ] `MinimapWorldConnectionManager.isConnected(2)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java:54)
- [ ] `MinimapWorldConnectionManager.isEmpty(0)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java:67)
- [ ] `MinimapWorldConnectionManager.save(1)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java:71)
- [ ] `MinimapWorldConnectionManager.swapConnections(2)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java:89)
- [ ] `MinimapWorldConnectionManager.swapConnections(2)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java:93)
- [ ] `MinimapWorldConnectionManager.renameDimension(2)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java:122)

### `MinimapWorldConnectionManager.Builder`

- [x] `MinimapWorldConnectionManager.Builder.Builder(0)` (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java:134)
- [ ] `MinimapWorldConnectionManager.Builder.setDefault(0)`: `this` used as a value (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java:137)
- [ ] `MinimapWorldConnectionManager.Builder.setMultiplayer(1)`: `this` used as a value (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java:142)
- [ ] `MinimapWorldConnectionManager.Builder.build(0)`: it uses MinimapWorldConnectionManager, which isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java:147)
- [ ] `MinimapWorldConnectionManager.Builder.begin(0)`: it calls Builder.setDefault, which isn't translated (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java:151)

### `RootConfigIO`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)

- [ ] `RootConfigIO.RootConfigIO(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/config/io/RootConfigIO.java:35)
- [ ] `RootConfigIO.getFile(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/config/io/RootConfigIO.java:39)
- [ ] `RootConfigIO.save(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/config/io/RootConfigIO.java:53)
- [ ] `RootConfigIO.load(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/config/io/RootConfigIO.java:144)

### `RootConfig`: it uses WaypointsSort, which isn't translated (its constants carry values)

- [ ] `RootConfig.RootConfig(1)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:20)
- [ ] `RootConfig.isUsingMultiworldDetection(0)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:24)
- [ ] `RootConfig.setUsingMultiworldDetection(1)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:28)
- [ ] `RootConfig.getDefaultMultiworldId(0)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:32)
- [ ] `RootConfig.setDefaultMultiworldId(1)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:36)
- [ ] `RootConfig.isTeleportationEnabled(0)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:40)
- [ ] `RootConfig.setTeleportationEnabled(1)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:44)
- [ ] `RootConfig.isUsingDefaultTeleportCommand(0)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:48)
- [ ] `RootConfig.setUsingDefaultTeleportCommand(1)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:52)
- [ ] `RootConfig.getServerTeleportCommandFormat(0)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:56)
- [ ] `RootConfig.getServerTeleportCommandRotationFormat(0)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:60)
- [ ] `RootConfig.setServerTeleportCommandFormat(1)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:64)
- [ ] `RootConfig.setServerTeleportCommandRotationFormat(1)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:68)
- [ ] `RootConfig.getSortType(0)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:72)
- [ ] `RootConfig.setSortType(1)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:76)
- [ ] `RootConfig.toggleSortType(0)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:80)
- [ ] `RootConfig.isSortReversed(0)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:84)
- [ ] `RootConfig.setSortReversed(1)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:88)
- [ ] `RootConfig.toggleSortReversed(0)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:92)
- [ ] `RootConfig.isIgnoreServerLevelId(0)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:96)
- [ ] `RootConfig.setIgnoreServerLevelId(1)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:100)
- [ ] `RootConfig.isIgnoreHeightmaps(0)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:104)
- [ ] `RootConfig.setIgnoreHeightmaps(1)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:108)
- [ ] `RootConfig.resetSubWorldConnections(1)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:112)
- [ ] `RootConfig.getSubWorldConnections(0)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:116)
- [ ] `RootConfig.isLoaded(0)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:120)
- [ ] `RootConfig.setLoaded(1)`: its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)) (xaero/hud/minimap/world/container/config/RootConfig.java:124)

### `MinimapWorldContainer`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)

- [x] `MinimapWorldContainer.UNKNOWN_DIM_ID` (xaero/hud/minimap/world/container/MinimapWorldContainer.java:25)
- [ ] `MinimapWorldContainer.MinimapWorldContainer(10)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:38)
- [ ] `MinimapWorldContainer.getEquivalentDimKey(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:66)
- [ ] `MinimapWorldContainer.getEquivalentDimId(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:70)
- [ ] `MinimapWorldContainer.setPath(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:74)
- [ ] `MinimapWorldContainer.addSubContainer(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:86)
- [ ] `MinimapWorldContainer.containsSubContainer(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:109)
- [ ] `MinimapWorldContainer.deleteSubContainer(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:123)
- [ ] `MinimapWorldContainer.isEmpty(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:136)
- [ ] `MinimapWorldContainer.addWorld(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:140)
- [ ] `MinimapWorldContainer.addWorld(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:170)
- [ ] `MinimapWorldContainer.removeWorld(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:178)
- [ ] `MinimapWorldContainer.setName(2)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:182)
- [ ] `MinimapWorldContainer.getName(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:191)
- [ ] `MinimapWorldContainer.removeName(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:211)
- [ ] `MinimapWorldContainer.getLastNode(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:215)
- [ ] `MinimapWorldContainer.getSubName(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:219)
- [ ] `MinimapWorldContainer.getFullWorldName(2)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:233)
- [ ] `MinimapWorldContainer.getPath(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:255)
- [ ] `MinimapWorldContainer.getFirstWorld(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:259)
- [ ] `MinimapWorldContainer.getFirstWorldConnectedTo(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:274)
- [ ] `MinimapWorldContainer.toString(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:295)
- [ ] `MinimapWorldContainer.getWorlds(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:300)
- [ ] `MinimapWorldContainer.getWorldsCopy(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:304)
- [ ] `MinimapWorldContainer.getSubContainers(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:308)
- [ ] `MinimapWorldContainer.getAllWorldsIterable(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:312)
- [ ] `MinimapWorldContainer.fixPathCharacterCases(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:322)
- [ ] `MinimapWorldContainer.getRoot(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:347)
- [ ] `MinimapWorldContainer.getRootConfig(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:351)
- [ ] `MinimapWorldContainer.getDirectoryPath(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:355)
- [ ] `MinimapWorldContainer.getSession(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:360)
- [ ] `MinimapWorldContainer.getServerWaypointManager(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:364)
- [ ] `MinimapWorldContainer.getThirdPartyWaypointManager(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:369)

### `MinimapWorldContainer.Builder`: it's generic

- [ ] `MinimapWorldContainer.Builder.Builder(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:380)
- [ ] `MinimapWorldContainer.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:383)
- [ ] `MinimapWorldContainer.Builder.setModMain(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:391)
- [ ] `MinimapWorldContainer.Builder.setSession(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:396)
- [ ] `MinimapWorldContainer.Builder.setPath(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:401)
- [ ] `MinimapWorldContainer.Builder.setRootContainer(1)`: its class isn't translated (it's generic) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:406)
- [ ] `MinimapWorldContainer.Builder.determineDimensionEquivalent(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:411)
- [ ] `MinimapWorldContainer.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:427)
- [ ] `MinimapWorldContainer.Builder.buildInternally(6)`: it has no body (xaero/hud/minimap/world/container/MinimapWorldContainer.java:438)

### `MinimapWorldContainer.FinalBuilder`: it extends `Builder`

- [ ] `MinimapWorldContainer.FinalBuilder.FinalBuilder(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:449)
- [ ] `MinimapWorldContainer.FinalBuilder.build(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:452)
- [ ] `MinimapWorldContainer.FinalBuilder.buildInternally(6)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:461)
- [ ] `MinimapWorldContainer.FinalBuilder.begin(0)`: it uses FinalBuilder, which isn't translated (it extends `Builder`) (xaero/hud/minimap/world/container/MinimapWorldContainer.java:484)

### `MinimapWorldContainerUtil`

- [ ] `MinimapWorldContainerUtil.isMultiplayer(1)`: it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/minimap/world/container/MinimapWorldContainerUtil.java:6)
- [x] `MinimapWorldContainerUtil.convertWorldFolderToContainerNode(1)` (xaero/hud/minimap/world/container/MinimapWorldContainerUtil.java:11)
- [x] `MinimapWorldContainerUtil.convertWorldFolderToContainerNode(2)` (xaero/hud/minimap/world/container/MinimapWorldContainerUtil.java:15)

### `MinimapWorldRootContainer`: it extends `MinimapWorldContainer`

- [ ] `MinimapWorldRootContainer.MinimapWorldRootContainer(12)`: its class isn't translated (it extends `MinimapWorldContainer`) (xaero/hud/minimap/world/container/MinimapWorldRootContainer.java:34)
- [ ] `MinimapWorldRootContainer.updateConnectionsField(1)`: its class isn't translated (it extends `MinimapWorldContainer`) (xaero/hud/minimap/world/container/MinimapWorldRootContainer.java:54)
- [ ] `MinimapWorldRootContainer.getSubWorldConnections(0)`: its class isn't translated (it extends `MinimapWorldContainer`) (xaero/hud/minimap/world/container/MinimapWorldRootContainer.java:58)
- [ ] `MinimapWorldRootContainer.getDimensionType(1)`: its class isn't translated (it extends `MinimapWorldContainer`) (xaero/hud/minimap/world/container/MinimapWorldRootContainer.java:62)
- [ ] `MinimapWorldRootContainer.getDimensionScale(1)`: its class isn't translated (it extends `MinimapWorldContainer`) (xaero/hud/minimap/world/container/MinimapWorldRootContainer.java:102)
- [ ] `MinimapWorldRootContainer.updateDimensionType(1)`: its class isn't translated (it extends `MinimapWorldContainer`) (xaero/hud/minimap/world/container/MinimapWorldRootContainer.java:107)
- [ ] `MinimapWorldRootContainer.renameOldContainer(1)`: its class isn't translated (it extends `MinimapWorldContainer`) (xaero/hud/minimap/world/container/MinimapWorldRootContainer.java:118)
- [ ] `MinimapWorldRootContainer.getDimensionTypeIds(0)`: its class isn't translated (it extends `MinimapWorldContainer`) (xaero/hud/minimap/world/container/MinimapWorldRootContainer.java:161)
- [ ] `MinimapWorldRootContainer.setDimensionTypeId(2)`: its class isn't translated (it extends `MinimapWorldContainer`) (xaero/hud/minimap/world/container/MinimapWorldRootContainer.java:165)
- [ ] `MinimapWorldRootContainer.getRoot(0)`: its class isn't translated (it extends `MinimapWorldContainer`) (xaero/hud/minimap/world/container/MinimapWorldRootContainer.java:170)
- [ ] `MinimapWorldRootContainer.isConfigLoaded(0)`: its class isn't translated (it extends `MinimapWorldContainer`) (xaero/hud/minimap/world/container/MinimapWorldRootContainer.java:175)
- [ ] `MinimapWorldRootContainer.getConfig(0)`: its class isn't translated (it extends `MinimapWorldContainer`) (xaero/hud/minimap/world/container/MinimapWorldRootContainer.java:179)

### `MinimapWorldRootContainer.Builder`: it extends `Builder`

- [ ] `MinimapWorldRootContainer.Builder.Builder(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/world/container/MinimapWorldRootContainer.java:184)
- [ ] `MinimapWorldRootContainer.Builder.setDefault(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/world/container/MinimapWorldRootContainer.java:187)
- [ ] `MinimapWorldRootContainer.Builder.build(0)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/world/container/MinimapWorldRootContainer.java:192)
- [ ] `MinimapWorldRootContainer.Builder.buildInternally(6)`: its class isn't translated (it extends `Builder`) (xaero/hud/minimap/world/container/MinimapWorldRootContainer.java:196)
- [ ] `MinimapWorldRootContainer.Builder.begin(0)`: it uses Builder, which isn't translated (it extends `Builder`) (xaero/hud/minimap/world/container/MinimapWorldRootContainer.java:221)

### `MinimapWorldManagerIO`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)

- [ ] `MinimapWorldManagerIO.MinimapWorldManagerIO(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:40)
- [ ] `MinimapWorldManagerIO.loadWorldsFromAllSources(2)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:47)
- [ ] `MinimapWorldManagerIO.loadAllWorlds(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:56)
- [ ] `MinimapWorldManagerIO.loadWorldFolder(3)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:87)
- [ ] `MinimapWorldManagerIO.loadDimensionFolder(4)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:129)
- [ ] `MinimapWorldManagerIO.loadWorldFile(3)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:161)
- [ ] `MinimapWorldManagerIO.loadWorld(2)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:187)
- [ ] `MinimapWorldManagerIO.checkWorldFileLine(2)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:228)
- [ ] `MinimapWorldManagerIO.saveWorlds(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:244)
- [ ] `MinimapWorldManagerIO.saveAllWorlds(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:250)
- [ ] `MinimapWorldManagerIO.saveWorld(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:256)
- [ ] `MinimapWorldManagerIO.saveWorld(2)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:260)
- [ ] `MinimapWorldManagerIO.convertWorldDimFilesToFolders(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:291)
- [ ] `MinimapWorldManagerIO.convertWorldDimFoldersToSingleFolder(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:335)
- [ ] `MinimapWorldManagerIO.copyTempFilesBack(1)`: it uses `Stream` (java.util.stream) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:394)
- [ ] `MinimapWorldManagerIO.fixOldRootFolder(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:415)
- [ ] `MinimapWorldManagerIO.fixOldRootFolder(2)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:423)
- [ ] `MinimapWorldManagerIO.onRootContainerAdded(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:441)
- [ ] `MinimapWorldManagerIO.getWorldFile(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:447)
- [ ] `MinimapWorldManagerIO.getRootConfigIO(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java:463)

### `MinimapDimensionHelper`: it uses Minecraft's `ResourceKey`

- [ ] `MinimapDimensionHelper.getDimensionDivision(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapDimensionHelper.java:18)
- [ ] `MinimapDimensionHelper.getDimCoordinateScale(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapDimensionHelper.java:27)
- [ ] `MinimapDimensionHelper.getDimensionDirectoryName(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapDimensionHelper.java:37)
- [ ] `MinimapDimensionHelper.convertToDimensionDirectoryName(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapDimensionHelper.java:47)
- [ ] `MinimapDimensionHelper.findDimensionKeyForOldName(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapDimensionHelper.java:54)
- [ ] `MinimapDimensionHelper.getDimensionKeyForDirectoryName(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapDimensionHelper.java:64)

### `MinimapWorld`: it uses Minecraft's `ResourceKey`

- [x] `MinimapWorld.DEFAULT_SET` (xaero/hud/minimap/world/MinimapWorld.java:18)
- [ ] `MinimapWorld.MinimapWorld(3)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:27)
- [ ] `MinimapWorld.getCurrentWaypointSet(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:35)
- [ ] `MinimapWorld.addWaypointSet(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:39)
- [ ] `MinimapWorld.cleanupOnSave(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:43)
- [ ] `MinimapWorld.getLocalWorldKey(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:52)
- [ ] `MinimapWorld.addWaypointSet(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:57)
- [ ] `MinimapWorld.getWaypointSet(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:61)
- [ ] `MinimapWorld.removeWaypointSet(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:65)
- [ ] `MinimapWorld.getIterableWaypointSets(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:69)
- [ ] `MinimapWorld.getCurrentWaypointSetId(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:73)
- [ ] `MinimapWorld.setCurrentWaypointSetId(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:77)
- [ ] `MinimapWorld.getNode(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:81)
- [ ] `MinimapWorld.getFullPath(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:85)
- [ ] `MinimapWorld.setNode(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:89)
- [ ] `MinimapWorld.getContainer(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:93)
- [ ] `MinimapWorld.setContainer(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:97)
- [ ] `MinimapWorld.requestRemovalOnSave(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:101)
- [ ] `MinimapWorld.hasSomethingToRemoveOnSave(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:105)
- [ ] `MinimapWorld.getDimId(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:109)
- [ ] `MinimapWorld.setDimId(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:113)
- [ ] `MinimapWorld.getSetCount(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:117)
- [ ] `MinimapWorld.getSlimeChunkSeed(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:121)
- [ ] `MinimapWorld.setSlimeChunkSeed(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:125)
- [ ] `MinimapWorld.getRootConfig(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/hud/minimap/world/MinimapWorld.java:129)

### `MinimapWorld.Builder`: it uses MinimapWorldContainer, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))

- [ ] `MinimapWorld.Builder.Builder(0)`: its class isn't translated (it uses MinimapWorldContainer, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))) (xaero/hud/minimap/world/MinimapWorld.java:138)
- [ ] `MinimapWorld.Builder.setDefault(0)`: its class isn't translated (it uses MinimapWorldContainer, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))) (xaero/hud/minimap/world/MinimapWorld.java:141)
- [ ] `MinimapWorld.Builder.setContainer(1)`: its class isn't translated (it uses MinimapWorldContainer, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))) (xaero/hud/minimap/world/MinimapWorld.java:148)
- [ ] `MinimapWorld.Builder.setNode(1)`: its class isn't translated (it uses MinimapWorldContainer, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))) (xaero/hud/minimap/world/MinimapWorld.java:153)
- [ ] `MinimapWorld.Builder.setDimId(1)`: its class isn't translated (it uses MinimapWorldContainer, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))) (xaero/hud/minimap/world/MinimapWorld.java:158)
- [ ] `MinimapWorld.Builder.build(0)`: its class isn't translated (it uses MinimapWorldContainer, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))) (xaero/hud/minimap/world/MinimapWorld.java:163)
- [ ] `MinimapWorld.Builder.begin(0)`: it uses Builder, which isn't translated (it uses MinimapWorldContainer, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))) (xaero/hud/minimap/world/MinimapWorld.java:174)

### `MinimapWorldManager`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)

- [ ] `MinimapWorldManager.MinimapWorldManager(2)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:24)
- [ ] `MinimapWorldManager.getWorldContainer(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:31)
- [ ] `MinimapWorldManager.getWorldContainerNullable(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:35)
- [ ] `MinimapWorldManager.getRootWorldContainer(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:39)
- [ ] `MinimapWorldManager.getRootWorldContainer(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:43)
- [ ] `MinimapWorldManager.addWorldContainer(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:47)
- [ ] `MinimapWorldManager.addRootWorldContainer(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:62)
- [ ] `MinimapWorldManager.removeContainer(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:70)
- [ ] `MinimapWorldManager.containerExists(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:79)
- [ ] `MinimapWorldManager.getWorld(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:88)
- [ ] `MinimapWorldManager.addWorld(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:92)
- [ ] `MinimapWorldManager.getCurrentWorld(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:101)
- [ ] `MinimapWorldManager.getCurrentWorld(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:105)
- [ ] `MinimapWorldManager.getAutoWorld(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:109)
- [ ] `MinimapWorldManager.getRootContainers(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:114)
- [ ] `MinimapWorldManager.getAutoRootContainer(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:118)
- [ ] `MinimapWorldManager.getCurrentRootContainer(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:122)
- [ ] `MinimapWorldManager.getCustomWaypoints(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:127)
- [ ] `MinimapWorldManager.hasCustomWaypoints(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:131)
- [ ] `MinimapWorldManager.getCustomWaypoints(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/MinimapWorldManager.java:135)

### `MinimapWorldState`: it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))

- [ ] `MinimapWorldState.setAutoRootContainerPath(1)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/state/MinimapWorldState.java:12)
- [ ] `MinimapWorldState.getAutoRootContainerPath(0)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/state/MinimapWorldState.java:20)
- [ ] `MinimapWorldState.setOutdatedAutoRootContainerPath(2)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/state/MinimapWorldState.java:24)
- [ ] `MinimapWorldState.getOutdatedAutoRootContainerPath(1)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/state/MinimapWorldState.java:32)
- [ ] `MinimapWorldState.getAutoWorldPath(0)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/state/MinimapWorldState.java:36)
- [ ] `MinimapWorldState.setAutoWorldPath(1)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/state/MinimapWorldState.java:40)
- [ ] `MinimapWorldState.getCustomContainerPath(0)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/state/MinimapWorldState.java:44)
- [ ] `MinimapWorldState.getCustomWorldPath(0)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/state/MinimapWorldState.java:48)
- [ ] `MinimapWorldState.setCustomWorldPath(1)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/state/MinimapWorldState.java:52)
- [ ] `MinimapWorldState.getAutoContainerPathIgnoreCaseCache(0)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/state/MinimapWorldState.java:56)
- [ ] `MinimapWorldState.setAutoContainerPathIgnoreCaseCache(1)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/state/MinimapWorldState.java:60)
- [ ] `MinimapWorldState.getCurrentWorldPath(0)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/state/MinimapWorldState.java:64)
- [ ] `MinimapWorldState.getCurrentContainerPath(0)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/state/MinimapWorldState.java:68)
- [ ] `MinimapWorldState.getCurrentRootContainerPath(0)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/state/MinimapWorldState.java:73)
- [ ] `MinimapWorldState.getCurrentWorldPath(1)`: its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))) (xaero/hud/minimap/world/state/MinimapWorldState.java:78)

### `MinimapWorldStateUpdater`: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)

- [x] `MinimapWorldStateUpdater.ROOT_CONTAINER_FORMAT` (xaero/hud/minimap/world/state/MinimapWorldStateUpdater.java:27)
- [ ] `MinimapWorldStateUpdater.MinimapWorldStateUpdater(3)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/state/MinimapWorldStateUpdater.java:33)
- [ ] `MinimapWorldStateUpdater.init(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/state/MinimapWorldStateUpdater.java:39)
- [ ] `MinimapWorldStateUpdater.update(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/state/MinimapWorldStateUpdater.java:47)
- [ ] `MinimapWorldStateUpdater.getPotentialContainerPath(0)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/state/MinimapWorldStateUpdater.java:69)
- [ ] `MinimapWorldStateUpdater.ignoreContainerCase(2)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/state/MinimapWorldStateUpdater.java:75)
- [ ] `MinimapWorldStateUpdater.getAutoRootContainerPath(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/state/MinimapWorldStateUpdater.java:90)
- [ ] `MinimapWorldStateUpdater.getPotentialWorldNode(2)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/state/MinimapWorldStateUpdater.java:143)
- [ ] `MinimapWorldStateUpdater.hasServerLevelId(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/state/MinimapWorldStateUpdater.java:184)
- [ ] `MinimapWorldStateUpdater.getAutoWorldNodeBase(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/state/MinimapWorldStateUpdater.java:189)
- [ ] `MinimapWorldStateUpdater.onServerLevelId(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/state/MinimapWorldStateUpdater.java:194)
- [ ] `MinimapWorldStateUpdater.setCurrentWorldSpawn(1)`: its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)) (xaero/hud/minimap/world/state/MinimapWorldStateUpdater.java:200)

### `HudModule`: it's generic

- [ ] `HudModule.HudModule(6)`: its class isn't translated (it's generic) (xaero/hud/module/HudModule.java:30)
- [ ] `HudModule.getId(0)`: its class isn't translated (it's generic) (xaero/hud/module/HudModule.java:48)
- [ ] `HudModule.isActive(1)`: its class isn't translated (it's generic) (xaero/hud/module/HudModule.java:52)
- [ ] `HudModule.setActive(2)`: its class isn't translated (it's generic) (xaero/hud/module/HudModule.java:56)
- [ ] `HudModule.getCurrentSession(0)`: its class isn't translated (it's generic) (xaero/hud/module/HudModule.java:60)
- [ ] `HudModule.getRenderer(0)`: its class isn't translated (it's generic) (xaero/hud/module/HudModule.java:65)
- [ ] `HudModule.getUsedTransform(0)`: its class isn't translated (it's generic) (xaero/hud/module/HudModule.java:73)
- [ ] `HudModule.getUnconfirmedTransform(0)`: its class isn't translated (it's generic) (xaero/hud/module/HudModule.java:85)
- [ ] `HudModule.confirmTransform(0)`: its class isn't translated (it's generic) (xaero/hud/module/HudModule.java:93)
- [ ] `HudModule.getConfirmedTransform(0)`: its class isn't translated (it's generic) (xaero/hud/module/HudModule.java:100)
- [ ] `HudModule.setTransform(1)`: its class isn't translated (it's generic) (xaero/hud/module/HudModule.java:104)
- [ ] `HudModule.cancelTransform(0)`: its class isn't translated (it's generic) (xaero/hud/module/HudModule.java:109)
- [ ] `HudModule.getPushState(0)`: its class isn't translated (it's generic) (xaero/hud/module/HudModule.java:113)
- [ ] `HudModule.getDisplayName(0)`: its class isn't translated (it's generic) (xaero/hud/module/HudModule.java:117)
- [ ] `HudModule.getConfigScreenFactory(0)`: its class isn't translated (it's generic) (xaero/hud/module/HudModule.java:121)
- [ ] `HudModule.getSessionFactory(0)`: its class isn't translated (it's generic) (xaero/hud/module/HudModule.java:125)
- [ ] `HudModule.setRenderer(1)`: its class isn't translated (it's generic) (xaero/hud/module/HudModule.java:129)

### `ModuleManager`: it uses HudModule, which isn't translated (it's generic)

- [ ] `ModuleManager.register(1)`: its class isn't translated (it uses HudModule, which isn't translated (it's generic)) (xaero/hud/module/ModuleManager.java:10)
- [ ] `ModuleManager.get(1)`: its class isn't translated (it uses HudModule, which isn't translated (it's generic)) (xaero/hud/module/ModuleManager.java:14)
- [ ] `ModuleManager.getModules(0)`: its class isn't translated (it uses HudModule, which isn't translated (it's generic)) (xaero/hud/module/ModuleManager.java:18)

### `ModuleSession`: it's generic

- [ ] `ModuleSession.ModuleSession(2)`: its class isn't translated (it's generic) (xaero/hud/module/ModuleSession.java:9)
- [ ] `ModuleSession.getModule(0)`: its class isn't translated (it's generic) (xaero/hud/module/ModuleSession.java:14)
- [ ] `ModuleSession.getModMain(0)`: its class isn't translated (it's generic) (xaero/hud/module/ModuleSession.java:18)
- [ ] `ModuleSession.isActive(0)`: its class isn't translated (it's generic) (xaero/hud/module/ModuleSession.java:22)
- [ ] `ModuleSession.getEffectiveX(2)`: its class isn't translated (it's generic) (xaero/hud/module/ModuleSession.java:26)
- [ ] `ModuleSession.getEffectiveY(2)`: its class isn't translated (it's generic) (xaero/hud/module/ModuleSession.java:36)
- [ ] `ModuleSession.isFlippedHor(0)`: its class isn't translated (it's generic) (xaero/hud/module/ModuleSession.java:46)
- [ ] `ModuleSession.isFlippedVer(0)`: its class isn't translated (it's generic) (xaero/hud/module/ModuleSession.java:50)
- [ ] `ModuleSession.isCentered(0)`: its class isn't translated (it's generic) (xaero/hud/module/ModuleSession.java:54)
- [ ] `ModuleSession.shouldFlipHorizontally(2)`: its class isn't translated (it's generic) (xaero/hud/module/ModuleSession.java:58)
- [ ] `ModuleSession.shouldFlipVertically(2)`: its class isn't translated (it's generic) (xaero/hud/module/ModuleSession.java:69)
- [ ] `ModuleSession.prePotentialRender(0)`: its class isn't translated (it's generic) (xaero/hud/module/ModuleSession.java:76)
- [ ] `ModuleSession.onPostGameOverlay(0)`: its class isn't translated (it's generic) (xaero/hud/module/ModuleSession.java:79)
- [ ] `ModuleSession.getWidth(1)`: it has no body (xaero/hud/module/ModuleSession.java:82)
- [ ] `ModuleSession.getHeight(1)`: it has no body (xaero/hud/module/ModuleSession.java:84)
- [ ] `ModuleSession.close(0)`: it has no body (xaero/hud/module/ModuleSession.java:86)

### `ModuleSessionHandler`: it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))

- [ ] `ModuleSessionHandler.ModuleSessionHandler(1)`: its class isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))) (xaero/hud/module/ModuleSessionHandler.java:10)
- [ ] `ModuleSessionHandler.resetSessions(3)`: its class isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))) (xaero/hud/module/ModuleSessionHandler.java:14)
- [ ] `ModuleSessionHandler.closeSessions(1)`: its class isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))) (xaero/hud/module/ModuleSessionHandler.java:20)
- [ ] `ModuleSessionHandler.resetSession(4)`: it's generic (xaero/hud/module/ModuleSessionHandler.java:26)
- [ ] `ModuleSessionHandler.closeSession(2)`: it's generic (xaero/hud/module/ModuleSessionHandler.java:34)

### `ModuleTransform`

- [x] `ModuleTransform.copy(0)` (xaero/hud/module/ModuleTransform.java:13)

### `XaeroNbtUtils`

- [ ] `XaeroNbtUtils.getUUID(2)`: it uses `UUID` (java.util) (xaero/hud/nbt/util/XaeroNbtUtils.java:10)
- [ ] `XaeroNbtUtils.putUUID(3)`: it uses Minecraft's `CompoundTag` (xaero/hud/nbt/util/XaeroNbtUtils.java:20)
- [ ] `XaeroNbtUtils.putUUIDAsLongArray(3)`: it uses Minecraft's `CompoundTag` (xaero/hud/nbt/util/XaeroNbtUtils.java:24)
- [ ] `XaeroNbtUtils.createUUIDTag(1)`: it uses Minecraft's `IntArrayTag` (xaero/hud/nbt/util/XaeroNbtUtils.java:28)

### `ClientboundRulesPacket`

- [x] `ClientboundRulesPacket.ClientboundRulesPacket(3)` (xaero/hud/packet/basic/ClientboundRulesPacket.java:13)
- [ ] `ClientboundRulesPacket.write(1)`: it uses Minecraft's `FriendlyByteBuf` (xaero/hud/packet/basic/ClientboundRulesPacket.java:19)
- [ ] `ClientboundRulesPacket.read(1)`: it uses Minecraft's `FriendlyByteBuf` (xaero/hud/packet/basic/ClientboundRulesPacket.java:27)

### `ClientboundRulesPacket.ClientHandler`: it implements `Consumer` (java.util.function)

- [ ] `ClientboundRulesPacket.ClientHandler.accept(1)`: its class isn't translated (it implements `Consumer` (java.util.function)) (xaero/hud/packet/basic/ClientboundRulesPacket.java:33)

### `HandshakePacket`

- [x] `HandshakePacket.NETWORK_COMPATIBILITY` (xaero/hud/packet/basic/HandshakePacket.java:11)
- [x] `HandshakePacket.HandshakePacket(1)` (xaero/hud/packet/basic/HandshakePacket.java:14)
- [x] `HandshakePacket.HandshakePacket(0)` (xaero/hud/packet/basic/HandshakePacket.java:18)
- [ ] `HandshakePacket.write(1)`: it uses Minecraft's `FriendlyByteBuf` (xaero/hud/packet/basic/HandshakePacket.java:22)
- [ ] `HandshakePacket.read(1)`: it uses Minecraft's `FriendlyByteBuf` (xaero/hud/packet/basic/HandshakePacket.java:26)

### `HandshakePacket.ClientHandler`: it implements `Consumer` (java.util.function)

- [ ] `HandshakePacket.ClientHandler.accept(1)`: its class isn't translated (it implements `Consumer` (java.util.function)) (xaero/hud/packet/basic/HandshakePacket.java:31)

### `HandshakePacket.ServerHandler`: it implements `BiConsumer` (java.util.function)

- [ ] `HandshakePacket.ServerHandler.accept(2)`: its class isn't translated (it implements `BiConsumer` (java.util.function)) (xaero/hud/packet/basic/HandshakePacket.java:41)

### `LevelMapPropertiesConsumer`: it implements `Consumer` (java.util.function)

- [ ] `LevelMapPropertiesConsumer.accept(1)`: its class isn't translated (it implements `Consumer` (java.util.function)) (xaero/hud/packet/LevelMapPropertiesConsumer.java:9)

### `MinimapPacketRegister`

- [ ] `MinimapPacketRegister.register(1)`: it uses `IPacketHandler` (xaero.lib.common.packet) (xaero/hud/packet/MinimapPacketRegister.java:11)

### `ClientboundPlayerTrackerResetPacket`

- [ ] `ClientboundPlayerTrackerResetPacket.write(1)`: it uses Minecraft's `FriendlyByteBuf` (xaero/hud/packet/tracker/ClientboundPlayerTrackerResetPacket.java:8)
- [ ] `ClientboundPlayerTrackerResetPacket.read(1)`: it uses Minecraft's `FriendlyByteBuf` (xaero/hud/packet/tracker/ClientboundPlayerTrackerResetPacket.java:11)

### `ClientboundPlayerTrackerResetPacket.Handler`: it implements `Consumer` (java.util.function)

- [ ] `ClientboundPlayerTrackerResetPacket.Handler.accept(1)`: its class isn't translated (it implements `Consumer` (java.util.function)) (xaero/hud/packet/tracker/ClientboundPlayerTrackerResetPacket.java:16)

### `ClientboundTrackedPlayerPacket`: it uses `UUID` (java.util)

- [ ] `ClientboundTrackedPlayerPacket.ClientboundTrackedPlayerPacket(7)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/hud/packet/tracker/ClientboundTrackedPlayerPacket.java:23)
- [ ] `ClientboundTrackedPlayerPacket.write(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/hud/packet/tracker/ClientboundTrackedPlayerPacket.java:33)
- [ ] `ClientboundTrackedPlayerPacket.read(1)`: it uses ClientboundTrackedPlayerPacket, which isn't translated (it uses `UUID` (java.util)) (xaero/hud/packet/tracker/ClientboundTrackedPlayerPacket.java:52)

### `ClientboundTrackedPlayerPacket.Handler`: it implements `Consumer` (java.util.function)

- [ ] `ClientboundTrackedPlayerPacket.Handler.accept(1)`: its class isn't translated (it implements `Consumer` (java.util.function)) (xaero/hud/packet/tracker/ClientboundTrackedPlayerPacket.java:65)

### `XaeroPath`: it holds an object of its own class (a linked structure)

- [ ] `XaeroPath.XaeroPath(4)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/path/XaeroPath.java:16)
- [ ] `XaeroPath.resolve(1)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/path/XaeroPath.java:28)
- [ ] `XaeroPath.resolve(1)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/path/XaeroPath.java:32)
- [ ] `XaeroPath.resolveSibling(1)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/path/XaeroPath.java:46)
- [ ] `XaeroPath.getSubPath(1)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/path/XaeroPath.java:50)
- [ ] `XaeroPath.getAtIndex(1)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/path/XaeroPath.java:69)
- [ ] `XaeroPath.isSubOf(1)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/path/XaeroPath.java:81)
- [ ] `XaeroPath.applyToFilePath(1)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/path/XaeroPath.java:89)
- [ ] `XaeroPath.toString(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/path/XaeroPath.java:97)
- [ ] `XaeroPath.getNodeCount(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/path/XaeroPath.java:106)
- [ ] `XaeroPath.getLastNode(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/path/XaeroPath.java:110)
- [ ] `XaeroPath.getParent(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/path/XaeroPath.java:114)
- [ ] `XaeroPath.getRoot(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/path/XaeroPath.java:118)
- [ ] `XaeroPath.equals(1)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/path/XaeroPath.java:122)
- [ ] `XaeroPath.hashCode(0)`: its class isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/path/XaeroPath.java:142)
- [ ] `XaeroPath.root(1)`: it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/path/XaeroPath.java:152)
- [ ] `XaeroPath.root(2)`: it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/path/XaeroPath.java:156)

### `XaeroPathReader`

- [ ] `XaeroPathReader.read(1)`: it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/path/XaeroPathReader.java:4)
- [ ] `XaeroPathReader.read(2)`: it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)) (xaero/hud/path/XaeroPathReader.java:8)

### `IPresetAction`: it's an interface

- [ ] `IPresetAction.apply(1)`: it has no body (xaero/hud/preset/action/IPresetAction.java:4)
- [ ] `IPresetAction.confirm(1)`: it has no body (xaero/hud/preset/action/IPresetAction.java:6)
- [ ] `IPresetAction.cancel(1)`: it has no body (xaero/hud/preset/action/IPresetAction.java:8)

### `PlacementPresetAction`: it's generic

- [ ] `PlacementPresetAction.PlacementPresetAction(7)`: its class isn't translated (it's generic) (xaero/hud/preset/action/PlacementPresetAction.java:16)
- [ ] `PlacementPresetAction.apply(1)`: its class isn't translated (it's generic) (xaero/hud/preset/action/PlacementPresetAction.java:26)
- [ ] `PlacementPresetAction.confirm(1)`: its class isn't translated (it's generic) (xaero/hud/preset/action/PlacementPresetAction.java:37)
- [ ] `PlacementPresetAction.cancel(1)`: its class isn't translated (it's generic) (xaero/hud/preset/action/PlacementPresetAction.java:40)

### `HudPreset`: it uses Minecraft's `Component`

- [ ] `HudPreset.HudPreset(3)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/hud/preset/HudPreset.java:14)
- [ ] `HudPreset.getId(0)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/hud/preset/HudPreset.java:20)
- [ ] `HudPreset.getName(0)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/hud/preset/HudPreset.java:24)
- [ ] `HudPreset.apply(0)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/hud/preset/HudPreset.java:28)
- [ ] `HudPreset.confirm(0)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/hud/preset/HudPreset.java:36)
- [ ] `HudPreset.cancel(0)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/hud/preset/HudPreset.java:46)
- [ ] `HudPreset.applyAndConfirm(0)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/hud/preset/HudPreset.java:56)

### `HudPreset.Builder`: it uses Minecraft's `Component`

- [ ] `HudPreset.Builder.Builder(0)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/hud/preset/HudPreset.java:70)
- [ ] `HudPreset.Builder.setDefault(0)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/hud/preset/HudPreset.java:73)
- [ ] `HudPreset.Builder.setId(1)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/hud/preset/HudPreset.java:78)
- [ ] `HudPreset.Builder.setName(1)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/hud/preset/HudPreset.java:83)
- [ ] `HudPreset.Builder.addModulePreset(1)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/hud/preset/HudPreset.java:88)
- [ ] `HudPreset.Builder.build(0)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/hud/preset/HudPreset.java:93)
- [ ] `HudPreset.Builder.begin(0)`: it uses Builder, which isn't translated (it uses Minecraft's `Component`) (xaero/hud/preset/HudPreset.java:101)

### `HudPresetManager`: it uses HudPreset, which isn't translated (it uses Minecraft's `Component`)

- [ ] `HudPresetManager.register(1)`: its class isn't translated (it uses HudPreset, which isn't translated (it uses Minecraft's `Component`)) (xaero/hud/preset/HudPresetManager.java:10)
- [ ] `HudPresetManager.getPresets(0)`: its class isn't translated (it uses HudPreset, which isn't translated (it uses Minecraft's `Component`)) (xaero/hud/preset/HudPresetManager.java:14)

### `ModulePreset`: it's generic

- [ ] `ModulePreset.ModulePreset(2)`: its class isn't translated (it's generic) (xaero/hud/preset/ModulePreset.java:14)
- [ ] `ModulePreset.apply(0)`: its class isn't translated (it's generic) (xaero/hud/preset/ModulePreset.java:19)
- [ ] `ModulePreset.confirm(0)`: its class isn't translated (it's generic) (xaero/hud/preset/ModulePreset.java:25)
- [ ] `ModulePreset.cancel(0)`: its class isn't translated (it's generic) (xaero/hud/preset/ModulePreset.java:31)
- [ ] `ModulePreset.getModule(0)`: its class isn't translated (it's generic) (xaero/hud/preset/ModulePreset.java:37)

### `ModulePreset.Builder`: it's generic

- [ ] `ModulePreset.Builder.Builder(1)`: its class isn't translated (it's generic) (xaero/hud/preset/ModulePreset.java:46)
- [ ] `ModulePreset.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/hud/preset/ModulePreset.java:51)
- [ ] `ModulePreset.Builder.setPlacement(7)`: its class isn't translated (it's generic) (xaero/hud/preset/ModulePreset.java:57)
- [ ] `ModulePreset.Builder.addAction(1)`: its class isn't translated (it's generic) (xaero/hud/preset/ModulePreset.java:64)
- [ ] `ModulePreset.Builder.build(0)`: its class isn't translated (it's generic) (xaero/hud/preset/ModulePreset.java:69)
- [ ] `ModulePreset.Builder.begin(1)`: it's generic (xaero/hud/preset/ModulePreset.java:77)

### `BossHealthPushBox`: it extends `PushBox` (xaero.hud.pushbox)

- [ ] `BossHealthPushBox.BossHealthPushBox(0)`: its class isn't translated (it extends `PushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/boss/BossHealthPushBox.java:6)
- [ ] `BossHealthPushBox.postUpdate(0)`: its class isn't translated (it extends `PushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/boss/BossHealthPushBox.java:10)
- [ ] `BossHealthPushBox.setLastBossHealthHeight(1)`: its class isn't translated (it extends `PushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/boss/BossHealthPushBox.java:17)

### `BossHealthShiftPushBox`: it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)

- [ ] `BossHealthShiftPushBox.BossHealthShiftPushBox(0)`: its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/boss/BossHealthShiftPushBox.java:8)
- [ ] `BossHealthShiftPushBox.getShift(0)`: its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/boss/BossHealthShiftPushBox.java:12)
- [ ] `BossHealthShiftPushBox.postUpdate(0)`: its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/boss/BossHealthShiftPushBox.java:17)
- [ ] `BossHealthShiftPushBox.setLastBossHealthHeight(1)`: its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/boss/BossHealthShiftPushBox.java:24)

### `IBossHealthPushBox`: it's an interface

- [ ] `IBossHealthPushBox.setLastBossHealthHeight(1)`: it has no body (xaero/hud/pushbox/boss/IBossHealthPushBox.java:4)
- [ ] `IBossHealthPushBox.setActive(1)`: it has no body (xaero/hud/pushbox/boss/IBossHealthPushBox.java:6)

### `BuiltInPushBoxes`

- [ ] `BuiltInPushBoxes.POTION_EFFECTS_PUSH_BOX`: it uses PotionEffectsPushBox, which isn't translated (it extends `PushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/BuiltInPushBoxes.java:14)
- [ ] `BuiltInPushBoxes.POTION_EFFECTS_SHIFT_PUSH_BOX`: it uses PotionEffectsShiftPushBox, which isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/BuiltInPushBoxes.java:15)
- [ ] `BuiltInPushBoxes.BOSS_HEALTH_PUSH_BOX`: it uses BossHealthPushBox, which isn't translated (it extends `PushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/BuiltInPushBoxes.java:16)
- [ ] `BuiltInPushBoxes.BOSS_HEALTH_SHIFT_PUSH_BOX`: it uses BossHealthShiftPushBox, which isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/BuiltInPushBoxes.java:17)
- [ ] `BuiltInPushBoxes.getPotionEffectPushBox(1)`: it uses IPotionEffectsPushBox, which isn't translated (it's an interface) (xaero/hud/pushbox/BuiltInPushBoxes.java:19)
- [ ] `BuiltInPushBoxes.getBossHealthPushBox(1)`: it uses IBossHealthPushBox, which isn't translated (it's an interface) (xaero/hud/pushbox/BuiltInPushBoxes.java:24)
- [ ] `BuiltInPushBoxes.addAll(1)`: it uses PushboxManager, which isn't translated (a Map or Set keyed by a PushBox) (xaero/hud/pushbox/BuiltInPushBoxes.java:29)

### `IPotionEffectsPushBox`: it's an interface

- [ ] `IPotionEffectsPushBox.setHasNegative(1)`: it has no body (xaero/hud/pushbox/effect/IPotionEffectsPushBox.java:4)
- [ ] `IPotionEffectsPushBox.setActive(1)`: it has no body (xaero/hud/pushbox/effect/IPotionEffectsPushBox.java:6)

### `PotionEffectsPushBox`: it extends `PushBox` (xaero.hud.pushbox)

- [ ] `PotionEffectsPushBox.PotionEffectsPushBox(0)`: its class isn't translated (it extends `PushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/effect/PotionEffectsPushBox.java:14)
- [ ] `PotionEffectsPushBox.getX(2)`: its class isn't translated (it extends `PushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/effect/PotionEffectsPushBox.java:18)
- [ ] `PotionEffectsPushBox.update(0)`: its class isn't translated (it extends `PushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/effect/PotionEffectsPushBox.java:23)
- [ ] `PotionEffectsPushBox.postUpdate(0)`: its class isn't translated (it extends `PushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/effect/PotionEffectsPushBox.java:31)
- [ ] `PotionEffectsPushBox.calculatePotionDisplayWidth(1)`: it uses IPotionEffectsPushBox, which isn't translated (it's an interface) (xaero/hud/pushbox/effect/PotionEffectsPushBox.java:37)
- [ ] `PotionEffectsPushBox.setHasNegative(1)`: its class isn't translated (it extends `PushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/effect/PotionEffectsPushBox.java:65)

### `PotionEffectsShiftPushBox`: it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)

- [ ] `PotionEffectsShiftPushBox.PotionEffectsShiftPushBox(0)`: its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/effect/PotionEffectsShiftPushBox.java:8)
- [ ] `PotionEffectsShiftPushBox.getX(2)`: its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/effect/PotionEffectsShiftPushBox.java:12)
- [ ] `PotionEffectsShiftPushBox.getShift(0)`: its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/effect/PotionEffectsShiftPushBox.java:17)
- [ ] `PotionEffectsShiftPushBox.update(0)`: its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/effect/PotionEffectsShiftPushBox.java:22)
- [ ] `PotionEffectsShiftPushBox.postUpdate(0)`: its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/effect/PotionEffectsShiftPushBox.java:29)
- [ ] `PotionEffectsShiftPushBox.setHasNegative(1)`: its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)) (xaero/hud/pushbox/effect/PotionEffectsShiftPushBox.java:35)

### `FullHeightShiftPushBox`: it extends `PushBox`

- [ ] `FullHeightShiftPushBox.FullHeightShiftPushBox(3)`: its class isn't translated (it extends `PushBox`) (xaero/hud/pushbox/FullHeightShiftPushBox.java:6)
- [ ] `FullHeightShiftPushBox.update(0)`: its class isn't translated (it extends `PushBox`) (xaero/hud/pushbox/FullHeightShiftPushBox.java:10)
- [ ] `FullHeightShiftPushBox.getH(2)`: its class isn't translated (it extends `PushBox`) (xaero/hud/pushbox/FullHeightShiftPushBox.java:16)
- [ ] `FullHeightShiftPushBox.push(3)`: its class isn't translated (it extends `PushBox`) (xaero/hud/pushbox/FullHeightShiftPushBox.java:21)
- [ ] `FullHeightShiftPushBox.getShift(0)`: it has no body (xaero/hud/pushbox/FullHeightShiftPushBox.java:26)

### `PushBox`

- [x] `PushBox.PushBox(7)` (xaero/hud/pushbox/PushBox.java:13)
- [x] `PushBox.isActive(0)` (xaero/hud/pushbox/PushBox.java:23)
- [x] `PushBox.setActive(1)` (xaero/hud/pushbox/PushBox.java:27)
- [x] `PushBox.getX(2)` (xaero/hud/pushbox/PushBox.java:31)
- [x] `PushBox.getY(2)` (xaero/hud/pushbox/PushBox.java:35)
- [x] `PushBox.getW(2)` (xaero/hud/pushbox/PushBox.java:39)
- [x] `PushBox.getH(2)` (xaero/hud/pushbox/PushBox.java:43)
- [x] `PushBox.getAnchorX(0)` (xaero/hud/pushbox/PushBox.java:47)
- [x] `PushBox.getAnchorY(0)` (xaero/hud/pushbox/PushBox.java:51)
- [x] `PushBox.getVerticalBias(0)` (xaero/hud/pushbox/PushBox.java:55)
- [x] `PushBox.update(0)` (xaero/hud/pushbox/PushBox.java:59)
- [x] `PushBox.postUpdate(0)` (xaero/hud/pushbox/PushBox.java:62)
- [x] `PushBox.push(3)` (xaero/hud/pushbox/PushBox.java:65)

### `PushboxHandler`

- [ ] `PushboxHandler.applyPushboxes(5)`: it uses PushboxManager, which isn't translated (a Map or Set keyed by a PushBox) (xaero/hud/pushbox/PushboxHandler.java:6)
- [ ] `PushboxHandler.updateAll(1)`: it uses PushboxManager, which isn't translated (a Map or Set keyed by a PushBox) (xaero/hud/pushbox/PushboxHandler.java:48)
- [ ] `PushboxHandler.postUpdateAll(1)`: it uses PushboxManager, which isn't translated (a Map or Set keyed by a PushBox) (xaero/hud/pushbox/PushboxHandler.java:54)
- [x] `PushboxHandler.applyScreenEdges(4)` (xaero/hud/pushbox/PushboxHandler.java:60)

### `PushboxHandler.State`

- [ ] `PushboxHandler.State.resetForModule(4)`: it uses ModuleSession, which isn't translated (it's generic) (xaero/hud/pushbox/PushboxHandler.java:84)

### `PushboxManager`: a Map or Set keyed by a PushBox

- [ ] `PushboxManager.add(1)`: its class isn't translated (a Map or Set keyed by a PushBox) (xaero/hud/pushbox/PushboxManager.java:9)
- [ ] `PushboxManager.getAll(0)`: its class isn't translated (a Map or Set keyed by a PushBox) (xaero/hud/pushbox/PushboxManager.java:13)

### `HudRenderer`

- [x] `HudRenderer.HudRenderer(1)` (xaero/hud/render/HudRenderer.java:16)
- [ ] `HudRenderer.render(3)`: it uses Minecraft's `GuiGraphicsExtractor` (xaero/hud/render/HudRenderer.java:20)
- [ ] `HudRenderer.renderModule(5)`: it's generic (xaero/hud/render/HudRenderer.java:40)
- [x] `HudRenderer.getPushboxHandler(0)` (xaero/hud/render/HudRenderer.java:65)

### `IModuleRenderer`: it's an interface

- [ ] `IModuleRenderer.render(4)`: it has no body (xaero/hud/render/module/IModuleRenderer.java:7)

### `ModuleRenderContext`

- [x] `ModuleRenderContext.ModuleRenderContext(3)` (xaero/hud/render/module/ModuleRenderContext.java:14)

### `RenderPipelineWrapper`: it extends Minecraft's `RenderPipeline`

- [ ] `RenderPipelineWrapper.RenderPipelineWrapper(2)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:35)
- [ ] `RenderPipelineWrapper.getDepthStencilState(0)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:53)
- [ ] `RenderPipelineWrapper.getPolygonMode(0)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:57)
- [ ] `RenderPipelineWrapper.isCull(0)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:61)
- [ ] `RenderPipelineWrapper.getColorTargetStates(0)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:65)
- [ ] `RenderPipelineWrapper.getVertexFormatBindings(0)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:69)
- [ ] `RenderPipelineWrapper.getVertexFormatBinding(1)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:73)
- [ ] `RenderPipelineWrapper.getPrimitiveTopology(0)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:77)
- [ ] `RenderPipelineWrapper.getShaders(0)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:81)
- [ ] `RenderPipelineWrapper.getShaderDefines(0)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:85)
- [ ] `RenderPipelineWrapper.getBindGroupLayouts(0)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:89)
- [ ] `RenderPipelineWrapper.pushConstantSize(0)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:93)
- [ ] `RenderPipelineWrapper.getSortKey(0)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:97)
- [ ] `RenderPipelineWrapper.wantsDepthTexture(0)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:101)
- [ ] `RenderPipelineWrapper.setShaderOverrides(1)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:105)
- [ ] `RenderPipelineWrapper.setShaderOverride(2)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:109)
- [ ] `RenderPipelineWrapper.setShaderDefinesOverride(1)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:117)
- [ ] `RenderPipelineWrapper.setBindGroupLayoutOverride(1)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:121)
- [ ] `RenderPipelineWrapper.setDepthStencilStateOverride(1)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:125)
- [ ] `RenderPipelineWrapper.setPolygonModeOverride(1)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:129)
- [ ] `RenderPipelineWrapper.setCullOverride(1)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:133)
- [ ] `RenderPipelineWrapper.setColorTargetStateOverride(2)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:137)
- [ ] `RenderPipelineWrapper.resetColorTargetStateOverrides(0)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:145)
- [ ] `RenderPipelineWrapper.setVertexFormatOverride(2)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:149)
- [ ] `RenderPipelineWrapper.resetVertexFormatOverrides(0)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:157)
- [ ] `RenderPipelineWrapper.setPrimitiveTopologyOverride(1)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:161)
- [ ] `RenderPipelineWrapper.setPushConstantSizeOverride(1)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:165)
- [ ] `RenderPipelineWrapper.setSortKeyOverride(1)`: its class isn't translated (it extends Minecraft's `RenderPipeline`) (xaero/hud/render/pipeline/RenderPipelineWrapper.java:169)

### `TextureLocations`

- [x] `TextureLocations.GUI_TEXTURES` (xaero/hud/render/TextureLocations.java:6)
- [x] `TextureLocations.MINIMAP_FRAME_TEXTURES` (xaero/hud/render/TextureLocations.java:7)
- [x] `TextureLocations.VANILLA_GUI_ICONS_LOCATION` (xaero/hud/render/TextureLocations.java:8)

### `MultiTextureRenderUtil`

- [ ] `MultiTextureRenderUtil.prepareTexturedColoredRect(15)`: it's overloaded with the same number of parameters (xaero/hud/render/util/MultiTextureRenderUtil.java:9)
- [ ] `MultiTextureRenderUtil.prepareTexturedColoredRect(15)`: it's overloaded with the same number of parameters (xaero/hud/render/util/MultiTextureRenderUtil.java:35)
- [ ] `MultiTextureRenderUtil.prepareTexturedRect(11)`: it uses Minecraft's `GpuTextureView` (xaero/hud/render/util/MultiTextureRenderUtil.java:59)

### `RenderBufferUtil`

- [ ] `RenderBufferUtil.addColoredRect(7)`: it uses Minecraft's `VertexConsumer` (xaero/hud/render/util/RenderBufferUtil.java:7)
- [ ] `RenderBufferUtil.addColoredRect(10)`: it uses Minecraft's `VertexConsumer` (xaero/hud/render/util/RenderBufferUtil.java:15)
- [ ] `RenderBufferUtil.addTexturedColoredRect(13)`: it uses Minecraft's `VertexConsumer` (xaero/hud/render/util/RenderBufferUtil.java:22)
- [ ] `RenderBufferUtil.addTexturedColoredRect(15)`: it uses Minecraft's `VertexConsumer` (xaero/hud/render/util/RenderBufferUtil.java:28)

### `SeparatedKeysParser`: a function kept in a variable or field (only parameters can be)

- [ ] `SeparatedKeysParser.SeparatedKeysParser(1)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (xaero/hud/util/SeparatedKeysParser.java:11)
- [ ] `SeparatedKeysParser.parseKeys(1)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (xaero/hud/util/SeparatedKeysParser.java:15)
- [ ] `SeparatedKeysParser.parseKeyUntilChar(4)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (xaero/hud/util/SeparatedKeysParser.java:62)
- [ ] `SeparatedKeysParser.throwError(3)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (xaero/hud/util/SeparatedKeysParser.java:87)

### `MinimapSettingsFunction`: it extends `KeyMappingFunction` (xaero.hud.controls.key.function)

- [ ] `MinimapSettingsFunction.MinimapSettingsFunction(1)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/xminimap/controls/key/function/MinimapSettingsFunction.java:14)
- [ ] `MinimapSettingsFunction.onPress(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/xminimap/controls/key/function/MinimapSettingsFunction.java:19)
- [ ] `MinimapSettingsFunction.onRelease(0)`: its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/xminimap/controls/key/function/MinimapSettingsFunction.java:32)

### `XMinimapKeyMappingFunctions`

- [ ] `XMinimapKeyMappingFunctions.SETTINGS`: it creates a MinimapSettingsFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/xminimap/controls/key/function/XMinimapKeyMappingFunctions.java:8)
- [ ] `XMinimapKeyMappingFunctions.SERVER_PROFILES`: it creates a MinimapSettingsFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)) (xaero/hud/xminimap/controls/key/function/XMinimapKeyMappingFunctions.java:9)
- [ ] `XMinimapKeyMappingFunctions.registerAll(1)`: it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`) (xaero/hud/xminimap/controls/key/function/XMinimapKeyMappingFunctions.java:11)

### `XMinimapKeyMappings`

- [ ] `XMinimapKeyMappings.SETTINGS`: it uses Minecraft's `KeyMapping` (xaero/hud/xminimap/controls/key/XMinimapKeyMappings.java:12)
- [ ] `XMinimapKeyMappings.SERVER_PROFILES`: it uses Minecraft's `KeyMapping` (xaero/hud/xminimap/controls/key/XMinimapKeyMappings.java:13)
- [ ] `XMinimapKeyMappings.registerAll(3)`: it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`) (xaero/hud/xminimap/controls/key/XMinimapKeyMappings.java:17)

### `XMinimapControlsRegister`: it extends `ControlsRegister` (xaero.hud.controls)

- [ ] `XMinimapControlsRegister.registerKeybindings(2)`: its class isn't translated (it extends `ControlsRegister` (xaero.hud.controls)) (xaero/hud/xminimap/controls/XMinimapControlsRegister.java:10)

### `BuiltInHudPresets`

- [ ] `BuiltInHudPresets.TOP_LEFT`: it uses HudPreset, which isn't translated (it uses Minecraft's `Component`) (xaero/hud/xminimap/preset/BuiltInHudPresets.java:11)
- [ ] `BuiltInHudPresets.TOP_RIGHT`: it uses HudPreset, which isn't translated (it uses Minecraft's `Component`) (xaero/hud/xminimap/preset/BuiltInHudPresets.java:16)
- [ ] `BuiltInHudPresets.BOTTOM_LEFT`: it uses HudPreset, which isn't translated (it uses Minecraft's `Component`) (xaero/hud/xminimap/preset/BuiltInHudPresets.java:21)
- [ ] `BuiltInHudPresets.BOTTOM_RIGHT`: it uses HudPreset, which isn't translated (it uses Minecraft's `Component`) (xaero/hud/xminimap/preset/BuiltInHudPresets.java:26)
- [ ] `BuiltInHudPresets.addAll(1)`: it uses HudPresetManager, which isn't translated (it uses HudPreset, which isn't translated (it uses Minecraft's `Component`)) (xaero/hud/xminimap/preset/BuiltInHudPresets.java:32)

### `MinimapGuiHelper`: it extends `GuiHelper` (xaero.common.gui)

- [ ] `MinimapGuiHelper.MinimapGuiHelper(1)`: its class isn't translated (it extends `GuiHelper` (xaero.common.gui)) (xaero/minimap/gui/MinimapGuiHelper.java:12)
- [ ] `MinimapGuiHelper.getMainSettingsScreen(1)`: its class isn't translated (it extends `GuiHelper` (xaero.common.gui)) (xaero/minimap/gui/MinimapGuiHelper.java:16)
- [ ] `MinimapGuiHelper.openMainSettingsFromScreen(1)`: its class isn't translated (it extends `GuiHelper` (xaero.common.gui)) (xaero/minimap/gui/MinimapGuiHelper.java:21)

### `MinimapClientOnly`: it extends `HudClientOnlyBase` (xaero.common)

- [ ] `MinimapClientOnly.preInit(2)`: its class isn't translated (it extends `HudClientOnlyBase` (xaero.common)) (xaero/minimap/MinimapClientOnly.java:8)
- [ ] `MinimapClientOnly.preLoadLater(1)`: its class isn't translated (it extends `HudClientOnlyBase` (xaero.common)) (xaero/minimap/MinimapClientOnly.java:13)

### `XaeroMinimapSupportMods`

- [ ] `XaeroMinimapSupportMods.checkForMinimapDuplicates(0)`: it calls SupportMods.checkForMinimapDuplicates, which isn't translated (xaero/minimap/mods/XaeroMinimapSupportMods.java:6)

### `XaeroMinimap`: it extends `HudMod` (xaero.common)

- [x] `XaeroMinimap.LOGGER` (xaero/minimap/XaeroMinimap.java:26)
- [ ] `XaeroMinimap.instance`: it uses XaeroMinimap, which isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:27)
- [x] `XaeroMinimap.MOD_ID` (xaero/minimap/XaeroMinimap.java:28)
- [ ] `XaeroMinimap.XaeroMinimap(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:30)
- [ ] `XaeroMinimap.loadClient(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:34)
- [ ] `XaeroMinimap.loadCommon(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:39)
- [ ] `XaeroMinimap.getCommonConfigFileName(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:45)
- [ ] `XaeroMinimap.getModId(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:50)
- [ ] `XaeroMinimap.createModSettings(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:55)
- [ ] `XaeroMinimap.createGuiHelper(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:60)
- [ ] `XaeroMinimap.getOldConfigFileName(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:65)
- [ ] `XaeroMinimap.createClientOnly(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:70)
- [ ] `XaeroMinimap.getModName(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:75)
- [ ] `XaeroMinimap.getLogger(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:80)
- [ ] `XaeroMinimap.createForgeEventHandlerListener(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:85)
- [ ] `XaeroMinimap.getVersionsURL(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:90)
- [ ] `XaeroMinimap.getUpdateLink(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:95)
- [ ] `XaeroMinimap.createControlsRegister(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:100)
- [ ] `XaeroMinimap.createSession(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:105)
- [ ] `XaeroMinimap.getSettingsKey(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:110)
- [ ] `XaeroMinimap.getServerSettingsKey(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:115)
- [ ] `XaeroMinimap.getConfigSubFolder(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:120)
- [ ] `XaeroMinimap.getDefaultConfigsSubFolder(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:125)
- [ ] `XaeroMinimap.createConfigCommonRegistryHandler(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:130)
- [ ] `XaeroMinimap.createConfigClientRegistryHandlerSupplier(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:135)
- [ ] `XaeroMinimap.isStandalone(0)`: its class isn't translated (it extends `HudMod` (xaero.common)) (xaero/minimap/XaeroMinimap.java:140)

### `XaeroMinimapFabric`: it extends `XaeroMinimap`

- [x] `XaeroMinimapFabric.fileLayoutID` (xaero/minimap/XaeroMinimapFabric.java:10)
- [ ] `XaeroMinimapFabric.onInitializeClient(0)`: its class isn't translated (it extends `XaeroMinimap`) (xaero/minimap/XaeroMinimapFabric.java:13)
- [ ] `XaeroMinimapFabric.onInitializeServer(0)`: its class isn't translated (it extends `XaeroMinimap`) (xaero/minimap/XaeroMinimapFabric.java:22)
- [ ] `XaeroMinimapFabric.loadClient(0)`: its class isn't translated (it extends `XaeroMinimap`) (xaero/minimap/XaeroMinimapFabric.java:31)
- [ ] `XaeroMinimapFabric.loadCommon(0)`: its class isn't translated (it extends `XaeroMinimap`) (xaero/minimap/XaeroMinimapFabric.java:37)
- [ ] `XaeroMinimapFabric.loadServer(0)`: its class isn't translated (it extends `XaeroMinimap`) (xaero/minimap/XaeroMinimapFabric.java:43)
- [ ] `XaeroMinimapFabric.tryLoadLater(0)`: its class isn't translated (it extends `XaeroMinimap`) (xaero/minimap/XaeroMinimapFabric.java:49)
- [ ] `XaeroMinimapFabric.tryLoadLaterServer(0)`: its class isn't translated (it extends `XaeroMinimap`) (xaero/minimap/XaeroMinimapFabric.java:57)
- [ ] `XaeroMinimapFabric.createPlatformContext(0)`: its class isn't translated (it extends `XaeroMinimap`) (xaero/minimap/XaeroMinimapFabric.java:64)
- [ ] `XaeroMinimapFabric.getFileLayoutID(0)`: its class isn't translated (it extends `XaeroMinimap`) (xaero/minimap/XaeroMinimapFabric.java:69)

### `XaeroMinimapStandaloneSession`: it extends `XaeroMinimapSession` (xaero.common)

- [ ] `XaeroMinimapStandaloneSession.XaeroMinimapStandaloneSession(1)`: its class isn't translated (it extends `XaeroMinimapSession` (xaero.common)) (xaero/minimap/XaeroMinimapStandaloneSession.java:9)
- [ ] `XaeroMinimapStandaloneSession.init(1)`: its class isn't translated (it extends `XaeroMinimapSession` (xaero.common)) (xaero/minimap/XaeroMinimapStandaloneSession.java:13)
