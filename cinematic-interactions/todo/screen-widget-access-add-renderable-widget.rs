// Not ported yet: the Java @Invoker on Screen.addRenderableWidget (ScreenWidgetAccess.cinematic$addRenderableWidget).
// Why it's off: No Horizon function mapped for Screen.addRenderableWidget (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for Screen.addRenderableWidget: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Screen.addRenderableWidget -> (not mapped yet)
//
// Decompiled Java, dev/dyslexic/cinematic/mixin/ScreenWidgetAccess.java:12:
// | @Invoker("addRenderableWidget")
// | <T extends GuiEventListener & Renderable & NarratableEntry> T cinematic$addRenderableWidget(T var1);
