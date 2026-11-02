// Not ported yet: the Java @Accessor on CraftingMenu.access (CraftingMenuAccess.cinematic$access).
// Why it's off: No Horizon function mapped for CraftingMenu.access (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for CraftingMenu.access: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   CraftingMenu.access -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/CraftingMenuAccess.java:10:
// | @Accessor("access")
// | ContainerLevelAccess cinematic$access();
