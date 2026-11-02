# Horizon port report: Cinematic Interactions

`cinematic` 0.18.30 · Cinematic Interaction-Fabric-26.3-0.18.30.jar · 322 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 19 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 1 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 39 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 19 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 11 | Horizon has nothing this can attach to yet. |

## Entry points

- `main`: dev.dyslexic.cinematic.Cinematic
- `client`: dev.dyslexic.cinematic.client.CinematicClient
- `modmenu`: dev.dyslexic.cinematic.client.ModMenuIntegration
- `jei_mod_plugin`: dev.dyslexic.cinematic.client.compat.jei.CinematicJeiPlugin

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| ScreenEvents.AFTER_INIT | `CinematicClient` | – | unsupported | No screen hook in Horizon yet. |
| KeyMappingHelper.registerKeyMapping | `CinematicClient` | hook `key` | auto | Mojang-named variant of KeyBindingHelper. |
| ClientPlayNetworking.registerGlobalReceiver | `CinematicClient` | – | unsupported | Custom packets aren't supported. |
| ClientTickEvents.END_CLIENT_TICK | `CinematicClient` | hook `tick` | auto | START and END of the tick both land on Horizon's one tick hook. |
| HudElementRegistry.addLast | `CinematicClient` | hook `hud` | auto |  |
| ClientPlayConnectionEvents.DISCONNECT | `CinematicClient` | hook `tick` | auto | JOIN runs on the window's first tick with a player after none; DISCONNECT isn't mapped. |
| ClientPlayConnectionEvents.JOIN | `CinematicClient` | hook `tick` | auto | JOIN runs on the window's first tick with a player after none; DISCONNECT isn't mapped. |
| ClientPlayNetworking.canSend | `ClientOnly` | – | unsupported | Custom packets aren't supported. |
| KeyMappingHelper.getBoundKeyOf | `KeyLabels` | hook `key` | auto | Mojang-named variant of KeyBindingHelper. |
| PayloadTypeRegistry.serverboundPlay | `Network` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| PayloadTypeRegistry.register | `Network` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| PayloadTypeRegistry.clientboundPlay | `Network` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| ServerPlayNetworking.registerGlobalReceiver | `Network` | – | unsupported | Custom packets aren't supported. |
| ServerPlayNetworking.canSend | `PacketDistributor` | – | unsupported | Custom packets aren't supported. |
| ServerPlayNetworking.send | `PacketDistributor` | – | unsupported | Custom packets aren't supported. |
| ClientPlayNetworking.send | `PacketDistributor` | – | unsupported | Custom packets aren't supported. |
| UseBlockCallback.EVENT | `StationEvents` | – | unsupported | No interaction hook yet: needs a raw target in Horizon's game code. |
| ServerTickEvents.END_SERVER_TICK | `StationEvents` | hook `tick` | auto | The tick hook runs in every Game, the server's too: run server logic only when `!self.native_window()`. |
| ServerTickEvents.START_LEVEL_TICK | `StationEvents` | hook `tick` | auto | The tick hook runs in every Game, the server's too: run server logic only when `!self.native_window()`. |
| ServerTickEvents.END_LEVEL_TICK | `StationEvents` | hook `tick` | auto | The tick hook runs in every Game, the server's too: run server logic only when `!self.native_window()`. |

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Inject(RETURN) on AbstractFurnaceBlockEntity.serverTick | `AbstractFurnaceBlockEntityMixin.cinematic$watch` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for AbstractFurnaceBlockEntity.serverTick: run `match`. |
| @Inject(HEAD) on BossHealthOverlay.extractRenderState | `BossHealthOverlayMixin.cinematic$hide` | HEAD | needs-mapping | No Horizon function known for BossHealthOverlay.extractRenderState: run `match`. |
| @Inject(RETURN) on BrewingStandBlockEntity.serverTick | `BrewingStandBlockEntityMixin.cinematic$watch` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for BrewingStandBlockEntity.serverTick: run `match`. |
| @Inject(TAIL) on Camera.alignWithEntity | `CameraMixin.cinematic$scene` | RETURN on horizon_client_render::camera::Camera::align_with_entity | auto | Horizon has no TAIL: RETURN runs at every exit, early ones too. |
| @Inject(RETURN) on Camera.calculateFov | `CameraMixin.cinematic$fov` | RETURN on horizon_client_render::camera::Camera::calculate_fov | auto | Horizon's RETURN runs at every normal exit and passes the return value on. |
| @Inject(RETURN) on HumanoidArmorLayer.getArmorModel | `CampfireArmorMixin.cinematic$hideStraightArmor` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for HumanoidArmorLayer.getArmorModel: run `match`. |
| @Inject(TAIL) on HumanoidModel.setupAnim | `CampfireLegModelMixin.cinematic$campfireLeg` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for HumanoidModel.setupAnim: run `match`. |
| @Inject(HEAD) on CampfireRenderer.submit | `CampfireRendererMixin.cinematic$layout` | HEAD | needs-mapping | No Horizon function known for CampfireRenderer.submit: run `match`. |
| @Inject(HEAD) on ClientLevel.disconnect | `ClientLevelMixin.cinematic$unload` | HEAD | needs-mapping | No Horizon function known for ClientLevel.disconnect: run `match`. |
| @Accessor on CraftingMenu.access | `CraftingMenuAccess.cinematic$access` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for CraftingMenu.access: run `match`. |
| @Inject(RETURN) on EnchantmentMenu.clickMenuButton | `EnchantmentMenuMixin.cinematic$enchanted` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for EnchantmentMenu.clickMenuButton: run `match`. |
| @Inject(HEAD) on ExperienceOrbRenderer.submit | `ExperienceOrbRendererMixin.cinematic$notInTheLens` | HEAD | needs-mapping | No Horizon function known for ExperienceOrbRenderer.submit: run `match`. |
| mixin into FirstPersonHandsAndItemsRenderer with no injectors (interfaces or @Shadow only) | `FirstPersonHandsMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on Hud.extractCrosshair | `GuiMixin.cinematic$hide` | HEAD | needs-mapping | No Horizon function known for Hud.extractCrosshair: run `match`. |
| @Inject(HEAD) on Hud.extractHotbarAndDecorations | `GuiMixin.cinematic$hide` | HEAD | needs-mapping | No Horizon function known for Hud.extractHotbarAndDecorations: run `match`. |
| @Inject(HEAD) on Hud.extractEffects | `GuiMixin.cinematic$hide` | HEAD | needs-mapping | No Horizon function known for Hud.extractEffects: run `match`. |
| @Inject(HEAD) on Hud.extractCameraOverlays | `GuiMixin.cinematic$hide` | HEAD | needs-mapping | No Horizon function known for Hud.extractCameraOverlays: run `match`. |
| @Inject(HEAD) on Hud.extractOverlayMessage | `GuiMixin.cinematic$hide` | HEAD | needs-mapping | No Horizon function known for Hud.extractOverlayMessage: run `match`. |
| @Inject(HEAD) on Hud.extractTitle | `GuiMixin.cinematic$hide` | HEAD | needs-mapping | No Horizon function known for Hud.extractTitle: run `match`. |
| @Inject(HEAD) on Hud.extractSelectedItemName | `GuiMixin.cinematic$hide` | HEAD | needs-mapping | No Horizon function known for Hud.extractSelectedItemName: run `match`. |
| @Inject(HEAD) on Hud.extractScoreboardSidebar | `GuiMixin.cinematic$hide` | HEAD | needs-mapping | No Horizon function known for Hud.extractScoreboardSidebar: run `match`. |
| @Inject(HEAD) on Hud.extractDemoOverlay | `GuiMixin.cinematic$hide` | HEAD | needs-mapping | No Horizon function known for Hud.extractDemoOverlay: run `match`. |
| @Inject(HEAD) on Hud.extractSleepOverlay | `GuiMixin.cinematic$hide` | HEAD | needs-mapping | No Horizon function known for Hud.extractSleepOverlay: run `match`. |
| @Inject(HEAD) on Gui.setScreen | `GuiScreenMixin.cinematic$screen` | HEAD | needs-mapping | No Horizon function known for Gui.setScreen: run `match`. |
| @Inject(TAIL) on HumanoidModel.setupAnim | `HumanoidModelMixin.cinematic$stationPose` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for HumanoidModel.setupAnim: run `match`. |
| @Inject(HEAD) on ItemInHandLayer.submitArmWithItem | `ItemInHandLayerMixin.cinematic$hammerInHand` | HEAD | needs-mapping | No Horizon function known for ItemInHandLayer.submitArmWithItem: run `match`. |
| @Inject(TAIL) on KeyboardInput.tick | `KeyboardInputMixin.cinematic$input` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for KeyboardInput.tick: run `match`. |
| mixin into LecternRenderer with no injectors (interfaces or @Shadow only) | `LecternRendererMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on LevelRenderer.submitFeatures | `LevelRendererMixin.cinematic$outline` | HEAD | needs-mapping | No Horizon function known for LevelRenderer.submitFeatures: run `match`. |
| @Inject(TAIL) on LevelRenderer.submitFeatures | `LevelRendererMixin.cinematic$scene` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for LevelRenderer.submitFeatures: run `match`. |
| @Inject(HEAD) on LivingEntity.swing | `LivingEntitySwingMixin.cinematic$noSwing` | HEAD on horizon_client::mobs::ai::Mob::swing | auto |  |
| @Inject(TAIL) on LocalPlayer.tick | `LocalPlayerMixin.cinematic$steer` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for LocalPlayer.tick: run `match`. |
| @Inject(INVOKE) on Minecraft.renderFrame at Lnet/minecraft/client/renderer/GameRenderer;update(Lnet/minecraft/client/DeltaTracker;)V | `MinecraftMixin.cinematic$frame` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for Minecraft.renderFrame: run `match`. The call it hooks (GameRenderer.update) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on Minecraft.startUseItem | `MinecraftMixin.cinematic$use` | HEAD on horizon_client::first_person::Clicks::start_use_item | auto |  |
| @Inject(RETURN) on Minecraft.startUseItem | `MinecraftMixin.cinematic$used` | RETURN on horizon_client::first_person::Clicks::start_use_item | auto | Horizon's RETURN runs at every normal exit and passes the return value on. |
| @Inject(HEAD) on Minecraft.tick | `MinecraftMixin.cinematic$tick` | HEAD on hook `tick` | auto |  |
| @Inject(HEAD) on Minecraft.startAttack | `MinecraftMixin.cinematic$attack` | HEAD on horizon_client::first_person::Clicks::start_attack | auto |  |
| @Inject(HEAD) on Minecraft.continueAttack | `MinecraftMixin.cinematic$break` | HEAD on horizon_client::first_person::Clicks::continue_attack | auto |  |
| @Inject(HEAD) on MouseHandler.onMove | `MouseHandlerMixin.cinematic$showcaseCursor` | HEAD | needs-mapping | No Horizon function known for MouseHandler.onMove: run `match`. |
| @Inject(HEAD) on MouseHandler.turnPlayer | `MouseHandlerMixin.cinematic$turn` | HEAD | needs-mapping | No Horizon function known for MouseHandler.turnPlayer: run `match`. |
| @Inject(HEAD) on MouseHandler.onScroll | `MouseHandlerMixin.cinematic$scroll` | HEAD on hook `wheel` | auto |  |
| @Inject(TAIL) on PlayerChunkSender.sendChunk | `PlayerChunkSenderMixin.cinematic$sent` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for PlayerChunkSender.sendChunk: run `match`. |
| @Inject(TAIL) on AvatarRenderer.<init> | `PlayerRendererMixin.cinematic$layer` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for AvatarRenderer.<init>: run `match`. |
| @Inject(RETURN) on AvatarRenderer.getRenderOffset | `PlayerRendererMixin.cinematic$sit` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for AvatarRenderer.getRenderOffset: run `match`. |
| @Inject(HEAD) on GameRenderer.extract | `SceneExtractionMixin.cinematic$begin` | HEAD | needs-mapping | No Horizon function known for GameRenderer.extract: run `match`. |
| @Inject(INVOKE) on GameRenderer.extract at Lnet/minecraft/client/gui/Gui;extractRenderState(Lnet/minecraft/client/DeltaTracker;ZZ)V | `SceneExtractionMixin.cinematic$extract` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for GameRenderer.extract: run `match`. The call it hooks (Gui.extractRenderState) isn't mapped to a Rust function yet. |
| mixin into LevelRenderState with no injectors (interfaces or @Shadow only) | `SceneFrameMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(TAIL) on Screen.init | `ScreenMixin.cinematic$initialized` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for Screen.init: run `match`. |
| @Invoker on Screen.addRenderableWidget | `ScreenWidgetAccess.cinematic$addRenderableWidget` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for Screen.addRenderableWidget: run `match`. |
| @Inject(HEAD) on ServerLevel.updateNeighborsAt | `ServerLevelMixin.cinematic$neighbours` | HEAD on horizon_world::level::Behaviours::update_neighbors_at | auto |  |
| @Inject(RETURN) on ServerPlayer.openMenu | `ServerPlayerMixin.cinematic$openedMenu` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for ServerPlayer.openMenu: run `match`. |
| @Inject(HEAD) on CameraAnimationState.isActive | `PunchyCameraStateMixin.cinematic$ownCameraDuringScene` | HEAD | needs-mapping | No Horizon function known for CameraAnimationState.isActive: run `match`. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends AbstractTexture | `BookPages$Page` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `ClientOnlyHintScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends Model | `KneeLayer$1` | needs-translation | No mapping for this superclass yet. |
| extends RenderLayer | `KneeLayer` | needs-translation | No mapping for this superclass yet. |
| extends PictureInPictureRenderer | `LoomFlags$Renderer` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `MultiplayerSettingsScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends RenderLayer | `PropLayer` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `SceneScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends Plain | `SettingsScreen$4$1` | needs-translation | No mapping for this superclass yet. |
| extends AbstractSliderButton | `SettingsScreen$IntSlider` | needs-translation | No mapping for this superclass yet. |
| extends Entry | `SettingsScreen$Row` | needs-translation | No mapping for this superclass yet. |
| extends ContainerObjectSelectionList | `SettingsScreen$Rows` | needs-translation | No mapping for this superclass yet. |
| extends Plain | `SettingsScreen$StationButton` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `SettingsScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends SavedData | `TableStore` | needs-translation | No mapping for this superclass yet. |

## Access wideners

| File | In | Status | Note |
|---|---|---|---|
| access widener (opens private game fields) | `cinematic.accesswidener` | needs-translation | Native helpers already see the game's fields, once each field is mapped. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| 21 asset files, 0 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

67 of 1491 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/client/Camera#alignWithEntity` | horizon_client_render::camera::Camera::align_with_entity (suggested) |
| `net/minecraft/client/Camera#calculateFov` | horizon_client_render::camera::Camera::calculate_fov (suggested) |
| `net/minecraft/client/KeyMapping#<init>` | `self.native_mod_key(MOD_ID, name, default)` |
| `net/minecraft/client/KeyMapping#click` | horizon_client_input::keymapping::KeyState::click (suggested) |
| `net/minecraft/client/KeyMapping#consumeClick` | `key == self.native_mod_key(..) && pressed, in the key hook` |
| `net/minecraft/client/Minecraft#continueAttack` | horizon_client::first_person::Clicks::continue_attack (suggested) |
| `net/minecraft/client/Minecraft#font` | `(implicit)` |
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#hasControlDown` | horizon_client::game::Game<'_>::has_control_down (suggested) |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/Minecraft#startAttack` | horizon_client::first_person::Clicks::start_attack (suggested) |
| `net/minecraft/client/Minecraft#startUseItem` | horizon_client::first_person::Clicks::start_use_item (suggested) |
| `net/minecraft/client/Minecraft#tick` | hook `tick` |
| `net/minecraft/client/MouseHandler#onScroll` | hook `wheel` |
| `net/minecraft/client/OptionInstance#set` | horizon_client_render::options_file::OptionsFile::set (suggested) |
| `net/minecraft/client/gui/Font#plainSubstrByWidth` | horizon_client_render::font::Font::fit (suggested) |
| `net/minecraft/client/gui/Font#split` | horizon_client_render::font::Font::split_formatted (suggested) |
| `net/minecraft/client/gui/Font#width` | `self.native_text_width(text)` |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blitSprite` | horizon_client_render::ui::blit_sprite (suggested) |
| `net/minecraft/client/gui/GuiGraphicsExtractor#itemDecorations` | horizon_client_render::hud_overlays::item_decorations (suggested) |
| `net/minecraft/client/gui/components/EditBox#keyPressed` | horizon_client_render::ui::EditBox::shortcut (suggested) |
| `net/minecraft/client/player/LocalPlayer#isInLava` | horizon_client::motion::fluids::PlayerMotion::is_in_lava (suggested) |
| `net/minecraft/client/player/LocalPlayer#onClimbable` | horizon_client::mobs::ai::on_climbable (suggested) |
| `net/minecraft/client/player/LocalPlayer#onGround` | horizon_client::game::minecarts::Minecart::on_ground (suggested) |
| `net/minecraft/client/player/LocalPlayer#swing` | horizon_client::mobs::ai::Mob::swing (suggested) |
| `net/minecraft/client/player/RemotePlayer#setPose` | horizon_client::mobs::warden::set_pose (suggested) |
| `net/minecraft/client/renderer/item/ItemStackRenderState#submit` | horizon_client::item_render::ItemRenderState::submit (suggested) |
| `net/minecraft/client/renderer/special/SpecialModelRenderer#submit` | horizon_client::item_render::special::submit (suggested) |
| `net/minecraft/core/Direction#fromYRot` | horizon_world::level::place::from_y_rot (suggested) |
| `net/minecraft/core/Direction#getOpposite` | horizon_client::mobs::shulker::opposite (suggested) |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK` | content |
| `net/minecraft/core/registries/BuiltInRegistries#ITEM` | content |
| `net/minecraft/network/chat/MutableComponent#append` | horizon_client::game::dedicated::append (suggested) |
| `net/minecraft/network/chat/MutableComponent#withStyle` | horizon_client_core::text::Text::with_style (suggested) |
| `net/minecraft/network/codec/ByteBufCodecs#list` | horizon_protocol::item::skip::list (suggested) |
| `net/minecraft/network/codec/ByteBufCodecs#optional` | horizon_protocol::item::skip::optional (suggested) |
| `net/minecraft/server/level/ServerLevel#destroyBlock` | horizon_world::level::update::Sim<'_, L>::destroy_block (suggested) |
| `net/minecraft/server/level/ServerLevel#sendParticles` | horizon_client::game::fishing::Game<'_>::send_particles (suggested) |
| `net/minecraft/server/level/ServerLevel#setBlock` | horizon_world::level::Behaviours::set_block (suggested) |
| `net/minecraft/server/level/ServerLevel#updateNeighborsAt` | horizon_world::level::Behaviours::update_neighbors_at (suggested) |
| `net/minecraft/server/level/ServerPlayer#triggerRecipeCrafted` | horizon_client::game::advancements::Game<'_>::trigger_recipe_crafted (suggested) |
| `net/minecraft/util/ARGB#colorFromFloat` | horizon_client_render::hud_layout::color_from_float (suggested) |
| `net/minecraft/util/Mth#atan2` | horizon_core::geometry::trig::atan2 (suggested) |
| `net/minecraft/util/Mth#clamp` | horizon_client_render::chat::clamp (suggested) |
| `net/minecraft/util/Mth#lerp` | horizon_world::stepped_interpolation::lerp (suggested) |
| `net/minecraft/util/Mth#rotLerp` | horizon_world::stepped_interpolation::rot_lerp (suggested) |
| `net/minecraft/util/Mth#wrapDegrees` | horizon_world::stepped_interpolation::wrap_degrees (suggested) |
| `net/minecraft/world/Containers#dropItemStack` | horizon_world::level::removal::drop_item_stack (suggested) |
| `net/minecraft/world/entity/LivingEntity#swing` | horizon_client::mobs::ai::Mob::swing (suggested) |
| `net/minecraft/world/entity/player/Inventory#add` | horizon_client::cauldron::add (suggested) |
| `net/minecraft/world/entity/player/Inventory#contains` | horizon_client::cauldron::contains (suggested) |
| `net/minecraft/world/inventory/Slot#setByPlayer` | horizon_client::menu::Menu::set_by_player (suggested) |
| `net/minecraft/world/item/ItemDisplayContext#leftHand` | horizon_assets::item_model::DisplayContext::left_hand (suggested) |
| `net/minecraft/world/item/ItemStack#enchant` | horizon_items::item_data::ItemStack::enchant (suggested) |
| `net/minecraft/world/item/ItemStack#getCount` | horizon_world::loot::LootStack::get_count (suggested) |
| `net/minecraft/world/item/ItemStack#hasFoil` | horizon_items::item_data::ItemStack::has_foil (suggested) |
| `net/minecraft/world/item/ItemStack#isSameItemSameComponents` | horizon_items::item_data::ItemStack::same_item_same_components (suggested) |
| `net/minecraft/world/item/ItemStack#matches` | horizon_client_render::vault::matches (suggested) |
| `net/minecraft/world/item/ItemStack#shrink` | horizon_client::game::dispenser::shrink (suggested) |
| `net/minecraft/world/item/crafting/Recipe#assemble` | horizon_client::crafting::Special::assemble (suggested) |
| `net/minecraft/world/item/crafting/Recipe#matches` | horizon_client::crafting::Special::matches (suggested) |
| `net/minecraft/world/level/Level#isClientSide` | horizon_world::level::is_client_side (suggested) |
| `net/minecraft/world/level/Level#noCollision` | horizon_client::mobs::shulker::no_collision (suggested) |
| `net/minecraft/world/level/block/state/BlockState#getCollisionShape` | horizon_client_render::collisions::<SceneCollisions<'_, '_> as PistonCollisionWorld>::collision_shape (suggested) |
| `net/minecraft/world/phys/Vec3#directionFromRotation` | horizon_client::auto_jump::direction_from_rotation (suggested) |
| `net/minecraft/world/phys/Vec3#normalize` | horizon_client::mobs::creaking::normalize (suggested) |
| `net/minecraft/world/phys/Vec3#yRot` | horizon_client::mobs::raider::y_rot (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/mojang/blaze3d/ProjectionType#ORTHOGRAPHIC` | field | 1 |
| `com/mojang/blaze3d/platform/FramerateLimitTracker#onInputReceived` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants#getKey` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants#grabMouse` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants#releaseMouse` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants$Key#getType` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants$Key#getValue` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants$Type#KEYBOARD` | field | 3 |
| `com/mojang/blaze3d/platform/Lighting#setupFor` | method | 1 |
| `com/mojang/blaze3d/platform/Lighting$Entry#ITEMS_3D` | field | 1 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledHeight` | method | 11 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledWidth` | method | 11 |
| `com/mojang/blaze3d/platform/Window#getScreenHeight` | method | 2 |
| `com/mojang/blaze3d/platform/Window#getScreenWidth` | method | 2 |
| `com/mojang/blaze3d/systems/RenderSystem#backupProjectionMatrix` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#bindDefaultUniforms` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#getDevice` | method | 2 |
| `com/mojang/blaze3d/systems/RenderSystem#getModelViewStack` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#getSamplerCache` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#getShaderFog` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#restoreProjectionMatrix` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#setProjectionMatrix` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#setShaderFog` | method | 1 |
| `com/mojang/blaze3d/systems/SamplerCache#getClampToEdge` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#<init>` | method | 5 |
| `com/mojang/blaze3d/vertex/PoseStack#last` | method | 22 |
| `com/mojang/blaze3d/vertex/PoseStack#mulPose` | method | 15 |
| `com/mojang/blaze3d/vertex/PoseStack#popPose` | method | 22 |
| `com/mojang/blaze3d/vertex/PoseStack#pushPose` | method | 22 |
| `com/mojang/blaze3d/vertex/PoseStack#rotate` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#scale` | method | 16 |
| `com/mojang/blaze3d/vertex/PoseStack#translate` | method | 21 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#copy` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#mulPose` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#pose` | method | 13 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#set` | method | 1 |
| `com/mojang/blaze3d/vertex/QuadInstance#<init>` | method | 1 |
| `com/mojang/blaze3d/vertex/QuadInstance#setColor` | method | 1 |
| `com/mojang/blaze3d/vertex/QuadInstance#setLightCoords` | method | 1 |
| `com/mojang/blaze3d/vertex/QuadInstance#setOverlayCoords` | method | 1 |
| `com/mojang/blaze3d/vertex/SheetedDecalTextureGenerator#<init>` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#addVertex` | method | 19 |
| `com/mojang/blaze3d/vertex/VertexConsumer#putBakedQuad` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#putBakedQuadWithGlint` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setColor` | method | 20 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setLight` | method | 10 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setLineWidth` | method | 2 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setNormal` | method | 10 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setOverlay` | method | 8 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setUv` | method | 11 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setUv1` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setUv2` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setUv3` | method | 2 |
| `net/minecraft/ChatFormatting#AQUA` | field | 2 |
| `net/minecraft/ChatFormatting#BOLD` | field | 1 |
| `net/minecraft/ChatFormatting#DARK_GRAY` | field | 5 |
| `net/minecraft/ChatFormatting#DARK_RED` | field | 1 |
| `net/minecraft/ChatFormatting#GOLD` | field | 12 |
| `net/minecraft/ChatFormatting#GRAY` | field | 19 |
| `net/minecraft/ChatFormatting#GREEN` | field | 3 |
| `net/minecraft/ChatFormatting#LIGHT_PURPLE` | field | 3 |
| `net/minecraft/ChatFormatting#RED` | field | 15 |
| `net/minecraft/ChatFormatting#WHITE` | field | 6 |
| `net/minecraft/ChatFormatting#YELLOW` | field | 2 |
| `net/minecraft/client/Camera#forwardVector` | method | 4 |
| `net/minecraft/client/Camera#leftVector` | method | 4 |
| `net/minecraft/client/Camera#position` | method | 26 |
| `net/minecraft/client/Camera#rotation` | method | 5 |
| `net/minecraft/client/Camera#upVector` | method | 4 |
| `net/minecraft/client/Camera#xRot` | method | 2 |
| `net/minecraft/client/Camera#yRot` | method | 12 |
| `net/minecraft/client/CameraType#FIRST_PERSON` | field | 3 |
| `net/minecraft/client/CameraType#THIRD_PERSON_BACK` | field | 2 |
| `net/minecraft/client/CameraType#THIRD_PERSON_FRONT` | field | 1 |
| `net/minecraft/client/CameraType#isFirstPerson` | method | 3 |
| `net/minecraft/client/ClientRecipeBook#getCollections` | method | 1 |
| `net/minecraft/client/DeltaTracker#getGameTimeDeltaPartialTick` | method | 3 |
| `net/minecraft/client/KeyMapping#getTranslatedKeyMessage` | method | 1 |
| `net/minecraft/client/KeyMapping#matches` | method | 2 |
| `net/minecraft/client/KeyMapping$Category#register` | method | 1 |
| `net/minecraft/client/Minecraft#createWorldOpenFlows` | method | 1 |
| `net/minecraft/client/Minecraft#execute` | method | 2 |
| `net/minecraft/client/Minecraft#gameDirectory` | field | 2 |
| `net/minecraft/client/Minecraft#gameMode` | field | 18 |
| `net/minecraft/client/Minecraft#gameRenderer` | field | 17 |
| `net/minecraft/client/Minecraft#getAtlasManager` | method | 5 |
| `net/minecraft/client/Minecraft#getCameraEntity` | method | 1 |
| `net/minecraft/client/Minecraft#getConnection` | method | 4 |
| `net/minecraft/client/Minecraft#getDeltaTracker` | method | 3 |
| `net/minecraft/client/Minecraft#getEntityModels` | method | 1 |
| `net/minecraft/client/Minecraft#getFps` | method | 1 |
| `net/minecraft/client/Minecraft#getFramerateLimitTracker` | method | 1 |
| `net/minecraft/client/Minecraft#getItemModelResolver` | method | 2 |
| `net/minecraft/client/Minecraft#getLevelSource` | method | 1 |
| `net/minecraft/client/Minecraft#getMapRenderer` | method | 1 |
| `net/minecraft/client/Minecraft#getModelManager` | method | 1 |
| `net/minecraft/client/Minecraft#getSingleplayerServer` | method | 1 |
| `net/minecraft/client/Minecraft#getTextureManager` | method | 1 |
| `net/minecraft/client/Minecraft#getWindow` | method | 12 |
| `net/minecraft/client/Minecraft#gui` | field | 11 |
| `net/minecraft/client/Minecraft#hasShiftDown` | method | 2 |
| `net/minecraft/client/Minecraft#hitResult` | field | 3 |
| `net/minecraft/client/Minecraft#isGameLoadFinished` | method | 1 |
| `net/minecraft/client/Minecraft#isPaused` | method | 1 |
| `net/minecraft/client/Minecraft#isWindowActive` | method | 1 |
| `net/minecraft/client/Minecraft#level` | field | 35 |
| `net/minecraft/client/Minecraft#levelExtractor` | field | 2 |
| `net/minecraft/client/Minecraft#mouseHandler` | field | 4 |
| `net/minecraft/client/Minecraft#options` | field | 5 |
| `net/minecraft/client/Minecraft#renderFrame` | method | 1 |
| `net/minecraft/client/Minecraft#resizeGui` | method | 1 |
| `net/minecraft/client/Minecraft#schedule` | method | 1 |
| `net/minecraft/client/Minecraft#stop` | method | 2 |
| `net/minecraft/client/MouseHandler#onMove` | method | 1 |
| `net/minecraft/client/MouseHandler#turnPlayer` | method | 1 |
| `net/minecraft/client/MouseHandler#xpos` | method | 5 |
| `net/minecraft/client/MouseHandler#ypos` | method | 5 |
| `net/minecraft/client/OptionInstance#get` | method | 2 |
| `net/minecraft/client/Options#getCameraType` | method | 3 |
| `net/minecraft/client/Options#guiScale` | method | 1 |
| `net/minecraft/client/Options#keyDown` | field | 1 |
| `net/minecraft/client/Options#keyDrop` | field | 1 |
| `net/minecraft/client/Options#keyHotbarSlots` | field | 1 |
| `net/minecraft/client/Options#keyInventory` | field | 1 |
| `net/minecraft/client/Options#keyJump` | field | 1 |
| `net/minecraft/client/Options#keyLeft` | field | 1 |
| `net/minecraft/client/Options#keyRight` | field | 1 |
| `net/minecraft/client/Options#keyUp` | field | 1 |
| `net/minecraft/client/Options#onboardAccessibility` | field | 1 |
| `net/minecraft/client/Options#pauseOnLostFocus` | field | 1 |
| `net/minecraft/client/Options#save` | method | 1 |
| `net/minecraft/client/Options#sensitivity` | method | 1 |
| `net/minecraft/client/Options#setCameraType` | method | 3 |
| `net/minecraft/client/Screenshot#grab` | method | 2 |
| `net/minecraft/client/gui/ActiveTextCollector#acceptScrollingWithDefaultCenter` | method | 1 |
| `net/minecraft/client/gui/Font$DisplayMode#NORMAL` | field | 4 |
| `net/minecraft/client/gui/Gui#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/Gui#hud` | field | 1 |
| `net/minecraft/client/gui/Gui#screen` | method | 8 |
| `net/minecraft/client/gui/Gui#setScreen` | method | 7 |
| `net/minecraft/client/gui/Gui#toastManager` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#centeredText` | method | 9 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#disableScissor` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#enableScissor` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fill` | method | 15 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#guiHeight` | method | 4 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#guiRenderState` | field | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#guiWidth` | method | 4 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#item` | method | 5 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#nextStratum` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#outline` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#pose` | method | 9 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#scissorStack` | field | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#setTooltipForNextFrame` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#text` | method | 24 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#textRendererForWidget` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#textWithWordWrap` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor$HoveredTextEffects#NONE` | field | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor$ScissorStack#peek` | method | 1 |
| `net/minecraft/client/gui/Hud#extractCameraOverlays` | method | 1 |
| `net/minecraft/client/gui/Hud#extractCrosshair` | method | 1 |
| `net/minecraft/client/gui/Hud#extractDemoOverlay` | method | 1 |
| `net/minecraft/client/gui/Hud#extractEffects` | method | 1 |
| `net/minecraft/client/gui/Hud#extractHotbarAndDecorations` | method | 1 |
| `net/minecraft/client/gui/Hud#extractOverlayMessage` | method | 1 |
| `net/minecraft/client/gui/Hud#extractScoreboardSidebar` | method | 1 |
| `net/minecraft/client/gui/Hud#extractSelectedItemName` | method | 1 |
| `net/minecraft/client/gui/Hud#extractSleepOverlay` | method | 1 |
| `net/minecraft/client/gui/Hud#extractTitle` | method | 1 |
| `net/minecraft/client/gui/Hud#getMobEffectSprite` | method | 1 |
| `net/minecraft/client/gui/Hud#isHidden` | method | 1 |
| `net/minecraft/client/gui/Hud#toggle` | method | 1 |
| `net/minecraft/client/gui/components/AbstractButton#getMessage` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSliderButton#<init>` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#active` | field | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#getHeight` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#getMessage` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#getWidth` | method | 2 |
| `net/minecraft/client/gui/components/AbstractWidget#getX` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#getY` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#isHovered` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#setPosition` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#visible` | field | 1 |
| `net/minecraft/client/gui/components/BossHealthOverlay#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/components/Button#active` | field | 2 |
| `net/minecraft/client/gui/components/Button#builder` | method | 3 |
| `net/minecraft/client/gui/components/Button#setMessage` | method | 1 |
| `net/minecraft/client/gui/components/Button#setPosition` | method | 1 |
| `net/minecraft/client/gui/components/Button#setTooltip` | method | 1 |
| `net/minecraft/client/gui/components/Button$Builder#bounds` | method | 2 |
| `net/minecraft/client/gui/components/Button$Builder#build` | method | 3 |
| `net/minecraft/client/gui/components/Button$Builder#size` | method | 1 |
| `net/minecraft/client/gui/components/Button$Builder#tooltip` | method | 1 |
| `net/minecraft/client/gui/components/Button$Builder#width` | method | 1 |
| `net/minecraft/client/gui/components/Button$Plain#<init>` | method | 2 |
| `net/minecraft/client/gui/components/ContainerObjectSelectionList#<init>` | method | 1 |
| `net/minecraft/client/gui/components/ContainerObjectSelectionList$Entry#<init>` | method | 1 |
| `net/minecraft/client/gui/components/CycleButton#builder` | method | 3 |
| `net/minecraft/client/gui/components/CycleButton#getValue` | method | 1 |
| `net/minecraft/client/gui/components/CycleButton#onPress` | method | 1 |
| `net/minecraft/client/gui/components/CycleButton$Builder#create` | method | 3 |
| `net/minecraft/client/gui/components/CycleButton$Builder#displayOnlyValue` | method | 3 |
| `net/minecraft/client/gui/components/CycleButton$Builder#withValues` | method | 3 |
| `net/minecraft/client/gui/components/EditBox#<init>` | method | 4 |
| `net/minecraft/client/gui/components/EditBox#charTyped` | method | 3 |
| `net/minecraft/client/gui/components/EditBox#extractRenderState` | method | 3 |
| `net/minecraft/client/gui/components/EditBox#getValue` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#isFocused` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#mouseClicked` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#moveCursorToEnd` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setAlpha` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#setBordered` | method | 3 |
| `net/minecraft/client/gui/components/EditBox#setFocused` | method | 3 |
| `net/minecraft/client/gui/components/EditBox#setHighlightPos` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setHint` | method | 3 |
| `net/minecraft/client/gui/components/EditBox#setMaxLength` | method | 3 |
| `net/minecraft/client/gui/components/EditBox#setResponder` | method | 3 |
| `net/minecraft/client/gui/components/EditBox#setTextColor` | method | 3 |
| `net/minecraft/client/gui/components/EditBox#setValue` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#setWidth` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setX` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#setY` | method | 2 |
| `net/minecraft/client/gui/components/MultiLineLabel#EMPTY` | field | 1 |
| `net/minecraft/client/gui/components/MultiLineLabel#create` | method | 1 |
| `net/minecraft/client/gui/components/MultiLineLabel#getLineCount` | method | 1 |
| `net/minecraft/client/gui/components/PlainTextButton#<init>` | method | 1 |
| `net/minecraft/client/gui/components/PlainTextButton#setTooltip` | method | 1 |
| `net/minecraft/client/gui/components/Tooltip#create` | method | 3 |
| `net/minecraft/client/gui/components/events/ContainerEventHandler#children` | method | 1 |
| `net/minecraft/client/gui/components/events/GuiEventListener#getClass` | method | 1 |
| `net/minecraft/client/gui/components/toasts/ToastManager#clear` | method | 1 |
| `net/minecraft/client/gui/render/pip/PictureInPictureRenderer#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/ConnectScreen#startConnecting` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#<init>` | method | 4 |
| `net/minecraft/client/gui/screens/Screen#addRenderableWidget` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#charTyped` | method | 2 |
| `net/minecraft/client/gui/screens/Screen#children` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#extractRenderState` | method | 3 |
| `net/minecraft/client/gui/screens/Screen#getTooltipFromItem` | method | 16 |
| `net/minecraft/client/gui/screens/Screen#height` | field | 1 |
| `net/minecraft/client/gui/screens/Screen#init` | method | 2 |
| `net/minecraft/client/gui/screens/Screen#isPauseScreen` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#keyPressed` | method | 2 |
| `net/minecraft/client/gui/screens/Screen#keyReleased` | method | 2 |
| `net/minecraft/client/gui/screens/Screen#mouseClicked` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#mouseDragged` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#mouseReleased` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#tick` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#width` | field | 1 |
| `net/minecraft/client/gui/screens/TitleScreen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/BookViewScreen$BookAccess#fromItem` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/BookViewScreen$BookAccess#pages` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/EnchantmentNames#getInstance` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/EnchantmentNames#getRandomName` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/EnchantmentNames#initSeed` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/MenuAccess#getMenu` | method | 2 |
| `net/minecraft/client/gui/screens/multiplayer/JoinMultiplayerScreen#getTitle` | method | 1 |
| `net/minecraft/client/gui/screens/multiplayer/JoinMultiplayerScreen#width` | field | 1 |
| `net/minecraft/client/gui/screens/recipebook/RecipeCollection#getRecipes` | method | 1 |
| `net/minecraft/client/gui/screens/worldselection/WorldOpenFlows#createFreshLevel` | method | 1 |
| `net/minecraft/client/gui/screens/worldselection/WorldOpenFlows#openWorld` | method | 1 |
| `net/minecraft/client/input/CharacterEvent#<init>` | method | 4 |
| `net/minecraft/client/input/CharacterEvent#codepoint` | method | 2 |
| `net/minecraft/client/input/KeyEvent#<init>` | method | 6 |
| `net/minecraft/client/input/KeyEvent#key` | method | 2 |
| `net/minecraft/client/input/KeyEvent#keycode` | method | 2 |
| `net/minecraft/client/input/KeyEvent#modifiers` | method | 1 |
| `net/minecraft/client/input/MouseButtonEvent#<init>` | method | 3 |
| `net/minecraft/client/input/MouseButtonEvent#buttonInfo` | method | 1 |
| `net/minecraft/client/input/MouseButtonEvent#x` | method | 1 |
| `net/minecraft/client/input/MouseButtonEvent#y` | method | 1 |
| `net/minecraft/client/input/MouseButtonInfo#<init>` | method | 3 |
| `net/minecraft/client/input/MouseButtonInfo#button` | method | 1 |
| `net/minecraft/client/model/HumanoidModel#rightLeg` | field | 1 |
| `net/minecraft/client/model/HumanoidModel#setupAnim` | method | 2 |
| `net/minecraft/client/model/Model#<init>` | method | 1 |
| `net/minecraft/client/model/Model#renderToBuffer` | method | 1 |
| `net/minecraft/client/model/Model#setupAnim` | method | 1 |
| `net/minecraft/client/model/Model$Simple#<init>` | method | 1 |
| `net/minecraft/client/model/geom/EntityModelSet#bakeLayer` | method | 1 |
| `net/minecraft/client/model/geom/ModelLayers#STANDING_BANNER_FLAG` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#<init>` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#getChild` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#translateAndRotate` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#visible` | field | 2 |
| `net/minecraft/client/model/geom/ModelPart#xRot` | field | 3 |
| `net/minecraft/client/model/geom/ModelPart#y` | field | 3 |
| `net/minecraft/client/model/geom/ModelPart#yRot` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#yScale` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#z` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart#zRot` | field | 1 |
| `net/minecraft/client/model/geom/ModelPart$Cube#<init>` | method | 3 |
| `net/minecraft/client/model/geom/ModelPart$Cube#compile` | method | 2 |
| `net/minecraft/client/model/player/PlayerModel#rightLeg` | field | 1 |
| `net/minecraft/client/model/player/PlayerModel#translateToHand` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#addEntity` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#addParticle` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#clip` | method | 3 |
| `net/minecraft/client/multiplayer/ClientLevel#disconnect` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getBlockEntity` | method | 7 |
| `net/minecraft/client/multiplayer/ClientLevel#getBlockState` | method | 21 |
| `net/minecraft/client/multiplayer/ClientLevel#getEntity` | method | 3 |
| `net/minecraft/client/multiplayer/ClientLevel#getGameTime` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getMapData` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#isLoaded` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#playLocalSound` | method | 3 |
| `net/minecraft/client/multiplayer/ClientLevel#recipeAccess` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#registryAccess` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#removeEntity` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#send` | method | 3 |
| `net/minecraft/client/multiplayer/MultiPlayerGameMode#handleContainerInput` | method | 14 |
| `net/minecraft/client/multiplayer/MultiPlayerGameMode#handleInventoryButtonClick` | method | 4 |
| `net/minecraft/client/multiplayer/MultiPlayerGameMode#handleSlotStateChanged` | method | 1 |
| `net/minecraft/client/multiplayer/MultiPlayerGameMode#useItemOn` | method | 2 |
| `net/minecraft/client/multiplayer/ServerData#<init>` | method | 1 |
| `net/minecraft/client/multiplayer/ServerData$Type#OTHER` | field | 1 |
| `net/minecraft/client/multiplayer/resolver/ServerAddress#parseString` | method | 1 |
| `net/minecraft/client/player/ClientInput#keyPresses` | field | 1 |
| `net/minecraft/client/player/ClientInput#moveVector` | field | 1 |
| `net/minecraft/client/player/KeyboardInput#tick` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#blockPosition` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#closeContainer` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#connection` | field | 1 |
| `net/minecraft/client/player/LocalPlayer#containerMenu` | field | 16 |
| `net/minecraft/client/player/LocalPlayer#distanceToSqr` | method | 4 |
| `net/minecraft/client/player/LocalPlayer#experienceLevel` | field | 1 |
| `net/minecraft/client/player/LocalPlayer#getAbilities` | method | 3 |
| `net/minecraft/client/player/LocalPlayer#getEyePosition` | method | 6 |
| `net/minecraft/client/player/LocalPlayer#getEyeY` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getId` | method | 4 |
| `net/minecraft/client/player/LocalPlayer#getInventory` | method | 10 |
| `net/minecraft/client/player/LocalPlayer#getMainHandItem` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#getPosition` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getRecipeBook` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getSkin` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#getSwingAnimation` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getUUID` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getX` | method | 18 |
| `net/minecraft/client/player/LocalPlayer#getXRot` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#getY` | method | 19 |
| `net/minecraft/client/player/LocalPlayer#getYRot` | method | 5 |
| `net/minecraft/client/player/LocalPlayer#getZ` | method | 18 |
| `net/minecraft/client/player/LocalPlayer#hurtTime` | field | 1 |
| `net/minecraft/client/player/LocalPlayer#inventoryMenu` | field | 4 |
| `net/minecraft/client/player/LocalPlayer#isAlive` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#isCrouching` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#isFallFlying` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#isInWater` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#isModelPartShown` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#isPassenger` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#isSpectator` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#isSwinging` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#level` | method | 4 |
| `net/minecraft/client/player/LocalPlayer#position` | method | 6 |
| `net/minecraft/client/player/LocalPlayer#setSprinting` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#setXRot` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#setYBodyRot` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#setYHeadRot` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#setYRot` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#tick` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#tickCount` | field | 3 |
| `net/minecraft/client/player/LocalPlayer#xRotO` | field | 1 |
| `net/minecraft/client/player/LocalPlayer#yRotO` | field | 1 |
| `net/minecraft/client/player/LocalPlayer#yo` | field | 1 |
| `net/minecraft/client/player/RemotePlayer#<init>` | method | 1 |
| `net/minecraft/client/player/RemotePlayer#setId` | method | 1 |
| `net/minecraft/client/player/RemotePlayer#setShiftKeyDown` | method | 1 |
| `net/minecraft/client/player/RemotePlayer#setYHeadRot` | method | 1 |
| `net/minecraft/client/player/RemotePlayer#snapTo` | method | 1 |
| `net/minecraft/client/player/RemotePlayer#yBodyRot` | field | 1 |
| `net/minecraft/client/player/RemotePlayer#yBodyRotO` | field | 1 |
| `net/minecraft/client/player/RemotePlayer#yHeadRotO` | field | 1 |
| `net/minecraft/client/renderer/GameRenderer#extract` | method | 2 |
| `net/minecraft/client/renderer/GameRenderer#featureRenderDispatcher` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#gameRenderState` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#lighting` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#mainCamera` | method | 13 |
| `net/minecraft/client/renderer/GameRenderer#mainRenderTarget` | method | 2 |
| `net/minecraft/client/renderer/GameRenderer#update` | method | 1 |
| `net/minecraft/client/renderer/LevelRenderer#submitFeatures` | method | 2 |
| `net/minecraft/client/renderer/MapRenderer#extractRenderState` | method | 1 |
| `net/minecraft/client/renderer/MapRenderer#render` | method | 1 |
| `net/minecraft/client/renderer/Projection#<init>` | method | 1 |
| `net/minecraft/client/renderer/Projection#setupOrtho` | method | 1 |
| `net/minecraft/client/renderer/ProjectionMatrixBuffer#<init>` | method | 1 |
| `net/minecraft/client/renderer/ProjectionMatrixBuffer#getBuffer` | method | 1 |
| `net/minecraft/client/renderer/Rect2i#getHeight` | method | 1 |
| `net/minecraft/client/renderer/Rect2i#getWidth` | method | 1 |
| `net/minecraft/client/renderer/RenderPipelines#DEBUG_FILLED_SNIPPET` | field | 1 |
| `net/minecraft/client/renderer/RenderPipelines#ENTITY_NO_LIGHTMAP_SNIPPET` | field | 1 |
| `net/minecraft/client/renderer/RenderPipelines#ENTITY_TRANSLUCENT` | field | 1 |
| `net/minecraft/client/renderer/RenderPipelines#EYES` | field | 1 |
| `net/minecraft/client/renderer/RenderPipelines#EYES_SNIPPET` | field | 1 |
| `net/minecraft/client/renderer/RenderPipelines#GUI_TEXTURED` | field | 1 |
| `net/minecraft/client/renderer/RenderPipelines#register` | method | 1 |
| `net/minecraft/client/renderer/ShaderDefines#<init>` | method | 1 |
| `net/minecraft/client/renderer/ShaderDefines#flags` | method | 1 |
| `net/minecraft/client/renderer/ShaderDefines#values` | method | 1 |
| `net/minecraft/client/renderer/Sheets#BANNER_BASE` | field | 2 |
| `net/minecraft/client/renderer/Sheets#BANNER_PATTERN_BASE` | field | 1 |
| `net/minecraft/client/renderer/Sheets#BANNER_SHEET` | field | 1 |
| `net/minecraft/client/renderer/Sheets#GUI_SHEET` | field | 1 |
| `net/minecraft/client/renderer/Sheets#getBannerSprite` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollection#allPhases` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollection#submitText` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitCustomGeometry` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitModel` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitModelPart` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitShapeOutline` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitText` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector$CustomGeometryRenderer#render` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeStorage#<init>` | method | 3 |
| `net/minecraft/client/renderer/SubmitNodeStorage#drainPhases` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeStorage#getSubmitsPerOrder` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeStorage#order` | method | 2 |
| `net/minecraft/client/renderer/block/BlockStateModelSet#get` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModel#collectParts` | method | 2 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModel#emitQuads` | method | 1 |
| `net/minecraft/client/renderer/block/dispatch/BlockStateModelPart#getQuads` | method | 1 |
| `net/minecraft/client/renderer/blockentity/BannerRenderer#submitPatterns` | method | 1 |
| `net/minecraft/client/renderer/blockentity/BlockEntityRenderer#extractRenderState` | method | 2 |
| `net/minecraft/client/renderer/blockentity/CampfireRenderer#submit` | method | 1 |
| `net/minecraft/client/renderer/blockentity/state/BlockEntityRenderState#<init>` | method | 2 |
| `net/minecraft/client/renderer/blockentity/state/BlockEntityRenderState#blockPos` | field | 1 |
| `net/minecraft/client/renderer/blockentity/state/BlockEntityRenderState#lightCoords` | field | 1 |
| `net/minecraft/client/renderer/blockentity/state/CampfireRenderState#blockPos` | field | 1 |
| `net/minecraft/client/renderer/blockentity/state/CampfireRenderState#facing` | field | 1 |
| `net/minecraft/client/renderer/blockentity/state/CampfireRenderState#items` | field | 1 |
| `net/minecraft/client/renderer/blockentity/state/CampfireRenderState#lightCoords` | field | 1 |
| `net/minecraft/client/renderer/blockentity/state/LecternRenderState#blockPos` | field | 2 |
| `net/minecraft/client/renderer/blockentity/state/LecternRenderState#yRot` | field | 1 |
| `net/minecraft/client/renderer/culling/Frustum#<init>` | method | 1 |
| `net/minecraft/client/renderer/culling/Frustum#isVisible` | method | 6 |
| `net/minecraft/client/renderer/culling/Frustum#prepare` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRendererProvider$Context#getEquipmentRenderer` | method | 1 |
| `net/minecraft/client/renderer/entity/ExperienceOrbRenderer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/LivingEntityRenderer#getOverlayCoords` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/EquipmentLayerRenderer#renderLayers` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/HumanoidArmorLayer#getArmorModel` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/ItemInHandLayer#submitArmWithItem` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/RenderLayer#<init>` | method | 2 |
| `net/minecraft/client/renderer/entity/player/AvatarRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/player/AvatarRenderer#addLayer` | method | 1 |
| `net/minecraft/client/renderer/entity/player/AvatarRenderer#getRenderOffset` | method | 1 |
| `net/minecraft/client/renderer/entity/state/AvatarRenderState#feetEquipment` | field | 1 |
| `net/minecraft/client/renderer/entity/state/AvatarRenderState#id` | field | 7 |
| `net/minecraft/client/renderer/entity/state/AvatarRenderState#isInvisible` | field | 1 |
| `net/minecraft/client/renderer/entity/state/AvatarRenderState#legsEquipment` | field | 1 |
| `net/minecraft/client/renderer/entity/state/AvatarRenderState#outlineColor` | field | 1 |
| `net/minecraft/client/renderer/entity/state/AvatarRenderState#showRightPants` | field | 1 |
| `net/minecraft/client/renderer/entity/state/AvatarRenderState#skin` | field | 1 |
| `net/minecraft/client/renderer/entity/state/ExperienceOrbRenderState#x` | field | 1 |
| `net/minecraft/client/renderer/entity/state/ExperienceOrbRenderState#y` | field | 1 |
| `net/minecraft/client/renderer/entity/state/ExperienceOrbRenderState#z` | field | 1 |
| `net/minecraft/client/renderer/entity/state/HumanoidRenderState#ageInTicks` | field | 1 |
| `net/minecraft/client/renderer/extract/LevelExtractor#setSectionDirty` | method | 2 |
| `net/minecraft/client/renderer/feature/CustomFeatureRenderer$Submit#customGeometryRenderer` | method | 1 |
| `net/minecraft/client/renderer/feature/CustomFeatureRenderer$Submit#pose` | method | 1 |
| `net/minecraft/client/renderer/feature/CustomFeatureRenderer$Submit#renderType` | method | 1 |
| `net/minecraft/client/renderer/feature/FeatureRenderDispatcher#prepareFrame` | method | 1 |
| `net/minecraft/client/renderer/feature/FeatureRenderDispatcher#renderAllFeatures` | method | 1 |
| `net/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame#close` | method | 1 |
| `net/minecraft/client/renderer/feature/ModelFeatureRenderer$Submit#lightCoords` | method | 1 |
| `net/minecraft/client/renderer/feature/ModelFeatureRenderer$Submit#model` | method | 1 |
| `net/minecraft/client/renderer/feature/ModelFeatureRenderer$Submit#overlayCoords` | method | 1 |
| `net/minecraft/client/renderer/feature/ModelFeatureRenderer$Submit#pose` | method | 1 |
| `net/minecraft/client/renderer/feature/ModelFeatureRenderer$Submit#renderType` | method | 1 |
| `net/minecraft/client/renderer/feature/ModelFeatureRenderer$Submit#sheetedDecalPose` | method | 1 |
| `net/minecraft/client/renderer/feature/ModelFeatureRenderer$Submit#state` | method | 1 |
| `net/minecraft/client/renderer/feature/ModelFeatureRenderer$Submit#tintedColor` | method | 1 |
| `net/minecraft/client/renderer/feature/ModelFeatureRenderer$Submit#uvMapping` | method | 1 |
| `net/minecraft/client/renderer/feature/phase/FeatureRenderPhase#sortInto` | method | 2 |
| `net/minecraft/client/renderer/feature/phase/FeatureRenderPhase#submit` | method | 1 |
| `net/minecraft/client/renderer/fog/FogRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/fog/FogRenderer#getBuffer` | method | 1 |
| `net/minecraft/client/renderer/fog/FogRenderer$FogMode#NONE` | field | 1 |
| `net/minecraft/client/renderer/item/ItemModelResolver#updateForTopItem` | method | 2 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#<init>` | method | 2 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#activeLayerCount` | field | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#displayContext` | field | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#firstLayer` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#isEmpty` | method | 2 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#layers` | field | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#usesBlockLight` | method | 2 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$FoilType#NONE` | field | 2 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$FoilType#SPECIAL` | field | 2 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$FoilType#STANDARD` | field | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$FoilType#ordinal` | method | 2 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$FoilType#values` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#argumentForSpecialRendering` | field | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#foilType` | field | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#itemTransform` | field | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#localTransform` | field | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#quads` | field | 2 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#specialRenderer` | field | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#tintLayers` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup#builder` | method | 3 |
| `net/minecraft/client/renderer/rendertype/RenderSetup#layeringTransform` | field | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup#textureTransform` | field | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup#textures` | field | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup#useLightmap` | field | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup#useOverlay` | field | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#createRenderSetup` | method | 3 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#setLayeringTransform` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#setTextureTransform` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#sortOnUpload` | method | 3 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#useLightmap` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#useOverlay` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#withTexture` | method | 3 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$TextureBinding#location` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$TextureBinding#sampler` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderType#create` | method | 3 |
| `net/minecraft/client/renderer/rendertype/RenderType#format` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderType#pipeline` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderType#state` | field | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entitySolid` | method | 6 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityTranslucent` | method | 6 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#lightning` | method | 2 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#lines` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#text` | method | 2 |
| `net/minecraft/client/renderer/state/GameRenderState#levelRenderState` | field | 1 |
| `net/minecraft/client/renderer/state/MapRenderState#<init>` | method | 1 |
| `net/minecraft/client/renderer/state/gui/GuiRenderState#addPicturesInPictureState` | method | 1 |
| `net/minecraft/client/renderer/state/gui/pip/PictureInPictureRenderState#getBounds` | method | 1 |
| `net/minecraft/client/renderer/state/level/CameraRenderState#projectionMatrix` | field | 1 |
| `net/minecraft/client/renderer/state/level/CameraRenderState#viewRotationMatrix` | field | 1 |
| `net/minecraft/client/renderer/state/level/LevelRenderState#blockOutlineRenderState` | field | 1 |
| `net/minecraft/client/renderer/state/level/LevelRenderState#cameraRenderState` | field | 1 |
| `net/minecraft/client/renderer/texture/AbstractTexture#<init>` | method | 1 |
| `net/minecraft/client/renderer/texture/OverlayTexture#NO_OVERLAY` | field | 15 |
| `net/minecraft/client/renderer/texture/SpriteContents#name` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlas#LOCATION_BLOCKS` | field | 2 |
| `net/minecraft/client/renderer/texture/TextureAtlas#getSprite` | method | 2 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#atlasLocation` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#contents` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU` | method | 4 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU0` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU1` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV` | method | 4 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV0` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV1` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureManager#register` | method | 1 |
| `net/minecraft/client/renderer/texture/UvMapping#wrap` | method | 1 |
| `net/minecraft/client/resources/model/EquipmentClientInfo$LayerType#HUMANOID` | field | 1 |
| `net/minecraft/client/resources/model/EquipmentClientInfo$LayerType#HUMANOID_LEGGINGS` | field | 1 |
| `net/minecraft/client/resources/model/ModelManager#getBlockStateModelSet` | method | 1 |
| `net/minecraft/client/resources/model/cuboid/ItemTransform#apply` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#materialInfo` | method | 2 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#isTinted` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#itemGlintRenderType` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#itemGlintSpecialRenderType` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#itemRenderType` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#sprite` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#tintIndex` | method | 1 |
| `net/minecraft/client/resources/model/geometry/ItemQuads#all` | method | 2 |
| `net/minecraft/client/resources/model/sprite/AtlasManager#get` | method | 2 |
| `net/minecraft/client/resources/model/sprite/AtlasManager#getAtlasOrThrow` | method | 2 |
| `net/minecraft/client/resources/model/sprite/SpriteId#<init>` | method | 1 |
| `net/minecraft/client/server/IntegratedServer#execute` | method | 1 |
| `net/minecraft/client/server/IntegratedServer#getPlayerList` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#withPermission` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#withSuppressedOutput` | method | 1 |
| `net/minecraft/commands/Commands#performPrefixedCommand` | method | 1 |
| `net/minecraft/core/BlockPos#<init>` | method | 2 |
| `net/minecraft/core/BlockPos#STREAM_CODEC` | field | 7 |
| `net/minecraft/core/BlockPos#ZERO` | field | 2 |
| `net/minecraft/core/BlockPos#above` | method | 8 |
| `net/minecraft/core/BlockPos#asLong` | method | 4 |
| `net/minecraft/core/BlockPos#below` | method | 2 |
| `net/minecraft/core/BlockPos#betweenClosed` | method | 1 |
| `net/minecraft/core/BlockPos#containing` | method | 6 |
| `net/minecraft/core/BlockPos#distToCenterSqr` | method | 4 |
| `net/minecraft/core/BlockPos#east` | method | 1 |
| `net/minecraft/core/BlockPos#equals` | method | 19 |
| `net/minecraft/core/BlockPos#getX` | method | 30 |
| `net/minecraft/core/BlockPos#getY` | method | 21 |
| `net/minecraft/core/BlockPos#getZ` | method | 30 |
| `net/minecraft/core/BlockPos#hashCode` | method | 1 |
| `net/minecraft/core/BlockPos#immutable` | method | 14 |
| `net/minecraft/core/BlockPos#north` | method | 1 |
| `net/minecraft/core/BlockPos#of` | method | 2 |
| `net/minecraft/core/BlockPos#offset` | method | 4 |
| `net/minecraft/core/BlockPos#relative` | method | 6 |
| `net/minecraft/core/BlockPos#south` | method | 2 |
| `net/minecraft/core/BlockPos#toShortString` | method | 3 |
| `net/minecraft/core/BlockPos#west` | method | 1 |
| `net/minecraft/core/ClientAsset$Texture#texturePath` | method | 3 |
| `net/minecraft/core/DefaultedRegistry#getKey` | method | 3 |
| `net/minecraft/core/DefaultedRegistry#iterator` | method | 1 |
| `net/minecraft/core/Direction#DOWN` | field | 4 |
| `net/minecraft/core/Direction#EAST` | field | 5 |
| `net/minecraft/core/Direction#NORTH` | field | 8 |
| `net/minecraft/core/Direction#SOUTH` | field | 7 |
| `net/minecraft/core/Direction#UP` | field | 11 |
| `net/minecraft/core/Direction#WEST` | field | 4 |
| `net/minecraft/core/Direction#from2DDataValue` | method | 7 |
| `net/minecraft/core/Direction#get2DDataValue` | method | 11 |
| `net/minecraft/core/Direction#getApproximateNearest` | method | 2 |
| `net/minecraft/core/Direction#getAxis` | method | 1 |
| `net/minecraft/core/Direction#getClockWise` | method | 9 |
| `net/minecraft/core/Direction#getCounterClockWise` | method | 3 |
| `net/minecraft/core/Direction#getName` | method | 1 |
| `net/minecraft/core/Direction#getStepX` | method | 16 |
| `net/minecraft/core/Direction#getStepZ` | method | 16 |
| `net/minecraft/core/Direction#getUnitVec3i` | method | 12 |
| `net/minecraft/core/Direction#toYRot` | method | 10 |
| `net/minecraft/core/Direction$Axis#Y` | field | 1 |
| `net/minecraft/core/Direction$Axis#isHorizontal` | method | 1 |
| `net/minecraft/core/FrontAndTop#SOUTH_UP` | field | 1 |
| `net/minecraft/core/FrontAndTop#UP_SOUTH` | field | 1 |
| `net/minecraft/core/FrontAndTop#front` | method | 1 |
| `net/minecraft/core/FrontAndTop#top` | method | 2 |
| `net/minecraft/core/Holder#equals` | method | 7 |
| `net/minecraft/core/Holder#getRegisteredName` | method | 1 |
| `net/minecraft/core/Holder#is` | method | 1 |
| `net/minecraft/core/Holder#value` | method | 8 |
| `net/minecraft/core/Holder$Reference#areComponentsBound` | method | 1 |
| `net/minecraft/core/Holder$Reference#value` | method | 3 |
| `net/minecraft/core/HolderLookup$Provider#lookupOrThrow` | method | 1 |
| `net/minecraft/core/HolderLookup$RegistryLookup#getOrThrow` | method | 1 |
| `net/minecraft/core/NonNullList#get` | method | 7 |
| `net/minecraft/core/NonNullList#iterator` | method | 7 |
| `net/minecraft/core/NonNullList#set` | method | 3 |
| `net/minecraft/core/NonNullList#size` | method | 4 |
| `net/minecraft/core/NonNullList#stream` | method | 1 |
| `net/minecraft/core/NonNullList#withSize` | method | 1 |
| `net/minecraft/core/Registry#get` | method | 1 |
| `net/minecraft/core/Registry#getKey` | method | 2 |
| `net/minecraft/core/Registry#getOrThrow` | method | 1 |
| `net/minecraft/core/RegistryAccess#lookupOrThrow` | method | 2 |
| `net/minecraft/core/SectionPos#blockToSectionCoord` | method | 1 |
| `net/minecraft/core/SectionPos#of` | method | 1 |
| `net/minecraft/core/SectionPos#x` | method | 1 |
| `net/minecraft/core/SectionPos#y` | method | 1 |
| `net/minecraft/core/SectionPos#z` | method | 1 |
| `net/minecraft/core/component/DataComponents#BANNER_PATTERNS` | field | 2 |
| `net/minecraft/core/component/DataComponents#COOKING_FUEL` | field | 2 |
| `net/minecraft/core/component/DataComponents#CUSTOM_NAME` | field | 2 |
| `net/minecraft/core/component/DataComponents#DYE` | field | 1 |
| `net/minecraft/core/component/DataComponents#ENCHANTMENT_GLINT_OVERRIDE` | field | 1 |
| `net/minecraft/core/component/DataComponents#EQUIPPABLE` | field | 1 |
| `net/minecraft/core/component/DataComponents#ITEM_MODEL` | field | 1 |
| `net/minecraft/core/component/DataComponents#JUKEBOX_PLAYABLE` | field | 2 |
| `net/minecraft/core/component/DataComponents#MAP_ID` | field | 1 |
| `net/minecraft/core/component/DataComponents#POTION_CONTENTS` | field | 2 |
| `net/minecraft/core/component/DataComponents#STORED_ENCHANTMENTS` | field | 1 |
| `net/minecraft/core/component/DataComponents#TRIM` | field | 2 |
| `net/minecraft/core/component/DataComponents#WRITABLE_BOOK_CONTENT` | field | 2 |
| `net/minecraft/core/component/DataComponents#WRITTEN_BOOK_CONTENT` | field | 2 |
| `net/minecraft/core/particles/ItemParticleOption#<init>` | method | 1 |
| `net/minecraft/core/particles/ParticleTypes#ENCHANT` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#ITEM` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#REVERSE_PORTAL` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#RECIPE_SERIALIZER` | field | 2 |
| `net/minecraft/core/registries/BuiltInRegistries#VILLAGER_PROFESSION` | field | 1 |
| `net/minecraft/core/registries/Registries#ENCHANTMENT` | field | 2 |
| `net/minecraft/core/registries/Registries#WORLD_PRESET` | field | 1 |
| `net/minecraft/data/AtlasIds#BLOCKS` | field | 1 |
| `net/minecraft/data/AtlasIds#GUI` | field | 1 |
| `net/minecraft/locale/Language#getInstance` | method | 2 |
| `net/minecraft/locale/Language#getOrDefault` | method | 1 |
| `net/minecraft/locale/Language#getVisualOrder` | method | 1 |
| `net/minecraft/nbt/CompoundTag#<init>` | method | 1 |
| `net/minecraft/nbt/CompoundTag#CODEC` | field | 1 |
| `net/minecraft/nbt/CompoundTag#getByteArray` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getListOrEmpty` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getLongOr` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getStringOr` | method | 1 |
| `net/minecraft/nbt/CompoundTag#put` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putByteArray` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putLong` | method | 1 |
| `net/minecraft/nbt/ListTag#<init>` | method | 1 |
| `net/minecraft/nbt/ListTag#add` | method | 1 |
| `net/minecraft/nbt/ListTag#getCompoundOrEmpty` | method | 1 |
| `net/minecraft/nbt/ListTag#size` | method | 1 |
| `net/minecraft/network/FriendlyByteBuf#readBlockPos` | method | 3 |
| `net/minecraft/network/FriendlyByteBuf#readBoolean` | method | 2 |
| `net/minecraft/network/FriendlyByteBuf#readByte` | method | 1 |
| `net/minecraft/network/FriendlyByteBuf#readEnum` | method | 1 |
| `net/minecraft/network/FriendlyByteBuf#readVarInt` | method | 3 |
| `net/minecraft/network/FriendlyByteBuf#writeBlockPos` | method | 3 |
| `net/minecraft/network/FriendlyByteBuf#writeBoolean` | method | 2 |
| `net/minecraft/network/FriendlyByteBuf#writeByte` | method | 1 |
| `net/minecraft/network/FriendlyByteBuf#writeEnum` | method | 1 |
| `net/minecraft/network/FriendlyByteBuf#writeVarInt` | method | 3 |
| `net/minecraft/network/RegistryFriendlyByteBuf#<init>` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readBlockPos` | method | 2 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readEnum` | method | 3 |
| `net/minecraft/network/RegistryFriendlyByteBuf#readVarInt` | method | 2 |
| `net/minecraft/network/RegistryFriendlyByteBuf#release` | method | 1 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeBlockPos` | method | 2 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeEnum` | method | 3 |
| `net/minecraft/network/RegistryFriendlyByteBuf#writeVarInt` | method | 2 |
| `net/minecraft/network/chat/CommonComponents#EMPTY` | field | 1 |
| `net/minecraft/network/chat/CommonComponents#GUI_CANCEL` | field | 1 |
| `net/minecraft/network/chat/CommonComponents#GUI_DONE` | field | 1 |
| `net/minecraft/network/chat/Component#copy` | method | 9 |
| `net/minecraft/network/chat/Component#empty` | method | 12 |
| `net/minecraft/network/chat/Component#getString` | method | 10 |
| `net/minecraft/network/chat/Component#getStyle` | method | 1 |
| `net/minecraft/network/chat/Component#getVisualOrderText` | method | 2 |
| `net/minecraft/network/chat/Component#literal` | method | 10 |
| `net/minecraft/network/chat/Component#translatable` | method | 48 |
| `net/minecraft/network/chat/MutableComponent#getString` | method | 2 |
| `net/minecraft/network/chat/MutableComponent#getVisualOrderText` | method | 2 |
| `net/minecraft/network/chat/MutableComponent#withColor` | method | 2 |
| `net/minecraft/network/chat/Style#getColor` | method | 1 |
| `net/minecraft/network/chat/Style#withColor` | method | 1 |
| `net/minecraft/network/chat/TextColor#getValue` | method | 1 |
| `net/minecraft/network/codec/ByteBufCodecs#BOOL` | field | 1 |
| `net/minecraft/network/codec/ByteBufCodecs#BYTE_ARRAY` | field | 1 |
| `net/minecraft/network/codec/ByteBufCodecs#VAR_INT` | field | 4 |
| `net/minecraft/network/codec/ByteBufCodecs#idMapper` | method | 1 |
| `net/minecraft/network/codec/StreamCodec#apply` | method | 1 |
| `net/minecraft/network/codec/StreamCodec#composite` | method | 8 |
| `net/minecraft/network/codec/StreamCodec#decode` | method | 3 |
| `net/minecraft/network/codec/StreamCodec#encode` | method | 3 |
| `net/minecraft/network/codec/StreamCodec#of` | method | 6 |
| `net/minecraft/network/codec/StreamCodec#unit` | method | 2 |
| `net/minecraft/network/protocol/common/custom/CustomPacketPayload#type` | method | 1 |
| `net/minecraft/network/protocol/common/custom/CustomPacketPayload$Type#<init>` | method | 15 |
| `net/minecraft/network/protocol/game/ServerboundRenameItemPacket#<init>` | method | 1 |
| `net/minecraft/network/protocol/game/ServerboundSetBeaconPacket#<init>` | method | 1 |
| `net/minecraft/network/protocol/game/ServerboundSetCarriedItemPacket#<init>` | method | 1 |
| `net/minecraft/resources/Identifier#equals` | method | 7 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 2 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 5 |
| `net/minecraft/resources/Identifier#getPath` | method | 2 |
| `net/minecraft/resources/Identifier#toString` | method | 3 |
| `net/minecraft/resources/Identifier#withDefaultNamespace` | method | 6 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 8 |
| `net/minecraft/resources/ResourceKey#toString` | method | 2 |
| `net/minecraft/server/MinecraftServer#getCommands` | method | 1 |
| `net/minecraft/server/MinecraftServer#getPlayerList` | method | 2 |
| `net/minecraft/server/MinecraftServer#getRecipeManager` | method | 2 |
| `net/minecraft/server/level/ServerLevel#dimension` | method | 2 |
| `net/minecraft/server/level/ServerLevel#enabledFeatures` | method | 1 |
| `net/minecraft/server/level/ServerLevel#gameEvent` | method | 3 |
| `net/minecraft/server/level/ServerLevel#getBlockEntity` | method | 6 |
| `net/minecraft/server/level/ServerLevel#getBlockState` | method | 8 |
| `net/minecraft/server/level/ServerLevel#getDataStorage` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getEntitiesOfClass` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getGameRules` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getGameTime` | method | 4 |
| `net/minecraft/server/level/ServerLevel#getRandom` | method | 2 |
| `net/minecraft/server/level/ServerLevel#getServer` | method | 2 |
| `net/minecraft/server/level/ServerLevel#isLoaded` | method | 2 |
| `net/minecraft/server/level/ServerLevel#playSound` | method | 3 |
| `net/minecraft/server/level/ServerLevel#players` | method | 3 |
| `net/minecraft/server/level/ServerLevel#recipeAccess` | method | 4 |
| `net/minecraft/server/level/ServerLevel#registryAccess` | method | 2 |
| `net/minecraft/server/level/ServerLevel#sendBlockUpdated` | method | 1 |
| `net/minecraft/server/level/ServerLevel#setBlockAndUpdate` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#awardRecipes` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#awardStat` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#blockPosition` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#containerMenu` | field | 7 |
| `net/minecraft/server/level/ServerPlayer#createCommandSourceStack` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#distanceToSqr` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#doCloseContainer` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#drop` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#experienceLevel` | field | 1 |
| `net/minecraft/server/level/ServerPlayer#experienceProgress` | field | 1 |
| `net/minecraft/server/level/ServerPlayer#getEyeHeight` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getId` | method | 4 |
| `net/minecraft/server/level/ServerPlayer#getInventory` | method | 9 |
| `net/minecraft/server/level/ServerPlayer#getMainHandItem` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#getRecipeBook` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getUUID` | method | 5 |
| `net/minecraft/server/level/ServerPlayer#getX` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#getXRot` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getY` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getYRot` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#getZ` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#giveExperienceLevels` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#inventoryMenu` | field | 7 |
| `net/minecraft/server/level/ServerPlayer#isAlive` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#isCrouching` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#isSpectator` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#level` | method | 11 |
| `net/minecraft/server/level/ServerPlayer#openMenu` | method | 3 |
| `net/minecraft/server/level/ServerPlayer#setItemInHand` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#teleportTo` | method | 1 |
| `net/minecraft/server/level/ServerPlayer#tickCount` | field | 1 |
| `net/minecraft/server/network/Filterable#passThrough` | method | 1 |
| `net/minecraft/server/network/PlayerChunkSender#sendChunk` | method | 1 |
| `net/minecraft/server/network/ServerGamePacketListenerImpl#player` | field | 3 |
| `net/minecraft/server/permissions/LevelBasedPermissionSet#OWNER` | field | 1 |
| `net/minecraft/server/players/PlayerList#getPlayer` | method | 1 |
| `net/minecraft/server/players/PlayerList#getPlayers` | method | 2 |
| `net/minecraft/sounds/SoundEvents#AMETHYST_BLOCK_CHIME` | field | 3 |
| `net/minecraft/sounds/SoundEvents#AMETHYST_BLOCK_RESONATE` | field | 2 |
| `net/minecraft/sounds/SoundEvents#AMETHYST_CLUSTER_BREAK` | field | 1 |
| `net/minecraft/sounds/SoundEvents#ANVIL_USE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#ARMOR_EQUIP_IRON` | field | 1 |
| `net/minecraft/sounds/SoundEvents#ARMOR_EQUIP_LEATHER` | field | 1 |
| `net/minecraft/sounds/SoundEvents#ARMOR_EQUIP_NETHERITE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#BEACON_ACTIVATE` | field | 2 |
| `net/minecraft/sounds/SoundEvents#BEACON_AMBIENT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#BEACON_DEACTIVATE` | field | 2 |
| `net/minecraft/sounds/SoundEvents#BEACON_POWER_SELECT` | field | 2 |
| `net/minecraft/sounds/SoundEvents#BOOK_PAGE_TURN` | field | 5 |
| `net/minecraft/sounds/SoundEvents#BOOK_PUT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#BOTTLE_FILL` | field | 1 |
| `net/minecraft/sounds/SoundEvents#CANDLE_EXTINGUISH` | field | 1 |
| `net/minecraft/sounds/SoundEvents#CHAIN_HIT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#CHAIN_PLACE` | field | 2 |
| `net/minecraft/sounds/SoundEvents#CHISELED_BOOKSHELF_INSERT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#CHISELED_BOOKSHELF_INSERT_ENCHANTED` | field | 1 |
| `net/minecraft/sounds/SoundEvents#CHISELED_BOOKSHELF_PICKUP` | field | 1 |
| `net/minecraft/sounds/SoundEvents#CHISELED_BOOKSHELF_PICKUP_ENCHANTED` | field | 1 |
| `net/minecraft/sounds/SoundEvents#CHISELED_BOOKSHELF_STEP` | field | 1 |
| `net/minecraft/sounds/SoundEvents#DEEPSLATE_HIT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#FIREWORK_ROCKET_LARGE_BLAST` | field | 1 |
| `net/minecraft/sounds/SoundEvents#GLASS_PLACE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#GLASS_STEP` | field | 1 |
| `net/minecraft/sounds/SoundEvents#GRASS_STEP` | field | 1 |
| `net/minecraft/sounds/SoundEvents#GRINDSTONE_USE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#ITEM_PICKUP` | field | 3 |
| `net/minecraft/sounds/SoundEvents#LAVA_EXTINGUISH` | field | 1 |
| `net/minecraft/sounds/SoundEvents#METAL_STEP` | field | 4 |
| `net/minecraft/sounds/SoundEvents#SAND_PLACE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#SMITHING_TABLE_USE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#STONE_BUTTON_CLICK_OFF` | field | 1 |
| `net/minecraft/sounds/SoundEvents#STONE_BUTTON_CLICK_ON` | field | 1 |
| `net/minecraft/sounds/SoundEvents#STONE_HIT` | field | 3 |
| `net/minecraft/sounds/SoundEvents#STONE_PLACE` | field | 2 |
| `net/minecraft/sounds/SoundEvents#STONE_STEP` | field | 5 |
| `net/minecraft/sounds/SoundEvents#UI_BUTTON_CLICK` | field | 2 |
| `net/minecraft/sounds/SoundEvents#UI_LOOM_SELECT_PATTERN` | field | 1 |
| `net/minecraft/sounds/SoundEvents#UI_STONECUTTER_SELECT_RECIPE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#WOOD_HIT` | field | 2 |
| `net/minecraft/sounds/SoundEvents#WOOD_PLACE` | field | 2 |
| `net/minecraft/sounds/SoundEvents#WOOD_STEP` | field | 3 |
| `net/minecraft/sounds/SoundEvents#WOOL_HIT` | field | 1 |
| `net/minecraft/sounds/SoundEvents#WOOL_PLACE` | field | 1 |
| `net/minecraft/sounds/SoundEvents#WOOL_STEP` | field | 1 |
| `net/minecraft/sounds/SoundSource#BLOCKS` | field | 3 |
| `net/minecraft/sounds/SoundSource#PLAYERS` | field | 3 |
| `net/minecraft/stats/ServerRecipeBook#contains` | method | 1 |
| `net/minecraft/stats/StatType#get` | method | 1 |
| `net/minecraft/stats/Stats#INTERACT_WITH_CAMPFIRE` | field | 1 |
| `net/minecraft/stats/Stats#ITEM_USED` | field | 1 |
| `net/minecraft/tags/BlockTags#ANVIL` | field | 1 |
| `net/minecraft/tags/BlockTags#CAMPFIRES` | field | 2 |
| `net/minecraft/tags/BlockTags#FIRE` | field | 1 |
| `net/minecraft/tags/EnchantmentTags#CURSE` | field | 1 |
| `net/minecraft/tags/FluidTags#LAVA` | field | 1 |
| `net/minecraft/tags/ItemTags#BEACON_PAYMENT_ITEMS` | field | 1 |
| `net/minecraft/tags/ItemTags#BOOKSHELF_BOOKS` | field | 2 |
| `net/minecraft/util/ARGB#alpha` | method | 1 |
| `net/minecraft/util/ARGB#blue` | method | 1 |
| `net/minecraft/util/ARGB#green` | method | 1 |
| `net/minecraft/util/ARGB#red` | method | 1 |
| `net/minecraft/util/LightCoordsUtil#block` | method | 3 |
| `net/minecraft/util/LightCoordsUtil#getLightCoords` | method | 9 |
| `net/minecraft/util/LightCoordsUtil#pack` | method | 4 |
| `net/minecraft/util/LightCoordsUtil#sky` | method | 4 |
| `net/minecraft/util/Mth#cos` | method | 1 |
| `net/minecraft/util/Mth#sin` | method | 1 |
| `net/minecraft/util/Prediction#SERVER_ONLY` | field | 4 |
| `net/minecraft/util/ProblemReporter#DISCARDING` | field | 1 |
| `net/minecraft/util/RandomSource#create` | method | 5 |
| `net/minecraft/util/RandomSource#nextBoolean` | method | 1 |
| `net/minecraft/util/RandomSource#nextDouble` | method | 13 |
| `net/minecraft/util/RandomSource#nextFloat` | method | 12 |
| `net/minecraft/util/RandomSource#nextGaussian` | method | 5 |
| `net/minecraft/util/RandomSource#nextInt` | method | 2 |
| `net/minecraft/util/RandomSource#setSeed` | method | 1 |
| `net/minecraft/util/Unit#INSTANCE` | field | 1 |
| `net/minecraft/util/Util#memoize` | method | 1 |
| `net/minecraft/world/Container#getItem` | method | 1 |
| `net/minecraft/world/ContainerHelper#loadAllItems` | method | 1 |
| `net/minecraft/world/ContainerHelper#saveAllItems` | method | 1 |
| `net/minecraft/world/Containers#dropContents` | method | 1 |
| `net/minecraft/world/Difficulty#PEACEFUL` | field | 1 |
| `net/minecraft/world/InteractionHand#MAIN_HAND` | field | 4 |
| `net/minecraft/world/InteractionResult#CONSUME` | field | 1 |
| `net/minecraft/world/InteractionResult#PASS` | field | 1 |
| `net/minecraft/world/InteractionResult#SUCCESS` | field | 1 |
| `net/minecraft/world/SimpleContainer#<init>` | method | 1 |
| `net/minecraft/world/effect/MobEffect#getColor` | method | 1 |
| `net/minecraft/world/effect/MobEffect#getDescriptionId` | method | 1 |
| `net/minecraft/world/entity/Entity#getBbHeight` | method | 1 |
| `net/minecraft/world/entity/Entity#getBbWidth` | method | 1 |
| `net/minecraft/world/entity/Entity#getEyePosition` | method | 2 |
| `net/minecraft/world/entity/Entity#getId` | method | 2 |
| `net/minecraft/world/entity/Entity#getViewYRot` | method | 1 |
| `net/minecraft/world/entity/Entity#isCrouching` | method | 1 |
| `net/minecraft/world/entity/Entity#position` | method | 2 |
| `net/minecraft/world/entity/Entity$RemovalReason#DISCARDED` | field | 1 |
| `net/minecraft/world/entity/EquipmentSlot#FEET` | field | 2 |
| `net/minecraft/world/entity/EquipmentSlot#LEGS` | field | 2 |
| `net/minecraft/world/entity/HumanoidArm#RIGHT` | field | 2 |
| `net/minecraft/world/entity/LivingEntity#yBodyRot` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#yBodyRotO` | field | 1 |
| `net/minecraft/world/entity/Pose#CROUCHING` | field | 1 |
| `net/minecraft/world/entity/Pose#STANDING` | field | 1 |
| `net/minecraft/world/entity/ai/village/poi/PoiTypes#forState` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getItem` | method | 1 |
| `net/minecraft/world/entity/npc/villager/VillagerProfession#NONE` | field | 1 |
| `net/minecraft/world/entity/npc/villager/VillagerProfession#heldJobSite` | method | 1 |
| `net/minecraft/world/entity/player/Abilities#flying` | field | 1 |
| `net/minecraft/world/entity/player/Abilities#instabuild` | field | 2 |
| `net/minecraft/world/entity/player/Input#<init>` | method | 1 |
| `net/minecraft/world/entity/player/Input#EMPTY` | field | 1 |
| `net/minecraft/world/entity/player/Inventory#clearContent` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#getContainerSize` | method | 2 |
| `net/minecraft/world/entity/player/Inventory#getFreeSlot` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#getItem` | method | 13 |
| `net/minecraft/world/entity/player/Inventory#getNonEquipmentItems` | method | 2 |
| `net/minecraft/world/entity/player/Inventory#getSelectedSlot` | method | 3 |
| `net/minecraft/world/entity/player/Inventory#getSlotWithRemainingSpace` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#placeItemBackInInventory` | method | 3 |
| `net/minecraft/world/entity/player/Inventory#removeItemNoUpdate` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#setChanged` | method | 3 |
| `net/minecraft/world/entity/player/Inventory#setItem` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#setSelectedSlot` | method | 1 |
| `net/minecraft/world/entity/player/Player#getX` | method | 1 |
| `net/minecraft/world/entity/player/Player#getZ` | method | 1 |
| `net/minecraft/world/entity/player/Player#isSecondaryUseActive` | method | 1 |
| `net/minecraft/world/entity/player/Player#isSpectator` | method | 1 |
| `net/minecraft/world/entity/player/Player#yBodyRot` | field | 1 |
| `net/minecraft/world/entity/player/PlayerModelPart#LEFT_PANTS_LEG` | field | 1 |
| `net/minecraft/world/entity/player/PlayerModelPart#LEFT_SLEEVE` | field | 1 |
| `net/minecraft/world/entity/player/PlayerModelPart#RIGHT_PANTS_LEG` | field | 1 |
| `net/minecraft/world/entity/player/PlayerModelPart#RIGHT_SLEEVE` | field | 1 |
| `net/minecraft/world/entity/player/PlayerModelType#SLIM` | field | 1 |
| `net/minecraft/world/entity/player/PlayerSkin#body` | method | 3 |
| `net/minecraft/world/entity/player/PlayerSkin#model` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#broadcastChanges` | method | 4 |
| `net/minecraft/world/inventory/AbstractContainerMenu#clicked` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#containerId` | field | 5 |
| `net/minecraft/world/inventory/AbstractContainerMenu#getCarried` | method | 12 |
| `net/minecraft/world/inventory/AbstractContainerMenu#getQuickcraftMask` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#getSlot` | method | 4 |
| `net/minecraft/world/inventory/AbstractContainerMenu#setCarried` | method | 5 |
| `net/minecraft/world/inventory/AbstractContainerMenu#slots` | field | 4 |
| `net/minecraft/world/inventory/AbstractContainerMenu#stillValid` | method | 1 |
| `net/minecraft/world/inventory/AbstractFurnaceMenu#containerId` | field | 2 |
| `net/minecraft/world/inventory/AbstractFurnaceMenu#getBurnProgress` | method | 1 |
| `net/minecraft/world/inventory/AbstractFurnaceMenu#getLitProgress` | method | 1 |
| `net/minecraft/world/inventory/AbstractFurnaceMenu#getSlot` | method | 1 |
| `net/minecraft/world/inventory/AnvilMenu#containerId` | field | 1 |
| `net/minecraft/world/inventory/AnvilMenu#getCost` | method | 2 |
| `net/minecraft/world/inventory/AnvilMenu#getResultSlot` | method | 1 |
| `net/minecraft/world/inventory/AnvilMenu#getSlot` | method | 1 |
| `net/minecraft/world/inventory/AnvilMenu#setItemName` | method | 1 |
| `net/minecraft/world/inventory/BeaconMenu#getLevels` | method | 2 |
| `net/minecraft/world/inventory/BeaconMenu#getPrimaryEffect` | method | 1 |
| `net/minecraft/world/inventory/BeaconMenu#getSecondaryEffect` | method | 1 |
| `net/minecraft/world/inventory/BrewingStandMenu#getBrewingTicks` | method | 1 |
| `net/minecraft/world/inventory/BrewingStandMenu#getFuel` | method | 1 |
| `net/minecraft/world/inventory/CartographyTableMenu#containerId` | field | 1 |
| `net/minecraft/world/inventory/ContainerInput#PICKUP` | field | 14 |
| `net/minecraft/world/inventory/ContainerInput#QUICK_CRAFT` | field | 1 |
| `net/minecraft/world/inventory/ContainerInput#QUICK_MOVE` | field | 14 |
| `net/minecraft/world/inventory/ContainerInput#SWAP` | field | 1 |
| `net/minecraft/world/inventory/ContainerInput#THROW` | field | 1 |
| `net/minecraft/world/inventory/ContainerLevelAccess#evaluate` | method | 1 |
| `net/minecraft/world/inventory/CrafterMenu#containerId` | field | 1 |
| `net/minecraft/world/inventory/CrafterMenu#isPowered` | method | 1 |
| `net/minecraft/world/inventory/CrafterMenu#isSlotDisabled` | method | 2 |
| `net/minecraft/world/inventory/CrafterMenu#setSlotState` | method | 1 |
| `net/minecraft/world/inventory/CraftingMenu#access` | method | 1 |
| `net/minecraft/world/inventory/CraftingMenu#broadcastChanges` | method | 1 |
| `net/minecraft/world/inventory/CraftingMenu#containerId` | field | 3 |
| `net/minecraft/world/inventory/CraftingMenu#getCarried` | method | 2 |
| `net/minecraft/world/inventory/CraftingMenu#getSlot` | method | 4 |
| `net/minecraft/world/inventory/EnchantmentMenu#clickMenuButton` | method | 1 |
| `net/minecraft/world/inventory/EnchantmentMenu#containerId` | field | 1 |
| `net/minecraft/world/inventory/EnchantmentMenu#costs` | field | 1 |
| `net/minecraft/world/inventory/EnchantmentMenu#enchantClue` | field | 1 |
| `net/minecraft/world/inventory/EnchantmentMenu#getEnchantmentSeed` | method | 1 |
| `net/minecraft/world/inventory/EnchantmentMenu#getSlot` | method | 4 |
| `net/minecraft/world/inventory/EnchantmentMenu#levelClue` | field | 1 |
| `net/minecraft/world/inventory/GrindstoneMenu#containerId` | field | 1 |
| `net/minecraft/world/inventory/InventoryMenu#broadcastChanges` | method | 6 |
| `net/minecraft/world/inventory/InventoryMenu#getCarried` | method | 3 |
| `net/minecraft/world/inventory/InventoryMenu#setCarried` | method | 4 |
| `net/minecraft/world/inventory/LecternMenu#containerId` | field | 1 |
| `net/minecraft/world/inventory/LecternMenu#getBook` | method | 1 |
| `net/minecraft/world/inventory/LecternMenu#getPage` | method | 1 |
| `net/minecraft/world/inventory/LecternMenu#setData` | method | 1 |
| `net/minecraft/world/inventory/LoomMenu#clickMenuButton` | method | 1 |
| `net/minecraft/world/inventory/LoomMenu#containerId` | field | 1 |
| `net/minecraft/world/inventory/LoomMenu#getSelectablePatterns` | method | 1 |
| `net/minecraft/world/inventory/LoomMenu#getSelectedBannerPatternIndex` | method | 1 |
| `net/minecraft/world/inventory/Slot#<init>` | method | 1 |
| `net/minecraft/world/inventory/Slot#container` | field | 4 |
| `net/minecraft/world/inventory/Slot#getContainerSlot` | method | 3 |
| `net/minecraft/world/inventory/Slot#getItem` | method | 12 |
| `net/minecraft/world/inventory/Slot#getMaxStackSize` | method | 1 |
| `net/minecraft/world/inventory/Slot#hasItem` | method | 2 |
| `net/minecraft/world/inventory/Slot#index` | field | 1 |
| `net/minecraft/world/inventory/Slot#mayPickup` | method | 1 |
| `net/minecraft/world/inventory/Slot#mayPlace` | method | 3 |
| `net/minecraft/world/inventory/Slot#set` | method | 2 |
| `net/minecraft/world/inventory/Slot#setChanged` | method | 1 |
| `net/minecraft/world/inventory/SmithingMenu#broadcastChanges` | method | 1 |
| `net/minecraft/world/inventory/SmithingMenu#containerId` | field | 1 |
| `net/minecraft/world/inventory/SmithingMenu#createResult` | method | 1 |
| `net/minecraft/world/inventory/SmithingMenu#getSlot` | method | 1 |
| `net/minecraft/world/inventory/StonecutterMenu#clickMenuButton` | method | 1 |
| `net/minecraft/world/inventory/StonecutterMenu#containerId` | field | 1 |
| `net/minecraft/world/inventory/StonecutterMenu#getSelectedRecipeIndex` | method | 1 |
| `net/minecraft/world/inventory/StonecutterMenu#getVisibleRecipes` | method | 2 |
| `net/minecraft/world/inventory/StonecutterMenu#hasInputItem` | method | 1 |
| `net/minecraft/world/item/BannerItem#getColor` | method | 1 |
| `net/minecraft/world/item/BlockItem#getBlock` | method | 1 |
| `net/minecraft/world/item/DyeColor#WHITE` | field | 1 |
| `net/minecraft/world/item/DyeColor#getTextureDiffuseColor` | method | 1 |
| `net/minecraft/world/item/Item#builtInRegistryHolder` | method | 1 |
| `net/minecraft/world/item/ItemDisplayContext#FIXED` | field | 1 |
| `net/minecraft/world/item/ItemDisplayContext#GUI` | field | 1 |
| `net/minecraft/world/item/ItemDisplayContext#NONE` | field | 1 |
| `net/minecraft/world/item/ItemDisplayContext#firstPerson` | method | 1 |
| `net/minecraft/world/item/ItemStack#<init>` | method | 16 |
| `net/minecraft/world/item/ItemStack#EMPTY` | field | 42 |
| `net/minecraft/world/item/ItemStack#OPTIONAL_LIST_STREAM_CODEC` | field | 6 |
| `net/minecraft/world/item/ItemStack#OPTIONAL_STREAM_CODEC` | field | 2 |
| `net/minecraft/world/item/ItemStack#consumeAndReturn` | method | 2 |
| `net/minecraft/world/item/ItemStack#copy` | method | 26 |
| `net/minecraft/world/item/ItemStack#copyWithCount` | method | 11 |
| `net/minecraft/world/item/ItemStack#get` | method | 5 |
| `net/minecraft/world/item/ItemStack#getDamageValue` | method | 1 |
| `net/minecraft/world/item/ItemStack#getEnchantments` | method | 1 |
| `net/minecraft/world/item/ItemStack#getHoverName` | method | 12 |
| `net/minecraft/world/item/ItemStack#getItem` | method | 9 |
| `net/minecraft/world/item/ItemStack#getMaxStackSize` | method | 9 |
| `net/minecraft/world/item/ItemStack#getOrDefault` | method | 2 |
| `net/minecraft/world/item/ItemStack#grow` | method | 5 |
| `net/minecraft/world/item/ItemStack#has` | method | 7 |
| `net/minecraft/world/item/ItemStack#hashItemAndComponents` | method | 6 |
| `net/minecraft/world/item/ItemStack#is` | method | 11 |
| `net/minecraft/world/item/ItemStack#isDamaged` | method | 1 |
| `net/minecraft/world/item/ItemStack#isEmpty` | method | 54 |
| `net/minecraft/world/item/ItemStack#isEnchanted` | method | 2 |
| `net/minecraft/world/item/ItemStack#isItemEnabled` | method | 2 |
| `net/minecraft/world/item/ItemStack#isSameItem` | method | 1 |
| `net/minecraft/world/item/ItemStack#onCraftedBy` | method | 2 |
| `net/minecraft/world/item/ItemStack#remove` | method | 1 |
| `net/minecraft/world/item/ItemStack#set` | method | 2 |
| `net/minecraft/world/item/ItemStack#setDamageValue` | method | 1 |
| `net/minecraft/world/item/ItemStack#split` | method | 5 |
| `net/minecraft/world/item/ItemStackTemplate#fromNonEmptyStack` | method | 1 |
| `net/minecraft/world/item/Items#AIR` | field | 2 |
| `net/minecraft/world/item/Items#AMETHYST_SHARD` | field | 1 |
| `net/minecraft/world/item/Items#ANVIL` | field | 1 |
| `net/minecraft/world/item/Items#APPLE` | field | 1 |
| `net/minecraft/world/item/Items#BANNER` | field | 1 |
| `net/minecraft/world/item/Items#BARRIER` | field | 1 |
| `net/minecraft/world/item/Items#BEACON` | field | 1 |
| `net/minecraft/world/item/Items#BEEF` | field | 3 |
| `net/minecraft/world/item/Items#BIRCH_PLANKS` | field | 1 |
| `net/minecraft/world/item/Items#BLAST_FURNACE` | field | 1 |
| `net/minecraft/world/item/Items#BLAZE_POWDER` | field | 2 |
| `net/minecraft/world/item/Items#BOOK` | field | 4 |
| `net/minecraft/world/item/Items#BREWING_STAND` | field | 1 |
| `net/minecraft/world/item/Items#CAMPFIRE` | field | 1 |
| `net/minecraft/world/item/Items#CARTOGRAPHY_TABLE` | field | 1 |
| `net/minecraft/world/item/Items#CHEST` | field | 1 |
| `net/minecraft/world/item/Items#CHICKEN` | field | 1 |
| `net/minecraft/world/item/Items#CHISELED_BOOKSHELF` | field | 1 |
| `net/minecraft/world/item/Items#COAL` | field | 2 |
| `net/minecraft/world/item/Items#COAST_ARMOR_TRIM_SMITHING_TEMPLATE` | field | 1 |
| `net/minecraft/world/item/Items#COBBLED_DEEPSLATE` | field | 1 |
| `net/minecraft/world/item/Items#COBBLESTONE` | field | 1 |
| `net/minecraft/world/item/Items#COOKED_BEEF` | field | 2 |
| `net/minecraft/world/item/Items#CRAFTER` | field | 1 |
| `net/minecraft/world/item/Items#CRAFTING_TABLE` | field | 2 |
| `net/minecraft/world/item/Items#DIAMOND` | field | 1 |
| `net/minecraft/world/item/Items#DIAMOND_CHESTPLATE` | field | 2 |
| `net/minecraft/world/item/Items#DIAMOND_SWORD` | field | 2 |
| `net/minecraft/world/item/Items#DYE` | field | 2 |
| `net/minecraft/world/item/Items#DYED_SHULKER_BOX` | field | 1 |
| `net/minecraft/world/item/Items#ENCHANTED_BOOK` | field | 7 |
| `net/minecraft/world/item/Items#ENCHANTING_TABLE` | field | 1 |
| `net/minecraft/world/item/Items#FILLED_MAP` | field | 1 |
| `net/minecraft/world/item/Items#FLINT` | field | 1 |
| `net/minecraft/world/item/Items#FLOWER_BANNER_PATTERN` | field | 2 |
| `net/minecraft/world/item/Items#FURNACE` | field | 1 |
| `net/minecraft/world/item/Items#GLASS_BOTTLE` | field | 1 |
| `net/minecraft/world/item/Items#GLASS_PANE` | field | 2 |
| `net/minecraft/world/item/Items#GOLDEN_SWORD` | field | 1 |
| `net/minecraft/world/item/Items#GOLD_INGOT` | field | 2 |
| `net/minecraft/world/item/Items#GRINDSTONE` | field | 1 |
| `net/minecraft/world/item/Items#IRON_INGOT` | field | 2 |
| `net/minecraft/world/item/Items#IRON_PICKAXE` | field | 2 |
| `net/minecraft/world/item/Items#IRON_SWORD` | field | 1 |
| `net/minecraft/world/item/Items#JUKEBOX` | field | 1 |
| `net/minecraft/world/item/Items#LAPIS_LAZULI` | field | 2 |
| `net/minecraft/world/item/Items#LECTERN` | field | 1 |
| `net/minecraft/world/item/Items#LOOM` | field | 1 |
| `net/minecraft/world/item/Items#MAP` | field | 2 |
| `net/minecraft/world/item/Items#MUSIC_DISC_13` | field | 2 |
| `net/minecraft/world/item/Items#MUSIC_DISC_CAT` | field | 1 |
| `net/minecraft/world/item/Items#MUTTON` | field | 1 |
| `net/minecraft/world/item/Items#NAME_TAG` | field | 1 |
| `net/minecraft/world/item/Items#NETHERITE_CHESTPLATE` | field | 1 |
| `net/minecraft/world/item/Items#NETHERITE_INGOT` | field | 2 |
| `net/minecraft/world/item/Items#NETHERITE_UPGRADE_SMITHING_TEMPLATE` | field | 2 |
| `net/minecraft/world/item/Items#NETHER_STAR` | field | 1 |
| `net/minecraft/world/item/Items#NETHER_WART` | field | 2 |
| `net/minecraft/world/item/Items#OAK_LOG` | field | 2 |
| `net/minecraft/world/item/Items#OAK_PLANKS` | field | 3 |
| `net/minecraft/world/item/Items#PAPER` | field | 4 |
| `net/minecraft/world/item/Items#POTION` | field | 1 |
| `net/minecraft/world/item/Items#RAW_IRON` | field | 2 |
| `net/minecraft/world/item/Items#REDSTONE` | field | 1 |
| `net/minecraft/world/item/Items#SHULKER_BOX` | field | 1 |
| `net/minecraft/world/item/Items#SMITHING_TABLE` | field | 1 |
| `net/minecraft/world/item/Items#SMOKER` | field | 1 |
| `net/minecraft/world/item/Items#STICK` | field | 2 |
| `net/minecraft/world/item/Items#STONE` | field | 2 |
| `net/minecraft/world/item/Items#STONECUTTER` | field | 1 |
| `net/minecraft/world/item/Items#STONE_BRICKS` | field | 1 |
| `net/minecraft/world/item/Items#STONE_SLAB` | field | 2 |
| `net/minecraft/world/item/Items#STONE_STAIRS` | field | 2 |
| `net/minecraft/world/item/Items#TORCH` | field | 1 |
| `net/minecraft/world/item/Items#WOODEN_PICKAXE` | field | 1 |
| `net/minecraft/world/item/Items#WRITABLE_BOOK` | field | 1 |
| `net/minecraft/world/item/Items#WRITTEN_BOOK` | field | 2 |
| `net/minecraft/world/item/JukeboxPlayable#tryInsertIntoJukebox` | method | 1 |
| `net/minecraft/world/item/JukeboxSong#description` | method | 1 |
| `net/minecraft/world/item/JukeboxSong#fromStack` | method | 1 |
| `net/minecraft/world/item/MapItem#create` | method | 1 |
| `net/minecraft/world/item/MapItem#getSavedData` | method | 1 |
| `net/minecraft/world/item/MapItem#update` | method | 1 |
| `net/minecraft/world/item/alchemy/PotionContents#createItemStack` | method | 1 |
| `net/minecraft/world/item/alchemy/PotionContents#getColor` | method | 1 |
| `net/minecraft/world/item/alchemy/PotionContents#is` | method | 1 |
| `net/minecraft/world/item/alchemy/Potions#AWKWARD` | field | 1 |
| `net/minecraft/world/item/alchemy/Potions#WATER` | field | 1 |
| `net/minecraft/world/item/component/SwingAnimation#DEFAULT` | field | 1 |
| `net/minecraft/world/item/component/WritableBookContent#<init>` | method | 1 |
| `net/minecraft/world/item/component/WrittenBookContent#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/AbstractCookingRecipe#assemble` | method | 1 |
| `net/minecraft/world/item/crafting/CampfireCookingRecipe#assemble` | method | 1 |
| `net/minecraft/world/item/crafting/CampfireCookingRecipe#cookingTime` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingInput#height` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingInput#isEmpty` | method | 2 |
| `net/minecraft/world/item/crafting/CraftingInput#ofPositioned` | method | 2 |
| `net/minecraft/world/item/crafting/CraftingInput#width` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingInput$Positioned#input` | method | 2 |
| `net/minecraft/world/item/crafting/CraftingInput$Positioned#left` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingInput$Positioned#top` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingRecipe#assemble` | method | 2 |
| `net/minecraft/world/item/crafting/CraftingRecipe#display` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingRecipe#getRemainingItems` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingRecipe#getSerializer` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingRecipe#isSpecial` | method | 2 |
| `net/minecraft/world/item/crafting/CraftingRecipe#matches` | method | 1 |
| `net/minecraft/world/item/crafting/CraftingRecipe#placementInfo` | method | 2 |
| `net/minecraft/world/item/crafting/Ingredient#items` | method | 1 |
| `net/minecraft/world/item/crafting/Ingredient#of` | method | 3 |
| `net/minecraft/world/item/crafting/Ingredient#test` | method | 3 |
| `net/minecraft/world/item/crafting/PlacementInfo#ingredients` | method | 1 |
| `net/minecraft/world/item/crafting/PlacementInfo#isImpossibleToPlace` | method | 1 |
| `net/minecraft/world/item/crafting/Recipe#getSerializer` | method | 1 |
| `net/minecraft/world/item/crafting/Recipe#getType` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeAccess#propertySet` | method | 2 |
| `net/minecraft/world/item/crafting/RecipeHolder#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeHolder#STREAM_CODEC` | field | 1 |
| `net/minecraft/world/item/crafting/RecipeHolder#id` | method | 9 |
| `net/minecraft/world/item/crafting/RecipeHolder#value` | method | 8 |
| `net/minecraft/world/item/crafting/RecipeManager#createCheck` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeManager#getRecipeFor` | method | 2 |
| `net/minecraft/world/item/crafting/RecipeManager#getRecipes` | method | 3 |
| `net/minecraft/world/item/crafting/RecipeManager#propertySet` | method | 1 |
| `net/minecraft/world/item/crafting/RecipeManager$CachedCheck#getRecipeFor` | method | 1 |
| `net/minecraft/world/item/crafting/RecipePropertySet#BLAST_FURNACE_INPUT` | field | 2 |
| `net/minecraft/world/item/crafting/RecipePropertySet#CAMPFIRE_INPUT` | field | 1 |
| `net/minecraft/world/item/crafting/RecipePropertySet#FURNACE_INPUT` | field | 2 |
| `net/minecraft/world/item/crafting/RecipePropertySet#SMOKER_INPUT` | field | 2 |
| `net/minecraft/world/item/crafting/RecipePropertySet#test` | method | 3 |
| `net/minecraft/world/item/crafting/RecipeType#BLASTING` | field | 2 |
| `net/minecraft/world/item/crafting/RecipeType#CAMPFIRE_COOKING` | field | 2 |
| `net/minecraft/world/item/crafting/RecipeType#CRAFTING` | field | 2 |
| `net/minecraft/world/item/crafting/RecipeType#SMELTING` | field | 3 |
| `net/minecraft/world/item/crafting/RecipeType#SMITHING` | field | 1 |
| `net/minecraft/world/item/crafting/RecipeType#SMOKING` | field | 3 |
| `net/minecraft/world/item/crafting/SelectableRecipe#optionDisplay` | method | 2 |
| `net/minecraft/world/item/crafting/SelectableRecipe$SingleInputEntry#recipe` | method | 2 |
| `net/minecraft/world/item/crafting/SelectableRecipe$SingleInputSet#entries` | method | 2 |
| `net/minecraft/world/item/crafting/ShapedRecipe#getHeight` | method | 2 |
| `net/minecraft/world/item/crafting/ShapedRecipe#getIngredients` | method | 1 |
| `net/minecraft/world/item/crafting/ShapedRecipe#getWidth` | method | 2 |
| `net/minecraft/world/item/crafting/SingleRecipeInput#<init>` | method | 2 |
| `net/minecraft/world/item/crafting/SmithingRecipe#assemble` | method | 1 |
| `net/minecraft/world/item/crafting/SmithingRecipeInput#<init>` | method | 1 |
| `net/minecraft/world/item/crafting/display/RecipeDisplay#result` | method | 1 |
| `net/minecraft/world/item/crafting/display/RecipeDisplayEntry#display` | method | 2 |
| `net/minecraft/world/item/crafting/display/RecipeDisplayEntry#id` | method | 2 |
| `net/minecraft/world/item/crafting/display/RecipeDisplayId#index` | method | 2 |
| `net/minecraft/world/item/crafting/display/ShapedCraftingRecipeDisplay#height` | method | 2 |
| `net/minecraft/world/item/crafting/display/ShapedCraftingRecipeDisplay#ingredients` | method | 1 |
| `net/minecraft/world/item/crafting/display/ShapedCraftingRecipeDisplay#width` | method | 2 |
| `net/minecraft/world/item/crafting/display/ShapelessCraftingRecipeDisplay#ingredients` | method | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplay#resolveForFirstStack` | method | 3 |
| `net/minecraft/world/item/crafting/display/SlotDisplay#resolveForStacks` | method | 1 |
| `net/minecraft/world/item/crafting/display/SlotDisplayContext#fromLevel` | method | 3 |
| `net/minecraft/world/item/enchantment/Enchantment#getFullname` | method | 1 |
| `net/minecraft/world/item/enchantment/EnchantmentHelper#createBook` | method | 1 |
| `net/minecraft/world/item/enchantment/EnchantmentHelper#getEnchantmentsForCrafting` | method | 1 |
| `net/minecraft/world/item/enchantment/EnchantmentInstance#<init>` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#EFFICIENCY` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#LOOTING` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#MENDING` | field | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#SHARPNESS` | field | 1 |
| `net/minecraft/world/item/enchantment/ItemEnchantments#keySet` | method | 1 |
| `net/minecraft/world/item/equipment/Equippable#assetId` | method | 1 |
| `net/minecraft/world/item/equipment/Equippable#slot` | method | 1 |
| `net/minecraft/world/item/equipment/trim/ArmorTrim#material` | method | 2 |
| `net/minecraft/world/item/equipment/trim/TrimMaterial#description` | method | 1 |
| `net/minecraft/world/level/BlockGetter#getBlockState` | method | 1 |
| `net/minecraft/world/level/ChunkPos#containing` | method | 7 |
| `net/minecraft/world/level/ChunkPos#pack` | method | 2 |
| `net/minecraft/world/level/ChunkPos#x` | method | 7 |
| `net/minecraft/world/level/ChunkPos#z` | method | 7 |
| `net/minecraft/world/level/ClipContext#<init>` | method | 4 |
| `net/minecraft/world/level/ClipContext$Block#COLLIDER` | field | 2 |
| `net/minecraft/world/level/ClipContext$Block#OUTLINE` | field | 1 |
| `net/minecraft/world/level/ClipContext$Block#VISUAL` | field | 1 |
| `net/minecraft/world/level/ClipContext$Fluid#NONE` | field | 4 |
| `net/minecraft/world/level/GameType#SURVIVAL` | field | 1 |
| `net/minecraft/world/level/ItemLike#asItem` | method | 1 |
| `net/minecraft/world/level/Level#clip` | method | 1 |
| `net/minecraft/world/level/Level#enabledFeatures` | method | 1 |
| `net/minecraft/world/level/Level#getBlockState` | method | 8 |
| `net/minecraft/world/level/LevelSettings#<init>` | method | 1 |
| `net/minecraft/world/level/LevelSettings$DifficultySettings#<init>` | method | 1 |
| `net/minecraft/world/level/WorldDataConfiguration#DEFAULT` | field | 1 |
| `net/minecraft/world/level/block/AbstractFurnaceBlock#FACING` | field | 5 |
| `net/minecraft/world/level/block/AbstractFurnaceBlock#LIT` | field | 2 |
| `net/minecraft/world/level/block/AnvilBlock#FACING` | field | 2 |
| `net/minecraft/world/level/block/Block#asItem` | method | 1 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 2 |
| `net/minecraft/world/level/block/Block#defaultMapColor` | method | 1 |
| `net/minecraft/world/level/block/Block#getName` | method | 3 |
| `net/minecraft/world/level/block/Blocks#AIR` | field | 1 |
| `net/minecraft/world/level/block/Blocks#ANVIL` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BARREL` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BEACON` | field | 2 |
| `net/minecraft/world/level/block/Blocks#BLAST_FURNACE` | field | 2 |
| `net/minecraft/world/level/block/Blocks#BOOKSHELF` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BREWING_STAND` | field | 3 |
| `net/minecraft/world/level/block/Blocks#CACTUS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CAMPFIRE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CANDLE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CARPET` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CARTOGRAPHY_TABLE` | field | 2 |
| `net/minecraft/world/level/block/Blocks#CHISELED_BOOKSHELF` | field | 4 |
| `net/minecraft/world/level/block/Blocks#CRAFTER` | field | 2 |
| `net/minecraft/world/level/block/Blocks#CRAFTING_TABLE` | field | 4 |
| `net/minecraft/world/level/block/Blocks#DEEPSLATE_BRICKS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DIAMOND_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#EMERALD_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#ENCHANTING_TABLE` | field | 3 |
| `net/minecraft/world/level/block/Blocks#FURNACE` | field | 2 |
| `net/minecraft/world/level/block/Blocks#GOLD_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GRINDSTONE` | field | 3 |
| `net/minecraft/world/level/block/Blocks#IRON_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#JUKEBOX` | field | 3 |
| `net/minecraft/world/level/block/Blocks#LANTERN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LAVA` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LECTERN` | field | 2 |
| `net/minecraft/world/level/block/Blocks#LOOM` | field | 2 |
| `net/minecraft/world/level/block/Blocks#MAGMA_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#POWDER_SNOW` | field | 1 |
| `net/minecraft/world/level/block/Blocks#REDSTONE_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SMITHING_TABLE` | field | 2 |
| `net/minecraft/world/level/block/Blocks#SMOKER` | field | 2 |
| `net/minecraft/world/level/block/Blocks#SOUL_CAMPFIRE` | field | 2 |
| `net/minecraft/world/level/block/Blocks#SPRUCE_LOG` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SPRUCE_PLANKS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#STONE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#STONECUTTER` | field | 2 |
| `net/minecraft/world/level/block/Blocks#STONE_BRICKS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#STONE_SLAB` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SWEET_BERRY_BUSH` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WITHER_ROSE` | field | 1 |
| `net/minecraft/world/level/block/BrewingStandBlock#HAS_BOTTLE` | field | 2 |
| `net/minecraft/world/level/block/CampfireBlock#FACING` | field | 2 |
| `net/minecraft/world/level/block/CampfireBlock#LIT` | field | 3 |
| `net/minecraft/world/level/block/CandleBlock#LIT` | field | 1 |
| `net/minecraft/world/level/block/ChiseledBookShelfBlock#SLOT_OCCUPIED_PROPERTIES` | field | 5 |
| `net/minecraft/world/level/block/ColorCollection#red` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#white` | method | 1 |
| `net/minecraft/world/level/block/CrafterBlock#CRAFTING` | field | 1 |
| `net/minecraft/world/level/block/EnchantingTableBlock#BOOKSHELF_OFFSETS` | field | 1 |
| `net/minecraft/world/level/block/EnchantingTableBlock#isValidBookShelf` | method | 1 |
| `net/minecraft/world/level/block/GrindstoneBlock#FACE` | field | 1 |
| `net/minecraft/world/level/block/GrindstoneBlock#FACING` | field | 3 |
| `net/minecraft/world/level/block/HorizontalDirectionalBlock#FACING` | field | 4 |
| `net/minecraft/world/level/block/JukeboxBlock#HAS_RECORD` | field | 3 |
| `net/minecraft/world/level/block/LecternBlock#FACING` | field | 2 |
| `net/minecraft/world/level/block/LecternBlock#HAS_BOOK` | field | 2 |
| `net/minecraft/world/level/block/LecternBlock#tryPlaceBook` | method | 1 |
| `net/minecraft/world/level/block/LoomBlock#FACING` | field | 2 |
| `net/minecraft/world/level/block/StonecutterBlock#FACING` | field | 2 |
| `net/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity#awardUsedRecipesAndPopExperience` | method | 1 |
| `net/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity#canPlaceItem` | method | 1 |
| `net/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity#cookingTimer` | field | 2 |
| `net/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity#cookingTotalTime` | field | 2 |
| `net/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity#getItem` | method | 4 |
| `net/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity#getLevel` | method | 1 |
| `net/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity#getMaxStackSize` | method | 2 |
| `net/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity#litTimeRemaining` | field | 2 |
| `net/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity#litTotalTime` | field | 1 |
| `net/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity#removeItem` | method | 1 |
| `net/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity#serverTick` | method | 1 |
| `net/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity#setChanged` | method | 2 |
| `net/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity#setItem` | method | 1 |
| `net/minecraft/world/level/block/entity/BannerPattern#assetId` | method | 2 |
| `net/minecraft/world/level/block/entity/BannerPatternLayers#<init>` | method | 1 |
| `net/minecraft/world/level/block/entity/BannerPatternLayers#EMPTY` | field | 2 |
| `net/minecraft/world/level/block/entity/BannerPatternLayers#layers` | method | 2 |
| `net/minecraft/world/level/block/entity/BannerPatternLayers$Layer#<init>` | method | 1 |
| `net/minecraft/world/level/block/entity/BannerPatternLayers$Layer#color` | method | 1 |
| `net/minecraft/world/level/block/entity/BannerPatternLayers$Layer#description` | method | 1 |
| `net/minecraft/world/level/block/entity/BannerPatternLayers$Layer#pattern` | method | 2 |
| `net/minecraft/world/level/block/entity/BaseContainerBlockEntity#<init>` | method | 1 |
| `net/minecraft/world/level/block/entity/BeaconBlockEntity#BEACON_EFFECTS` | field | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#getBlockPos` | method | 2 |
| `net/minecraft/world/level/block/entity/BlockEntity#getBlockState` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#saveWithoutMetadata` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntityTypes#BREWING_STAND` | field | 1 |
| `net/minecraft/world/level/block/entity/BlockEntityTypes#CHISELED_BOOKSHELF` | field | 1 |
| `net/minecraft/world/level/block/entity/BrewingStandBlockEntity#getBlockPos` | method | 2 |
| `net/minecraft/world/level/block/entity/BrewingStandBlockEntity#getItem` | method | 3 |
| `net/minecraft/world/level/block/entity/BrewingStandBlockEntity#serverTick` | method | 1 |
| `net/minecraft/world/level/block/entity/CampfireBlockEntity#cookingProgress` | field | 2 |
| `net/minecraft/world/level/block/entity/CampfireBlockEntity#cookingTime` | field | 2 |
| `net/minecraft/world/level/block/entity/CampfireBlockEntity#getBlockState` | method | 1 |
| `net/minecraft/world/level/block/entity/CampfireBlockEntity#getItems` | method | 4 |
| `net/minecraft/world/level/block/entity/CampfireBlockEntity#setChanged` | method | 2 |
| `net/minecraft/world/level/block/entity/ChiseledBookShelfBlockEntity#getBlockPos` | method | 1 |
| `net/minecraft/world/level/block/entity/ChiseledBookShelfBlockEntity#getBlockState` | method | 1 |
| `net/minecraft/world/level/block/entity/ChiseledBookShelfBlockEntity#getItem` | method | 2 |
| `net/minecraft/world/level/block/entity/ChiseledBookShelfBlockEntity#removeItem` | method | 1 |
| `net/minecraft/world/level/block/entity/ChiseledBookShelfBlockEntity#setItem` | method | 2 |
| `net/minecraft/world/level/block/entity/EnchantingTableBlockEntity#flipT` | field | 1 |
| `net/minecraft/world/level/block/entity/JukeboxBlockEntity#getTheItem` | method | 1 |
| `net/minecraft/world/level/block/entity/JukeboxBlockEntity#popOutTheItem` | method | 1 |
| `net/minecraft/world/level/block/entity/JukeboxBlockEntity#removeTheItem` | method | 1 |
| `net/minecraft/world/level/block/entity/LecternBlockEntity#getBook` | method | 1 |
| `net/minecraft/world/level/block/entity/LecternBlockEntity#getPage` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getBlock` | method | 8 |
| `net/minecraft/world/level/block/state/BlockState#getFluidState` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getMenuProvider` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getShape` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#getValue` | method | 16 |
| `net/minecraft/world/level/block/state/BlockState#hasProperty` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#is` | method | 11 |
| `net/minecraft/world/level/block/state/BlockState#setValue` | method | 2 |
| `net/minecraft/world/level/block/state/properties/AttachFace#CEILING` | field | 1 |
| `net/minecraft/world/level/block/state/properties/AttachFace#FLOOR` | field | 1 |
| `net/minecraft/world/level/block/state/properties/AttachFace#WALL` | field | 1 |
| `net/minecraft/world/level/block/state/properties/AttachFace#ordinal` | method | 2 |
| `net/minecraft/world/level/block/state/properties/AttachFace#values` | method | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#HORIZONTAL_FACING` | field | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#ORIENTATION` | field | 3 |
| `net/minecraft/world/level/chunk/LevelChunk#getBlockEntities` | method | 2 |
| `net/minecraft/world/level/chunk/LevelChunk#getPos` | method | 2 |
| `net/minecraft/world/level/gameevent/GameEvent#BLOCK_CHANGE` | field | 3 |
| `net/minecraft/world/level/gameevent/GameEvent$Context#of` | method | 3 |
| `net/minecraft/world/level/gamerules/GameRules#LIMITED_CRAFTING` | field | 1 |
| `net/minecraft/world/level/gamerules/GameRules#get` | method | 1 |
| `net/minecraft/world/level/levelgen/WorldOptions#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/presets/WorldPreset#createWorldDimensions` | method | 1 |
| `net/minecraft/world/level/levelgen/presets/WorldPresets#FLAT` | field | 1 |
| `net/minecraft/world/level/material/FluidState#is` | method | 1 |
| `net/minecraft/world/level/material/MapColor#col` | field | 1 |
| `net/minecraft/world/level/saveddata/SavedData#<init>` | method | 1 |
| `net/minecraft/world/level/saveddata/SavedDataType#<init>` | method | 1 |
| `net/minecraft/world/level/saveddata/maps/MapItemSavedData#locked` | field | 1 |
| `net/minecraft/world/level/storage/LevelStorageSource#levelExists` | method | 1 |
| `net/minecraft/world/level/storage/SavedDataStorage#computeIfAbsent` | method | 1 |
| `net/minecraft/world/level/storage/TagValueInput#create` | method | 1 |
| `net/minecraft/world/level/storage/TagValueOutput#buildResult` | method | 1 |
| `net/minecraft/world/level/storage/TagValueOutput#createWithContext` | method | 1 |
| `net/minecraft/world/phys/AABB#<init>` | method | 12 |
| `net/minecraft/world/phys/AABB#clip` | method | 1 |
| `net/minecraft/world/phys/AABB#contains` | method | 2 |
| `net/minecraft/world/phys/AABB#expandTowards` | method | 2 |
| `net/minecraft/world/phys/AABB#inflate` | method | 9 |
| `net/minecraft/world/phys/AABB#maxY` | field | 1 |
| `net/minecraft/world/phys/AABB#minX` | field | 1 |
| `net/minecraft/world/phys/AABB#minZ` | field | 1 |
| `net/minecraft/world/phys/AABB#move` | method | 3 |
| `net/minecraft/world/phys/BlockHitResult#<init>` | method | 2 |
| `net/minecraft/world/phys/BlockHitResult#getBlockPos` | method | 5 |
| `net/minecraft/world/phys/BlockHitResult#getDirection` | method | 1 |
| `net/minecraft/world/phys/BlockHitResult#getLocation` | method | 3 |
| `net/minecraft/world/phys/BlockHitResult#getType` | method | 5 |
| `net/minecraft/world/phys/HitResult$Type#BLOCK` | field | 2 |
| `net/minecraft/world/phys/HitResult$Type#MISS` | field | 3 |
| `net/minecraft/world/phys/Vec2#<init>` | method | 1 |
| `net/minecraft/world/phys/Vec2#ZERO` | field | 1 |
| `net/minecraft/world/phys/Vec3#<init>` | method | 43 |
| `net/minecraft/world/phys/Vec3#ZERO` | field | 9 |
| `net/minecraft/world/phys/Vec3#add` | method | 77 |
| `net/minecraft/world/phys/Vec3#atBottomCenterOf` | method | 6 |
| `net/minecraft/world/phys/Vec3#atCenterOf` | method | 14 |
| `net/minecraft/world/phys/Vec3#atLowerCornerOf` | method | 15 |
| `net/minecraft/world/phys/Vec3#cross` | method | 10 |
| `net/minecraft/world/phys/Vec3#distanceTo` | method | 6 |
| `net/minecraft/world/phys/Vec3#distanceToSqr` | method | 15 |
| `net/minecraft/world/phys/Vec3#dot` | method | 21 |
| `net/minecraft/world/phys/Vec3#length` | method | 5 |
| `net/minecraft/world/phys/Vec3#lengthSqr` | method | 6 |
| `net/minecraft/world/phys/Vec3#scale` | method | 40 |
| `net/minecraft/world/phys/Vec3#subtract` | method | 41 |
| `net/minecraft/world/phys/Vec3#x` | field | 44 |
| `net/minecraft/world/phys/Vec3#y` | field | 45 |
| `net/minecraft/world/phys/Vec3#z` | field | 44 |
| `net/minecraft/world/phys/shapes/Shapes#create` | method | 1 |
| `net/minecraft/world/phys/shapes/VoxelShape#bounds` | method | 1 |
| `net/minecraft/world/phys/shapes/VoxelShape#isEmpty` | method | 2 |
| `net/minecraft/world/phys/shapes/VoxelShape#max` | method | 1 |
| `net/minecraft/world/phys/shapes/VoxelShape#toAabbs` | method | 1 |
| `punchy/client/render/CameraAnimationState#isActive` | method | 1 |

</details>

