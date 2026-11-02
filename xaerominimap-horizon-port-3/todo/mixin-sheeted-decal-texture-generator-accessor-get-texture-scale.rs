// Not ported yet: the Java @Accessor on SheetedDecalTextureGenerator.textureScale (MixinSheetedDecalTextureGeneratorAccessor.getTextureScale).
// Why it's off: No Horizon function mapped for SheetedDecalTextureGenerator.textureScale (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for SheetedDecalTextureGenerator.textureScale: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SheetedDecalTextureGenerator.textureScale -> (not mapped yet)
//
// Decompiled Java, xaero/common/mixin/MixinSheetedDecalTextureGeneratorAccessor.java:9:
// | @Accessor
// | float getTextureScale();
