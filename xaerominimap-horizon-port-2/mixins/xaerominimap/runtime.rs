// Xaero's Minimap, ported to Horizon by horizon-port from xaerominimap-fabric-26.3-26.6.2.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_xaerominimap). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "xaerominimap";
pub const NAME: &str = "Xaero's Minimap";
pub const VERSION: &str = "26.6.2";
pub const DESCRIPTION: &str = "The most vanilla-looking minimap for Minecraft.";

/// The mod's keys for Options > Mods: name, label, default key.
pub const KEYS: &[[&str; 3]] = &[];

/// The mod's own Java that doesn't need the game, translated to Rust by horizon-port.
/// Each Java method is here whole or not at all; the ones left out say why.
#[allow(dead_code, unused_variables, unused_mut, unused_parens, unused_imports, unreachable_code,
    non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
pub mod java {
    use std::sync::{LazyLock, Mutex};

    /// A Java static that changes: one value behind a lock.
    pub struct JCell<T>(Mutex<T>);
    impl<T: Clone> JCell<T> {
        pub const fn new(v: T) -> Self {
            Self(Mutex::new(v))
        }
        pub fn get(&self) -> T {
            self.0.lock().unwrap_or_else(|e| e.into_inner()).clone()
        }
        pub fn set(&self, v: T) {
            *self.0.lock().unwrap_or_else(|e| e.into_inner()) = v;
        }
        pub fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            f(&mut self.0.lock().unwrap_or_else(|e| e.into_inner()))
        }
    }

    /// Java's HashMap, LinkedHashMap and TreeMap. Keeps insertion order (LinkedHashMap's; HashMap's
    /// order is unspecified), or key order when made with `new_sorted` (TreeMap's).
    #[derive(Clone, Debug)]
    pub struct JMap<K, V> {
        index: std::collections::HashMap<K, usize>,
        entries: Vec<(K, V)>,
        sorted: bool,
    }
    /// Java's Map.equals: the same keys with equal values, in any order.
    impl<K: std::hash::Hash + Eq, V: PartialEq> PartialEq for JMap<K, V> {
        fn eq(&self, o: &Self) -> bool {
            self.entries.len() == o.entries.len()
                && self.entries.iter().all(|(k, v)| o.index.get(k).is_some_and(|&i| o.entries[i].1 == *v))
        }
    }
    impl<K, V> Default for JMap<K, V> {
        fn default() -> Self {
            Self { index: std::collections::HashMap::new(), entries: Vec::new(), sorted: false }
        }
    }
    impl<K: std::hash::Hash + Ord + Clone, V> JMap<K, V> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn new_sorted() -> Self {
            Self { sorted: true, ..Self::default() }
        }
        pub fn from_iter(pairs: impl IntoIterator<Item = (K, V)>) -> Self {
            let mut m = Self::new();
            for (k, v) in pairs {
                m.insert(k, v);
            }
            m
        }
        pub fn from_iter_sorted(pairs: impl IntoIterator<Item = (K, V)>) -> Self {
            let mut m = Self::new_sorted();
            for (k, v) in pairs {
                m.insert(k, v);
            }
            m
        }
        pub fn len(&self) -> usize {
            self.entries.len()
        }
        pub fn is_empty(&self) -> bool {
            self.entries.is_empty()
        }
        pub fn clear(&mut self) {
            self.index.clear();
            self.entries.clear();
        }
        pub fn contains_key<Q: std::hash::Hash + Eq + ?Sized>(&self, k: &Q) -> bool
        where
            K: std::borrow::Borrow<Q>,
        {
            self.index.contains_key(k)
        }
        pub fn get<Q: std::hash::Hash + Eq + ?Sized>(&self, k: &Q) -> Option<&V>
        where
            K: std::borrow::Borrow<Q>,
        {
            self.index.get(k).map(|&i| &self.entries[i].1)
        }
        fn reindex(&mut self) {
            self.index.clear();
            for (i, (k, _)) in self.entries.iter().enumerate() {
                self.index.insert(k.clone(), i);
            }
        }
        fn slot(&mut self, k: K, v: V) -> usize {
            if self.sorted {
                let at = self.entries.partition_point(|e| e.0 < k);
                self.entries.insert(at, (k, v));
                self.reindex();
                at
            } else {
                self.index.insert(k.clone(), self.entries.len());
                self.entries.push((k, v));
                self.entries.len() - 1
            }
        }
        pub fn insert(&mut self, k: K, v: V) -> Option<V> {
            if let Some(&i) = self.index.get(&k) {
                return Some(std::mem::replace(&mut self.entries[i].1, v));
            }
            self.slot(k, v);
            None
        }
        pub fn remove<Q: std::hash::Hash + Eq + ?Sized>(&mut self, k: &Q) -> Option<V>
        where
            K: std::borrow::Borrow<Q>,
        {
            let i = *self.index.get(k)?;
            let (_, v) = self.entries.remove(i);
            self.reindex();
            Some(v)
        }
        /// computeIfAbsent and putIfAbsent: the value, made first when the key is missing.
        pub fn or_insert_with(&mut self, k: K, f: impl FnOnce() -> V) -> &mut V {
            let i = match self.index.get(&k) {
                Some(&i) => i,
                None => self.slot(k, f()),
            };
            &mut self.entries[i].1
        }
        pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
            self.entries.iter().map(|(k, v)| (k, v))
        }
        pub fn keys(&self) -> impl Iterator<Item = &K> {
            self.entries.iter().map(|e| &e.0)
        }
        pub fn values(&self) -> impl Iterator<Item = &V> {
            self.entries.iter().map(|e| &e.1)
        }
        pub fn clone_pairs(&self) -> Vec<(K, V)>
        where
            V: Clone,
        {
            self.entries.clone()
        }
    }

    /// Java's HashSet, LinkedHashSet and TreeSet, kept like JMap's keys.
    #[derive(Clone, Debug)]
    pub struct JSet<T>(JMap<T, ()>);
    impl<T: std::hash::Hash + Eq> PartialEq for JSet<T> {
        fn eq(&self, o: &Self) -> bool {
            self.0 == o.0
        }
    }
    impl<T> Default for JSet<T> {
        fn default() -> Self {
            Self(JMap::default())
        }
    }
    impl<T: std::hash::Hash + Ord + Clone> JSet<T> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn new_sorted() -> Self {
            Self(JMap::new_sorted())
        }
        pub fn from_iter(items: impl IntoIterator<Item = T>) -> Self {
            let mut s = Self::new();
            for v in items {
                s.insert(v);
            }
            s
        }
        pub fn from_iter_sorted(items: impl IntoIterator<Item = T>) -> Self {
            let mut s = Self::new_sorted();
            for v in items {
                s.insert(v);
            }
            s
        }
        pub fn len(&self) -> usize {
            self.0.len()
        }
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
        pub fn clear(&mut self) {
            self.0.clear();
        }
        /// True when it wasn't there yet, as Java's add.
        pub fn insert(&mut self, v: T) -> bool {
            self.0.insert(v, ()).is_none()
        }
        pub fn contains<Q: std::hash::Hash + Eq + ?Sized>(&self, v: &Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
        {
            self.0.contains_key(v)
        }
        pub fn remove<Q: std::hash::Hash + Eq + ?Sized>(&mut self, v: &Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
        {
            self.0.remove(v).is_some()
        }
        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.0.keys()
        }
        pub fn clone_pairs(&self) -> Vec<T> {
            self.0.keys().cloned().collect()
        }
    }

    /// The mod's data folder in Horizon, set by the hooks (game_side.rs `ensure_init`) before the mod's code runs.
    pub static J_DATA_DIR: JCell<Option<std::path::PathBuf>> = JCell::new(None);
    /// FabricLoader's config folder: the mod's data folder in Horizon (./config until the game sets it).
    pub fn j_config_dir() -> std::path::PathBuf {
        J_DATA_DIR.get().unwrap_or_else(|| std::path::PathBuf::from("config"))
    }

    /// Minecraft's RandomSource / java.util.Random: other numbers than Java's, the same ranges.
    #[derive(Clone, Debug)]
    pub struct JRandom(u64);
    impl JRandom {
        pub fn new() -> Self {
            let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(0x9E37_79B9);
            Self(t | 1)
        }
        fn next(&mut self) -> u64 {
            let mut v = self.0;
            v ^= v << 13;
            v ^= v >> 7;
            v ^= v << 17;
            self.0 = v;
            v
        }
        /// 0 inclusive to n exclusive; Java throws on n <= 0.
        pub fn next_int(&mut self, n: i32) -> i32 {
            assert!(n > 0, "bound must be positive");
            (self.next() % n as u64) as i32
        }
        pub fn next_int_between(&mut self, lo: i32, hi: i32) -> i32 {
            lo + self.next_int(hi - lo)
        }
        pub fn next_float(&mut self) -> f32 {
            (self.next() >> 40) as f32 / (1u64 << 24) as f32
        }
        pub fn next_double(&mut self) -> f64 {
            (self.next() >> 11) as f64 / (1u64 << 53) as f64
        }
        pub fn next_boolean(&mut self) -> bool {
            self.next() & 1 == 1
        }
        pub fn next_gaussian(&mut self) -> f64 {
            let (u, v) = (self.next_double().max(1e-12), self.next_double());
            (-2.0 * u.ln()).sqrt() * (std::f64::consts::TAU * v).cos()
        }
    }

    /// Minecraft's Identifier (`namespace:path`), which is only text.
    #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct JIdentifier {
        pub namespace: String,
        pub path: String,
    }
    impl JIdentifier {
        pub fn new(namespace: &str, path: &str) -> Self {
            Self { namespace: namespace.to_string(), path: path.to_string() }
        }
        /// `a:b`, or `minecraft:b` without a namespace.
        pub fn parse(s: &str) -> Self {
            match s.split_once(':') {
                Some((n, p)) => Self::new(n, p),
                None => Self::new("minecraft", s),
            }
        }
        /// Identifier.tryParse: None (null) when it has characters an identifier can't.
        pub fn try_parse(s: &str) -> Option<Self> {
            let id = Self::parse(s);
            Self::try_build(&id.namespace, &id.path)
        }
        pub fn try_build(namespace: &str, path: &str) -> Option<Self> {
            let ns_ok = namespace.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-'));
            let path_ok = path.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-' | '/'));
            if ns_ok && path_ok { Some(Self::new(namespace, path)) } else { None }
        }
    }
    impl std::fmt::Display for JIdentifier {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}:{}", self.namespace, self.path)
        }
    }

    /// Java's String.equals.
    pub fn j_eq(a: &str, b: &str) -> bool {
        a == b
    }
    pub fn j_eq_ignore_case(a: &str, b: &str) -> bool {
        a.to_lowercase() == b.to_lowercase()
    }
    /// Java's String.length (in chars).
    pub fn j_len(s: &str) -> i32 {
        s.chars().count() as i32
    }
    pub fn j_char_at(s: &str, i: i32) -> char {
        s.chars().nth(i as usize).expect("charAt out of range")
    }
    /// Java's substring, by char.
    pub fn j_substring(s: &str, from: i32, to: Option<i32>) -> String {
        let n = s.chars().count() as i32;
        let to = to.unwrap_or(n);
        assert!(0 <= from && from <= to && to <= n, "substring out of range");
        s.chars().skip(from as usize).take((to - from) as usize).collect()
    }
    pub fn j_index_of(s: &str, sub: &str) -> i32 {
        s.find(sub).map_or(-1, |b| s[..b].chars().count() as i32)
    }
    pub fn j_index_of_char(s: &str, c: char) -> i32 {
        s.chars().position(|x| x == c).map_or(-1, |i| i as i32)
    }
    pub fn j_last_index_of(s: &str, sub: &str) -> i32 {
        s.rfind(sub).map_or(-1, |b| s[..b].chars().count() as i32)
    }
    /// Java's String.split on plain text: trailing empty parts dropped.
    pub fn j_split(s: &str, sep: &str) -> Vec<String> {
        let mut v: Vec<String> = s.split(sep).map(str::to_string).collect();
        while v.len() > 1 && v.last().is_some_and(|p| p.is_empty()) {
            v.pop();
        }
        v
    }
    /// Java's String.split(sep, limit) on plain text.
    pub fn j_split_limit(s: &str, sep: &str, limit: i32) -> Vec<String> {
        if limit == 0 {
            return j_split(s, sep);
        }
        if limit > 0 {
            return s.splitn(limit as usize, sep).map(str::to_string).collect();
        }
        s.split(sep).map(str::to_string).collect()
    }
    /// Java's String.compareTo: by UTF-16 unit, then by length.
    pub fn j_compare(a: &str, b: &str) -> i32 {
        let (x, y): (Vec<u16>, Vec<u16>) = (a.encode_utf16().collect(), b.encode_utf16().collect());
        for (p, q) in x.iter().zip(y.iter()) {
            if p != q {
                return *p as i32 - *q as i32;
            }
        }
        x.len() as i32 - y.len() as i32
    }
    /// Java's String.regionMatches, by char.
    pub fn j_region_matches(s: &str, ignore_case: bool, from: i32, other: &str, other_from: i32, len: i32) -> bool {
        let (a, b): (Vec<char>, Vec<char>) = (s.chars().collect(), other.chars().collect());
        if from < 0 || other_from < 0 || from as i64 + len as i64 > a.len() as i64 || other_from as i64 + len as i64 > b.len() as i64 {
            return false;
        }
        (0..len.max(0) as usize).all(|i| {
            let (p, q) = (a[from as usize + i], b[other_from as usize + i]);
            p == q || ignore_case && (j_upper_char(p) == j_upper_char(q) || j_lower_char(p) == j_lower_char(q))
        })
    }
    /// Java's Character.toLowerCase / toUpperCase (one char).
    pub fn j_lower_char(c: char) -> char {
        let mut l = c.to_lowercase();
        if l.len() == 1 { l.next().unwrap_or(c) } else { c }
    }
    pub fn j_upper_char(c: char) -> char {
        let mut u = c.to_uppercase();
        if u.len() == 1 { u.next().unwrap_or(c) } else { c }
    }
    /// Java's Arrays.copyOfRange: past the end is filled with defaults.
    pub fn j_copy_of_range<T: Clone + Default>(a: &[T], from: usize, to: usize) -> Vec<T> {
        assert!(from <= to && from <= a.len(), "copyOfRange out of range");
        let mut v: Vec<T> = a[from..to.min(a.len())].to_vec();
        v.resize(to - from, T::default());
        v
    }
    pub fn j_hash(s: &str) -> i32 {
        s.encode_utf16().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32))
    }
    /// Java's Boolean.parseBoolean.
    pub fn j_parse_bool(s: &str) -> bool {
        s.eq_ignore_ascii_case("true")
    }
    /// A float as Java prints it in "..." + x (1.0, not 1).
    pub fn j_float(x: f64) -> String {
        if x.is_finite() && x == x.trunc() && x.abs() < 1e7 { format!("{x:.1}") } else { format!("{x}") }
    }
    /// Java's Math.round.
    pub fn j_round_f(x: f32) -> i32 {
        (x + 0.5).floor() as i32
    }
    pub fn j_round_d(x: f64) -> i64 {
        (x + 0.5).floor() as i64
    }
    /// Java's Math.clamp / Mth.clamp (no panic when min > max).
    pub fn j_clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
        if v < min { min } else if v > max { max } else { v }
    }
    pub fn j_lerp<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>>(d: T, a: T, b: T) -> T {
        a + d * (b - a)
    }
    pub fn j_signum_f(x: f32) -> f32 {
        if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { x }
    }
    pub fn j_signum_d(x: f64) -> f64 {
        if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { x }
    }
    pub fn j_wrap_degrees(x: f64) -> f64 {
        let mut v = x % 360.0;
        if v >= 180.0 {
            v -= 360.0;
        }
        if v < -180.0 {
            v += 360.0;
        }
        v
    }
    pub fn j_frac(x: f64) -> f64 {
        x - x.floor()
    }
    pub fn j_compare_f(a: f64, b: f64) -> i32 {
        a.partial_cmp(&b).map_or(0, |o| o as i32)
    }
    pub fn j_copy_of<T: Clone + Default>(a: &[T], n: usize) -> Vec<T> {
        let mut v: Vec<T> = a.iter().take(n).cloned().collect();
        v.resize(n, T::default());
        v
    }
    /// Java's System.currentTimeMillis.
    pub fn j_millis() -> i64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
    }
    /// Java's System.nanoTime: only differences mean anything.
    pub fn j_nanos() -> i64 {
        static START: LazyLock<std::time::Instant> = LazyLock::new(std::time::Instant::now);
        START.elapsed().as_nanos() as i64
    }

    /// `MultiplyAnimationHelper` (xaero/common/anim/MultiplyAnimationHelper.java)
    pub mod multiply_animation_helper {
        use super::*;
        pub static CURRENT_TICK: LazyLock<JCell<i64>> = LazyLock::new(|| JCell::new(j_millis()));
        // lastTick: not translated, its value reads other changing statics.
        pub const STEP_TIME: f64 = 16.666666666666668f64;
        #[derive(Clone, Debug)]
        pub struct MultiplyAnimationHelper {
        }
        impl MultiplyAnimationHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MultiplyAnimationHelper.tick(0): not translated, it uses MultiplyAnimationHelper.lastTick, which isn't translated (its value reads other changing statics).
        // MultiplyAnimationHelper.animate(2): not translated, it uses MultiplyAnimationHelper.lastTick, which isn't translated (its value reads other changing statics).
    }

    /// `BlockStateShortShapeCache` (xaero/common/cache/BlockStateShortShapeCache.java)
    pub mod block_state_short_shape_cache {
        use super::*;
        // The class itself isn't translated: it uses IXaeroMinimap, which isn't translated (it's an interface). Its static parts are below.
        // BlockStateShortShapeCache.BlockStateShortShapeCache(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // BlockStateShortShapeCache.isShort(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // BlockStateShortShapeCache.reset(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
    }

    /// `PlaceholderBlockGetter` (xaero/common/cache/placeholder/PlaceholderBlockGetter.java)
    pub mod placeholder_block_getter {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockGetter`. Its static parts are below.
        // PlaceholderBlockGetter.setPlaceholderState(1): not translated, its class isn't translated (it implements Minecraft's `BlockGetter`).
        // PlaceholderBlockGetter.getBlockEntity(1): not translated, its class isn't translated (it implements Minecraft's `BlockGetter`).
        // PlaceholderBlockGetter.getBlockState(1): not translated, its class isn't translated (it implements Minecraft's `BlockGetter`).
        // PlaceholderBlockGetter.getFluidState(1): not translated, its class isn't translated (it implements Minecraft's `BlockGetter`).
        // PlaceholderBlockGetter.getHeight(0): not translated, its class isn't translated (it implements Minecraft's `BlockGetter`).
        // PlaceholderBlockGetter.getMinY(0): not translated, its class isn't translated (it implements Minecraft's `BlockGetter`).
    }

    /// `LegacyCommonConfigInit` (xaero/common/config/LegacyCommonConfigInit.java)
    pub mod legacy_common_config_init {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LegacyCommonConfigInit {
        }
        impl LegacyCommonConfigInit {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // LegacyCommonConfigInit.init(2): not translated, it uses IXaeroMinimap, which isn't translated (it's an interface).
        }
    }

    /// `LegacyCommonConfigIO` (xaero/common/config/LegacyCommonConfigIO.java)
    pub mod legacy_common_config_io {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LegacyCommonConfigIO {
            pub config_file_path: std::path::PathBuf,
            pub allow_cave_mode_on_server: bool,
            pub allow_nether_cave_mode_on_server: bool,
            pub should_enable_everyone_tracks_everyone: bool,
        }
        impl LegacyCommonConfigIO {
            pub fn new(config_file_path: &std::path::Path) -> Self {
                let mut s = Self { config_file_path: config_file_path.to_path_buf(), allow_cave_mode_on_server: false, allow_nether_cave_mode_on_server: false, should_enable_everyone_tracks_everyone: false };
                s.config_file_path = config_file_path.to_path_buf();
                s
            }
            // LegacyCommonConfigIO.load(0): not translated, it uses `ConfigProfile` (xaero.lib.common.config.profile).
            // LegacyCommonConfigIO.readLine(1): not translated, it uses HudMod, which isn't translated (it implements `IXaeroMinimap`).
            pub fn should_enable_everyone_tracks_everyone(&self) -> bool {
                return self.should_enable_everyone_tracks_everyone;
            }
        }
    }

    /// `IBufferSource` (xaero/common/core/IBufferSource.java)
    pub mod i_buffer_source {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IBufferSource.getXaero_lastRenderType(0): not translated, it has no body.
        // IBufferSource.setXaero_lastRenderType(1): not translated, it has no body.
    }

    /// `IGuiGraphics` (xaero/common/core/IGuiGraphics.java)
    pub mod i_gui_graphics {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IGuiGraphics.xaero_mm_getGuiRenderState(0): not translated, it has no body.
    }

    /// `IGuiRenderer` (xaero/common/core/IGuiRenderer.java)
    pub mod i_gui_renderer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IGuiRenderer.xaero_mm_getGuiProjection(0): not translated, it has no body.
        // IGuiRenderer.xaero_mm_getGuiProjectionMatrixBuffer(0): not translated, it has no body.
    }

    /// `IPictureInPictureRenderer` (xaero/common/core/IPictureInPictureRenderer.java)
    pub mod i_picture_in_picture_renderer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IPictureInPictureRenderer.xaero_mm_getTexture(0): not translated, it has no body.
        // IPictureInPictureRenderer.xaero_mm_getTextureView(0): not translated, it has no body.
        // IPictureInPictureRenderer.xaero_mm_getDepthTextureView(0): not translated, it has no body.
        // IPictureInPictureRenderer.xaero_mm_prepareTexturesAndProjection(3): not translated, it has no body.
    }

    /// `IXaeroMinimapClientPlayNetHandler` (xaero/common/core/IXaeroMinimapClientPlayNetHandler.java)
    pub mod i_xaero_minimap_client_play_net_handler {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IXaeroMinimapClientPlayNetHandler.getXaero_minimapSession(0): not translated, it has no body.
        // IXaeroMinimapClientPlayNetHandler.setXaero_minimapSession(1): not translated, it has no body.
    }

    /// `IXaeroMinimapGameMessageS2CPacket` (xaero/common/core/IXaeroMinimapGameMessageS2CPacket.java)
    pub mod i_xaero_minimap_game_message_s2_c_packet {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IXaeroMinimapGameMessageS2CPacket.setMessage(1): not translated, it has no body.
    }

    /// `IXaeroMinimapMinecraftClient` (xaero/common/core/IXaeroMinimapMinecraftClient.java)
    pub mod i_xaero_minimap_minecraft_client {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IXaeroMinimapMinecraftClient.getXaeroMinimap_fps(0): not translated, it has no body.
    }

    /// `IXaeroMinimapSMultiBlockChangePacket` (xaero/common/core/IXaeroMinimapSMultiBlockChangePacket.java)
    pub mod i_xaero_minimap_s_multi_block_change_packet {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IXaeroMinimapSMultiBlockChangePacket.xaero_mm_getSectionPos(0): not translated, it has no body.
    }

    /// `XaeroMinimapCore` (xaero/common/core/XaeroMinimapCore.java)
    pub mod xaero_minimap_core {
        use super::*;
        // modMain: not translated, it uses IXaeroMinimap, which isn't translated (it's an interface).
        // chunkCleanField: not translated, it uses `Field` (java.lang.reflect).
        // currentSession: not translated, it uses XaeroMinimapSession, which isn't translated (it extends `HudSession` (xaero.hud)).
        pub static WAYPOINTS_PROJECTION: JCell<JMat4> = JCell::new(JMat4::IDENTITY);
        pub static WAYPOINT_MODEL_VIEW: JCell<JMat4> = JCell::new(JMat4::IDENTITY);
        #[derive(Clone, Debug)]
        pub struct XaeroMinimapCore {
        }
        impl XaeroMinimapCore {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // XaeroMinimapCore.ensureField(0): not translated, it uses XaeroMinimapCore.chunkCleanField, which isn't translated (it uses `Field` (java.lang.reflect)).
        // XaeroMinimapCore.chunkUpdateCallback(2): not translated, it calls XaeroMinimapCore.ensureField, which isn't translated.
        // XaeroMinimapCore.onHandleLevelChunkWithLight(1): not translated, it uses Minecraft's `ClientboundLevelChunkWithLightPacket`.
        // XaeroMinimapCore.onHandleLightUpdatePacket(1): not translated, it uses Minecraft's `ClientboundLightUpdatePacket`.
        // XaeroMinimapCore.onQueueLightRemoval(1): not translated, it uses Minecraft's `ClientboundForgetLevelChunkPacket`.
        // XaeroMinimapCore.onBlockChange(1): not translated, it uses Minecraft's `ClientboundBlockUpdatePacket`.
        // XaeroMinimapCore.onMultiBlockChange(1): not translated, it uses Minecraft's `ClientboundSectionBlocksUpdatePacket`.
        // XaeroMinimapCore.onPlayNetHandler(2): not translated, it uses Minecraft's `ClientPacketListener`.
        // XaeroMinimapCore.cleanupCurrentSession(0): not translated, a try/finally.
        // XaeroMinimapCore.onPlayNetHandlerCleanup(1): not translated, it uses Minecraft's `ClientPacketListener`.
        // XaeroMinimapCore.beforeRespawn(1): not translated, it uses Minecraft's `Player`.
        pub fn on_projection_matrix(matrix_in: JMat4) {
            { let joml_0 = JMat4::IDENTITY; xaero_minimap_core::WAYPOINTS_PROJECTION.set(joml_0); joml_0 };
            { let joml_1 = xaero_minimap_core::WAYPOINTS_PROJECTION.get().mul(matrix_in); xaero_minimap_core::WAYPOINTS_PROJECTION.set(joml_1); joml_1 };
        }
        pub fn on_world_model_view_matrix(matrix: JMat4) {
            { let joml_0 = JMat4::IDENTITY; xaero_minimap_core::WAYPOINT_MODEL_VIEW.set(joml_0); joml_0 };
            { let joml_1 = xaero_minimap_core::WAYPOINT_MODEL_VIEW.get().mul(matrix); xaero_minimap_core::WAYPOINT_MODEL_VIEW.set(joml_1); joml_1 };
        }
        pub fn on_render_level_matrices(world_model_view_matrix: JMat4, projection_matrix: JMat4) {
            xaero_minimap_core::on_projection_matrix(projection_matrix);
            xaero_minimap_core::on_world_model_view_matrix(world_model_view_matrix);
        }
        // XaeroMinimapCore.beforeIngameGuiRender(0): not translated, it uses HudMod, which isn't translated (it implements `IXaeroMinimap`).
        // XaeroMinimapCore.afterIngameGuiRender(0): not translated, it uses HudMod, which isn't translated (it implements `IXaeroMinimap`).
        // XaeroMinimapCore.onRenderStatusEffectOverlayPost(1): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // XaeroMinimapCore.onBossHealthRender(1): not translated, it uses IBossHealthPushBox, which isn't translated (it's an interface).
        // XaeroMinimapCore.onEntityIconsModelPrepareDetection(1): not translated, it uses Minecraft's `Submit`.
        // XaeroMinimapCore.onEntityIconsModelRenderDetection(3): not translated, it uses Minecraft's `Model`.
        // XaeroMinimapCore.onEntityIconsModelPartRenderDetection(2): not translated, it uses Minecraft's `ModelPart`.
        // XaeroMinimapCore.onDeleteWorld(1): not translated, it uses Minecraft's `LevelStorageAccess`.
        // XaeroMinimapCore.onSpawn(1): not translated, it uses Minecraft's `ClientboundSetDefaultSpawnPositionPacket`.
        // XaeroMinimapCore.onLocalPlayerCommand(1): not translated, it uses XaeroMinimapCore.modMain, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // XaeroMinimapCore.onSystemChat(1): not translated, it uses Minecraft's `Component`.
        // XaeroMinimapCore.onHandleDisguisedChatMessage(2): not translated, it uses Minecraft's `Bound`.
        // XaeroMinimapCore.isModLoaded(0): not translated, it uses XaeroMinimapCore.modMain, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // XaeroMinimapCore.onRenderStatusEffectOverlay(1): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // XaeroMinimapCore.onRenderCrosshair(1): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // XaeroMinimapCore.handleRenderModOverlay(2): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // XaeroMinimapCore.onBufferSourceGetBuffer(2): not translated, it uses IBufferSource, which isn't translated (it's an interface).
        // XaeroMinimapCore.onToggleKeyIsDown(1): not translated, it uses Minecraft's `ToggleKeyMapping`.
    }

    /// `XaeroMinimapCoreFabric` (xaero/common/core/XaeroMinimapCoreFabric.java)
    pub mod xaero_minimap_core_fabric {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct XaeroMinimapCoreFabric {
        }
        impl XaeroMinimapCoreFabric {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `Effects` (xaero/common/effect/Effects.java)
    pub mod effects {
        use super::*;
        // NO_MINIMAP_UNHELD: not translated, it uses Minecraft's `MobEffect`.
        // NO_MINIMAP_HARMFUL_UNHELD: not translated, it uses Minecraft's `MobEffect`.
        // NO_RADAR_UNHELD: not translated, it uses Minecraft's `MobEffect`.
        // NO_RADAR_HARMFUL_UNHELD: not translated, it uses Minecraft's `MobEffect`.
        // NO_WAYPOINTS_UNHELD: not translated, it uses Minecraft's `MobEffect`.
        // NO_WAYPOINTS_HARMFUL_UNHELD: not translated, it uses Minecraft's `MobEffect`.
        // NO_CAVE_MAPS_UNHELD: not translated, it uses Minecraft's `MobEffect`.
        // NO_CAVE_MAPS_HARMFUL_UNHELD: not translated, it uses Minecraft's `MobEffect`.
        // NO_MINIMAP: not translated, it uses Minecraft's `Holder`.
        // NO_MINIMAP_HARMFUL: not translated, it uses Minecraft's `Holder`.
        // NO_RADAR: not translated, it uses Minecraft's `Holder`.
        // NO_RADAR_HARMFUL: not translated, it uses Minecraft's `Holder`.
        // NO_WAYPOINTS: not translated, it uses Minecraft's `Holder`.
        // NO_WAYPOINTS_HARMFUL: not translated, it uses Minecraft's `Holder`.
        // NO_CAVE_MAPS: not translated, it uses Minecraft's `Holder`.
        // NO_CAVE_MAPS_HARMFUL: not translated, it uses Minecraft's `Holder`.
        #[derive(Clone, Debug)]
        pub struct Effects {
        }
        impl Effects {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Effects.init(0): not translated, it uses Effects.NO_MINIMAP, which isn't translated (it uses Minecraft's `Holder`).
    }

    /// `EffectsRegister` (xaero/common/effect/EffectsRegister.java)
    pub mod effects_register {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EffectsRegister {
        }
        impl EffectsRegister {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // EffectsRegister.registerEffects(1): not translated, it uses MinimapStatusEffect, which isn't translated (it extends Minecraft's `MobEffect`).
        }
    }

    /// `MinimapStatusEffect` (xaero/common/effect/MinimapStatusEffect.java)
    pub mod minimap_status_effect {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MobEffect`. Its static parts are below.
        // MinimapStatusEffect.MinimapStatusEffect(3): not translated, its class isn't translated (it extends Minecraft's `MobEffect`).
        // MinimapStatusEffect.setRegistryName(1): not translated, its class isn't translated (it extends Minecraft's `MobEffect`).
        // MinimapStatusEffect.getRegistryName(0): not translated, its class isn't translated (it extends Minecraft's `MobEffect`).
    }

    /// `NoCaveMapsEffect` (xaero/common/effect/NoCaveMapsEffect.java)
    pub mod no_cave_maps_effect {
        use super::*;
        // The class itself isn't translated: it extends `MinimapStatusEffect`. Its static parts are below.
        // NoCaveMapsEffect.NoCaveMapsEffect(1): not translated, its class isn't translated (it extends `MinimapStatusEffect`).
    }

    /// `NoMinimapEffect` (xaero/common/effect/NoMinimapEffect.java)
    pub mod no_minimap_effect {
        use super::*;
        // The class itself isn't translated: it extends `MinimapStatusEffect`. Its static parts are below.
        // NoMinimapEffect.NoMinimapEffect(1): not translated, its class isn't translated (it extends `MinimapStatusEffect`).
    }

    /// `NoRadarEffect` (xaero/common/effect/NoRadarEffect.java)
    pub mod no_radar_effect {
        use super::*;
        // The class itself isn't translated: it extends `MinimapStatusEffect`. Its static parts are below.
        // NoRadarEffect.NoRadarEffect(1): not translated, its class isn't translated (it extends `MinimapStatusEffect`).
    }

    /// `NoWaypointsEffect` (xaero/common/effect/NoWaypointsEffect.java)
    pub mod no_waypoints_effect {
        use super::*;
        // The class itself isn't translated: it extends `MinimapStatusEffect`. Its static parts are below.
        // NoWaypointsEffect.NoWaypointsEffect(1): not translated, its class isn't translated (it extends `MinimapStatusEffect`).
    }

    /// `ClientEvents` (xaero/common/events/ClientEvents.java)
    pub mod client_events {
        use super::*;
        // The class itself isn't translated: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`). Its static parts are below.
        // ClientEvents.ClientEvents(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ClientEvents.handleGuiOpen(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ClientEvents.handleRenderGameOverlayEventPre(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ClientEvents.handleRenderGameOverlayEventPost(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ClientEvents.handleClientSendChatEvent(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ClientEvents.handleClientPlayerChatReceivedEvent(3): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ClientEvents.handleClientSystemChatReceivedEvent(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ClientEvents.handleChatMessage(2): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ClientEvents.handleDrawScreenEventPost(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ClientEvents.handlePlayerSetSpawnEvent(2): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ClientEvents.getLastGuiOpen(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ClientEvents.worldUnload(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ClientEvents.handleClientTickStart(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ClientEvents.handlePlayerTickStart(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ClientEvents.handleRenderTickStart(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ClientEvents.handleRenderStatusEffectOverlay(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ClientEvents.handleRenderCrosshairOverlay(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ClientEvents.handleForceToggleKeyMapping(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ClientEvents.handleTextureStitchEventPost(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ClientEvents.handleTextureStitchEventPost_onReset(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
    }

    /// `ClientEventsFabric` (xaero/common/events/ClientEventsFabric.java)
    pub mod client_events_fabric {
        use super::*;
        // The class itself isn't translated: it extends `ClientEvents`. Its static parts are below.
        // ClientEventsFabric.ClientEventsFabric(1): not translated, its class isn't translated (it extends `ClientEvents`).
        // ClientEventsFabric.handleTextureStitchEventPost_onReset(0): not translated, its class isn't translated (it extends `ClientEvents`).
    }

    /// `ClientEventsListener` (xaero/common/events/ClientEventsListener.java)
    pub mod client_events_listener {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientEventsListener {
        }
        impl ClientEventsListener {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // ClientEventsListener.playerTickPost(1): not translated, it uses HudSession, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
            // ClientEventsListener.clientTickPost(1): not translated, it uses HudSession, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
            // ClientEventsListener.handleRenderStatusEffectOverlay(1): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
            // ClientEventsListener.handleForceToggleKeyMapping(1): not translated, it uses Minecraft's `ToggleKeyMapping`.
        }
    }

    /// `CommonEvents` (xaero/common/events/CommonEvents.java)
    pub mod common_events {
        use super::*;
        // The class itself isn't translated: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`). Its static parts are below.
        // CommonEvents.CommonEvents(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // CommonEvents.onPlayerClone(3): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // CommonEvents.onServerStarting(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // CommonEvents.onServerStopped(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // CommonEvents.onPlayerLogIn(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // CommonEvents.onPlayerWorldJoin(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // CommonEvents.handlePlayerTickStart(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
    }

    /// `CommonEventsFabric` (xaero/common/events/CommonEventsFabric.java)
    pub mod common_events_fabric {
        use super::*;
        // The class itself isn't translated: it extends `CommonEvents`. Its static parts are below.
        // CommonEventsFabric.CommonEventsFabric(1): not translated, its class isn't translated (it extends `CommonEvents`).
        // CommonEventsFabric.register(0): not translated, its class isn't translated (it extends `CommonEvents`).
    }

    /// `ModClientEvents` (xaero/common/events/ModClientEvents.java)
    pub mod mod_client_events {
        use super::*;
        // The class itself isn't translated: it uses IXaeroMinimap, which isn't translated (it's an interface). Its static parts are below.
        // ModClientEvents.ModClientEvents(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModClientEvents.handleRenderModOverlay(2): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
    }

    /// `ModClientEventsFabric` (xaero/common/events/ModClientEventsFabric.java)
    pub mod mod_client_events_fabric {
        use super::*;
        // The class itself isn't translated: it extends `ModClientEvents`. Its static parts are below.
        // ModClientEventsFabric.ModClientEventsFabric(1): not translated, its class isn't translated (it extends `ModClientEvents`).
        // ModClientEventsFabric.register(0): not translated, its class isn't translated (it extends `ModClientEvents`).
    }

    /// `ModCommonEvents` (xaero/common/events/ModCommonEvents.java)
    pub mod mod_common_events {
        use super::*;
        // The class itself isn't translated: it uses IXaeroMinimap, which isn't translated (it's an interface). Its static parts are below.
        // ModCommonEvents.ModCommonEvents(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
    }

    /// `ModCommonEventsFabric` (xaero/common/events/ModCommonEventsFabric.java)
    pub mod mod_common_events_fabric {
        use super::*;
        // The class itself isn't translated: it extends `ModCommonEvents`. Its static parts are below.
        // ModCommonEventsFabric.ModCommonEventsFabric(1): not translated, its class isn't translated (it extends `ModCommonEvents`).
    }

    /// `SilentException` (xaero/common/exception/SilentException.java)
    pub mod silent_exception {
        use super::*;
        // The class itself isn't translated: it extends `Exception`. Its static parts are below.
        pub const SERIAL_VERSION_UID: i64 = 1i64;
        // SilentException.SilentException(1): not translated, its class isn't translated (it extends `Exception`).
    }

    /// `SimpleBackup` (xaero/common/file/SimpleBackup.java)
    pub mod simple_backup {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SimpleBackup {
        }
        impl SimpleBackup {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SimpleBackup.moveToBackup(1): not translated, it calls SimpleBackup.moveToBackup, which isn't translated.
        // SimpleBackup.moveToBackup(2): not translated, a catch block that uses its exception.
        // SimpleBackup.copyDirectoryWithContents(4): not translated, its body couldn't be read: an anonymous class.
    }

    /// `CustomRenderTypes` (xaero/common/graphics/CustomRenderTypes.java)
    pub mod custom_render_types {
        use super::*;
        // GUI_NEAREST: not translated, it uses Minecraft's `RenderType`.
        // GUI_NEAREST_PRE: not translated, it uses Minecraft's `RenderType`.
        // GUI_BILINEAR: not translated, it uses Minecraft's `RenderType`.
        // GUI_BILINEAR_PRE: not translated, it uses Minecraft's `RenderType`.
        // GUI_BILINEAR_NO_DEPTH: not translated, it uses Minecraft's `RenderType`.
        // COLORED_WAYPOINTS_BGS: not translated, it uses Minecraft's `RenderType`.
        // MAP_CHUNK_OVERLAY: not translated, it uses Minecraft's `RenderType`.
        // MAP_LINES: not translated, it uses Minecraft's `RenderType`.
        // MINIMAP_WORLD_MAP: not translated, it uses Minecraft's `RenderType`.
        // MINIMAP_WORLD_MAP_ZOOM: not translated, it uses Minecraft's `RenderType`.
        // MINIMAP: not translated, it uses Minecraft's `RenderType`.
        // MINIMAP_ZOOM: not translated, it uses Minecraft's `RenderType`.
        // RADAR_NAME_BGS: not translated, it uses Minecraft's `RenderType`.
        // DEPTH_CLEAR: not translated, it uses Minecraft's `RenderType`.
        #[derive(Clone, Debug)]
        pub struct CustomRenderTypes {
        }
        impl CustomRenderTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CustomRenderTypes.entityIconRenderType(7): not translated, it uses Minecraft's `RenderType`.
        // CustomRenderTypes.applyFixedOrder(0): not translated, it uses `XaeroBufferProvider` (xaero.lib.client.graphics).
    }

    /// `ImprovedFramebuffer` (xaero/common/graphics/ImprovedFramebuffer.java)
    pub mod improved_framebuffer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `TextureTarget`. Its static parts are below.
        // MAIN_RENDER_TARGET_FIELD: not translated, it uses `Field` (java.lang.reflect).
        // mainRenderTargetBackup: not translated, it uses Minecraft's `RenderTarget`.
        // outputColorTextureOverrideBU: not translated, it uses Minecraft's `GpuTextureView`.
        // outputDepthTextureOverrideBU: not translated, it uses Minecraft's `GpuTextureView`.
        // ImprovedFramebuffer.ImprovedFramebuffer(4): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.bindDefaultFramebuffer(1): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.generateMipmaps(0): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.forceAsMainRenderTarget(0): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        pub fn restore_main_render_target() {
            // (`if` always false in Horizon: only the else part)
        }
        // ImprovedFramebuffer.bindAsMainTarget(1): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.setColorTexture(2): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.setColorTexture(1): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.setDepthTexture(2): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.getSampler(0): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.setSampler(1): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.closeColorTexture(0): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.closeDepthTexture(0): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.getTas(0): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.getFormat(0): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
    }

    /// `MinimapTexture` (xaero/common/graphics/MinimapTexture.java)
    pub mod minimap_texture {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SimpleTexture`. Its static parts are below.
        // MinimapTexture.loadIfNeeded(0): not translated, its class isn't translated (it extends Minecraft's `SimpleTexture`).
        // MinimapTexture.MinimapTexture(1): not translated, its class isn't translated (it extends Minecraft's `SimpleTexture`).
        // MinimapTexture.load(1): not translated, its class isn't translated (it extends Minecraft's `SimpleTexture`).
        // MinimapTexture.getTextureAndView(0): not translated, its class isn't translated (it extends Minecraft's `SimpleTexture`).
        // MinimapTexture.getTas(0): not translated, its class isn't translated (it extends Minecraft's `SimpleTexture`).
    }

    /// `PixelBuffers` (xaero/common/graphics/PixelBuffers.java)
    pub mod pixel_buffers {
        use super::*;
        pub static BUFFERS_TYPE: JCell<i32> = JCell::new(0);
        #[derive(Clone, Debug)]
        pub struct PixelBuffers {
        }
        impl PixelBuffers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // PixelBuffers.glGenBuffers(0): not translated, it calls `GL15` (org.lwjgl.opengl).glGenBuffers.
        // PixelBuffers.glBindBuffer(2): not translated, it calls `GL15` (org.lwjgl.opengl).glBindBuffer.
        // PixelBuffers.glBufferData(3): not translated, it calls `GL15` (org.lwjgl.opengl).glBufferData.
        // PixelBuffers.glMapBuffer(4): not translated, it uses `ByteBuffer` (java.nio).
        // PixelBuffers.glUnmapBuffer(1): not translated, it calls `GL15` (org.lwjgl.opengl).glUnmapBuffer.
        // PixelBuffers.glDeleteBuffers(1): not translated, it's overloaded with the same number of parameters.
        // PixelBuffers.glDeleteBuffers(1): not translated, it's overloaded with the same number of parameters.
        // PixelBuffers.glMapBuffer(2): not translated, it uses `ByteBuffer` (java.nio).
    }

    /// `MultiTextureRenderTypeRenderer` (xaero/common/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java)
    pub mod multi_texture_render_type_renderer {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ByteBufferBuilder`. Its static parts are below.
        pub const RENDER_PASS_NAME: &str = "xaero multitexture render pass";
        // MultiTextureRenderTypeRenderer.MultiTextureRenderTypeRenderer(0): not translated, its class isn't translated (it uses Minecraft's `ByteBufferBuilder`).
        // MultiTextureRenderTypeRenderer.init(3): not translated, its class isn't translated (it uses Minecraft's `ByteBufferBuilder`).
        // MultiTextureRenderTypeRenderer.draw(0): not translated, its class isn't translated (it uses Minecraft's `ByteBufferBuilder`).
        // MultiTextureRenderTypeRenderer.uploadImmediateVertexBuffer(1): not translated, its class isn't translated (it uses Minecraft's `ByteBufferBuilder`).
        // MultiTextureRenderTypeRenderer.endBuffer(1): not translated, its class isn't translated (it uses Minecraft's `ByteBufferBuilder`).
        // MultiTextureRenderTypeRenderer.begin(1): not translated, its class isn't translated (it uses Minecraft's `ByteBufferBuilder`).
    }

    /// `MultiTextureRenderTypeRenderer.DrawCall` (xaero/common/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java)
    pub mod multi_texture_render_type_renderer_draw_call {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `GpuBufferSlice`. Its static parts are below.
        // MultiTextureRenderTypeRenderer.DrawCall.DrawCall(5): not translated, its class isn't translated (it uses Minecraft's `GpuBufferSlice`).
    }

    /// `MultiTextureRenderTypeRendererProvider` (xaero/common/graphics/renderer/multitexture/MultiTextureRenderTypeRendererProvider.java)
    pub mod multi_texture_render_type_renderer_provider {
        use super::*;
        // The class itself isn't translated: it uses `Deque` (java.util). Its static parts are below.
        // MultiTextureRenderTypeRendererProvider.MultiTextureRenderTypeRendererProvider(1): not translated, its class isn't translated (it uses `Deque` (java.util)).
        // MultiTextureRenderTypeRendererProvider.getRenderer(1): not translated, its class isn't translated (it uses `Deque` (java.util)).
        // MultiTextureRenderTypeRendererProvider.getRenderer(2): not translated, its class isn't translated (it uses `Deque` (java.util)).
        // MultiTextureRenderTypeRendererProvider.getRenderer(2): not translated, its class isn't translated (it uses `Deque` (java.util)).
        // MultiTextureRenderTypeRendererProvider.getRenderer(3): not translated, its class isn't translated (it uses `Deque` (java.util)).
        // MultiTextureRenderTypeRendererProvider.draw(1): not translated, its class isn't translated (it uses `Deque` (java.util)).
    }

    /// `CachedComponentSupplier` (xaero/common/gui/component/CachedComponentSupplier.java)
    pub mod cached_component_supplier {
        use super::*;
        // The class itself isn't translated: it uses `Object`. Its static parts are below.
        // CachedComponentSupplier.CachedComponentSupplier(1): not translated, its class isn't translated (it uses `Object`).
        // CachedComponentSupplier.get(1): not translated, its class isn't translated (it uses `Object`).
    }

    /// `GuiAddWaypoint` (xaero/common/gui/GuiAddWaypoint.java)
    pub mod gui_add_waypoint {
        use super::*;
        // The class itself isn't translated: it extends `ScreenBase` (xaero.lib.client.gui). Its static parts are below.
        // VISIBILITY_TYPE_TOOLTIP: not translated, it uses `Tooltip` (xaero.lib.client.gui.widget).
        // TYPE_TOOLTIP: not translated, it uses `Tooltip` (xaero.lib.client.gui.widget).
        // GuiAddWaypoint.GuiAddWaypoint(15): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.GuiAddWaypoint(9): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.GuiAddWaypoint(7): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.fillFormWaypoint(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.getDimDiv(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.getAutomaticX(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.getAutomaticYInput(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.getAutomaticZ(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.fillFormAutomatic(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.createForms(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.resetCurrentForm(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.updateMutual(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.confirmMutual(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.differentValues(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.createColorOptions(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.getMutualEditConfig(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.setMutualEditConfig(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.init(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.setInitialFocus(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.applyEditBoxResponder(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.createSetsDropdown(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.createContainersDropdown(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.createWorldsDropdown(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.getDisableButtonText(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.getCurrent(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.keyPressed(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.charTyped(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.mouseClicked(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.mouseReleased(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.preType(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.postType(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.setFocused(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.canConfirm(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.updateConfirmButton(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.handleCoordinateInputSpaces(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.checkFields(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.tick(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.extractBackground(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.extractRenderState(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.renderPreDropdown(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiAddWaypoint.onSelected(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
    }

    /// `GuiChoosePreset` (xaero/common/gui/GuiChoosePreset.java)
    pub mod gui_choose_preset {
        use super::*;
        // The class itself isn't translated: it extends `GuiSettings` (xaero.lib.client.gui). Its static parts are below.
        // GuiChoosePreset.GuiChoosePreset(3): not translated, its class isn't translated (it extends `GuiSettings` (xaero.lib.client.gui)).
        // GuiChoosePreset.goBack(0): not translated, its class isn't translated (it extends `GuiSettings` (xaero.lib.client.gui)).
    }

    /// `GuiClearSet` (xaero/common/gui/GuiClearSet.java)
    pub mod gui_clear_set {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ConfirmScreen`. Its static parts are below.
        // GuiClearSet.GuiClearSet(7): not translated, its class isn't translated (it extends Minecraft's `ConfirmScreen`).
        // GuiClearSet.confirmClearSet(7): not translated, it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)).
    }

    /// `GuiDeleteSet` (xaero/common/gui/GuiDeleteSet.java)
    pub mod gui_delete_set {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ConfirmScreen`. Its static parts are below.
        // GuiDeleteSet.GuiDeleteSet(7): not translated, its class isn't translated (it extends Minecraft's `ConfirmScreen`).
        // GuiDeleteSet.confirmDeleteSet(7): not translated, it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)).
    }

    /// `GuiDropdownHelper` (xaero/common/gui/GuiDropdownHelper.java)
    pub mod gui_dropdown_helper {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiDropdownHelper.getCurrentKey(0): not translated, its class isn't translated (it's generic).
        // GuiDropdownHelper.getCurrentName(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiEditMode` (xaero/common/gui/GuiEditMode.java)
    pub mod gui_edit_mode {
        use super::*;
        // The class itself isn't translated: it extends `ScreenBase` (xaero.lib.client.gui). Its static parts are below.
        // CENTERED_COMPONENT: a game value, so it's in game_side.rs.
        // FLIPPED_COMPONENT: a game value, so it's in game_side.rs.
        // TRUE_COMPONENT: a game value, so it's in game_side.rs.
        // FALSE_COMPONENT: a game value, so it's in game_side.rs.
        // PRESS_C_COMPONENT: a game value, so it's in game_side.rs.
        // PRESS_F_COMPONENT: a game value, so it's in game_side.rs.
        // NOT_INGAME: a game value, so it's in game_side.rs.
        // GuiEditMode.GuiEditMode(5): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiEditMode.init(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiEditMode.confirm(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiEditMode.cancel(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiEditMode.extractBackground(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiEditMode.applyPushes(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiEditMode.extractRenderState(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiEditMode.renderModuleBox(4): not translated, it's generic.
        // GuiEditMode.getHoveredModule(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiEditMode.isHovered(3): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiEditMode.mouseClicked(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiEditMode.mouseReleased(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiEditMode.keyPressed(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiEditMode.handleDraggedModule(3): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiEditMode.getTooltipText(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
    }

    /// `GuiEntityRadarCategoryEditor` (xaero/common/gui/GuiEntityRadarCategoryEditor.java)
    pub mod gui_entity_radar_category_editor {
        use super::*;
        // The class itself isn't translated: it extends `GuiCategoryEditor` (xaero.hud.category.ui). Its static parts are below.
        // SERVER_ENFORCED_COMPONENT: a game value, so it's in game_side.rs.
        // GuiEntityRadarCategoryEditor.GuiEntityRadarCategoryEditor(5): not translated, its class isn't translated (it extends `GuiCategoryEditor` (xaero.hud.category.ui)).
        // GuiEntityRadarCategoryEditor.constructEditorData(1): not translated, its class isn't translated (it extends `GuiCategoryEditor` (xaero.hud.category.ui)).
        // GuiEntityRadarCategoryEditor.constructDefaultData(1): not translated, its class isn't translated (it extends `GuiCategoryEditor` (xaero.hud.category.ui)).
        // GuiEntityRadarCategoryEditor.onConfigConfirmed(1): not translated, its class isn't translated (it extends `GuiCategoryEditor` (xaero.hud.category.ui)).
        // GuiEntityRadarCategoryEditor.extractRenderState(4): not translated, its class isn't translated (it extends `GuiCategoryEditor` (xaero.hud.category.ui)).
    }

    /// `GuiEntityRadarSettings` (xaero/common/gui/GuiEntityRadarSettings.java)
    pub mod gui_entity_radar_settings {
        use super::*;
        // The class itself isn't translated: it extends `GuiMinimapSettings`. Its static parts are below.
        // GuiEntityRadarSettings.GuiEntityRadarSettings(4): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
    }

    /// `GuiHelper` (xaero/common/gui/GuiHelper.java)
    pub mod gui_helper {
        use super::*;
        // The class itself isn't translated: it uses IXaeroMinimap, which isn't translated (it's an interface). Its static parts are below.
        // GuiHelper.GuiHelper(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // GuiHelper.getMinimapSettingsFromScreen(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // GuiHelper.openMinimapSettingsFromScreen(2): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // GuiHelper.getMainSettingsScreen(1): not translated, it has no body.
        // GuiHelper.openMainSettingsFromScreen(1): not translated, it has no body.
    }

    /// `GuiInfoDisplayEdit` (xaero/common/gui/GuiInfoDisplayEdit.java)
    pub mod gui_info_display_edit {
        use super::*;
        // The class itself isn't translated: it extends `ScreenBase` (xaero.lib.client.gui). Its static parts are below.
        pub const FRAME_TOP_SIZE: i32 = 30;
        pub const FRAME_BOTTOM_SIZE: i32 = 61;
        pub const SELECTION_ITEM_HEIGHT: i32 = 24;
        // HELP_COMPONENT: a game value, so it's in game_side.rs.
        // SERVER_ENFORCED_COMPONENT: a game value, so it's in game_side.rs.
        // GuiInfoDisplayEdit.GuiInfoDisplayEdit(5): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiInfoDisplayEdit.init(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiInfoDisplayEdit.createEntryFor(1): not translated, it's generic.
        // GuiInfoDisplayEdit.refreshEntry(1): not translated, it's generic.
        // GuiInfoDisplayEdit.onExit(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiInfoDisplayEdit.saveToConfigProfile(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiInfoDisplayEdit.encodeState(1): not translated, it's generic.
        // GuiInfoDisplayEdit.createColorOptions(3): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiInfoDisplayEdit.addSubElements(1): not translated, it's generic.
        // GuiInfoDisplayEdit.mouseReleased(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiInfoDisplayEdit.setFocused(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiInfoDisplayEdit.extractBackground(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
    }

    /// `GuiInfoDisplayEdit.MoveableEntry` (xaero/common/gui/GuiInfoDisplayEdit.java)
    pub mod gui_info_display_edit_moveable_entry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // GuiInfoDisplayEdit.MoveableEntry.MoveableEntry(1): not translated, its class isn't translated (it's generic).
        // GuiInfoDisplayEdit.MoveableEntry.addSubElement(1): not translated, its class isn't translated (it's generic).
        // GuiInfoDisplayEdit.MoveableEntry.getState(0): not translated, its class isn't translated (it's generic).
        // GuiInfoDisplayEdit.MoveableEntry.setState(1): not translated, its class isn't translated (it's generic).
        // GuiInfoDisplayEdit.MoveableEntry.clearSubElements(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiInfoDisplayEdit.SelectionList` (xaero/common/gui/GuiInfoDisplayEdit.java)
    pub mod gui_info_display_edit_selection_list {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ObjectSelectionList`. Its static parts are below.
        // USAGE_NARRATION: a game value, so it's in game_side.rs.
        // LEFT_RIGHT_USAGE: a game value, so it's in game_side.rs.
        // GuiInfoDisplayEdit.SelectionList.SelectionList(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiInfoDisplayEdit.SelectionList.isFocused(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiInfoDisplayEdit.SelectionList.setFocused(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiInfoDisplayEdit.SelectionList.setSelected(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiInfoDisplayEdit.SelectionList.updateWidgetNarration(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiInfoDisplayEdit.SelectionList.createEntries(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiInfoDisplayEdit.SelectionList.releaseDrag(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiInfoDisplayEdit.SelectionList.scrollBarX(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiInfoDisplayEdit.SelectionList.getRowWidth(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiInfoDisplayEdit.SelectionList.extractWidgetRenderState(4): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiInfoDisplayEdit.SelectionList.nextFocusPath(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
    }

    /// `GuiInfoDisplayEdit.SelectionList.Entry` (xaero/common/gui/GuiInfoDisplayEdit.java)
    pub mod gui_info_display_edit_selection_list_entry {
        use super::*;
        // The class itself isn't translated: it extends `Entry`. Its static parts are below.
        // GuiInfoDisplayEdit.SelectionList.Entry.Entry(1): not translated, its class isn't translated (it extends `Entry`).
        // GuiInfoDisplayEdit.SelectionList.Entry.renderNonInteractable(3): not translated, its class isn't translated (it extends `Entry`).
        // GuiInfoDisplayEdit.SelectionList.Entry.getMoveable(0): not translated, its class isn't translated (it extends `Entry`).
        // GuiInfoDisplayEdit.SelectionList.Entry.extractContent(5): not translated, its class isn't translated (it extends `Entry`).
        // GuiInfoDisplayEdit.SelectionList.Entry.mouseClicked(2): not translated, its class isn't translated (it extends `Entry`).
        // GuiInfoDisplayEdit.SelectionList.Entry.mouseReleased(1): not translated, its class isn't translated (it extends `Entry`).
        // GuiInfoDisplayEdit.SelectionList.Entry.mouseMoved(2): not translated, its class isn't translated (it extends `Entry`).
        // GuiInfoDisplayEdit.SelectionList.Entry.mouseDragged(3): not translated, its class isn't translated (it extends `Entry`).
        // GuiInfoDisplayEdit.SelectionList.Entry.mouseScrolled(4): not translated, its class isn't translated (it extends `Entry`).
        // GuiInfoDisplayEdit.SelectionList.Entry.keyPressed(1): not translated, its class isn't translated (it extends `Entry`).
        // GuiInfoDisplayEdit.SelectionList.Entry.keyReleased(1): not translated, its class isn't translated (it extends `Entry`).
        // GuiInfoDisplayEdit.SelectionList.Entry.charTyped(1): not translated, its class isn't translated (it extends `Entry`).
        // GuiInfoDisplayEdit.SelectionList.Entry.updateNarration(1): not translated, its class isn't translated (it extends `Entry`).
        // GuiInfoDisplayEdit.SelectionList.Entry.getNarration(0): not translated, its class isn't translated (it extends `Entry`).
    }

    /// `GuiInstructions` (xaero/common/gui/GuiInstructions.java)
    pub mod gui_instructions {
        use super::*;
        // The class itself isn't translated: it extends `ScreenBase` (xaero.lib.client.gui). Its static parts are below.
        // GuiInstructions.GuiInstructions(3): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiInstructions.init(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiInstructions.extractBackground(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
    }

    /// `GuiLightOverlay` (xaero/common/gui/GuiLightOverlay.java)
    pub mod gui_light_overlay {
        use super::*;
        // The class itself isn't translated: it extends `GuiMinimapSettings`. Its static parts are below.
        // GuiLightOverlay.GuiLightOverlay(4): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
    }

    /// `GuiMinimapBlockMapSettings` (xaero/common/gui/GuiMinimapBlockMapSettings.java)
    pub mod gui_minimap_block_map_settings {
        use super::*;
        // The class itself isn't translated: it extends `GuiMinimapSettings`. Its static parts are below.
        // GuiMinimapBlockMapSettings.GuiMinimapBlockMapSettings(4): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
    }

    /// `GuiMinimapInfoSettings` (xaero/common/gui/GuiMinimapInfoSettings.java)
    pub mod gui_minimap_info_settings {
        use super::*;
        // The class itself isn't translated: it extends `GuiMinimapSettings`. Its static parts are below.
        // GuiMinimapInfoSettings.GuiMinimapInfoSettings(4): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
    }

    /// `GuiMinimapMain` (xaero/common/gui/GuiMinimapMain.java)
    pub mod gui_minimap_main {
        use super::*;
        // The class itself isn't translated: it extends `GuiMinimapSettings`. Its static parts are below.
        // GuiMinimapMain.GuiMinimapMain(1): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
        // GuiMinimapMain.GuiMinimapMain(5): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
        // GuiMinimapMain.resetConfirmResult(1): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
        // GuiMinimapMain.updateSearchableEntries(0): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
        // GuiMinimapMain.init(0): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
    }

    /// `GuiMinimapMiscSettings` (xaero/common/gui/GuiMinimapMiscSettings.java)
    pub mod gui_minimap_misc_settings {
        use super::*;
        // The class itself isn't translated: it extends `GuiMinimapSettings`. Its static parts are below.
        // GuiMinimapMiscSettings.GuiMinimapMiscSettings(3): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
    }

    /// `GuiMinimapOverlaysSettings` (xaero/common/gui/GuiMinimapOverlaysSettings.java)
    pub mod gui_minimap_overlays_settings {
        use super::*;
        // The class itself isn't translated: it extends `GuiMinimapSettings`. Its static parts are below.
        // GuiMinimapOverlaysSettings.GuiMinimapOverlaysSettings(4): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
    }

    /// `GuiMinimapSettings` (xaero/common/gui/GuiMinimapSettings.java)
    pub mod gui_minimap_settings {
        use super::*;
        // The class itself isn't translated: it extends `EditConfigScreen` (xaero.lib.client.gui.config). Its static parts are below.
        // GuiMinimapSettings.GuiMinimapSettings(4): not translated, its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)).
        // GuiMinimapSettings.onEditedProfileSwitch(0): not translated, its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)).
        // GuiMinimapSettings.extractRenderState(4): not translated, its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)).
        // GuiMinimapSettings.handleChanges(0): not translated, its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)).
        // GuiMinimapSettings.saveEditedCategoryIfNeeded(0): not translated, its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)).
        // GuiMinimapSettings.keyPressed(1): not translated, its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)).
    }

    /// `GuiMinimapViewSettings` (xaero/common/gui/GuiMinimapViewSettings.java)
    pub mod gui_minimap_view_settings {
        use super::*;
        // The class itself isn't translated: it extends `GuiMinimapSettings`. Its static parts are below.
        // GuiMinimapViewSettings.GuiMinimapViewSettings(4): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
    }

    /// `GuiMisc` (xaero/common/gui/GuiMisc.java)
    pub mod gui_misc {
        use super::*;
        pub static WP_DISTANCE_FORMAT_PRECISION: JCell<i32> = JCell::new(1);
        // wpDistanceFormat: not translated, it uses `DecimalFormat` (java.text).
        #[derive(Clone, Debug)]
        pub struct GuiMisc {
        }
        impl GuiMisc {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // GuiMisc.getFormat(1): not translated, it uses `DecimalFormat` (java.text).
    }

    /// `GuiNewSet` (xaero/common/gui/GuiNewSet.java)
    pub mod gui_new_set {
        use super::*;
        // The class itself isn't translated: it extends `ScreenBase` (xaero.lib.client.gui). Its static parts are below.
        // GuiNewSet.GuiNewSet(5): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiNewSet.init(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiNewSet.setInitialFocus(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiNewSet.canConfirm(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiNewSet.updateConfirmButton(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiNewSet.keyPressed(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiNewSet.tick(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiNewSet.extractBackground(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
    }

    /// `GuiReset` (xaero/common/gui/GuiReset.java)
    pub mod gui_reset {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ConfirmScreen`. Its static parts are below.
        // GuiReset.GuiReset(3): not translated, its class isn't translated (it extends Minecraft's `ConfirmScreen`).
    }

    /// `GuiSlimeSeed` (xaero/common/gui/GuiSlimeSeed.java)
    pub mod gui_slime_seed {
        use super::*;
        // The class itself isn't translated: it extends `GuiMinimapSettings`. Its static parts are below.
        // GuiSlimeSeed.GuiSlimeSeed(5): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
        // GuiSlimeSeed.init(0): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
        // GuiSlimeSeed.extractRenderState(4): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
        // GuiSlimeSeed.tick(0): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
        // GuiSlimeSeed.keyPressed(1): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
        // GuiSlimeSeed.charTyped(1): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
        // GuiSlimeSeed.updateSlimeSeed(0): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
    }

    /// `GuiTransfer` (xaero/common/gui/GuiTransfer.java)
    pub mod gui_transfer {
        use super::*;
        // The class itself isn't translated: it extends `ScreenBase` (xaero.lib.client.gui). Its static parts are below.
        // GuiTransfer.GuiTransfer(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiTransfer.init(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiTransfer.createWorlds1DD(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiTransfer.createWorlds2DD(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiTransfer.createContainers1DD(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiTransfer.createContainers2DD(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiTransfer.openParent(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiTransfer.transfer(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiTransfer.renderPreDropdown(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiTransfer.onSelected(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
    }

    /// `GuiWaypointContainers` (xaero/common/gui/GuiWaypointContainers.java)
    pub mod gui_waypoint_containers {
        use super::*;
        // The class itself isn't translated: it extends `GuiDropdownHelper`. Its static parts are below.
        // GuiWaypointContainers.GuiWaypointContainers(4): not translated, its class isn't translated (it extends `GuiDropdownHelper`).
    }

    /// `GuiWaypoints` (xaero/common/gui/GuiWaypoints.java)
    pub mod gui_waypoints {
        use super::*;
        // The class itself isn't translated: it extends `ScreenBase` (xaero.lib.client.gui). Its static parts are below.
        pub const FRAME_TOP_SIZE: i32 = 58;
        pub const FRAME_BOTTOM_SIZE: i32 = 61;
        pub static DISTANCE_DIVIDED: JCell<f64> = JCell::new(0.0f64);
        pub static SHOWING_THIRD_PARTY_DELETED: JCell<bool> = JCell::new(false);
        // GuiWaypoints.GuiWaypoints(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.init(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.setDisabled(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.createSetsDropdown(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.createContainersDropdown(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.createWorldsDropdown(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.getSelectedWaypointsList(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.allWaypointsAre(2): not translated, it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // GuiWaypoints.shouldDeleteSet(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.undrag(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.mouseClicked(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.mouseReleased(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.keyReleased(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.extractBackground(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.extractRenderState(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.renderHoveredWaypointDistance(3): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.renderPreDropdown(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.updateButtons(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.isAddEditEnabled(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.isSomethingSelected(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.isOneSelected(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.clearSelection(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.canTeleport(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.onSelected(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.updateSortedList(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.getSortObject(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.isFiltering(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.getWaypointFilterMatch(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.onFilterTyped(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.keyPressed(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.clearFilter(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypoints.selectWaypoints(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
    }

    /// `GuiWaypoints.List` (xaero/common/gui/GuiWaypoints.java)
    pub mod gui_waypoints_list {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ObjectSelectionList`. Its static parts are below.
        // GuiWaypoints.List.List(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiWaypoints.List.getThirdPartyCount(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiWaypoints.List.getThirdPartyWaypoint(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiWaypoints.List.getWaypointCount(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiWaypoints.List.getWaypoint(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiWaypoints.List.entriesCanBeSelected(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiWaypoints.List.isSelectedItem(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiWaypoints.List.createEntries(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiWaypoints.List.extractWidgetRenderState(4): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiWaypoints.List.extractItem(5): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiWaypoints.List.isFocused(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiWaypoints.List.setSelected(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiWaypoints.List.getItemHeight(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiWaypoints.List.drawWaypointSlot(4): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiWaypoints.List.getEntryAt(2): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiWaypoints.List.ensureVisible(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiWaypoints.List.recreateEntries(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiWaypoints.List.mouseClicked(2): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
    }

    /// `GuiWaypoints.List.WaypointEntry` (xaero/common/gui/GuiWaypoints.java)
    pub mod gui_waypoints_list_waypoint_entry {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Entry`. Its static parts are below.
        // GuiWaypoints.List.WaypointEntry.WaypointEntry(1): not translated, its class isn't translated (it extends Minecraft's `Entry`).
        // GuiWaypoints.List.WaypointEntry.extractContent(5): not translated, its class isn't translated (it extends Minecraft's `Entry`).
        // GuiWaypoints.List.WaypointEntry.mouseClicked(2): not translated, its class isn't translated (it extends Minecraft's `Entry`).
        // GuiWaypoints.List.WaypointEntry.getNarration(0): not translated, its class isn't translated (it extends Minecraft's `Entry`).
    }

    /// `GuiWaypointSets` (xaero/common/gui/GuiWaypointSets.java)
    pub mod gui_waypoint_sets {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct GuiWaypointSets {
            pub current_set: i32,
            pub options: Option<Vec<String>>,
        }
        impl GuiWaypointSets {
            // GuiWaypointSets.GuiWaypointSets(2): not translated, it stores a second reference to an object (Rust would need it copied or shared).
            // GuiWaypointSets.GuiWaypointSets(3): not translated, it uses `KeySortableByOther` (xaero.lib.common.util).
            pub fn get_current_set(&self) -> i32 {
                return self.current_set;
            }
            pub fn get_current_set_key(&self) -> String {
                return ((self.options.as_ref().expect("null"))[self.current_set as usize]).to_string();
            }
            pub fn set_current_set(&mut self, current_set: i32) {
                self.current_set = current_set;
            }
            pub fn get_options(&self) -> Option<Vec<String>> {
                return self.options.clone();
            }
        }
    }

    /// `GuiWaypointSettings` (xaero/common/gui/GuiWaypointSettings.java)
    pub mod gui_waypoint_settings {
        use super::*;
        // The class itself isn't translated: it extends `GuiMinimapSettings`. Its static parts are below.
        // GuiWaypointSettings.GuiWaypointSettings(4): not translated, its class isn't translated (it extends `GuiMinimapSettings`).
    }

    /// `GuiWaypointsOptions` (xaero/common/gui/GuiWaypointsOptions.java)
    pub mod gui_waypoints_options {
        use super::*;
        // The class itself isn't translated: it extends `ScreenBase` (xaero.lib.client.gui). Its static parts are below.
        // GuiWaypointsOptions.GuiWaypointsOptions(6): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypointsOptions.init(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypointsOptions.getConfigButtonName(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypointsOptions.onMultiWorldButton(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypointsOptions.onTeleportationButton(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypointsOptions.onSortButton(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypointsOptions.onSortReverseButton(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypointsOptions.onShowThirdPartyDeleted(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypointsOptions.getThirdPartyDeletedMessage(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypointsOptions.mouseClicked(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypointsOptions.actionPerformed(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypointsOptions.confirmResult(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypointsOptions.multiplyWaypoints(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypointsOptions.isDifferentRootContainer(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypointsOptions.isDifferentSubWorld(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypointsOptions.extractBackground(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypointsOptions.extractRenderState(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWaypointsOptions.isInGameUi(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
    }

    /// `GuiWaypointWorlds` (xaero/common/gui/GuiWaypointWorlds.java)
    pub mod gui_waypoint_worlds {
        use super::*;
        // The class itself isn't translated: it extends `GuiDropdownHelper`. Its static parts are below.
        // GuiWaypointWorlds.GuiWaypointWorlds(4): not translated, its class isn't translated (it extends `GuiDropdownHelper`).
        // GuiWaypointWorlds.addWorlds(7): not translated, its class isn't translated (it extends `GuiDropdownHelper`).
    }

    /// `GuiWorldTpCommand` (xaero/common/gui/GuiWorldTpCommand.java)
    pub mod gui_world_tp_command {
        use super::*;
        // The class itself isn't translated: it extends `ScreenBase` (xaero.lib.client.gui). Its static parts are below.
        // GuiWorldTpCommand.GuiWorldTpCommand(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWorldTpCommand.init(0): not translated, its body couldn't be read: an anonymous class.
        // GuiWorldTpCommand.extractBackground(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWorldTpCommand.extractRenderState(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWorldTpCommand.tick(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWorldTpCommand.keyPressed(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiWorldTpCommand.isInGameUi(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
    }

    /// `HudPresetSettingEntry` (xaero/common/gui/HudPresetSettingEntry.java)
    pub mod hud_preset_setting_entry {
        use super::*;
        // The class itself isn't translated: it implements `ISettingEntry` (xaero.lib.client.gui). Its static parts are below.
        // HudPresetSettingEntry.HudPresetSettingEntry(2): not translated, its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)).
        // HudPresetSettingEntry.getStringForSearch(0): not translated, its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)).
        // HudPresetSettingEntry.createWidget(3): not translated, its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)).
    }

    /// `IXaeroConfirmScreenCallback` (xaero/common/gui/IXaeroConfirmScreenCallback.java)
    pub mod i_xaero_confirm_screen_callback {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IXaeroConfirmScreenCallback.accept(1): not translated, it has no body.
    }

    /// `IXaeroNarratableWidget` (xaero/common/gui/IXaeroNarratableWidget.java)
    pub mod i_xaero_narratable_widget {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IXaeroNarratableWidget.createNarrationMessage(0): not translated, it has no body.
    }

    /// `MyBigButton` (xaero/common/gui/MyBigButton.java)
    pub mod my_big_button {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Plain`. Its static parts are below.
        // MyBigButton.MyBigButton(8): not translated, its class isn't translated (it extends Minecraft's `Plain`).
        // MyBigButton.MyBigButton(5): not translated, its class isn't translated (it extends Minecraft's `Plain`).
        // MyBigButton.getId(0): not translated, its class isn't translated (it extends Minecraft's `Plain`).
    }

    /// `MySuperTinyButton` (xaero/common/gui/MySuperTinyButton.java)
    pub mod my_super_tiny_button {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Plain`. Its static parts are below.
        // MySuperTinyButton.MySuperTinyButton(6): not translated, its class isn't translated (it extends Minecraft's `Plain`).
        // MySuperTinyButton.MySuperTinyButton(4): not translated, its class isn't translated (it extends Minecraft's `Plain`).
    }

    /// `ScreenSwitchSettingEntry` (xaero/common/gui/ScreenSwitchSettingEntry.java)
    pub mod screen_switch_setting_entry {
        use super::*;
        // The class itself isn't translated: it implements `ISettingEntry` (xaero.lib.client.gui). Its static parts are below.
        // ScreenSwitchSettingEntry.ScreenSwitchSettingEntry(4): not translated, its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)).
        // ScreenSwitchSettingEntry.ScreenSwitchSettingEntry(4): not translated, its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)).
        // ScreenSwitchSettingEntry.ScreenSwitchSettingEntry(5): not translated, its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)).
        // ScreenSwitchSettingEntry.ScreenSwitchSettingEntry(5): not translated, its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)).
        // ScreenSwitchSettingEntry.getStringForSearch(0): not translated, its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)).
        // ScreenSwitchSettingEntry.createWidget(3): not translated, its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)).
        // ScreenSwitchSettingEntry.getScreenFactory(0): not translated, its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)).
    }

    /// `TooltipButton` (xaero/common/gui/TooltipButton.java)
    pub mod tooltip_button {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Plain`. Its static parts are below.
        // TooltipButton.TooltipButton(7): not translated, its class isn't translated (it extends Minecraft's `Plain`).
        // TooltipButton.getXaero_tooltip(0): not translated, its class isn't translated (it extends Minecraft's `Plain`).
    }

    /// `WaypointEditForm` (xaero/common/gui/WaypointEditForm.java)
    pub mod waypoint_edit_form {
        use super::*;
        // The class itself isn't translated: it uses WaypointVisibilityType, which isn't translated (its constants carry values). Its static parts are below.
        // WaypointEditForm.getName(0): not translated, its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)).
        // WaypointEditForm.getxText(0): not translated, its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)).
        // WaypointEditForm.getyText(0): not translated, its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)).
        // WaypointEditForm.getzText(0): not translated, its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)).
        // WaypointEditForm.getYawText(0): not translated, its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)).
        // WaypointEditForm.getInitial(0): not translated, its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)).
        // WaypointEditForm.getVisibilityType(0): not translated, its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)).
        // WaypointEditForm.getDisabledOrTemporary(0): not translated, its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)).
        // WaypointEditForm.getColor(0): not translated, its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)).
        // WaypointEditForm.isThirdParty(0): not translated, its class isn't translated (it uses WaypointVisibilityType, which isn't translated (its constants carry values)).
    }

    /// `HudClientOnlyBase` (xaero/common/HudClientOnlyBase.java)
    pub mod hud_client_only_base {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct HudClientOnlyBase {
        }
        impl HudClientOnlyBase {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // HudClientOnlyBase.preInit(2): not translated, it uses HudMod, which isn't translated (it implements `IXaeroMinimap`).
            // HudClientOnlyBase.preLoadLater(1): not translated, it uses HudMod, which isn't translated (it implements `IXaeroMinimap`).
            pub fn load_later_render(&self) {
            }
        }
    }

    /// `HudCommonBase` (xaero/common/HudCommonBase.java)
    pub mod hud_common_base {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct HudCommonBase {
        }
        impl HudCommonBase {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // HudCommonBase.setup(1): not translated, it uses HudMod, which isn't translated (it implements `IXaeroMinimap`).
            // HudCommonBase.preLoadLater(1): not translated, it uses HudMod, which isn't translated (it implements `IXaeroMinimap`).
        }
    }

    /// `HudMod` (xaero/common/HudMod.java)
    pub mod hud_mod {
        use super::*;
        // The class itself isn't translated: it implements `IXaeroMinimap`. Its static parts are below.
        // LOGGER: the logger. Log calls become comments.
        pub const FAIRPLAY: bool = false;
        pub const MINIMAP_ID: &str = "xaerominimap";
        // INSTANCE: not translated, it uses HudMod, which isn't translated (it implements `IXaeroMinimap`).
        pub const VERSION_ID_MINECRAFT: &str = "26.3";
        // HudMod.isLoadedClient(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.isLoadedServer(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.HudMod(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.createPlatformContext(0): not translated, it has no body.
        // HudMod.getCommonConfigFileName(0): not translated, it has no body.
        // HudMod.createModSettings(0): not translated, it has no body.
        // HudMod.createControlsRegister(0): not translated, it has no body.
        // HudMod.createGuiHelper(0): not translated, it has no body.
        // HudMod.getOldConfigFileName(0): not translated, it has no body.
        // HudMod.createClientOnly(0): not translated, it has no body.
        // HudMod.getModName(0): not translated, it has no body.
        // HudMod.getLogger(0): not translated, it has no body.
        // HudMod.createForgeEventHandlerListener(0): not translated, it has no body.
        // HudMod.createConfigCommonRegistryHandler(0): not translated, it has no body.
        // HudMod.createConfigClientRegistryHandlerSupplier(0): not translated, it has no body.
        // HudMod.loadClient(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.loadLater(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.loadLaterClientRender(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.loadServer(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.loadCommon(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.loadLaterServer(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getConfigFile(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getModJAR(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getSettings(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.setSettings(1): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.resetSettings(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.isOutdated(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.setOutdated(1): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getMessage(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.setMessage(1): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getLatestVersion(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.setLatestVersion(1): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getNewestUpdateID(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.setNewestUpdateID(1): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getPatreon(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getWaypointsFile(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getMinimapFolder(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getSupportMods(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getModEvents(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getGuiHelper(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getFieldValidators(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getControlsRegister(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getEvents(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.setLatestVersionMD5(1): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getLatestVersionMD5(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.isStandalone(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getEntityRadarCategoryManager(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.isFairPlay(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getTrackedPlayerRenderer(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getRenderedPlayerTrackerManager(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getServerPlayerTickHandler(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.setServerPlayerTickHandler(1): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getMessageHandler(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getCommonEvents(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getSupportServerMods(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.setCommonConfigIO(1): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getCommonConfigIO(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getClientEventsListener(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getPlatformContext(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.ensureControlsRegister(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getModClientEvents(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getModCommonEvents(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.tryLoadLater(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.tryLoadLaterServer(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getVersionID(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getHud(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getHudRenderer(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getHudIO(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getMinimap(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.isFirstStageLoaded(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getKeyMappingControllers(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getConfigSubFolder(0): not translated, it has no body.
        // HudMod.getDefaultConfigsSubFolder(0): not translated, it has no body.
        // HudMod.getHudConfigs(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getInfoDisplaysIO(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
        // HudMod.getEntityRadarCategorySerializers(0): not translated, its class isn't translated (it implements `IXaeroMinimap`).
    }

    /// `XaeroIcon` (xaero/common/icon/XaeroIcon.java)
    pub mod xaero_icon {
        use super::*;
        // The class itself isn't translated: it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`). Its static parts are below.
        // XaeroIcon.XaeroIcon(3): not translated, it stores a second reference to an object (Rust would need it copied or shared).
        // XaeroIcon.getTextureAtlas(0): not translated, it stores a second reference to an object (Rust would need it copied or shared).
        // XaeroIcon.getOffsetX(0): not translated, its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)).
        // XaeroIcon.getOffsetY(0): not translated, its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)).
    }

    /// `XaeroIconAtlas` (xaero/common/icon/XaeroIconAtlas.java)
    pub mod xaero_icon_atlas {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `GpuTexture`. Its static parts are below.
        // XaeroIconAtlas.XaeroIconAtlas(4): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.getTextureId(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.getTextureView(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.close(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.getWidth(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.getCurrentIndex(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.isFull(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.createIcon(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
    }

    /// `XaeroIconAtlas.Builder` (xaero/common/icon/XaeroIconAtlas.java)
    pub mod xaero_icon_atlas_builder {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `GpuTexture`. Its static parts are below.
        // XaeroIconAtlas.Builder.Builder(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.Builder.setDefault(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.Builder.setPreparedTexture(1): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.Builder.setWidth(1): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.Builder.setIconWidth(1): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.Builder.createGlTexture(1): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.Builder.build(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.Builder.begin(0): not translated, it uses Builder, which isn't translated (it uses Minecraft's `GpuTexture`).
    }

    /// `XaeroIconAtlasManager` (xaero/common/icon/XaeroIconAtlasManager.java)
    pub mod xaero_icon_atlas_manager {
        use super::*;
        // The class itself isn't translated: it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`). Its static parts are below.
        // XaeroIconAtlasManager.XaeroIconAtlasManager(3): not translated, its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)).
        // XaeroIconAtlasManager.clearAtlases(0): not translated, its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)).
        // XaeroIconAtlasManager.getCurrentAtlas(0): not translated, its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)).
        // XaeroIconAtlasManager.getAtlasTextureSize(0): not translated, its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)).
    }

    /// `IXaeroMinimap` (xaero/common/IXaeroMinimap.java)
    pub mod i_xaero_minimap {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IXaeroMinimap.getVersionID(0): not translated, it has no body.
        // IXaeroMinimap.getFileLayoutID(0): not translated, it has no body.
        // IXaeroMinimap.getConfigFile(0): not translated, it has no body.
        // IXaeroMinimap.getModJAR(0): not translated, it has no body.
        // IXaeroMinimap.getSettings(0): not translated, it has no body.
        // IXaeroMinimap.setSettings(1): not translated, it has no body.
        // IXaeroMinimap.isOutdated(0): not translated, it has no body.
        // IXaeroMinimap.setOutdated(1): not translated, it has no body.
        // IXaeroMinimap.getMessage(0): not translated, it has no body.
        // IXaeroMinimap.setMessage(1): not translated, it has no body.
        // IXaeroMinimap.getLatestVersion(0): not translated, it has no body.
        // IXaeroMinimap.getLatestVersionMD5(0): not translated, it has no body.
        // IXaeroMinimap.setLatestVersion(1): not translated, it has no body.
        // IXaeroMinimap.setLatestVersionMD5(1): not translated, it has no body.
        // IXaeroMinimap.getNewestUpdateID(0): not translated, it has no body.
        // IXaeroMinimap.setNewestUpdateID(1): not translated, it has no body.
        // IXaeroMinimap.getPatreon(0): not translated, it has no body.
        // IXaeroMinimap.getVersionsURL(0): not translated, it has no body.
        // IXaeroMinimap.resetSettings(0): not translated, it has no body.
        // IXaeroMinimap.getUpdateLink(0): not translated, it has no body.
        // IXaeroMinimap.getSettingsKey(0): not translated, it has no body.
        // IXaeroMinimap.getServerSettingsKey(0): not translated, it has no body.
        // IXaeroMinimap.getWaypointsFile(0): not translated, it has no body.
        // IXaeroMinimap.getMinimapFolder(0): not translated, it has no body.
        // IXaeroMinimap.createSession(0): not translated, it has no body.
        // IXaeroMinimap.getSupportMods(0): not translated, it has no body.
        // IXaeroMinimap.getSupportServerMods(0): not translated, it has no body.
        // IXaeroMinimap.getGuiHelper(0): not translated, it has no body.
        // IXaeroMinimap.getFieldValidators(0): not translated, it has no body.
        // IXaeroMinimap.getControlsRegister(0): not translated, it has no body.
        // IXaeroMinimap.getEvents(0): not translated, it has no body.
        // IXaeroMinimap.tryLoadLater(0): not translated, it has no body.
        // IXaeroMinimap.tryLoadLaterServer(0): not translated, it has no body.
        // IXaeroMinimap.getModEvents(0): not translated, it has no body.
        // IXaeroMinimap.isStandalone(0): not translated, it has no body.
        // IXaeroMinimap.getEntityRadarCategoryManager(0): not translated, it has no body.
        // IXaeroMinimap.isFairPlay(0): not translated, it has no body.
        // IXaeroMinimap.getTrackedPlayerRenderer(0): not translated, it has no body.
        // IXaeroMinimap.getRenderedPlayerTrackerManager(0): not translated, it has no body.
        // IXaeroMinimap.getServerPlayerTickHandler(0): not translated, it has no body.
        // IXaeroMinimap.setServerPlayerTickHandler(1): not translated, it has no body.
        // IXaeroMinimap.getMessageHandler(0): not translated, it has no body.
        // IXaeroMinimap.getCommonEvents(0): not translated, it has no body.
        // IXaeroMinimap.setCommonConfigIO(1): not translated, it has no body.
        // IXaeroMinimap.getCommonConfigIO(0): not translated, it has no body.
        // IXaeroMinimap.getClientEventsListener(0): not translated, it has no body.
        // IXaeroMinimap.getPlatformContext(0): not translated, it has no body.
        // IXaeroMinimap.ensureControlsRegister(0): not translated, it has no body.
        // IXaeroMinimap.getModClientEvents(0): not translated, it has no body.
        // IXaeroMinimap.getModCommonEvents(0): not translated, it has no body.
        // IXaeroMinimap.getModId(0): not translated, it has no body.
        // IXaeroMinimap.isLoadedClient(0): not translated, it has no body.
        // IXaeroMinimap.isLoadedServer(0): not translated, it has no body.
        // IXaeroMinimap.getHud(0): not translated, it has no body.
        // IXaeroMinimap.getHudRenderer(0): not translated, it has no body.
        // IXaeroMinimap.getHudIO(0): not translated, it has no body.
        // IXaeroMinimap.getMinimap(0): not translated, it has no body.
        // IXaeroMinimap.isFirstStageLoaded(0): not translated, it has no body.
    }

    /// `AbstractHighlighter` (xaero/common/minimap/highlight/AbstractHighlighter.java)
    pub mod abstract_highlighter {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct AbstractHighlighter {
            pub covering_outside_discovered: bool,
            pub result_store: Vec<i32>,
        }
        impl AbstractHighlighter {
            pub fn new(covering_outside_discovered: bool) -> Self {
                let mut s = Self { covering_outside_discovered: covering_outside_discovered, result_store: vec![0i32; 256] };
                s.covering_outside_discovered = covering_outside_discovered;
                s
            }
            // AbstractHighlighter.regionHasHighlights(3): not translated, it has no body.
            // AbstractHighlighter.chunkIsHighlit(3): not translated, it has no body.
            // AbstractHighlighter.getChunkHighlitColor(3): not translated, it has no body.
            // AbstractHighlighter.addBlockHighlightTooltips(5): not translated, it has no body.
            pub fn set_result(&mut self, x: i32, z: i32, color: i32) {
                self.result_store[((z << 4) | x) as usize] = color;
            }
            pub fn get_blend(&self, color1: i32, color2: i32) -> i32 {
                if color1 == color2 {
                    return color1;
                }
                let red1: i32 = (color1 >> 8) & 0xFF;
                let green1: i32 = (color1 >> 16) & 0xFF;
                let blue1: i32 = (color1 >> 24) & 0xFF;
                let alpha1: i32 = color1 & 0xFF;
                let red2: i32 = (color2 >> 8) & 0xFF;
                let green2: i32 = (color2 >> 16) & 0xFF;
                let blue2: i32 = (color2 >> 24) & 0xFF;
                let alpha2: i32 = color2 & 0xFF;
                let red: i32 = (red1 + red2) >> 1;
                let green: i32 = (green1 + green2) >> 1;
                let blue: i32 = (blue1 + blue2) >> 1;
                let alpha: i32 = (alpha1 + alpha2) >> 1;
                return (((blue << 24) | (green << 16)) | (red << 8)) | alpha;
            }
            pub fn is_covering_outside_discovered(&self) -> bool {
                return self.covering_outside_discovered;
            }
        }
    }

    /// `ChunkHighlighter` (xaero/common/minimap/highlight/ChunkHighlighter.java)
    pub mod chunk_highlighter {
        use super::*;
        // The class itself isn't translated: it extends `AbstractHighlighter`. Its static parts are below.
        // ChunkHighlighter.ChunkHighlighter(1): not translated, its class isn't translated (it extends `AbstractHighlighter`).
        // ChunkHighlighter.getColors(3): not translated, it has no body.
        // ChunkHighlighter.getChunkHighlitColor(3): not translated, its class isn't translated (it extends `AbstractHighlighter`).
        // ChunkHighlighter.getSideBlend(3): not translated, its class isn't translated (it extends `AbstractHighlighter`).
        // ChunkHighlighter.addBlockHighlightTooltips(5): not translated, its class isn't translated (it extends `AbstractHighlighter`).
        // ChunkHighlighter.addChunkHighlightTooltips(5): not translated, it has no body.
    }

    /// `DimensionHighlighterHandler` (xaero/common/minimap/highlight/DimensionHighlighterHandler.java)
    pub mod dimension_highlighter_handler {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ResourceKey`. Its static parts are below.
        // DimensionHighlighterHandler.DimensionHighlighterHandler(3): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.shouldApplyRegionHighlights(3): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.shouldApplyTileChunkHighlights(5): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.shouldApplyTileChunkHighlights(6): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.shouldApplyTileChunkHighlightsHelp(6): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.applyChunkHighlightColors(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.requestRefresh(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.requestRefresh(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.addBlockHighlightTooltips(5): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.getVersion(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
    }

    /// `HighlighterRegistry` (xaero/common/minimap/highlight/HighlighterRegistry.java)
    pub mod highlighter_registry {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct HighlighterRegistry {
            pub highlighters: Vec<abstract_highlighter::AbstractHighlighter>,
        }
        impl HighlighterRegistry {
            pub fn new() -> Self {
                let mut s = Self { highlighters: Vec::new() };
                s
            }
            // HighlighterRegistry.register(1): not translated, it stores a second reference to an object (Rust would need it copied or shared).
            pub fn end(&mut self) {
                self.highlighters = self.highlighters.to_vec();
            }
            pub fn get_highlighters(&self) -> Vec<abstract_highlighter::AbstractHighlighter> {
                return self.highlighters.to_vec();
            }
        }
    }

    /// `TestHighlighter` (xaero/common/minimap/highlight/TestHighlighter.java)
    pub mod test_highlighter {
        use super::*;
        // The class itself isn't translated: it extends `ChunkHighlighter`. Its static parts are below.
        // TestHighlighter.TestHighlighter(0): not translated, its class isn't translated (it extends `ChunkHighlighter`).
        // TestHighlighter.regionHasHighlights(3): not translated, its class isn't translated (it extends `ChunkHighlighter`).
        // TestHighlighter.getColors(3): not translated, its class isn't translated (it extends `ChunkHighlighter`).
        // TestHighlighter.chunkIsHighlit(3): not translated, its class isn't translated (it extends `ChunkHighlighter`).
        // TestHighlighter.addChunkHighlightTooltips(5): not translated, its class isn't translated (it extends `ChunkHighlighter`).
    }

    /// `IXaeroMinimapClientWorld` (xaero/common/minimap/mcworld/IXaeroMinimapClientWorld.java)
    pub mod i_xaero_minimap_client_world {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IXaeroMinimapClientWorld.getXaero_minimapData(0): not translated, it has no body.
        // IXaeroMinimapClientWorld.setXaero_minimapData(1): not translated, it has no body.
    }

    /// `MinimapClientWorldData` (xaero/common/minimap/mcworld/MinimapClientWorldData.java)
    pub mod minimap_client_world_data {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `EnvironmentAttributeSystem`. Its static parts are below.
        // MinimapClientWorldData.MinimapClientWorldData(1): not translated, its class isn't translated (it uses Minecraft's `EnvironmentAttributeSystem`).
        // MinimapClientWorldData.setServerModNetworkVersion(1): not translated, its class isn't translated (it uses Minecraft's `EnvironmentAttributeSystem`).
        // MinimapClientWorldData.getServerModNetworkVersion(0): not translated, its class isn't translated (it uses Minecraft's `EnvironmentAttributeSystem`).
        // MinimapClientWorldData.setSyncedRules(1): not translated, its class isn't translated (it uses Minecraft's `EnvironmentAttributeSystem`).
        // MinimapClientWorldData.getSyncedRules(0): not translated, its class isn't translated (it uses Minecraft's `EnvironmentAttributeSystem`).
        // MinimapClientWorldData.getAttributeSystem(0): not translated, its class isn't translated (it uses Minecraft's `EnvironmentAttributeSystem`).
    }

    /// `MinimapClientWorldDataHelper` (xaero/common/minimap/mcworld/MinimapClientWorldDataHelper.java)
    pub mod minimap_client_world_data_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MinimapClientWorldDataHelper {
        }
        impl MinimapClientWorldDataHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MinimapClientWorldDataHelper.getCurrentWorldData(0): not translated, it uses MinimapClientWorldData, which isn't translated (it uses Minecraft's `EnvironmentAttributeSystem`).
        // MinimapClientWorldDataHelper.getWorldData(1): not translated, it uses MinimapClientWorldData, which isn't translated (it uses Minecraft's `EnvironmentAttributeSystem`).
    }

    /// `MinimapProcessor` (xaero/common/minimap/MinimapProcessor.java)
    pub mod minimap_processor {
        use super::*;
        // The class itself isn't translated: it uses IXaeroMinimap, which isn't translated (it's an interface). Its static parts are below.
        pub const DEBUG: bool = false;
        pub const FRAME: i32 = 9;
        // MinimapProcessor.MinimapProcessor(5): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.getMinimapSize(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.getMinimapBufferSize(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.isEnlargedMap(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.setEnlargedMap(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.getMinimapZoom(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.isCaveModeDisplayed(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.getTargetZoom(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.instantZoom(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.updateZoom(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.getMinimapWriter(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.canUseFrameBuffer(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.getFBOBufferSize(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.onClientTick(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.onPlayerTick(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.checkFBO(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.onRender(12): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.getRenderer(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.hasMinimapItem(1): not translated, it uses Minecraft's `Player`.
        // MinimapProcessor.isToResetImage(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.setToResetImage(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.getRadarSession(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.cleanup(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.isManualCaveMode(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.toggleManualCaveMode(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.getMinimap(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.getNoMinimapMessageReceived(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.setNoMinimapMessageReceived(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.getForcedFairPlay(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.setFairPlayOnlyMessageReceived(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.getSyncedTrackedPlayerManager(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.serverHasMod(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.setServerModNetworkVersion(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.getServerModNetworkVersion(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.getLastMapDimensionScale(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.setLastMapDimensionScale(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.getLastMapDimension(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.setLastMapDimension(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.getSession(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.updateMinimapItem(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.getMinimapItem(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.setConsideringNetherFairPlayMessage(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.isConsideringNetherFairPlayMessage(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.getRenderState(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapProcessor.getEnlargedRenderState(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
    }

    /// `MinimapChunk` (xaero/common/minimap/region/MinimapChunk.java)
    pub mod minimap_chunk {
        use super::*;
        // The class itself isn't translated: it uses `GpuTextureAndView` (xaero.lib.client.graphics). Its static parts are below.
        pub const SIZE_TILES: i32 = 4;
        pub const INT_BUFFER_SIZE: i32 = 4096;
        pub const LIGHT_LEVELS: i32 = 5;
        // MinimapChunk.MinimapChunk(2): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.reset(2): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.recycleTiles(0): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.getLevelToRefresh(1): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.bindTexture(1): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.updateBuffers(1): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.putColour(8): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.copyBuffer(2): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.getLevelsBuffered(0): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.isHasSomething(0): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.setHasSomething(1): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.getX(0): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.getZ(0): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.getGlTexture(1): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.setGlTexture(2): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.getTile(2): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.setTile(3): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.isChanged(0): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.setChanged(1): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.setLevelsBuffered(1): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.isBlockTextureUpload(0): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.setBlockTextureUpload(1): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.isRefreshRequired(1): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.setRefreshRequired(2): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.getBuffer(1): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // MinimapChunk.cleanup(1): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
    }

    /// `MinimapTile` (xaero/common/minimap/region/MinimapTile.java)
    pub mod minimap_tile {
        use super::*;
        pub static RECYCLED: LazyLock<JCell<Vec<minimap_tile::MinimapTile>>> = LazyLock::new(|| JCell::new(Vec::new()));
        #[derive(Clone, Debug)]
        pub struct MinimapTile {
            pub was_transfered: bool,
            pub comparison_codes: Vec<Vec<i64>>,
            pub comparison_codes_add: Vec<Vec<i8>>,
            pub comparison_codes_add2: Vec<Vec<i8>>,
            pub vertical_slopes: Vec<Vec<i8>>,
            pub diagonal_slopes: Vec<Vec<i8>>,
            pub red: Vec<Vec<Vec<i8>>>,
            pub green: Vec<Vec<Vec<i8>>>,
            pub blue: Vec<Vec<Vec<i8>>>,
            pub chunk_grid: bool,
            pub slime_chunk: bool,
            pub x: i32,
            pub z: i32,
            pub success: bool,
            pub heights: Option<Vec<Vec<i16>>>,
            pub cave_level: i32,
            pub has_something: bool,
            pub has_terrain: bool,
            pub highlight_version: i32,
            pub highlights: Option<Vec<i32>>,
        }
        impl MinimapTile {
            // MinimapTile.MinimapTile(4): not translated, it uses ModSettings, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
            // MinimapTile.create(4): not translated, it uses ModSettings, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
            // MinimapTile.recycle(0): not translated, `this` used as a value.
            pub fn is_was_transfered(&self) -> bool {
                return self.was_transfered;
            }
            pub fn set_was_transfered(&mut self, was_transfered: bool) {
                self.was_transfered = was_transfered;
            }
            pub fn is_chunk_grid(&self) -> bool {
                return self.chunk_grid;
            }
            pub fn is_slime_chunk_0(&self) -> bool {
                return self.slime_chunk;
            }
            pub fn get_red(&self, l: i32, x: i32, z: i32) -> i32 {
                return (self.red[l as usize][x as usize][z as usize] as i32) & 0xFF;
            }
            pub fn get_green(&self, l: i32, x: i32, z: i32) -> i32 {
                return (self.green[l as usize][x as usize][z as usize] as i32) & 0xFF;
            }
            pub fn get_blue(&self, l: i32, x: i32, z: i32) -> i32 {
                return (self.blue[l as usize][x as usize][z as usize] as i32) & 0xFF;
            }
            pub fn set_rgb(&mut self, l: i32, x: i32, z: i32, r: i32, g: i32, b: i32) {
                self.red[l as usize][x as usize][z as usize] = (r as i8);
                self.green[l as usize][x as usize][z as usize] = (g as i8);
                self.blue[l as usize][x as usize][z as usize] = (b as i8);
            }
            pub fn is_success(&self) -> bool {
                return self.success;
            }
            pub fn set_success(&mut self, success: bool) {
                self.success = success;
            }
            pub fn get_x(&self) -> i32 {
                return self.x;
            }
            pub fn get_z(&self) -> i32 {
                return self.z;
            }
            pub fn get_height(&self, x: i32, z: i32) -> i32 {
                return ((((self.heights.as_ref().expect("null"))[x as usize])[z as usize]) as i32);
            }
            pub fn set_height(&mut self, x: i32, z: i32, h: i32) {
                ((self.heights.as_mut().expect("null"))[x as usize])[z as usize] = (h as i16);
            }
            pub fn pixel_changed(&self, x: i32, z: i32, code: i64, add: i8, add2: i8, vertical_slope: i8, diagonal_slope: i8) -> bool {
                return (((((self.comparison_codes_add[x as usize][z as usize] as i32) != (add as i32)) || ((self.comparison_codes_add2[x as usize][z as usize] as i32) != (add2 as i32))) || (self.comparison_codes[x as usize][z as usize] != code)) || ((self.vertical_slopes[x as usize][z as usize] as i32) != (vertical_slope as i32))) || ((self.diagonal_slopes[x as usize][z as usize] as i32) != (diagonal_slope as i32));
            }
            pub fn set_code(&mut self, x: i32, z: i32, code: i64, add: i8, add2: i8, vertical_slope: i8, diagonal_slope: i8) {
                self.comparison_codes[x as usize][z as usize] = code;
                self.comparison_codes_add[x as usize][z as usize] = add;
                self.comparison_codes_add2[x as usize][z as usize] = add2;
                self.vertical_slopes[x as usize][z as usize] = vertical_slope;
                self.diagonal_slopes[x as usize][z as usize] = diagonal_slope;
            }
            pub fn set_has_something(&mut self, has_something: bool) {
                self.has_something = has_something;
            }
            pub fn is_has_something(&self) -> bool {
                return self.has_something;
            }
            pub fn set_has_terrain(&mut self, has_terrain: bool) {
                self.has_terrain = has_terrain;
            }
            pub fn has_terrain(&self) -> bool {
                return self.has_terrain;
            }
            pub fn get_highlight_version(&self) -> i32 {
                return self.highlight_version;
            }
            pub fn set_highlight_version(&mut self, highlight_version: i32) {
                self.highlight_version = highlight_version;
            }
            pub fn get_highlights(&self) -> Option<Vec<i32>> {
                return self.highlights.clone();
            }
            pub fn set_highlights(&mut self, highlights: &[i32]) {
                self.highlights = Some(highlights.to_vec());
            }
        }
        // MinimapTile.getANewTile(4): not translated, List.remove on a list kept in a changing static.
        // MinimapTile.isSlimeChunk(4): not translated, it calls Minecraft's `WorldgenRandom`.seedSlimeChunk.
    }

    /// `MinimapFBORenderer` (xaero/common/minimap/render/MinimapFBORenderer.java)
    pub mod minimap_fbo_renderer {
        use super::*;
        // The class itself isn't translated: it extends `MinimapRenderer`. Its static parts are below.
        // MinimapFBORenderer.MinimapFBORenderer(6): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapFBORenderer.loadFrameBuffer(1): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapFBORenderer.restoreDefaultTarget(0): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapFBORenderer.renderChunks(19): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapFBORenderer.renderChunksToFBO(16): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapFBORenderer.deleteFramebuffers(0): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapFBORenderer.isLoadedFBO(0): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapFBORenderer.setLoadedFBO(1): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapFBORenderer.isTriedFBO(0): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapFBORenderer.assumeUsingFBO(0): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapFBORenderer.resetEntityIcons(0): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapFBORenderer.resetEntityIconsResources(0): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapFBORenderer.onRadarIconModelPrepareDetection(1): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapFBORenderer.onRadarIconModelRenderTrace(3): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapFBORenderer.onEntityIconModelPartRenderTrace(2): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapFBORenderer.renderMainEntityDot(3): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapFBORenderer.getTas(0): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapFBORenderer.getEntityRadarRenderer(0): not translated, its class isn't translated (it extends `MinimapRenderer`).
    }

    /// `MinimapRenderer` (xaero/common/minimap/render/MinimapRenderer.java)
    pub mod minimap_renderer {
        use super::*;
        // The class itself isn't translated: another class has the same name, in a different package. Its static parts are below.
        pub const BLACK: i32 = -16777216;
        pub const SLIME: i32 = -2142047936;
        // MinimapRenderer.MinimapRenderer(6): not translated, its class isn't translated (another class has the same name, in a different package).
        // MinimapRenderer.getRenderAngle(1): not translated, its class isn't translated (another class has the same name, in a different package).
        // MinimapRenderer.getActualAngle(0): not translated, its class isn't translated (another class has the same name, in a different package).
        // MinimapRenderer.renderChunks(19): not translated, it has no body.
        // MinimapRenderer.renderMinimap(11): not translated, its class isn't translated (another class has the same name, in a different package).
        // MinimapRenderer.renderOutsidePip(10): not translated, its class isn't translated (another class has the same name, in a different package).
        // MinimapRenderer.renderCompass(11): not translated, its class isn't translated (another class has the same name, in a different package).
        // MinimapRenderer.drawArrow(10): not translated, its class isn't translated (another class has the same name, in a different package).
        // MinimapRenderer.getZoom(0): not translated, its class isn't translated (another class has the same name, in a different package).
        // MinimapRenderer.setZoom(1): not translated, its class isn't translated (another class has the same name, in a different package).
        // MinimapRenderer.getSunBrightness(2): not translated, its class isn't translated (another class has the same name, in a different package).
        // MinimapRenderer.getHelper(0): not translated, its class isn't translated (another class has the same name, in a different package).
        // MinimapRenderer.getTas(0): not translated, it has no body.
    }

    /// `MinimapRendererHelper` (xaero/common/minimap/render/MinimapRendererHelper.java)
    pub mod minimap_renderer_helper {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ProjectionMatrixBuffer`. Its static parts are below.
        // MinimapRendererHelper.drawMyTexturedModalRect(11): not translated, its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`).
        // MinimapRendererHelper.drawMyTexturedModalRect(12): not translated, its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`).
        // MinimapRendererHelper.drawIconOutline(11): not translated, its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`).
        // MinimapRendererHelper.drawTexturedElipseInsideRectangle(11): not translated, its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`).
        // MinimapRendererHelper.drawTexturedElipseInsideRectangle(12): not translated, its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`).
        // MinimapRendererHelper.drawTexturedElipseInsideRectangleFrame(18): not translated, its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`).
        // MinimapRendererHelper.addTexturedRectToExistingBuffer(8): not translated, its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`).
        // MinimapRendererHelper.drawMyColoredRect(6): not translated, its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`).
        // MinimapRendererHelper.addColoredLineToExistingBuffer(11): not translated, its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`).
        // MinimapRendererHelper.drawMyColoredRect(6): not translated, its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`).
        // MinimapRendererHelper.bindTextureBuffer(5): not translated, its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`).
        // MinimapRendererHelper.putColor(7): not translated, its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`).
        // MinimapRendererHelper.gridOverlay(5): not translated, its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`).
        // MinimapRendererHelper.slimeOverlay(4): not translated, its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`).
        // MinimapRendererHelper.defaultOrtho(1): not translated, its class isn't translated (it uses Minecraft's `ProjectionMatrixBuffer`).
        pub fn restore_default_shader_blend_state() {
        }
    }

    /// `MinimapSafeModeRenderer` (xaero/common/minimap/render/MinimapSafeModeRenderer.java)
    pub mod minimap_safe_mode_renderer {
        use super::*;
        // The class itself isn't translated: it extends `MinimapRenderer`. Its static parts are below.
        pub static MAP_TEXTURES: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::parse("xaeromaptexture"));
        // MinimapSafeModeRenderer.MinimapSafeModeRenderer(6): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapSafeModeRenderer.updateMapFrameSafeMode(14): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapSafeModeRenderer.renderEntityListSafeMode(18): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapSafeModeRenderer.renderEntityDotSafeMode(18): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapSafeModeRenderer.renderChunks(19): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapSafeModeRenderer.getLoadedBlockColor(8): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapSafeModeRenderer.chunkOverlay(8): not translated, its class isn't translated (it extends `MinimapRenderer`).
        // MinimapSafeModeRenderer.getTas(0): not translated, its class isn't translated (it extends `MinimapRenderer`).
    }

    /// `EntityIconDefinitions` (xaero/common/minimap/render/radar/EntityIconDefinitions.java)
    pub mod entity_icon_definitions {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EntityIconDefinitions {
        }
        impl EntityIconDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EntityIconDefinitions.getVariant(3): not translated, it's generic.
        // EntityIconDefinitions.buildVariantIdString(3): not translated, it uses Minecraft's `EntityRenderer`.
        // EntityIconDefinitions.getVariantString(2): not translated, it uses Minecraft's `EntityRenderer`.
    }

    /// `Waypoint` (xaero/common/minimap/waypoints/Waypoint.java)
    pub mod waypoint {
        use super::*;
        // The class itself isn't translated: it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint). Its static parts are below.
        pub const ONEOFF_DESTINATION_SAFE_FOR: i32 = 5000;
        pub const ONEOFF_DESTINATION_REMOVE_DISTANCE: i32 = 4;
        // RENDER_SORTING_POS: not translated, it uses Minecraft's `Vec3`.
        // Waypoint.Waypoint(6): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.Waypoint(7): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.Waypoint(8): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.Waypoint(9): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.Waypoint(6): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.Waypoint(7): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.Waypoint(8): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.Waypoint(9): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getX(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setX(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getX(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getY(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setY(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getZ(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setZ(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getZ(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getName(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getLocalizedName(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getInitials(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getNameSafe(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setName(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getSymbol(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setSymbol(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getSymbolSafe(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getColor(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getActualColor(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setColor(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getWaypointColor(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.isGlobal(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getVisibilityType(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getVisibility(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setVisibilityType(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setVisibility(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.isDisabled(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getDisabled(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setDisabled(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setDisabled(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getWaypointType(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setType(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getPurpose(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setPurpose(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.isRotation(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setRotation(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getYaw(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setYaw(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.isTemporary(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setTemporary(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.isYIncluded(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setYIncluded(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getCreatedAt(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.isOneoffDestination(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setOneoffDestination(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.isDestination(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getDistanceSq(3): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        pub fn get_string_from_string_safe(string_safe: &str, replacement: &str) -> String {
            return string_safe.replace(&replacement, ":");
        }
        // Waypoint.isServerWaypoint(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getComparisonName(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getComparisonDistance(2): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getComparisonAngleCos(2): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getRenderSortingDistanceSquared(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.compareTo(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setThirdPartyOrigin(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getThirdPartyOrigin(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.isThirdParty(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.isEffectivelyDeleted(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getThirdPartyRenderOverride(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setThirdPartyRenderOverride(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.isThirdPartyDeleted(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.setThirdPartyDeleted(1): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // Waypoint.getRenderInfoEditDest(0): not translated, its class isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
    }

    /// `WaypointUtil` (xaero/common/minimap/waypoints/WaypointUtil.java)
    pub mod waypoint_util {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct WaypointUtil {
        }
        impl WaypointUtil {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn get_added_minimap_icon_frame_1(initials_width: i32) -> i32 {
            return waypoint_util::get_added_minimap_icon_frame_2(0, initials_width);
        }
        pub fn get_added_minimap_icon_frame_2(mut added_frame: i32, initials_width: i32) -> i32 {
            if initials_width > 8 {
                let total_to_add: i32 = initials_width - 8;
                let frame_to_add: i32 = total_to_add - (total_to_add / 2);
                if frame_to_add > added_frame {
                    added_frame = frame_to_add;
                }
            }
            return added_frame;
        }
    }

    /// `BiomeBlendCalculator` (xaero/common/minimap/write/biome/BiomeBlendCalculator.java)
    pub mod biome_blend_calculator {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockAndTintGetter`. Its static parts are below.
        // BiomeBlendCalculator.prepare(2): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BiomeBlendCalculator.getBlockEntity(1): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BiomeBlendCalculator.getBlockState(1): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BiomeBlendCalculator.getFluidState(1): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BiomeBlendCalculator.getLightEngine(0): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BiomeBlendCalculator.getHeight(0): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BiomeBlendCalculator.getMinY(0): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BiomeBlendCalculator.cardinalLighting(0): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BiomeBlendCalculator.getBlockTint(2): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
    }

    /// `MinimapWriter` (xaero/common/minimap/write/MinimapWriter.java)
    pub mod minimap_writer {
        use super::*;
        // The class itself isn't translated: it uses IXaeroMinimap, which isn't translated (it's an interface). Its static parts are below.
        pub const VOID_COLOR: i32 = -16121833;
        pub const DEFAULT_AMBIENT_LIGHT: f32 = 0.7f32;
        pub const DEFAULT_AMBIENT_LIGHT_COLORED: f32 = 0.2f32;
        pub const DEFAULT_AMBIENT_LIGHT_WHITE: f32 = 0.5f32;
        pub const DEFAULT_MAX_DIRECT_LIGHT: f32 = 0.6666667f32;
        pub const GLOWING_MAX_DIRECT_LIGHT: f32 = 0.22222224f32;
        pub static DIMENSIONS_TO_IGNORE: LazyLock<Vec<String>> = LazyLock::new(|| vec!["FZHammer".to_string()]);
        pub const UPDATE_EVERY_RUNS: i32 = 5;
        pub const MAXIMUM_OVERLAYS: i32 = 5;
        pub const SUN_MINIMUM: i32 = 9;
        pub static NO_Y_VALUE: LazyLock<i32> = LazyLock::new(|| i32::MAX);
        pub const MAX_TRANSPARENCY_BLEND_DEPTH: i32 = 5;
        // MinimapWriter.MinimapWriter(4): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getBlockStateLightEmission(3): not translated, it has no body.
        // MinimapWriter.setupDimensionHighlightHandler(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.updateTimeDebug(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.onRender(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.beforeWriting(4): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.writeChunk(7): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.writeTile(16): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.loadBlockColor(64): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.unpackFramedBlocks(3): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.findBlock(21): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.findBlockHelp(22): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.calculateBlockColors(15): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.isTransparent(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.isGlowing(3): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.applyTransparentLayer(10): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.applyTransparentLayer(5): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getBlockTint(3): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getQuads(5): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getParticleIcon(5): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.loadBlockColourFromTexture(5): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.addBlockColorMultipliers(4): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.ignoreWorld(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getCaving(4): not translated, its body couldn't be read: a labelled statement.
        // MinimapWriter.getSectionBasedHeight(2): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.furtherZoomout(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getLoadSide(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getUpdateRadiusInChunks(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getMapCoord(2): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getLoadedCaving(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.notEmptyColor(3): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getFixedSkyLightBlockBrightness(3): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getBlockBrightness(4): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getLoadingMapChunkX(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getLoadingMapChunkZ(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getLoadingSideInChunks(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getLoadedBlocks(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getLoadedMapChunkZ(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getLoadedMapChunkX(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getLoadedLevels(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.setClearBlockColours(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.cleanup(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.resetShortBlocks(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getDimensionHighlightHandler(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.getLoadedSideInChunks(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.isLoadedNonWorldMap(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // MinimapWriter.onResourceReload(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
    }

    /// `MinimapWriterFabric` (xaero/common/minimap/write/MinimapWriterFabric.java)
    pub mod minimap_writer_fabric {
        use super::*;
        // The class itself isn't translated: it extends `MinimapWriter`. Its static parts are below.
        // MinimapWriterFabric.MinimapWriterFabric(4): not translated, its class isn't translated (it extends `MinimapWriter`).
        // MinimapWriterFabric.getBlockStateLightEmission(3): not translated, its class isn't translated (it extends `MinimapWriter`).
    }

    /// `MinimapWriterHelper` (xaero/common/minimap/write/MinimapWriterHelper.java)
    pub mod minimap_writer_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MinimapWriterHelper {
        }
        impl MinimapWriterHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            pub fn get_glowing_colour(&self, r: i32, g: i32, b: i32, result: &mut [i32]) {
                let total: i32 = (r + g) + b;
                let min_brightness: f32 = 407.0f32;
                let brightener: f32 = 1.0f32.max(min_brightness / (total as f32));
                result[0] = (((r as f32) * brightener) as i32);
                result[1] = (((g as f32) * brightener) as i32);
                result[2] = (((b as f32) * brightener) as i32);
            }
        }
    }

    /// `CachedFunction` (xaero/common/misc/CachedFunction.java)
    pub mod cached_function {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // CachedFunction.CachedFunction(1): not translated, its class isn't translated (it's generic).
        // CachedFunction.apply(1): not translated, its class isn't translated (it's generic).
        // CachedFunction.clear(0): not translated, its class isn't translated (it's generic).
    }

    /// `Internet` (xaero/common/misc/Internet.java)
    pub mod internet {
        use super::*;
        // cipher: not translated, it uses `Cipher` (javax.crypto).
        #[derive(Clone, Debug)]
        pub struct Internet {
        }
        impl Internet {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Internet.checkModVersion(1): not translated, it uses IXaeroMinimap, which isn't translated (it's an interface).
    }

    /// `ListFactory` (xaero/common/misc/ListFactory.java)
    pub mod list_factory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ListFactory.get(0): not translated, it has no body.
    }

    /// `MapFactory` (xaero/common/misc/MapFactory.java)
    pub mod map_factory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // MapFactory.get(0): not translated, it has no body.
    }

    /// `Misc` (xaero/common/misc/Misc.java)
    pub mod misc {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Misc {
        }
        impl Misc {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Misc.getMouseX(2): not translated, it uses Minecraft's `Minecraft`.
        // Misc.getMouseY(2): not translated, it uses Minecraft's `Minecraft`.
        // Misc.getChunkPosAsLong(1): not translated, it uses Minecraft's `LevelChunk`.
        // Misc.hasItem(2): not translated, it uses Minecraft's `Player`.
        // Misc.hasItem(3): not translated, it uses Minecraft's `NonNullList`.
        // Misc.getFixedDisplayName(1): not translated, it uses Minecraft's `Component`.
        // Misc.hasEffect(2): not translated, it uses Minecraft's `Player`.
        // Misc.hasEffect(1): not translated, it uses Minecraft's `Holder`.
        // Misc.isValidResourceLocationString(1): not translated, it calls Minecraft's `Identifier`.isAllowedInIdentifier.
    }

    /// `OptimizedMath` (xaero/common/misc/OptimizedMath.java)
    pub mod optimized_math {
        use super::*;
        pub static XP: LazyLock<JVec3> = LazyLock::new(|| JVec3::new(1.0f32, 0.0f32, 0.0f32));
        pub static YP: LazyLock<JVec3> = LazyLock::new(|| JVec3::new(0.0f32, 1.0f32, 0.0f32));
        pub static ZP: LazyLock<JVec3> = LazyLock::new(|| JVec3::new(0.0f32, 0.0f32, 1.0f32));
        #[derive(Clone, Debug)]
        pub struct OptimizedMath {
        }
        impl OptimizedMath {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn my_floor(d: f64) -> i32 {
            let mut as_int: i32 = (d as i32);
            if ((as_int as f64) != d) && (d < 0.0f64) {
                as_int -= 1;
            }
            return as_int;
        }
        // OptimizedMath.rotatePose(3): not translated, it uses Minecraft's `Pose`.
        // OptimizedMath.rotateMatrix(3): not translated, JOML's Matrix4f.rotate with 2 values.
    }

    /// `MixinPlayer` (xaero/common/mixin/fwaystones/MixinPlayer.java)
    pub mod mixin_player {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MixinPlayer {
        }
        impl MixinPlayer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `MixinWaystoneStorage` (xaero/common/mixin/fwaystones/MixinWaystoneStorage.java)
    pub mod mixin_waystone_storage {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MixinWaystoneStorage {
        }
        impl MixinWaystoneStorage {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `ClientClaimChangeListener` (xaero/common/mods/pac/ClientClaimChangeListener.java)
    pub mod client_claim_change_listener {
        use super::*;
        // The class itself isn't translated: it implements `IClaimsManagerListenerAPI` (xaero.pac.common.claims.tracker.api). Its static parts are below.
        // ClientClaimChangeListener.onWholeRegionChange(3): not translated, its class isn't translated (it implements `IClaimsManagerListenerAPI` (xaero.pac.common.claims.tracker.api)).
        // ClientClaimChangeListener.onChunkChange(4): not translated, its class isn't translated (it implements `IClaimsManagerListenerAPI` (xaero.pac.common.claims.tracker.api)).
        // ClientClaimChangeListener.onDimensionChange(1): not translated, its class isn't translated (it implements `IClaimsManagerListenerAPI` (xaero.pac.common.claims.tracker.api)).
    }

    /// `ClaimsHighlighter` (xaero/common/mods/pac/highlight/ClaimsHighlighter.java)
    pub mod claims_highlighter {
        use super::*;
        // The class itself isn't translated: it extends `ChunkHighlighter` (xaero.common.minimap.highlight). Its static parts are below.
        // ClaimsHighlighter.ClaimsHighlighter(2): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.common.minimap.highlight)).
        // ClaimsHighlighter.regionHasHighlights(3): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.common.minimap.highlight)).
        // ClaimsHighlighter.getColors(3): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.common.minimap.highlight)).
        // ClaimsHighlighter.getChunkHighlitColor(3): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.common.minimap.highlight)).
        // ClaimsHighlighter.applyAnchorPixel(4): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.common.minimap.highlight)).
        // ClaimsHighlighter.chunkIsHighlit(3): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.common.minimap.highlight)).
        // ClaimsHighlighter.addChunkHighlightTooltips(5): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.common.minimap.highlight)).
        // ClaimsHighlighter.getClaimsCustomName(2): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.common.minimap.highlight)).
        // ClaimsHighlighter.getClaimsColor(2): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.common.minimap.highlight)).
    }

    /// `OPACPlayerTrackerSystem` (xaero/common/mods/pac/party/OPACPlayerTrackerSystem.java)
    pub mod opac_player_tracker_system {
        use super::*;
        // The class itself isn't translated: it implements `IRenderedPlayerTracker` (xaero.hud.minimap.player.tracker.system). Its static parts are below.
        // OPACPlayerTrackerSystem.OPACPlayerTrackerSystem(1): not translated, its class isn't translated (it implements `IRenderedPlayerTracker` (xaero.hud.minimap.player.tracker.system)).
        // OPACPlayerTrackerSystem.getReader(0): not translated, its class isn't translated (it implements `IRenderedPlayerTracker` (xaero.hud.minimap.player.tracker.system)).
        // OPACPlayerTrackerSystem.getTrackedPlayerIterator(0): not translated, its class isn't translated (it implements `IRenderedPlayerTracker` (xaero.hud.minimap.player.tracker.system)).
    }

    /// `OPACTrackedPlayerReader` (xaero/common/mods/pac/party/OPACTrackedPlayerReader.java)
    pub mod opac_tracked_player_reader {
        use super::*;
        // The class itself isn't translated: it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system). Its static parts are below.
        // OPACTrackedPlayerReader.getId(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)).
        // OPACTrackedPlayerReader.getX(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)).
        // OPACTrackedPlayerReader.getY(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)).
        // OPACTrackedPlayerReader.getZ(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)).
        // OPACTrackedPlayerReader.getDimension(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)).
    }

    /// `SupportOpenPartiesAndClaims` (xaero/common/mods/pac/SupportOpenPartiesAndClaims.java)
    pub mod support_open_parties_and_claims {
        use super::*;
        // The class itself isn't translated: it uses IXaeroMinimap, which isn't translated (it's an interface). Its static parts are below.
        // SupportOpenPartiesAndClaims.SupportOpenPartiesAndClaims(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportOpenPartiesAndClaims.register(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportOpenPartiesAndClaims.claimAt(3): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportOpenPartiesAndClaims.onMapRender(8): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportOpenPartiesAndClaims.isFromParty(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportOpenPartiesAndClaims.registerHighlighters(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportOpenPartiesAndClaims.getModMain(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportOpenPartiesAndClaims.getAllyIterator(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
    }

    /// `SupportAmecs` (xaero/common/mods/SupportAmecs.java)
    pub mod support_amecs {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SupportAmecs {
        }
        impl SupportAmecs {
            // SupportAmecs.SupportAmecs(1): not translated, it uses `Logger` (org.apache.logging.log4j).
            // SupportAmecs.modifiersArePressed(1): not translated, it uses Minecraft's `KeyMapping`.
        }
    }

    /// `SupportFramedBlocks` (xaero/common/mods/SupportFramedBlocks.java)
    pub mod support_framed_blocks {
        use super::*;
        // The class itself isn't translated: it uses `Class`. Its static parts are below.
        // SupportFramedBlocks.SupportFramedBlocks(0): not translated, its class isn't translated (it uses `Class`).
        // SupportFramedBlocks.onWorldChange(0): not translated, its class isn't translated (it uses `Class`).
        // SupportFramedBlocks.findFramedBlocks(2): not translated, its class isn't translated (it uses `Class`).
        // SupportFramedBlocks.isFrameBlock(3): not translated, its class isn't translated (it uses `Class`).
        // SupportFramedBlocks.unpackFramedBlock(4): not translated, its class isn't translated (it uses `Class`).
    }

    /// `SupportIris` (xaero/common/mods/SupportIris.java)
    pub mod support_iris {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SupportIris {
        }
        impl SupportIris {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `SupportMods` (xaero/common/mods/SupportMods.java)
    pub mod support_mods {
        use super::*;
        // The class itself isn't translated: it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)). Its static parts are below.
        // SupportMods.ensureCheckedLater(0): not translated, its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))).
        // SupportMods.worldmap(0): not translated, its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))).
        // SupportMods.pac(0): not translated, its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))).
        // SupportMods.shouldUseWorldMapChunks(0): not translated, its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))).
        // SupportMods.shouldUseWorldMapCaveChunks(0): not translated, its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))).
        // SupportMods.framedBlocks(0): not translated, its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))).
        // SupportMods.waystones(0): not translated, its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))).
        // SupportMods.getSupportWaystones(0): not translated, its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))).
        // SupportMods.checkForMinimapDuplicates(1): not translated, a try/catch (ClassNotFoundException).
        // SupportMods.SupportMods(1): not translated, its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))).
        // SupportMods.checkLater(0): not translated, its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))).
        // SupportMods.registerClientEvents(0): not translated, its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))).
        // SupportMods.onClientTick(0): not translated, its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))).
        // SupportMods.onMinimapSessionStarted(1): not translated, its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))).
    }

    /// `SupportModsFabric` (xaero/common/mods/SupportModsFabric.java)
    pub mod support_mods_fabric {
        use super::*;
        // The class itself isn't translated: it extends `SupportMods`. Its static parts are below.
        // SupportModsFabric.SupportModsFabric(1): not translated, its class isn't translated (it extends `SupportMods`).
        // SupportModsFabric.amecs(0): not translated, its class isn't translated (it extends `SupportMods`).
        // SupportModsFabric.wraithWaystones(0): not translated, its class isn't translated (it extends `SupportMods`).
        // SupportModsFabric.registerClientEvents(0): not translated, its class isn't translated (it extends `SupportMods`).
        // SupportModsFabric.onClientTick(0): not translated, its class isn't translated (it extends `SupportMods`).
        // SupportModsFabric.onMinimapSessionStarted(1): not translated, its class isn't translated (it extends `SupportMods`).
    }

    /// `SupportWraithWaystones` (xaero/common/mods/SupportWraithWaystones.java)
    pub mod support_wraith_waystones {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SupportWraithWaystones {
        }
        impl SupportWraithWaystones {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `SupportXaeroWorldmap` (xaero/common/mods/SupportXaeroWorldmap.java)
    pub mod support_xaero_worldmap {
        use super::*;
        // The class itself isn't translated: it uses IXaeroMinimap, which isn't translated (it's an interface). Its static parts are below.
        pub static WORLDMAP_COMPATIBILITY_VERSION: JCell<i32> = JCell::new(20);
        pub const MINIMAP_MW: &str = "minimap";
        // seedsUsed: not translated, it uses `MapTileChunk` (xaero.map.region).
        pub const BLACK: i32 = -16777216;
        pub const SLIME: i32 = -2142047936;
        // SupportXaeroWorldmap.SupportXaeroWorldmap(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.drawMinimap(14): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.renderChunks(23): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.bumpLoadedRegion(2): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.renderSlimeChunks(7): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getWorldMapWaypoints(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getWorldMapColours(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getWorldMapFlowers(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getWorldMapTerrainDepth(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getWorldMapTerrainSlopes(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getWorldMapBiomeColorsVanillaMode(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getWorldMapIgnoreHeightmaps(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.tryToGetMultiworldId(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getMultiworldIdUnsynced(2): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getPotentialMultiworldIds(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getMultiworldIds(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.tryToGetMultiworldName(2): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getMultiworldNameUnsynced(3): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.openSettings(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getSettingsScreen(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getSettingsScreen(2): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getMinimapBrightness(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.prepareMapTexturedRect(11): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getAdjustHeightForCarpetLikeBlocks(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.registerHighlighters(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.createRadarRenderWrapper(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.worldMapIsRenderingRadar(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getPartialYTeleport(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.isStainedGlassDisplayed(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.isMultiplayerMap(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getManualCaveStart(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.hasEnabledCaveLayers(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getCaveModeType(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getWorldMapScreenForOption(2): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getCaveModeDepth(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.isLegibleCaveMaps(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getBiomeBlending(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.confirmPlayerRadarRender(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getDisplayClaims(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getClaimsBorderOpacity(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getClaimsFillOpacity(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.toggleChunkClaims(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.caveLayersAreUsable(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.shouldPreventAutoCaveMode(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getMapDimensionScale(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SupportXaeroWorldmap.getMapDimension(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
    }

    /// `WorldMapHighlighter` (xaero/common/mods/WorldMapHighlighter.java)
    pub mod world_map_highlighter {
        use super::*;
        // The class itself isn't translated: it extends `AbstractHighlighter` (xaero.common.minimap.highlight). Its static parts are below.
        // WorldMapHighlighter.WorldMapHighlighter(1): not translated, its class isn't translated (it extends `AbstractHighlighter` (xaero.common.minimap.highlight)).
        // WorldMapHighlighter.regionHasHighlights(3): not translated, its class isn't translated (it extends `AbstractHighlighter` (xaero.common.minimap.highlight)).
        // WorldMapHighlighter.chunkIsHighlit(3): not translated, its class isn't translated (it extends `AbstractHighlighter` (xaero.common.minimap.highlight)).
        // WorldMapHighlighter.getChunkHighlitColor(3): not translated, its class isn't translated (it extends `AbstractHighlighter` (xaero.common.minimap.highlight)).
        // WorldMapHighlighter.addBlockHighlightTooltips(5): not translated, its class isn't translated (it extends `AbstractHighlighter` (xaero.common.minimap.highlight)).
        // WorldMapHighlighter.isCoveringOutsideDiscovered(0): not translated, its class isn't translated (it extends `AbstractHighlighter` (xaero.common.minimap.highlight)).
    }

    /// `PlatformContext` (xaero/common/PlatformContext.java)
    pub mod platform_context {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PlatformContext {
        }
        impl PlatformContext {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // PlatformContext.createClientEvents(1): not translated, it has no body.
            // PlatformContext.createCommonEvents(1): not translated, it has no body.
            // PlatformContext.getLoaderClientOnly(0): not translated, it has no body.
            // PlatformContext.getLoaderCommon(0): not translated, it has no body.
            // PlatformContext.createModClientEvents(1): not translated, it has no body.
            // PlatformContext.createSupportMods(1): not translated, it has no body.
            // PlatformContext.createModCommonEvents(1): not translated, it has no body.
            // PlatformContext.createMinimapWriter(4): not translated, it has no body.
            // PlatformContext.getModInfoVersion(0): not translated, it has no body.
            // PlatformContext.createSupportServerMods(1): not translated, it has no body.
        }
    }

    /// `PlatformContextFabric` (xaero/common/PlatformContextFabric.java)
    pub mod platform_context_fabric {
        use super::*;
        // The class itself isn't translated: it extends `PlatformContext`. Its static parts are below.
        // PlatformContextFabric.PlatformContextFabric(1): not translated, its class isn't translated (it extends `PlatformContext`).
        // PlatformContextFabric.postLoadClient(0): not translated, its class isn't translated (it extends `PlatformContext`).
        // PlatformContextFabric.postLoadCommon(0): not translated, its class isn't translated (it extends `PlatformContext`).
        // PlatformContextFabric.postLoadServer(0): not translated, its class isn't translated (it extends `PlatformContext`).
        // PlatformContextFabric.preTryLoadLater(0): not translated, its class isn't translated (it extends `PlatformContext`).
        // PlatformContextFabric.preTryLoadLaterServer(0): not translated, its class isn't translated (it extends `PlatformContext`).
        // PlatformContextFabric.createClientEvents(1): not translated, its class isn't translated (it extends `PlatformContext`).
        // PlatformContextFabric.createCommonEvents(1): not translated, its class isn't translated (it extends `PlatformContext`).
        // PlatformContextFabric.getLoaderClientOnly(0): not translated, its class isn't translated (it extends `PlatformContext`).
        // PlatformContextFabric.getLoaderCommon(0): not translated, its class isn't translated (it extends `PlatformContext`).
        // PlatformContextFabric.createModClientEvents(1): not translated, its class isn't translated (it extends `PlatformContext`).
        // PlatformContextFabric.createModCommonEvents(1): not translated, its class isn't translated (it extends `PlatformContext`).
        // PlatformContextFabric.createSupportMods(1): not translated, its class isn't translated (it extends `PlatformContext`).
        // PlatformContextFabric.createSupportServerMods(1): not translated, its class isn't translated (it extends `PlatformContext`).
        // PlatformContextFabric.createMinimapWriter(4): not translated, its class isn't translated (it extends `PlatformContext`).
        // PlatformContextFabric.getModInfoVersion(0): not translated, its class isn't translated (it extends `PlatformContext`).
        // PlatformContextFabric.get(0): not translated, it uses PlatformContextFabric, which isn't translated (it extends `PlatformContext`).
    }

    /// `PlatformContextLoaderClientOnly` (xaero/common/PlatformContextLoaderClientOnly.java)
    pub mod platform_context_loader_client_only {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PlatformContextLoaderClientOnly {
        }
        impl PlatformContextLoaderClientOnly {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // PlatformContextLoaderClientOnly.preInit(2): not translated, it has no body.
        }
    }

    /// `PlatformContextLoaderClientOnlyFabric` (xaero/common/PlatformContextLoaderClientOnlyFabric.java)
    pub mod platform_context_loader_client_only_fabric {
        use super::*;
        // The class itself isn't translated: it extends `PlatformContextLoaderClientOnly`. Its static parts are below.
        // PlatformContextLoaderClientOnlyFabric.preInit(2): not translated, its class isn't translated (it extends `PlatformContextLoaderClientOnly`).
    }

    /// `PlatformContextLoaderCommon` (xaero/common/PlatformContextLoaderCommon.java)
    pub mod platform_context_loader_common {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PlatformContextLoaderCommon {
        }
        impl PlatformContextLoaderCommon {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // PlatformContextLoaderCommon.setup(1): not translated, it has no body.
        }
    }

    /// `PlatformContextLoaderCommonFabric` (xaero/common/PlatformContextLoaderCommonFabric.java)
    pub mod platform_context_loader_common_fabric {
        use super::*;
        // The class itself isn't translated: it extends `PlatformContextLoaderCommon`. Its static parts are below.
        // PlatformContextLoaderCommonFabric.setup(1): not translated, its class isn't translated (it extends `PlatformContextLoaderCommon`).
    }

    /// `Services` (xaero/common/platform/Services.java)
    pub mod services {
        use super::*;
        // PLATFORM: not translated, it uses IPlatformHelper, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct Services {
        }
        impl Services {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Services.load(1): not translated, it's generic.
    }

    /// `FabricPlatformHelper` (xaero/common/platform/services/FabricPlatformHelper.java)
    pub mod fabric_platform_helper {
        use super::*;
        // The class itself isn't translated: it implements `IPlatformHelper`. Its static parts are below.
        // FabricPlatformHelper.getPlatformName(0): not translated, its class isn't translated (it implements `IPlatformHelper`).
        // FabricPlatformHelper.isModLoaded(1): not translated, its class isn't translated (it implements `IPlatformHelper`).
        // FabricPlatformHelper.isDevelopmentEnvironment(0): not translated, its class isn't translated (it implements `IPlatformHelper`).
        // FabricPlatformHelper.isDedicatedServer(0): not translated, its class isn't translated (it implements `IPlatformHelper`).
        // FabricPlatformHelper.getGameDir(0): not translated, its class isn't translated (it implements `IPlatformHelper`).
        // FabricPlatformHelper.getConfigDir(0): not translated, its class isn't translated (it implements `IPlatformHelper`).
        // FabricPlatformHelper.getModFile(1): not translated, its class isn't translated (it implements `IPlatformHelper`).
    }

    /// `IPlatformHelper` (xaero/common/platform/services/IPlatformHelper.java)
    pub mod i_platform_helper {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IPlatformHelper.getPlatformName(0): not translated, it has no body.
        // IPlatformHelper.isModLoaded(1): not translated, it has no body.
        // IPlatformHelper.checkModForMixin(1): not translated, its class isn't translated (it's an interface).
        // IPlatformHelper.isDevelopmentEnvironment(0): not translated, it has no body.
        // IPlatformHelper.getEnvironmentName(0): not translated, its class isn't translated (it's an interface).
        // IPlatformHelper.isDedicatedServer(0): not translated, it has no body.
        // IPlatformHelper.getGameDir(0): not translated, it has no body.
        // IPlatformHelper.getConfigDir(0): not translated, it has no body.
        // IPlatformHelper.getModFile(1): not translated, it has no body.
    }

    /// `MapPool` (xaero/common/pool/MapPool.java)
    pub mod map_pool {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MapPool.MapPool(1): not translated, its class isn't translated (it's generic).
        // MapPool.get(1): not translated, its class isn't translated (it's generic).
        // MapPool.addToPool(1): not translated, its class isn't translated (it's generic).
        // MapPool.takeFromPool(0): not translated, its class isn't translated (it's generic).
        // MapPool.size(0): not translated, its class isn't translated (it's generic).
        // MapPool.construct(1): not translated, it has no body.
    }

    /// `PoolUnit` (xaero/common/pool/PoolUnit.java)
    pub mod pool_unit {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // PoolUnit.create(1): not translated, it has no body.
    }

    /// `XaeroMinimapServerCore` (xaero/common/server/core/XaeroMinimapServerCore.java)
    pub mod xaero_minimap_server_core {
        use super::*;
        // modMain: not translated, it uses IXaeroMinimap, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct XaeroMinimapServerCore {
        }
        impl XaeroMinimapServerCore {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // XaeroMinimapServerCore.onServerWorldInfo(1): not translated, it uses Minecraft's `Player`.
        // XaeroMinimapServerCore.isModLoaded(0): not translated, it uses XaeroMinimapServerCore.modMain, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
    }

    /// `ServerEvents` (xaero/common/server/events/ServerEvents.java)
    pub mod server_events {
        use super::*;
        // The class itself isn't translated: it uses XaeroMinimapServer, which isn't translated (it uses `Logger` (org.apache.logging.log4j)). Its static parts are below.
        // ServerEvents.ServerEvents(1): not translated, it stores a second reference to an object (Rust would need it copied or shared).
        // ServerEvents.register(0): not translated, its class isn't translated (it uses XaeroMinimapServer, which isn't translated (it uses `Logger` (org.apache.logging.log4j))).
    }

    /// `IMinecraftServer` (xaero/common/server/IMinecraftServer.java)
    pub mod i_minecraft_server {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IMinecraftServer.getXaeroMinimapServerData(0): not translated, it has no body.
        // IMinecraftServer.setXaeroMinimapServerData(1): not translated, it has no body.
    }

    /// `LevelMapProperties` (xaero/common/server/level/LevelMapProperties.java)
    pub mod level_map_properties {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LevelMapProperties {
            pub id: i32,
            pub usable: bool,
        }
        impl LevelMapProperties {
            // LevelMapProperties.write(1): not translated, it's overloaded with the same number of parameters.
            // LevelMapProperties.read(1): not translated, it's overloaded with the same number of parameters.
            pub fn is_usable(&self) -> bool {
                return self.usable;
            }
            pub fn set_usable(&mut self, usable: bool) {
                self.usable = usable;
            }
            pub fn get_id(&self) -> i32 {
                return self.id;
            }
            // LevelMapProperties.write(1): not translated, it's overloaded with the same number of parameters.
        }
        // LevelMapProperties.read(1): not translated, it's overloaded with the same number of parameters.
    }

    /// `LevelMapPropertiesIO` (xaero/common/server/level/LevelMapPropertiesIO.java)
    pub mod level_map_properties_io {
        use super::*;
        pub const FILE_NAME: &str = "xaeromap.txt";
        #[derive(Clone, Debug)]
        pub struct LevelMapPropertiesIO {
        }
        impl LevelMapPropertiesIO {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // LevelMapPropertiesIO.load(2): not translated, a try/finally.
            // LevelMapPropertiesIO.save(2): not translated, a try-with-resources over something other than a file reader or writer.
        }
    }

    /// `MinecraftServerData` (xaero/common/server/MinecraftServerData.java)
    pub mod minecraft_server_data {
        use super::*;
        // The class itself isn't translated: a Map or Set keyed by a Path. Its static parts are below.
        // MinecraftServerData.MinecraftServerData(3): not translated, its class isn't translated (a Map or Set keyed by a Path).
        // MinecraftServerData.getLevelProperties(1): not translated, its class isn't translated (a Map or Set keyed by a Path).
        // MinecraftServerData.getSyncedPlayerTrackerSystemManager(0): not translated, its class isn't translated (a Map or Set keyed by a Path).
        // MinecraftServerData.getSyncedPlayerTracker(0): not translated, its class isn't translated (a Map or Set keyed by a Path).
        // MinecraftServerData.get(1): not translated, it uses MinecraftServerData, which isn't translated (a Map or Set keyed by a Path).
        // MinecraftServerData.getModMain(0): not translated, its class isn't translated (a Map or Set keyed by a Path).
    }

    /// `MineraftServerDataInitializer` (xaero/common/server/MineraftServerDataInitializer.java)
    pub mod mineraft_server_data_initializer {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MineraftServerDataInitializer {
        }
        impl MineraftServerDataInitializer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // MineraftServerDataInitializer.init(2): not translated, it uses Minecraft's `MinecraftServer`.
        }
    }

    /// `ArgonautsSyncedPlayerTrackerSystem` (xaero/common/server/mods/argonauts/ArgonautsSyncedPlayerTrackerSystem.java)
    pub mod argonauts_synced_player_tracker_system {
        use super::*;
        // The class itself isn't translated: it implements `ISyncedPlayerTrackerSystem` (xaero.common.server.radar.tracker). Its static parts are below.
        // ArgonautsSyncedPlayerTrackerSystem.getTrackingLevel(2): not translated, its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.common.server.radar.tracker)).
        // ArgonautsSyncedPlayerTrackerSystem.isPartySystem(0): not translated, its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.common.server.radar.tracker)).
        // ArgonautsSyncedPlayerTrackerSystem.getPartyTrackingLevel(2): not translated, its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.common.server.radar.tracker)).
        // ArgonautsSyncedPlayerTrackerSystem.getGuildTrackingLevel(2): not translated, its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.common.server.radar.tracker)).
    }

    /// `SupportArgonautsServer` (xaero/common/server/mods/argonauts/SupportArgonautsServer.java)
    pub mod support_argonauts_server {
        use super::*;
        // The class itself isn't translated: it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface). Its static parts are below.
        // SupportArgonautsServer.getSyncedPlayerTrackerSystem(0): not translated, its class isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)).
    }

    /// `FTBTeamsSyncedPlayerTrackerSystem` (xaero/common/server/mods/ftbteams/FTBTeamsSyncedPlayerTrackerSystem.java)
    pub mod ftb_teams_synced_player_tracker_system {
        use super::*;
        // The class itself isn't translated: it implements `ISyncedPlayerTrackerSystem` (xaero.common.server.radar.tracker). Its static parts are below.
        // FTBTeamsSyncedPlayerTrackerSystem.getTrackingLevel(2): not translated, its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.common.server.radar.tracker)).
        // FTBTeamsSyncedPlayerTrackerSystem.isPartySystem(0): not translated, its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.common.server.radar.tracker)).
    }

    /// `SupportFTBTeamsServer` (xaero/common/server/mods/ftbteams/SupportFTBTeamsServer.java)
    pub mod support_ftb_teams_server {
        use super::*;
        // The class itself isn't translated: it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface). Its static parts are below.
        // SupportFTBTeamsServer.getSyncedPlayerTrackerSystem(0): not translated, its class isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)).
    }

    /// `ServerPlayerOpacData` (xaero/common/server/mods/opac/ServerPlayerOpacData.java)
    pub mod server_player_opac_data {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ServerPlayerOpacData {
            pub share_location_with_party: bool,
            pub share_location_with_mutual_allies: bool,
        }
        impl ServerPlayerOpacData {
            pub fn new() -> Self {
                let mut s = Self { share_location_with_party: false, share_location_with_mutual_allies: false };
                s
            }
        }
    }

    /// `SupportOPACServer` (xaero/common/server/mods/opac/SupportOPACServer.java)
    pub mod support_opac_server {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SupportOPACServer {
        }
        impl SupportOPACServer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // SupportOPACServer.isPositionSyncAllowed(3): not translated, it uses ServerPlayerData, which isn't translated (it uses `UUID` (java.util)).
            // SupportOPACServer.getReceiveLocationsFromMutualAlliesConfigValue(1): not translated, it uses Minecraft's `ServerPlayer`.
            // SupportOPACServer.getReceiveLocationsFromPartyConfigValue(1): not translated, it uses Minecraft's `ServerPlayer`.
            // SupportOPACServer.updateShareLocationConfigValues(2): not translated, it uses Minecraft's `ServerPlayer`.
            // SupportOPACServer.getPlayerOpacData(1): not translated, it uses `Object`.
        }
    }

    /// `SupportServerMods` (xaero/common/server/mods/SupportServerMods.java)
    pub mod support_server_mods {
        use super::*;
        // The class itself isn't translated: it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)). Its static parts are below.
        // SupportServerMods.check(1): not translated, its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))).
        // SupportServerMods.hasFtbTeams(0): not translated, its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))).
        // SupportServerMods.getFtbTeams(0): not translated, its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))).
        // SupportServerMods.hasArgonauts(0): not translated, its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))).
        // SupportServerMods.getArgonauts(0): not translated, its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))).
        // SupportServerMods.hasOpac(0): not translated, its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))).
        // SupportServerMods.getOpac(0): not translated, its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))).
        // SupportServerMods.hasWorldmap(0): not translated, its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))).
        // SupportServerMods.getWorldmap(0): not translated, its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))).
        // SupportServerMods.registerEvents(0): not translated, its class isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))).
    }

    /// `SupportServerModsFabric` (xaero/common/server/mods/SupportServerModsFabric.java)
    pub mod support_server_mods_fabric {
        use super::*;
        // The class itself isn't translated: it extends `SupportServerMods`. Its static parts are below.
    }

    /// `SupportWorldMapServer` (xaero/common/server/mods/SupportWorldMapServer.java)
    pub mod support_world_map_server {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SupportWorldMapServer {
            pub compatibility_version: i32,
        }
        impl SupportWorldMapServer {
            pub fn supports_tracked_players(&self) -> bool {
                return self.compatibility_version >= 22;
            }
        }
    }

    /// `IServerPlayer` (xaero/common/server/player/IServerPlayer.java)
    pub mod i_server_player {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IServerPlayer.getXaeroMinimapPlayerData(0): not translated, it has no body.
        // IServerPlayer.setXaeroMinimapPlayerData(1): not translated, it has no body.
    }

    /// `ServerPlayerData` (xaero/common/server/player/ServerPlayerData.java)
    pub mod server_player_data {
        use super::*;
        // The class itself isn't translated: it uses `UUID` (java.util). Its static parts are below.
        // ServerPlayerData.ServerPlayerData(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.getLastSyncedData(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.ensureLastSyncedData(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.getCurrentlySyncedPlayers(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.ensureCurrentlySyncedPlayers(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.getLastTrackedPlayerSync(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.setLastTrackedPlayerSync(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.get(1): not translated, it uses ServerPlayerData, which isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.hasMod(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.setClientModNetworkVersion(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.getClientModNetworkVersion(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.setOpacData(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.getOpacData(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
    }

    /// `ServerPlayerTickHandler` (xaero/common/server/player/ServerPlayerTickHandler.java)
    pub mod server_player_tick_handler {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ServerPlayerTickHandler {
        }
        impl ServerPlayerTickHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // ServerPlayerTickHandler.tick(1): not translated, it uses Minecraft's `ServerPlayer`.
        }
    }

    /// `ISyncedPlayerTrackerSystem` (xaero/common/server/radar/tracker/ISyncedPlayerTrackerSystem.java)
    pub mod i_synced_player_tracker_system {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ISyncedPlayerTrackerSystem.getTrackingLevel(2): not translated, it has no body.
        // ISyncedPlayerTrackerSystem.isPartySystem(0): not translated, it has no body.
    }

    /// `SyncedPlayerTracker` (xaero/common/server/radar/tracker/SyncedPlayerTracker.java)
    pub mod synced_player_tracker {
        use super::*;
        // The class itself isn't translated: it uses IXaeroMinimap, which isn't translated (it's an interface). Its static parts are below.
        // SyncedPlayerTracker.SyncedPlayerTracker(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SyncedPlayerTracker.onTick(4): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SyncedPlayerTracker.sendRemovePacket(3): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // SyncedPlayerTracker.sendTrackedPlayerPacket(3): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
    }

    /// `SyncedPlayerTrackerSystemManager` (xaero/common/server/radar/tracker/SyncedPlayerTrackerSystemManager.java)
    pub mod synced_player_tracker_system_manager {
        use super::*;
        // The class itself isn't translated: it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface). Its static parts are below.
        // SyncedPlayerTrackerSystemManager.register(2): not translated, its class isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)).
        // SyncedPlayerTrackerSystemManager.getSystems(0): not translated, its class isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)).
    }

    /// `SyncedTrackedPlayer` (xaero/common/server/radar/tracker/SyncedTrackedPlayer.java)
    pub mod synced_tracked_player {
        use super::*;
        // The class itself isn't translated: it uses `UUID` (java.util). Its static parts are below.
        // SyncedTrackedPlayer.SyncedTrackedPlayer(5): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.setPos(3): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.setDimension(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.getId(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.getX(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.getY(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.getZ(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.getDimensionKey(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.matchesEnough(2): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.update(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.copyFrom(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
    }

    /// `XaeroMinimapServer` (xaero/common/server/XaeroMinimapServer.java)
    pub mod xaero_minimap_server {
        use super::*;
        // The class itself isn't translated: it uses `Logger` (org.apache.logging.log4j). Its static parts are below.
        pub static SERVER_COMPATIBILITY: JCell<i32> = JCell::new(1);
        // XaeroMinimapServer.XaeroMinimapServer(1): not translated, its class isn't translated (it uses `Logger` (org.apache.logging.log4j)).
        // XaeroMinimapServer.load(0): not translated, its class isn't translated (it uses `Logger` (org.apache.logging.log4j)).
        // XaeroMinimapServer.loadLater(0): not translated, its class isn't translated (it uses `Logger` (org.apache.logging.log4j)).
        // XaeroMinimapServer.getModMain(0): not translated, its class isn't translated (it uses `Logger` (org.apache.logging.log4j)).
        // XaeroMinimapServer.getServerEvents(0): not translated, its class isn't translated (it uses `Logger` (org.apache.logging.log4j)).
    }

    /// `ModSettings` (xaero/common/settings/ModSettings.java)
    pub mod mod_settings {
        use super::*;
        // The class itself isn't translated: it uses IXaeroMinimap, which isn't translated (it's an interface). Its static parts are below.
        pub static DEFAULT_SETTINGS: JCell<i32> = JCell::new(0i32);
        pub const FORMAT: &str = "\u{a7}";
        pub static SERVER_SETTINGS: JCell<i32> = JCell::new(0i32);
        // legacyServerSlimeSeeds: not translated, it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)).
        pub static SHOW_LIGHT_LEVEL_NAMES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["gui.xaero_off".to_string(), "gui.xaero_light_block".to_string(), "gui.xaero_light_sky".to_string(), "gui.xaero_light_all".to_string(), "gui.xaero_light_both2".to_string()]);
        pub static OLD_MINIMAP_SIZES: LazyLock<JCell<Vec<i32>>> = LazyLock::new(|| JCell::new(vec![57, 85, 113, 169]));
        // ModSettings.ModSettings(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.isKeyRepeat(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.getMinimap(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.setSlimeChunksSeed(2): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.getSlimeChunksSeed(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.getLegacySlimeChunksSeed(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.removeLegacySlimeChunksSeed(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.getSlimeChunks(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.waypointsGUI(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.getMinimapScale(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.getUIScale(3): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.getAutoUIScale(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.getMaxWaypointsDistance(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.assumeUsingFBO(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.isIgnoreHeightmaps(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.writeSettings(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.saveSettings(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.readSetting(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.loadDefaultSettings(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.loadSettings(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.loadSettingsFile(2): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.getTranslation(1): not translated, it uses `Object`.
        // ModSettings.refreshScreen(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.minimapDisabled(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.caveMapsDisabled(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.showWaypointsDisabled(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.deathpointsDisabled(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.resetServerSettings(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        pub fn set_server_settings() {
        }
        // ModSettings.canEditIngameSettings(0): not translated, it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // ModSettings.getEntityRadarBackwardsCompatibilityConfig(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.resetEntityRadarBackwardsCompatibilityConfig(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.foundOldRadarSettings(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.getLoadedWaypointLines(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // ModSettings.removeLoadedWaypointLines(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
    }

    /// `FieldValidatorHolder` (xaero/common/validator/FieldValidatorHolder.java)
    pub mod field_validator_holder {
        use super::*;
        // The class itself isn't translated: it uses WaypointCoordinateFieldValidator, which isn't translated (it extends `NumericFieldValidator`). Its static parts are below.
        // FieldValidatorHolder.FieldValidatorHolder(2): not translated, its class isn't translated (it uses WaypointCoordinateFieldValidator, which isn't translated (it extends `NumericFieldValidator`)).
        // FieldValidatorHolder.getNumericFieldValidator(0): not translated, its class isn't translated (it uses WaypointCoordinateFieldValidator, which isn't translated (it extends `NumericFieldValidator`)).
        // FieldValidatorHolder.getWpCoordFieldValidator(0): not translated, its class isn't translated (it uses WaypointCoordinateFieldValidator, which isn't translated (it extends `NumericFieldValidator`)).
    }

    /// `NumericFieldValidator` (xaero/common/validator/NumericFieldValidator.java)
    pub mod numeric_field_validator {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct NumericFieldValidator {
            pub string_builder: String,
        }
        impl NumericFieldValidator {
            pub fn new() -> Self {
                let mut s = Self { string_builder: String::new() };
                s
            }
            pub fn char_is_valid(&self, c: char, index: i32) -> bool {
                return ((c >= '0') && (c <= '9')) || ((c == '-') && (index == 0));
            }
            // NumericFieldValidator.validate(1): not translated, it uses Minecraft's `EditBox`.
            // NumericFieldValidator.checkNumberFormat(1): not translated, it calls `Integer`.parseInt.
        }
    }

    /// `WaypointCoordinateFieldValidator` (xaero/common/validator/WaypointCoordinateFieldValidator.java)
    pub mod waypoint_coordinate_field_validator {
        use super::*;
        // The class itself isn't translated: it extends `NumericFieldValidator`. Its static parts are below.
        // WaypointCoordinateFieldValidator.charIsValid(2): not translated, its class isn't translated (it extends `NumericFieldValidator`).
        // WaypointCoordinateFieldValidator.checkNumberFormat(1): not translated, its class isn't translated (it extends `NumericFieldValidator`).
    }

    /// `XaeroHudModMenu` (xaero/common/XaeroHudModMenu.java)
    pub mod xaero_hud_mod_menu {
        use super::*;
        // The class itself isn't translated: it implements `ModMenuApi` (com.terraformersmc.modmenu.api). Its static parts are below.
        // XaeroHudModMenu.getModConfigScreenFactory(0): not translated, its class isn't translated (it implements `ModMenuApi` (com.terraformersmc.modmenu.api)).
    }

    /// `XaeroMinimapSession` (xaero/common/XaeroMinimapSession.java)
    pub mod xaero_minimap_session {
        use super::*;
        // The class itself isn't translated: it extends `HudSession` (xaero.hud). Its static parts are below.
        // XaeroMinimapSession.XaeroMinimapSession(1): not translated, its class isn't translated (it extends `HudSession` (xaero.hud)).
        // XaeroMinimapSession.init(1): not translated, its class isn't translated (it extends `HudSession` (xaero.hud)).
        // XaeroMinimapSession.cleanup(0): not translated, its class isn't translated (it extends `HudSession` (xaero.hud)).
        // XaeroMinimapSession.getMinimapProcessor(0): not translated, its class isn't translated (it extends `HudSession` (xaero.hud)).
        // XaeroMinimapSession.getCurrentSession(0): not translated, it uses XaeroMinimapSession, which isn't translated (it extends `HudSession` (xaero.hud)).
        // XaeroMinimapSession.getForPlayer(1): not translated, it uses XaeroMinimapSession, which isn't translated (it extends `HudSession` (xaero.hud)).
        // XaeroMinimapSession.getModMain(0): not translated, its class isn't translated (it extends `HudSession` (xaero.hud)).
    }

    /// `FilterObjectCategory` (xaero/hud/category/FilterObjectCategory.java)
    pub mod filter_object_category {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FilterObjectCategory.FilterObjectCategory(12): not translated, its class isn't translated (it's generic).
        // FilterObjectCategory.getBaseRule(0): not translated, its class isn't translated (it's generic).
        // FilterObjectCategory.getIncludeList(1): not translated, it's generic.
        // FilterObjectCategory.getExcludeList(1): not translated, it's generic.
        // FilterObjectCategory.getIncludeLists(0): not translated, its class isn't translated (it's generic).
        // FilterObjectCategory.getExcludeLists(0): not translated, its class isn't translated (it's generic).
        // FilterObjectCategory.getExcludeMode(0): not translated, its class isn't translated (it's generic).
        // FilterObjectCategory.getIncludeInSuperCategory(0): not translated, its class isn't translated (it's generic).
    }

    /// `FilterObjectCategory.Builder` (xaero/hud/category/FilterObjectCategory.java)
    pub mod filter_object_category_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FilterObjectCategory.Builder.Builder(3): not translated, its class isn't translated (it's generic).
        // FilterObjectCategory.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // FilterObjectCategory.Builder.getIncludeListBuilder(1): not translated, it's generic.
        // FilterObjectCategory.Builder.getExcludeListBuilder(1): not translated, it's generic.
        // FilterObjectCategory.Builder.setBaseRule(1): not translated, its class isn't translated (it's generic).
        // FilterObjectCategory.Builder.setExcludeMode(1): not translated, its class isn't translated (it's generic).
        // FilterObjectCategory.Builder.setIncludeInSuperCategory(1): not translated, its class isn't translated (it's generic).
        // FilterObjectCategory.Builder.build(0): not translated, its class isn't translated (it's generic).
        // FilterObjectCategory.Builder.buildUnchecked(1): not translated, its class isn't translated (it's generic).
        // FilterObjectCategory.Builder.buildUncheckedFilter(5): not translated, it has no body.
    }

    /// `ObjectCategory` (xaero/hud/category/ObjectCategory.java)
    pub mod object_category {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategory.ObjectCategory(5): not translated, its class isn't translated (it's generic).
        // ObjectCategory.getName(0): not translated, its class isn't translated (it's generic).
        // ObjectCategory.getSuperCategory(0): not translated, its class isn't translated (it's generic).
        // ObjectCategory.setSuperCategory(1): not translated, its class isn't translated (it's generic).
        // ObjectCategory.getDirectSubCategoryIterator(0): not translated, its class isn't translated (it's generic).
        // ObjectCategory.getSettingValue(1): not translated, it's generic.
        // ObjectCategory.setSettingValue(2): not translated, it's generic.
        // ObjectCategory.getSettingOverridesIterator(0): not translated, its class isn't translated (it's generic).
        // ObjectCategory.getProtection(0): not translated, its class isn't translated (it's generic).
    }

    /// `ObjectCategory.Builder` (xaero/hud/category/ObjectCategory.java)
    pub mod object_category_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategory.Builder.Builder(2): not translated, its class isn't translated (it's generic).
        // ObjectCategory.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // ObjectCategory.Builder.setName(1): not translated, its class isn't translated (it's generic).
        // ObjectCategory.Builder.setSuperCategory(1): not translated, its class isn't translated (it's generic).
        // ObjectCategory.Builder.addSubCategoryBuilder(1): not translated, its class isn't translated (it's generic).
        // ObjectCategory.Builder.setSettingValue(2): not translated, it's generic.
        // ObjectCategory.Builder.setProtection(1): not translated, its class isn't translated (it's generic).
        // ObjectCategory.Builder.build(0): not translated, its class isn't translated (it's generic).
        // ObjectCategory.Builder.buildSubCategories(0): not translated, its class isn't translated (it's generic).
        // ObjectCategory.Builder.buildUnchecked(1): not translated, it has no body.
    }

    /// `ExcludeListMode` (xaero/hud/category/rule/ExcludeListMode.java)
    pub mod exclude_list_mode {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum ExcludeListMode {
            Only,
            AllBut,
        }
        impl ExcludeListMode {
            pub const VALUES: [ExcludeListMode; 2] = [ExcludeListMode::Only, ExcludeListMode::AllBut, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    ExcludeListMode::Only => "ONLY",
                    ExcludeListMode::AllBut => "ALL_BUT",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "ONLY" => ExcludeListMode::Only,
                    "ALL_BUT" => ExcludeListMode::AllBut,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `ObjectCategoryExcludeList` (xaero/hud/category/rule/ObjectCategoryExcludeList.java)
    pub mod object_category_exclude_list {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategoryExcludeList.ObjectCategoryExcludeList(4): not translated, its class isn't translated (it's generic).
        // ObjectCategoryExcludeList.isFollowedBy(2): not translated, its class isn't translated (it's generic).
        // ObjectCategoryExcludeList.getExcludeMode(0): not translated, its class isn't translated (it's generic).
    }

    /// `ObjectCategoryExcludeList.Builder` (xaero/hud/category/rule/ObjectCategoryExcludeList.java)
    pub mod object_category_exclude_list_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategoryExcludeList.Builder.Builder(2): not translated, its class isn't translated (it's generic).
        // ObjectCategoryExcludeList.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryExcludeList.Builder.setExcludeMode(1): not translated, its class isn't translated (it's generic).
        // ObjectCategoryExcludeList.Builder.build(3): not translated, it's generic.
        // ObjectCategoryExcludeList.Builder.build(1): not translated, it's generic.
        // ObjectCategoryExcludeList.Builder.buildInternally(1): not translated, its class isn't translated (it's generic).
        // ObjectCategoryExcludeList.Builder.begin(2): not translated, it's generic.
    }

    /// `ObjectCategoryHardRule` (xaero/hud/category/rule/ObjectCategoryHardRule.java)
    pub mod object_category_hard_rule {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategoryHardRule.ObjectCategoryHardRule(3): not translated, its class isn't translated (it's generic).
        // ObjectCategoryHardRule.isFollowedBy(2): not translated, its class isn't translated (it's generic).
    }

    /// `ObjectCategoryHardRule.Builder` (xaero/hud/category/rule/ObjectCategoryHardRule.java)
    pub mod object_category_hard_rule_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategoryHardRule.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryHardRule.Builder.setName(1): not translated, its class isn't translated (it's generic).
        // ObjectCategoryHardRule.Builder.setPredicate(1): not translated, its class isn't translated (it's generic).
        // ObjectCategoryHardRule.Builder.setReversed(1): not translated, its class isn't translated (it's generic).
        // ObjectCategoryHardRule.Builder.build(2): not translated, its class isn't translated (it's generic).
    }

    /// `ObjectCategoryHardRule.Predicate` (xaero/hud/category/rule/ObjectCategoryHardRule.java)
    pub mod object_category_hard_rule_predicate {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ObjectCategoryHardRule.Predicate.test(2): not translated, it has no body.
    }

    /// `ObjectCategoryIncludeList` (xaero/hud/category/rule/ObjectCategoryIncludeList.java)
    pub mod object_category_include_list {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategoryIncludeList.ObjectCategoryIncludeList(3): not translated, its class isn't translated (it's generic).
        // ObjectCategoryIncludeList.isFollowedBy(2): not translated, its class isn't translated (it's generic).
    }

    /// `ObjectCategoryIncludeList.Builder` (xaero/hud/category/rule/ObjectCategoryIncludeList.java)
    pub mod object_category_include_list_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategoryIncludeList.Builder.Builder(2): not translated, its class isn't translated (it's generic).
        // ObjectCategoryIncludeList.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryIncludeList.Builder.begin(2): not translated, it's generic.
        // ObjectCategoryIncludeList.Builder.build(3): not translated, it's generic.
        // ObjectCategoryIncludeList.Builder.build(1): not translated, it's generic.
        // ObjectCategoryIncludeList.Builder.buildInternally(1): not translated, its class isn't translated (it's generic).
    }

    /// `ObjectCategoryListRule` (xaero/hud/category/rule/ObjectCategoryListRule.java)
    pub mod object_category_list_rule {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategoryListRule.ObjectCategoryListRule(4): not translated, its class isn't translated (it's generic).
        // ObjectCategoryListRule.inList(2): not translated, its class isn't translated (it's generic).
        // ObjectCategoryListRule.iterator(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryListRule.getStringValidator(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryListRule.getType(0): not translated, its class isn't translated (it's generic).
    }

    /// `ObjectCategoryListRule.Builder` (xaero/hud/category/rule/ObjectCategoryListRule.java)
    pub mod object_category_list_rule_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategoryListRule.Builder.Builder(2): not translated, its class isn't translated (it's generic).
        // ObjectCategoryListRule.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryListRule.Builder.addListElement(1): not translated, its class isn't translated (it's generic).
        // ObjectCategoryListRule.Builder.getType(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryListRule.Builder.build(3): not translated, it's generic.
        // ObjectCategoryListRule.Builder.buildInternally(1): not translated, it has no body.
    }

    /// `ObjectCategoryListRuleType` (xaero/hud/category/rule/ObjectCategoryListRuleType.java)
    pub mod object_category_list_rule_type {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategoryListRuleType.ObjectCategoryListRuleType(9): not translated, its class isn't translated (it's generic).
        // ObjectCategoryListRuleType.getId(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryListRuleType.getGetter(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryListRuleType.getAllElementSupplier(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryListRuleType.getElementResolver(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryListRuleType.getStringFixer(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryListRuleType.getStringValidator(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryListRuleType.getSerializer(0): not translated, its class isn't translated (it's generic).
    }

    /// `ObjectCategoryRule` (xaero/hud/category/rule/ObjectCategoryRule.java)
    pub mod object_category_rule {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategoryRule.ObjectCategoryRule(1): not translated, its class isn't translated (it's generic).
        // ObjectCategoryRule.getName(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryRule.isFollowedBy(2): not translated, it has no body.
        // ObjectCategoryRule.toString(0): not translated, its class isn't translated (it's generic).
    }

    /// `ObjectCategoryRuleResolver` (xaero/hud/category/rule/resolver/ObjectCategoryRuleResolver.java)
    pub mod object_category_rule_resolver {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ObjectCategoryRuleResolver {
        }
        impl ObjectCategoryRuleResolver {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // ObjectCategoryRuleResolver.resolve(3): not translated, it's generic.
            // ObjectCategoryRuleResolver.followsRules(3): not translated, it's generic.
        }
    }

    /// `ObjectCategoryRuleResolver.Builder` (xaero/hud/category/rule/resolver/ObjectCategoryRuleResolver.java)
    pub mod object_category_rule_resolver_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // ObjectCategoryRuleResolver.Builder.setDefault(0): not translated, `this` used as a value.
            pub fn build(&self) -> object_category_rule_resolver::ObjectCategoryRuleResolver {
                return object_category_rule_resolver::ObjectCategoryRuleResolver::new();
            }
        }
        // ObjectCategoryRuleResolver.Builder.begin(0): not translated, it calls Builder.setDefault, which isn't translated.
    }

    /// `FilterObjectCategoryData` (xaero/hud/category/serialization/data/FilterObjectCategoryData.java)
    pub mod filter_object_category_data {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FilterObjectCategoryData.FilterObjectCategoryData(9): not translated, its class isn't translated (it's generic).
        // FilterObjectCategoryData.getIncludeListIterator(0): not translated, its class isn't translated (it's generic).
        // FilterObjectCategoryData.getExcludeListIterator(0): not translated, its class isn't translated (it's generic).
        // FilterObjectCategoryData.getHardInclude(0): not translated, its class isn't translated (it's generic).
        // FilterObjectCategoryData.getExcludeMode(0): not translated, its class isn't translated (it's generic).
        // FilterObjectCategoryData.getIncludeListInSuperCategory(0): not translated, its class isn't translated (it's generic).
    }

    /// `FilterObjectCategoryData.Builder` (xaero/hud/category/serialization/data/FilterObjectCategoryData.java)
    pub mod filter_object_category_data_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FilterObjectCategoryData.Builder.Builder(2): not translated, its class isn't translated (it's generic).
        // FilterObjectCategoryData.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // FilterObjectCategoryData.Builder.setHardInclude(1): not translated, its class isn't translated (it's generic).
        // FilterObjectCategoryData.Builder.setExcludeMode(1): not translated, its class isn't translated (it's generic).
        // FilterObjectCategoryData.Builder.addToIncludeList(1): not translated, its class isn't translated (it's generic).
        // FilterObjectCategoryData.Builder.addToExcludeList(1): not translated, its class isn't translated (it's generic).
        // FilterObjectCategoryData.Builder.setIncludeListInSuperCategory(1): not translated, its class isn't translated (it's generic).
        // FilterObjectCategoryData.Builder.build(0): not translated, its class isn't translated (it's generic).
    }

    /// `FilterObjectCategoryDataNbtSerializer` (xaero/hud/category/serialization/data/FilterObjectCategoryDataNbtSerializer.java)
    pub mod filter_object_category_data_nbt_serializer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FilterObjectCategoryDataNbtSerializer.FilterObjectCategoryDataNbtSerializer(1): not translated, its class isn't translated (it's generic).
        // FilterObjectCategoryDataNbtSerializer.serialize(1): not translated, its class isn't translated (it's generic).
        // FilterObjectCategoryDataNbtSerializer.getConfiguredBuilder(1): not translated, its class isn't translated (it's generic).
    }

    /// `FilterObjectCategoryDataNbtSerializer.Builder` (xaero/hud/category/serialization/data/FilterObjectCategoryDataNbtSerializer.java)
    pub mod filter_object_category_data_nbt_serializer_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FilterObjectCategoryDataNbtSerializer.Builder.Builder(0): not translated, its class isn't translated (it's generic).
        // FilterObjectCategoryDataNbtSerializer.Builder.build(0): not translated, its class isn't translated (it's generic).
    }

    /// `FilterObjectCategoryDataNbtSerializer.FinalBuilder` (xaero/hud/category/serialization/data/FilterObjectCategoryDataNbtSerializer.java)
    pub mod filter_object_category_data_nbt_serializer_final_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FilterObjectCategoryDataNbtSerializer.FinalBuilder.FinalBuilder(0): not translated, its class isn't translated (it's generic).
        // FilterObjectCategoryDataNbtSerializer.FinalBuilder.buildInternally(0): not translated, its class isn't translated (it's generic).
        // FilterObjectCategoryDataNbtSerializer.FinalBuilder.begin(0): not translated, it's generic.
    }

    /// `ObjectCategoryData` (xaero/hud/category/serialization/data/ObjectCategoryData.java)
    pub mod object_category_data {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategoryData.ObjectCategoryData(4): not translated, its class isn't translated (it's generic).
        // ObjectCategoryData.getName(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryData.getSettingOverrideIterator(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryData.getSubCategoryIterator(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryData.getProtection(0): not translated, its class isn't translated (it's generic).
    }

    /// `ObjectCategoryData.Builder` (xaero/hud/category/serialization/data/ObjectCategoryData.java)
    pub mod object_category_data_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategoryData.Builder.Builder(2): not translated, its class isn't translated (it's generic).
        // ObjectCategoryData.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryData.Builder.setName(1): not translated, its class isn't translated (it's generic).
        // ObjectCategoryData.Builder.setSettingOverride(2): not translated, its class isn't translated (it's generic).
        // ObjectCategoryData.Builder.addSubCategoryBuilder(1): not translated, its class isn't translated (it's generic).
        // ObjectCategoryData.Builder.setProtection(1): not translated, its class isn't translated (it's generic).
        // ObjectCategoryData.Builder.buildSubCategories(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryData.Builder.build(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryData.Builder.buildInternally(1): not translated, it has no body.
    }

    /// `ObjectCategoryDataGsonSerializer` (xaero/hud/category/serialization/data/ObjectCategoryDataGsonSerializer.java)
    pub mod object_category_data_gson_serializer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategoryDataGsonSerializer.ObjectCategoryDataGsonSerializer(2): not translated, its class isn't translated (it's generic).
        // ObjectCategoryDataGsonSerializer.serialize(1): not translated, its class isn't translated (it's generic).
        // ObjectCategoryDataGsonSerializer.deserialize(1): not translated, its class isn't translated (it's generic).
    }

    /// `ObjectCategoryDataGsonSerializer.Builder` (xaero/hud/category/serialization/data/ObjectCategoryDataGsonSerializer.java)
    pub mod object_category_data_gson_serializer_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategoryDataGsonSerializer.Builder.Builder(2): not translated, its class isn't translated (it's generic).
        // ObjectCategoryDataGsonSerializer.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryDataGsonSerializer.Builder.build(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryDataGsonSerializer.Builder.begin(2): not translated, it's generic.
    }

    /// `ObjectCategoryDataNbtSerializer` (xaero/hud/category/serialization/data/ObjectCategoryDataNbtSerializer.java)
    pub mod object_category_data_nbt_serializer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategoryDataNbtSerializer.ObjectCategoryDataNbtSerializer(1): not translated, its class isn't translated (it's generic).
        // ObjectCategoryDataNbtSerializer.serialize(1): not translated, its class isn't translated (it's generic).
        // ObjectCategoryDataNbtSerializer.deserialize(1): not translated, its class isn't translated (it's generic).
        // ObjectCategoryDataNbtSerializer.getConfiguredBuilder(1): not translated, its class isn't translated (it's generic).
    }

    /// `ObjectCategoryDataNbtSerializer.Builder` (xaero/hud/category/serialization/data/ObjectCategoryDataNbtSerializer.java)
    pub mod object_category_data_nbt_serializer_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategoryDataNbtSerializer.Builder.Builder(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryDataNbtSerializer.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryDataNbtSerializer.Builder.setBuilderSupplier(1): not translated, its class isn't translated (it's generic).
        // ObjectCategoryDataNbtSerializer.Builder.build(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryDataNbtSerializer.Builder.buildInternally(0): not translated, it has no body.
    }

    /// `ObjectCategoryDataNbtSerializer.FinalBuilder` (xaero/hud/category/serialization/data/ObjectCategoryDataNbtSerializer.java)
    pub mod object_category_data_nbt_serializer_final_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategoryDataNbtSerializer.FinalBuilder.FinalBuilder(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryDataNbtSerializer.FinalBuilder.buildInternally(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryDataNbtSerializer.FinalBuilder.begin(0): not translated, it's generic.
    }

    /// `ObjectCategoryDataSerializer` (xaero/hud/category/serialization/data/ObjectCategoryDataSerializer.java)
    pub mod object_category_data_serializer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategoryDataSerializer.ObjectCategoryDataSerializer(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryDataSerializer.serialize(1): not translated, it has no body.
        // ObjectCategoryDataSerializer.deserialize(1): not translated, it has no body.
    }

    /// `ObjectCategoryDataSerializer.Builder` (xaero/hud/category/serialization/data/ObjectCategoryDataSerializer.java)
    pub mod object_category_data_serializer_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategoryDataSerializer.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // ObjectCategoryDataSerializer.Builder.build(0): not translated, it has no body.
    }

    /// `FilterObjectCategorySerializationHandler` (xaero/hud/category/serialization/FilterObjectCategorySerializationHandler.java)
    pub mod filter_object_category_serialization_handler {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FilterObjectCategorySerializationHandler.FilterObjectCategorySerializationHandler(8): not translated, its class isn't translated (it's generic).
        // FilterObjectCategorySerializationHandler.getConfiguredDataBuilderForCategory(1): not translated, its class isn't translated (it's generic).
        // FilterObjectCategorySerializationHandler.getListRulePrefix(1): not translated, its class isn't translated (it's generic).
        // FilterObjectCategorySerializationHandler.getConfiguredCategoryBuilderForData(1): not translated, its class isn't translated (it's generic).
        // FilterObjectCategorySerializationHandler.handleListRuleSerializedElement(2): not translated, its class isn't translated (it's generic).
        // FilterObjectCategorySerializationHandler.handleListRuleSerializedElement(5): not translated, it's generic.
    }

    /// `FilterObjectCategorySerializationHandler.Builder` (xaero/hud/category/serialization/FilterObjectCategorySerializationHandler.java)
    pub mod filter_object_category_serialization_handler_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FilterObjectCategorySerializationHandler.Builder.Builder(1): not translated, its class isn't translated (it's generic).
        // FilterObjectCategorySerializationHandler.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // FilterObjectCategorySerializationHandler.Builder.setDefaultListRuleType(1): not translated, its class isn't translated (it's generic).
        // FilterObjectCategorySerializationHandler.Builder.setHardRuleGetter(1): not translated, its class isn't translated (it's generic).
        // FilterObjectCategorySerializationHandler.Builder.setListRuleTypeGetter(1): not translated, its class isn't translated (it's generic).
        // FilterObjectCategorySerializationHandler.Builder.setListRuleTypePrefixSeparator(1): not translated, its class isn't translated (it's generic).
        // FilterObjectCategorySerializationHandler.Builder.build(0): not translated, its class isn't translated (it's generic).
    }

    /// `ObjectCategorySerializationHandler` (xaero/hud/category/serialization/ObjectCategorySerializationHandler.java)
    pub mod object_category_serialization_handler {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategorySerializationHandler.ObjectCategorySerializationHandler(4): not translated, its class isn't translated (it's generic).
        // ObjectCategorySerializationHandler.serialize(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySerializationHandler.convertToData(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySerializationHandler.getConfiguredDataBuilderForCategory(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySerializationHandler.convertFromData(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySerializationHandler.deserialize(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySerializationHandler.getConfiguredCategoryBuilderForData(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySerializationHandler.setSettingValue(3): not translated, it's generic.
    }

    /// `ObjectCategorySerializationHandler.Builder` (xaero/hud/category/serialization/ObjectCategorySerializationHandler.java)
    pub mod object_category_serialization_handler_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategorySerializationHandler.Builder.Builder(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySerializationHandler.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // ObjectCategorySerializationHandler.Builder.setDataBuilderFactory(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySerializationHandler.Builder.setObjectCategoryBuilderFactory(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySerializationHandler.Builder.setSettingTypeGetter(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySerializationHandler.Builder.build(0): not translated, its class isn't translated (it's generic).
        // ObjectCategorySerializationHandler.Builder.buildInternally(0): not translated, it has no body.
    }

    /// `ObjectCategoryDefaultSettingsSetter` (xaero/hud/category/setting/ObjectCategoryDefaultSettingsSetter.java)
    pub mod object_category_default_settings_setter {
        use super::*;
        // The class itself isn't translated: it uses ObjectCategorySetting, which isn't translated (it's generic). Its static parts are below.
        // ObjectCategoryDefaultSettingsSetter.ObjectCategoryDefaultSettingsSetter(1): not translated, its class isn't translated (it uses ObjectCategorySetting, which isn't translated (it's generic)).
        // ObjectCategoryDefaultSettingsSetter.setDefaultsFor(2): not translated, its class isn't translated (it uses ObjectCategorySetting, which isn't translated (it's generic)).
        // ObjectCategoryDefaultSettingsSetter.setForSetting(2): not translated, it's generic.
    }

    /// `ObjectCategoryDefaultSettingsSetter.Builder` (xaero/hud/category/setting/ObjectCategoryDefaultSettingsSetter.java)
    pub mod object_category_default_settings_setter_builder {
        use super::*;
        // The class itself isn't translated: it uses ObjectCategorySetting, which isn't translated (it's generic). Its static parts are below.
        // ObjectCategoryDefaultSettingsSetter.Builder.Builder(0): not translated, its class isn't translated (it uses ObjectCategorySetting, which isn't translated (it's generic)).
        // ObjectCategoryDefaultSettingsSetter.Builder.setDefault(0): not translated, its class isn't translated (it uses ObjectCategorySetting, which isn't translated (it's generic)).
        // ObjectCategoryDefaultSettingsSetter.Builder.setSettings(1): not translated, its class isn't translated (it uses ObjectCategorySetting, which isn't translated (it's generic)).
        // ObjectCategoryDefaultSettingsSetter.Builder.build(0): not translated, its class isn't translated (it uses ObjectCategorySetting, which isn't translated (it's generic)).
        // ObjectCategoryDefaultSettingsSetter.Builder.begin(0): not translated, it uses Builder, which isn't translated (it uses ObjectCategorySetting, which isn't translated (it's generic)).
    }

    /// `ObjectCategorySetting` (xaero/hud/category/setting/ObjectCategorySetting.java)
    pub mod object_category_setting {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategorySetting.ObjectCategorySetting(10): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.getSettingUIType(0): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.getId(0): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.getDisplayName(0): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.getDefaultValue(0): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.getTooltip(0): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.getUiFirstOption(0): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.getUiLastOption(0): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.getIndexReader(0): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.getIndexWriter(0): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.getWidgetValueNameProvider(0): not translated, its class isn't translated (it's generic).
    }

    /// `ObjectCategorySetting.Builder` (xaero/hud/category/setting/ObjectCategorySetting.java)
    pub mod object_category_setting_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ObjectCategorySetting.Builder.Builder(0): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.Builder.setId(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.Builder.setDisplayName(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.Builder.setDefaultValue(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.Builder.setSettingUIType(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.Builder.setTooltip(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.Builder.setUiFirstOption(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.Builder.setUiLastOption(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.Builder.setIndexReader(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.Builder.setIndexWriter(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.Builder.setUiValueNameProvider(1): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.Builder.build(2): not translated, its class isn't translated (it's generic).
        // ObjectCategorySetting.Builder.begin(0): not translated, it's generic.
    }

    /// `EditorCategoryNodeConverter` (xaero/hud/category/ui/EditorCategoryNodeConverter.java)
    pub mod editor_category_node_converter {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorCategoryNodeConverter.EditorCategoryNodeConverter(2): not translated, its class isn't translated (it's generic).
        // EditorCategoryNodeConverter.convert(2): not translated, its class isn't translated (it's generic).
        // EditorCategoryNodeConverter.getConfiguredBuilder(2): not translated, its class isn't translated (it's generic).
        // EditorCategoryNodeConverter.setSettingValue(3): not translated, it's generic.
        // EditorCategoryNodeConverter.convert(1): not translated, its class isn't translated (it's generic).
        // EditorCategoryNodeConverter.getConfiguredBuilder(1): not translated, its class isn't translated (it's generic).
        // EditorCategoryNodeConverter.setSettingValue(3): not translated, it's generic.
    }

    /// `EditorCategoryNodeConverter.Builder` (xaero/hud/category/ui/EditorCategoryNodeConverter.java)
    pub mod editor_category_node_converter_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorCategoryNodeConverter.Builder.Builder(2): not translated, its class isn't translated (it's generic).
        // EditorCategoryNodeConverter.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorCategoryNodeConverter.Builder.build(0): not translated, its class isn't translated (it's generic).
        // EditorCategoryNodeConverter.Builder.buildInternally(0): not translated, it has no body.
    }

    /// `EditorFilterCategoryNodeConverter` (xaero/hud/category/ui/EditorFilterCategoryNodeConverter.java)
    pub mod editor_filter_category_node_converter {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorFilterCategoryNodeConverter.EditorFilterCategoryNodeConverter(6): not translated, its class isn't translated (it's generic).
        // EditorFilterCategoryNodeConverter.getConfiguredBuilder(2): not translated, its class isn't translated (it's generic).
        // EditorFilterCategoryNodeConverter.getListRulePrefix(1): not translated, its class isn't translated (it's generic).
        // EditorFilterCategoryNodeConverter.getConfiguredBuilder(1): not translated, its class isn't translated (it's generic).
    }

    /// `EditorFilterCategoryNodeConverter.Builder` (xaero/hud/category/ui/EditorFilterCategoryNodeConverter.java)
    pub mod editor_filter_category_node_converter_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorFilterCategoryNodeConverter.Builder.Builder(2): not translated, its class isn't translated (it's generic).
        // EditorFilterCategoryNodeConverter.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorFilterCategoryNodeConverter.Builder.setDefaultListRuleType(1): not translated, its class isn't translated (it's generic).
        // EditorFilterCategoryNodeConverter.Builder.setListRuleTypeGetter(1): not translated, its class isn't translated (it's generic).
        // EditorFilterCategoryNodeConverter.Builder.setListRuleTypePrefixSeparator(1): not translated, its class isn't translated (it's generic).
        // EditorFilterCategoryNodeConverter.Builder.setInputRuleTypeStringValidator(1): not translated, its class isn't translated (it's generic).
        // EditorFilterCategoryNodeConverter.Builder.build(0): not translated, its class isn't translated (it's generic).
        // EditorFilterCategoryNodeConverter.Builder.buildInternally(0): not translated, it has no body.
    }

    /// `ConnectionLineType` (xaero/hud/category/ui/entry/ConnectionLineType.java)
    pub mod connection_line_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum ConnectionLineType {
            Path,
            HeadLeaf,
            TailLeaf,
            None,
        }
        impl ConnectionLineType {
            pub const VALUES: [ConnectionLineType; 4] = [ConnectionLineType::Path, ConnectionLineType::HeadLeaf, ConnectionLineType::TailLeaf, ConnectionLineType::None, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    ConnectionLineType::Path => "PATH",
                    ConnectionLineType::HeadLeaf => "HEAD_LEAF",
                    ConnectionLineType::TailLeaf => "TAIL_LEAF",
                    ConnectionLineType::None => "NONE",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "PATH" => ConnectionLineType::Path,
                    "HEAD_LEAF" => ConnectionLineType::HeadLeaf,
                    "TAIL_LEAF" => ConnectionLineType::TailLeaf,
                    "NONE" => ConnectionLineType::None,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `EditorListEntry` (xaero/hud/category/ui/entry/EditorListEntry.java)
    pub mod editor_list_entry {
        use super::*;
        // The class itself isn't translated: it holds an object of its own class (a linked structure). Its static parts are below.
        // EditorListEntry.EditorListEntry(7): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.onSelected(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.mouseClicked(6): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.confirmSelection(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.mouseReleased(4): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.mouseScrolled(4): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.mouseDragged(6): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.mouseMoved(2): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.keyPressed(2): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.keyReleased(1): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.charTyped(1): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.tick(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.getSubNarration(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.getHoveredNarration(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.getSelectedNarration(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.getTooltipSupplier(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.getMessage(0): not translated, it has no body.
        // EditorListEntry.getNarrationMessage(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.getNarration(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.getHoverNarration(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.preRender(3): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.render(13): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.postRender(1): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.isHoveredOver(2): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.selectAction(0): not translated, it has no body.
        // EditorListEntry.setFocused(1): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.unhoverRecursively(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.moveFocus(1): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.moveFocus(2): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.unfocusRecursively(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.focusFirstRecursively(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.focusLastRecursively(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.withSubEntry(1): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.getEntryRelativeX(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EditorListEntry.getEntryRelativeY(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
    }

    /// `EditorListEntryCategory` (xaero/hud/category/ui/entry/EditorListEntryCategory.java)
    pub mod editor_list_entry_category {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // HELP_TOOLTIP: not translated, it uses `Tooltip` (xaero.lib.client.gui.widget).
        // PROTECTED_TOOLTIP: not translated, it uses `Tooltip` (xaero.lib.client.gui.widget).
        // UP_TOOLTIP: not translated, it uses `Tooltip` (xaero.lib.client.gui.widget).
        // DOWN_TOOLTIP: not translated, it uses `Tooltip` (xaero.lib.client.gui.widget).
        // EditorListEntryCategory.EditorListEntryCategory(8): not translated, its class isn't translated (it's generic).
        // EditorListEntryCategory.getCategoryNameEntryFactory(4): not translated, its class isn't translated (it's generic).
        // EditorListEntryCategory.getPasteEntryFactory(4): not translated, its class isn't translated (it's generic).
        // EditorListEntryCategory.getPasteTooltip(2): not translated, its class isn't translated (it's generic).
        // EditorListEntryCategory.getDuplicateEntryFactory(3): not translated, its class isn't translated (it's generic).
        // EditorListEntryCategory.getProtectedEntryFactory(0): not translated, its class isn't translated (it's generic).
        // EditorListEntryCategory.getCutEntryFactory(3): not translated, its class isn't translated (it's generic).
        // EditorListEntryCategory.getPriorityEntryFactory(3): not translated, its class isn't translated (it's generic).
        // EditorListEntryCategory.getMessage(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorListEntryDeletableListElement` (xaero/hud/category/ui/entry/EditorListEntryDeletableListElement.java)
    pub mod editor_list_entry_deletable_list_element {
        use super::*;
        // The class itself isn't translated: it extends `EditorListRootEntry`. Its static parts are below.
        // DELETE_TOOLTIP: not translated, it uses `Tooltip` (xaero.lib.client.gui.widget).
        // EditorListEntryDeletableListElement.EditorListEntryDeletableListElement(9): not translated, its class isn't translated (it extends `EditorListRootEntry`).
        // EditorListEntryDeletableListElement.keyPressed(2): not translated, its class isn't translated (it extends `EditorListRootEntry`).
        // EditorListEntryDeletableListElement.getMessage(0): not translated, its class isn't translated (it extends `EditorListRootEntry`).
    }

    /// `EditorListEntryExpandingOption` (xaero/hud/category/ui/entry/EditorListEntryExpandingOption.java)
    pub mod editor_list_entry_expanding_option {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorListEntryExpandingOption.EditorListEntryExpandingOption(9): not translated, its class isn't translated (it's generic).
        // EditorListEntryExpandingOption.selectAction(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorListEntryExpandingOptions` (xaero/hud/category/ui/entry/EditorListEntryExpandingOptions.java)
    pub mod editor_list_entry_expanding_options {
        use super::*;
        // The class itself isn't translated: it extends `EditorListEntryWidget`. Its static parts are below.
        // EditorListEntryExpandingOptions.EditorListEntryExpandingOptions(10): not translated, its class isn't translated (it extends `EditorListEntryWidget`).
    }

    /// `EditorListEntryTextWithAction` (xaero/hud/category/ui/entry/EditorListEntryTextWithAction.java)
    pub mod editor_list_entry_text_with_action {
        use super::*;
        // The class itself isn't translated: it extends `EditorListEntryWithIconAndText`. Its static parts are below.
        // EditorListEntryTextWithAction.EditorListEntryTextWithAction(9): not translated, its class isn't translated (it extends `EditorListEntryWithIconAndText`).
        // EditorListEntryTextWithAction.EditorListEntryTextWithAction(10): not translated, its class isn't translated (it extends `EditorListEntryWithIconAndText`).
        // EditorListEntryTextWithAction.selectAction(0): not translated, its class isn't translated (it extends `EditorListEntryWithIconAndText`).
    }

    /// `EditorListEntryWidget` (xaero/hud/category/ui/entry/EditorListEntryWidget.java)
    pub mod editor_list_entry_widget {
        use super::*;
        // The class itself isn't translated: it extends `EditorListEntryWithRootReference`. Its static parts are below.
        // EditorListEntryWidget.EditorListEntryWidget(9): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWidget.mouseClicked(6): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWidget.mouseReleased(4): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWidget.mouseDragged(6): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWidget.mouseScrolled(4): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWidget.mouseMoved(2): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWidget.keyPressed(2): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWidget.keyReleased(1): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWidget.charTyped(1): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWidget.tick(0): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWidget.getNarration(0): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWidget.getHoverNarration(0): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWidget.getMessage(0): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWidget.getNarrationMessage(0): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWidget.render(13): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWidget.setFocused(1): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWidget.selectAction(0): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
    }

    /// `EditorListEntryWithIcon` (xaero/hud/category/ui/entry/EditorListEntryWithIcon.java)
    pub mod editor_list_entry_with_icon {
        use super::*;
        // The class itself isn't translated: it extends `EditorListEntryWithRootReference`. Its static parts are below.
        // EditorListEntryWithIcon.EditorListEntryWithIcon(12): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWithIcon.getIconX(0): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWithIcon.getIconY(0): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWithIcon.getIconW(0): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWithIcon.getIconH(0): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWithIcon.render(13): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWithIcon.selectAction(0): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListEntryWithIcon.getMessage(0): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
    }

    /// `EditorListEntryWithIconAndText` (xaero/hud/category/ui/entry/EditorListEntryWithIconAndText.java)
    pub mod editor_list_entry_with_icon_and_text {
        use super::*;
        // The class itself isn't translated: it extends `EditorListEntryWithIcon`. Its static parts are below.
        // EditorListEntryWithIconAndText.EditorListEntryWithIconAndText(9): not translated, its class isn't translated (it extends `EditorListEntryWithIcon`).
        // EditorListEntryWithIconAndText.setColor(1): not translated, its class isn't translated (it extends `EditorListEntryWithIcon`).
        // EditorListEntryWithIconAndText.setHoverColor(1): not translated, its class isn't translated (it extends `EditorListEntryWithIcon`).
        // EditorListEntryWithIconAndText.getColor(0): not translated, its class isn't translated (it extends `EditorListEntryWithIcon`).
        // EditorListEntryWithIconAndText.getHoverColor(0): not translated, its class isn't translated (it extends `EditorListEntryWithIcon`).
        // EditorListEntryWithIconAndText.render(13): not translated, its class isn't translated (it extends `EditorListEntryWithIcon`).
        // EditorListEntryWithIconAndText.getMessage(0): not translated, its class isn't translated (it extends `EditorListEntryWithIcon`).
    }

    /// `EditorListEntryWithRootReference` (xaero/hud/category/ui/entry/EditorListEntryWithRootReference.java)
    pub mod editor_list_entry_with_root_reference {
        use super::*;
        // The class itself isn't translated: it extends `EditorListEntry`. Its static parts are below.
        // EditorListEntryWithRootReference.EditorListEntryWithRootReference(8): not translated, its class isn't translated (it extends `EditorListEntry`).
    }

    /// `EditorListEntryWrapper` (xaero/hud/category/ui/entry/EditorListEntryWrapper.java)
    pub mod editor_list_entry_wrapper {
        use super::*;
        // The class itself isn't translated: it extends `EditorListRootEntry`. Its static parts are below.
        // EditorListEntryWrapper.EditorListEntryWrapper(6): not translated, its class isn't translated (it extends `EditorListRootEntry`).
        // EditorListEntryWrapper.EditorListEntryWrapper(7): not translated, its class isn't translated (it extends `EditorListRootEntry`).
        // EditorListEntryWrapper.getMessage(0): not translated, its class isn't translated (it extends `EditorListRootEntry`).
    }

    /// `EditorListRootEntry` (xaero/hud/category/ui/entry/EditorListRootEntry.java)
    pub mod editor_list_root_entry {
        use super::*;
        // The class itself isn't translated: it extends `EditorListEntry`. Its static parts are below.
        // EditorListRootEntry.EditorListRootEntry(5): not translated, its class isn't translated (it extends `EditorListEntry`).
        // EditorListRootEntry.addHelpElement(1): not translated, its class isn't translated (it extends `EditorListEntry`).
        // EditorListRootEntry.render(13): not translated, its class isn't translated (it extends `EditorListEntry`).
        // EditorListRootEntry.withSubEntry(1): not translated, its class isn't translated (it extends `EditorListEntry`).
        // EditorListRootEntry.selectAction(0): not translated, its class isn't translated (it extends `EditorListEntry`).
    }

    /// `EditorListRootEntry.CenteredEntryFactory` (xaero/hud/category/ui/entry/EditorListRootEntry.java)
    pub mod editor_list_root_entry_centered_entry_factory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // EditorListRootEntry.CenteredEntryFactory.get(5): not translated, it has no body.
    }

    /// `EditorListRootEntryFactory` (xaero/hud/category/ui/entry/EditorListRootEntryFactory.java)
    pub mod editor_list_root_entry_factory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // EditorListRootEntryFactory.get(7): not translated, it has no body.
    }

    /// `EditorListTextButtonEntry` (xaero/hud/category/ui/entry/EditorListTextButtonEntry.java)
    pub mod editor_list_text_button_entry {
        use super::*;
        // The class itself isn't translated: it extends `EditorListEntryWithRootReference`. Its static parts are below.
        // EditorListTextButtonEntry.EditorListTextButtonEntry(11): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListTextButtonEntry.render(13): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListTextButtonEntry.selectAction(0): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
        // EditorListTextButtonEntry.getMessage(0): not translated, its class isn't translated (it extends `EditorListEntryWithRootReference`).
    }

    /// `EditorButton` (xaero/hud/category/ui/entry/widget/EditorButton.java)
    pub mod editor_button {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Plain`. Its static parts are below.
        // EditorButton.EditorButton(6): not translated, its class isn't translated (it extends Minecraft's `Plain`).
        // EditorButton.EditorButton(7): not translated, its class isn't translated (it extends Minecraft's `Plain`).
        // EditorButton.updateMessage(0): not translated, its class isn't translated (it extends Minecraft's `Plain`).
        // EditorButton.createNarrationMessage(0): not translated, its class isn't translated (it extends Minecraft's `Plain`).
    }

    /// `EditorButton.PressActionWithContext` (xaero/hud/category/ui/entry/widget/EditorButton.java)
    pub mod editor_button_press_action_with_context {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `OnPress`. Its static parts are below.
        // EditorButton.PressActionWithContext.onPress(1): not translated, its class isn't translated (it implements Minecraft's `OnPress`).
        // EditorButton.PressActionWithContext.onPress(3): not translated, it has no body.
    }

    /// `EditorButtonIteration` (xaero/hud/category/ui/entry/widget/EditorButtonIteration.java)
    pub mod editor_button_iteration {
        use super::*;
        // The class itself isn't translated: it extends `EditorButton`. Its static parts are below.
        // EditorButtonIteration.EditorButtonIteration(9): not translated, its class isn't translated (it extends `EditorButton`).
        // EditorButtonIteration.toggle(0): not translated, its class isn't translated (it extends `EditorButton`).
        // EditorButtonIteration.iterate(1): not translated, its class isn't translated (it extends `EditorButton`).
        // EditorButtonIteration.putCurrentIndexInRange(0): not translated, its class isn't translated (it extends `EditorButton`).
        // EditorButtonIteration.mouseScrolled(4): not translated, its class isn't translated (it extends `EditorButton`).
    }

    /// `EditorSlider` (xaero/hud/category/ui/entry/widget/EditorSlider.java)
    pub mod editor_slider {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractOptionSliderButton`. Its static parts are below.
        // EditorSlider.EditorSlider(7): not translated, its class isn't translated (it extends Minecraft's `AbstractOptionSliderButton`).
        // EditorSlider.keyPressed(1): not translated, its class isn't translated (it extends Minecraft's `AbstractOptionSliderButton`).
        // EditorSlider.manualOptionChange(1): not translated, its class isn't translated (it extends Minecraft's `AbstractOptionSliderButton`).
        // EditorSlider.createNarrationMessage(0): not translated, its class isn't translated (it extends Minecraft's `AbstractOptionSliderButton`).
        // EditorSlider.applyValue(0): not translated, its class isn't translated (it extends Minecraft's `AbstractOptionSliderButton`).
        // EditorSlider.updateMessage(0): not translated, its class isn't translated (it extends Minecraft's `AbstractOptionSliderButton`).
        // EditorSlider.toSliderValue(1): not translated, its class isn't translated (it extends Minecraft's `AbstractOptionSliderButton`).
        // EditorSlider.toValue(1): not translated, its class isn't translated (it extends Minecraft's `AbstractOptionSliderButton`).
        // EditorSlider.clamp(1): not translated, its class isn't translated (it extends Minecraft's `AbstractOptionSliderButton`).
    }

    /// `EditorTextField` (xaero/hud/category/ui/entry/widget/EditorTextField.java)
    pub mod editor_text_field {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EditBox`. Its static parts are below.
        // EditorTextField.EditorTextField(11): not translated, its class isn't translated (it extends Minecraft's `EditBox`).
        // EditorTextField.updateColorForValue(1): not translated, its class isn't translated (it extends Minecraft's `EditBox`).
        // EditorTextField.setCursorPosition(1): not translated, its class isn't translated (it extends Minecraft's `EditBox`).
        // EditorTextField.setHighlightPos(1): not translated, its class isn't translated (it extends Minecraft's `EditBox`).
        // EditorTextField.createNarrationMessage(0): not translated, its class isn't translated (it extends Minecraft's `EditBox`).
        // EditorTextField.extractWidgetRenderState(4): not translated, its class isn't translated (it extends Minecraft's `EditBox`).
    }

    /// `EditorTextField.UpdatedValueConsumer` (xaero/hud/category/ui/entry/widget/EditorTextField.java)
    pub mod editor_text_field_updated_value_consumer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // EditorTextField.UpdatedValueConsumer.accept(4): not translated, it has no body.
    }

    /// `GuiCategoryEditor` (xaero/hud/category/ui/GuiCategoryEditor.java)
    pub mod gui_category_editor {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // READ_ONLY_COMPONENT: a game value, so it's in game_side.rs.
        pub const FRAME_TOP_SIZE: i32 = 32;
        pub const FRAME_BOTTOM_SIZE: i32 = 48;
        pub const ROW_HEIGHT: i32 = 24;
        pub const ROW_WIDTH: i32 = 220;
        // GuiCategoryEditor.GuiCategoryEditor(6): not translated, its class isn't translated (it's generic).
        // GuiCategoryEditor.constructEditorData(1): not translated, it has no body.
        // GuiCategoryEditor.constructDefaultData(1): not translated, it has no body.
        // GuiCategoryEditor.onConfigConfirmed(1): not translated, it has no body.
        // GuiCategoryEditor.init(0): not translated, its class isn't translated (it's generic).
        // GuiCategoryEditor.confirm(0): not translated, its class isn't translated (it's generic).
        // GuiCategoryEditor.onExit(1): not translated, its body couldn't be read: an anonymous class.
        // GuiCategoryEditor.cancel(1): not translated, its class isn't translated (it's generic).
        // GuiCategoryEditor.extractBackground(4): not translated, its class isn't translated (it's generic).
        // GuiCategoryEditor.extractRenderState(4): not translated, its class isn't translated (it's generic).
        // GuiCategoryEditor.keyPressed(1): not translated, its class isn't translated (it's generic).
        // GuiCategoryEditor.tick(0): not translated, its class isn't translated (it's generic).
        // GuiCategoryEditor.getRowList(0): not translated, its class isn't translated (it's generic).
    }

    /// `GuiCategoryEditor.SettingRowList` (xaero/hud/category/ui/GuiCategoryEditor.java)
    pub mod gui_category_editor_setting_row_list {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ObjectSelectionList`. Its static parts are below.
        // USAGE_NARRATION: a game value, so it's in game_side.rs.
        // LEFT_RIGHT_USAGE: a game value, so it's in game_side.rs.
        // GuiCategoryEditor.SettingRowList.SettingRowList(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.entriesCanBeSelected(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.hasCut(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.getCut(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.isCut(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.setCutCategory(2): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.pasteTo(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.isFocused(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.setLastExpandedData(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.restoreScrollAfterUpdate(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.updateEntries(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.addEntriesForExpanded(2): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.mouseClicked(2): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.mouseMoved(2): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.keyReleased(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.charTyped(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.tick(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.confirmSelection(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.setFocused(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.setSelected(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.getRowWidth(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.scrollBarX(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.narrateSelection(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.updateWidgetNarration(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.extractWidgetRenderState(4): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.nextFocusPath(1): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
        // GuiCategoryEditor.SettingRowList.getDataConverter(0): not translated, its class isn't translated (it extends Minecraft's `ObjectSelectionList`).
    }

    /// `GuiCategoryEditor.SettingRowList.Entry` (xaero/hud/category/ui/GuiCategoryEditor.java)
    pub mod gui_category_editor_setting_row_list_entry {
        use super::*;
        // The class itself isn't translated: it extends `Entry`. Its static parts are below.
        // GuiCategoryEditor.SettingRowList.Entry.Entry(2): not translated, its class isn't translated (it extends `Entry`).
        // GuiCategoryEditor.SettingRowList.Entry.extractContent(5): not translated, its class isn't translated (it extends `Entry`).
        // GuiCategoryEditor.SettingRowList.Entry.mouseClicked(2): not translated, its class isn't translated (it extends `Entry`).
        // GuiCategoryEditor.SettingRowList.Entry.mouseReleased(1): not translated, its class isn't translated (it extends `Entry`).
        // GuiCategoryEditor.SettingRowList.Entry.mouseScrolled(4): not translated, its class isn't translated (it extends `Entry`).
        // GuiCategoryEditor.SettingRowList.Entry.mouseDragged(3): not translated, its class isn't translated (it extends `Entry`).
        // GuiCategoryEditor.SettingRowList.Entry.keyPressed(1): not translated, its class isn't translated (it extends `Entry`).
        // GuiCategoryEditor.SettingRowList.Entry.keyReleased(1): not translated, its class isn't translated (it extends `Entry`).
        // GuiCategoryEditor.SettingRowList.Entry.charTyped(1): not translated, its class isn't translated (it extends `Entry`).
        // GuiCategoryEditor.SettingRowList.Entry.setFocused(1): not translated, its class isn't translated (it extends `Entry`).
        // GuiCategoryEditor.SettingRowList.Entry.tick(0): not translated, its class isn't translated (it extends `Entry`).
        // GuiCategoryEditor.SettingRowList.Entry.getNarration(0): not translated, its class isn't translated (it extends `Entry`).
    }

    /// `EditorAdderNode` (xaero/hud/category/ui/node/EditorAdderNode.java)
    pub mod editor_adder_node {
        use super::*;
        // The class itself isn't translated: it extends `EditorNode`. Its static parts are below.
        // EditorAdderNode.EditorAdderNode(6): not translated, its class isn't translated (it extends `EditorNode`).
        // EditorAdderNode.isConfirmed(0): not translated, its class isn't translated (it extends `EditorNode`).
        // EditorAdderNode.setExpanded(1): not translated, its class isn't translated (it extends `EditorNode`).
        // EditorAdderNode.reset(0): not translated, its class isn't translated (it extends `EditorNode`).
        // EditorAdderNode.getNameField(0): not translated, its class isn't translated (it extends `EditorNode`).
        // EditorAdderNode.getDisplayName(0): not translated, its class isn't translated (it extends `EditorNode`).
        // EditorAdderNode.getSubNodes(0): not translated, its class isn't translated (it extends `EditorNode`).
    }

    /// `EditorAdderNode.Builder` (xaero/hud/category/ui/node/EditorAdderNode.java)
    pub mod editor_adder_node_builder {
        use super::*;
        // The class itself isn't translated: it extends `Builder`. Its static parts are below.
        // EditorAdderNode.Builder.Builder(1): not translated, its class isn't translated (it extends `Builder`).
        // EditorAdderNode.Builder.setDefault(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorAdderNode.Builder.getCenteredEntryFactory(4): not translated, its class isn't translated (it extends `Builder`).
        // EditorAdderNode.Builder.setDisplayName(1): not translated, its class isn't translated (it extends `Builder`).
        // EditorAdderNode.Builder.getNameFieldBuilder(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorAdderNode.Builder.getConfirmButtonBuilder(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorAdderNode.Builder.begin(1): not translated, it uses Builder, which isn't translated (it extends `Builder`).
        // EditorAdderNode.Builder.build(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorAdderNode.Builder.buildInternally(0): not translated, its class isn't translated (it extends `Builder`).
    }

    /// `EditorCategoryNode` (xaero/hud/category/ui/node/EditorCategoryNode.java)
    pub mod editor_category_node {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorCategoryNode.EditorCategoryNode(8): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.getSettingsNode(0): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.getSubCategories(0): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.getName(0): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.getDisplayName(0): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.getAdderHandler(0): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.getDeletionHandler(0): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.getMoveAction(3): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.getDuplicateAction(2): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.getCutAction(2): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.getPasteAction(1): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.getSubNodes(0): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.removeProtectionRecursive(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorCategoryNode.Builder` (xaero/hud/category/ui/node/EditorCategoryNode.java)
    pub mod editor_category_node_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorCategoryNode.Builder.Builder(2): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.Builder.mainEntryFactory(7): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.Builder.setNewCategorySupplier(1): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.Builder.setSubIndex(1): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.Builder.setName(1): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.Builder.getSettingDataBuilder(0): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.Builder.addSubCategoryBuilder(1): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.Builder.buildSubCategories(0): not translated, its class isn't translated (it's generic).
        // EditorCategoryNode.Builder.build(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorFilterCategoryNode` (xaero/hud/category/ui/node/EditorFilterCategoryNode.java)
    pub mod editor_filter_category_node {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorFilterCategoryNode.EditorFilterCategoryNode(8): not translated, its class isn't translated (it's generic).
    }

    /// `EditorFilterCategoryNode.Builder` (xaero/hud/category/ui/node/EditorFilterCategoryNode.java)
    pub mod editor_filter_category_node_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorFilterCategoryNode.Builder.Builder(2): not translated, its class isn't translated (it's generic).
        // EditorFilterCategoryNode.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorFilterCategoryNode.Builder.setListRuleTypePrefixSeparator(1): not translated, its class isn't translated (it's generic).
        // EditorFilterCategoryNode.Builder.setInputRuleTypeStringValidator(1): not translated, its class isn't translated (it's generic).
        // EditorFilterCategoryNode.Builder.build(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorFilterSettingsNode` (xaero/hud/category/ui/node/EditorFilterSettingsNode.java)
    pub mod editor_filter_settings_node {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorFilterSettingsNode.EditorFilterSettingsNode(14): not translated, its class isn't translated (it's generic).
        // EditorFilterSettingsNode.getBaseRule(0): not translated, its class isn't translated (it's generic).
        // EditorFilterSettingsNode.getIncludeList(0): not translated, its class isn't translated (it's generic).
        // EditorFilterSettingsNode.getExcludeList(0): not translated, its class isn't translated (it's generic).
        // EditorFilterSettingsNode.getSubNodes(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorFilterSettingsNode.Builder` (xaero/hud/category/ui/node/EditorFilterSettingsNode.java)
    pub mod editor_filter_settings_node_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorFilterSettingsNode.Builder.Builder(5): not translated, its class isn't translated (it's generic).
        // EditorFilterSettingsNode.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorFilterSettingsNode.Builder.setBaseRule(1): not translated, its class isn't translated (it's generic).
        // EditorFilterSettingsNode.Builder.setListRuleTypePrefixSeparator(1): not translated, its class isn't translated (it's generic).
        // EditorFilterSettingsNode.Builder.setInputRuleTypeStringValidator(1): not translated, its class isn't translated (it's generic).
        // EditorFilterSettingsNode.Builder.getIncludeListBuilder(0): not translated, its class isn't translated (it's generic).
        // EditorFilterSettingsNode.Builder.getExcludeListBuilder(0): not translated, its class isn't translated (it's generic).
        // EditorFilterSettingsNode.Builder.buildIncludeList(0): not translated, its class isn't translated (it's generic).
        // EditorFilterSettingsNode.Builder.buildExcludeList(0): not translated, its class isn't translated (it's generic).
        // EditorFilterSettingsNode.Builder.build(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorFilterSettingsNode.FinalBuilder` (xaero/hud/category/ui/node/EditorFilterSettingsNode.java)
    pub mod editor_filter_settings_node_final_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorFilterSettingsNode.FinalBuilder.FinalBuilder(5): not translated, its class isn't translated (it's generic).
        // EditorFilterSettingsNode.FinalBuilder.buildInternally(2): not translated, its class isn't translated (it's generic).
        // EditorFilterSettingsNode.FinalBuilder.begin(5): not translated, it's generic.
    }

    /// `EditorNode` (xaero/hud/category/ui/node/EditorNode.java)
    pub mod editor_node {
        use super::*;
        // The class itself isn't translated: it uses EditorListRootEntryFactory, which isn't translated (it's an interface). Its static parts are below.
        // EditorNode.EditorNode(3): not translated, its class isn't translated (it uses EditorListRootEntryFactory, which isn't translated (it's an interface)).
        // EditorNode.isMovable(0): not translated, its class isn't translated (it uses EditorListRootEntryFactory, which isn't translated (it's an interface)).
        // EditorNode.isExpanded(0): not translated, its class isn't translated (it uses EditorListRootEntryFactory, which isn't translated (it's an interface)).
        // EditorNode.setExpanded(1): not translated, its class isn't translated (it uses EditorListRootEntryFactory, which isn't translated (it's an interface)).
        // EditorNode.getListEntryFactory(0): not translated, its class isn't translated (it uses EditorListRootEntryFactory, which isn't translated (it's an interface)).
        // EditorNode.getTooltipSupplier(1): not translated, its class isn't translated (it uses EditorListRootEntryFactory, which isn't translated (it's an interface)).
        // EditorNode.getExpandAction(1): not translated, its class isn't translated (it uses EditorListRootEntryFactory, which isn't translated (it's an interface)).
        // EditorNode.getDisplayName(0): not translated, it has no body.
        // EditorNode.getSubNodes(0): not translated, it has no body.
    }

    /// `EditorNode.Builder` (xaero/hud/category/ui/node/EditorNode.java)
    pub mod editor_node_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorNode.Builder.Builder(0): not translated, its class isn't translated (it's generic).
        // EditorNode.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorNode.Builder.mainEntryFactory(7): not translated, its class isn't translated (it's generic).
        // EditorNode.Builder.getCenteredEntryFactory(4): not translated, its class isn't translated (it's generic).
        // EditorNode.Builder.setMovable(1): not translated, its class isn't translated (it's generic).
        // EditorNode.Builder.setListEntryFactory(1): not translated, its class isn't translated (it's generic).
        // EditorNode.Builder.setTooltipSupplier(1): not translated, its class isn't translated (it's generic).
        // EditorNode.Builder.setTooltipInfoSupplier(1): not translated, its class isn't translated (it's generic).
        // EditorNode.Builder.build(0): not translated, its class isn't translated (it's generic).
        // EditorNode.Builder.buildInternally(0): not translated, it has no body.
    }

    /// `EditorSettingsNode` (xaero/hud/category/ui/node/EditorSettingsNode.java)
    pub mod editor_settings_node {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorSettingsNode.EditorSettingsNode(11): not translated, its class isn't translated (it's generic).
        // EditorSettingsNode.getSettings(0): not translated, its class isn't translated (it's generic).
        // EditorSettingsNode.getSettingData(1): not translated, its class isn't translated (it's generic).
        // EditorSettingsNode.isRootSettings(0): not translated, its class isn't translated (it's generic).
        // EditorSettingsNode.isToBeDeleted(0): not translated, its class isn't translated (it's generic).
        // EditorSettingsNode.setToBeDeleted(0): not translated, its class isn't translated (it's generic).
        // EditorSettingsNode.getProtection(0): not translated, its class isn't translated (it's generic).
        // EditorSettingsNode.setProtected(1): not translated, its class isn't translated (it's generic).
        // EditorSettingsNode.getSubNodes(0): not translated, its class isn't translated (it's generic).
        // EditorSettingsNode.getDisplayName(0): not translated, its class isn't translated (it's generic).
        // EditorSettingsNode.getNameOption(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorSettingsNode.Builder` (xaero/hud/category/ui/node/EditorSettingsNode.java)
    pub mod editor_settings_node_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorSettingsNode.Builder.Builder(4): not translated, its class isn't translated (it's generic).
        // EditorSettingsNode.Builder.addSetting(2): not translated, it's generic.
        // EditorSettingsNode.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorSettingsNode.Builder.getCenteredEntryFactory(4): not translated, its class isn't translated (it's generic).
        // EditorSettingsNode.Builder.setSettingValue(2): not translated, it's generic.
        // EditorSettingsNode.Builder.setRootSettings(1): not translated, its class isn't translated (it's generic).
        // EditorSettingsNode.Builder.setProtection(1): not translated, its class isn't translated (it's generic).
        // EditorSettingsNode.Builder.getNameOptionBuilder(0): not translated, its class isn't translated (it's generic).
        // EditorSettingsNode.Builder.getDeleteButtonBuilder(0): not translated, its class isn't translated (it's generic).
        // EditorSettingsNode.Builder.buildInternally(0): not translated, its class isn't translated (it's generic).
        // EditorSettingsNode.Builder.buildInternally(2): not translated, it has no body.
    }

    /// `EditorSettingsNode.FinalBuilder` (xaero/hud/category/ui/node/EditorSettingsNode.java)
    pub mod editor_settings_node_final_builder {
        use super::*;
        // The class itself isn't translated: it extends `Builder`. Its static parts are below.
        // EditorSettingsNode.FinalBuilder.FinalBuilder(4): not translated, its class isn't translated (it extends `Builder`).
        // EditorSettingsNode.FinalBuilder.buildInternally(2): not translated, its class isn't translated (it extends `Builder`).
    }

    /// `EditorSimpleDeletableWrapperNode` (xaero/hud/category/ui/node/EditorSimpleDeletableWrapperNode.java)
    pub mod editor_simple_deletable_wrapper_node {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorSimpleDeletableWrapperNode.EditorSimpleDeletableWrapperNode(5): not translated, its class isn't translated (it's generic).
        // EditorSimpleDeletableWrapperNode.getDeletionCallback(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorSimpleDeletableWrapperNode.Builder` (xaero/hud/category/ui/node/EditorSimpleDeletableWrapperNode.java)
    pub mod editor_simple_deletable_wrapper_node_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorSimpleDeletableWrapperNode.Builder.Builder(0): not translated, its class isn't translated (it's generic).
        // EditorSimpleDeletableWrapperNode.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorSimpleDeletableWrapperNode.Builder.mainEntryFactory(7): not translated, its class isn't translated (it's generic).
        // EditorSimpleDeletableWrapperNode.Builder.setDeletionCallback(1): not translated, its class isn't translated (it's generic).
        // EditorSimpleDeletableWrapperNode.Builder.build(0): not translated, its class isn't translated (it's generic).
        // EditorSimpleDeletableWrapperNode.Builder.buildInternally(0): not translated, its class isn't translated (it's generic).
        // EditorSimpleDeletableWrapperNode.Builder.begin(0): not translated, it's generic.
    }

    /// `EditorSimpleDeletableWrapperNode.DeletionCallback` (xaero/hud/category/ui/node/EditorSimpleDeletableWrapperNode.java)
    pub mod editor_simple_deletable_wrapper_node_deletion_callback {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // EditorSimpleDeletableWrapperNode.DeletionCallback.delete(3): not translated, it has no body.
    }

    /// `EditorSimpleWrapperNode` (xaero/hud/category/ui/node/EditorSimpleWrapperNode.java)
    pub mod editor_simple_wrapper_node {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorSimpleWrapperNode.EditorSimpleWrapperNode(4): not translated, its class isn't translated (it's generic).
        // EditorSimpleWrapperNode.getElement(0): not translated, its class isn't translated (it's generic).
        // EditorSimpleWrapperNode.setElement(1): not translated, its class isn't translated (it's generic).
        // EditorSimpleWrapperNode.getSubNodes(0): not translated, its class isn't translated (it's generic).
        // EditorSimpleWrapperNode.getDisplayName(0): not translated, its class isn't translated (it's generic).
        // EditorSimpleWrapperNode.equals(1): not translated, its class isn't translated (it's generic).
        // EditorSimpleWrapperNode.compareTo(1): not translated, its class isn't translated (it's generic).
    }

    /// `EditorSimpleWrapperNode.Builder` (xaero/hud/category/ui/node/EditorSimpleWrapperNode.java)
    pub mod editor_simple_wrapper_node_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorSimpleWrapperNode.Builder.Builder(0): not translated, its class isn't translated (it's generic).
        // EditorSimpleWrapperNode.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorSimpleWrapperNode.Builder.setElement(1): not translated, its class isn't translated (it's generic).
        // EditorSimpleWrapperNode.Builder.build(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorSimpleWrapperNode.FinalBuilder` (xaero/hud/category/ui/node/EditorSimpleWrapperNode.java)
    pub mod editor_simple_wrapper_node_final_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorSimpleWrapperNode.FinalBuilder.begin(0): not translated, it's generic.
        // EditorSimpleWrapperNode.FinalBuilder.buildInternally(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorCompactBooleanOptionsNode` (xaero/hud/category/ui/node/options/EditorCompactBooleanOptionsNode.java)
    pub mod editor_compact_boolean_options_node {
        use super::*;
        // The class itself isn't translated: it extends `EditorCompactOptionsNode`. Its static parts are below.
        // EditorCompactBooleanOptionsNode.EditorCompactBooleanOptionsNode(9): not translated, its class isn't translated (it extends `EditorCompactOptionsNode`).
        // EditorCompactBooleanOptionsNode.getIndexReader(0): not translated, its class isn't translated (it extends `EditorCompactOptionsNode`).
    }

    /// `EditorCompactBooleanOptionsNode.Builder` (xaero/hud/category/ui/node/options/EditorCompactBooleanOptionsNode.java)
    pub mod editor_compact_boolean_options_node_builder {
        use super::*;
        // The class itself isn't translated: it extends `Builder`. Its static parts are below.
        // EditorCompactBooleanOptionsNode.Builder.Builder(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorCompactBooleanOptionsNode.Builder.setDefault(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorCompactBooleanOptionsNode.Builder.getTrueOptionBuilder(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorCompactBooleanOptionsNode.Builder.getFalseOptionBuilder(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorCompactBooleanOptionsNode.Builder.build(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorCompactBooleanOptionsNode.Builder.buildInternally(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorCompactBooleanOptionsNode.Builder.begin(0): not translated, it uses Builder, which isn't translated (it extends `Builder`).
    }

    /// `EditorCompactOptionsNode` (xaero/hud/category/ui/node/options/EditorCompactOptionsNode.java)
    pub mod editor_compact_options_node {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorCompactOptionsNode.EditorCompactOptionsNode(7): not translated, its class isn't translated (it's generic).
        // EditorCompactOptionsNode.getCurrentIndex(0): not translated, its class isn't translated (it's generic).
        // EditorCompactOptionsNode.getOptionCount(0): not translated, its class isn't translated (it's generic).
        // EditorCompactOptionsNode.getUpdatedIndexConsumer(0): not translated, its class isn't translated (it's generic).
        // EditorCompactOptionsNode.getIndexReader(0): not translated, it has no body.
        // EditorCompactOptionsNode.getSubNodes(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorCompactOptionsNode.Builder` (xaero/hud/category/ui/node/options/EditorCompactOptionsNode.java)
    pub mod editor_compact_options_node_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorCompactOptionsNode.Builder.Builder(0): not translated, its class isn't translated (it's generic).
        // EditorCompactOptionsNode.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorCompactOptionsNode.Builder.setSlider(1): not translated, its class isn't translated (it's generic).
        // EditorCompactOptionsNode.Builder.build(0): not translated, its class isn't translated (it's generic).
        // EditorCompactOptionsNode.Builder.getMainEntryFactory(1): not translated, its class isn't translated (it's generic).
        // EditorCompactOptionsNode.Builder.getCenteredEntryFactory(5): not translated, its class isn't translated (it's generic).
        // EditorCompactOptionsNode.Builder.getEntryWidget(5): not translated, its class isn't translated (it's generic).
        // EditorCompactOptionsNode.Builder.buildInternally(0): not translated, it has no body.
    }

    /// `EditorExpandingOptionsNode` (xaero/hud/category/ui/node/options/EditorExpandingOptionsNode.java)
    pub mod editor_expanding_options_node {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorExpandingOptionsNode.EditorExpandingOptionsNode(7): not translated, its class isn't translated (it's generic).
        // EditorExpandingOptionsNode.onSelected(1): not translated, its class isn't translated (it's generic).
        // EditorExpandingOptionsNode.getSubNodes(0): not translated, its class isn't translated (it's generic).
        // EditorExpandingOptionsNode.getDisplayName(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorExpandingOptionsNode.Builder` (xaero/hud/category/ui/node/options/EditorExpandingOptionsNode.java)
    pub mod editor_expanding_options_node_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorExpandingOptionsNode.Builder.Builder(1): not translated, its class isn't translated (it's generic).
        // EditorExpandingOptionsNode.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorExpandingOptionsNode.Builder.getCenteredEntryFactory(4): not translated, its class isn't translated (it's generic).
        // EditorExpandingOptionsNode.Builder.addOptionBuilderFor(1): not translated, its class isn't translated (it's generic).
        // EditorExpandingOptionsNode.Builder.addOptionBuilder(1): not translated, its class isn't translated (it's generic).
        // EditorExpandingOptionsNode.Builder.build(0): not translated, its class isn't translated (it's generic).
        // EditorExpandingOptionsNode.Builder.buildInternally(0): not translated, its class isn't translated (it's generic).
        // EditorExpandingOptionsNode.Builder.buildInternally(2): not translated, it has no body.
    }

    /// `EditorExpandingOptionsNode.FinalBuilder` (xaero/hud/category/ui/node/options/EditorExpandingOptionsNode.java)
    pub mod editor_expanding_options_node_final_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorExpandingOptionsNode.FinalBuilder.FinalBuilder(1): not translated, its class isn't translated (it's generic).
        // EditorExpandingOptionsNode.FinalBuilder.begin(1): not translated, it's generic.
        // EditorExpandingOptionsNode.FinalBuilder.buildInternally(2): not translated, its class isn't translated (it's generic).
    }

    /// `EditorOptionNode` (xaero/hud/category/ui/node/options/EditorOptionNode.java)
    pub mod editor_option_node {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorOptionNode.EditorOptionNode(5): not translated, its class isn't translated (it's generic).
        // EditorOptionNode.getValue(0): not translated, its class isn't translated (it's generic).
        // EditorOptionNode.getDisplayName(0): not translated, its class isn't translated (it's generic).
        // EditorOptionNode.getSubNodes(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorOptionNode.Builder` (xaero/hud/category/ui/node/options/EditorOptionNode.java)
    pub mod editor_option_node_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorOptionNode.Builder.Builder(0): not translated, its class isn't translated (it's generic).
        // EditorOptionNode.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorOptionNode.Builder.getCenteredEntryFactory(4): not translated, its class isn't translated (it's generic).
        // EditorOptionNode.Builder.setValue(1): not translated, its class isn't translated (it's generic).
        // EditorOptionNode.Builder.setDisplayName(1): not translated, its class isn't translated (it's generic).
        // EditorOptionNode.Builder.build(0): not translated, its class isn't translated (it's generic).
        // EditorOptionNode.Builder.begin(0): not translated, it's generic.
        // EditorOptionNode.Builder.buildInternally(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorOptionsNode` (xaero/hud/category/ui/node/options/EditorOptionsNode.java)
    pub mod editor_options_node {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorOptionsNode.EditorOptionsNode(5): not translated, its class isn't translated (it's generic).
        // EditorOptionsNode.getCurrentValue(0): not translated, its class isn't translated (it's generic).
        // EditorOptionsNode.setCurrentValue(1): not translated, its class isn't translated (it's generic).
        // EditorOptionsNode.getMessageSupplier(0): not translated, its class isn't translated (it's generic).
        // EditorOptionsNode.getIsActiveSupplier(0): not translated, its class isn't translated (it's generic).
        // EditorOptionsNode.getDisplayName(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorOptionsNode.Builder` (xaero/hud/category/ui/node/options/EditorOptionsNode.java)
    pub mod editor_options_node_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorOptionsNode.Builder.Builder(0): not translated, its class isn't translated (it's generic).
        // EditorOptionsNode.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorOptionsNode.Builder.setCurrentValue(1): not translated, its class isn't translated (it's generic).
        // EditorOptionsNode.Builder.setDisplayName(1): not translated, its class isn't translated (it's generic).
        // EditorOptionsNode.Builder.setIsActiveSupplier(1): not translated, its class isn't translated (it's generic).
        // EditorOptionsNode.Builder.build(0): not translated, its class isn't translated (it's generic).
        // EditorOptionsNode.Builder.buildInternally(0): not translated, it has no body.
    }

    /// `EditorOptionsNode.IOptionsNodeIsActiveSupplier` (xaero/hud/category/ui/node/options/EditorOptionsNode.java)
    pub mod editor_options_node_i_options_node_is_active_supplier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // EditorOptionsNode.IOptionsNodeIsActiveSupplier.get(2): not translated, it has no body.
    }

    /// `EditorSimpleButtonNode` (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java)
    pub mod editor_simple_button_node {
        use super::*;
        // The class itself isn't translated: it extends `EditorNode` (xaero.hud.category.ui.node). Its static parts are below.
        // EditorSimpleButtonNode.EditorSimpleButtonNode(7): not translated, its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)).
        // EditorSimpleButtonNode.getMessageSupplier(2): not translated, its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)).
        // EditorSimpleButtonNode.getIsActiveSupplier(3): not translated, its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)).
        // EditorSimpleButtonNode.getDisplayName(0): not translated, its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)).
        // EditorSimpleButtonNode.getPressAction(0): not translated, its body couldn't be read: an anonymous class.
        // EditorSimpleButtonNode.getSubNodes(0): not translated, its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)).
    }

    /// `EditorSimpleButtonNode.Builder` (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java)
    pub mod editor_simple_button_node_builder {
        use super::*;
        // The class itself isn't translated: it extends `Builder`. Its static parts are below.
        // EditorSimpleButtonNode.Builder.Builder(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorSimpleButtonNode.Builder.setDefault(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorSimpleButtonNode.Builder.getCenteredEntryFactory(4): not translated, its class isn't translated (it extends `Builder`).
        // EditorSimpleButtonNode.Builder.setDisplayName(1): not translated, its class isn't translated (it extends `Builder`).
        // EditorSimpleButtonNode.Builder.setCallback(1): not translated, its class isn't translated (it extends `Builder`).
        // EditorSimpleButtonNode.Builder.setMessageSupplier(1): not translated, its class isn't translated (it extends `Builder`).
        // EditorSimpleButtonNode.Builder.setIsActiveSupplier(1): not translated, its class isn't translated (it extends `Builder`).
        // EditorSimpleButtonNode.Builder.build(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorSimpleButtonNode.Builder.buildInternally(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorSimpleButtonNode.Builder.begin(0): not translated, it uses Builder, which isn't translated (it extends `Builder`).
    }

    /// `EditorSimpleButtonNode.ISimpleButtonCallback` (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java)
    pub mod editor_simple_button_node_i_simple_button_callback {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // EditorSimpleButtonNode.ISimpleButtonCallback.onButtonPress(3): not translated, it has no body.
    }

    /// `EditorSimpleButtonNode.ISimpleButtonIsActiveSupplier` (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java)
    pub mod editor_simple_button_node_i_simple_button_is_active_supplier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // EditorSimpleButtonNode.ISimpleButtonIsActiveSupplier.get(3): not translated, it has no body.
    }

    /// `EditorSimpleButtonNode.ISimpleButtonMessageSupplier` (xaero/hud/category/ui/node/options/EditorSimpleButtonNode.java)
    pub mod editor_simple_button_node_i_simple_button_message_supplier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // EditorSimpleButtonNode.ISimpleButtonMessageSupplier.get(2): not translated, it has no body.
    }

    /// `EditorCompactListOptionsNode` (xaero/hud/category/ui/node/options/list/EditorCompactListOptionsNode.java)
    pub mod editor_compact_list_options_node {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorCompactListOptionsNode.EditorCompactListOptionsNode(7): not translated, its class isn't translated (it's generic).
        // EditorCompactListOptionsNode.getIndexReader(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorCompactListOptionsNode.Builder` (xaero/hud/category/ui/node/options/list/EditorCompactListOptionsNode.java)
    pub mod editor_compact_list_options_node_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorCompactListOptionsNode.Builder.Builder(1): not translated, its class isn't translated (it's generic).
        // EditorCompactListOptionsNode.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorCompactListOptionsNode.Builder.addOptionBuilder(1): not translated, its class isn't translated (it's generic).
        // EditorCompactListOptionsNode.Builder.build(0): not translated, its class isn't translated (it's generic).
        // EditorCompactListOptionsNode.Builder.buildInternally(0): not translated, its class isn't translated (it's generic).
        // EditorCompactListOptionsNode.Builder.begin(1): not translated, it's generic.
    }

    /// `EditorCompactRangeNode` (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java)
    pub mod editor_compact_range_node {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorCompactRangeNode.EditorCompactRangeNode(12): not translated, its class isn't translated (it's generic).
        // EditorCompactRangeNode.setCurrentValue(1): not translated, its class isn't translated (it's generic).
        // EditorCompactRangeNode.getCurrentRangeValue(0): not translated, its class isn't translated (it's generic).
        // EditorCompactRangeNode.getIndexReader(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorCompactRangeNode.Builder` (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java)
    pub mod editor_compact_range_node_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorCompactRangeNode.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorCompactRangeNode.Builder.setCurrentRangeValue(1): not translated, its class isn't translated (it's generic).
        // EditorCompactRangeNode.Builder.setMinNumber(1): not translated, its class isn't translated (it's generic).
        // EditorCompactRangeNode.Builder.setMaxNumber(1): not translated, its class isn't translated (it's generic).
        // EditorCompactRangeNode.Builder.setNumberReader(1): not translated, its class isn't translated (it's generic).
        // EditorCompactRangeNode.Builder.setNumberWriter(1): not translated, its class isn't translated (it's generic).
        // EditorCompactRangeNode.Builder.setValueNamer(1): not translated, its class isn't translated (it's generic).
        // EditorCompactRangeNode.Builder.setHasNullOption(1): not translated, its class isn't translated (it's generic).
        // EditorCompactRangeNode.Builder.build(0): not translated, its class isn't translated (it's generic).
        // EditorCompactRangeNode.Builder.buildInternally(0): not translated, its class isn't translated (it's generic).
        // EditorCompactRangeNode.Builder.buildInternally(3): not translated, it has no body.
    }

    /// `EditorCompactRangeNode.FinalBuilder` (xaero/hud/category/ui/node/options/range/EditorCompactRangeNode.java)
    pub mod editor_compact_range_node_final_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorCompactRangeNode.FinalBuilder.FinalBuilder(0): not translated, its class isn't translated (it's generic).
        // EditorCompactRangeNode.FinalBuilder.buildInternally(3): not translated, its class isn't translated (it's generic).
        // EditorCompactRangeNode.FinalBuilder.begin(0): not translated, it's generic.
    }

    /// `EditorExpandingRangeNode` (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java)
    pub mod editor_expanding_range_node {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorExpandingRangeNode.EditorExpandingRangeNode(9): not translated, its class isn't translated (it's generic).
        // EditorExpandingRangeNode.onSelected(1): not translated, its class isn't translated (it's generic).
        // EditorExpandingRangeNode.getCurrentRangeValue(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorExpandingRangeNode.Builder` (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java)
    pub mod editor_expanding_range_node_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorExpandingRangeNode.Builder.Builder(1): not translated, its class isn't translated (it's generic).
        // EditorExpandingRangeNode.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorExpandingRangeNode.Builder.setCurrentRangeValue(1): not translated, its class isn't translated (it's generic).
        // EditorExpandingRangeNode.Builder.setMinNumber(1): not translated, its class isn't translated (it's generic).
        // EditorExpandingRangeNode.Builder.setMaxNumber(1): not translated, its class isn't translated (it's generic).
        // EditorExpandingRangeNode.Builder.setNumberReader(1): not translated, its class isn't translated (it's generic).
        // EditorExpandingRangeNode.Builder.setNumberWriter(1): not translated, its class isn't translated (it's generic).
        // EditorExpandingRangeNode.Builder.setValueNamer(1): not translated, its class isn't translated (it's generic).
        // EditorExpandingRangeNode.Builder.setHasNullOption(1): not translated, its class isn't translated (it's generic).
        // EditorExpandingRangeNode.Builder.build(0): not translated, its class isn't translated (it's generic).
        // EditorExpandingRangeNode.Builder.buildInternally(2): not translated, it has no body.
    }

    /// `EditorExpandingRangeNode.FinalBuilder` (xaero/hud/category/ui/node/options/range/EditorExpandingRangeNode.java)
    pub mod editor_expanding_range_node_final_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorExpandingRangeNode.FinalBuilder.FinalBuilder(1): not translated, its class isn't translated (it's generic).
        // EditorExpandingRangeNode.FinalBuilder.buildInternally(2): not translated, its class isn't translated (it's generic).
        // EditorExpandingRangeNode.FinalBuilder.begin(1): not translated, it's generic.
    }

    /// `EditorCompactSettingNode` (xaero/hud/category/ui/node/options/range/setting/EditorCompactSettingNode.java)
    pub mod editor_compact_setting_node {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorCompactSettingNode.EditorCompactSettingNode(14): not translated, its class isn't translated (it's generic).
        // EditorCompactSettingNode.getSetting(0): not translated, its class isn't translated (it's generic).
        // EditorCompactSettingNode.getSettingValue(0): not translated, its class isn't translated (it's generic).
        // EditorCompactSettingNode.isRootSettings(0): not translated, its class isn't translated (it's generic).
        // EditorCompactSettingNode.getValueName(2): not translated, it's generic.
    }

    /// `EditorCompactSettingNode.Builder` (xaero/hud/category/ui/node/options/range/setting/EditorCompactSettingNode.java)
    pub mod editor_compact_setting_node_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorCompactSettingNode.Builder.Builder(0): not translated, its class isn't translated (it's generic).
        // EditorCompactSettingNode.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorCompactSettingNode.Builder.setSetting(1): not translated, its class isn't translated (it's generic).
        // EditorCompactSettingNode.Builder.setSettingValue(1): not translated, its class isn't translated (it's generic).
        // EditorCompactSettingNode.Builder.setRootSettings(1): not translated, its class isn't translated (it's generic).
        // EditorCompactSettingNode.Builder.setSlider(1): not translated, its class isn't translated (it's generic).
        // EditorCompactSettingNode.Builder.build(0): not translated, its class isn't translated (it's generic).
        // EditorCompactSettingNode.Builder.begin(0): not translated, it's generic.
        // EditorCompactSettingNode.Builder.buildInternally(3): not translated, its class isn't translated (it's generic).
    }

    /// `EditorExpandingSettingNode` (xaero/hud/category/ui/node/options/range/setting/EditorExpandingSettingNode.java)
    pub mod editor_expanding_setting_node {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorExpandingSettingNode.EditorExpandingSettingNode(11): not translated, its class isn't translated (it's generic).
        // EditorExpandingSettingNode.getSetting(0): not translated, its class isn't translated (it's generic).
        // EditorExpandingSettingNode.getSettingValue(0): not translated, its class isn't translated (it's generic).
        // EditorExpandingSettingNode.isRootSettings(0): not translated, its class isn't translated (it's generic).
    }

    /// `EditorExpandingSettingNode.Builder` (xaero/hud/category/ui/node/options/range/setting/EditorExpandingSettingNode.java)
    pub mod editor_expanding_setting_node_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorExpandingSettingNode.Builder.Builder(1): not translated, its class isn't translated (it's generic).
        // EditorExpandingSettingNode.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorExpandingSettingNode.Builder.setSetting(1): not translated, its class isn't translated (it's generic).
        // EditorExpandingSettingNode.Builder.setSettingValue(1): not translated, its class isn't translated (it's generic).
        // EditorExpandingSettingNode.Builder.setRootSettings(1): not translated, its class isn't translated (it's generic).
        // EditorExpandingSettingNode.Builder.build(0): not translated, its class isn't translated (it's generic).
        // EditorExpandingSettingNode.Builder.buildInternally(2): not translated, its class isn't translated (it's generic).
        // EditorExpandingSettingNode.Builder.begin(1): not translated, it's generic.
    }

    /// `IEditorSettingNode` (xaero/hud/category/ui/node/options/range/setting/IEditorSettingNode.java)
    pub mod i_editor_setting_node {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IEditorSettingNode.getSetting(0): not translated, it has no body.
        // IEditorSettingNode.getSettingValue(0): not translated, it has no body.
        // IEditorSettingNode.isRootSettings(0): not translated, it has no body.
    }

    /// `IEditorSettingNodeBuilder` (xaero/hud/category/ui/node/options/range/setting/IEditorSettingNodeBuilder.java)
    pub mod i_editor_setting_node_builder {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IEditorSettingNodeBuilder.setSetting(1): not translated, it has no body.
        // IEditorSettingNodeBuilder.setSettingValue(1): not translated, it has no body.
        // IEditorSettingNodeBuilder.setRootSettings(1): not translated, it has no body.
        // IEditorSettingNodeBuilder.build(0): not translated, it has no body.
    }

    /// `EditorTextFieldOptionsNode` (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java)
    pub mod editor_text_field_options_node {
        use super::*;
        // The class itself isn't translated: it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options). Its static parts are below.
        // EditorTextFieldOptionsNode.EditorTextFieldOptionsNode(13): not translated, its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)).
        // EditorTextFieldOptionsNode.resetInput(1): not translated, its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)).
        // EditorTextFieldOptionsNode.setCurrentValue(1): not translated, its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)).
        // EditorTextFieldOptionsNode.getCurrentValue(0): not translated, its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)).
        // EditorTextFieldOptionsNode.getInput(0): not translated, its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)).
        // EditorTextFieldOptionsNode.getResult(0): not translated, its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)).
        // EditorTextFieldOptionsNode.getCursorPos(0): not translated, its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)).
        // EditorTextFieldOptionsNode.getHighlightPos(0): not translated, its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)).
        // EditorTextFieldOptionsNode.getMaxLength(0): not translated, its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)).
        // EditorTextFieldOptionsNode.getDisplayName(0): not translated, its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)).
        // EditorTextFieldOptionsNode.setExpanded(1): not translated, its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)).
        // EditorTextFieldOptionsNode.onSelected(1): not translated, its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)).
        // EditorTextFieldOptionsNode.getUpdatedValueConsumer(0): not translated, its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)).
        // EditorTextFieldOptionsNode.getSubNodes(0): not translated, its class isn't translated (it extends `EditorExpandingOptionsNode` (xaero.hud.category.ui.node.options)).
    }

    /// `EditorTextFieldOptionsNode.Builder` (xaero/hud/category/ui/node/options/text/EditorTextFieldOptionsNode.java)
    pub mod editor_text_field_options_node_builder {
        use super::*;
        // The class itself isn't translated: it extends `Builder`. Its static parts are below.
        // EditorTextFieldOptionsNode.Builder.Builder(1): not translated, its class isn't translated (it extends `Builder`).
        // EditorTextFieldOptionsNode.Builder.setDefault(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorTextFieldOptionsNode.Builder.getCenteredEntryFactory(4): not translated, its class isn't translated (it extends `Builder`).
        // EditorTextFieldOptionsNode.Builder.setInput(1): not translated, its class isn't translated (it extends `Builder`).
        // EditorTextFieldOptionsNode.Builder.setInputStringValidator(1): not translated, its class isn't translated (it extends `Builder`).
        // EditorTextFieldOptionsNode.Builder.needsInputStringValidator(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorTextFieldOptionsNode.Builder.setAllowCustomInput(1): not translated, its class isn't translated (it extends `Builder`).
        // EditorTextFieldOptionsNode.Builder.setAutoConfirm(1): not translated, its class isn't translated (it extends `Builder`).
        // EditorTextFieldOptionsNode.Builder.setMaxLength(1): not translated, its class isn't translated (it extends `Builder`).
        // EditorTextFieldOptionsNode.Builder.setCurrentValue(1): not translated, its class isn't translated (it extends `Builder`).
        // EditorTextFieldOptionsNode.Builder.build(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorTextFieldOptionsNode.Builder.buildInternally(2): not translated, its class isn't translated (it extends `Builder`).
        // EditorTextFieldOptionsNode.Builder.begin(1): not translated, it uses Builder, which isn't translated (it extends `Builder`).
    }

    /// `TextFieldSuggestionsResolver` (xaero/hud/category/ui/node/options/text/TextFieldSuggestionsResolver.java)
    pub mod text_field_suggestions_resolver {
        use super::*;
        // The class itself isn't translated: it uses ListFactory, which isn't translated (it's an interface). Its static parts are below.
        // TextFieldSuggestionsResolver.TextFieldSuggestionsResolver(1): not translated, its class isn't translated (it uses ListFactory, which isn't translated (it's an interface)).
        // TextFieldSuggestionsResolver.getSuggestions(2): not translated, its class isn't translated (it uses ListFactory, which isn't translated (it's an interface)).
    }

    /// `TextFieldSuggestionsResolver.Builder` (xaero/hud/category/ui/node/options/text/TextFieldSuggestionsResolver.java)
    pub mod text_field_suggestions_resolver_builder {
        use super::*;
        // The class itself isn't translated: it uses ListFactory, which isn't translated (it's an interface). Its static parts are below.
        // TextFieldSuggestionsResolver.Builder.Builder(1): not translated, its class isn't translated (it uses ListFactory, which isn't translated (it's an interface)).
        // TextFieldSuggestionsResolver.Builder.setDefault(0): not translated, its class isn't translated (it uses ListFactory, which isn't translated (it's an interface)).
        // TextFieldSuggestionsResolver.Builder.build(0): not translated, its class isn't translated (it uses ListFactory, which isn't translated (it's an interface)).
        // TextFieldSuggestionsResolver.Builder.begin(1): not translated, it uses Builder, which isn't translated (it uses ListFactory, which isn't translated (it's an interface)).
    }

    /// `EditorExcludeListNode` (xaero/hud/category/ui/node/rule/EditorExcludeListNode.java)
    pub mod editor_exclude_list_node {
        use super::*;
        // The class itself isn't translated: it extends `EditorListNode`. Its static parts are below.
        // EditorExcludeListNode.EditorExcludeListNode(10): not translated, its class isn't translated (it extends `EditorListNode`).
        // EditorExcludeListNode.getExcludeMode(0): not translated, its class isn't translated (it extends `EditorListNode`).
        // EditorExcludeListNode.getDisplayName(0): not translated, its class isn't translated (it extends `EditorListNode`).
        // EditorExcludeListNode.getSubNodes(0): not translated, its class isn't translated (it extends `EditorListNode`).
    }

    /// `EditorExcludeListNode.Builder` (xaero/hud/category/ui/node/rule/EditorExcludeListNode.java)
    pub mod editor_exclude_list_node_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorExcludeListNode.Builder.Builder(1): not translated, its class isn't translated (it's generic).
        // EditorExcludeListNode.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorExcludeListNode.Builder.setExcludeMode(1): not translated, its class isn't translated (it's generic).
        // EditorExcludeListNode.Builder.build(0): not translated, its class isn't translated (it's generic).
        // EditorExcludeListNode.Builder.buildInternally(0): not translated, its class isn't translated (it's generic).
        // EditorExcludeListNode.Builder.begin(1): not translated, it's generic.
    }

    /// `EditorIncludeListNode` (xaero/hud/category/ui/node/rule/EditorIncludeListNode.java)
    pub mod editor_include_list_node {
        use super::*;
        // The class itself isn't translated: it extends `EditorListNode`. Its static parts are below.
        // EditorIncludeListNode.EditorIncludeListNode(10): not translated, its class isn't translated (it extends `EditorListNode`).
        // EditorIncludeListNode.getDisplayName(0): not translated, its class isn't translated (it extends `EditorListNode`).
        // EditorIncludeListNode.getSubNodes(0): not translated, its class isn't translated (it extends `EditorListNode`).
        // EditorIncludeListNode.getIncludeInSuper(0): not translated, its class isn't translated (it extends `EditorListNode`).
    }

    /// `EditorIncludeListNode.Builder` (xaero/hud/category/ui/node/rule/EditorIncludeListNode.java)
    pub mod editor_include_list_node_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorIncludeListNode.Builder.Builder(1): not translated, its class isn't translated (it's generic).
        // EditorIncludeListNode.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorIncludeListNode.Builder.getIncludeInSuperToggleDataBuilder(0): not translated, its class isn't translated (it's generic).
        // EditorIncludeListNode.Builder.build(0): not translated, its class isn't translated (it's generic).
        // EditorIncludeListNode.Builder.buildInternally(0): not translated, its class isn't translated (it's generic).
        // EditorIncludeListNode.Builder.begin(1): not translated, it's generic.
    }

    /// `EditorListNode` (xaero/hud/category/ui/node/rule/EditorListNode.java)
    pub mod editor_list_node {
        use super::*;
        // The class itself isn't translated: it extends `EditorNode` (xaero.hud.category.ui.node). Its static parts are below.
        // EditorListNode.EditorListNode(9): not translated, its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)).
        // EditorListNode.getList(0): not translated, its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)).
        // EditorListNode.getDeletionCallback(0): not translated, its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)).
        // EditorListNode.getAdderHandler(0): not translated, its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)).
        // EditorListNode.getSubNodes(0): not translated, its class isn't translated (it extends `EditorNode` (xaero.hud.category.ui.node)).
    }

    /// `EditorListNode.Builder` (xaero/hud/category/ui/node/rule/EditorListNode.java)
    pub mod editor_list_node_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorListNode.Builder.Builder(1): not translated, its class isn't translated (it's generic).
        // EditorListNode.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // EditorListNode.Builder.mainEntryFactory(7): not translated, its class isn't translated (it's generic).
        // EditorListNode.Builder.getCenteredEntryFactory(4): not translated, its class isn't translated (it's generic).
        // EditorListNode.Builder.addListElement(1): not translated, its class isn't translated (it's generic).
        // EditorListNode.Builder.setDeletionCallback(1): not translated, its class isn't translated (it's generic).
        // EditorListNode.Builder.getAdderBuilder(0): not translated, its class isn't translated (it's generic).
        // EditorListNode.Builder.buildList(0): not translated, its class isn't translated (it's generic).
        // EditorListNode.Builder.setInputRuleTypeStringValidator(1): not translated, its class isn't translated (it's generic).
        // EditorListNode.Builder.setHelpTooltipSupplier(1): not translated, its class isn't translated (it's generic).
        // EditorListNode.Builder.setHelpTooltipInfoSupplier(1): not translated, its class isn't translated (it's generic).
        // EditorListNode.Builder.setDefaultListRuleType(1): not translated, its class isn't translated (it's generic).
        // EditorListNode.Builder.setListRuleTypes(1): not translated, its class isn't translated (it's generic).
        // EditorListNode.Builder.setListRuleTypePrefixSeparator(1): not translated, its class isn't translated (it's generic).
        // EditorListNode.Builder.build(0): not translated, its class isn't translated (it's generic).
        // EditorListNode.Builder.addSuggestionsForListRuleType(2): not translated, it's generic.
        // EditorListNode.Builder.buildInternally(0): not translated, it has no body.
    }

    /// `IEditorDataTooltipSupplier` (xaero/hud/category/ui/node/tooltip/IEditorDataTooltipSupplier.java)
    pub mod i_editor_data_tooltip_supplier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `RootCategorySettingEntry` (xaero/hud/category/ui/RootCategorySettingEntry.java)
    pub mod root_category_setting_entry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // RootCategorySettingEntry.RootCategorySettingEntry(3): not translated, its class isn't translated (it's generic).
        // RootCategorySettingEntry.RootCategorySettingEntry(4): not translated, its class isn't translated (it's generic).
        // RootCategorySettingEntry.RootCategorySettingEntry(5): not translated, its class isn't translated (it's generic).
        // RootCategorySettingEntry.getStringForSearch(0): not translated, its class isn't translated (it's generic).
        // RootCategorySettingEntry.createWidget(3): not translated, its class isn't translated (it's generic).
    }

    /// `EditorSettingType` (xaero/hud/category/ui/setting/EditorSettingType.java)
    pub mod editor_setting_type {
        use super::*;
        pub static ITERATION_BUTTON: LazyLock<editor_setting_type::EditorSettingType> = LazyLock::new(|| editor_setting_type::EditorSettingType::new(true, "iteration button"));
        pub static SLIDER: LazyLock<editor_setting_type::EditorSettingType> = LazyLock::new(|| editor_setting_type::EditorSettingType::new(true, "slider"));
        pub static EXPANDING: LazyLock<editor_setting_type::EditorSettingType> = LazyLock::new(|| editor_setting_type::EditorSettingType::new(true, "expanding"));
        #[derive(Clone, Debug)]
        pub struct EditorSettingType {
            pub using_indices: bool,
            pub id: String,
        }
        impl EditorSettingType {
            pub fn new(using_indices: bool, id: &str) -> Self {
                let mut s = Self { using_indices: using_indices, id: String::new() };
                s.using_indices = using_indices;
                s.id = id.to_string();
                s
            }
            pub fn is_using_indices(&self) -> bool {
                return self.using_indices;
            }
        }
    }

    /// `SettingNodeBuilderFactory` (xaero/hud/category/ui/setting/SettingNodeBuilderFactory.java)
    pub mod setting_node_builder_factory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SettingNodeBuilderFactory.apply(1): not translated, it has no body.
    }

    /// `SettingNodeBuilderFactoryManager` (xaero/hud/category/ui/setting/SettingNodeBuilderFactoryManager.java)
    pub mod setting_node_builder_factory_manager {
        use super::*;
        // The class itself isn't translated: it uses SettingNodeBuilderFactory, which isn't translated (it's an interface). Its static parts are below.
        // SettingNodeBuilderFactoryManager.SettingNodeBuilderFactoryManager(1): not translated, its class isn't translated (it uses SettingNodeBuilderFactory, which isn't translated (it's an interface)).
        // SettingNodeBuilderFactoryManager.register(2): not translated, its class isn't translated (it uses SettingNodeBuilderFactory, which isn't translated (it's an interface)).
        // SettingNodeBuilderFactoryManager.get(1): not translated, its class isn't translated (it uses SettingNodeBuilderFactory, which isn't translated (it's an interface)).
    }

    /// `SettingNodeBuilderFactoryManager.Builder` (xaero/hud/category/ui/setting/SettingNodeBuilderFactoryManager.java)
    pub mod setting_node_builder_factory_manager_builder {
        use super::*;
        // The class itself isn't translated: it uses MapFactory, which isn't translated (it's an interface). Its static parts are below.
        // SettingNodeBuilderFactoryManager.Builder.Builder(1): not translated, its class isn't translated (it uses MapFactory, which isn't translated (it's an interface)).
        // SettingNodeBuilderFactoryManager.Builder.setDefault(0): not translated, its class isn't translated (it uses MapFactory, which isn't translated (it's an interface)).
        // SettingNodeBuilderFactoryManager.Builder.build(0): not translated, its body couldn't be read: an anonymous class.
        // SettingNodeBuilderFactoryManager.Builder.begin(1): not translated, it uses Builder, which isn't translated (it uses MapFactory, which isn't translated (it's an interface)).
    }

    /// `CategoryConstants` (xaero/hud/category/util/CategoryConstants.java)
    pub mod category_constants {
        use super::*;
        // ON: a game value, so it's in game_side.rs.
        // OFF: a game value, so it's in game_side.rs.
        // INHERIT: a game value, so it's in game_side.rs.
        #[derive(Clone, Debug)]
        pub struct CategoryConstants {
        }
        impl CategoryConstants {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CategoryConstants.getBooleanComponent(1): needs the game, so it's in game_side.rs.
    }

    /// `HudModConfigCommonRegistryHandler` (xaero/hud/common/config/channel/register/HudModConfigCommonRegistryHandler.java)
    pub mod hud_mod_config_common_registry_handler {
        use super::*;
        // The class itself isn't translated: it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler). Its static parts are below.
        // HudModConfigCommonRegistryHandler.registerPrimaryCommonOptions(1): not translated, its class isn't translated (it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler)).
        // HudModConfigCommonRegistryHandler.registerProfiledOptions(1): not translated, its class isn't translated (it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler)).
        // HudModConfigCommonRegistryHandler.registerServerOptionChangeHandlers(1): not translated, its class isn't translated (it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler)).
        // HudModConfigCommonRegistryHandler.registerOptionServerRedirectors(1): not translated, its class isn't translated (it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler)).
    }

    /// `ImmediatelyFastHelper` (xaero/hud/compat/mods/ImmediatelyFastHelper.java)
    pub mod immediately_fast_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ImmediatelyFastHelper {
        }
        impl ImmediatelyFastHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ImmediatelyFastHelper.triggerBatchingBuffersFlush(1): not translated, it calls `ImmediateRenderUtil` (xaero.lib.client.graphics.util).coloredRectangle.
    }

    /// `SupportWaystones` (xaero/hud/compat/mods/SupportWaystones.java)
    pub mod support_waystones {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SupportWaystones {
        }
        impl SupportWaystones {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // SupportWaystones.registerClientEvents(0): not translated, it uses `WaystonesListReceivedEvent` (net.blay09.mods.waystones.api.event).EVENT.
            // SupportWaystones.onWaystoneDeactivation(1): not translated, it uses `Waystone` (net.blay09.mods.waystones.api).
            // SupportWaystones.addWaypoint(2): not translated, it uses `Waystone` (net.blay09.mods.waystones.api).
            // SupportWaystones.removeAllOfOrigin(2): not translated, it uses MinimapWorldRootContainer, which isn't translated (it extends `MinimapWorldContainer`).
            // SupportWaystones.removeWaypoint(4): not translated, it uses `UUID` (java.util).
            // SupportWaystones.removeWaypointFromAllOrigins(2): not translated, it uses `UUID` (java.util).
            // SupportWaystones.getWithConfigCheck(2): not translated, it calls `WaystoneKinds` (net.blay09.mods.waystones.api).isSharestone.
            pub fn get_discovered_origin_id(&self, waystone_type: &JIdentifier) -> JIdentifier {
                return waystone_type.clone();
            }
            pub fn get_global_origin_id(&self, waystone_type: &JIdentifier) -> JIdentifier {
                return waystone_type.clone();
            }
            // SupportWaystones.getSharestoneColor(1): not translated, it uses WaypointColor, which isn't translated (its constants carry values).
        }
    }

    /// `OldSystemCompatibility` (xaero/hud/compat/OldSystemCompatibility.java)
    pub mod old_system_compatibility {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct OldSystemCompatibility {
        }
        impl OldSystemCompatibility {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // OldSystemCompatibility.convertTransform(3): not translated, it uses ModuleSession, which isn't translated (it's generic).
            // OldSystemCompatibility.loadOldTransform(1): not translated, it calls `Integer`.parseInt.
        }
    }

    /// `HudModConfigClientRegistryHandler` (xaero/hud/config/channel/register/HudModConfigClientRegistryHandler.java)
    pub mod hud_mod_config_client_registry_handler {
        use super::*;
        // The class itself isn't translated: it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler). Its static parts are below.
        // HudModConfigClientRegistryHandler.registerPrimaryClientOptions(1): not translated, its class isn't translated (it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler)).
        // HudModConfigClientRegistryHandler.registerConfigOptionUITypes(1): not translated, its class isn't translated (it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler)).
        // HudModConfigClientRegistryHandler.registerClientOptionChangeHandlers(1): not translated, its class isn't translated (it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler)).
        // HudModConfigClientRegistryHandler.registerOptionClientRedirectors(1): not translated, its class isn't translated (it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler)).
    }

    /// `ControlsRegister` (xaero/hud/controls/ControlsRegister.java)
    pub mod controls_register {
        use super::*;
        // The class itself isn't translated: it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`). Its static parts are below.
        // ControlsRegister.ControlsRegister(0): not translated, its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)).
        // ControlsRegister.registerKeybindings(2): not translated, its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)).
        // ControlsRegister.onStage2(0): not translated, its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)).
        // ControlsRegister.getKeyMappingControllers(0): not translated, its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)).
    }

    /// `KeyMappingFunction` (xaero/hud/controls/key/function/KeyMappingFunction.java)
    pub mod key_mapping_function {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct KeyMappingFunction {
            pub held: bool,
        }
        impl KeyMappingFunction {
            pub fn new(held: bool) -> Self {
                let mut s = Self { held: held };
                s.held = held;
                s
            }
            // KeyMappingFunction.onPress(0): not translated, it has no body.
            // KeyMappingFunction.onRelease(0): not translated, it has no body.
            pub fn is_held(&self) -> bool {
                return self.held;
            }
        }
    }

    /// `KeyMappingController` (xaero/hud/controls/key/KeyMappingController.java)
    pub mod key_mapping_controller {
        use super::*;
        // The class itself isn't translated: it implements `Iterable`. Its static parts are below.
        // KeyMappingController.KeyMappingController(2): not translated, its class isn't translated (it implements `Iterable`).
        // KeyMappingController.add(1): not translated, its class isn't translated (it implements `Iterable`).
        // KeyMappingController.getKeyMapping(0): not translated, its class isn't translated (it implements `Iterable`).
        // KeyMappingController.getFunctions(0): not translated, its class isn't translated (it implements `Iterable`).
        // KeyMappingController.isXaeroKey(0): not translated, its class isn't translated (it implements `Iterable`).
        // KeyMappingController.isPressed(0): not translated, its class isn't translated (it implements `Iterable`).
        // KeyMappingController.setPressed(1): not translated, its class isn't translated (it implements `Iterable`).
        // KeyMappingController.iterator(0): not translated, its class isn't translated (it implements `Iterable`).
    }

    /// `KeyMappingControllerManager` (xaero/hud/controls/key/KeyMappingControllerManager.java)
    pub mod key_mapping_controller_manager {
        use super::*;
        // The class itself isn't translated: it implements `Iterable`. Its static parts are below.
        // KeyMappingControllerManager.getController(1): not translated, its class isn't translated (it implements `Iterable`).
        // KeyMappingControllerManager.registerController(2): not translated, its class isn't translated (it implements `Iterable`).
        // KeyMappingControllerManager.registerController(3): not translated, its class isn't translated (it implements `Iterable`).
        // KeyMappingControllerManager.registerFunction(2): not translated, its class isn't translated (it implements `Iterable`).
        // KeyMappingControllerManager.iterator(0): not translated, its class isn't translated (it implements `Iterable`).
    }

    /// `KeyMappingTickHandler` (xaero/hud/controls/key/KeyMappingTickHandler.java)
    pub mod key_mapping_tick_handler {
        use super::*;
        // The class itself isn't translated: it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`). Its static parts are below.
        pub static DISABLE_KEY_MAPPING_OVERRIDES: JCell<bool> = JCell::new(false);
        // KeyMappingTickHandler.KeyMappingTickHandler(1): not translated, its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)).
        // KeyMappingTickHandler.tick(0): not translated, its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)).
        // KeyMappingTickHandler.handlePress(1): not translated, its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)).
        // KeyMappingTickHandler.handleExtraPresses(1): not translated, its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)).
        // KeyMappingTickHandler.handleRelease(1): not translated, its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)).
        // KeyMappingTickHandler.handlePressFunctions(2): not translated, its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)).
        // KeyMappingTickHandler.handleReleaseFunctions(1): not translated, its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)).
        // KeyMappingTickHandler.getUpdatedPressedValue(1): not translated, its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)).
        // KeyMappingTickHandler.isPhysicallyDown(1): not translated, its class isn't translated (it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`)).
    }

    /// `EntityUtils` (xaero/hud/entity/EntityUtils.java)
    pub mod entity_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EntityUtils {
        }
        impl EntityUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EntityUtils.getEntityX(2): not translated, it uses Minecraft's `Entity`.
        // EntityUtils.getEntityY(2): not translated, it uses Minecraft's `Entity`.
        // EntityUtils.getEntityZ(2): not translated, it uses Minecraft's `Entity`.
        // EntityUtils.getEntityPos(2): not translated, it uses Minecraft's `Vec3`.
    }

    /// `HudEventHandler` (xaero/hud/event/HudEventHandler.java)
    pub mod hud_event_handler {
        use super::*;
        // The class itself isn't translated: it uses Hud, which isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))). Its static parts are below.
        // HudEventHandler.setHud(1): not translated, its class isn't translated (it uses Hud, which isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic)))).
        // HudEventHandler.handleRenderGameOverlayEventPost(0): not translated, it calls XaeroMinimapSession.getCurrentSession, which isn't translated.
    }

    /// `GuiUtils` (xaero/hud/gui/util/GuiUtils.java)
    pub mod gui_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct GuiUtils {
        }
        impl GuiUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // GuiUtils.getMinimapScale(1): not translated, it uses `ClientConfigManager` (xaero.lib.client.config).
        // GuiUtils.getUIScale(3): not translated, it calls Minecraft's `Minecraft`.getInstance.
        // GuiUtils.getAutoUIScale(0): not translated, it calls Minecraft's `Minecraft`.getInstance.
        // GuiUtils.refreshScreenBase(0): not translated, it uses Minecraft's `Screen`.
    }

    /// `XaeroSliderWidget` (xaero/hud/gui/widget/XaeroSliderWidget.java)
    pub mod xaero_slider_widget {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractSliderButton`. Its static parts are below.
        // XaeroSliderWidget.XaeroSliderWidget(8): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
        // XaeroSliderWidget.applyValue(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
        // XaeroSliderWidget.updateMessage(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
        // XaeroSliderWidget.setXaero_tooltip(1): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
        // XaeroSliderWidget.getXaero_tooltip(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
    }

    /// `Hud` (xaero/hud/Hud.java)
    pub mod hud {
        use super::*;
        // The class itself isn't translated: it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic)). Its static parts are below.
        // Hud.Hud(6): not translated, it stores a second reference to an object (Rust would need it copied or shared).
        // Hud.getModuleManager(0): not translated, its class isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))).
        // Hud.getPushboxManager(0): not translated, its class isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))).
        // Hud.getPresetManager(0): not translated, its class isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))).
        // Hud.getEventHandler(0): not translated, it stores a second reference to an object (Rust would need it copied or shared).
        // Hud.getOldSystemCompatibility(0): not translated, its class isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))).
        // Hud.getSessionHandler(0): not translated, its class isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))).
    }

    /// `Hud.Builder` (xaero/hud/Hud.java)
    pub mod hud_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // Hud.Builder.setDefault(0): not translated, `this` used as a value.
            // Hud.Builder.build(0): not translated, it creates a Hud with a constructor that isn't translated.
        }
        // Hud.Builder.begin(0): not translated, it calls Builder.setDefault, which isn't translated.
    }

    /// `HudSession` (xaero/hud/HudSession.java)
    pub mod hud_session {
        use super::*;
        // The class itself isn't translated: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`). Its static parts are below.
        // HudSession.HudSession(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // HudSession.getSession(1): not translated, it's generic.
        // HudSession.init(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // HudSession.tryCleanup(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // HudSession.cleanup(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // HudSession.getCurrentSession(0): not translated, it uses HudSession, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // HudSession.getForPlayer(1): not translated, it uses HudSession, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // HudSession.getKeyMappingTickHandler(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // HudSession.getHudMod(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
    }

    /// `HudIO` (xaero/hud/io/HudIO.java)
    pub mod hud_io {
        use super::*;
        // The class itself isn't translated: it uses Hud, which isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))). Its static parts are below.
        pub const SEPARATOR: &str = ";";
        pub const MODULE_LINE_PREFIX: &str = "module;";
        // HudIO.HudIO(2): not translated, its class isn't translated (it uses Hud, which isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic)))).
        // HudIO.save(1): not translated, it uses `PrintWriter` (java.io).
        // HudIO.load(2): not translated, it uses HudModule, which isn't translated (it's generic).
    }

    /// `HudIO.Builder` (xaero/hud/io/HudIO.java)
    pub mod hud_io_builder {
        use super::*;
        // The class itself isn't translated: it uses Hud, which isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))). Its static parts are below.
        // HudIO.Builder.Builder(0): not translated, its class isn't translated (it uses Hud, which isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic)))).
        // HudIO.Builder.setDefault(0): not translated, `this` used as a value.
        // HudIO.Builder.setHud(1): not translated, `this` used as a value.
        // HudIO.Builder.build(0): not translated, its class isn't translated (it uses Hud, which isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic)))).
        // HudIO.Builder.begin(0): not translated, it calls Builder.setDefault, which isn't translated.
    }

    /// `BuiltInHudModules` (xaero/hud/minimap/BuiltInHudModules.java)
    pub mod built_in_hud_modules {
        use super::*;
        // MINIMAP: not translated, it uses HudModule, which isn't translated (it's generic).
        #[derive(Clone, Debug)]
        pub struct BuiltInHudModules {
        }
        impl BuiltInHudModules {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BuiltInHudModules.addAll(1): not translated, it uses HudModule, which isn't translated (it's generic).
    }

    /// `InfoDisplayConfigData` (xaero/hud/minimap/common/config/info/config/InfoDisplayConfigData.java)
    pub mod info_display_config_data {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct InfoDisplayConfigData {
            pub background_color: i32,
            pub text_color: i32,
            pub state: String,
        }
        impl InfoDisplayConfigData {
            pub fn new(background_color: i32, text_color: i32, state: &str) -> Self {
                let mut s = Self { background_color: background_color, text_color: text_color, state: String::new() };
                s.background_color = background_color;
                s.text_color = text_color;
                s.state = state.to_string();
                s
            }
            pub fn get_background_color(&self) -> i32 {
                return self.background_color;
            }
            pub fn get_text_color(&self) -> i32 {
                return self.text_color;
            }
            pub fn get_state(&self) -> String {
                return self.state.to_string();
            }
        }
    }

    /// `InfoDisplayManagerConfigData` (xaero/hud/minimap/common/config/info/config/InfoDisplayManagerConfigData.java)
    pub mod info_display_manager_config_data {
        use super::*;
        pub static EMPTY: LazyLock<info_display_manager_config_data::InfoDisplayManagerConfigData> = LazyLock::new(|| info_display_manager_config_data::InfoDisplayManagerConfigData::new(&Vec::new(), &JMap::new()));
        #[derive(Clone, Debug)]
        pub struct InfoDisplayManagerConfigData {
            pub order: Vec<String>,
            pub configs: JMap<String, info_display_config_data::InfoDisplayConfigData>,
        }
        impl InfoDisplayManagerConfigData {
            pub fn new(order: &[String], configs: &JMap<String, info_display_config_data::InfoDisplayConfigData>) -> Self {
                let mut s = Self { order: Vec::new(), configs: configs.clone() };
                s.order = order.to_vec();
                s.configs = configs.clone();
                s
            }
            pub fn get_from_order(&self, index: i32) -> String {
                return self.order[index as usize].to_string();
            }
            // InfoDisplayManagerConfigData.getOrderStream(0): not translated, it uses `Stream` (java.util.stream).
            pub fn copy_order(&self) -> Vec<String> {
                return self.order.to_vec();
            }
            pub fn get(&self, id: &str) -> info_display_config_data::InfoDisplayConfigData {
                return ((self.configs.get(&*id)).expect("null")).clone();
            }
        }
    }

    /// `InfoDisplayManagerConfigData.Builder` (xaero/hud/minimap/common/config/info/config/InfoDisplayManagerConfigData.java)
    pub mod info_display_manager_config_data_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
            pub order: Vec<String>,
            pub configs: JMap<String, info_display_config_data::InfoDisplayConfigData>,
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self { order: Vec::new(), configs: JMap::new() };
                s
            }
            // InfoDisplayManagerConfigData.Builder.setDefault(0): not translated, `this` used as a value.
            // InfoDisplayManagerConfigData.Builder.add(2): not translated, `this` used as a value.
            pub fn build(&self) -> info_display_manager_config_data::InfoDisplayManagerConfigData {
                return info_display_manager_config_data::InfoDisplayManagerConfigData::new(&self.order, &self.configs);
            }
        }
        // InfoDisplayManagerConfigData.Builder.begin(0): not translated, it calls Builder.setDefault, which isn't translated.
    }

    /// `MinimapConfigConstants` (xaero/hud/minimap/common/config/MinimapConfigConstants.java)
    pub mod minimap_config_constants {
        use super::*;
        // DOTS_STYLES: a game value, so it's in game_side.rs.
        // MAIN_ENTITY_TYPES: a game value, so it's in game_side.rs.
        // ARROW_COLOR_NAMES: a game value, so it's in game_side.rs.
        pub static ARROW_COLORS: LazyLock<JCell<Vec<Vec<f32>>>> = LazyLock::new(|| JCell::new(vec![vec![0.8f32, 0.1f32, 0.1f32, 1.0f32], vec![0.09f32, 0.57f32, 0.0f32, 1.0f32], vec![0.0f32, 0.55f32, 1.0f32, 1.0f32], vec![1.0f32, 0.93f32, 0.0f32, 1.0f32], vec![0.73f32, 0.33f32, 0.83f32, 1.0f32], vec![1.0f32, 1.0f32, 1.0f32, 1.0f32], vec![0.0f32, 0.0f32, 0.0f32, 1.0f32], vec![0.4588f32, 0.0f32, 0.0f32, 1.0f32]]));
        // AUTO_SCALE_COMPONENT: a game value, so it's in game_side.rs.
        // MINECRAFT_SCALE_COMPONENT: a game value, so it's in game_side.rs.
        pub const DEFAULT_WORLD_WAYPOINT_SCALE: f32 = 0.8f32;
        pub const WORLD_WAYPOINT_MINECRAFT_SCALE: f32 = 0.02666667f32;
        pub const WAYPOINT_ICON_WORLD_SCALE: f64 = (0.021333335f32 as f64);
        // MULTIPLE_WAYPOINTS_INFO_NAMES: a game value, so it's in game_side.rs.
        // WAYPOINT_DISTANCE_NAMES: a game value, so it's in game_side.rs.
        // UNLIMITED_COMPONENT: a game value, so it's in game_side.rs.
        // CONVERT_DISTANCE_NEVER_COMPONENT: a game value, so it's in game_side.rs.
        // HIDE_WORLD_NAMES_NAMES: a game value, so it's in game_side.rs.
        // COMPASS_LOCATION_NAMES: a game value, so it's in game_side.rs.
        // COLOR_NAMES: a game value, so it's in game_side.rs.
        pub static COLORS: LazyLock<Vec<i32>> = LazyLock::new(|| vec![-16777216, -16777046, -16733696, -16733526, -5636096, -5635926, -22016, -5592406, -11184811, -11184641, -11141291, -11141121, -65536, -43521, -171, -1]);
        pub static COLOR_CODES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["0".to_string(), "1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(), "6".to_string(), "7".to_string(), "8".to_string(), "9".to_string(), "a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string(), "f".to_string()]);
        // BLOCK_COLOR_NAMES: a game value, so it's in game_side.rs.
        // TERRAIN_SLOPES_NAMES: a game value, so it's in game_side.rs.
        // MINIMAP_SHAPE_NAMES: a game value, so it's in game_side.rs.
        pub const MAX_ZOOM: i32 = 6;
        // FRAME_NAMES: a game value, so it's in game_side.rs.
        // PUSH_BOX_NAMES: a game value, so it's in game_side.rs.
        // LIGHT_NAMES: a game value, so it's in game_side.rs.
        // INFO_DISPLAY_ALIGNMENT_NAMES: a game value, so it's in game_side.rs.
        #[derive(Clone, Debug)]
        pub struct MinimapConfigConstants {
        }
        impl MinimapConfigConstants {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `MinimapProfiledConfigOptions` (xaero/hud/minimap/common/config/option/MinimapProfiledConfigOptions.java)
    pub mod minimap_profiled_config_options {
        use super::*;
        // ALL: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // DISPLAY_MINIMAP: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // MINIMAP_ITEM: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // SAFE_MODE: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // UI_SCALE: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // RADAR_CATEGORIES: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // DISPLAY_RADAR: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // RADAR_DOTS_STYLE: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // RADAR_SMOOTH_DOTS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // RADAR_NAME_SCALE: not translated, it uses `SteppedConfigOption` (xaero.lib.common.config.option).
        // RADAR_MAIN_ENTITY: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // RADAR_MAIN_DOT_SIZE: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // RADAR_HIDE_INVISIBLE: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // ARROW_SCALE: not translated, it uses `SteppedConfigOption` (xaero.lib.common.config.option).
        // ARROW_COLOR: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // ARROW_OPACITY: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // TRACKED_PLAYERS_ON_MINIMAP: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // TRACKED_PLAYERS_IN_WORLD: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // TRACKED_PLAYER_MINIMAP_ICON_SCALE: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // TRACKED_PLAYER_WORLD_ICON_SCALE: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // TRACKED_PLAYER_WORLD_NAME_SCALE: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // WAYPOINTS_IN_WORLD: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // WAYPOINTS_ON_MINIMAP: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // WAYPOINTS_ALL_SETS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // DEATHPOINTS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // OLD_DEATHPOINTS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // DELETE_REACHED_DEATHPOINTS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // AUTO_WAYPOINTS_ON_DEATH: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_ICON_SCALE_IN_WORLD: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_NAME_SCALE_IN_WORLD: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_CLOSE_SCALE_IN_WORLD: not translated, it uses `SteppedConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_DISTANCE_SCALE_IN_WORLD: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // MULTIPLE_WAYPOINTS_INFO: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_DISTANCE_IN_WORLD: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_DISTANCE_IN_MENU: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_NAME_IN_WORLD: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_SHORT_DISTANCE_IN_WORLD: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_VERTICAL_POINTING_ANGLE: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_HORIZONTAL_POINTING_ANGLE: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_MAX_DISTANCE: not translated, it uses `IndexedConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_MAX_DISTANCE_DIMENSION_SCALE: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_MIN_DISTANCE_IN_WORLD: not translated, it uses `SteppedConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_CONVERT_DISTANCE_TO_KM_AT: not translated, it uses `IndexedConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_DISTANCE_PRECISION: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_OPACITY_IN_WORLD: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_OPACITY_ON_MINIMAP: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_ICON_SCALE_ON_MINIMAP: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // TEMPORARY_WAYPOINTS_GLOBAL: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_DEFAULT_TELEPORT_FORMAT: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_DEFAULT_TELEPORT_ROTATION_FORMAT: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_TELEPORT_CROSS_DIMENSION: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_PARTIAL_Y_TELEPORT: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // NEW_WAYPOINTS_TO_BOTTOM: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // HIDE_WORLD_NAMES: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // COMPASS_OVER_EVERYTHING: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // HIDE_WAYPOINT_COORDINATES: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // COMPASS_LOCATION: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // COMPASS_SHADOW_COLOR: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // COMPASS_NORTH_SHADOW_COLOR: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // COMPASS_SCALE: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // BLOCK_COLORS: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // BIOMES_IN_VANILLA_COLORS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // CAVE_MODE_ALLOWED: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // AUTO_CAVE_MODE: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // CAVE_MODE_ALLOWED_DIMENSIONS: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // MANUAL_CAVE_MODE_START: not translated, it uses `IndexedConfigOption` (xaero.lib.common.config.option).
        // CAVE_MODE_DEPTH: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // LEGIBLE_CAVE_MAPS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // CAVE_MODE_TOGGLE_TIMER: not translated, it uses `SteppedConfigOption` (xaero.lib.common.config.option).
        // BIOME_BLENDING: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // DISPLAY_WORLD_MAP_CHUNKS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // TERRAIN_DEPTH: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // TERRAIN_SLOPES: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // DISPLAY_REDSTONE: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // DISPLAY_FLOWERS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // BLOCK_TRANSPARENCY: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // DISPLAY_STAINED_GLASS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // ADJUST_HEIGHT_FOR_SHORT_BLOCKS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // ANTI_ALIASING: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // SIZE: not translated, it uses `IndexedConfigOption` (xaero.lib.common.config.option).
        // SHAPE: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // NORTH_LOCKED: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // ZOOM: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // FURTHER_ZOOMOUT: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // CAVE_ZOOM: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // ZOOM_ENLARGED: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // CENTERED_ENLARGED: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // KEEP_ENLARGED_UNLOCKED: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // TOGGLED_ENLARGED: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // LIGHTING: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // OPACITY: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // FRAME: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // FRAME_COLOR: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // HIDE_UNDER_SCREEN: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // HIDE_UNDER_F3: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // BOSS_HEALTH_PUSH_BOX: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // POTION_EFFECT_PUSH_BOX: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // CHUNK_GRID: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // CHUNK_GRID_LINE_WIDTH: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // SLIME_CHUNKS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // OPEN_SLIME_CHUNKS_SCREEN: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // LIGHT_OVERLAY_TYPE: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // LIGHT_OVERLAY_MAX_LIGHT: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // LIGHT_OVERLAY_MIN_LIGHT: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // LIGHT_OVERLAY_COLOR: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // OPAC_CLAIMS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // OPAC_CLAIMS_FILL_OPACITY: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // OPAC_CLAIMS_BORDER_OPACITY: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // OPAC_CURRENT_CLAIM: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // INFO_DISPLAY_BG_OPACITY: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // INFO_DISPLAY_ALIGNMENT: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // INFO_DISPLAY_CONFIG: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // RENDER_LAYER: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // UNDISCOVERED_OPACITY: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // DISCOVERED_WAYSTONE_WAYPOINTS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // OTHER_WAYSTONE_WAYPOINTS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        #[derive(Clone, Debug)]
        pub struct MinimapProfiledConfigOptions {
        }
        impl MinimapProfiledConfigOptions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MinimapProfiledConfigOptions.registerAll(1): not translated, it uses `ConfigOptionManager` (xaero.lib.common.config.option).
        // MinimapProfiledConfigOptions.getDefaultBossHealthPushBox(0): not translated, a try/catch (ClassNotFoundException).
    }

    /// `MinimapConfigOptionServerRedirectors` (xaero/hud/minimap/common/config/option/value/redirect/MinimapConfigOptionServerRedirectors.java)
    pub mod minimap_config_option_server_redirectors {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MinimapConfigOptionServerRedirectors {
        }
        impl MinimapConfigOptionServerRedirectors {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MinimapConfigOptionServerRedirectors.registerAll(1): not translated, it uses `OptionValueRedirectorManager` (xaero.lib.common.config.option.value.redirect).
    }

    /// `MinimapValueIOCodecs` (xaero/hud/minimap/common/config/option/value/type/io/serialization/MinimapValueIOCodecs.java)
    pub mod minimap_value_io_codecs {
        use super::*;
        // ENTITY_RADAR_CATEGORIES: not translated, it uses `LargeConfigValueIOCodec` (xaero.lib.common.config.option.value.io.serialization).
        // INFO_DISPLAY_CONFIG: not translated, it uses `LargeConfigValueIOCodec` (xaero.lib.common.config.option.value.io.serialization).
        #[derive(Clone, Debug)]
        pub struct MinimapValueIOCodecs {
        }
        impl MinimapValueIOCodecs {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `MinimapConfigValueTypes` (xaero/hud/minimap/common/config/option/value/type/MinimapConfigValueTypes.java)
    pub mod minimap_config_value_types {
        use super::*;
        // ENTITY_RADAR_CATEGORIES: not translated, it uses `ConfigValueType` (xaero.lib.common.config.option.value.type).
        // INFO_DISPLAY_CONFIG: not translated, it uses `ConfigValueType` (xaero.lib.common.config.option.value.type).
        #[derive(Clone, Debug)]
        pub struct MinimapConfigValueTypes {
        }
        impl MinimapConfigValueTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `MinimapValueSyncCodecs` (xaero/hud/minimap/common/config/option/value/type/sync/serialization/MinimapValueSyncCodecs.java)
    pub mod minimap_value_sync_codecs {
        use super::*;
        // ENTITY_RADAR_CATEGORIES: not translated, it uses `ConfigValueSyncCodec` (xaero.lib.common.config.option.value.sync.serialization).
        // INFO_DISPLAY_CONFIG: not translated, it uses `ConfigValueSyncCodec` (xaero.lib.common.config.option.value.sync.serialization).
        #[derive(Clone, Debug)]
        pub struct MinimapValueSyncCodecs {
        }
        impl MinimapValueSyncCodecs {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `MinimapPrimaryCommonConfigOptions` (xaero/hud/minimap/common/config/primary/option/MinimapPrimaryCommonConfigOptions.java)
    pub mod minimap_primary_common_config_options {
        use super::*;
        // ALL: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // REGISTER_EFFECTS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        #[derive(Clone, Debug)]
        pub struct MinimapPrimaryCommonConfigOptions {
        }
        impl MinimapPrimaryCommonConfigOptions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MinimapPrimaryCommonConfigOptions.registerAll(1): not translated, it uses `ConfigOptionManager` (xaero.lib.common.config.option).
    }

    /// `MinimapConfigOptionServerHandlers` (xaero/hud/minimap/common/config/server/listener/handler/MinimapConfigOptionServerHandlers.java)
    pub mod minimap_config_option_server_handlers {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MinimapConfigOptionServerHandlers {
        }
        impl MinimapConfigOptionServerHandlers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MinimapConfigOptionServerHandlers.registerAll(1): not translated, it uses `HandlerBasedConfigChangeListener` (xaero.lib.common.config.listener.handler).
    }

    /// `MinimapConfigUtils` (xaero/hud/minimap/common/config/util/MinimapConfigUtils.java)
    pub mod minimap_config_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MinimapConfigUtils {
        }
        impl MinimapConfigUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MinimapConfigUtils.getUIScaleDisplayGetter(3): not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // MinimapConfigUtils.getUIScaleDisplayGetter(4): not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // MinimapConfigUtils.getUIScaleDisplayGetter(2): not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // MinimapConfigUtils.getUIScaleDisplayGetter(3): not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // MinimapConfigUtils.getAutoMinimapSizeName(0): not translated, it uses Minecraft's `Component`.
    }

    /// `EntityRadarCategorySerializers` (xaero/hud/minimap/common/radar/category/EntityRadarCategorySerializers.java)
    pub mod entity_radar_category_serializers {
        use super::*;
        // The class itself isn't translated: it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic). Its static parts are below.
        // EntityRadarCategorySerializers.EntityRadarCategorySerializers(2): not translated, its class isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)).
        // EntityRadarCategorySerializers.getGson(0): not translated, its class isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)).
        // EntityRadarCategorySerializers.getNbt(0): not translated, its class isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)).
    }

    /// `EntityRadarCategorySerializers.Builder` (xaero/hud/minimap/common/radar/category/EntityRadarCategorySerializers.java)
    pub mod entity_radar_category_serializers_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // EntityRadarCategorySerializers.Builder.setDefault(0): not translated, `this` used as a value.
            // EntityRadarCategorySerializers.Builder.build(0): not translated, it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)).
        }
        // EntityRadarCategorySerializers.Builder.begin(0): not translated, it calls Builder.setDefault, which isn't translated.
    }

    /// `CardinalDirection` (xaero/hud/minimap/compass/render/CardinalDirection.java)
    pub mod cardinal_direction {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CardinalDirection.CardinalDirection(1): not translated, its class isn't translated (its constants carry values).
        // CardinalDirection.getInitials(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `CompassRenderer` (xaero/hud/minimap/compass/render/CompassRenderer.java)
    pub mod compass_renderer {
        use super::*;
        // The class itself isn't translated: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`). Its static parts are below.
        // CompassRenderer.CompassRenderer(2): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // CompassRenderer.drawCompass(11): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
    }

    /// `MinimapConfigOptionClientHandlers` (xaero/hud/minimap/config/listener/handler/MinimapConfigOptionClientHandlers.java)
    pub mod minimap_config_option_client_handlers {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MinimapConfigOptionClientHandlers {
        }
        impl MinimapConfigOptionClientHandlers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MinimapConfigOptionClientHandlers.resetImage(0): not translated, it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapConfigOptionClientHandlers.handleSafeMode(1): not translated, it uses `Config` (xaero.lib.common.config).
        // MinimapConfigOptionClientHandlers.handleMinimapItem(1): not translated, it uses `Config` (xaero.lib.common.config).
        // MinimapConfigOptionClientHandlers.handleEntityRadarCategories(1): not translated, it uses `Config` (xaero.lib.common.config).
        // MinimapConfigOptionClientHandlers.requestClaimsRefresh(0): not translated, it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapConfigOptionClientHandlers.handleOpacClaims(1): not translated, it uses `Config` (xaero.lib.common.config).
        // MinimapConfigOptionClientHandlers.handleOpacClaimsOpacity(1): not translated, it uses `Config` (xaero.lib.common.config).
        // MinimapConfigOptionClientHandlers.handleLegibleCaveMaps(1): not translated, it uses `Config` (xaero.lib.common.config).
        // MinimapConfigOptionClientHandlers.handleDisplayWorldMapChunks(1): not translated, it uses `Config` (xaero.lib.common.config).
        // MinimapConfigOptionClientHandlers.handleBlockTransparency(1): not translated, it uses `Config` (xaero.lib.common.config).
        // MinimapConfigOptionClientHandlers.handleDisplayStainedGlass(1): not translated, it uses `Config` (xaero.lib.common.config).
        // MinimapConfigOptionClientHandlers.handleInfoDisplayConfig(1): not translated, it uses `Config` (xaero.lib.common.config).
        // MinimapConfigOptionClientHandlers.registerAll(1): not translated, it uses `ClientConfigChangeListener` (xaero.lib.client.config.listener).
    }

    /// `MinimapConfigOptionUIRegister` (xaero/hud/minimap/config/option/ui/MinimapConfigOptionUIRegister.java)
    pub mod minimap_config_option_ui_register {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MinimapConfigOptionUIRegister {
        }
        impl MinimapConfigOptionUIRegister {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MinimapConfigOptionUIRegister.registerAll(1): not translated, it uses `ConfigOptionUITypeManager` (xaero.lib.client.config.option.ui).
    }

    /// `MinimapConfigOptionUITypes` (xaero/hud/minimap/config/option/ui/type/MinimapConfigOptionUITypes.java)
    pub mod minimap_config_option_ui_types {
        use super::*;
        // RADAR_CATEGORIES_EDITOR: not translated, it uses `ConfigOptionUIType` (xaero.lib.client.config.option.ui.type).
        // INFO_DISPLAY_CONFIG_EDITOR: not translated, it uses `ConfigOptionUIType` (xaero.lib.client.config.option.ui.type).
        #[derive(Clone, Debug)]
        pub struct MinimapConfigOptionUITypes {
        }
        impl MinimapConfigOptionUITypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `MinimapConfigOptionClientRedirectors` (xaero/hud/minimap/config/option/value/redirect/MinimapConfigOptionClientRedirectors.java)
    pub mod minimap_config_option_client_redirectors {
        use super::*;
        // LEGACY_PLUGIN_TOOLTIP: a game value, so it's in game_side.rs.
        // FAIRPLAY_TOOLTIP: a game value, so it's in game_side.rs.
        // SAFE_MODE_TOOLTIP: a game value, so it's in game_side.rs.
        // EFFECT_TOOLTIP: a game value, so it's in game_side.rs.
        // FROM_WORLDMAP_TOOLTIP: a game value, so it's in game_side.rs.
        // HARD_WORLDMAP_TOOLTIP: a game value, so it's in game_side.rs.
        // FROM_WORLDMAP_SCREEN_TOOLTIP: a game value, so it's in game_side.rs.
        // WORLD_MAP_COMPONENT: a game value, so it's in game_side.rs.
        #[derive(Clone, Debug)]
        pub struct MinimapConfigOptionClientRedirectors {
        }
        impl MinimapConfigOptionClientRedirectors {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MinimapConfigOptionClientRedirectors.registerAll(1): not translated, it uses `ClientOptionValueRedirectorManager` (xaero.lib.client.config.option.value.redirect).
    }

    /// `MinimapPrimaryClientConfigOptions` (xaero/hud/minimap/config/primary/option/MinimapPrimaryClientConfigOptions.java)
    pub mod minimap_primary_client_config_options {
        use super::*;
        // ALL: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // UPDATE_NOTIFICATIONS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // IGNORED_UPDATE: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // WRONG_WORLD_TELEPORT: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // DIFFERENTIATE_BY_SERVER_ADDRESS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // DEBUG_ENTITY_ICONS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // DEBUG_ENTITY_VARIANT_IDS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_MUTUAL_EDIT: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        #[derive(Clone, Debug)]
        pub struct MinimapPrimaryClientConfigOptions {
        }
        impl MinimapPrimaryClientConfigOptions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MinimapPrimaryClientConfigOptions.registerAll(1): not translated, it uses `ConfigOptionManager` (xaero.lib.common.config.option).
    }

    /// `MinimapConfigClientUtils` (xaero/hud/minimap/config/util/MinimapConfigClientUtils.java)
    pub mod minimap_config_client_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MinimapConfigClientUtils {
        }
        impl MinimapConfigClientUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MinimapConfigClientUtils.getUIScale(2): not translated, it uses `ClientConfigManager` (xaero.lib.client.config).
        // MinimapConfigClientUtils.getUIScale(3): not translated, it uses `ClientConfigManager` (xaero.lib.client.config).
        // MinimapConfigClientUtils.getUIScale(5): not translated, it uses `ClientConfigManager` (xaero.lib.client.config).
        // MinimapConfigClientUtils.getWaypointsClampDepth(3): not translated, it uses `ClientConfigManager` (xaero.lib.client.config).
        // MinimapConfigClientUtils.addAutoUIScaleValueToComponent(2): not translated, it uses Minecraft's `MutableComponent`.
        // MinimapConfigClientUtils.addAutoMCScaleValueToComponent(1): not translated, it uses Minecraft's `MutableComponent`.
        // MinimapConfigClientUtils.getAutoMinimapSize(0): not translated, it calls Minecraft's `Minecraft`.getInstance.
        // MinimapConfigClientUtils.getEffectiveMinimapSize(0): not translated, it uses `ClientConfigManager` (xaero.lib.client.config).
        // MinimapConfigClientUtils.getEffectiveNorthLocked(2): not translated, it uses `ClientConfigManager` (xaero.lib.client.config).
        // MinimapConfigClientUtils.changeZoomUnchecked(2): not translated, it uses `ClientConfigManager` (xaero.lib.client.config).
        // MinimapConfigClientUtils.changeZoom(1): not translated, it uses `ClientConfigManager` (xaero.lib.client.config).
        // MinimapConfigClientUtils.getEffectiveSlimeChunks(1): not translated, it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapConfigClientUtils.getEffectiveSlimeChunksSeed(1): not translated, it uses Minecraft's `MinecraftServer`.
        // MinimapConfigClientUtils.isFairPlayForCaveMode(0): not translated, it uses HudMod.INSTANCE, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapConfigClientUtils.hasNoCaveModeEffect(0): not translated, it uses Minecraft's `Minecraft`.
        // MinimapConfigClientUtils.getEffectiveCaveModeAllowed(0): not translated, it uses `ClientConfigManager` (xaero.lib.client.config).
        // MinimapConfigClientUtils.getActualZoom(1): not translated, it's overloaded with the same number of parameters.
        // MinimapConfigClientUtils.getActualZoom(1): not translated, it's overloaded with the same number of parameters.
        // MinimapConfigClientUtils.areDiscoveredWaystonesEnabled(0): not translated, it uses `ClientConfigManager` (xaero.lib.client.config).
        // MinimapConfigClientUtils.areOtherWaystonesEnabled(0): not translated, it uses `ClientConfigManager` (xaero.lib.client.config).
    }

    /// `AddWaypointFunction` (xaero/hud/minimap/controls/key/function/AddWaypointFunction.java)
    pub mod add_waypoint_function {
        use super::*;
        // The class itself isn't translated: it extends `KeyMappingFunction` (xaero.hud.controls.key.function). Its static parts are below.
        // AddWaypointFunction.AddWaypointFunction(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // AddWaypointFunction.onPress(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // AddWaypointFunction.onRelease(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
    }

    /// `HeldEnlargeMapFunction` (xaero/hud/minimap/controls/key/function/HeldEnlargeMapFunction.java)
    pub mod held_enlarge_map_function {
        use super::*;
        // The class itself isn't translated: it extends `KeyMappingFunction` (xaero.hud.controls.key.function). Its static parts are below.
        // HeldEnlargeMapFunction.HeldEnlargeMapFunction(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // HeldEnlargeMapFunction.onPress(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // HeldEnlargeMapFunction.onRelease(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
    }

    /// `LightOverlayFunction` (xaero/hud/minimap/controls/key/function/LightOverlayFunction.java)
    pub mod light_overlay_function {
        use super::*;
        // The class itself isn't translated: it extends `KeyMappingFunction` (xaero.hud.controls.key.function). Its static parts are below.
        // LightOverlayFunction.LightOverlayFunction(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // LightOverlayFunction.onPress(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // LightOverlayFunction.onRelease(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
    }

    /// `ManualCaveModeFunction` (xaero/hud/minimap/controls/key/function/ManualCaveModeFunction.java)
    pub mod manual_cave_mode_function {
        use super::*;
        // The class itself isn't translated: it extends `KeyMappingFunction` (xaero.hud.controls.key.function). Its static parts are below.
        // ManualCaveModeFunction.ManualCaveModeFunction(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // ManualCaveModeFunction.onPress(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // ManualCaveModeFunction.onRelease(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
    }

    /// `MinimapKeyMappingFunctions` (xaero/hud/minimap/controls/key/function/MinimapKeyMappingFunctions.java)
    pub mod minimap_key_mapping_functions {
        use super::*;
        // ZOOM_IN: not translated, it creates a ZoomFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // ZOOM_OUT: not translated, it creates a ZoomFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // TOGGLE_RADAR: not translated, it creates a ToggleSettingFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // TOGGLE_TRACKED_PLAYER_MAP: not translated, it creates a ToggleSettingFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // TOGGLE_TRACKED_PLAYER_WORLD: not translated, it creates a ToggleSettingFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // ADD_WAYPOINT: not translated, it creates a AddWaypointFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // WAYPOINT_MENU: not translated, it creates a WaypointMenuFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // HELD_ENLARGE_MAP: not translated, it creates a HeldEnlargeMapFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // TOGGLED_ENLARGE_MAP: not translated, it creates a ToggledEnlargeMapFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // TOGGLE_MAP: not translated, it creates a ToggleMapFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // TOGGLE_WORLD_WAYPOINTS: not translated, it creates a ToggleSettingFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // TOGGLE_MAP_WAYPOINTS: not translated, it creates a ToggleSettingFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // TOGGLE_SLIME_CHUNKS: not translated, it creates a ToggleSlimeChunksFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // TOGGLE_GRID: not translated, it creates a ToggleGridFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // TEMPORARY_WAYPOINT: not translated, it creates a TemporaryWaypointFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // SWITCH_WAYPOINT_SET: not translated, it creates a SwitchWaypointSetFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // RENDER_ALL_SETS: not translated, it creates a ToggleSettingFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // LIGHT_OVERLAY: not translated, it creates a LightOverlayFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // MANUAL_CAVE_MODE: not translated, it creates a ManualCaveModeFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // TOGGLE_OPAC_CLAIMS: not translated, it creates a OpacClaimsFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        #[derive(Clone, Debug)]
        pub struct MinimapKeyMappingFunctions {
        }
        impl MinimapKeyMappingFunctions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MinimapKeyMappingFunctions.registerAll(1): not translated, it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`).
    }

    /// `OpacClaimsFunction` (xaero/hud/minimap/controls/key/function/OpacClaimsFunction.java)
    pub mod opac_claims_function {
        use super::*;
        // The class itself isn't translated: it extends `KeyMappingFunction` (xaero.hud.controls.key.function). Its static parts are below.
        // OpacClaimsFunction.OpacClaimsFunction(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // OpacClaimsFunction.onPress(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // OpacClaimsFunction.onRelease(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
    }

    /// `SwitchWaypointSetFunction` (xaero/hud/minimap/controls/key/function/SwitchWaypointSetFunction.java)
    pub mod switch_waypoint_set_function {
        use super::*;
        // The class itself isn't translated: it extends `KeyMappingFunction` (xaero.hud.controls.key.function). Its static parts are below.
        // SwitchWaypointSetFunction.SwitchWaypointSetFunction(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // SwitchWaypointSetFunction.onPress(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // SwitchWaypointSetFunction.onRelease(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
    }

    /// `TemporaryWaypointFunction` (xaero/hud/minimap/controls/key/function/TemporaryWaypointFunction.java)
    pub mod temporary_waypoint_function {
        use super::*;
        // The class itself isn't translated: it extends `KeyMappingFunction` (xaero.hud.controls.key.function). Its static parts are below.
        // TemporaryWaypointFunction.TemporaryWaypointFunction(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // TemporaryWaypointFunction.onPress(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // TemporaryWaypointFunction.onRelease(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
    }

    /// `ToggledEnlargeMapFunction` (xaero/hud/minimap/controls/key/function/ToggledEnlargeMapFunction.java)
    pub mod toggled_enlarge_map_function {
        use super::*;
        // The class itself isn't translated: it extends `KeyMappingFunction` (xaero.hud.controls.key.function). Its static parts are below.
        // ToggledEnlargeMapFunction.ToggledEnlargeMapFunction(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // ToggledEnlargeMapFunction.onPress(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // ToggledEnlargeMapFunction.onRelease(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
    }

    /// `ToggleGridFunction` (xaero/hud/minimap/controls/key/function/ToggleGridFunction.java)
    pub mod toggle_grid_function {
        use super::*;
        // The class itself isn't translated: it extends `KeyMappingFunction` (xaero.hud.controls.key.function). Its static parts are below.
        // ToggleGridFunction.ToggleGridFunction(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // ToggleGridFunction.onPress(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // ToggleGridFunction.onRelease(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
    }

    /// `ToggleMapFunction` (xaero/hud/minimap/controls/key/function/ToggleMapFunction.java)
    pub mod toggle_map_function {
        use super::*;
        // The class itself isn't translated: it extends `KeyMappingFunction` (xaero.hud.controls.key.function). Its static parts are below.
        // ToggleMapFunction.ToggleMapFunction(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // ToggleMapFunction.onPress(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // ToggleMapFunction.onRelease(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
    }

    /// `ToggleSettingFunction` (xaero/hud/minimap/controls/key/function/ToggleSettingFunction.java)
    pub mod toggle_setting_function {
        use super::*;
        // The class itself isn't translated: it extends `KeyMappingFunction` (xaero.hud.controls.key.function). Its static parts are below.
        // ToggleSettingFunction.ToggleSettingFunction(1): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // ToggleSettingFunction.onPress(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // ToggleSettingFunction.onRelease(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
    }

    /// `ToggleSlimeChunksFunction` (xaero/hud/minimap/controls/key/function/ToggleSlimeChunksFunction.java)
    pub mod toggle_slime_chunks_function {
        use super::*;
        // The class itself isn't translated: it extends `KeyMappingFunction` (xaero.hud.controls.key.function). Its static parts are below.
        // ToggleSlimeChunksFunction.ToggleSlimeChunksFunction(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // ToggleSlimeChunksFunction.onPress(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // ToggleSlimeChunksFunction.onRelease(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
    }

    /// `WaypointMenuFunction` (xaero/hud/minimap/controls/key/function/WaypointMenuFunction.java)
    pub mod waypoint_menu_function {
        use super::*;
        // The class itself isn't translated: it extends `KeyMappingFunction` (xaero.hud.controls.key.function). Its static parts are below.
        // WaypointMenuFunction.WaypointMenuFunction(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // WaypointMenuFunction.onPress(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // WaypointMenuFunction.onRelease(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
    }

    /// `ZoomFunction` (xaero/hud/minimap/controls/key/function/ZoomFunction.java)
    pub mod zoom_function {
        use super::*;
        // The class itself isn't translated: it extends `KeyMappingFunction` (xaero.hud.controls.key.function). Its static parts are below.
        // ZoomFunction.ZoomFunction(1): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // ZoomFunction.onPress(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // ZoomFunction.onRelease(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
    }

    /// `MinimapKeyMappings` (xaero/hud/minimap/controls/key/MinimapKeyMappings.java)
    pub mod minimap_key_mappings {
        use super::*;
        // CATEGORY: not translated, it uses Minecraft's `Category`.
        // ZOOM_IN: not translated, it uses Minecraft's `KeyMapping`.
        // ZOOM_OUT: not translated, it uses Minecraft's `KeyMapping`.
        // ADD_WAYPOINT: not translated, it uses Minecraft's `KeyMapping`.
        // WAYPOINT_MENU: not translated, it uses Minecraft's `KeyMapping`.
        // ENLARGE_MAP: not translated, it uses Minecraft's `KeyMapping`.
        // TOGGLE_MAP: not translated, it uses Minecraft's `KeyMapping`.
        // TOGGLE_WORLD_WAYPOINTS: not translated, it uses Minecraft's `KeyMapping`.
        // TOGGLE_MAP_WAYPOINTS: not translated, it uses Minecraft's `KeyMapping`.
        // TOGGLE_SLIME_CHUNKS: not translated, it uses Minecraft's `KeyMapping`.
        // TOGGLE_GRID: not translated, it uses Minecraft's `KeyMapping`.
        // TEMPORARY_WAYPOINT: not translated, it uses Minecraft's `KeyMapping`.
        // SWITCH_WAYPOINT_SET: not translated, it uses Minecraft's `KeyMapping`.
        // RENDER_ALL_SETS: not translated, it uses Minecraft's `KeyMapping`.
        // LIGHT_OVERLAY: not translated, it uses Minecraft's `KeyMapping`.
        // TOGGLE_RADAR: not translated, it uses Minecraft's `KeyMapping`.
        // REVERSE_ENTITY_RADAR: not translated, it uses Minecraft's `KeyMapping`.
        // MANUAL_CAVE_MODE: not translated, it uses Minecraft's `KeyMapping`.
        // ALTERNATIVE_LIST_PLAYERS: not translated, it uses Minecraft's `KeyMapping`.
        // TOGGLE_TRACKED_PLAYERS_MAP: not translated, it uses Minecraft's `KeyMapping`.
        // TOGGLE_TRACKED_PLAYERS_WORLD: not translated, it uses Minecraft's `KeyMapping`.
        // TOGGLE_OPAC_CLAIMS: not translated, it uses Minecraft's `KeyMapping`.
        #[derive(Clone, Debug)]
        pub struct MinimapKeyMappings {
        }
        impl MinimapKeyMappings {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MinimapKeyMappings.registerAll(3): not translated, it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`).
    }

    /// `MinimapElementMapRendererHandler` (xaero/hud/minimap/element/render/map/MinimapElementMapRendererHandler.java)
    pub mod minimap_element_map_renderer_handler {
        use super::*;
        // The class itself isn't translated: it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render). Its static parts are below.
        // MinimapElementMapRendererHandler.MinimapElementMapRendererHandler(3): not translated, its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)).
        // MinimapElementMapRendererHandler.prepareRender(4): not translated, its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)).
        // MinimapElementMapRendererHandler.transformAndRenderForRenderer(10): not translated, it's generic.
        // MinimapElementMapRendererHandler.beforeRender(2): not translated, its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)).
        // MinimapElementMapRendererHandler.afterRender(2): not translated, its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)).
    }

    /// `MinimapElementMapRendererHandler.Builder` (xaero/hud/minimap/element/render/map/MinimapElementMapRendererHandler.java)
    pub mod minimap_element_map_renderer_handler_builder {
        use super::*;
        // The class itself isn't translated: it extends `Builder`. Its static parts are below.
        // MinimapElementMapRendererHandler.Builder.build(0): not translated, its class isn't translated (it extends `Builder`).
        // MinimapElementMapRendererHandler.Builder.buildInternally(2): not translated, its class isn't translated (it extends `Builder`).
        // MinimapElementMapRendererHandler.Builder.setDefault(0): not translated, its class isn't translated (it extends `Builder`).
        // MinimapElementMapRendererHandler.Builder.begin(0): not translated, it uses Builder, which isn't translated (it extends `Builder`).
    }

    /// `MinimapElementGraphics` (xaero/hud/minimap/element/render/MinimapElementGraphics.java)
    pub mod minimap_element_graphics {
        use super::*;
        // The class itself isn't translated: it uses `XaeroBufferProvider` (xaero.lib.client.graphics). Its static parts are below.
        // MinimapElementGraphics.MinimapElementGraphics(1): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MinimapElementGraphics.fill(5): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MinimapElementGraphics.fill(6): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MinimapElementGraphics.fillGradient(6): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MinimapElementGraphics.blit(6): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MinimapElementGraphics.blit(11): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MinimapElementGraphics.blit(11): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MinimapElementGraphics.drawCenteredString(5): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MinimapElementGraphics.drawCenteredString(5): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MinimapElementGraphics.drawCenteredString(5): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MinimapElementGraphics.drawString(5): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MinimapElementGraphics.drawString(6): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MinimapElementGraphics.drawString(5): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MinimapElementGraphics.drawString(6): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MinimapElementGraphics.drawString(5): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MinimapElementGraphics.drawString(6): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MinimapElementGraphics.getBufferSource(0): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MinimapElementGraphics.pose(0): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MinimapElementGraphics.flush(0): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
    }

    /// `MinimapElementReader` (xaero/hud/minimap/element/render/MinimapElementReader.java)
    pub mod minimap_element_reader {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MinimapElementReader.isHidden(2): not translated, it has no body.
        // MinimapElementReader.getRenderX(3): not translated, it has no body.
        // MinimapElementReader.getRenderY(3): not translated, it has no body.
        // MinimapElementReader.getRenderZ(3): not translated, it has no body.
        // MinimapElementReader.getCoordinateScale(3): not translated, its class isn't translated (it's generic).
        // MinimapElementReader.shouldScalePartialCoordinates(3): not translated, its class isn't translated (it's generic).
        // MinimapElementReader.getInteractionBoxLeft(3): not translated, it has no body.
        // MinimapElementReader.getInteractionBoxRight(3): not translated, it has no body.
        // MinimapElementReader.getInteractionBoxTop(3): not translated, it has no body.
        // MinimapElementReader.getInteractionBoxBottom(3): not translated, it has no body.
        // MinimapElementReader.getRenderBoxLeft(3): not translated, it has no body.
        // MinimapElementReader.getRenderBoxRight(3): not translated, it has no body.
        // MinimapElementReader.getRenderBoxTop(3): not translated, it has no body.
        // MinimapElementReader.getRenderBoxBottom(3): not translated, it has no body.
        // MinimapElementReader.getLeftSideLength(2): not translated, it has no body.
        // MinimapElementReader.getMenuName(1): not translated, it has no body.
        // MinimapElementReader.getFilterName(1): not translated, it has no body.
        // MinimapElementReader.getMenuTextFillLeftPadding(1): not translated, it has no body.
        // MinimapElementReader.getRightClickTitleBackgroundColor(1): not translated, it has no body.
        // MinimapElementReader.shouldScaleBoxWithOptionalScale(0): not translated, it has no body.
        // MinimapElementReader.isInteractable(2): not translated, its class isn't translated (it's generic).
        // MinimapElementReader.getBoxScale(3): not translated, its class isn't translated (it's generic).
        // MinimapElementReader.isAlwaysHighlightedWhenHovered(2): not translated, its class isn't translated (it's generic).
    }

    /// `MinimapElementRenderer` (xaero/hud/minimap/element/render/MinimapElementRenderer.java)
    pub mod minimap_element_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MinimapElementRenderer.MinimapElementRenderer(3): not translated, its class isn't translated (it's generic).
        // MinimapElementRenderer.getOrder(0): not translated, its class isn't translated (it's generic).
        // MinimapElementRenderer.compareTo(1): not translated, its class isn't translated (it's generic).
        // MinimapElementRenderer.getContext(0): not translated, its class isn't translated (it's generic).
        // MinimapElementRenderer.getProvider(0): not translated, its class isn't translated (it's generic).
        // MinimapElementRenderer.getElementReader(0): not translated, its class isn't translated (it's generic).
        // MinimapElementRenderer.renderElement(10): not translated, it has no body.
        // MinimapElementRenderer.preRender(3): not translated, it has no body.
        // MinimapElementRenderer.postRender(3): not translated, it has no body.
        // MinimapElementRenderer.shouldRender(1): not translated, it has no body.
    }

    /// `MinimapElementRendererHandler` (xaero/hud/minimap/element/render/MinimapElementRendererHandler.java)
    pub mod minimap_element_renderer_handler {
        use super::*;
        // The class itself isn't translated: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`). Its static parts are below.
        // MinimapElementRendererHandler.MinimapElementRendererHandler(5): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapElementRendererHandler.add(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapElementRendererHandler.render(5): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapElementRendererHandler.renderForRenderer(5): not translated, it's generic.
        // MinimapElementRendererHandler.transformAndRenderForRenderer(7): not translated, it's generic.
        // MinimapElementRendererHandler.getElementIndexDepth(2): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapElementRendererHandler.getGuiGraphics(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapElementRendererHandler.getIndexLimit(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapElementRendererHandler.transformAndRenderForRenderer(10): not translated, it has no body.
        // MinimapElementRendererHandler.beforeRender(2): not translated, it has no body.
        // MinimapElementRendererHandler.afterRender(2): not translated, it has no body.
    }

    /// `MinimapElementRendererHandler.Builder` (xaero/hud/minimap/element/render/MinimapElementRendererHandler.java)
    pub mod minimap_element_renderer_handler_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MinimapElementRendererHandler.Builder.Builder(0): not translated, its class isn't translated (it's generic).
        // MinimapElementRendererHandler.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // MinimapElementRendererHandler.Builder.setPoseStack(1): not translated, its class isn't translated (it's generic).
        // MinimapElementRendererHandler.Builder.build(0): not translated, its class isn't translated (it's generic).
        // MinimapElementRendererHandler.Builder.buildInternally(2): not translated, it has no body.
    }

    /// `MinimapElementRenderInfo` (xaero/hud/minimap/element/render/MinimapElementRenderInfo.java)
    pub mod minimap_element_render_info {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Entity`. Its static parts are below.
        // MinimapElementRenderInfo.MinimapElementRenderInfo(9): not translated, its class isn't translated (it uses Minecraft's `Entity`).
    }

    /// `MinimapElementRenderLocation` (xaero/hud/minimap/element/render/MinimapElementRenderLocation.java)
    pub mod minimap_element_render_location {
        use super::*;
        pub static UNKNOWN: LazyLock<minimap_element_render_location::MinimapElementRenderLocation> = LazyLock::new(|| minimap_element_render_location::MinimapElementRenderLocation::new("unknown"));
        pub static IN_MINIMAP: LazyLock<minimap_element_render_location::MinimapElementRenderLocation> = LazyLock::new(|| minimap_element_render_location::MinimapElementRenderLocation::new("in_minimap"));
        pub static OVER_MINIMAP: LazyLock<minimap_element_render_location::MinimapElementRenderLocation> = LazyLock::new(|| minimap_element_render_location::MinimapElementRenderLocation::new("over_minimap"));
        pub static IN_WORLD: LazyLock<minimap_element_render_location::MinimapElementRenderLocation> = LazyLock::new(|| minimap_element_render_location::MinimapElementRenderLocation::new("in_world"));
        pub static WORLD_MAP: LazyLock<minimap_element_render_location::MinimapElementRenderLocation> = LazyLock::new(|| minimap_element_render_location::MinimapElementRenderLocation::new("world_map"));
        pub static WORLD_MAP_MENU: LazyLock<minimap_element_render_location::MinimapElementRenderLocation> = LazyLock::new(|| minimap_element_render_location::MinimapElementRenderLocation::new("world_map_menu"));
        #[derive(Clone, Debug)]
        pub struct MinimapElementRenderLocation {
            pub name: String,
        }
        impl MinimapElementRenderLocation {
            pub fn new(name: &str) -> Self {
                let mut s = Self { name: String::new() };
                s.name = name.to_string();
                s
            }
            pub fn get_name(&self) -> String {
                return self.name.to_string();
            }
        }
    }

    /// `MinimapElementRenderProvider` (xaero/hud/minimap/element/render/MinimapElementRenderProvider.java)
    pub mod minimap_element_render_provider {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MinimapElementRenderProvider.begin(2): not translated, it has no body.
        // MinimapElementRenderProvider.hasNext(2): not translated, it has no body.
        // MinimapElementRenderProvider.getNext(2): not translated, it has no body.
        // MinimapElementRenderProvider.setupContextAndGetNext(2): not translated, its class isn't translated (it's generic).
        // MinimapElementRenderProvider.end(2): not translated, it has no body.
    }

    /// `MinimapElementOverMapRendererHandler` (xaero/hud/minimap/element/render/over/MinimapElementOverMapRendererHandler.java)
    pub mod minimap_element_over_map_renderer_handler {
        use super::*;
        // The class itself isn't translated: it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render). Its static parts are below.
        // MinimapElementOverMapRendererHandler.MinimapElementOverMapRendererHandler(4): not translated, its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)).
        // MinimapElementOverMapRendererHandler.prepareRender(9): not translated, its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)).
        // MinimapElementOverMapRendererHandler.transformAndRenderForRenderer(10): not translated, it's generic.
        // MinimapElementOverMapRendererHandler.beforeRender(2): not translated, its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)).
        // MinimapElementOverMapRendererHandler.afterRender(2): not translated, its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)).
        pub fn translate_position(matrix_stack: &mut JPoseStack, spec_w: i32, spec_h: i32, half_view_w: i32, half_view_h: i32, ps: f64, pc: f64, offx: f64, offy: f64, zoom: f64, circle: bool, partial_translate: &mut [f64]) -> bool {
            let mut out_of_bounds: bool = false;
            let y: f64 = ((pc * offx) + (ps * offy)) * zoom;
            let x: f64 = ((ps * offx) - (pc * offy)) * zoom;
            let mut bordered_x: f64 = x;
            let mut bordered_y: f64 = y;
            if !circle {
                if bordered_x > (spec_w as f64) {
                    bordered_x = (spec_w as f64);
                    bordered_y = (y * (spec_w as f64)) / x;
                    out_of_bounds = true;
                } else if bordered_x < ((-spec_w) as f64) {
                    bordered_x = ((-spec_w) as f64);
                    bordered_y = ((-y) * (spec_w as f64)) / x;
                    out_of_bounds = true;
                }
                if bordered_y > (spec_h as f64) {
                    bordered_y = (spec_h as f64);
                    bordered_x = (x * (spec_h as f64)) / y;
                    out_of_bounds = true;
                } else if bordered_y < ((-spec_h) as f64) {
                    bordered_y = ((-spec_h) as f64);
                    bordered_x = ((-x) * (spec_h as f64)) / y;
                    out_of_bounds = true;
                }
                if (!out_of_bounds) && ((((bordered_x > (half_view_w as f64)) || (bordered_x < ((-half_view_w) as f64))) || (bordered_y > (half_view_h as f64))) || (bordered_y < ((-half_view_h) as f64))) {
                    out_of_bounds = true;
                }
            } else {
                let dist_squared: f64 = (bordered_x * bordered_x) + (bordered_y * bordered_y);
                let max_dist_squared: f64 = ((spec_w * spec_w) as f64);
                if dist_squared > max_dist_squared {
                    let scale_down: f64 = (max_dist_squared / dist_squared).sqrt();
                    bordered_x *= scale_down;
                    bordered_y *= scale_down;
                    out_of_bounds = true;
                }
                if (!out_of_bounds) && (dist_squared > ((half_view_w * half_view_w) as f64)) {
                    out_of_bounds = true;
                }
            }
            let rounded_x: i64 = j_round_d(bordered_x);
            let rounded_y: i64 = j_round_d(bordered_y);
            partial_translate[0] = bordered_x - (rounded_x as f64);
            partial_translate[1] = bordered_y - (rounded_y as f64);
            matrix_stack.translate((rounded_x as f32), (rounded_y as f32), 0.0f32);
            return out_of_bounds;
        }
    }

    /// `MinimapElementOverMapRendererHandler.Builder` (xaero/hud/minimap/element/render/over/MinimapElementOverMapRendererHandler.java)
    pub mod minimap_element_over_map_renderer_handler_builder {
        use super::*;
        // The class itself isn't translated: it extends `Builder`. Its static parts are below.
        // MinimapElementOverMapRendererHandler.Builder.build(0): not translated, its class isn't translated (it extends `Builder`).
        // MinimapElementOverMapRendererHandler.Builder.buildInternally(2): not translated, its class isn't translated (it extends `Builder`).
        // MinimapElementOverMapRendererHandler.Builder.setDefault(0): not translated, its class isn't translated (it extends `Builder`).
        // MinimapElementOverMapRendererHandler.Builder.begin(0): not translated, it uses Builder, which isn't translated (it extends `Builder`).
    }

    /// `MinimapElementWorldRendererHandler` (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java)
    pub mod minimap_element_world_renderer_handler {
        use super::*;
        // The class itself isn't translated: it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render). Its static parts are below.
        pub const DEFAULT_SCALE: f32 = 0.8f32;
        pub const MINECRAFT_SCALE: f32 = 0.02666667f32;
        pub const ELEMENT_WORLD_SCALE: f64 = (0.021333335f32 as f64);
        // MinimapElementWorldRendererHandler.MinimapElementWorldRendererHandler(5): not translated, its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)).
        // MinimapElementWorldRendererHandler.prepareRender(2): not translated, its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)).
        // MinimapElementWorldRendererHandler.render(5): not translated, its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)).
        // MinimapElementWorldRendererHandler.transformAndRenderForRenderer(10): not translated, it's generic.
        // MinimapElementWorldRendererHandler.isElementHovered(6): not translated, it's generic.
        // MinimapElementWorldRendererHandler.handleClosestHovered(4): not translated, it's generic.
        // MinimapElementWorldRendererHandler.renderMainHighlightedElement(2): not translated, it's generic.
        // MinimapElementWorldRendererHandler.beforeRender(2): not translated, its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)).
        // MinimapElementWorldRendererHandler.afterRender(2): not translated, its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)).
        // MinimapElementWorldRendererHandler.ensureOrthoProjectionCache(0): not translated, its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)).
        // MinimapElementWorldRendererHandler.getOrthoProjection(2): not translated, its class isn't translated (it extends `MinimapElementRendererHandler` (xaero.hud.minimap.element.render)).
    }

    /// `MinimapElementWorldRendererHandler.Builder` (xaero/hud/minimap/element/render/world/MinimapElementWorldRendererHandler.java)
    pub mod minimap_element_world_renderer_handler_builder {
        use super::*;
        // The class itself isn't translated: it extends `Builder`. Its static parts are below.
        // MinimapElementWorldRendererHandler.Builder.buildInternally(2): not translated, its class isn't translated (it extends `Builder`).
        // MinimapElementWorldRendererHandler.Builder.build(0): not translated, its class isn't translated (it extends `Builder`).
        // MinimapElementWorldRendererHandler.Builder.setDefault(0): not translated, its class isn't translated (it extends `Builder`).
        // MinimapElementWorldRendererHandler.Builder.begin(0): not translated, it uses Builder, which isn't translated (it extends `Builder`).
    }

    /// `BuiltInInfoDisplays` (xaero/hud/minimap/info/BuiltInInfoDisplays.java)
    pub mod built_in_info_displays {
        use super::*;
        // ALL: not translated, it uses InfoDisplay, which isn't translated (it's generic).
        // COORDINATES: not translated, it uses InfoDisplay, which isn't translated (it's generic).
        // OVERWORLD_COORDINATES: not translated, it uses InfoDisplay, which isn't translated (it's generic).
        // CHUNK_COORDINATES: not translated, it uses InfoDisplay, which isn't translated (it's generic).
        // ANGLES: not translated, it uses InfoDisplay, which isn't translated (it's generic).
        // DIMENSION: not translated, it uses InfoDisplay, which isn't translated (it's generic).
        // BIOME: not translated, it uses InfoDisplay, which isn't translated (it's generic).
        // WEATHER: not translated, it uses InfoDisplay, which isn't translated (it's generic).
        // LIGHT_LEVEL: not translated, it uses InfoDisplay, which isn't translated (it's generic).
        // TIME: not translated, it uses InfoDisplay, which isn't translated (it's generic).
        // CALENDAR: not translated, it uses `Calendar` (java.util).
        // REAL_TIME: not translated, it uses InfoDisplay, which isn't translated (it's generic).
        // HIGHLIGHTS: not translated, it uses InfoDisplay, which isn't translated (it's generic).
        // LIGHT_OVERLAY_INDICATOR: not translated, it uses InfoDisplay, which isn't translated (it's generic).
        // MANUAL_CAVE_MODE_INDICATOR: not translated, it uses InfoDisplay, which isn't translated (it's generic).
        // CUSTOM_SUB_WORLD: not translated, it uses InfoDisplay, which isn't translated (it's generic).
        #[derive(Clone, Debug)]
        pub struct BuiltInInfoDisplays {
        }
        impl BuiltInInfoDisplays {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BuiltInInfoDisplays.forEach(1): not translated, it uses InfoDisplay, which isn't translated (it's generic).
    }

    /// `InfoDisplayCommonStateCodecs` (xaero/hud/minimap/info/codec/InfoDisplayCommonStateCodecs.java)
    pub mod info_display_common_state_codecs {
        use super::*;
        // BOOLEAN: not translated, it uses InfoDisplayStateCodec, which isn't translated (it's generic).
        // INTEGER: not translated, it uses InfoDisplayStateCodec, which isn't translated (it's generic).
        #[derive(Clone, Debug)]
        pub struct InfoDisplayCommonStateCodecs {
        }
        impl InfoDisplayCommonStateCodecs {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `InfoDisplayStateCodec` (xaero/hud/minimap/info/codec/InfoDisplayStateCodec.java)
    pub mod info_display_state_codec {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // InfoDisplayStateCodec.InfoDisplayStateCodec(3): not translated, its class isn't translated (it's generic).
    }

    /// `InfoDisplayConfigClientUtils` (xaero/hud/minimap/info/config/InfoDisplayConfigClientUtils.java)
    pub mod info_display_config_client_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct InfoDisplayConfigClientUtils {
        }
        impl InfoDisplayConfigClientUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // InfoDisplayConfigClientUtils.createDefaultConfig(3): not translated, it's generic.
        // InfoDisplayConfigClientUtils.createDefaultConfig(3): not translated, it's generic.
    }

    /// `InfoDisplay` (xaero/hud/minimap/info/InfoDisplay.java)
    pub mod info_display {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // InfoDisplay.InfoDisplay(7): not translated, its class isn't translated (it's generic).
        // InfoDisplay.setManager(1): not translated, its class isn't translated (it's generic).
        // InfoDisplay.getState(0): not translated, its class isn't translated (it's generic).
        // InfoDisplay.getEffectiveState(0): not translated, its class isn't translated (it's generic).
        // InfoDisplay.getTextColor(0): not translated, its class isn't translated (it's generic).
        // InfoDisplay.getBackgroundColor(0): not translated, its class isn't translated (it's generic).
        // InfoDisplay.updateConfigCache(0): not translated, its class isn't translated (it's generic).
        // InfoDisplay.getId(0): not translated, its class isn't translated (it's generic).
        // InfoDisplay.getName(0): not translated, its class isn't translated (it's generic).
        // InfoDisplay.getDefaultState(0): not translated, its class isn't translated (it's generic).
        // InfoDisplay.getCodec(0): not translated, its class isn't translated (it's generic).
        // InfoDisplay.createWidget(7): not translated, its class isn't translated (it's generic).
        // InfoDisplay.getCompiler(0): not translated, its class isn't translated (it's generic).
        // InfoDisplay.clearStateCache(0): not translated, its class isn't translated (it's generic).
        // InfoDisplay.getLegacyValue(1): not translated, its class isn't translated (it's generic).
    }

    /// `InfoDisplay.Builder` (xaero/hud/minimap/info/InfoDisplay.java)
    pub mod info_display_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // InfoDisplay.Builder.Builder(0): not translated, its class isn't translated (it's generic).
        // InfoDisplay.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // InfoDisplay.Builder.setId(1): not translated, its class isn't translated (it's generic).
        // InfoDisplay.Builder.setName(1): not translated, its class isn't translated (it's generic).
        // InfoDisplay.Builder.setDefaultState(1): not translated, its class isn't translated (it's generic).
        // InfoDisplay.Builder.setCodec(1): not translated, its class isn't translated (it's generic).
        // InfoDisplay.Builder.setCodec(1): not translated, its class isn't translated (it's generic).
        // InfoDisplay.Builder.setWidgetFactory(1): not translated, its class isn't translated (it's generic).
        // InfoDisplay.Builder.setCompiler(1): not translated, its class isn't translated (it's generic).
        // InfoDisplay.Builder.setDestination(1): not translated, its class isn't translated (it's generic).
        // InfoDisplay.Builder.setLegacyGetter(1): not translated, its class isn't translated (it's generic).
        // InfoDisplay.Builder.build(0): not translated, its class isn't translated (it's generic).
        // InfoDisplay.Builder.begin(0): not translated, it's generic.
    }

    /// `InfoDisplay.ConfigCache` (xaero/hud/minimap/info/InfoDisplay.java)
    pub mod info_display_config_cache {
        use super::*;
        // The class itself isn't translated: it uses `T`. Its static parts are below.
        // BG_COLOR_GETTER: not translated, a function kept in a variable or field (only parameters can be).
        // TEXT_COLOR_GETTER: not translated, a function kept in a variable or field (only parameters can be).
        // InfoDisplay.ConfigCache.ConfigCache(0): not translated, its class isn't translated (it uses `T`).
        // InfoDisplay.ConfigCache.fetchStateFromConfig(0): not translated, its class isn't translated (it uses `T`).
        // InfoDisplay.ConfigCache.fetchBackgroundColorFromConfig(0): not translated, its class isn't translated (it uses `T`).
        // InfoDisplay.ConfigCache.fetchTextColorFromConfig(0): not translated, its class isn't translated (it uses `T`).
        // InfoDisplay.ConfigCache.fetchColorFromConfig(2): not translated, its class isn't translated (it uses `T`).
    }

    /// `InfoDisplayIO` (xaero/hud/minimap/info/InfoDisplayIO.java)
    pub mod info_display_io {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct InfoDisplayIO {
        }
        impl InfoDisplayIO {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // InfoDisplayIO.encode(1): not translated, it calls InfoDisplayManagerConfigData.getOrderStream, which isn't translated.
            // InfoDisplayIO.getInfoDisplayLine(2): not translated, `null` (Rust has no null: this needs an Option).
            // InfoDisplayIO.decode(1): not translated, it calls InfoDisplayIO.loadInfoDisplay, which isn't translated.
            pub fn load_info_display_order_line(&self, args: &[String], destination: &mut Vec<String>) {
                if (args.len() as i32) > 1 {
                    for i in 1..(args.len() as i32) {
                        if !(destination.iter().any(|e| e == &args[i as usize])) {
                            destination.push(args[i as usize].to_string());
                        }
                    }
                }
            }
            // InfoDisplayIO.loadInfoDisplay(2): not translated, `null` (Rust has no null: this needs an Option).
        }
    }

    /// `InfoDisplayManager` (xaero/hud/minimap/info/InfoDisplayManager.java)
    pub mod info_display_manager {
        use super::*;
        // The class itself isn't translated: it uses InfoDisplay, which isn't translated (it's generic). Its static parts are below.
        // InfoDisplayManager.InfoDisplayManager(6): not translated, its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)).
        // InfoDisplayManager.add(1): not translated, its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)).
        // InfoDisplayManager.applyLocalConfig(0): not translated, its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)).
        // InfoDisplayManager.adaptOrder(1): not translated, its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)).
        // InfoDisplayManager.getLocalConfig(0): not translated, its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)).
        // InfoDisplayManager.getEnforcedConfig(0): not translated, its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)).
        // InfoDisplayManager.get(1): not translated, its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)).
        // InfoDisplayManager.getOrderedStream(0): not translated, its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)).
        // InfoDisplayManager.getStream(0): not translated, its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)).
        // InfoDisplayManager.getCount(0): not translated, its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)).
        // InfoDisplayManager.getDefaultOrder(0): not translated, its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)).
        // InfoDisplayManager.clearStateCache(0): not translated, its class isn't translated (it uses InfoDisplay, which isn't translated (it's generic)).
    }

    /// `InfoDisplayManager.Builder` (xaero/hud/minimap/info/InfoDisplayManager.java)
    pub mod info_display_manager_builder {
        use super::*;
        // The class itself isn't translated: a function kept in a variable or field (only parameters can be). Its static parts are below.
        // InfoDisplayManager.Builder.Builder(0): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // InfoDisplayManager.Builder.setDefault(0): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // InfoDisplayManager.Builder.setLocalConfigSupplier(1): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // InfoDisplayManager.Builder.setLocalConfigSetter(1): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // InfoDisplayManager.Builder.setEnforcedConfigSupplier(1): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // InfoDisplayManager.Builder.build(0): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // InfoDisplayManager.Builder.begin(0): not translated, it uses Builder, which isn't translated (a function kept in a variable or field (only parameters can be)).
    }

    /// `InfoDisplays` (xaero/hud/minimap/info/InfoDisplays.java)
    pub mod info_displays {
        use super::*;
        // The class itself isn't translated: it uses InfoDisplayManager, which isn't translated (it uses InfoDisplay, which isn't translated (it's generic)). Its static parts are below.
        // InfoDisplays.InfoDisplays(1): not translated, its class isn't translated (it uses InfoDisplayManager, which isn't translated (it uses InfoDisplay, which isn't translated (it's generic))).
        // InfoDisplays.getManager(0): not translated, its class isn't translated (it uses InfoDisplayManager, which isn't translated (it uses InfoDisplay, which isn't translated (it's generic))).
        // InfoDisplays.getIo(0): not translated, its class isn't translated (it uses InfoDisplayManager, which isn't translated (it uses InfoDisplay, which isn't translated (it's generic))).
        // InfoDisplays.getRenderer(0): not translated, its class isn't translated (it uses InfoDisplayManager, which isn't translated (it uses InfoDisplay, which isn't translated (it's generic))).
        // InfoDisplays.clearStateCache(0): not translated, its class isn't translated (it uses InfoDisplayManager, which isn't translated (it uses InfoDisplay, which isn't translated (it's generic))).
    }

    /// `InfoDisplayCompiler` (xaero/hud/minimap/info/render/compile/InfoDisplayCompiler.java)
    pub mod info_display_compiler {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Component`. Its static parts are below.
        // InfoDisplayCompiler.InfoDisplayCompiler(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // InfoDisplayCompiler.compile(4): not translated, it's generic.
        // InfoDisplayCompiler.addWords(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // InfoDisplayCompiler.addLine(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // InfoDisplayCompiler.addLine(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
    }

    /// `InfoDisplayCompiler.Builder` (xaero/hud/minimap/info/render/compile/InfoDisplayCompiler.java)
    pub mod info_display_compiler_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // InfoDisplayCompiler.Builder.setDefault(0): not translated, `this` used as a value.
            // InfoDisplayCompiler.Builder.build(0): not translated, it uses InfoDisplayCompiler, which isn't translated (it uses Minecraft's `Component`).
        }
        // InfoDisplayCompiler.Builder.begin(0): not translated, it calls Builder.setDefault, which isn't translated.
    }

    /// `InfoDisplayOnCompile` (xaero/hud/minimap/info/render/compile/InfoDisplayOnCompile.java)
    pub mod info_display_on_compile {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // InfoDisplayOnCompile.onCompile(5): not translated, it has no body.
    }

    /// `InfoDisplayRenderer` (xaero/hud/minimap/info/render/InfoDisplayRenderer.java)
    pub mod info_display_renderer {
        use super::*;
        // The class itself isn't translated: it uses InfoDisplayCompiler, which isn't translated (it uses Minecraft's `Component`). Its static parts are below.
        pub const DEPTH_OFFSET: i32 = 2;
        // InfoDisplayRenderer.InfoDisplayRenderer(1): not translated, its class isn't translated (it uses InfoDisplayCompiler, which isn't translated (it uses Minecraft's `Component`)).
        // InfoDisplayRenderer.render(9): not translated, its class isn't translated (it uses InfoDisplayCompiler, which isn't translated (it uses Minecraft's `Component`)).
    }

    /// `InfoDisplayRenderer.Builder` (xaero/hud/minimap/info/render/InfoDisplayRenderer.java)
    pub mod info_display_renderer_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // InfoDisplayRenderer.Builder.setDefault(0): not translated, `this` used as a value.
            // InfoDisplayRenderer.Builder.build(0): not translated, it uses InfoDisplayRenderer, which isn't translated (it uses InfoDisplayCompiler, which isn't translated (it uses Minecraft's `Component`)).
        }
        // InfoDisplayRenderer.Builder.begin(0): not translated, it calls Builder.setDefault, which isn't translated.
    }

    /// `InfoDisplayCommonWidgetFactories` (xaero/hud/minimap/info/widget/InfoDisplayCommonWidgetFactories.java)
    pub mod info_display_common_widget_factories {
        use super::*;
        // OFF_ON: not translated, it uses InfoDisplayCycleWidgetFactory, which isn't translated (it's generic).
        // ALWAYS_ON: not translated, it uses InfoDisplayWidgetFactory, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct InfoDisplayCommonWidgetFactories {
        }
        impl InfoDisplayCommonWidgetFactories {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `InfoDisplayCycleButton` (xaero/hud/minimap/info/widget/InfoDisplayCycleButton.java)
    pub mod info_display_cycle_button {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Plain`. Its static parts are below.
        // InfoDisplayCycleButton.InfoDisplayCycleButton(7): not translated, its class isn't translated (it extends Minecraft's `Plain`).
    }

    /// `InfoDisplayCycleButton.Builder` (xaero/hud/minimap/info/widget/InfoDisplayCycleButton.java)
    pub mod info_display_cycle_button_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // InfoDisplayCycleButton.Builder.Builder(0): not translated, its class isn't translated (it's generic).
        // InfoDisplayCycleButton.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // InfoDisplayCycleButton.Builder.setBounds(4): not translated, its class isn't translated (it's generic).
        // InfoDisplayCycleButton.Builder.setValues(2): not translated, its class isn't translated (it's generic).
        // InfoDisplayCycleButton.Builder.setEntry(1): not translated, its class isn't translated (it's generic).
        // InfoDisplayCycleButton.Builder.setOnChange(1): not translated, its class isn't translated (it's generic).
        // InfoDisplayCycleButton.Builder.setIncludeNull(1): not translated, its class isn't translated (it's generic).
        // InfoDisplayCycleButton.Builder.build(0): not translated, its class isn't translated (it's generic).
        // InfoDisplayCycleButton.Builder.begin(0): not translated, it's generic.
    }

    /// `InfoDisplayCycleWidgetFactory` (xaero/hud/minimap/info/widget/InfoDisplayCycleWidgetFactory.java)
    pub mod info_display_cycle_widget_factory {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // InfoDisplayCycleWidgetFactory.InfoDisplayCycleWidgetFactory(2): not translated, its class isn't translated (it's generic).
        // InfoDisplayCycleWidgetFactory.create(7): not translated, its class isn't translated (it's generic).
    }

    /// `InfoDisplayWidgetFactory` (xaero/hud/minimap/info/widget/InfoDisplayWidgetFactory.java)
    pub mod info_display_widget_factory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // InfoDisplayWidgetFactory.create(7): not translated, it has no body.
    }

    /// `Minimap` (xaero/hud/minimap/Minimap.java)
    pub mod minimap {
        use super::*;
        // The class itself isn't translated: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`). Its static parts are below.
        // Minimap.Minimap(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // Minimap.getCrashedWith(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // Minimap.setCrashedWith(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // Minimap.checkCrashes(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // Minimap.getWaypointMapRenderer(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // Minimap.getWaypointWorldRenderer(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // Minimap.getMinimapFBORenderer(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // Minimap.getMinimapSafeModeRenderer(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // Minimap.getOverMapRendererHandler(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // Minimap.getWorldRendererHandler(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // Minimap.usingFBO(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // Minimap.getCompassRenderer(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // Minimap.getInfoDisplays(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // Minimap.getMatrixStack(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // Minimap.getModMain(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
    }

    /// `MinimapLogs` (xaero/hud/minimap/MinimapLogs.java)
    pub mod minimap_logs {
        use super::*;
        // LOGGER: the logger. Log calls become comments.
        #[derive(Clone, Debug)]
        pub struct MinimapLogs {
        }
        impl MinimapLogs {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `MinimapRenderer` (xaero/hud/minimap/module/MinimapRenderer.java)
    pub mod minimap_renderer_2 {
        use super::*;
        // The class itself isn't translated: another class has the same name, in a different package. Its static parts are below.
        // MinimapRenderer.render(4): not translated, its class isn't translated (another class has the same name, in a different package).
    }

    /// `MinimapSession` (xaero/hud/minimap/module/MinimapSession.java)
    pub mod minimap_session {
        use super::*;
        // The class itself isn't translated: it extends `ModuleSession` (xaero.hud.module). Its static parts are below.
        // MinimapSession.MinimapSession(3): not translated, its class isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapSession.prePotentialRender(0): not translated, its class isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapSession.close(0): not translated, its class isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapSession.getWidth(1): not translated, its class isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapSession.getHeight(1): not translated, its class isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapSession.getConfiguredWidth(0): not translated, its class isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapSession.getProcessor(0): not translated, its class isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapSession.getHideMinimapUnderScreen(0): not translated, its class isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapSession.getHideMinimapUnderF3(0): not translated, its class isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapSession.getMultiTextureRenderTypeRenderers(0): not translated, its class isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapSession.getWaypointSession(0): not translated, its class isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapSession.getRadarSession(0): not translated, its class isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapSession.getMc(0): not translated, its class isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapSession.getWorldManager(0): not translated, its class isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapSession.getWorldState(0): not translated, its class isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapSession.getWorldStateUpdater(0): not translated, its class isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapSession.getDimensionHelper(0): not translated, its class isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        // MinimapSession.getWorldManagerIO(0): not translated, its class isn't translated (it extends `ModuleSession` (xaero.hud.module)).
    }

    /// `PlayerTrackerIconRenderer` (xaero/hud/minimap/player/tracker/PlayerTrackerIconRenderer.java)
    pub mod player_tracker_icon_renderer {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PlayerTrackerIconRenderer {
        }
        impl PlayerTrackerIconRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // PlayerTrackerIconRenderer.isPlayerUpsideDown(1): not translated, it uses Minecraft's `Player`.
            // PlayerTrackerIconRenderer.renderIcon(7): not translated, it uses Minecraft's `Minecraft`.
        }
        pub fn is_upside_down_name(player_name: &str) -> bool {
            return j_eq("Dinnerbone", &player_name) || j_eq("Grumm", &player_name);
        }
    }

    /// `PlayerTrackerMinimapElement` (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElement.java)
    pub mod player_tracker_minimap_element {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // PlayerTrackerMinimapElement.PlayerTrackerMinimapElement(2): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMinimapElement.getPlayerId(0): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMinimapElement.getX(0): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMinimapElement.getY(0): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMinimapElement.getZ(0): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMinimapElement.getDimension(0): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMinimapElement.getPlayer(0): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMinimapElement.setRenderedOnRadar(1): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMinimapElement.wasRenderedOnRadar(0): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMinimapElement.getSystem(0): not translated, its class isn't translated (it's generic).
    }

    /// `PlayerTrackerMinimapElementCollector` (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementCollector.java)
    pub mod player_tracker_minimap_element_collector {
        use super::*;
        // The class itself isn't translated: it uses `UUID` (java.util). Its static parts are below.
        // PlayerTrackerMinimapElementCollector.PlayerTrackerMinimapElementCollector(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // PlayerTrackerMinimapElementCollector.update(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // PlayerTrackerMinimapElementCollector.updateForSystem(3): not translated, it's generic.
        // PlayerTrackerMinimapElementCollector.playerExists(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // PlayerTrackerMinimapElementCollector.getElements(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // PlayerTrackerMinimapElementCollector.resetRenderedOnRadarFlags(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // PlayerTrackerMinimapElementCollector.confirmPlayerRadarRender(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
    }

    /// `PlayerTrackerMinimapElementReader` (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementReader.java)
    pub mod player_tracker_minimap_element_reader {
        use super::*;
        // The class itself isn't translated: it extends `MinimapElementReader` (xaero.hud.minimap.element.render). Its static parts are below.
        // PlayerTrackerMinimapElementReader.isHidden(2): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.getRenderX(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.getRenderY(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.getRenderZ(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.getCoordinateScale(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.getInteractionBoxLeft(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.getInteractionBoxRight(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.getInteractionBoxTop(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.getInteractionBoxBottom(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.getRenderBoxLeft(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.getRenderBoxRight(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.getRenderBoxTop(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.getRenderBoxBottom(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.getLeftSideLength(2): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.getMenuName(1): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.getFilterName(1): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.getMenuTextFillLeftPadding(1): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.getRightClickTitleBackgroundColor(1): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.shouldScaleBoxWithOptionalScale(0): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.getBoxScale(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementReader.isInteractable(2): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
    }

    /// `PlayerTrackerMinimapElementRenderContext` (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderContext.java)
    pub mod player_tracker_minimap_element_render_context {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `VertexConsumer`. Its static parts are below.
    }

    /// `PlayerTrackerMinimapElementRenderer` (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderer.java)
    pub mod player_tracker_minimap_element_renderer {
        use super::*;
        // The class itself isn't translated: it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render). Its static parts are below.
        // PlayerTrackerMinimapElementRenderer.PlayerTrackerMinimapElementRenderer(6): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementRenderer.getPlayerSkin(2): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementRenderer.preRender(3): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementRenderer.postRender(3): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementRenderer.renderElement(10): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementRenderer.shouldRender(1): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementRenderer.getOrder(0): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // PlayerTrackerMinimapElementRenderer.getCollector(0): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
    }

    /// `PlayerTrackerMinimapElementRenderer.Builder` (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderer.java)
    pub mod player_tracker_minimap_element_renderer_builder {
        use super::*;
        // The class itself isn't translated: it uses IXaeroMinimap, which isn't translated (it's an interface). Its static parts are below.
        // PlayerTrackerMinimapElementRenderer.Builder.Builder(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // PlayerTrackerMinimapElementRenderer.Builder.setDefault(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // PlayerTrackerMinimapElementRenderer.Builder.build(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // PlayerTrackerMinimapElementRenderer.Builder.begin(1): not translated, it uses Builder, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
    }

    /// `PlayerTrackerMinimapElementRenderProvider` (xaero/hud/minimap/player/tracker/PlayerTrackerMinimapElementRenderProvider.java)
    pub mod player_tracker_minimap_element_render_provider {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // PlayerTrackerMinimapElementRenderProvider.PlayerTrackerMinimapElementRenderProvider(1): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMinimapElementRenderProvider.begin(2): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMinimapElementRenderProvider.hasNext(2): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMinimapElementRenderProvider.getNext(2): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMinimapElementRenderProvider.end(2): not translated, its class isn't translated (it's generic).
    }

    /// `ClientSyncedTrackedPlayerManager` (xaero/hud/minimap/player/tracker/synced/ClientSyncedTrackedPlayerManager.java)
    pub mod client_synced_tracked_player_manager {
        use super::*;
        // The class itself isn't translated: it uses `UUID` (java.util). Its static parts are below.
        // ClientSyncedTrackedPlayerManager.ClientSyncedTrackedPlayerManager(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ClientSyncedTrackedPlayerManager.remove(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ClientSyncedTrackedPlayerManager.update(5): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ClientSyncedTrackedPlayerManager.getPlayers(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ClientSyncedTrackedPlayerManager.reset(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
    }

    /// `ClientSyncedTrackedPlayerManager.Builder` (xaero/hud/minimap/player/tracker/synced/ClientSyncedTrackedPlayerManager.java)
    pub mod client_synced_tracked_player_manager_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // ClientSyncedTrackedPlayerManager.Builder.setDefault(0): not translated, `this` used as a value.
            // ClientSyncedTrackedPlayerManager.Builder.build(0): not translated, it uses ClientSyncedTrackedPlayerManager, which isn't translated (it uses `UUID` (java.util)).
        }
        // ClientSyncedTrackedPlayerManager.Builder.begin(0): not translated, it calls Builder.setDefault, which isn't translated.
    }

    /// `SyncedRenderedPlayerTracker` (xaero/hud/minimap/player/tracker/synced/SyncedRenderedPlayerTracker.java)
    pub mod synced_rendered_player_tracker {
        use super::*;
        // The class itself isn't translated: it implements `IRenderedPlayerTracker` (xaero.hud.minimap.player.tracker.system). Its static parts are below.
        // SyncedRenderedPlayerTracker.getReader(0): not translated, its class isn't translated (it implements `IRenderedPlayerTracker` (xaero.hud.minimap.player.tracker.system)).
        // SyncedRenderedPlayerTracker.getTrackedPlayerIterator(0): not translated, its class isn't translated (it implements `IRenderedPlayerTracker` (xaero.hud.minimap.player.tracker.system)).
        // SyncedRenderedPlayerTracker.shouldUseWorldMapTrackedPlayers(1): not translated, its class isn't translated (it implements `IRenderedPlayerTracker` (xaero.hud.minimap.player.tracker.system)).
    }

    /// `SyncedTrackedPlayerReader` (xaero/hud/minimap/player/tracker/synced/SyncedTrackedPlayerReader.java)
    pub mod synced_tracked_player_reader {
        use super::*;
        // The class itself isn't translated: it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system). Its static parts are below.
        // SyncedTrackedPlayerReader.getId(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)).
        // SyncedTrackedPlayerReader.getX(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)).
        // SyncedTrackedPlayerReader.getY(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)).
        // SyncedTrackedPlayerReader.getZ(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)).
        // SyncedTrackedPlayerReader.getDimension(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.hud.minimap.player.tracker.system)).
    }

    /// `IRenderedPlayerTracker` (xaero/hud/minimap/player/tracker/system/IRenderedPlayerTracker.java)
    pub mod i_rendered_player_tracker {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IRenderedPlayerTracker.getReader(0): not translated, it has no body.
        // IRenderedPlayerTracker.getTrackedPlayerIterator(0): not translated, it has no body.
    }

    /// `ITrackedPlayerReader` (xaero/hud/minimap/player/tracker/system/ITrackedPlayerReader.java)
    pub mod i_tracked_player_reader {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ITrackedPlayerReader.getId(1): not translated, it has no body.
        // ITrackedPlayerReader.getX(1): not translated, it has no body.
        // ITrackedPlayerReader.getY(1): not translated, it has no body.
        // ITrackedPlayerReader.getZ(1): not translated, it has no body.
        // ITrackedPlayerReader.getDimension(1): not translated, it has no body.
    }

    /// `RenderedPlayerTrackerManager` (xaero/hud/minimap/player/tracker/system/RenderedPlayerTrackerManager.java)
    pub mod rendered_player_tracker_manager {
        use super::*;
        // The class itself isn't translated: it uses IRenderedPlayerTracker, which isn't translated (it's an interface). Its static parts are below.
        // RenderedPlayerTrackerManager.RenderedPlayerTrackerManager(1): not translated, its class isn't translated (it uses IRenderedPlayerTracker, which isn't translated (it's an interface)).
        // RenderedPlayerTrackerManager.register(2): not translated, its class isn't translated (it uses IRenderedPlayerTracker, which isn't translated (it's an interface)).
        // RenderedPlayerTrackerManager.getAllSystems(0): not translated, its class isn't translated (it uses IRenderedPlayerTracker, which isn't translated (it's an interface)).
    }

    /// `RenderedPlayerTrackerManager.Builder` (xaero/hud/minimap/player/tracker/system/RenderedPlayerTrackerManager.java)
    pub mod rendered_player_tracker_manager_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // RenderedPlayerTrackerManager.Builder.setDefault(0): not translated, `this` used as a value.
            // RenderedPlayerTrackerManager.Builder.build(0): not translated, it uses RenderedPlayerTrackerManager, which isn't translated (it uses IRenderedPlayerTracker, which isn't translated (it's an interface)).
        }
        // RenderedPlayerTrackerManager.Builder.begin(0): not translated, it calls Builder.setDefault, which isn't translated.
    }

    /// `EntityRadarBackwardsCompatibilityConfig` (xaero/hud/minimap/radar/category/EntityRadarBackwardsCompatibilityConfig.java)
    pub mod entity_radar_backwards_compatibility_config {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EntityRadarBackwardsCompatibilityConfig {
            pub entity_amount: i32,
            pub show_players: bool,
            pub show_hostile: bool,
            pub show_mobs: bool,
            pub show_items: bool,
            pub show_other: bool,
            pub show_other_team: bool,
            pub show_tamed: bool,
            pub players_color: i32,
            pub mobs_color: i32,
            pub hostile_color: i32,
            pub items_color: i32,
            pub other_color: i32,
            pub other_team_color: i32,
            pub tamed_mobs_color: i32,
            pub dots_size: i32,
            pub heads_scale: f64,
            pub show_entity_height: bool,
            pub player_icons: i32,
            pub mob_icons: i32,
            pub hostile_icons: i32,
            pub tamed_icons: i32,
            pub height_limit: i32,
            pub player_names: i32,
            pub other_teams_names: i32,
            pub friendly_mob_names: i32,
            pub hostile_mob_names: i32,
            pub item_names: i32,
            pub other_names: i32,
            pub tamed_mob_names: i32,
            pub always_entity_nametags: bool,
            pub item_frames_on_radar: bool,
            pub display_name_when_icon_fails: bool,
        }
        impl EntityRadarBackwardsCompatibilityConfig {
            pub fn new() -> Self {
                let mut s = Self { entity_amount: 10, show_players: true, show_hostile: true, show_mobs: true, show_items: true, show_other: true, show_other_team: true, show_tamed: true, players_color: 15, mobs_color: 14, hostile_color: 14, items_color: 12, other_color: 5, other_team_color: -1, tamed_mobs_color: -1, dots_size: 2, heads_scale: 1.0f64, show_entity_height: true, player_icons: 1, mob_icons: 1, hostile_icons: 1, tamed_icons: 3, height_limit: 20, player_names: 0, other_teams_names: 3, friendly_mob_names: 0, hostile_mob_names: 0, item_names: 0, other_names: 0, tamed_mob_names: 3, always_entity_nametags: false, item_frames_on_radar: false, display_name_when_icon_fails: true };
                s
            }
            // EntityRadarBackwardsCompatibilityConfig.readSetting(1): not translated, it calls `Integer`.parseInt.
        }
    }

    /// `EntityRadarCategory` (xaero/hud/minimap/radar/category/EntityRadarCategory.java)
    pub mod entity_radar_category {
        use super::*;
        // The class itself isn't translated: it extends `FilterObjectCategory` (xaero.hud.category). Its static parts are below.
        // EntityRadarCategory.EntityRadarCategory(12): not translated, its class isn't translated (it extends `FilterObjectCategory` (xaero.hud.category)).
    }

    /// `EntityRadarCategory.Builder` (xaero/hud/minimap/radar/category/EntityRadarCategory.java)
    pub mod entity_radar_category_builder {
        use super::*;
        // The class itself isn't translated: it extends `Builder`. Its static parts are below.
        // EntityRadarCategory.Builder.Builder(0): not translated, its class isn't translated (it extends `Builder`).
        // EntityRadarCategory.Builder.setDefault(0): not translated, its class isn't translated (it extends `Builder`).
        // EntityRadarCategory.Builder.buildUncheckedFilter(5): not translated, its class isn't translated (it extends `Builder`).
        // EntityRadarCategory.Builder.begin(0): not translated, it uses Builder, which isn't translated (it extends `Builder`).
    }

    /// `EntityRadarCategoryConfigIO` (xaero/hud/minimap/radar/category/EntityRadarCategoryConfigIO.java)
    pub mod entity_radar_category_config_io {
        use super::*;
        // The class itself isn't translated: it uses `ConfigOption` (xaero.lib.common.config.option). Its static parts are below.
        // EntityRadarCategoryConfigIO.EntityRadarCategoryConfigIO(2): not translated, its class isn't translated (it uses `ConfigOption` (xaero.lib.common.config.option)).
        // EntityRadarCategoryConfigIO.storeRootCategory(2): not translated, its class isn't translated (it uses `ConfigOption` (xaero.lib.common.config.option)).
        // EntityRadarCategoryConfigIO.loadRootCategory(1): not translated, its class isn't translated (it uses `ConfigOption` (xaero.lib.common.config.option)).
    }

    /// `EntityRadarCategoryConfigIO.Builder` (xaero/hud/minimap/radar/category/EntityRadarCategoryConfigIO.java)
    pub mod entity_radar_category_config_io_builder {
        use super::*;
        // The class itself isn't translated: it uses `ConfigOption` (xaero.lib.common.config.option). Its static parts are below.
        // EntityRadarCategoryConfigIO.Builder.Builder(0): not translated, its class isn't translated (it uses `ConfigOption` (xaero.lib.common.config.option)).
        // EntityRadarCategoryConfigIO.Builder.setDefault(0): not translated, its class isn't translated (it uses `ConfigOption` (xaero.lib.common.config.option)).
        // EntityRadarCategoryConfigIO.Builder.setOption(1): not translated, its class isn't translated (it uses `ConfigOption` (xaero.lib.common.config.option)).
        // EntityRadarCategoryConfigIO.Builder.setSerializationHandler(1): not translated, its class isn't translated (it uses `ConfigOption` (xaero.lib.common.config.option)).
        // EntityRadarCategoryConfigIO.Builder.build(0): not translated, its class isn't translated (it uses `ConfigOption` (xaero.lib.common.config.option)).
        // EntityRadarCategoryConfigIO.Builder.begin(0): not translated, it uses Builder, which isn't translated (it uses `ConfigOption` (xaero.lib.common.config.option)).
    }

    /// `EntityRadarCategoryConstants` (xaero/hud/minimap/radar/category/EntityRadarCategoryConstants.java)
    pub mod entity_radar_category_constants {
        use super::*;
        pub const FORMAT: &str = "\u{a7}";
        // LIST_FACTORY: not translated, it uses ListFactory, which isn't translated (it's an interface).
        // MAP_FACTORY: not translated, it uses MapFactory, which isn't translated (it's an interface).
        // LEGACY_CONFIG_PATH: not translated, it uses Services.PLATFORM, which isn't translated (it uses IPlatformHelper, which isn't translated (it's an interface)).
        // LEGACY_DEFAULT_CONFIG_PATH: not translated, it uses EntityRadarCategoryConstants.LEGACY_CONFIG_PATH, which isn't translated (it uses Services.PLATFORM, which isn't translated (it uses IPlatformHelper, which isn't translated (it's an interface))).
        // DATA_BUILDER_FACTORY: not translated, it uses Builder, which isn't translated (it extends `Builder`).
        // CATEGORY_BUILDER_FACTORY: not translated, it uses Builder, which isn't translated (it extends `Builder`).
        // DEFAULT_LIST_SERIALIZER: not translated, it uses Minecraft's `EntityType`.
        // WILDCARD_TO_REGEX: not translated, a function kept in a variable or field (only parameters can be).
        // NULL_DATA: not translated, it uses EntityRadarCategoryData, which isn't translated (it extends `FilterObjectCategoryData` (xaero.hud.category.serialization.data)).
        // DEFAULT_LIST_WILDCARD_VALIDATOR: not translated, a function kept in a variable or field (only parameters can be).
        // DEFAULT_LIST_STRING_VALIDATOR: not translated, a function kept in a variable or field (only parameters can be).
        // DEFAULT_LIST_STRING_VALIDATOR_FIXER: not translated, a function kept in a variable or field (only parameters can be).
        pub const PLAYER_NAME_LIST_ALLOWED_CHARS: &str = "A-Za-z_0-9\\_";
        // PLAYER_NAME_VALIDATOR: not translated, a function kept in a variable or field (only parameters can be).
        // PLAYER_NAME_VALIDATOR_FIXER: not translated, a function kept in a variable or field (only parameters can be).
        // PREDICATE_VALUE_ALL_SUPPLIER: not translated, a function kept in a variable or field (only parameters can be).
        // PREDICATE_VALUE_RESOLVER: not translated, a function kept in a variable or field (only parameters can be).
        // PREDICATE_VALUE_SERIALIZER: not translated, a function kept in a variable or field (only parameters can be).
        // PREDICATE_VALUE_VALIDATOR: not translated, a function kept in a variable or field (only parameters can be).
        // PREDICATE_VALUE_VALIDATOR_FIXER: not translated, a function kept in a variable or field (only parameters can be).
        pub const CATEGORY_ROOT: &str = "gui.xaero_entity_category_root";
        pub const CATEGORY_LIVING: &str = "gui.xaero_entity_category_living";
        pub const CATEGORY_HOSTILE: &str = "gui.xaero_entity_category_hostile";
        pub const CATEGORY_FRIENDLY: &str = "gui.xaero_entity_category_friendly";
        pub const CATEGORY_HOSTILE_TAMED: &str = "gui.xaero_entity_category_hostile_tamed";
        pub const CATEGORY_FRIENDLY_TAMED: &str = "gui.xaero_entity_category_friendly_tamed";
        pub const CATEGORY_PLAYERS: &str = "gui.xaero_entity_category_players";
        pub const CATEGORY_FRIENDS: &str = "gui.xaero_entity_category_friend";
        pub const CATEGORY_TRACKED: &str = "gui.xaero_entity_category_tracked";
        pub const CATEGORY_SAME_TEAM: &str = "gui.xaero_entity_category_same_team";
        pub const CATEGORY_OTHER_TEAMS: &str = "gui.xaero_entity_category_other_teams";
        pub const CATEGORY_ITEMS: &str = "gui.xaero_entity_category_items";
        pub const CATEGORY_OTHER: &str = "gui.xaero_entity_category_other_entities";
        pub const HARD_NOTHING: &str = "nothing";
        pub const HARD_LIVING: &str = "living";
        pub const HARD_HOSTILE: &str = "hostile";
        pub const HARD_FRIENDLY: &str = "friendly";
        pub const HARD_TAMED: &str = "tamed";
        pub const HARD_PLAYERS: &str = "players";
        pub const HARD_SAME_TEAM: &str = "same-team";
        pub const HARD_OTHER_TEAMS: &str = "other-teams";
        pub const HARD_ITEMS: &str = "items";
        pub const HARD_ANYTHING: &str = "anything";
        pub const HARD_BABY: &str = "baby";
        pub const HARD_VANILLA: &str = "vanilla";
        pub const HARD_MODDED: &str = "modded";
        pub const HARD_ABOVE_GROUND: &str = "above-ground";
        pub const HARD_BELOW_GROUND: &str = "below-ground";
        pub const HARD_MY_GROUND: &str = "my-ground";
        pub const HARD_NOT_MY_GROUND: &str = "not-my-ground";
        pub const HARD_LIT: &str = "block-lit";
        pub const HARD_UNLIT: &str = "block-unlit";
        pub const HARD_CUSTOM_NAME: &str = "has-custom-name";
        pub const HARD_NO_CUSTOM_NAME: &str = "no-custom-name";
        pub const HARD_TRACKED: &str = "tracked";
        pub const HARD_IN_TEAM: &str = "in-a-team";
        pub const HARD_TEAMLESS: &str = "teamless";
        #[derive(Clone, Debug)]
        pub struct EntityRadarCategoryConstants {
        }
        impl EntityRadarCategoryConstants {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EntityRadarCategoryConstants.getDefaultElementResolver(3): not translated, it's generic.
        pub fn contains_wildcard_characters(string: &str) -> bool {
            return ((string.contains("(")) || (string.contains(")")) || (string.contains("|"))) || (string.contains("*"));
        }
        // EntityRadarCategoryConstants.createHardRuleBasedPredicateListRuleType(3): not translated, it uses ObjectCategoryListRuleType, which isn't translated (it's generic).
    }

    /// `EntityRadarCategoryFileIO` (xaero/hud/minimap/radar/category/EntityRadarCategoryFileIO.java)
    pub mod entity_radar_category_file_io {
        use super::*;
        // The class itself isn't translated: it uses EntityRadarCategorySerializationHandler, which isn't translated (it extends `FilterObjectCategorySerializationHandler` (xaero.hud.category.serialization)). Its static parts are below.
        // EntityRadarCategoryFileIO.EntityRadarCategoryFileIO(2): not translated, its class isn't translated (it uses EntityRadarCategorySerializationHandler, which isn't translated (it extends `FilterObjectCategorySerializationHandler` (xaero.hud.category.serialization))).
        // EntityRadarCategoryFileIO.saveRootCategory(1): not translated, its class isn't translated (it uses EntityRadarCategorySerializationHandler, which isn't translated (it extends `FilterObjectCategorySerializationHandler` (xaero.hud.category.serialization))).
        // EntityRadarCategoryFileIO.saveRootCategory(3): not translated, its class isn't translated (it uses EntityRadarCategorySerializationHandler, which isn't translated (it extends `FilterObjectCategorySerializationHandler` (xaero.hud.category.serialization))).
        // EntityRadarCategoryFileIO.loadRootCategory(0): not translated, its class isn't translated (it uses EntityRadarCategorySerializationHandler, which isn't translated (it extends `FilterObjectCategorySerializationHandler` (xaero.hud.category.serialization))).
    }

    /// `EntityRadarCategoryFileIO.Builder` (xaero/hud/minimap/radar/category/EntityRadarCategoryFileIO.java)
    pub mod entity_radar_category_file_io_builder {
        use super::*;
        // The class itself isn't translated: it holds an object of its own class (a linked structure). Its static parts are below.
        // EntityRadarCategoryFileIO.Builder.Builder(1): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EntityRadarCategoryFileIO.Builder.setDefault(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EntityRadarCategoryFileIO.Builder.setSaveLocationPath(1): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EntityRadarCategoryFileIO.Builder.build(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EntityRadarCategoryFileIO.Builder.begin(1): not translated, it uses Builder, which isn't translated (it holds an object of its own class (a linked structure)).
    }

    /// `EntityRadarCategoryManager` (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java)
    pub mod entity_radar_category_manager {
        use super::*;
        // The class itself isn't translated: it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)). Its static parts are below.
        // EntityRadarCategoryManager.EntityRadarCategoryManager(7): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.init(0): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.fetchDefaultClientCategory(0): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.fetchDefaultClientCategory(1): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.fetchDefaultServerCategory(0): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.fetchDefaultServerCategory(1): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.updateFromConfigChange(1): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.getRuleResolver(0): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.getRootCategory(0): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.getSyncedRootCategory(0): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.getEffectiveSyncedRootCategory(0): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.getDefaultCategoryConfigurator(0): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.getSecondaryLegacyFilePath(0): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.loadEditedCategory(2): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.getEditedCategoryConfig(0): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.getEditedCategory(0): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.forgetEditedCategory(0): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.storeEditedCategory(1): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.storeEditedCategory(2): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.getMainIO(0): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.resetRootCategorySettings(1): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.copySettingValue(3): not translated, it's generic.
        // EntityRadarCategoryManager.editedCategoryNeedsSaving(0): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.setEditedCategoryNeedsSaving(1): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
    }

    /// `EntityRadarCategoryManager.Builder` (xaero/hud/minimap/radar/category/EntityRadarCategoryManager.java)
    pub mod entity_radar_category_manager_builder {
        use super::*;
        // The class itself isn't translated: it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)). Its static parts are below.
        // EntityRadarCategoryManager.Builder.Builder(0): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.Builder.setMainLegacyFilePath(1): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.Builder.setSecondaryLegacyFilePath(1): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.Builder.setDefaultClientFilePath(1): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.Builder.setDefaultServerFilePath(1): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.Builder.setSecondaryDefaultFolderPath(1): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.Builder.setSerializers(1): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.Builder.setDefault(0): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.Builder.build(0): not translated, its class isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
        // EntityRadarCategoryManager.Builder.begin(0): not translated, it uses Builder, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))).
    }

    /// `EntityRadarDefaultCategories` (xaero/hud/minimap/radar/category/EntityRadarDefaultCategories.java)
    pub mod entity_radar_default_categories {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EntityRadarDefaultCategories {
            pub for_server: bool,
        }
        impl EntityRadarDefaultCategories {
            pub fn new(for_server: bool) -> Self {
                let mut s = Self { for_server: for_server };
                s.for_server = for_server;
                s
            }
            // EntityRadarDefaultCategories.setupDefault(1): not translated, it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category)).
        }
    }

    /// `EntityRadarDefaultCategories.Builder` (xaero/hud/minimap/radar/category/EntityRadarDefaultCategories.java)
    pub mod entity_radar_default_categories_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
            pub for_server: bool,
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self { for_server: false };
                s
            }
            // EntityRadarDefaultCategories.Builder.setDefault(0): not translated, `this` used as a value.
            // EntityRadarDefaultCategories.Builder.setForServer(1): not translated, `this` used as a value.
            pub fn build(&self) -> entity_radar_default_categories::EntityRadarDefaultCategories {
                return entity_radar_default_categories::EntityRadarDefaultCategories::new(self.for_server);
            }
        }
        // EntityRadarDefaultCategories.Builder.begin(0): not translated, it calls Builder.setDefault, which isn't translated.
    }

    /// `EntityRadarCategoryHardRules` (xaero/hud/minimap/radar/category/rule/EntityRadarCategoryHardRules.java)
    pub mod entity_radar_category_hard_rules {
        use super::*;
        // HARD_RULES_LIST: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // HARD_RULES: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_NOTHING: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_ANYTHING: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_LIVING: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_PLAYER: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_SAME_TEAM: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_HOSTILE: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_TAMED: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_ITEM: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_FRIENDLY: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_OTHER_TEAMS: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_BABY: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_VANILLA: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_MODDED: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_ABOVE_GROUND: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_BELOW_GROUND: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_MY_GROUND: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_NOT_MY_GROUND: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_LIT: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_UNLIT: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // HAS_CUSTOM_NAME: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // NO_CUSTOM_NAME: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_TRACKED: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_IN_TEAM: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        // IS_TEAMLESS: not translated, it uses ObjectCategoryHardRule, which isn't translated (it's generic).
        #[derive(Clone, Debug)]
        pub struct EntityRadarCategoryHardRules {
        }
        impl EntityRadarCategoryHardRules {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EntityRadarListRuleTypes` (xaero/hud/minimap/radar/category/rule/EntityRadarListRuleTypes.java)
    pub mod entity_radar_list_rule_types {
        use super::*;
        // TYPE_LIST: not translated, it uses ObjectCategoryListRuleType, which isn't translated (it's generic).
        // TYPE_MAP: not translated, it uses ObjectCategoryListRuleType, which isn't translated (it's generic).
        // ENTITY_TYPE: not translated, it uses ObjectCategoryListRuleType, which isn't translated (it's generic).
        // ITEM_TYPE: not translated, it uses ObjectCategoryListRuleType, which isn't translated (it's generic).
        // PLAYER_NAME: not translated, it uses ObjectCategoryListRuleType, which isn't translated (it's generic).
        // CUSTOM_NAME: not translated, it uses ObjectCategoryListRuleType, which isn't translated (it's generic).
        // LIVING: not translated, it uses ObjectCategoryListRuleType, which isn't translated (it's generic).
        // HOSTILE: not translated, it uses ObjectCategoryListRuleType, which isn't translated (it's generic).
        // TAMED: not translated, it uses ObjectCategoryListRuleType, which isn't translated (it's generic).
        // SAME_TEAM: not translated, it uses ObjectCategoryListRuleType, which isn't translated (it's generic).
        // BABY: not translated, it uses ObjectCategoryListRuleType, which isn't translated (it's generic).
        // VANILLA: not translated, it uses ObjectCategoryListRuleType, which isn't translated (it's generic).
        // ABOVE_GROUND: not translated, it uses ObjectCategoryListRuleType, which isn't translated (it's generic).
        // MY_GROUND: not translated, it uses ObjectCategoryListRuleType, which isn't translated (it's generic).
        // LIT: not translated, it uses ObjectCategoryListRuleType, which isn't translated (it's generic).
        // HAS_CUSTOM_NAME: not translated, it uses ObjectCategoryListRuleType, which isn't translated (it's generic).
        // IN_TEAM: not translated, it uses ObjectCategoryListRuleType, which isn't translated (it's generic).
        // TRACKED: not translated, it uses ObjectCategoryListRuleType, which isn't translated (it's generic).
        #[derive(Clone, Debug)]
        pub struct EntityRadarListRuleTypes {
        }
        impl EntityRadarListRuleTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EntityRadarCategoryData` (xaero/hud/minimap/radar/category/serialization/data/EntityRadarCategoryData.java)
    pub mod entity_radar_category_data {
        use super::*;
        // The class itself isn't translated: it extends `FilterObjectCategoryData` (xaero.hud.category.serialization.data). Its static parts are below.
        // EntityRadarCategoryData.EntityRadarCategoryData(9): not translated, its class isn't translated (it extends `FilterObjectCategoryData` (xaero.hud.category.serialization.data)).
    }

    /// `EntityRadarCategoryData.Builder` (xaero/hud/minimap/radar/category/serialization/data/EntityRadarCategoryData.java)
    pub mod entity_radar_category_data_builder {
        use super::*;
        // The class itself isn't translated: it extends `Builder`. Its static parts are below.
        // EntityRadarCategoryData.Builder.Builder(0): not translated, its class isn't translated (it extends `Builder`).
        // EntityRadarCategoryData.Builder.buildInternally(1): not translated, its class isn't translated (it extends `Builder`).
        // EntityRadarCategoryData.Builder.begin(0): not translated, it uses Builder, which isn't translated (it extends `Builder`).
    }

    /// `EntityRadarCategorySerializationHandler` (xaero/hud/minimap/radar/category/serialization/EntityRadarCategorySerializationHandler.java)
    pub mod entity_radar_category_serialization_handler {
        use super::*;
        // The class itself isn't translated: it extends `FilterObjectCategorySerializationHandler` (xaero.hud.category.serialization). Its static parts are below.
        // EntityRadarCategorySerializationHandler.EntityRadarCategorySerializationHandler(8): not translated, its class isn't translated (it extends `FilterObjectCategorySerializationHandler` (xaero.hud.category.serialization)).
    }

    /// `EntityRadarCategorySerializationHandler.Builder` (xaero/hud/minimap/radar/category/serialization/EntityRadarCategorySerializationHandler.java)
    pub mod entity_radar_category_serialization_handler_builder {
        use super::*;
        // The class itself isn't translated: it extends `Builder`. Its static parts are below.
        // EntityRadarCategorySerializationHandler.Builder.Builder(1): not translated, its class isn't translated (it extends `Builder`).
        // EntityRadarCategorySerializationHandler.Builder.setDefault(0): not translated, its class isn't translated (it extends `Builder`).
        // EntityRadarCategorySerializationHandler.Builder.buildInternally(0): not translated, its class isn't translated (it extends `Builder`).
        // EntityRadarCategorySerializationHandler.Builder.begin(1): not translated, it uses Builder, which isn't translated (it extends `Builder`).
    }

    /// `EntityRadarCategorySettings` (xaero/hud/minimap/radar/category/setting/EntityRadarCategorySettings.java)
    pub mod entity_radar_category_settings {
        use super::*;
        pub static ENTITY_ICONS_OPTIONS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["gui.xaero_icons_off".to_string(), "gui.xaero_icons_list".to_string(), "gui.xaero_icons_always".to_string(), "-".to_string()]);
        pub static ENTITY_NAMES_OPTIONS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["gui.xaero_names_off".to_string(), "gui.xaero_names_list".to_string(), "gui.xaero_names_always".to_string(), "-".to_string()]);
        pub static RADAR_OVER_MAP_OPTIONS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["gui.xaero_radar_over_map_never".to_string(), "gui.xaero_radar_over_map_list".to_string(), "gui.xaero_radar_over_map_always".to_string(), "-".to_string()]);
        // SETTINGS: not translated, it uses ObjectCategorySetting, which isn't translated (it's generic).
        // SETTINGS_LIST: not translated, it uses ObjectCategorySetting, which isn't translated (it's generic).
        // DISPLAYED: not translated, it uses ObjectCategorySetting, which isn't translated (it's generic).
        // COLOR: not translated, it uses ObjectCategorySetting, which isn't translated (it's generic).
        // ICONS: not translated, it uses ObjectCategorySetting, which isn't translated (it's generic).
        // NAMES: not translated, it uses ObjectCategorySetting, which isn't translated (it's generic).
        // DOT_SIZE: not translated, it uses ObjectCategorySetting, which isn't translated (it's generic).
        // ICON_SCALE: not translated, it uses ObjectCategorySetting, which isn't translated (it's generic).
        // HEIGHT_LIMIT: not translated, it uses ObjectCategorySetting, which isn't translated (it's generic).
        // HEIGHT_FADE: not translated, it uses ObjectCategorySetting, which isn't translated (it's generic).
        // DISPLAY_Y: not translated, it uses ObjectCategorySetting, which isn't translated (it's generic).
        // START_FADING_AT: not translated, it uses ObjectCategorySetting, which isn't translated (it's generic).
        // ENTITY_NUMBER: not translated, it uses ObjectCategorySetting, which isn't translated (it's generic).
        // ALWAYS_NAMETAGS: not translated, it uses ObjectCategorySetting, which isn't translated (it's generic).
        // ICON_NAME_FALLBACK: not translated, it uses ObjectCategorySetting, which isn't translated (it's generic).
        // RENDER_OVER_MINIMAP: not translated, it uses ObjectCategorySetting, which isn't translated (it's generic).
        // RENDER_ORDER: not translated, it uses ObjectCategorySetting, which isn't translated (it's generic).
        #[derive(Clone, Debug)]
        pub struct EntityRadarCategorySettings {
        }
        impl EntityRadarCategorySettings {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EditorEntityRadarCategoryNodeConverter` (xaero/hud/minimap/radar/category/ui/EditorEntityRadarCategoryNodeConverter.java)
    pub mod editor_entity_radar_category_node_converter {
        use super::*;
        // The class itself isn't translated: it extends `EditorFilterCategoryNodeConverter` (xaero.hud.category.ui). Its static parts are below.
        // EditorEntityRadarCategoryNodeConverter.EditorEntityRadarCategoryNodeConverter(6): not translated, its class isn't translated (it extends `EditorFilterCategoryNodeConverter` (xaero.hud.category.ui)).
    }

    /// `EditorEntityRadarCategoryNodeConverter.Builder` (xaero/hud/minimap/radar/category/ui/EditorEntityRadarCategoryNodeConverter.java)
    pub mod editor_entity_radar_category_node_converter_builder {
        use super::*;
        // The class itself isn't translated: it extends `Builder`. Its static parts are below.
        // EditorEntityRadarCategoryNodeConverter.Builder.Builder(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorEntityRadarCategoryNodeConverter.Builder.setDefault(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorEntityRadarCategoryNodeConverter.Builder.buildInternally(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorEntityRadarCategoryNodeConverter.Builder.begin(0): not translated, it uses Builder, which isn't translated (it extends `Builder`).
    }

    /// `EntityRadarCategoryUIConstants` (xaero/hud/minimap/radar/category/ui/EntityRadarCategoryUIConstants.java)
    pub mod entity_radar_category_ui_constants {
        use super::*;
        // SETTING_NODE_BUILDER_FACTORIES: not translated, it uses SettingNodeBuilderFactoryManager, which isn't translated (it uses SettingNodeBuilderFactory, which isn't translated (it's an interface)).
        #[derive(Clone, Debug)]
        pub struct EntityRadarCategoryUIConstants {
        }
        impl EntityRadarCategoryUIConstants {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EditorEntityRadarCategoryNode` (xaero/hud/minimap/radar/category/ui/node/EditorEntityRadarCategoryNode.java)
    pub mod editor_entity_radar_category_node {
        use super::*;
        // The class itself isn't translated: it extends `EditorFilterCategoryNode` (xaero.hud.category.ui.node). Its static parts are below.
        // EditorEntityRadarCategoryNode.EditorEntityRadarCategoryNode(8): not translated, its class isn't translated (it extends `EditorFilterCategoryNode` (xaero.hud.category.ui.node)).
    }

    /// `EditorEntityRadarCategoryNode.Builder` (xaero/hud/minimap/radar/category/ui/node/EditorEntityRadarCategoryNode.java)
    pub mod editor_entity_radar_category_node_builder {
        use super::*;
        // The class itself isn't translated: it extends `Builder`. Its static parts are below.
        // EditorEntityRadarCategoryNode.Builder.Builder(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorEntityRadarCategoryNode.Builder.setDefault(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorEntityRadarCategoryNode.Builder.buildInternally(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorEntityRadarCategoryNode.Builder.begin(0): not translated, it uses Builder, which isn't translated (it extends `Builder`).
    }

    /// `EditorEntityRadarCategorySettingsNode` (xaero/hud/minimap/radar/category/ui/node/EditorEntityRadarCategorySettingsNode.java)
    pub mod editor_entity_radar_category_settings_node {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // EditorEntityRadarCategorySettingsNode.EditorEntityRadarCategorySettingsNode(14): not translated, its class isn't translated (it's generic).
    }

    /// `EditorEntityRadarCategorySettingsNode.Builder` (xaero/hud/minimap/radar/category/ui/node/EditorEntityRadarCategorySettingsNode.java)
    pub mod editor_entity_radar_category_settings_node_builder {
        use super::*;
        // The class itself isn't translated: it extends `Builder`. Its static parts are below.
        // EditorEntityRadarCategorySettingsNode.Builder.Builder(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorEntityRadarCategorySettingsNode.Builder.setDefault(0): not translated, its class isn't translated (it extends `Builder`).
        // EditorEntityRadarCategorySettingsNode.Builder.begin(0): not translated, it uses Builder, which isn't translated (it extends `Builder`).
        // EditorEntityRadarCategorySettingsNode.Builder.buildInternally(2): not translated, its class isn't translated (it extends `Builder`).
    }

    /// `RadarColor` (xaero/hud/minimap/radar/color/RadarColor.java)
    pub mod radar_color {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // RadarColor.RadarColor(3): not translated, its class isn't translated (its constants carry values).
        // RadarColor.getName(0): not translated, its class isn't translated (its constants carry values).
        // RadarColor.getFormat(0): not translated, its class isn't translated (its constants carry values).
        // RadarColor.getHex(0): not translated, its class isn't translated (its constants carry values).
        // RadarColor.fromIndex(1): not translated, it uses RadarColor, which isn't translated (its constants carry values).
        // RadarColor.getRandom(0): not translated, it uses RadarColor, which isn't translated (its constants carry values).
    }

    /// `RadarColorHelper` (xaero/hud/minimap/radar/color/RadarColorHelper.java)
    pub mod radar_color_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct RadarColorHelper {
        }
        impl RadarColorHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // RadarColorHelper.getTeamColor(1): not translated, it uses Minecraft's `Entity`.
            // RadarColorHelper.getEntityColor(8): not translated, it uses Minecraft's `Entity`.
            // RadarColorHelper.getRadarColorHex(3): not translated, it uses Minecraft's `Entity`.
            // RadarColorHelper.getFallbackColor(1): not translated, it uses RadarColor, which isn't translated (its constants carry values).
            // RadarColorHelper.getFallbackColor(2): not translated, it uses RadarColor, which isn't translated (its constants carry values).
            pub fn get_entity_height_fade(&self, off_y: f32, height_limit: i32, start_fading_at: i32) -> f32 {
                let mut level: f32 = (height_limit as f32) - off_y;
                if level < 0.0f32 {
                    level = 0.0f32;
                }
                let mut brightness: f32 = 1.0f32;
                let threshold: i32 = if start_fading_at == 0 { (height_limit * 3) / 4 } else { height_limit - start_fading_at };
                if level <= (threshold as f32) {
                    brightness = 0.25f32 + ((0.5f32 * level) / (threshold as f32));
                }
                return brightness;
            }
        }
    }

    /// `RadarIconArmor` (xaero/hud/minimap/radar/icon/cache/id/armor/RadarIconArmor.java)
    pub mod radar_icon_armor {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Item`. Its static parts are below.
        // RadarIconArmor.RadarIconArmor(3): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // RadarIconArmor.toString(0): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // RadarIconArmor.equals(1): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // RadarIconArmor.hashCode(0): not translated, its class isn't translated (it uses Minecraft's `Item`).
    }

    /// `RadarIconArmorHandler` (xaero/hud/minimap/radar/icon/cache/id/armor/RadarIconArmorHandler.java)
    pub mod radar_icon_armor_handler {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct RadarIconArmorHandler {
        }
        impl RadarIconArmorHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // RadarIconArmorHandler.getArmor(1): not translated, it uses RadarIconArmor, which isn't translated (it uses Minecraft's `Item`).
        }
    }

    /// `RadarIconKey` (xaero/hud/minimap/radar/icon/cache/id/RadarIconKey.java)
    pub mod radar_icon_key {
        use super::*;
        // The class itself isn't translated: it uses `Object`. Its static parts are below.
        // RadarIconKey.RadarIconKey(2): not translated, its class isn't translated (it uses `Object`).
        // RadarIconKey.getVariant(0): not translated, its class isn't translated (it uses `Object`).
        // RadarIconKey.toString(0): not translated, its class isn't translated (it uses `Object`).
        // RadarIconKey.equals(1): not translated, its class isn't translated (it uses `Object`).
        // RadarIconKey.hashCode(0): not translated, its class isn't translated (it uses `Object`).
    }

    /// `CushionVariant` (xaero/hud/minimap/radar/icon/cache/id/variant/CushionVariant.java)
    pub mod cushion_variant {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `DyeColor`. Its static parts are below.
        // CushionVariant.CushionVariant(1): not translated, its class isn't translated (it uses Minecraft's `DyeColor`).
        // CushionVariant.equals(1): not translated, its class isn't translated (it uses Minecraft's `DyeColor`).
        // CushionVariant.hashCode(0): not translated, its class isn't translated (it uses Minecraft's `DyeColor`).
        // CushionVariant.toString(0): not translated, its class isn't translated (it uses Minecraft's `DyeColor`).
    }

    /// `EndermanVariant` (xaero/hud/minimap/radar/icon/cache/id/variant/EndermanVariant.java)
    pub mod enderman_variant {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EndermanVariant {
            pub texture: Option<JIdentifier>,
            pub angry: bool,
        }
        impl EndermanVariant {
            pub fn new(texture: Option<JIdentifier>, angry: bool) -> Self {
                let mut s = Self { texture: texture.clone(), angry: angry };
                s.texture = texture.clone();
                s.angry = angry;
                s
            }
            pub fn to_string(&self) -> String {
                return format!("{}%{}", self.texture.as_ref().map_or_else(|| "null".to_string(), |v| v.to_string()), self.angry);
            }
            // EndermanVariant.equals(1): not translated, it uses `Object`.
            // EndermanVariant.hashCode(0): not translated, it calls `Objects` (java.util).hash.
        }
    }

    /// `HorseVariant` (xaero/hud/minimap/radar/icon/cache/id/variant/HorseVariant.java)
    pub mod horse_variant {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Markings`. Its static parts are below.
        // HORSE_MARKINGS: not translated, it uses Minecraft's `Markings`.
        // HorseVariant.HorseVariant(2): not translated, its class isn't translated (it uses Minecraft's `Markings`).
        // HorseVariant.toString(0): not translated, its class isn't translated (it uses Minecraft's `Markings`).
        // HorseVariant.equals(1): not translated, its class isn't translated (it uses Minecraft's `Markings`).
        // HorseVariant.hashCode(0): not translated, its class isn't translated (it uses Minecraft's `Markings`).
    }

    /// `IronGolemVariant` (xaero/hud/minimap/radar/icon/cache/id/variant/IronGolemVariant.java)
    pub mod iron_golem_variant {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Level`. Its static parts are below.
        // IronGolemVariant.IronGolemVariant(2): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // IronGolemVariant.toString(0): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // IronGolemVariant.equals(1): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // IronGolemVariant.hashCode(0): not translated, its class isn't translated (it uses Minecraft's `Level`).
    }

    /// `LlamaVariant` (xaero/hud/minimap/radar/icon/cache/id/variant/LlamaVariant.java)
    pub mod llama_variant {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Item`. Its static parts are below.
        // LlamaVariant.LlamaVariant(3): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // LlamaVariant.toString(0): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // LlamaVariant.equals(1): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // LlamaVariant.hashCode(0): not translated, its class isn't translated (it uses Minecraft's `Item`).
    }

    /// `RadarIconVariantHandler` (xaero/hud/minimap/radar/icon/cache/id/variant/RadarIconVariantHandler.java)
    pub mod radar_icon_variant_handler {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct RadarIconVariantHandler {
            pub legacy_entity_string_builder: String,
        }
        impl RadarIconVariantHandler {
            pub fn new() -> Self {
                let mut s = Self { legacy_entity_string_builder: String::new() };
                s
            }
            // RadarIconVariantHandler.getEntityVariant(4): not translated, it's generic.
            // RadarIconVariantHandler.getLegacyVariantId(3): not translated, it's generic.
        }
    }

    /// `SaddleVariant` (xaero/hud/minimap/radar/icon/cache/id/variant/SaddleVariant.java)
    pub mod saddle_variant {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SaddleVariant {
            pub texture: Option<JIdentifier>,
            pub saddled: bool,
        }
        impl SaddleVariant {
            pub fn new(texture: Option<JIdentifier>, saddled: bool) -> Self {
                let mut s = Self { texture: texture.clone(), saddled: saddled };
                s.texture = texture.clone();
                s.saddled = saddled;
                s
            }
            pub fn to_string(&self) -> String {
                return format!("{}%{}", self.texture.as_ref().map_or_else(|| "null".to_string(), |v| v.to_string()), self.saddled);
            }
            // SaddleVariant.equals(1): not translated, it uses `Object`.
            // SaddleVariant.hashCode(0): not translated, it calls `Objects` (java.util).hash.
        }
    }

    /// `TamableVariant` (xaero/hud/minimap/radar/icon/cache/id/variant/TamableVariant.java)
    pub mod tamable_variant {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TamableVariant {
            pub texture: Option<JIdentifier>,
            pub tame: bool,
        }
        impl TamableVariant {
            pub fn new(texture: Option<JIdentifier>, tame: bool) -> Self {
                let mut s = Self { texture: texture.clone(), tame: tame };
                s.texture = texture.clone();
                s.tame = tame;
                s
            }
            pub fn to_string(&self) -> String {
                return format!("{}%{}", self.texture.as_ref().map_or_else(|| "null".to_string(), |v| v.to_string()), self.tame);
            }
            // TamableVariant.equals(1): not translated, it uses `Object`.
            // TamableVariant.hashCode(0): not translated, it calls `Objects` (java.util).hash.
        }
    }

    /// `TropicalFishVariant` (xaero/hud/minimap/radar/icon/cache/id/variant/TropicalFishVariant.java)
    pub mod tropical_fish_variant {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `DyeColor`. Its static parts are below.
        // TropicalFishVariant.TropicalFishVariant(4): not translated, its class isn't translated (it uses Minecraft's `DyeColor`).
        // TropicalFishVariant.toString(0): not translated, its class isn't translated (it uses Minecraft's `DyeColor`).
        // TropicalFishVariant.equals(1): not translated, its class isn't translated (it uses Minecraft's `DyeColor`).
        // TropicalFishVariant.hashCode(0): not translated, its class isn't translated (it uses Minecraft's `DyeColor`).
    }

    /// `VillagerVariant` (xaero/hud/minimap/radar/icon/cache/id/variant/VillagerVariant.java)
    pub mod villager_variant {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct VillagerVariant {
            pub texture: Option<JIdentifier>,
            pub baby: bool,
            pub r#type: JIdentifier,
            pub profession: JIdentifier,
            pub level: i32,
        }
        impl VillagerVariant {
            pub fn new(texture: Option<JIdentifier>, baby: bool, r#type: &JIdentifier, profession: &JIdentifier, level: i32) -> Self {
                let mut s = Self { texture: texture.clone(), baby: baby, r#type: (r#type).clone(), profession: profession.clone(), level: level };
                s.texture = texture.clone();
                s.baby = baby;
                s.r#type = (r#type).clone();
                s.profession = profession.clone();
                s.level = level;
                s
            }
            pub fn to_string(&self) -> String {
                return format!("{}%{}%{}%{}%{}", self.texture.as_ref().map_or_else(|| "null".to_string(), |v| v.to_string()), self.baby, self.r#type, self.profession, self.level);
            }
            // VillagerVariant.equals(1): not translated, it uses `Object`.
            // VillagerVariant.hashCode(0): not translated, it calls `Objects` (java.util).hash.
        }
    }

    /// `RadarIconCache` (xaero/hud/minimap/radar/icon/cache/RadarIconCache.java)
    pub mod radar_icon_cache {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `EntityType`. Its static parts are below.
        // RadarIconCache.getEntityCache(1): not translated, its class isn't translated (it uses Minecraft's `EntityType`).
        // RadarIconCache.clear(0): not translated, its class isn't translated (it uses Minecraft's `EntityType`).
    }

    /// `RadarIconEntityCache` (xaero/hud/minimap/radar/icon/cache/RadarIconEntityCache.java)
    pub mod radar_icon_entity_cache {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `EntityType`. Its static parts are below.
        // RadarIconEntityCache.RadarIconEntityCache(1): not translated, its class isn't translated (it uses Minecraft's `EntityType`).
        // RadarIconEntityCache.get(1): not translated, its class isn't translated (it uses Minecraft's `EntityType`).
        // RadarIconEntityCache.add(2): not translated, its class isn't translated (it uses Minecraft's `EntityType`).
        // RadarIconEntityCache.getVariantString(1): not translated, its class isn't translated (it uses Minecraft's `EntityType`).
        // RadarIconEntityCache.isInvalidVariantClass(0): not translated, its class isn't translated (it uses Minecraft's `EntityType`).
    }

    /// `LivingEntityPoseResetter` (xaero/hud/minimap/radar/icon/creator/entity/LivingEntityPoseResetter.java)
    pub mod living_entity_pose_resetter {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LivingEntityPoseResetter {
        }
        impl LivingEntityPoseResetter {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // LivingEntityPoseResetter.resetValues(1): not translated, it uses Minecraft's `LivingEntityRenderState`.
        }
    }

    /// `RadarIconCreator` (xaero/hud/minimap/radar/icon/creator/RadarIconCreator.java)
    pub mod radar_icon_creator {
        use super::*;
        // The class itself isn't translated: it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`). Its static parts are below.
        pub const PREFERRED_ATLAS_WIDTH: i32 = 1024;
        pub const ICON_WIDTH: i32 = 64;
        pub const FAR_PLANE: i32 = 500;
        // RadarIconCreator.RadarIconCreator(0): not translated, its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)).
        // RadarIconCreator.create(6): not translated, it's generic.
        // RadarIconCreator.getFinalIcon(4): not translated, its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)).
        // RadarIconCreator.renderOutline(1): not translated, its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)).
        // RadarIconCreator.setupMatrices(3): not translated, its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)).
        // RadarIconCreator.restoreMatrices(1): not translated, its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)).
        // RadarIconCreator.endFormRendering(0): not translated, its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)).
        // RadarIconCreator.bindDefaultFramebuffer(1): not translated, its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)).
        // RadarIconCreator.reset(0): not translated, its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)).
        // RadarIconCreator.getRenderTracer(0): not translated, its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)).
        // RadarIconCreator.getCurrentAtlas(0): not translated, its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)).
        // RadarIconCreator.initFramebuffers(1): not translated, its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)).
    }

    /// `RadarIconCreator.Parameters` (xaero/hud/minimap/radar/icon/creator/RadarIconCreator.java)
    pub mod radar_icon_creator_parameters {
        use super::*;
        // The class itself isn't translated: it uses `Object`. Its static parts are below.
        // RadarIconCreator.Parameters.Parameters(5): not translated, its class isn't translated (it uses `Object`).
    }

    /// `IRadarIconFormPrerenderer` (xaero/hud/minimap/radar/icon/creator/render/form/IRadarIconFormPrerenderer.java)
    pub mod i_radar_icon_form_prerenderer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IRadarIconFormPrerenderer.requiresEntityModel(0): not translated, it has no body.
        // IRadarIconFormPrerenderer.isFlipped(0): not translated, it has no body.
        // IRadarIconFormPrerenderer.isOutlined(0): not translated, it has no body.
        // IRadarIconFormPrerenderer.prerender(8): not translated, it has no body.
    }

    /// `RadarIconItemFormPrerenderer` (xaero/hud/minimap/radar/icon/creator/render/form/item/RadarIconItemFormPrerenderer.java)
    pub mod radar_icon_item_form_prerenderer {
        use super::*;
        // The class itself isn't translated: it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form). Its static parts are below.
        // RadarIconItemFormPrerenderer.requiresEntityModel(0): not translated, its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)).
        // RadarIconItemFormPrerenderer.isFlipped(0): not translated, its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)).
        // RadarIconItemFormPrerenderer.isOutlined(0): not translated, its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)).
        // RadarIconItemFormPrerenderer.prerender(8): not translated, it's generic.
        // RadarIconItemFormPrerenderer.getItemToRender(2): not translated, its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)).
        // RadarIconItemFormPrerenderer.getSelfItem(1): not translated, its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)).
    }

    /// `CustomModelIconCustomPrenderer` (xaero/hud/minimap/radar/icon/creator/render/form/model/custom/CustomModelIconCustomPrenderer.java)
    pub mod custom_model_icon_custom_prenderer {
        use super::*;
        // The class itself isn't translated: it extends `RenderTypeIconCustomPrerenderer`. Its static parts are below.
        // CustomModelIconCustomPrenderer.getRenderedDest(1): not translated, its class isn't translated (it extends `RenderTypeIconCustomPrerenderer`).
        // CustomModelIconCustomPrenderer.getModelParts(4): not translated, it has no body.
    }

    /// `LayeredIconCustomPrerenderer` (xaero/hud/minimap/radar/icon/creator/render/form/model/custom/LayeredIconCustomPrerenderer.java)
    pub mod layered_icon_custom_prerenderer {
        use super::*;
        // The class itself isn't translated: it extends `RadarIconCustomPrerenderer`. Its static parts are below.
        // LayeredIconCustomPrerenderer.LayeredIconCustomPrerenderer(1): not translated, its class isn't translated (it extends `RadarIconCustomPrerenderer`).
        // LayeredIconCustomPrerenderer.render(11): not translated, it's generic.
    }

    /// `RadarIconCustomPrerenderer` (xaero/hud/minimap/radar/icon/creator/render/form/model/custom/RadarIconCustomPrerenderer.java)
    pub mod radar_icon_custom_prerenderer {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct RadarIconCustomPrerenderer {
        }
        impl RadarIconCustomPrerenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // RadarIconCustomPrerenderer.render(11): not translated, it has no body.
        }
    }

    /// `RenderTypeIconCustomPrerenderer` (xaero/hud/minimap/radar/icon/creator/render/form/model/custom/RenderTypeIconCustomPrerenderer.java)
    pub mod render_type_icon_custom_prerenderer {
        use super::*;
        // The class itself isn't translated: it extends `RadarIconCustomPrerenderer`. Its static parts are below.
        // RenderTypeIconCustomPrerenderer.render(11): not translated, it's generic.
        // RenderTypeIconCustomPrerenderer.getModelParts(4): not translated, it's generic.
        // RenderTypeIconCustomPrerenderer.getRenderedDest(1): not translated, its class isn't translated (it extends `RadarIconCustomPrerenderer`).
        // RenderTypeIconCustomPrerenderer.getRenderType(2): not translated, it has no body.
    }

    /// `ModelPartUtil` (xaero/hud/minimap/radar/icon/creator/render/form/model/part/ModelPartUtil.java)
    pub mod model_part_util {
        use super::*;
        // CUBES_FIELD: not translated, it uses `Field` (java.lang.reflect).
        // CHILDREN_FIELD: not translated, it uses `Field` (java.lang.reflect).
        #[derive(Clone, Debug)]
        pub struct ModelPartUtil {
        }
        impl ModelPartUtil {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModelPartUtil.getCubes(1): not translated, it uses Minecraft's `Cube`.
        // ModelPartUtil.getChildren(1): not translated, it uses Minecraft's `ModelPart`.
        // ModelPartUtil.hasDirectCubes(1): not translated, it uses Minecraft's `ModelPart`.
        // ModelPartUtil.hasCubes(1): not translated, it uses Minecraft's `ModelPart`.
        // ModelPartUtil.getBiggestCuboid(1): not translated, it uses Minecraft's `Cube`.
    }

    /// `RadarIconModelPartPrerenderer` (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java)
    pub mod radar_icon_model_part_prerenderer {
        use super::*;
        // The class itself isn't translated: it uses `Field` (java.lang.reflect). Its static parts are below.
        // RadarIconModelPartPrerenderer.renderPart(5): not translated, its class isn't translated (it uses `Field` (java.lang.reflect)).
        // RadarIconModelPartPrerenderer.testRenderEngineWrapperCompatibility(3): not translated, its class isn't translated (it uses `Field` (java.lang.reflect)).
        // RadarIconModelPartPrerenderer.renderDeclaredMethod(6): not translated, it's generic.
        // RadarIconModelPartPrerenderer.renderPartsIterable(5): not translated, its class isn't translated (it uses `Field` (java.lang.reflect)).
    }

    /// `RadarIconModelPartPrerenderer.Parameters` (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java)
    pub mod radar_icon_model_part_prerenderer_parameters {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ModelPart`. Its static parts are below.
        // RadarIconModelPartPrerenderer.Parameters.Parameters(3): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
    }

    /// `RadarIconModelPartPrerenderer.VertexConsumerWrapper` (xaero/hud/minimap/radar/icon/creator/render/form/model/part/RadarIconModelPartPrerenderer.java)
    pub mod radar_icon_model_part_prerenderer_vertex_consumer_wrapper {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `VertexConsumer`. Its static parts are below.
        // RadarIconModelPartPrerenderer.VertexConsumerWrapper.prepareDetection(7): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // RadarIconModelPartPrerenderer.VertexConsumerWrapper.addVertex(3): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // RadarIconModelPartPrerenderer.VertexConsumerWrapper.setColor(4): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // RadarIconModelPartPrerenderer.VertexConsumerWrapper.setColor(1): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // RadarIconModelPartPrerenderer.VertexConsumerWrapper.setUv(2): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // RadarIconModelPartPrerenderer.VertexConsumerWrapper.setUv1(2): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // RadarIconModelPartPrerenderer.VertexConsumerWrapper.setOverlay(1): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // RadarIconModelPartPrerenderer.VertexConsumerWrapper.setUv2(2): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // RadarIconModelPartPrerenderer.VertexConsumerWrapper.setUv3(2): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // RadarIconModelPartPrerenderer.VertexConsumerWrapper.setNormal(3): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // RadarIconModelPartPrerenderer.VertexConsumerWrapper.setLineWidth(1): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // RadarIconModelPartPrerenderer.VertexConsumerWrapper.hasDetectedVertex(0): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
    }

    /// `ResolvedFieldModelPartRenderer` (xaero/hud/minimap/radar/icon/creator/render/form/model/part/ResolvedFieldModelPartRenderer.java)
    pub mod resolved_field_model_part_renderer {
        use super::*;
        // The class itself isn't translated: it implements `Listener`. Its static parts are below.
        // ResolvedFieldModelPartRenderer.prepare(6): not translated, its class isn't translated (it implements `Listener`).
        // ResolvedFieldModelPartRenderer.isFieldAllowed(1): not translated, its class isn't translated (it implements `Listener`).
        // ResolvedFieldModelPartRenderer.shouldStop(0): not translated, its class isn't translated (it implements `Listener`).
        // ResolvedFieldModelPartRenderer.onFieldResolved(2): not translated, its class isn't translated (it implements `Listener`).
        // ResolvedFieldModelPartRenderer.getMainPart(0): not translated, its class isn't translated (it implements `Listener`).
    }

    /// `RadarIconModelFormPrerenderer` (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelFormPrerenderer.java)
    pub mod radar_icon_model_form_prerenderer {
        use super::*;
        // The class itself isn't translated: it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form). Its static parts are below.
        // RadarIconModelFormPrerenderer.requiresEntityModel(0): not translated, its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)).
        // RadarIconModelFormPrerenderer.isFlipped(0): not translated, its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)).
        // RadarIconModelFormPrerenderer.isOutlined(0): not translated, its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)).
        // RadarIconModelFormPrerenderer.prerender(8): not translated, it's generic.
        // RadarIconModelFormPrerenderer.renderLayers(9): not translated, it's generic.
        // RadarIconModelFormPrerenderer.addDefaultLayer(5): not translated, it's generic.
        // RadarIconModelFormPrerenderer.renderLayer(8): not translated, it's generic.
        // RadarIconModelFormPrerenderer.resetModelRotations(2): not translated, it's generic.
    }

    /// `RadarIconModelPrerenderer` (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelPrerenderer.java)
    pub mod radar_icon_model_prerenderer {
        use super::*;
        // The class itself isn't translated: it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect)). Its static parts are below.
        pub static EXTRA_PART_NAMES: LazyLock<JCell<Vec<String>>> = LazyLock::new(|| JCell::new(vec!["beak".to_string(), "nose".to_string(), "jaw".to_string(), "left_ear".to_string(), "right_ear".to_string(), "mane".to_string(), "mouth".to_string(), "eyes".to_string(), "tongue".to_string(), "hat".to_string()]));
        // ONE_RENDERER_ARRAY: not translated, it uses `Object`.
        // ONE_OBJECT_ARRAY: not translated, it uses `Object`.
        pub static LIGHTMAP: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("xaerominimap", "entity_icon_model_lightmap"));
        // RadarIconModelPrerenderer.RadarIconModelPrerenderer(0): not translated, its class isn't translated (it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect))).
        // RadarIconModelPrerenderer.renderModel(7): not translated, its class isn't translated (it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect))).
        // RadarIconModelPrerenderer.renderAgeableListModel(4): not translated, its class isn't translated (it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect))).
        // RadarIconModelPrerenderer.renderHierarchicalModel(5): not translated, its class isn't translated (it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect))).
        // RadarIconModelPrerenderer.renderPart(4): not translated, its class isn't translated (it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect))).
        // RadarIconModelPrerenderer.searchAndRenderFields(6): not translated, its class isn't translated (it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect))).
        // RadarIconModelPrerenderer.getLayerModelVertexConsumer(4): not translated, its class isn't translated (it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect))).
        // RadarIconModelPrerenderer.resolveModelRoot(3): not translated, its class isn't translated (it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect))).
        // RadarIconModelPrerenderer.getPartPrerenderer(0): not translated, its class isn't translated (it uses RadarIconModelPartPrerenderer, which isn't translated (it uses `Field` (java.lang.reflect))).
    }

    /// `RadarIconModelPrerenderer.Parameters` (xaero/hud/minimap/radar/icon/creator/render/form/model/RadarIconModelPrerenderer.java)
    pub mod radar_icon_model_prerenderer_parameters {
        use super::*;
        // The class itself isn't translated: it extends `Parameters`. Its static parts are below.
        // RadarIconModelPrerenderer.Parameters.Parameters(10): not translated, its class isn't translated (it extends `Parameters`).
    }

    /// `RadarIconModelFieldResolver` (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java)
    pub mod radar_icon_model_field_resolver {
        use super::*;
        // KEYS_PARSER: not translated, it uses SeparatedKeysParser, which isn't translated (a function kept in a variable or field (only parameters can be)).
        #[derive(Clone, Debug)]
        pub struct RadarIconModelFieldResolver {
        }
        impl RadarIconModelFieldResolver {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RadarIconModelFieldResolver.handleDeclaredField(4): not translated, it uses `Object`.
        // RadarIconModelFieldResolver.searchSuperclassFields(4): not translated, it uses `Object`.
        // RadarIconModelFieldResolver.handleFields(5): not translated, it uses `Object`.
        // RadarIconModelFieldResolver.passesFilter(2): not translated, String.lastIndexOf with 1 values.
        // RadarIconModelFieldResolver.getReferenceType(1): not translated, it uses FieldReferenceType, which isn't translated (it's generic).
    }

    /// `RadarIconModelFieldResolver.FieldReferenceElementGetter` (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java)
    pub mod radar_icon_model_field_resolver_field_reference_element_getter {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // RadarIconModelFieldResolver.FieldReferenceElementGetter.get(3): not translated, it has no body.
    }

    /// `RadarIconModelFieldResolver.FieldReferenceType` (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java)
    pub mod radar_icon_model_field_resolver_field_reference_type {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // SINGLE: not translated, it uses FieldReferenceType, which isn't translated (it's generic).
        // ARRAY: not translated, it uses FieldReferenceType, which isn't translated (it's generic).
        // COLLECTION: not translated, it uses FieldReferenceType, which isn't translated (it's generic).
        // MAP: not translated, it uses FieldReferenceType, which isn't translated (it's generic).
        // RadarIconModelFieldResolver.FieldReferenceType.FieldReferenceType(2): not translated, its class isn't translated (it's generic).
        // RadarIconModelFieldResolver.FieldReferenceType.getArray(2): not translated, its class isn't translated (it's generic).
        // RadarIconModelFieldResolver.FieldReferenceType.getElement(3): not translated, its class isn't translated (it's generic).
    }

    /// `RadarIconModelFieldResolver.Listener` (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/RadarIconModelFieldResolver.java)
    pub mod radar_icon_model_field_resolver_listener {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // RadarIconModelFieldResolver.Listener.isFieldAllowed(1): not translated, it has no body.
        // RadarIconModelFieldResolver.Listener.shouldStop(0): not translated, it has no body.
        // RadarIconModelFieldResolver.Listener.onFieldResolved(2): not translated, it has no body.
    }

    /// `ResolvedFieldModelRootPathListener` (xaero/hud/minimap/radar/icon/creator/render/form/model/resolver/ResolvedFieldModelRootPathListener.java)
    pub mod resolved_field_model_root_path_listener {
        use super::*;
        // The class itself isn't translated: it implements `Listener`. Its static parts are below.
        // ResolvedFieldModelRootPathListener.prepare(0): not translated, its class isn't translated (it implements `Listener`).
        // ResolvedFieldModelRootPathListener.isFieldAllowed(1): not translated, its class isn't translated (it implements `Listener`).
        // ResolvedFieldModelRootPathListener.shouldStop(0): not translated, its class isn't translated (it implements `Listener`).
        // ResolvedFieldModelRootPathListener.onFieldResolved(2): not translated, its class isn't translated (it implements `Listener`).
        // ResolvedFieldModelRootPathListener.getCurrentNode(0): not translated, its class isn't translated (it implements `Listener`).
        // ResolvedFieldModelRootPathListener.failed(0): not translated, its class isn't translated (it implements `Listener`).
    }

    /// `RadarIconSpriteFormPrerenderer` (xaero/hud/minimap/radar/icon/creator/render/form/sprite/RadarIconSpriteFormPrerenderer.java)
    pub mod radar_icon_sprite_form_prerenderer {
        use super::*;
        // The class itself isn't translated: it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form). Its static parts are below.
        // RadarIconSpriteFormPrerenderer.RadarIconSpriteFormPrerenderer(2): not translated, its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)).
        // RadarIconSpriteFormPrerenderer.requiresEntityModel(0): not translated, its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)).
        // RadarIconSpriteFormPrerenderer.isFlipped(0): not translated, its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)).
        // RadarIconSpriteFormPrerenderer.isOutlined(0): not translated, its class isn't translated (it implements `IRadarIconFormPrerenderer` (xaero.hud.minimap.radar.icon.creator.render.form)).
        // RadarIconSpriteFormPrerenderer.prerender(8): not translated, it's generic.
    }

    /// `EntityRenderTracer` (xaero/hud/minimap/radar/icon/creator/render/trace/EntityRenderTracer.java)
    pub mod entity_render_tracer {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Entity`. Its static parts are below.
        pub static TRACING_MODEL_RENDERS: JCell<bool> = JCell::new(false);
        // EntityRenderTracer.EntityRenderTracer(0): not translated, its class isn't translated (it uses Minecraft's `Entity`).
        // EntityRenderTracer.trace(5): not translated, it's generic.
        // EntityRenderTracer.onModelPrepare(1): not translated, its class isn't translated (it uses Minecraft's `Entity`).
        // EntityRenderTracer.onModelRender(3): not translated, its class isn't translated (it uses Minecraft's `Entity`).
        // EntityRenderTracer.fixAdditiveBlendUncached(1): not translated, its class isn't translated (it uses Minecraft's `Entity`).
        // EntityRenderTracer.fixAdditiveBlendUncached(3): not translated, its class isn't translated (it uses Minecraft's `Entity`).
        // EntityRenderTracer.onModelPartRender(2): not translated, its class isn't translated (it uses Minecraft's `Entity`).
        // EntityRenderTracer.getRenderState(1): not translated, its class isn't translated (it uses Minecraft's `Entity`).
        // EntityRenderTracer.getEntityRendererModel(1): not translated, it's generic.
        // EntityRenderTracer.initReflection(0): not translated, its class isn't translated (it uses Minecraft's `Entity`).
        // EntityRenderTracer.reset(0): not translated, its class isn't translated (it uses Minecraft's `Entity`).
    }

    /// `ModelPartRenderTrace` (xaero/hud/minimap/radar/icon/creator/render/trace/ModelPartRenderTrace.java)
    pub mod model_part_render_trace {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ModelPart`. Its static parts are below.
        // ModelPartRenderTrace.ModelPartRenderTrace(2): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
    }

    /// `ModelRenderTrace` (xaero/hud/minimap/radar/icon/creator/render/trace/ModelRenderTrace.java)
    pub mod model_render_trace {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Model`. Its static parts are below.
        // ModelRenderTrace.ModelRenderTrace(11): not translated, its class isn't translated (it uses Minecraft's `Model`).
        // ModelRenderTrace.toString(0): not translated, its class isn't translated (it uses Minecraft's `Model`).
        // ModelRenderTrace.addVisibleModelPart(2): not translated, its class isn't translated (it uses Minecraft's `Model`).
        // ModelRenderTrace.getModelPartRenderInfo(1): not translated, its class isn't translated (it uses Minecraft's `Model`).
        // ModelRenderTrace.isEmpty(0): not translated, its class isn't translated (it uses Minecraft's `Model`).
        // ModelRenderTrace.sameVisibility(1): not translated, its class isn't translated (it uses Minecraft's `Model`).
    }

    /// `BuiltInRadarIconDefinitions` (xaero/hud/minimap/radar/icon/definition/BuiltInRadarIconDefinitions.java)
    pub mod built_in_radar_icon_definitions {
        use super::*;
        // BUILD_VARIANT_ID_STRING_METHOD: not translated, it uses `Method` (java.lang.reflect).
        // GET_VARIANT_ID_STRING_METHOD: not translated, it uses `Method` (java.lang.reflect).
        #[derive(Clone, Debug)]
        pub struct BuiltInRadarIconDefinitions {
        }
        impl BuiltInRadarIconDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BuiltInRadarIconDefinitions.getMainModelPartFields(3): not translated, it uses Minecraft's `EntityRenderer`.
        // BuiltInRadarIconDefinitions.getSecondaryModelPartsFields(3): not translated, it uses Minecraft's `EntityRenderer`.
        // BuiltInRadarIconDefinitions.getModelRoot(1): not translated, it uses `Object`.
        // BuiltInRadarIconDefinitions.forceFieldCheck(1): not translated, it uses Minecraft's `EntityModel`.
        // BuiltInRadarIconDefinitions.prepareEntityRenderState(1): not translated, it uses Minecraft's `EntityRenderState`.
        // BuiltInRadarIconDefinitions.defaultTransformation(2): not translated, it uses Minecraft's `EntityModel`.
        // BuiltInRadarIconDefinitions.defaultPostIconModelRender(2): not translated, it uses Minecraft's `EntityModel`.
        // BuiltInRadarIconDefinitions.fullModelIcon(1): not translated, it uses Minecraft's `EntityModel`.
        // BuiltInRadarIconDefinitions.getCustomLayer(2): not translated, it's generic.
        // BuiltInRadarIconDefinitions.getVariant(3): not translated, it's generic.
    }

    /// `RadarIconItemForm` (xaero/hud/minimap/radar/icon/definition/form/item/RadarIconItemForm.java)
    pub mod radar_icon_item_form {
        use super::*;
        // The class itself isn't translated: it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form). Its static parts are below.
        // RadarIconItemForm.RadarIconItemForm(2): not translated, its class isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)).
        // RadarIconItemForm.getItemKey(0): not translated, its class isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)).
        // RadarIconItemForm.read(3): not translated, it uses RadarIconItemForm, which isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)).
    }

    /// `RadarIconModelConfig` (xaero/hud/minimap/radar/icon/definition/form/model/config/RadarIconModelConfig.java)
    pub mod radar_icon_model_config {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct RadarIconModelConfig {
            pub base_scale: f32,
            pub rotation_y: f32,
            pub rotation_x: f32,
            pub rotation_z: f32,
            pub offset_x: f32,
            pub offset_y: f32,
            pub model_parts_rotation_reset: bool,
            pub rendering_full_model: Option<bool>,
            pub model_main_part_field_aliases: Option<Vec<String>>,
            pub model_parts_fields: Option<Vec<String>>,
            pub model_root_path: Option<Vec<Vec<String>>>,
            pub layers_allowed: bool,
        }
        impl RadarIconModelConfig {
            pub fn new() -> Self {
                let mut s = Self { base_scale: 1.0f32, rotation_y: 0.0f32, rotation_x: 0.0f32, rotation_z: 0.0f32, offset_x: 0.0f32, offset_y: 0.0f32, model_parts_rotation_reset: true, rendering_full_model: None, model_main_part_field_aliases: None, model_parts_fields: None, model_root_path: None, layers_allowed: true };
                s
            }
        }
    }

    /// `RadarIconModelForm` (xaero/hud/minimap/radar/icon/definition/form/model/RadarIconModelForm.java)
    pub mod radar_icon_model_form {
        use super::*;
        // The class itself isn't translated: it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form). Its static parts are below.
        // RadarIconModelForm.RadarIconModelForm(2): not translated, its class isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)).
        // RadarIconModelForm.getConfig(0): not translated, its class isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)).
        // RadarIconModelForm.read(3): not translated, it uses RadarIconModelForm, which isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)).
    }

    /// `RadarIconBasicForms` (xaero/hud/minimap/radar/icon/definition/form/RadarIconBasicForms.java)
    pub mod radar_icon_basic_forms {
        use super::*;
        // DEFAULT_MODEL: not translated, it uses RadarIconModelForm, which isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)).
        // SELF_ITEM: not translated, it uses RadarIconItemForm, which isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)).
        // DOT: not translated, it uses RadarIconForm, which isn't translated (it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface))).
        #[derive(Clone, Debug)]
        pub struct RadarIconBasicForms {
        }
        impl RadarIconBasicForms {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `RadarIconForm` (xaero/hud/minimap/radar/icon/definition/form/RadarIconForm.java)
    pub mod radar_icon_form {
        use super::*;
        // The class itself isn't translated: it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)). Its static parts are below.
        // RadarIconForm.RadarIconForm(1): not translated, its class isn't translated (it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface))).
        // RadarIconForm.getType(0): not translated, its class isn't translated (it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface))).
        // RadarIconForm.getPrerenderer(0): not translated, it uses IRadarIconFormPrerenderer, which isn't translated (it's an interface).
        // RadarIconForm.getFailureResult(0): not translated, its class isn't translated (it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface))).
    }

    /// `RadarIconSpriteForm` (xaero/hud/minimap/radar/icon/definition/form/sprite/RadarIconSpriteForm.java)
    pub mod radar_icon_sprite_form {
        use super::*;
        // The class itself isn't translated: it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form). Its static parts are below.
        // RadarIconSpriteForm.RadarIconSpriteForm(2): not translated, its class isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)).
        // RadarIconSpriteForm.getSpriteLocation(0): not translated, its class isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)).
        // RadarIconSpriteForm.read(3): not translated, it uses RadarIconSpriteForm, which isn't translated (it extends `RadarIconForm` (xaero.hud.minimap.radar.icon.definition.form)).
    }

    /// `IRadarIconFormReader` (xaero/hud/minimap/radar/icon/definition/form/type/IRadarIconFormReader.java)
    pub mod i_radar_icon_form_reader {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IRadarIconFormReader.read(3): not translated, it has no body.
    }

    /// `RadarIconFormType` (xaero/hud/minimap/radar/icon/definition/form/type/RadarIconFormType.java)
    pub mod radar_icon_form_type {
        use super::*;
        // The class itself isn't translated: it uses IRadarIconFormReader, which isn't translated (it's an interface). Its static parts are below.
        // RadarIconFormType.RadarIconFormType(4): not translated, its class isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)).
        // RadarIconFormType.getId(0): not translated, its class isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)).
        // RadarIconFormType.readForm(2): not translated, its class isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)).
        // RadarIconFormType.addTo(1): not translated, its class isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)).
        // RadarIconFormType.getPrerenderer(0): not translated, its class isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)).
        // RadarIconFormType.getFailureResult(0): not translated, its class isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)).
    }

    /// `RadarIconFormTypes` (xaero/hud/minimap/radar/icon/definition/form/type/RadarIconFormTypes.java)
    pub mod radar_icon_form_types {
        use super::*;
        // ALL: not translated, it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)).
        // MODEL: not translated, it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)).
        // NORMAL_SPRITE: not translated, it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)).
        // OUTLINED_SPRITE: not translated, it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)).
        // OLD_SPRITE: not translated, it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)).
        // ITEM: not translated, it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)).
        // DOT: not translated, it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)).
        #[derive(Clone, Debug)]
        pub struct RadarIconFormTypes {
        }
        impl RadarIconFormTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RadarIconFormTypes.readType(1): not translated, it uses RadarIconFormType, which isn't translated (it uses IRadarIconFormReader, which isn't translated (it's an interface)).
    }

    /// `RadarIconDefinition` (xaero/hud/minimap/radar/icon/definition/RadarIconDefinition.java)
    pub mod radar_icon_definition {
        use super::*;
        // The class itself isn't translated: it uses `Method` (java.lang.reflect). Its static parts are below.
        // RadarIconDefinition.getVariantForm(1): not translated, its class isn't translated (it uses `Method` (java.lang.reflect)).
        // RadarIconDefinition.construct(1): not translated, its class isn't translated (it uses `Method` (java.lang.reflect)).
        // RadarIconDefinition.constructForm(1): not translated, its class isn't translated (it uses `Method` (java.lang.reflect)).
        // RadarIconDefinition.getVariantMethodString(0): not translated, its class isn't translated (it uses `Method` (java.lang.reflect)).
        // RadarIconDefinition.getVariantMethod(0): not translated, its class isn't translated (it uses `Method` (java.lang.reflect)).
        // RadarIconDefinition.setVariantMethod(1): not translated, its class isn't translated (it uses `Method` (java.lang.reflect)).
        // RadarIconDefinition.getVariantIdBuilderMethodString(0): not translated, its class isn't translated (it uses `Method` (java.lang.reflect)).
        // RadarIconDefinition.getVariantIdBuilderMethod(0): not translated, its class isn't translated (it uses `Method` (java.lang.reflect)).
        // RadarIconDefinition.setVariantIdBuilderMethod(1): not translated, its class isn't translated (it uses `Method` (java.lang.reflect)).
        // RadarIconDefinition.convertStringToMethod(5): not translated, its class isn't translated (it uses `Method` (java.lang.reflect)).
        // RadarIconDefinition.getOldVariantIdMethodString(0): not translated, its class isn't translated (it uses `Method` (java.lang.reflect)).
        // RadarIconDefinition.getOldVariantIdMethod(0): not translated, its class isn't translated (it uses `Method` (java.lang.reflect)).
        // RadarIconDefinition.setOldVariantIdMethod(1): not translated, its class isn't translated (it uses `Method` (java.lang.reflect)).
        // RadarIconDefinition.getModelConfig(1): not translated, its class isn't translated (it uses `Method` (java.lang.reflect)).
        // RadarIconDefinition.getEntityId(0): not translated, its class isn't translated (it uses `Method` (java.lang.reflect)).
    }

    /// `RadarIconDefinitionManager` (xaero/hud/minimap/radar/icon/definition/RadarIconDefinitionManager.java)
    pub mod radar_icon_definition_manager {
        use super::*;
        // The class itself isn't translated: it uses RadarIconDefinition, which isn't translated (it uses `Method` (java.lang.reflect)). Its static parts are below.
        // RadarIconDefinitionManager.get(1): not translated, its class isn't translated (it uses RadarIconDefinition, which isn't translated (it uses `Method` (java.lang.reflect))).
        // RadarIconDefinitionManager.reloadResources(0): not translated, its class isn't translated (it uses RadarIconDefinition, which isn't translated (it uses `Method` (java.lang.reflect))).
    }

    /// `RadarIconDefinitionReloader` (xaero/hud/minimap/radar/icon/definition/RadarIconDefinitionReloader.java)
    pub mod radar_icon_definition_reloader {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct RadarIconDefinitionReloader {
            pub gson: JGson,
        }
        impl RadarIconDefinitionReloader {
            // RadarIconDefinitionReloader.reloadResources(1): not translated, it uses RadarIconDefinition, which isn't translated (it uses `Method` (java.lang.reflect)).
            // RadarIconDefinitionReloader.reloadResourcesAttempt(3): not translated, it uses RadarIconDefinition, which isn't translated (it uses `Method` (java.lang.reflect)).
        }
    }

    /// `RadarIconManager` (xaero/hud/minimap/radar/icon/RadarIconManager.java)
    pub mod radar_icon_manager {
        use super::*;
        // The class itself isn't translated: it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)). Its static parts are below.
        // FAILED: not translated, it uses XaeroIcon, which isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)).
        // DOT: not translated, it uses XaeroIcon, which isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)).
        // RadarIconManager.RadarIconManager(1): not translated, its class isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))).
        // RadarIconManager.get(6): not translated, it's generic.
        // RadarIconManager.get(9): not translated, it's generic.
        // RadarIconManager.reset(0): not translated, its class isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))).
        // RadarIconManager.resetResources(0): not translated, its class isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))).
        // RadarIconManager.allowPrerender(0): not translated, its class isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))).
        // RadarIconManager.onModelPrepareDetection(1): not translated, its class isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))).
        // RadarIconManager.onModelRenderTrace(3): not translated, its class isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))).
        // RadarIconManager.onModelPartRenderTrace(2): not translated, its class isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))).
    }

    /// `RadarSession` (xaero/hud/minimap/radar/RadarSession.java)
    pub mod radar_session {
        use super::*;
        // The class itself isn't translated: it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))). Its static parts are below.
        // RadarSession.RadarSession(4): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
        // RadarSession.update(3): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
        // RadarSession.getCategoryManager(0): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
        // RadarSession.getState(0): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
        // RadarSession.getStateUpdater(0): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
        // RadarSession.getColorHelper(0): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
    }

    /// `RadarSession.Builder` (xaero/hud/minimap/radar/RadarSession.java)
    pub mod radar_session_builder {
        use super::*;
        // The class itself isn't translated: it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))). Its static parts are below.
        // RadarSession.Builder.Builder(0): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
        // RadarSession.Builder.setDefault(0): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
        // RadarSession.Builder.setCategoryManager(1): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
        // RadarSession.Builder.build(0): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
        // RadarSession.Builder.begin(0): not translated, it uses Builder, which isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
    }

    /// `RadarElementReader` (xaero/hud/minimap/radar/render/element/RadarElementReader.java)
    pub mod radar_element_reader {
        use super::*;
        // The class itself isn't translated: it extends `MinimapElementReader` (xaero.hud.minimap.element.render). Its static parts are below.
        // RadarElementReader.getRenderX(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // RadarElementReader.getRenderY(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // RadarElementReader.getRenderZ(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // RadarElementReader.isHidden(2): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // RadarElementReader.getInteractionBoxLeft(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // RadarElementReader.getInteractionBoxRight(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // RadarElementReader.getInteractionBoxTop(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // RadarElementReader.getInteractionBoxBottom(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // RadarElementReader.getRenderBoxLeft(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // RadarElementReader.getRenderBoxRight(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // RadarElementReader.getRenderBoxTop(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // RadarElementReader.getRenderBoxBottom(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // RadarElementReader.getLeftSideLength(2): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // RadarElementReader.getMenuName(1): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // RadarElementReader.getFilterName(1): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // RadarElementReader.getMenuTextFillLeftPadding(1): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // RadarElementReader.getRightClickTitleBackgroundColor(1): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // RadarElementReader.getBoxScale(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // RadarElementReader.isInteractable(2): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // RadarElementReader.shouldScaleBoxWithOptionalScale(0): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
    }

    /// `RadarRenderContext` (xaero/hud/minimap/radar/render/element/RadarRenderContext.java)
    pub mod radar_render_context {
        use super::*;
        // The class itself isn't translated: it uses RadarList, which isn't translated (it implements `Comparable`). Its static parts are below.
    }

    /// `RadarRenderer` (xaero/hud/minimap/radar/render/element/RadarRenderer.java)
    pub mod radar_renderer {
        use super::*;
        // The class itself isn't translated: it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render). Its static parts are below.
        // RadarRenderer.RadarRenderer(5): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // RadarRenderer.preRender(3): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // RadarRenderer.renderElement(10): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // RadarRenderer.postRender(3): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // RadarRenderer.renderIcon(6): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // RadarRenderer.renderDot(9): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // RadarRenderer.renderLabel(6): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // RadarRenderer.setupRenderForList(1): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // RadarRenderer.setupRenderForEntity(1): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // RadarRenderer.confirmTrackedPlayerRadarRender(1): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // RadarRenderer.renderSingleEntity(10): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // RadarRenderer.shouldRender(1): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
    }

    /// `RadarRenderer.Builder` (xaero/hud/minimap/radar/render/element/RadarRenderer.java)
    pub mod radar_renderer_builder {
        use super::*;
        // The class itself isn't translated: it uses RadarIconManager, which isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))). Its static parts are below.
        // RadarRenderer.Builder.Builder(0): not translated, its class isn't translated (it uses RadarIconManager, which isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)))).
        // RadarRenderer.Builder.setDefault(0): not translated, its class isn't translated (it uses RadarIconManager, which isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)))).
        // RadarRenderer.Builder.setRadarIconManager(1): not translated, its class isn't translated (it uses RadarIconManager, which isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)))).
        // RadarRenderer.Builder.setMinimap(1): not translated, its class isn't translated (it uses RadarIconManager, which isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)))).
        // RadarRenderer.Builder.build(0): not translated, its class isn't translated (it uses RadarIconManager, which isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)))).
        // RadarRenderer.Builder.begin(0): not translated, it uses Builder, which isn't translated (it uses RadarIconManager, which isn't translated (it uses RadarIconCreator, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)))).
    }

    /// `RadarRenderProvider` (xaero/hud/minimap/radar/render/element/RadarRenderProvider.java)
    pub mod radar_render_provider {
        use super::*;
        // The class itself isn't translated: it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render). Its static parts are below.
        // SCREEN_FIELD: not translated, it uses `Field` (java.lang.reflect).
        // RadarRenderProvider.begin(2): not translated, its class isn't translated (it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render)).
        // RadarRenderProvider.ensureList(2): not translated, its body couldn't be read: a labelled statement.
        // RadarRenderProvider.shouldRenderOverMinimap(1): not translated, its class isn't translated (it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render)).
        // RadarRenderProvider.hasNext(2): not translated, its class isn't translated (it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render)).
        // RadarRenderProvider.setupContextAndGetNext(2): not translated, its class isn't translated (it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render)).
        // RadarRenderProvider.getNext(2): not translated, its class isn't translated (it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render)).
        // RadarRenderProvider.end(2): not translated, its class isn't translated (it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render)).
        // RadarRenderProvider.setupContextForList(2): not translated, its class isn't translated (it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render)).
        // RadarRenderProvider.setupContextForEntity(2): not translated, its class isn't translated (it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render)).
        // RadarRenderProvider.isUsed(0): not translated, its class isn't translated (it extends `MinimapElementRenderProvider` (xaero.hud.minimap.element.render)).
    }

    /// `RadarList` (xaero/hud/minimap/radar/state/RadarList.java)
    pub mod radar_list {
        use super::*;
        // The class itself isn't translated: it implements `Comparable`. Its static parts are below.
        // RadarList.RadarList(1): not translated, its class isn't translated (it implements `Comparable`).
        // RadarList.getClientCategory(0): not translated, its class isn't translated (it implements `Comparable`).
        // RadarList.getSyncedCategory(0): not translated, its class isn't translated (it implements `Comparable`).
        // RadarList.setClientCategory(1): not translated, its class isn't translated (it implements `Comparable`).
        // RadarList.setSyncedCategory(1): not translated, its class isn't translated (it implements `Comparable`).
        // RadarList.getEffective(1): not translated, it's generic.
        // RadarList.clearEntities(0): not translated, its class isn't translated (it implements `Comparable`).
        // RadarList.add(1): not translated, its class isn't translated (it implements `Comparable`).
        // RadarList.get(1): not translated, its class isn't translated (it implements `Comparable`).
        // RadarList.size(0): not translated, its class isn't translated (it implements `Comparable`).
        // RadarList.getEntities(0): not translated, its class isn't translated (it implements `Comparable`).
        // RadarList.compareTo(1): not translated, its class isn't translated (it implements `Comparable`).
    }

    /// `RadarList.Builder` (xaero/hud/minimap/radar/state/RadarList.java)
    pub mod radar_list_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // RadarList.Builder.setDefault(0): not translated, `this` used as a value.
            // RadarList.Builder.build(0): not translated, it uses RadarList, which isn't translated (it implements `Comparable`).
        }
        // RadarList.Builder.getDefault(0): not translated, it calls Builder.setDefault, which isn't translated.
    }

    /// `RadarState` (xaero/hud/minimap/radar/state/RadarState.java)
    pub mod radar_state {
        use super::*;
        // The class itself isn't translated: it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category)). Its static parts are below.
        // RadarState.getListsReversedOrder(0): not translated, its class isn't translated (it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category))).
        // RadarState.getRadarLists(0): not translated, its class isn't translated (it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category))).
        // RadarState.getListsGeneratedForConfig(0): not translated, its class isn't translated (it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category))).
        // RadarState.getListsGeneratedForSyncedConfig(0): not translated, its class isn't translated (it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category))).
        // RadarState.getUpdatableLists(0): not translated, its class isn't translated (it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category))).
        // RadarState.setListsReversedOrder(1): not translated, its class isn't translated (it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category))).
        // RadarState.setListsGeneratedForConfig(1): not translated, its class isn't translated (it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category))).
        // RadarState.setListsGeneratedForSyncedConfig(1): not translated, its class isn't translated (it uses EntityRadarCategory, which isn't translated (it extends `FilterObjectCategory` (xaero.hud.category))).
    }

    /// `RadarStateUpdater` (xaero/hud/minimap/radar/state/RadarStateUpdater.java)
    pub mod radar_state_updater {
        use super::*;
        // The class itself isn't translated: it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic))). Its static parts are below.
        // RadarStateUpdater.RadarStateUpdater(2): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
        // RadarStateUpdater.update(3): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
        // RadarStateUpdater.ensureCategories(4): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
        // RadarStateUpdater.traceAddCategories(3): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
        // RadarStateUpdater.traceAddSyncedCategories(3): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
        // RadarStateUpdater.putOnUpdateMap(3): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
        // RadarStateUpdater.isWorldMapRadarEnabled(0): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
        // RadarStateUpdater.isInvisibleTo(2): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
        // RadarStateUpdater.shouldHideForSneaking(2): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
        // RadarStateUpdater.setLastRenderViewEntity(1): not translated, its class isn't translated (it uses EntityRadarCategoryManager, which isn't translated (it uses EntityRadarCategorySerializers, which isn't translated (it uses ObjectCategoryDataGsonSerializer, which isn't translated (it's generic)))).
    }

    /// `RadarUtils` (xaero/hud/minimap/radar/util/RadarUtils.java)
    pub mod radar_utils {
        use super::*;
        // FOX_TRUSTED_UUID_SECONDARY: not translated, it uses Minecraft's `EntityDataAccessor`.
        // FOX_TRUSTED_UUID_MAIN: not translated, it uses Minecraft's `EntityDataAccessor`.
        #[derive(Clone, Debug)]
        pub struct RadarUtils {
        }
        impl RadarUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RadarUtils.getMaxDistance(2): not translated, it uses MinimapProcessor, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // RadarUtils.isHostileException(1): not translated, it uses Minecraft's `Entity`.
        // RadarUtils.isTamed(2): not translated, it uses Minecraft's `Entity`.
        // RadarUtils.isHostile(1): not translated, it uses Minecraft's `Entity`.
        // RadarUtils.getCustomName(2): not translated, it uses Minecraft's `Entity`.
    }

    /// `MinimapPipRenderer` (xaero/hud/minimap/render/MinimapPipRenderer.java)
    pub mod minimap_pip_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MinimapPipRenderer.MinimapPipRenderer(1): not translated, its class isn't translated (it's generic).
        // MinimapPipRenderer.getRenderStateClass(0): not translated, its class isn't translated (it's generic).
        // MinimapPipRenderer.prepare(4): not translated, its class isn't translated (it's generic).
        // MinimapPipRenderer.renderToTexture(3): not translated, its class isn't translated (it's generic).
        // MinimapPipRenderer.getTextureLabel(0): not translated, its class isn't translated (it's generic).
    }

    /// `MinimapPipRenderState` (xaero/hud/minimap/render/MinimapPipRenderState.java)
    pub mod minimap_pip_render_state {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `PictureInPictureRenderState`. Its static parts are below.
        pub const HORIZONTAL_PADDING: i32 = 20;
        pub const VERTICAL_PADDING: i32 = 5;
        // MinimapPipRenderState.update(10): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // MinimapPipRenderState.getWidth(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // MinimapPipRenderState.getHeight(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // MinimapPipRenderState.getScale(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // MinimapPipRenderState.getMinimapScale(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // MinimapPipRenderState.getSize(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // MinimapPipRenderState.getBoxSize(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // MinimapPipRenderState.getPartial(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // MinimapPipRenderState.getCvc(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // MinimapPipRenderState.getScaledHorizontalPadding(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // MinimapPipRenderState.getScaledVerticalPadding(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // MinimapPipRenderState.x0(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // MinimapPipRenderState.y0(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // MinimapPipRenderState.x1(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // MinimapPipRenderState.y1(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // MinimapPipRenderState.scale(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // MinimapPipRenderState.scissorArea(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // MinimapPipRenderState.bounds(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
    }

    /// `MinimapPipRenderState.Enlarged` (xaero/hud/minimap/render/MinimapPipRenderState.java)
    pub mod minimap_pip_render_state_enlarged {
        use super::*;
        // The class itself isn't translated: it extends `MinimapPipRenderState`. Its static parts are below.
    }

    /// `DeathpointHandler` (xaero/hud/minimap/waypoint/DeathpointHandler.java)
    pub mod deathpoint_handler {
        use super::*;
        // The class itself isn't translated: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`). Its static parts are below.
        // DeathpointHandler.DeathpointHandler(2): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // DeathpointHandler.createDeathpoint(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // DeathpointHandler.createDeathpoint(3): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
    }

    /// `DestinationHandler` (xaero/hud/minimap/waypoint/DestinationHandler.java)
    pub mod destination_handler {
        use super::*;
        // The class itself isn't translated: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)). Its static parts are below.
        // DestinationHandler.DestinationHandler(2): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // DestinationHandler.begin(4): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // DestinationHandler.handle(1): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // DestinationHandler.end(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
    }

    /// `WaypointIO` (xaero/hud/minimap/waypoint/io/WaypointIO.java)
    pub mod waypoint_io {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct WaypointIO {
            pub old_io: waypoint_old_io::WaypointOldIO,
        }
        impl WaypointIO {
            // WaypointIO.WaypointIO(1): not translated, it uses HudMod, which isn't translated (it implements `IXaeroMinimap`).
            pub fn get_old_io(&self) -> waypoint_old_io::WaypointOldIO {
                return self.old_io.clone();
            }
            // WaypointIO.checkLine(2): not translated, `null` (Rust has no null: this needs an Option).
            // WaypointIO.saveWaypoints(2): not translated, it uses `OutputStreamWriter` (java.io).
        }
    }

    /// `WaypointOldIO` (xaero/hud/minimap/waypoint/io/WaypointOldIO.java)
    pub mod waypoint_old_io {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct WaypointOldIO {
            pub old_waypoints_file: std::path::PathBuf,
        }
        impl WaypointOldIO {
            pub fn new(old_waypoints_file: &std::path::Path) -> Self {
                let mut s = Self { old_waypoints_file: old_waypoints_file.to_path_buf() };
                s.old_waypoints_file = old_waypoints_file.to_path_buf();
                s
            }
            // WaypointOldIO.load(1): not translated, it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)).
            // WaypointOldIO.loadOldWaypoints(1): not translated, it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)).
            // WaypointOldIO.checkLine(2): not translated, it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)).
            // WaypointOldIO.loadFromFile(2): not translated, it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)).
            // WaypointOldIO.convertToNewContainerID(2): not translated, it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)).
            pub fn fix_old_dimension_name(&self, saved_dim_name: &str) -> String {
                if j_eq(&saved_dim_name, "Overworld") {
                    return "dim%0".to_string();
                } else if j_eq(&saved_dim_name, "Nether") {
                    return "dim%-1".to_string();
                } else {
                    return if j_eq(&saved_dim_name, "The End") { "dim%1".to_string() } else { saved_dim_name.to_string() };
                }
                #[allow(unreachable_code)]
                unreachable!()
            }
            // WaypointOldIO.loadFromConfigLines(1): not translated, it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)).
        }
    }

    /// `AbstractWaypointRenderContext` (xaero/hud/minimap/waypoint/render/AbstractWaypointRenderContext.java)
    pub mod abstract_waypoint_render_context {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct AbstractWaypointRenderContext {
            pub dim_coordinate_scale: f64,
        }
        impl AbstractWaypointRenderContext {
            pub fn new() -> Self {
                let mut s = Self { dim_coordinate_scale: 0.0f64 };
                s
            }
        }
    }

    /// `AbstractWaypointRenderProvider` (xaero/hud/minimap/waypoint/render/AbstractWaypointRenderProvider.java)
    pub mod abstract_waypoint_render_provider {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // AbstractWaypointRenderProvider.begin(2): not translated, its class isn't translated (it's generic).
        // AbstractWaypointRenderProvider.hasNext(2): not translated, its class isn't translated (it's generic).
        // AbstractWaypointRenderProvider.getNext(2): not translated, its class isn't translated (it's generic).
        // AbstractWaypointRenderProvider.end(2): not translated, its class isn't translated (it's generic).
    }

    /// `WaypointDeleter` (xaero/hud/minimap/waypoint/render/WaypointDeleter.java)
    pub mod waypoint_deleter {
        use super::*;
        // The class itself isn't translated: it uses IXaeroMinimap, which isn't translated (it's an interface). Its static parts are below.
        // WaypointDeleter.WaypointDeleter(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // WaypointDeleter.begin(0): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // WaypointDeleter.add(1): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
        // WaypointDeleter.deleteCollected(3): not translated, its class isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)).
    }

    /// `WaypointMapRenderContext` (xaero/hud/minimap/waypoint/render/WaypointMapRenderContext.java)
    pub mod waypoint_map_render_context {
        use super::*;
        // The class itself isn't translated: it extends `AbstractWaypointRenderContext`. Its static parts are below.
    }

    /// `WaypointMapRenderer` (xaero/hud/minimap/waypoint/render/WaypointMapRenderer.java)
    pub mod waypoint_map_renderer {
        use super::*;
        // The class itself isn't translated: it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render). Its static parts are below.
        // WaypointMapRenderer.WaypointMapRenderer(3): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderer.renderElement(10): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderer.preRender(3): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderer.postRender(3): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderer.drawIcon(9): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderer.drawIconGUI(5): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderer.drawIcon(16): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderer.drawIconGUI(13): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderer.drawSetChange(3): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderer.shouldRender(1): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderer.getOrder(0): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
    }

    /// `WaypointMapRenderer.Builder` (xaero/hud/minimap/waypoint/render/WaypointMapRenderer.java)
    pub mod waypoint_map_renderer_builder {
        use super::*;
        // The class itself isn't translated: it uses WaypointDeleter, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface)). Its static parts are below.
        // WaypointMapRenderer.Builder.Builder(1): not translated, its class isn't translated (it uses WaypointDeleter, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))).
        // WaypointMapRenderer.Builder.setDefault(0): not translated, its class isn't translated (it uses WaypointDeleter, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))).
        // WaypointMapRenderer.Builder.setWaypointDeleter(1): not translated, its class isn't translated (it uses WaypointDeleter, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))).
        // WaypointMapRenderer.Builder.build(0): not translated, its class isn't translated (it uses WaypointDeleter, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))).
        // WaypointMapRenderer.Builder.begin(1): not translated, it uses Builder, which isn't translated (it uses WaypointDeleter, which isn't translated (it uses IXaeroMinimap, which isn't translated (it's an interface))).
    }

    /// `WaypointMapRenderProvider` (xaero/hud/minimap/waypoint/render/WaypointMapRenderProvider.java)
    pub mod waypoint_map_render_provider {
        use super::*;
        // The class itself isn't translated: it extends `AbstractWaypointRenderProvider`. Its static parts are below.
    }

    /// `WaypointMapRenderReader` (xaero/hud/minimap/waypoint/render/WaypointMapRenderReader.java)
    pub mod waypoint_map_render_reader {
        use super::*;
        // The class itself isn't translated: it extends `MinimapElementReader` (xaero.hud.minimap.element.render). Its static parts are below.
        // WaypointMapRenderReader.getRenderX(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderReader.getRenderY(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderReader.getRenderZ(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderReader.getCoordinateScale(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderReader.shouldScalePartialCoordinates(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderReader.isHidden(2): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderReader.getInteractionBoxLeft(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderReader.getInteractionBoxRight(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderReader.getInteractionBoxTop(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderReader.getInteractionBoxBottom(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderReader.getRenderBoxLeft(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderReader.getRenderBoxRight(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderReader.getRenderBoxTop(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderReader.getRenderBoxBottom(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderReader.getLeftSideLength(2): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderReader.getMenuName(1): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderReader.getFilterName(1): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderReader.getMenuTextFillLeftPadding(1): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderReader.getRightClickTitleBackgroundColor(1): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointMapRenderReader.shouldScaleBoxWithOptionalScale(0): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
    }

    /// `WaypointWorldRenderContext` (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderContext.java)
    pub mod waypoint_world_render_context {
        use super::*;
        // The class itself isn't translated: it extends `AbstractWaypointRenderContext` (xaero.hud.minimap.waypoint.render). Its static parts are below.
    }

    /// `WaypointWorldRenderer` (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderer.java)
    pub mod waypoint_world_renderer {
        use super::*;
        // The class itself isn't translated: it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render). Its static parts are below.
        // WaypointWorldRenderer.WaypointWorldRenderer(3): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderer.renderElement(10): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderer.preRender(3): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderer.postRender(3): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderer.renderIconWithLabels(12): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderer.renderIcon(5): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderer.renderColorBackground(7): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderer.renderTexturedIcon(9): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderer.renderWaypointLabel(6): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderer.getDistanceText(1): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderer.shouldRender(1): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderer.getOrder(0): not translated, its class isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
    }

    /// `WaypointWorldRenderer.Builder` (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderer.java)
    pub mod waypoint_world_renderer_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // WaypointWorldRenderer.Builder.setDefault(0): not translated, `this` used as a value.
            // WaypointWorldRenderer.Builder.build(0): not translated, it uses WaypointWorldRenderer, which isn't translated (it extends `MinimapElementRenderer` (xaero.hud.minimap.element.render)).
        }
        // WaypointWorldRenderer.Builder.begin(0): not translated, it calls Builder.setDefault, which isn't translated.
    }

    /// `WaypointWorldRenderProvider` (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderProvider.java)
    pub mod waypoint_world_render_provider {
        use super::*;
        // The class itself isn't translated: it extends `AbstractWaypointRenderProvider` (xaero.hud.minimap.waypoint.render). Its static parts are below.
    }

    /// `WaypointWorldRenderReader` (xaero/hud/minimap/waypoint/render/world/WaypointWorldRenderReader.java)
    pub mod waypoint_world_render_reader {
        use super::*;
        // The class itself isn't translated: it extends `MinimapElementReader` (xaero.hud.minimap.element.render). Its static parts are below.
        // WaypointWorldRenderReader.WaypointWorldRenderReader(1): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.isHidden(2): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.getRenderX(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.getRenderY(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.getRenderZ(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.getCoordinateScale(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.shouldScalePartialCoordinates(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.getInteractionBoxLeft(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.getInteractionBoxRight(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.getInteractionBoxTop(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.getInteractionBoxBottom(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.getRenderBoxLeft(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.getRenderBoxRight(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.getRenderBoxTop(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.getRenderBoxBottom(3): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.getLeftSideLength(2): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.getMenuName(1): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.getFilterName(1): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.getMenuTextFillLeftPadding(1): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.getRightClickTitleBackgroundColor(1): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.shouldScaleBoxWithOptionalScale(0): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.isInteractable(2): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
        // WaypointWorldRenderReader.isAlwaysHighlightedWhenHovered(2): not translated, its class isn't translated (it extends `MinimapElementReader` (xaero.hud.minimap.element.render)).
    }

    /// `ServerWaypointManager` (xaero/hud/minimap/waypoint/server/ServerWaypointManager.java)
    pub mod server_waypoint_manager {
        use super::*;
        // The class itself isn't translated: it uses ThirdPartyWaypointManager, which isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))). Its static parts are below.
        // ORIGIN_ID: not translated, it uses HudMod.INSTANCE, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // ServerWaypointManager.ServerWaypointManager(1): not translated, its class isn't translated (it uses ThirdPartyWaypointManager, which isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)))).
        // ServerWaypointManager.clear(0): not translated, it uses ServerWaypointManager.ORIGIN_ID, which isn't translated (it uses HudMod.INSTANCE, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))).
        // ServerWaypointManager.remove(1): not translated, it uses ServerWaypointManager.ORIGIN_ID, which isn't translated (it uses HudMod.INSTANCE, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))).
        // ServerWaypointManager.add(2): not translated, it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // ServerWaypointManager.getById(1): not translated, it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // ServerWaypointManager.getBySlot(1): not translated, it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // ServerWaypointManager.getIds(0): not translated, it uses `IntIterable` (it.unimi.dsi.fastutil.ints).
        // ServerWaypointManager.addDisabled(1): not translated, it uses ServerWaypointManager.ORIGIN_ID, which isn't translated (it uses HudMod.INSTANCE, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))).
        // ServerWaypointManager.isEmpty(0): not translated, it uses ServerWaypointManager.ORIGIN_ID, which isn't translated (it uses HudMod.INSTANCE, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))).
        // ServerWaypointManager.getWaypoints(0): not translated, it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // ServerWaypointManager.size(0): not translated, it uses ServerWaypointManager.ORIGIN_ID, which isn't translated (it uses HudMod.INSTANCE, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))).
    }

    /// `WaypointSet` (xaero/hud/minimap/waypoint/set/WaypointSet.java)
    pub mod waypoint_set {
        use super::*;
        // The class itself isn't translated: it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)). Its static parts are below.
        // WaypointSet.WaypointSet(1): not translated, its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))).
        // WaypointSet.getName(0): not translated, its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))).
        // WaypointSet.getWaypoints(0): not translated, its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))).
        // WaypointSet.addTo(1): not translated, its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))).
        // WaypointSet.add(2): not translated, its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))).
        // WaypointSet.add(1): not translated, its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))).
        // WaypointSet.addAll(2): not translated, its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))).
        // WaypointSet.addAll(1): not translated, its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))).
        // WaypointSet.remove(1): not translated, its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))).
        // WaypointSet.remove(1): not translated, its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))).
        // WaypointSet.removeAll(1): not translated, its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))).
        // WaypointSet.clear(0): not translated, its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))).
        // WaypointSet.isEmpty(0): not translated, its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))).
        // WaypointSet.size(0): not translated, its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))).
        // WaypointSet.get(1): not translated, its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))).
        // WaypointSet.set(2): not translated, its class isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))).
    }

    /// `WaypointSet.Builder` (xaero/hud/minimap/waypoint/set/WaypointSet.java)
    pub mod waypoint_set_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
            pub name: Option<String>,
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self { name: None };
                s
            }
            // WaypointSet.Builder.setDefault(0): not translated, `this` used as a value.
            // WaypointSet.Builder.setName(1): not translated, `this` used as a value.
            // WaypointSet.Builder.build(0): not translated, it uses WaypointSet, which isn't translated (it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint))).
        }
        // WaypointSet.Builder.begin(0): not translated, it calls Builder.setDefault, which isn't translated.
    }

    /// `TemporaryWaypointHandler` (xaero/hud/minimap/waypoint/TemporaryWaypointHandler.java)
    pub mod temporary_waypoint_handler {
        use super::*;
        // The class itself isn't translated: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`). Its static parts are below.
        // TemporaryWaypointHandler.TemporaryWaypointHandler(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // TemporaryWaypointHandler.createTemporaryWaypoint(4): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // TemporaryWaypointHandler.createTemporaryWaypoint(5): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // TemporaryWaypointHandler.createTemporaryWaypoint(6): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
    }

    /// `ThirdPartyWaypointManager` (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypointManager.java)
    pub mod third_party_waypoint_manager {
        use super::*;
        // The class itself isn't translated: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)). Its static parts are below.
        // ThirdPartyWaypointManager.ThirdPartyWaypointManager(1): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypointManager.clear(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypointManager.clearOrigin(1): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypointManager.getOriginCount(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypointManager.get(1): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypointManager.getByIndex(1): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypointManager.getCount(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypointManager.getAll(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
    }

    /// `ThirdPartyWaypoints` (xaero/hud/minimap/waypoint/thirdparty/ThirdPartyWaypoints.java)
    pub mod third_party_waypoints {
        use super::*;
        // The class itself isn't translated: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)). Its static parts are below.
        // ThirdPartyWaypoints.ThirdPartyWaypoints(2): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypoints.clear(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypoints.remove(1): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypoints.add(2): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypoints.get(1): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypoints.getBySlot(1): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypoints.getIds(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypoints.addRenderInfoOverride(2): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypoints.isEmpty(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypoints.getWaypoints(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypoints.getCount(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypoints.getOriginId(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypoints.getRenderInfoOverrides(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypoints.setEnabledStateGetter(1): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypoints.isEnabled(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypoints.hasEnabledStateGetter(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // ThirdPartyWaypoints.getEnabledStateGetter(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
    }

    /// `WaypointUtils` (xaero/hud/minimap/waypoint/util/WaypointUtils.java)
    pub mod waypoint_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct WaypointUtils {
        }
        impl WaypointUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // WaypointUtils.getDistanceText(3): not translated, it calls GuiMisc.getFormat, which isn't translated.
        // WaypointUtils.getDistanceText(1): not translated, it uses `ClientConfigManager` (xaero.lib.client.config).
        // WaypointUtils.getDistanceText(6): not translated, it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
        // WaypointUtils.getDistanceTextForCurrentWorld(5): not translated, it uses Waypoint, which isn't translated (it extends `WaypointRenderInfo` (xaero.hud.minimap.waypoint)).
    }

    /// `WaypointCollector` (xaero/hud/minimap/waypoint/WaypointCollector.java)
    pub mod waypoint_collector {
        use super::*;
        // The class itself isn't translated: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)). Its static parts are below.
        // WaypointCollector.WaypointCollector(1): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // WaypointCollector.collect(1): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
    }

    /// `WaypointColor` (xaero/hud/minimap/waypoint/WaypointColor.java)
    pub mod waypoint_color {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // WaypointColor.WaypointColor(3): not translated, its class isn't translated (its constants carry values).
        // WaypointColor.getName(0): not translated, its class isn't translated (its constants carry values).
        // WaypointColor.getFormat(0): not translated, its class isn't translated (its constants carry values).
        // WaypointColor.getHex(0): not translated, its class isn't translated (its constants carry values).
        // WaypointColor.fromIndex(1): not translated, it uses WaypointColor, which isn't translated (its constants carry values).
        // WaypointColor.getRandom(0): not translated, it uses WaypointColor, which isn't translated (its constants carry values).
        // WaypointColor.fromDye(1): not translated, it uses WaypointColor, which isn't translated (its constants carry values).
    }

    /// `WaypointPurpose` (xaero/hud/minimap/waypoint/WaypointPurpose.java)
    pub mod waypoint_purpose {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // WaypointPurpose.WaypointPurpose(2): not translated, its class isn't translated (its constants carry values).
        // WaypointPurpose.isDeath(0): not translated, its class isn't translated (its constants carry values).
        // WaypointPurpose.isDestination(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `WaypointRenderInfo` (xaero/hud/minimap/waypoint/WaypointRenderInfo.java)
    pub mod waypoint_render_info {
        use super::*;
        // The class itself isn't translated: it uses WaypointColor, which isn't translated (its constants carry values). Its static parts are below.
        // WaypointRenderInfo.getInitials(0): not translated, its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)).
        // WaypointRenderInfo.getInitialsSafe(1): not translated, its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)).
        // WaypointRenderInfo.setInitials(1): not translated, its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)).
        // WaypointRenderInfo.getWaypointColor(0): not translated, its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)).
        // WaypointRenderInfo.setWaypointColor(1): not translated, its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)).
        // WaypointRenderInfo.getVisibility(0): not translated, its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)).
        // WaypointRenderInfo.setVisibility(1): not translated, its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)).
        // WaypointRenderInfo.getDisabled(0): not translated, its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)).
        // WaypointRenderInfo.setDisabled(1): not translated, its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)).
        // WaypointRenderInfo.isThirdPartyDeleted(0): not translated, its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)).
        // WaypointRenderInfo.setThirdPartyDeleted(1): not translated, its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)).
        // WaypointRenderInfo.nullEverythingMatching(1): not translated, its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)).
        // WaypointRenderInfo.clear(0): not translated, its class isn't translated (it uses WaypointColor, which isn't translated (its constants carry values)).
    }

    /// `WaypointSession` (xaero/hud/minimap/waypoint/WaypointSession.java)
    pub mod waypoint_session {
        use super::*;
        // The class itself isn't translated: it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module)). Its static parts are below.
        // WaypointSession.WaypointSession(2): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // WaypointSession.getSharing(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // WaypointSession.getDeathpointHandler(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // WaypointSession.getTemporaryHandler(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // WaypointSession.getTeleport(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // WaypointSession.getCollector(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // WaypointSession.getDeleter(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // WaypointSession.getDestinationHandler(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // WaypointSession.getMc(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // WaypointSession.getSetChangedTime(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // WaypointSession.setSetChangedTime(1): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
        // WaypointSession.getSession(0): not translated, its class isn't translated (it uses MinimapSession, which isn't translated (it extends `ModuleSession` (xaero.hud.module))).
    }

    /// `WaypointSharingHandler` (xaero/hud/minimap/waypoint/WaypointSharingHandler.java)
    pub mod waypoint_sharing_handler {
        use super::*;
        // The class itself isn't translated: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`). Its static parts are below.
        pub const WAYPOINT_OLD_SHARE_PREFIX: &str = "xaero_waypoint:";
        pub const WAYPOINT_ADD_PREFIX: &str = "xaero_waypoint_add:";
        pub const WAYPOINT_SHARE_PREFIX: &str = "xaero-waypoint:";
        pub const DESTINATION_PREFIX_INTERNAL: &str = "Internal";
        pub const DESTINATION_PREFIX_INTERNAL_HYPHEN: &str = "Internal-";
        pub const DESTINATION_PREFIX_EXTERNAL: &str = "External";
        // WaypointSharingHandler.WaypointSharingHandler(2): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointSharingHandler.shareWaypoint(3): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointSharingHandler.onShareConfirmationResult(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointSharingHandler.getSharedDestinationDetails(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointSharingHandler.onWaypointReceived(2): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointSharingHandler.getReceivedDimensionName(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointSharingHandler.onWaypointAdd(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointSharingHandler.getReceivedDestinationWorld(2): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointSharingHandler.getReceivedDimId(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointSharingHandler.removeFormatting(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointSharingHandler.restoreFormatting(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
    }

    /// `WaypointSharingHandler.Builder` (xaero/hud/minimap/waypoint/WaypointSharingHandler.java)
    pub mod waypoint_sharing_handler_builder {
        use super::*;
        // The class itself isn't translated: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`). Its static parts are below.
        // WaypointSharingHandler.Builder.Builder(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointSharingHandler.Builder.setDefault(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointSharingHandler.Builder.setModMain(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointSharingHandler.Builder.setSession(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointSharingHandler.Builder.build(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointSharingHandler.Builder.begin(0): not translated, it uses Builder, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
    }

    /// `WaypointsSort` (xaero/hud/minimap/waypoint/WaypointsSort.java)
    pub mod waypoints_sort {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // WaypointsSort.WaypointsSort(1): not translated, its class isn't translated (its constants carry values).
    }

    /// `WaypointTeleport` (xaero/hud/minimap/waypoint/WaypointTeleport.java)
    pub mod waypoint_teleport {
        use super::*;
        // The class itself isn't translated: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`). Its static parts are below.
        pub const TELEPORT_ANYWAY_COMMAND: &str = "xaero_tp_anyway";
        pub const SLASH_TELEPORT_ANYWAY_COMMAND: &str = "/xaero_tp_anyway";
        // WaypointTeleport.WaypointTeleport(3): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointTeleport.canTeleport(2): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointTeleport.teleportAnyway(0): not translated, its body couldn't be read: an anonymous class.
        // WaypointTeleport.teleportToWaypoint(3): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointTeleport.teleportToWaypoint(4): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointTeleport.isWorldTeleportable(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // WaypointTeleport.isTeleportationSafe(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
    }

    /// `WaypointVisibilityType` (xaero/hud/minimap/waypoint/WaypointVisibilityType.java)
    pub mod waypoint_visibility_type {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // WaypointVisibilityType.WaypointVisibilityType(2): not translated, its class isn't translated (its constants carry values).
        // WaypointVisibilityType.getTranslation(0): not translated, its class isn't translated (its constants carry values).
        // WaypointVisibilityType.isGlobal(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `MinimapWorldConnectionManager` (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java)
    pub mod minimap_world_connection_manager {
        use super::*;
        // The class itself isn't translated: it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)). Its static parts are below.
        // MinimapWorldConnectionManager.MinimapWorldConnectionManager(1): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldConnectionManager.addConnection(2): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldConnectionManager.addConnection(2): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldConnectionManager.addOneWayConnection(2): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldConnectionManager.removeConnection(2): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldConnectionManager.removeConnection(2): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldConnectionManager.removeOneWayConnection(2): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldConnectionManager.isConnected(2): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldConnectionManager.isEmpty(0): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldConnectionManager.save(1): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldConnectionManager.swapConnections(2): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldConnectionManager.swapConnections(2): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldConnectionManager.renameDimension(2): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
    }

    /// `MinimapWorldConnectionManager.Builder` (xaero/hud/minimap/world/connection/MinimapWorldConnectionManager.java)
    pub mod minimap_world_connection_manager_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
            pub multiplayer: bool,
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self { multiplayer: false };
                s
            }
            // MinimapWorldConnectionManager.Builder.setDefault(0): not translated, `this` used as a value.
            // MinimapWorldConnectionManager.Builder.setMultiplayer(1): not translated, `this` used as a value.
            // MinimapWorldConnectionManager.Builder.build(0): not translated, it uses MinimapWorldConnectionManager, which isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        }
        // MinimapWorldConnectionManager.Builder.begin(0): not translated, it calls Builder.setDefault, which isn't translated.
    }

    /// `RootConfigIO` (xaero/hud/minimap/world/container/config/io/RootConfigIO.java)
    pub mod root_config_io {
        use super::*;
        // The class itself isn't translated: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`). Its static parts are below.
        // RootConfigIO.RootConfigIO(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // RootConfigIO.getFile(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // RootConfigIO.save(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // RootConfigIO.load(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
    }

    /// `RootConfig` (xaero/hud/minimap/world/container/config/RootConfig.java)
    pub mod root_config {
        use super::*;
        // The class itself isn't translated: it uses WaypointsSort, which isn't translated (its constants carry values). Its static parts are below.
        // RootConfig.RootConfig(1): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.isUsingMultiworldDetection(0): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.setUsingMultiworldDetection(1): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.getDefaultMultiworldId(0): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.setDefaultMultiworldId(1): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.isTeleportationEnabled(0): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.setTeleportationEnabled(1): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.isUsingDefaultTeleportCommand(0): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.setUsingDefaultTeleportCommand(1): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.getServerTeleportCommandFormat(0): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.getServerTeleportCommandRotationFormat(0): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.setServerTeleportCommandFormat(1): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.setServerTeleportCommandRotationFormat(1): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.getSortType(0): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.setSortType(1): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.toggleSortType(0): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.isSortReversed(0): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.setSortReversed(1): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.toggleSortReversed(0): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.isIgnoreServerLevelId(0): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.setIgnoreServerLevelId(1): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.isIgnoreHeightmaps(0): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.setIgnoreHeightmaps(1): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.resetSubWorldConnections(1): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.getSubWorldConnections(0): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.isLoaded(0): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
        // RootConfig.setLoaded(1): not translated, its class isn't translated (it uses WaypointsSort, which isn't translated (its constants carry values)).
    }

    /// `MinimapWorldContainer` (xaero/hud/minimap/world/container/MinimapWorldContainer.java)
    pub mod minimap_world_container {
        use super::*;
        // The class itself isn't translated: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`). Its static parts are below.
        pub static UNKNOWN_DIM_ID: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("xaerominimap", "unknown"));
        // MinimapWorldContainer.MinimapWorldContainer(10): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.getEquivalentDimKey(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.getEquivalentDimId(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.setPath(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.addSubContainer(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.containsSubContainer(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.deleteSubContainer(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.isEmpty(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.addWorld(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.addWorld(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.removeWorld(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.setName(2): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.getName(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.removeName(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.getLastNode(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.getSubName(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.getFullWorldName(2): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.getPath(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.getFirstWorld(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.getFirstWorldConnectedTo(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.toString(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.getWorlds(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.getWorldsCopy(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.getSubContainers(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.getAllWorldsIterable(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.fixPathCharacterCases(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.getRoot(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.getRootConfig(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.getDirectoryPath(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.getSession(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.getServerWaypointManager(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldContainer.getThirdPartyWaypointManager(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
    }

    /// `MinimapWorldContainer.Builder` (xaero/hud/minimap/world/container/MinimapWorldContainer.java)
    pub mod minimap_world_container_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MinimapWorldContainer.Builder.Builder(0): not translated, its class isn't translated (it's generic).
        // MinimapWorldContainer.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // MinimapWorldContainer.Builder.setModMain(1): not translated, its class isn't translated (it's generic).
        // MinimapWorldContainer.Builder.setSession(1): not translated, its class isn't translated (it's generic).
        // MinimapWorldContainer.Builder.setPath(1): not translated, its class isn't translated (it's generic).
        // MinimapWorldContainer.Builder.setRootContainer(1): not translated, its class isn't translated (it's generic).
        // MinimapWorldContainer.Builder.determineDimensionEquivalent(0): not translated, its class isn't translated (it's generic).
        // MinimapWorldContainer.Builder.build(0): not translated, its class isn't translated (it's generic).
        // MinimapWorldContainer.Builder.buildInternally(6): not translated, it has no body.
    }

    /// `MinimapWorldContainer.FinalBuilder` (xaero/hud/minimap/world/container/MinimapWorldContainer.java)
    pub mod minimap_world_container_final_builder {
        use super::*;
        // The class itself isn't translated: it extends `Builder`. Its static parts are below.
        // MinimapWorldContainer.FinalBuilder.FinalBuilder(0): not translated, its class isn't translated (it extends `Builder`).
        // MinimapWorldContainer.FinalBuilder.build(0): not translated, its class isn't translated (it extends `Builder`).
        // MinimapWorldContainer.FinalBuilder.buildInternally(6): not translated, its class isn't translated (it extends `Builder`).
        // MinimapWorldContainer.FinalBuilder.begin(0): not translated, it uses FinalBuilder, which isn't translated (it extends `Builder`).
    }

    /// `MinimapWorldContainerUtil` (xaero/hud/minimap/world/container/MinimapWorldContainerUtil.java)
    pub mod minimap_world_container_util {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MinimapWorldContainerUtil {
        }
        impl MinimapWorldContainerUtil {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MinimapWorldContainerUtil.isMultiplayer(1): not translated, it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)).
        pub fn convert_world_folder_to_container_node_1(world_folder: &str) -> String {
            return minimap_world_container_util::convert_world_folder_to_container_node_2(&world_folder, 4);
        }
        pub fn convert_world_folder_to_container_node_2(world_folder: &str, version: i32) -> String {
            let mut result: String = ((world_folder.replace("_", "%us%")).replace("/", "%fs%")).replace("\\", "%bs%");
            if version >= 2 {
                result = (result.replace("[", "%lb%")).replace("]", "%rb%");
            }
            return result.to_string();
        }
    }

    /// `MinimapWorldRootContainer` (xaero/hud/minimap/world/container/MinimapWorldRootContainer.java)
    pub mod minimap_world_root_container {
        use super::*;
        // The class itself isn't translated: it extends `MinimapWorldContainer`. Its static parts are below.
        // MinimapWorldRootContainer.MinimapWorldRootContainer(12): not translated, its class isn't translated (it extends `MinimapWorldContainer`).
        // MinimapWorldRootContainer.updateConnectionsField(1): not translated, its class isn't translated (it extends `MinimapWorldContainer`).
        // MinimapWorldRootContainer.getSubWorldConnections(0): not translated, its class isn't translated (it extends `MinimapWorldContainer`).
        // MinimapWorldRootContainer.getDimensionType(1): not translated, its class isn't translated (it extends `MinimapWorldContainer`).
        // MinimapWorldRootContainer.getDimensionScale(1): not translated, its class isn't translated (it extends `MinimapWorldContainer`).
        // MinimapWorldRootContainer.updateDimensionType(1): not translated, its class isn't translated (it extends `MinimapWorldContainer`).
        // MinimapWorldRootContainer.renameOldContainer(1): not translated, its class isn't translated (it extends `MinimapWorldContainer`).
        // MinimapWorldRootContainer.getDimensionTypeIds(0): not translated, its class isn't translated (it extends `MinimapWorldContainer`).
        // MinimapWorldRootContainer.setDimensionTypeId(2): not translated, its class isn't translated (it extends `MinimapWorldContainer`).
        // MinimapWorldRootContainer.getRoot(0): not translated, its class isn't translated (it extends `MinimapWorldContainer`).
        // MinimapWorldRootContainer.isConfigLoaded(0): not translated, its class isn't translated (it extends `MinimapWorldContainer`).
        // MinimapWorldRootContainer.getConfig(0): not translated, its class isn't translated (it extends `MinimapWorldContainer`).
    }

    /// `MinimapWorldRootContainer.Builder` (xaero/hud/minimap/world/container/MinimapWorldRootContainer.java)
    pub mod minimap_world_root_container_builder {
        use super::*;
        // The class itself isn't translated: it extends `Builder`. Its static parts are below.
        // MinimapWorldRootContainer.Builder.Builder(0): not translated, its class isn't translated (it extends `Builder`).
        // MinimapWorldRootContainer.Builder.setDefault(0): not translated, its class isn't translated (it extends `Builder`).
        // MinimapWorldRootContainer.Builder.build(0): not translated, its class isn't translated (it extends `Builder`).
        // MinimapWorldRootContainer.Builder.buildInternally(6): not translated, its class isn't translated (it extends `Builder`).
        // MinimapWorldRootContainer.Builder.begin(0): not translated, it uses Builder, which isn't translated (it extends `Builder`).
    }

    /// `MinimapWorldManagerIO` (xaero/hud/minimap/world/io/MinimapWorldManagerIO.java)
    pub mod minimap_world_manager_io {
        use super::*;
        // The class itself isn't translated: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`). Its static parts are below.
        // MinimapWorldManagerIO.MinimapWorldManagerIO(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManagerIO.loadWorldsFromAllSources(2): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManagerIO.loadAllWorlds(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManagerIO.loadWorldFolder(3): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManagerIO.loadDimensionFolder(4): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManagerIO.loadWorldFile(3): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManagerIO.loadWorld(2): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManagerIO.checkWorldFileLine(2): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManagerIO.saveWorlds(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManagerIO.saveAllWorlds(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManagerIO.saveWorld(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManagerIO.saveWorld(2): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManagerIO.convertWorldDimFilesToFolders(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManagerIO.convertWorldDimFoldersToSingleFolder(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManagerIO.copyTempFilesBack(1): not translated, it uses `Stream` (java.util.stream).
        // MinimapWorldManagerIO.fixOldRootFolder(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManagerIO.fixOldRootFolder(2): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManagerIO.onRootContainerAdded(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManagerIO.getWorldFile(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManagerIO.getRootConfigIO(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
    }

    /// `MinimapDimensionHelper` (xaero/hud/minimap/world/MinimapDimensionHelper.java)
    pub mod minimap_dimension_helper {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ResourceKey`. Its static parts are below.
        // MinimapDimensionHelper.getDimensionDivision(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapDimensionHelper.getDimCoordinateScale(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapDimensionHelper.getDimensionDirectoryName(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapDimensionHelper.convertToDimensionDirectoryName(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapDimensionHelper.findDimensionKeyForOldName(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapDimensionHelper.getDimensionKeyForDirectoryName(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
    }

    /// `MinimapWorld` (xaero/hud/minimap/world/MinimapWorld.java)
    pub mod minimap_world {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ResourceKey`. Its static parts are below.
        pub const DEFAULT_SET: &str = "gui.xaero_default";
        // MinimapWorld.MinimapWorld(3): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.getCurrentWaypointSet(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.addWaypointSet(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.cleanupOnSave(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.getLocalWorldKey(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.addWaypointSet(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.getWaypointSet(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.removeWaypointSet(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.getIterableWaypointSets(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.getCurrentWaypointSetId(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.setCurrentWaypointSetId(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.getNode(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.getFullPath(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.setNode(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.getContainer(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.setContainer(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.requestRemovalOnSave(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.hasSomethingToRemoveOnSave(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.getDimId(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.setDimId(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.getSetCount(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.getSlimeChunkSeed(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.setSlimeChunkSeed(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MinimapWorld.getRootConfig(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
    }

    /// `MinimapWorld.Builder` (xaero/hud/minimap/world/MinimapWorld.java)
    pub mod minimap_world_builder {
        use super::*;
        // The class itself isn't translated: it uses MinimapWorldContainer, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)). Its static parts are below.
        // MinimapWorld.Builder.Builder(0): not translated, its class isn't translated (it uses MinimapWorldContainer, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))).
        // MinimapWorld.Builder.setDefault(0): not translated, its class isn't translated (it uses MinimapWorldContainer, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))).
        // MinimapWorld.Builder.setContainer(1): not translated, its class isn't translated (it uses MinimapWorldContainer, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))).
        // MinimapWorld.Builder.setNode(1): not translated, its class isn't translated (it uses MinimapWorldContainer, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))).
        // MinimapWorld.Builder.setDimId(1): not translated, its class isn't translated (it uses MinimapWorldContainer, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))).
        // MinimapWorld.Builder.build(0): not translated, its class isn't translated (it uses MinimapWorldContainer, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))).
        // MinimapWorld.Builder.begin(0): not translated, it uses Builder, which isn't translated (it uses MinimapWorldContainer, which isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`))).
    }

    /// `MinimapWorldManager` (xaero/hud/minimap/world/MinimapWorldManager.java)
    pub mod minimap_world_manager {
        use super::*;
        // The class itself isn't translated: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`). Its static parts are below.
        // MinimapWorldManager.MinimapWorldManager(2): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManager.getWorldContainer(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManager.getWorldContainerNullable(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManager.getRootWorldContainer(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManager.getRootWorldContainer(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManager.addWorldContainer(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManager.addRootWorldContainer(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManager.removeContainer(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManager.containerExists(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManager.getWorld(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManager.addWorld(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManager.getCurrentWorld(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManager.getCurrentWorld(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManager.getAutoWorld(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManager.getRootContainers(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManager.getAutoRootContainer(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManager.getCurrentRootContainer(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManager.getCustomWaypoints(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManager.hasCustomWaypoints(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldManager.getCustomWaypoints(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
    }

    /// `MinimapWorldState` (xaero/hud/minimap/world/state/MinimapWorldState.java)
    pub mod minimap_world_state {
        use super::*;
        // The class itself isn't translated: it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)). Its static parts are below.
        // MinimapWorldState.setAutoRootContainerPath(1): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldState.getAutoRootContainerPath(0): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldState.setOutdatedAutoRootContainerPath(2): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldState.getOutdatedAutoRootContainerPath(1): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldState.getAutoWorldPath(0): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldState.setAutoWorldPath(1): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldState.getCustomContainerPath(0): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldState.getCustomWorldPath(0): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldState.setCustomWorldPath(1): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldState.getAutoContainerPathIgnoreCaseCache(0): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldState.setAutoContainerPathIgnoreCaseCache(1): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldState.getCurrentWorldPath(0): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldState.getCurrentContainerPath(0): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldState.getCurrentRootContainerPath(0): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
        // MinimapWorldState.getCurrentWorldPath(1): not translated, its class isn't translated (it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure))).
    }

    /// `MinimapWorldStateUpdater` (xaero/hud/minimap/world/state/MinimapWorldStateUpdater.java)
    pub mod minimap_world_state_updater {
        use super::*;
        // The class itself isn't translated: it uses HudMod, which isn't translated (it implements `IXaeroMinimap`). Its static parts are below.
        pub const ROOT_CONTAINER_FORMAT: i32 = 4;
        // MinimapWorldStateUpdater.MinimapWorldStateUpdater(3): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldStateUpdater.init(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldStateUpdater.update(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldStateUpdater.getPotentialContainerPath(0): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldStateUpdater.ignoreContainerCase(2): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldStateUpdater.getAutoRootContainerPath(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldStateUpdater.getPotentialWorldNode(2): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldStateUpdater.hasServerLevelId(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldStateUpdater.getAutoWorldNodeBase(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldStateUpdater.onServerLevelId(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
        // MinimapWorldStateUpdater.setCurrentWorldSpawn(1): not translated, its class isn't translated (it uses HudMod, which isn't translated (it implements `IXaeroMinimap`)).
    }

    /// `HudModule` (xaero/hud/module/HudModule.java)
    pub mod hud_module {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // HudModule.HudModule(6): not translated, its class isn't translated (it's generic).
        // HudModule.getId(0): not translated, its class isn't translated (it's generic).
        // HudModule.isActive(1): not translated, its class isn't translated (it's generic).
        // HudModule.setActive(2): not translated, its class isn't translated (it's generic).
        // HudModule.getCurrentSession(0): not translated, its class isn't translated (it's generic).
        // HudModule.getRenderer(0): not translated, its class isn't translated (it's generic).
        // HudModule.getUsedTransform(0): not translated, its class isn't translated (it's generic).
        // HudModule.getUnconfirmedTransform(0): not translated, its class isn't translated (it's generic).
        // HudModule.confirmTransform(0): not translated, its class isn't translated (it's generic).
        // HudModule.getConfirmedTransform(0): not translated, its class isn't translated (it's generic).
        // HudModule.setTransform(1): not translated, its class isn't translated (it's generic).
        // HudModule.cancelTransform(0): not translated, its class isn't translated (it's generic).
        // HudModule.getPushState(0): not translated, its class isn't translated (it's generic).
        // HudModule.getDisplayName(0): not translated, its class isn't translated (it's generic).
        // HudModule.getConfigScreenFactory(0): not translated, its class isn't translated (it's generic).
        // HudModule.getSessionFactory(0): not translated, its class isn't translated (it's generic).
        // HudModule.setRenderer(1): not translated, its class isn't translated (it's generic).
    }

    /// `ModuleManager` (xaero/hud/module/ModuleManager.java)
    pub mod module_manager {
        use super::*;
        // The class itself isn't translated: it uses HudModule, which isn't translated (it's generic). Its static parts are below.
        // ModuleManager.register(1): not translated, its class isn't translated (it uses HudModule, which isn't translated (it's generic)).
        // ModuleManager.get(1): not translated, its class isn't translated (it uses HudModule, which isn't translated (it's generic)).
        // ModuleManager.getModules(0): not translated, its class isn't translated (it uses HudModule, which isn't translated (it's generic)).
    }

    /// `ModuleSession` (xaero/hud/module/ModuleSession.java)
    pub mod module_session {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ModuleSession.ModuleSession(2): not translated, its class isn't translated (it's generic).
        // ModuleSession.getModule(0): not translated, its class isn't translated (it's generic).
        // ModuleSession.getModMain(0): not translated, its class isn't translated (it's generic).
        // ModuleSession.isActive(0): not translated, its class isn't translated (it's generic).
        // ModuleSession.getEffectiveX(2): not translated, its class isn't translated (it's generic).
        // ModuleSession.getEffectiveY(2): not translated, its class isn't translated (it's generic).
        // ModuleSession.isFlippedHor(0): not translated, its class isn't translated (it's generic).
        // ModuleSession.isFlippedVer(0): not translated, its class isn't translated (it's generic).
        // ModuleSession.isCentered(0): not translated, its class isn't translated (it's generic).
        // ModuleSession.shouldFlipHorizontally(2): not translated, its class isn't translated (it's generic).
        // ModuleSession.shouldFlipVertically(2): not translated, its class isn't translated (it's generic).
        // ModuleSession.prePotentialRender(0): not translated, its class isn't translated (it's generic).
        // ModuleSession.onPostGameOverlay(0): not translated, its class isn't translated (it's generic).
        // ModuleSession.getWidth(1): not translated, it has no body.
        // ModuleSession.getHeight(1): not translated, it has no body.
        // ModuleSession.close(0): not translated, it has no body.
    }

    /// `ModuleSessionHandler` (xaero/hud/module/ModuleSessionHandler.java)
    pub mod module_session_handler {
        use super::*;
        // The class itself isn't translated: it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic)). Its static parts are below.
        // ModuleSessionHandler.ModuleSessionHandler(1): not translated, its class isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))).
        // ModuleSessionHandler.resetSessions(3): not translated, its class isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))).
        // ModuleSessionHandler.closeSessions(1): not translated, its class isn't translated (it uses ModuleManager, which isn't translated (it uses HudModule, which isn't translated (it's generic))).
        // ModuleSessionHandler.resetSession(4): not translated, it's generic.
        // ModuleSessionHandler.closeSession(2): not translated, it's generic.
    }

    /// `ModuleTransform` (xaero/hud/module/ModuleTransform.java)
    pub mod module_transform {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ModuleTransform {
            pub x: i32,
            pub y: i32,
            pub centered: bool,
            pub from_right: bool,
            pub from_bottom: bool,
            pub flipped_hor: bool,
            pub flipped_ver: bool,
            pub from_old_system: bool,
        }
        impl ModuleTransform {
            pub fn new() -> Self {
                let mut s = Self { x: 0i32, y: 0i32, centered: false, from_right: false, from_bottom: false, flipped_hor: false, flipped_ver: false, from_old_system: false };
                s
            }
            pub fn copy(&self) -> module_transform::ModuleTransform {
                let mut copy: module_transform::ModuleTransform = module_transform::ModuleTransform::new();
                copy.x = self.x;
                copy.y = self.y;
                copy.centered = self.centered;
                copy.from_right = self.from_right;
                copy.from_bottom = self.from_bottom;
                copy.flipped_hor = self.flipped_hor;
                copy.flipped_ver = self.flipped_ver;
                copy.from_old_system = self.from_old_system;
                return copy.clone();
            }
        }
    }

    /// `XaeroNbtUtils` (xaero/hud/nbt/util/XaeroNbtUtils.java)
    pub mod xaero_nbt_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct XaeroNbtUtils {
        }
        impl XaeroNbtUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // XaeroNbtUtils.getUUID(2): not translated, it uses `UUID` (java.util).
        // XaeroNbtUtils.putUUID(3): not translated, it uses Minecraft's `CompoundTag`.
        // XaeroNbtUtils.putUUIDAsLongArray(3): not translated, it uses Minecraft's `CompoundTag`.
        // XaeroNbtUtils.createUUIDTag(1): not translated, it uses Minecraft's `IntArrayTag`.
    }

    /// `ClientboundRulesPacket` (xaero/hud/packet/basic/ClientboundRulesPacket.java)
    pub mod clientbound_rules_packet {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientboundRulesPacket {
            pub allow_cave_mode_on_server: bool,
            pub allow_nether_cave_mode_on_server: bool,
            pub allow_radar_on_server: bool,
        }
        impl ClientboundRulesPacket {
            pub fn new(allow_cave_mode_on_server: bool, allow_nether_cave_mode_on_server: bool, allow_radar_on_server: bool) -> Self {
                let mut s = Self { allow_cave_mode_on_server: allow_cave_mode_on_server, allow_nether_cave_mode_on_server: allow_nether_cave_mode_on_server, allow_radar_on_server: allow_radar_on_server };
                s.allow_cave_mode_on_server = allow_cave_mode_on_server;
                s.allow_nether_cave_mode_on_server = allow_nether_cave_mode_on_server;
                s.allow_radar_on_server = allow_radar_on_server;
                s
            }
            // ClientboundRulesPacket.write(1): not translated, it uses Minecraft's `FriendlyByteBuf`.
        }
        // ClientboundRulesPacket.read(1): not translated, it uses Minecraft's `FriendlyByteBuf`.
    }

    /// `ClientboundRulesPacket.ClientHandler` (xaero/hud/packet/basic/ClientboundRulesPacket.java)
    pub mod clientbound_rules_packet_client_handler {
        use super::*;
        // The class itself isn't translated: it implements `Consumer` (java.util.function). Its static parts are below.
        // ClientboundRulesPacket.ClientHandler.accept(1): not translated, its class isn't translated (it implements `Consumer` (java.util.function)).
    }

    /// `HandshakePacket` (xaero/hud/packet/basic/HandshakePacket.java)
    pub mod handshake_packet {
        use super::*;
        pub const NETWORK_COMPATIBILITY: i32 = 3;
        #[derive(Clone, Debug)]
        pub struct HandshakePacket {
            pub network_version: i32,
        }
        impl HandshakePacket {
            pub fn new_1(network_version: i32) -> Self {
                let mut s = Self { network_version: network_version };
                s.network_version = network_version;
                s
            }
            pub fn new() -> Self {
                let mut s = Self::new_1(3);
                s
            }
            // HandshakePacket.write(1): not translated, it uses Minecraft's `FriendlyByteBuf`.
        }
        // HandshakePacket.read(1): not translated, it uses Minecraft's `FriendlyByteBuf`.
    }

    /// `HandshakePacket.ClientHandler` (xaero/hud/packet/basic/HandshakePacket.java)
    pub mod handshake_packet_client_handler {
        use super::*;
        // The class itself isn't translated: it implements `Consumer` (java.util.function). Its static parts are below.
        // HandshakePacket.ClientHandler.accept(1): not translated, its class isn't translated (it implements `Consumer` (java.util.function)).
    }

    /// `HandshakePacket.ServerHandler` (xaero/hud/packet/basic/HandshakePacket.java)
    pub mod handshake_packet_server_handler {
        use super::*;
        // The class itself isn't translated: it implements `BiConsumer` (java.util.function). Its static parts are below.
        // HandshakePacket.ServerHandler.accept(2): not translated, its class isn't translated (it implements `BiConsumer` (java.util.function)).
    }

    /// `LevelMapPropertiesConsumer` (xaero/hud/packet/LevelMapPropertiesConsumer.java)
    pub mod level_map_properties_consumer {
        use super::*;
        // The class itself isn't translated: it implements `Consumer` (java.util.function). Its static parts are below.
        // LevelMapPropertiesConsumer.accept(1): not translated, its class isn't translated (it implements `Consumer` (java.util.function)).
    }

    /// `MinimapPacketRegister` (xaero/hud/packet/MinimapPacketRegister.java)
    pub mod minimap_packet_register {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MinimapPacketRegister {
        }
        impl MinimapPacketRegister {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // MinimapPacketRegister.register(1): not translated, it uses `IPacketHandler` (xaero.lib.common.packet).
        }
    }

    /// `ClientboundPlayerTrackerResetPacket` (xaero/hud/packet/tracker/ClientboundPlayerTrackerResetPacket.java)
    pub mod clientbound_player_tracker_reset_packet {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientboundPlayerTrackerResetPacket {
        }
        impl ClientboundPlayerTrackerResetPacket {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // ClientboundPlayerTrackerResetPacket.write(1): not translated, it uses Minecraft's `FriendlyByteBuf`.
        }
        // ClientboundPlayerTrackerResetPacket.read(1): not translated, it uses Minecraft's `FriendlyByteBuf`.
    }

    /// `ClientboundPlayerTrackerResetPacket.Handler` (xaero/hud/packet/tracker/ClientboundPlayerTrackerResetPacket.java)
    pub mod clientbound_player_tracker_reset_packet_handler {
        use super::*;
        // The class itself isn't translated: it implements `Consumer` (java.util.function). Its static parts are below.
        // ClientboundPlayerTrackerResetPacket.Handler.accept(1): not translated, its class isn't translated (it implements `Consumer` (java.util.function)).
    }

    /// `ClientboundTrackedPlayerPacket` (xaero/hud/packet/tracker/ClientboundTrackedPlayerPacket.java)
    pub mod clientbound_tracked_player_packet {
        use super::*;
        // The class itself isn't translated: it uses `UUID` (java.util). Its static parts are below.
        // ClientboundTrackedPlayerPacket.ClientboundTrackedPlayerPacket(7): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ClientboundTrackedPlayerPacket.write(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ClientboundTrackedPlayerPacket.read(1): not translated, it uses ClientboundTrackedPlayerPacket, which isn't translated (it uses `UUID` (java.util)).
    }

    /// `ClientboundTrackedPlayerPacket.Handler` (xaero/hud/packet/tracker/ClientboundTrackedPlayerPacket.java)
    pub mod clientbound_tracked_player_packet_handler {
        use super::*;
        // The class itself isn't translated: it implements `Consumer` (java.util.function). Its static parts are below.
        // ClientboundTrackedPlayerPacket.Handler.accept(1): not translated, its class isn't translated (it implements `Consumer` (java.util.function)).
    }

    /// `XaeroPath` (xaero/hud/path/XaeroPath.java)
    pub mod xaero_path {
        use super::*;
        // The class itself isn't translated: it holds an object of its own class (a linked structure). Its static parts are below.
        // XaeroPath.XaeroPath(4): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // XaeroPath.resolve(1): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // XaeroPath.resolve(1): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // XaeroPath.resolveSibling(1): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // XaeroPath.getSubPath(1): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // XaeroPath.getAtIndex(1): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // XaeroPath.isSubOf(1): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // XaeroPath.applyToFilePath(1): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // XaeroPath.toString(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // XaeroPath.getNodeCount(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // XaeroPath.getLastNode(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // XaeroPath.getParent(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // XaeroPath.getRoot(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // XaeroPath.equals(1): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // XaeroPath.hashCode(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // XaeroPath.root(1): not translated, it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)).
        // XaeroPath.root(2): not translated, it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)).
    }

    /// `XaeroPathReader` (xaero/hud/path/XaeroPathReader.java)
    pub mod xaero_path_reader {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct XaeroPathReader {
        }
        impl XaeroPathReader {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // XaeroPathReader.read(1): not translated, it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)).
            // XaeroPathReader.read(2): not translated, it uses XaeroPath, which isn't translated (it holds an object of its own class (a linked structure)).
        }
    }

    /// `IPresetAction` (xaero/hud/preset/action/IPresetAction.java)
    pub mod i_preset_action {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IPresetAction.apply(1): not translated, it has no body.
        // IPresetAction.confirm(1): not translated, it has no body.
        // IPresetAction.cancel(1): not translated, it has no body.
    }

    /// `PlacementPresetAction` (xaero/hud/preset/action/PlacementPresetAction.java)
    pub mod placement_preset_action {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // PlacementPresetAction.PlacementPresetAction(7): not translated, its class isn't translated (it's generic).
        // PlacementPresetAction.apply(1): not translated, its class isn't translated (it's generic).
        // PlacementPresetAction.confirm(1): not translated, its class isn't translated (it's generic).
        // PlacementPresetAction.cancel(1): not translated, its class isn't translated (it's generic).
    }

    /// `HudPreset` (xaero/hud/preset/HudPreset.java)
    pub mod hud_preset {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Component`. Its static parts are below.
        // HudPreset.HudPreset(3): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // HudPreset.getId(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // HudPreset.getName(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // HudPreset.apply(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // HudPreset.confirm(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // HudPreset.cancel(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // HudPreset.applyAndConfirm(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
    }

    /// `HudPreset.Builder` (xaero/hud/preset/HudPreset.java)
    pub mod hud_preset_builder {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Component`. Its static parts are below.
        // HudPreset.Builder.Builder(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // HudPreset.Builder.setDefault(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // HudPreset.Builder.setId(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // HudPreset.Builder.setName(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // HudPreset.Builder.addModulePreset(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // HudPreset.Builder.build(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // HudPreset.Builder.begin(0): not translated, it uses Builder, which isn't translated (it uses Minecraft's `Component`).
    }

    /// `HudPresetManager` (xaero/hud/preset/HudPresetManager.java)
    pub mod hud_preset_manager {
        use super::*;
        // The class itself isn't translated: it uses HudPreset, which isn't translated (it uses Minecraft's `Component`). Its static parts are below.
        // HudPresetManager.register(1): not translated, its class isn't translated (it uses HudPreset, which isn't translated (it uses Minecraft's `Component`)).
        // HudPresetManager.getPresets(0): not translated, its class isn't translated (it uses HudPreset, which isn't translated (it uses Minecraft's `Component`)).
    }

    /// `ModulePreset` (xaero/hud/preset/ModulePreset.java)
    pub mod module_preset {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ModulePreset.ModulePreset(2): not translated, its class isn't translated (it's generic).
        // ModulePreset.apply(0): not translated, its class isn't translated (it's generic).
        // ModulePreset.confirm(0): not translated, its class isn't translated (it's generic).
        // ModulePreset.cancel(0): not translated, its class isn't translated (it's generic).
        // ModulePreset.getModule(0): not translated, its class isn't translated (it's generic).
    }

    /// `ModulePreset.Builder` (xaero/hud/preset/ModulePreset.java)
    pub mod module_preset_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ModulePreset.Builder.Builder(1): not translated, its class isn't translated (it's generic).
        // ModulePreset.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // ModulePreset.Builder.setPlacement(7): not translated, its class isn't translated (it's generic).
        // ModulePreset.Builder.addAction(1): not translated, its class isn't translated (it's generic).
        // ModulePreset.Builder.build(0): not translated, its class isn't translated (it's generic).
        // ModulePreset.Builder.begin(1): not translated, it's generic.
    }

    /// `BossHealthPushBox` (xaero/hud/pushbox/boss/BossHealthPushBox.java)
    pub mod boss_health_push_box {
        use super::*;
        // The class itself isn't translated: it extends `PushBox` (xaero.hud.pushbox). Its static parts are below.
        // BossHealthPushBox.BossHealthPushBox(0): not translated, its class isn't translated (it extends `PushBox` (xaero.hud.pushbox)).
        // BossHealthPushBox.postUpdate(0): not translated, its class isn't translated (it extends `PushBox` (xaero.hud.pushbox)).
        // BossHealthPushBox.setLastBossHealthHeight(1): not translated, its class isn't translated (it extends `PushBox` (xaero.hud.pushbox)).
    }

    /// `BossHealthShiftPushBox` (xaero/hud/pushbox/boss/BossHealthShiftPushBox.java)
    pub mod boss_health_shift_push_box {
        use super::*;
        // The class itself isn't translated: it extends `FullHeightShiftPushBox` (xaero.hud.pushbox). Its static parts are below.
        // BossHealthShiftPushBox.BossHealthShiftPushBox(0): not translated, its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)).
        // BossHealthShiftPushBox.getShift(0): not translated, its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)).
        // BossHealthShiftPushBox.postUpdate(0): not translated, its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)).
        // BossHealthShiftPushBox.setLastBossHealthHeight(1): not translated, its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)).
    }

    /// `IBossHealthPushBox` (xaero/hud/pushbox/boss/IBossHealthPushBox.java)
    pub mod i_boss_health_push_box {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IBossHealthPushBox.setLastBossHealthHeight(1): not translated, it has no body.
        // IBossHealthPushBox.setActive(1): not translated, it has no body.
    }

    /// `BuiltInPushBoxes` (xaero/hud/pushbox/BuiltInPushBoxes.java)
    pub mod built_in_push_boxes {
        use super::*;
        // POTION_EFFECTS_PUSH_BOX: not translated, it uses PotionEffectsPushBox, which isn't translated (it extends `PushBox` (xaero.hud.pushbox)).
        // POTION_EFFECTS_SHIFT_PUSH_BOX: not translated, it uses PotionEffectsShiftPushBox, which isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)).
        // BOSS_HEALTH_PUSH_BOX: not translated, it uses BossHealthPushBox, which isn't translated (it extends `PushBox` (xaero.hud.pushbox)).
        // BOSS_HEALTH_SHIFT_PUSH_BOX: not translated, it uses BossHealthShiftPushBox, which isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)).
        #[derive(Clone, Debug)]
        pub struct BuiltInPushBoxes {
        }
        impl BuiltInPushBoxes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BuiltInPushBoxes.getPotionEffectPushBox(1): not translated, it uses IPotionEffectsPushBox, which isn't translated (it's an interface).
        // BuiltInPushBoxes.getBossHealthPushBox(1): not translated, it uses IBossHealthPushBox, which isn't translated (it's an interface).
        // BuiltInPushBoxes.addAll(1): not translated, it uses BuiltInPushBoxes.POTION_EFFECTS_PUSH_BOX, which isn't translated (it uses PotionEffectsPushBox, which isn't translated (it extends `PushBox` (xaero.hud.pushbox))).
    }

    /// `IPotionEffectsPushBox` (xaero/hud/pushbox/effect/IPotionEffectsPushBox.java)
    pub mod i_potion_effects_push_box {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IPotionEffectsPushBox.setHasNegative(1): not translated, it has no body.
        // IPotionEffectsPushBox.setActive(1): not translated, it has no body.
    }

    /// `PotionEffectsPushBox` (xaero/hud/pushbox/effect/PotionEffectsPushBox.java)
    pub mod potion_effects_push_box {
        use super::*;
        // The class itself isn't translated: it extends `PushBox` (xaero.hud.pushbox). Its static parts are below.
        // PotionEffectsPushBox.PotionEffectsPushBox(0): not translated, its class isn't translated (it extends `PushBox` (xaero.hud.pushbox)).
        // PotionEffectsPushBox.getX(2): not translated, its class isn't translated (it extends `PushBox` (xaero.hud.pushbox)).
        // PotionEffectsPushBox.update(0): not translated, its class isn't translated (it extends `PushBox` (xaero.hud.pushbox)).
        // PotionEffectsPushBox.postUpdate(0): not translated, its class isn't translated (it extends `PushBox` (xaero.hud.pushbox)).
        // PotionEffectsPushBox.calculatePotionDisplayWidth(1): not translated, it uses IPotionEffectsPushBox, which isn't translated (it's an interface).
        // PotionEffectsPushBox.setHasNegative(1): not translated, its class isn't translated (it extends `PushBox` (xaero.hud.pushbox)).
    }

    /// `PotionEffectsShiftPushBox` (xaero/hud/pushbox/effect/PotionEffectsShiftPushBox.java)
    pub mod potion_effects_shift_push_box {
        use super::*;
        // The class itself isn't translated: it extends `FullHeightShiftPushBox` (xaero.hud.pushbox). Its static parts are below.
        // PotionEffectsShiftPushBox.PotionEffectsShiftPushBox(0): not translated, its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)).
        // PotionEffectsShiftPushBox.getX(2): not translated, its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)).
        // PotionEffectsShiftPushBox.getShift(0): not translated, its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)).
        // PotionEffectsShiftPushBox.update(0): not translated, its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)).
        // PotionEffectsShiftPushBox.postUpdate(0): not translated, its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)).
        // PotionEffectsShiftPushBox.setHasNegative(1): not translated, its class isn't translated (it extends `FullHeightShiftPushBox` (xaero.hud.pushbox)).
    }

    /// `FullHeightShiftPushBox` (xaero/hud/pushbox/FullHeightShiftPushBox.java)
    pub mod full_height_shift_push_box {
        use super::*;
        // The class itself isn't translated: it extends `PushBox`. Its static parts are below.
        // FullHeightShiftPushBox.FullHeightShiftPushBox(3): not translated, its class isn't translated (it extends `PushBox`).
        // FullHeightShiftPushBox.update(0): not translated, its class isn't translated (it extends `PushBox`).
        // FullHeightShiftPushBox.getH(2): not translated, its class isn't translated (it extends `PushBox`).
        // FullHeightShiftPushBox.push(3): not translated, its class isn't translated (it extends `PushBox`).
        // FullHeightShiftPushBox.getShift(0): not translated, it has no body.
    }

    /// `PushBox` (xaero/hud/pushbox/PushBox.java)
    pub mod push_box {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PushBox {
            pub x: i32,
            pub y: i32,
            pub w: i32,
            pub h: i32,
            pub anchor_x: f32,
            pub anchor_y: f32,
            pub active: bool,
            pub vertical_bias: i32,
        }
        impl PushBox {
            pub fn new(x: i32, y: i32, w: i32, h: i32, anchor_x: f32, anchor_y: f32, vertical_bias: i32) -> Self {
                let mut s = Self { x: x, y: y, w: w, h: h, anchor_x: anchor_x, anchor_y: anchor_y, active: false, vertical_bias: vertical_bias };
                s.x = x;
                s.y = y;
                s.w = w;
                s.h = h;
                s.anchor_x = anchor_x;
                s.anchor_y = anchor_y;
                s.vertical_bias = vertical_bias;
                s
            }
            pub fn is_active(&self) -> bool {
                return self.active;
            }
            pub fn set_active(&mut self, active: bool) {
                self.active = active;
            }
            pub fn get_x(&self, width: i32, height: i32) -> i32 {
                return self.x;
            }
            pub fn get_y(&self, width: i32, height: i32) -> i32 {
                return self.y;
            }
            pub fn get_w(&self, width: i32, height: i32) -> i32 {
                return self.w;
            }
            pub fn get_h(&self, width: i32, height: i32) -> i32 {
                return self.h;
            }
            pub fn get_anchor_x(&self) -> f32 {
                return self.anchor_x;
            }
            pub fn get_anchor_y(&self) -> f32 {
                return self.anchor_y;
            }
            pub fn get_vertical_bias(&self) -> i32 {
                return self.vertical_bias;
            }
            pub fn update(&self) {
            }
            pub fn post_update(&self) {
            }
            pub fn push(&self, state: &mut pushbox_handler_state::State, push_x: i32, push_y: i32) {
                state.x += push_x;
                state.y += push_y;
            }
        }
    }

    /// `PushboxHandler` (xaero/hud/pushbox/PushboxHandler.java)
    pub mod pushbox_handler {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PushboxHandler {
        }
        impl PushboxHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // PushboxHandler.applyPushboxes(5): not translated, it uses PushboxManager, which isn't translated (a Map or Set keyed by a PushBox).
            // PushboxHandler.updateAll(1): not translated, it uses PushboxManager, which isn't translated (a Map or Set keyed by a PushBox).
            // PushboxHandler.postUpdateAll(1): not translated, it uses PushboxManager, which isn't translated (a Map or Set keyed by a PushBox).
            pub fn apply_screen_edges(&self, state: &mut pushbox_handler_state::State, screen_width: i32, screen_height: i32, screen_scale: f64) {
                if (state.x + state.w) > screen_width {
                    state.x = screen_width - state.w;
                }
                if (state.y + state.h) > screen_height {
                    state.y = screen_height - state.h;
                }
                if state.x < 0 {
                    state.x = 0;
                }
                if state.y < 0 {
                    state.y = 0;
                }
            }
        }
    }

    /// `PushboxHandler.State` (xaero/hud/pushbox/PushboxHandler.java)
    pub mod pushbox_handler_state {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct State {
            pub x: i32,
            pub y: i32,
            pub w: i32,
            pub h: i32,
        }
        impl State {
            pub fn new() -> Self {
                let mut s = Self { x: 0i32, y: 0i32, w: 0i32, h: 0i32 };
                s
            }
            // PushboxHandler.State.resetForModule(4): not translated, it uses ModuleSession, which isn't translated (it's generic).
        }
    }

    /// `PushboxManager` (xaero/hud/pushbox/PushboxManager.java)
    pub mod pushbox_manager {
        use super::*;
        // The class itself isn't translated: a Map or Set keyed by a PushBox. Its static parts are below.
        // PushboxManager.add(1): not translated, its class isn't translated (a Map or Set keyed by a PushBox).
        // PushboxManager.getAll(0): not translated, its class isn't translated (a Map or Set keyed by a PushBox).
    }

    /// `HudRenderer` (xaero/hud/render/HudRenderer.java)
    pub mod hud_renderer {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct HudRenderer {
            pub pushbox_handler: pushbox_handler::PushboxHandler,
        }
        impl HudRenderer {
            pub fn new(pushbox_handler: &pushbox_handler::PushboxHandler) -> Self {
                let mut s = Self { pushbox_handler: pushbox_handler.clone() };
                s.pushbox_handler = pushbox_handler.clone();
                s
            }
            // HudRenderer.render(3): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
            // HudRenderer.renderModule(5): not translated, it's generic.
            pub fn get_pushbox_handler(&self) -> pushbox_handler::PushboxHandler {
                return self.pushbox_handler.clone();
            }
        }
    }

    /// `IModuleRenderer` (xaero/hud/render/module/IModuleRenderer.java)
    pub mod i_module_renderer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IModuleRenderer.render(4): not translated, it has no body.
    }

    /// `ModuleRenderContext` (xaero/hud/render/module/ModuleRenderContext.java)
    pub mod module_render_context {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ModuleRenderContext {
            pub x: i32,
            pub y: i32,
            pub w: i32,
            pub h: i32,
            pub flipped_vertically: bool,
            pub flipped_horizontally: bool,
            pub screen_width: i32,
            pub screen_height: i32,
            pub screen_scale: f64,
        }
        impl ModuleRenderContext {
            pub fn new(screen_width: i32, screen_height: i32, screen_scale: f64) -> Self {
                let mut s = Self { x: 0i32, y: 0i32, w: 0i32, h: 0i32, flipped_vertically: false, flipped_horizontally: false, screen_width: screen_width, screen_height: screen_height, screen_scale: screen_scale };
                s.screen_width = screen_width;
                s.screen_height = screen_height;
                s.screen_scale = screen_scale;
                s
            }
        }
    }

    /// `RenderPipelineWrapper` (xaero/hud/render/pipeline/RenderPipelineWrapper.java)
    pub mod render_pipeline_wrapper {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RenderPipeline`. Its static parts are below.
        // RenderPipelineWrapper.RenderPipelineWrapper(2): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.getDepthStencilState(0): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.getPolygonMode(0): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.isCull(0): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.getColorTargetStates(0): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.getVertexFormatBindings(0): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.getVertexFormatBinding(1): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.getPrimitiveTopology(0): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.getShaders(0): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.getShaderDefines(0): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.getBindGroupLayouts(0): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.pushConstantSize(0): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.getSortKey(0): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.wantsDepthTexture(0): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.setShaderOverrides(1): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.setShaderOverride(2): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.setShaderDefinesOverride(1): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.setBindGroupLayoutOverride(1): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.setDepthStencilStateOverride(1): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.setPolygonModeOverride(1): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.setCullOverride(1): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.setColorTargetStateOverride(2): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.resetColorTargetStateOverrides(0): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.setVertexFormatOverride(2): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.resetVertexFormatOverrides(0): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.setPrimitiveTopologyOverride(1): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.setPushConstantSizeOverride(1): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
        // RenderPipelineWrapper.setSortKeyOverride(1): not translated, its class isn't translated (it extends Minecraft's `RenderPipeline`).
    }

    /// `TextureLocations` (xaero/hud/render/TextureLocations.java)
    pub mod texture_locations {
        use super::*;
        pub static GUI_TEXTURES: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("xaerobetterpvp", "gui/guis.png"));
        pub static MINIMAP_FRAME_TEXTURES: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("xaerobetterpvp", "gui/minimap_frame.png"));
        pub static VANILLA_GUI_ICONS_LOCATION: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::parse("textures/gui/icons.png"));
        #[derive(Clone, Debug)]
        pub struct TextureLocations {
        }
        impl TextureLocations {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `MultiTextureRenderUtil` (xaero/hud/render/util/MultiTextureRenderUtil.java)
    pub mod multi_texture_render_util {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MultiTextureRenderUtil {
        }
        impl MultiTextureRenderUtil {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MultiTextureRenderUtil.prepareTexturedColoredRect(15): not translated, it's overloaded with the same number of parameters.
        // MultiTextureRenderUtil.prepareTexturedColoredRect(15): not translated, it's overloaded with the same number of parameters.
        // MultiTextureRenderUtil.prepareTexturedRect(11): not translated, it uses Minecraft's `GpuTextureView`.
    }

    /// `RenderBufferUtil` (xaero/hud/render/util/RenderBufferUtil.java)
    pub mod render_buffer_util {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct RenderBufferUtil {
        }
        impl RenderBufferUtil {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RenderBufferUtil.addColoredRect(7): not translated, it uses Minecraft's `VertexConsumer`.
        // RenderBufferUtil.addColoredRect(10): not translated, it uses Minecraft's `VertexConsumer`.
        // RenderBufferUtil.addTexturedColoredRect(13): not translated, it uses Minecraft's `VertexConsumer`.
        // RenderBufferUtil.addTexturedColoredRect(15): not translated, it uses Minecraft's `VertexConsumer`.
    }

    /// `SeparatedKeysParser` (xaero/hud/util/SeparatedKeysParser.java)
    pub mod separated_keys_parser {
        use super::*;
        // The class itself isn't translated: a function kept in a variable or field (only parameters can be). Its static parts are below.
        // SeparatedKeysParser.SeparatedKeysParser(1): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // SeparatedKeysParser.parseKeys(1): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // SeparatedKeysParser.parseKeyUntilChar(4): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // SeparatedKeysParser.throwError(3): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
    }

    /// `MinimapSettingsFunction` (xaero/hud/xminimap/controls/key/function/MinimapSettingsFunction.java)
    pub mod minimap_settings_function {
        use super::*;
        // The class itself isn't translated: it extends `KeyMappingFunction` (xaero.hud.controls.key.function). Its static parts are below.
        // MinimapSettingsFunction.MinimapSettingsFunction(1): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // MinimapSettingsFunction.onPress(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // MinimapSettingsFunction.onRelease(0): not translated, its class isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
    }

    /// `XMinimapKeyMappingFunctions` (xaero/hud/xminimap/controls/key/function/XMinimapKeyMappingFunctions.java)
    pub mod x_minimap_key_mapping_functions {
        use super::*;
        // SETTINGS: not translated, it creates a MinimapSettingsFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        // SERVER_PROFILES: not translated, it creates a MinimapSettingsFunction, which isn't translated (it extends `KeyMappingFunction` (xaero.hud.controls.key.function)).
        #[derive(Clone, Debug)]
        pub struct XMinimapKeyMappingFunctions {
        }
        impl XMinimapKeyMappingFunctions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // XMinimapKeyMappingFunctions.registerAll(1): not translated, it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`).
    }

    /// `XMinimapKeyMappings` (xaero/hud/xminimap/controls/key/XMinimapKeyMappings.java)
    pub mod x_minimap_key_mappings {
        use super::*;
        // SETTINGS: not translated, it uses Minecraft's `KeyMapping`.
        // SERVER_PROFILES: not translated, it uses Minecraft's `KeyMapping`.
        #[derive(Clone, Debug)]
        pub struct XMinimapKeyMappings {
        }
        impl XMinimapKeyMappings {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // XMinimapKeyMappings.registerAll(3): not translated, it uses KeyMappingControllerManager, which isn't translated (it implements `Iterable`).
    }

    /// `XMinimapControlsRegister` (xaero/hud/xminimap/controls/XMinimapControlsRegister.java)
    pub mod x_minimap_controls_register {
        use super::*;
        // The class itself isn't translated: it extends `ControlsRegister` (xaero.hud.controls). Its static parts are below.
        // XMinimapControlsRegister.registerKeybindings(2): not translated, its class isn't translated (it extends `ControlsRegister` (xaero.hud.controls)).
    }

    /// `BuiltInHudPresets` (xaero/hud/xminimap/preset/BuiltInHudPresets.java)
    pub mod built_in_hud_presets {
        use super::*;
        // TOP_LEFT: not translated, it uses HudPreset, which isn't translated (it uses Minecraft's `Component`).
        // TOP_RIGHT: not translated, it uses HudPreset, which isn't translated (it uses Minecraft's `Component`).
        // BOTTOM_LEFT: not translated, it uses HudPreset, which isn't translated (it uses Minecraft's `Component`).
        // BOTTOM_RIGHT: not translated, it uses HudPreset, which isn't translated (it uses Minecraft's `Component`).
        #[derive(Clone, Debug)]
        pub struct BuiltInHudPresets {
        }
        impl BuiltInHudPresets {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BuiltInHudPresets.addAll(1): not translated, it uses HudPresetManager, which isn't translated (it uses HudPreset, which isn't translated (it uses Minecraft's `Component`)).
    }

    /// `MinimapGuiHelper` (xaero/minimap/gui/MinimapGuiHelper.java)
    pub mod minimap_gui_helper {
        use super::*;
        // The class itself isn't translated: it extends `GuiHelper` (xaero.common.gui). Its static parts are below.
        // MinimapGuiHelper.MinimapGuiHelper(1): not translated, its class isn't translated (it extends `GuiHelper` (xaero.common.gui)).
        // MinimapGuiHelper.getMainSettingsScreen(1): not translated, its class isn't translated (it extends `GuiHelper` (xaero.common.gui)).
        // MinimapGuiHelper.openMainSettingsFromScreen(1): not translated, its class isn't translated (it extends `GuiHelper` (xaero.common.gui)).
    }

    /// `MinimapClientOnly` (xaero/minimap/MinimapClientOnly.java)
    pub mod minimap_client_only {
        use super::*;
        // The class itself isn't translated: it extends `HudClientOnlyBase` (xaero.common). Its static parts are below.
        // MinimapClientOnly.preInit(2): not translated, its class isn't translated (it extends `HudClientOnlyBase` (xaero.common)).
        // MinimapClientOnly.preLoadLater(1): not translated, its class isn't translated (it extends `HudClientOnlyBase` (xaero.common)).
    }

    /// `XaeroMinimapSupportMods` (xaero/minimap/mods/XaeroMinimapSupportMods.java)
    pub mod xaero_minimap_support_mods {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct XaeroMinimapSupportMods {
        }
        impl XaeroMinimapSupportMods {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // XaeroMinimapSupportMods.checkForMinimapDuplicates(0): not translated, it calls SupportMods.checkForMinimapDuplicates, which isn't translated.
    }

    /// `XaeroMinimap` (xaero/minimap/XaeroMinimap.java)
    pub mod xaero_minimap {
        use super::*;
        // The class itself isn't translated: it extends `HudMod` (xaero.common). Its static parts are below.
        // LOGGER: the logger. Log calls become comments.
        // instance: not translated, it uses XaeroMinimap, which isn't translated (it extends `HudMod` (xaero.common)).
        pub const MOD_ID: &str = "xaerominimap";
        // XaeroMinimap.XaeroMinimap(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.loadClient(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.loadCommon(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.getCommonConfigFileName(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.getModId(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.createModSettings(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.createGuiHelper(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.getOldConfigFileName(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.createClientOnly(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.getModName(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.getLogger(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.createForgeEventHandlerListener(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.getVersionsURL(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.getUpdateLink(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.createControlsRegister(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.createSession(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.getSettingsKey(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.getServerSettingsKey(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.getConfigSubFolder(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.getDefaultConfigsSubFolder(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.createConfigCommonRegistryHandler(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.createConfigClientRegistryHandlerSupplier(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
        // XaeroMinimap.isStandalone(0): not translated, its class isn't translated (it extends `HudMod` (xaero.common)).
    }

    /// `XaeroMinimapFabric` (xaero/minimap/XaeroMinimapFabric.java)
    pub mod xaero_minimap_fabric {
        use super::*;
        // The class itself isn't translated: it extends `XaeroMinimap`. Its static parts are below.
        pub static FILE_LAYOUT_ID: LazyLock<JCell<String>> = LazyLock::new(|| JCell::new("minimap_fabric".to_string()));
        // XaeroMinimapFabric.onInitializeClient(0): not translated, its class isn't translated (it extends `XaeroMinimap`).
        // XaeroMinimapFabric.onInitializeServer(0): not translated, its class isn't translated (it extends `XaeroMinimap`).
        // XaeroMinimapFabric.loadClient(0): not translated, its class isn't translated (it extends `XaeroMinimap`).
        // XaeroMinimapFabric.loadCommon(0): not translated, its class isn't translated (it extends `XaeroMinimap`).
        // XaeroMinimapFabric.loadServer(0): not translated, its class isn't translated (it extends `XaeroMinimap`).
        // XaeroMinimapFabric.tryLoadLater(0): not translated, its class isn't translated (it extends `XaeroMinimap`).
        // XaeroMinimapFabric.tryLoadLaterServer(0): not translated, its class isn't translated (it extends `XaeroMinimap`).
        // XaeroMinimapFabric.createPlatformContext(0): not translated, its class isn't translated (it extends `XaeroMinimap`).
        // XaeroMinimapFabric.getFileLayoutID(0): not translated, its class isn't translated (it extends `XaeroMinimap`).
    }

    /// `XaeroMinimapStandaloneSession` (xaero/minimap/XaeroMinimapStandaloneSession.java)
    pub mod xaero_minimap_standalone_session {
        use super::*;
        // The class itself isn't translated: it extends `XaeroMinimapSession` (xaero.common). Its static parts are below.
        // XaeroMinimapStandaloneSession.XaeroMinimapStandaloneSession(1): not translated, its class isn't translated (it extends `XaeroMinimapSession` (xaero.common)).
        // XaeroMinimapStandaloneSession.init(1): not translated, its class isn't translated (it extends `XaeroMinimapSession` (xaero.common)).
    }

    /// JOML's Vector3f as a value (Java's methods change the vector; the translation writes the result back).
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct JVec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
    impl JVec3 {
        pub const fn new(x: f32, y: f32, z: f32) -> Self {
            Self { x, y, z }
        }
        pub fn add(self, o: JVec3) -> Self {
            Self::new(self.x + o.x, self.y + o.y, self.z + o.z)
        }
        pub fn add3(self, x: f32, y: f32, z: f32) -> Self {
            Self::new(self.x + x, self.y + y, self.z + z)
        }
        pub fn sub(self, o: JVec3) -> Self {
            Self::new(self.x - o.x, self.y - o.y, self.z - o.z)
        }
        pub fn mul_s(self, s: f32) -> Self {
            Self::new(self.x * s, self.y * s, self.z * s)
        }
        pub fn mul_v(self, o: JVec3) -> Self {
            Self::new(self.x * o.x, self.y * o.y, self.z * o.z)
        }
        pub fn negate(self) -> Self {
            Self::new(-self.x, -self.y, -self.z)
        }
        pub fn dot(self, o: JVec3) -> f32 {
            self.x * o.x + self.y * o.y + self.z * o.z
        }
        pub fn length(self) -> f32 {
            self.dot(self).sqrt()
        }
        pub fn length_squared(self) -> f32 {
            self.dot(self)
        }
        /// JOML's normalize: divides by the length (a zero vector becomes NaNs, as in Java).
        pub fn normalize(self) -> Self {
            let inv = 1.0 / self.length();
            self.mul_s(inv)
        }
        pub fn cross(self, o: JVec3) -> Self {
            Self::new(self.y * o.z - self.z * o.y, self.z * o.x - self.x * o.z, self.x * o.y - self.y * o.x)
        }
        /// JOML's lerp: this + (other - this) * t.
        pub fn lerp(self, o: JVec3, t: f32) -> Self {
            Self::new(self.x + (o.x - self.x) * t, self.y + (o.y - self.y) * t, self.z + (o.z - self.z) * t)
        }
        pub fn distance(self, o: JVec3) -> f32 {
            self.sub(o).length()
        }
    }
    
    /// JOML's Quaternionf (x, y, z, w).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct JQuat {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }
    impl Default for JQuat {
        fn default() -> Self {
            Self::IDENTITY
        }
    }
    impl JQuat {
        pub const IDENTITY: JQuat = JQuat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
            Self { x, y, z, w }
        }
        /// `rotationX(angle)`: a turn about +X by `angle` radians (Axis.XP.rotation).
        pub fn rotation_x(a: f32) -> Self {
            let (s, c) = (a * 0.5).sin_cos();
            Self::new(s, 0.0, 0.0, c)
        }
        pub fn rotation_y(a: f32) -> Self {
            let (s, c) = (a * 0.5).sin_cos();
            Self::new(0.0, s, 0.0, c)
        }
        pub fn rotation_z(a: f32) -> Self {
            let (s, c) = (a * 0.5).sin_cos();
            Self::new(0.0, 0.0, s, c)
        }
        pub fn normalize(self) -> Self {
            let inv = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
            Self::new(self.x * inv, self.y * inv, self.z * inv, self.w * inv)
        }
        /// this * q.
        pub fn mul(self, q: JQuat) -> Self {
            Self::new(
                self.w * q.x + self.x * q.w + self.y * q.z - self.z * q.y,
                self.w * q.y - self.x * q.z + self.y * q.w + self.z * q.x,
                self.w * q.z + self.x * q.y - self.y * q.x + self.z * q.w,
                self.w * q.w - self.x * q.x - self.y * q.y - self.z * q.z,
            )
        }
        pub fn rotate_x(self, a: f32) -> Self {
            self.mul(Self::rotation_x(a))
        }
        pub fn rotate_y(self, a: f32) -> Self {
            self.mul(Self::rotation_y(a))
        }
        pub fn rotate_z(self, a: f32) -> Self {
            self.mul(Self::rotation_z(a))
        }
    }
    
    /// JOML's Matrix3f, column-major, from three column vectors.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct JMat3 {
        pub m: [f32; 9],
    }
    impl JMat3 {
        pub fn from_cols(c0: JVec3, c1: JVec3, c2: JVec3) -> Self {
            Self { m: [c0.x, c0.y, c0.z, c1.x, c1.y, c1.z, c2.x, c2.y, c2.z] }
        }
    }
    
    /// JOML's Matrix4f: column-major, `m[col * 4 + row]`; `a.mul(b)` is a * b, and translate/rotate/scale
    /// multiply on the right, as JOML does.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct JMat4 {
        pub m: [f32; 16],
    }
    impl Default for JMat4 {
        fn default() -> Self {
            Self::IDENTITY
        }
    }
    impl JMat4 {
        pub const IDENTITY: JMat4 = JMat4 { m: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0] };
        pub fn identity() -> Self {
            Self::IDENTITY
        }
        fn at(&self, row: usize, col: usize) -> f32 {
            self.m[col * 4 + row]
        }
        pub fn mul(self, r: JMat4) -> Self {
            let mut out = [0.0f32; 16];
            for col in 0..4 {
                for row in 0..4 {
                    let mut v = 0.0;
                    for k in 0..4 {
                        v += self.at(row, k) * r.at(k, col);
                    }
                    out[col * 4 + row] = v;
                }
            }
            Self { m: out }
        }
        pub fn translation(x: f32, y: f32, z: f32) -> Self {
            let mut t = Self::IDENTITY;
            t.m[12] = x;
            t.m[13] = y;
            t.m[14] = z;
            t
        }
        pub fn translate(self, x: f32, y: f32, z: f32) -> Self {
            self.mul(Self::translation(x, y, z))
        }
        pub fn translate_v(self, v: JVec3) -> Self {
            self.translate(v.x, v.y, v.z)
        }
        pub fn scaling(x: f32, y: f32, z: f32) -> Self {
            let mut t = Self::IDENTITY;
            t.m[0] = x;
            t.m[5] = y;
            t.m[10] = z;
            t
        }
        pub fn scale(self, x: f32, y: f32, z: f32) -> Self {
            self.mul(Self::scaling(x, y, z))
        }
        pub fn scale_s(self, s: f32) -> Self {
            self.scale(s, s, s)
        }
        /// The rotation a unit quaternion stands for.
        pub fn rotation(q: JQuat) -> Self {
            let (x, y, z, w) = (q.x, q.y, q.z, q.w);
            let (xx, yy, zz) = (x * x, y * y, z * z);
            let (xy, xz, yz, wx, wy, wz) = (x * y, x * z, y * z, w * x, w * y, w * z);
            Self {
                m: [
                    1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0,
                    2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx), 0.0,
                    2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy), 0.0,
                    0.0, 0.0, 0.0, 1.0,
                ],
            }
        }
        pub fn rotate(self, q: JQuat) -> Self {
            self.mul(Self::rotation(q))
        }
        pub fn rotate_x(self, a: f32) -> Self {
            self.rotate(JQuat::rotation_x(a))
        }
        pub fn rotate_y(self, a: f32) -> Self {
            self.rotate(JQuat::rotation_y(a))
        }
        pub fn rotate_z(self, a: f32) -> Self {
            self.rotate(JQuat::rotation_z(a))
        }
        /// JOML's rotateXYZ: about X, then Y, then Z.
        pub fn rotate_xyz(self, x: f32, y: f32, z: f32) -> Self {
            self.rotate_x(x).rotate_y(y).rotate_z(z)
        }
        /// JOML's rotateAround(q, ox, oy, oz): about the point (ox, oy, oz).
        pub fn rotate_around(self, q: JQuat, ox: f32, oy: f32, oz: f32) -> Self {
            self.translate(ox, oy, oz).rotate(q).translate(-ox, -oy, -oz)
        }
        /// The general inverse (JOML's invert); a singular matrix gives infinities, as in Java.
        pub fn invert(self) -> Self {
            let a = &self.m;
            let mut inv = [0.0f32; 16];
            inv[0] = a[5] * a[10] * a[15] - a[5] * a[11] * a[14] - a[9] * a[6] * a[15] + a[9] * a[7] * a[14] + a[13] * a[6] * a[11] - a[13] * a[7] * a[10];
            inv[4] = -a[4] * a[10] * a[15] + a[4] * a[11] * a[14] + a[8] * a[6] * a[15] - a[8] * a[7] * a[14] - a[12] * a[6] * a[11] + a[12] * a[7] * a[10];
            inv[8] = a[4] * a[9] * a[15] - a[4] * a[11] * a[13] - a[8] * a[5] * a[15] + a[8] * a[7] * a[13] + a[12] * a[5] * a[11] - a[12] * a[7] * a[9];
            inv[12] = -a[4] * a[9] * a[14] + a[4] * a[10] * a[13] + a[8] * a[5] * a[14] - a[8] * a[6] * a[13] - a[12] * a[5] * a[10] + a[12] * a[6] * a[9];
            inv[1] = -a[1] * a[10] * a[15] + a[1] * a[11] * a[14] + a[9] * a[2] * a[15] - a[9] * a[3] * a[14] - a[13] * a[2] * a[11] + a[13] * a[3] * a[10];
            inv[5] = a[0] * a[10] * a[15] - a[0] * a[11] * a[14] - a[8] * a[2] * a[15] + a[8] * a[3] * a[14] + a[12] * a[2] * a[11] - a[12] * a[3] * a[10];
            inv[9] = -a[0] * a[9] * a[15] + a[0] * a[11] * a[13] + a[8] * a[1] * a[15] - a[8] * a[3] * a[13] - a[12] * a[1] * a[11] + a[12] * a[3] * a[9];
            inv[13] = a[0] * a[9] * a[14] - a[0] * a[10] * a[13] - a[8] * a[1] * a[14] + a[8] * a[2] * a[13] + a[12] * a[1] * a[10] - a[12] * a[2] * a[9];
            inv[2] = a[1] * a[6] * a[15] - a[1] * a[7] * a[14] - a[5] * a[2] * a[15] + a[5] * a[3] * a[14] + a[13] * a[2] * a[7] - a[13] * a[3] * a[6];
            inv[6] = -a[0] * a[6] * a[15] + a[0] * a[7] * a[14] + a[4] * a[2] * a[15] - a[4] * a[3] * a[14] - a[12] * a[2] * a[7] + a[12] * a[3] * a[6];
            inv[10] = a[0] * a[5] * a[15] - a[0] * a[7] * a[13] - a[4] * a[1] * a[15] + a[4] * a[3] * a[13] + a[12] * a[1] * a[7] - a[12] * a[3] * a[5];
            inv[14] = -a[0] * a[5] * a[14] + a[0] * a[6] * a[13] + a[4] * a[1] * a[14] - a[4] * a[2] * a[13] - a[12] * a[1] * a[6] + a[12] * a[2] * a[5];
            inv[3] = -a[1] * a[6] * a[11] + a[1] * a[7] * a[10] + a[5] * a[2] * a[11] - a[5] * a[3] * a[10] - a[9] * a[2] * a[7] + a[9] * a[3] * a[6];
            inv[7] = a[0] * a[6] * a[11] - a[0] * a[7] * a[10] - a[4] * a[2] * a[11] + a[4] * a[3] * a[10] + a[8] * a[2] * a[7] - a[8] * a[3] * a[6];
            inv[11] = -a[0] * a[5] * a[11] + a[0] * a[7] * a[9] + a[4] * a[1] * a[11] - a[4] * a[3] * a[9] - a[8] * a[1] * a[7] + a[8] * a[3] * a[5];
            inv[15] = a[0] * a[5] * a[10] - a[0] * a[6] * a[9] - a[4] * a[1] * a[10] + a[4] * a[2] * a[9] + a[8] * a[1] * a[6] - a[8] * a[2] * a[5];
            let det = a[0] * inv[0] + a[1] * inv[4] + a[2] * inv[8] + a[3] * inv[12];
            let d = 1.0 / det;
            for v in inv.iter_mut() {
                *v *= d;
            }
            Self { m: inv }
        }
        /// The point v moved by this matrix (w = 1).
        pub fn transform_position(&self, v: JVec3) -> JVec3 {
            JVec3::new(
                self.at(0, 0) * v.x + self.at(0, 1) * v.y + self.at(0, 2) * v.z + self.at(0, 3),
                self.at(1, 0) * v.x + self.at(1, 1) * v.y + self.at(1, 2) * v.z + self.at(1, 3),
                self.at(2, 0) * v.x + self.at(2, 1) * v.y + self.at(2, 2) * v.z + self.at(2, 3),
            )
        }
        /// The direction v turned by this matrix (w = 0).
        pub fn transform_direction(&self, v: JVec3) -> JVec3 {
            JVec3::new(
                self.at(0, 0) * v.x + self.at(0, 1) * v.y + self.at(0, 2) * v.z,
                self.at(1, 0) * v.x + self.at(1, 1) * v.y + self.at(1, 2) * v.z,
                self.at(2, 0) * v.x + self.at(2, 1) * v.y + self.at(2, 2) * v.z,
            )
        }
        /// JOML's set3x3: the upper-left 3x3 from `m`, the rest kept.
        pub fn set3x3(self, m: JMat3) -> Self {
            let mut out = self;
            for col in 0..3 {
                for row in 0..3 {
                    out.m[col * 4 + row] = m.m[col * 3 + row];
                }
            }
            out
        }
        /// JOML's transpose3x3: the upper-left 3x3 transposed, the rest kept.
        pub fn transpose3x3(self) -> Self {
            let mut out = self;
            for col in 0..3 {
                for row in 0..3 {
                    out.m[col * 4 + row] = self.m[row * 4 + col];
                }
            }
            out
        }
        pub fn get_translation(&self) -> JVec3 {
            JVec3::new(self.m[12], self.m[13], self.m[14])
        }
    }
    
    /// Minecraft's PoseStack: a stack of matrices (normals aren't kept: Horizon's hand renderer takes one matrix).
    #[derive(Clone, Debug)]
    pub struct JPoseStack {
        pub stack: Vec<JMat4>,
    }
    impl Default for JPoseStack {
        fn default() -> Self {
            Self { stack: vec![JMat4::IDENTITY] }
        }
    }
    impl JPoseStack {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn of(m: JMat4) -> Self {
            Self { stack: vec![m] }
        }
        fn top(&mut self) -> &mut JMat4 {
            self.stack.last_mut().expect("PoseStack is empty")
        }
        pub fn push_pose(&mut self) {
            let m = *self.top();
            self.stack.push(m);
        }
        pub fn pop_pose(&mut self) {
            self.stack.pop();
        }
        /// Writes the top matrix (Java changed `pose.last().pose()` in place).
        pub fn set_last(&mut self, m: JMat4) {
            *self.top() = m;
        }
        pub fn last_pose(&self) -> JMat4 {
            *self.stack.last().expect("PoseStack is empty")
        }
        pub fn set_identity(&mut self) {
            *self.top() = JMat4::IDENTITY;
        }
        pub fn translate(&mut self, x: f32, y: f32, z: f32) {
            let m = self.top();
            *m = m.translate(x, y, z);
        }
        pub fn scale(&mut self, x: f32, y: f32, z: f32) {
            let m = self.top();
            *m = m.scale(x, y, z);
        }
        pub fn mul_pose(&mut self, q: JQuat) {
            let m = self.top();
            *m = m.rotate(q);
        }
        pub fn mul_pose_matrix(&mut self, r: JMat4) {
            let m = self.top();
            *m = m.mul(r);
        }
        pub fn rotate_around(&mut self, q: JQuat, x: f32, y: f32, z: f32) {
            let m = self.top();
            *m = m.rotate_around(q, x, y, z);
        }
    }
    
    #[cfg(test)]
    mod joml_tests {
        use super::*;
    
        fn close(a: JVec3, b: JVec3) -> bool {
            (a.x - b.x).abs() < 1e-5 && (a.y - b.y).abs() < 1e-5 && (a.z - b.z).abs() < 1e-5
        }
    
        #[test]
        fn rotations_turn_axes_as_joml_does() {
            let q = JQuat::rotation_x(std::f32::consts::FRAC_PI_2);
            // A positive turn about X takes +Y to +Z (right-handed), as JOML's rotationX.
            assert!(close(JMat4::rotation(q).transform_position(JVec3::new(0.0, 1.0, 0.0)), JVec3::new(0.0, 0.0, 1.0)));
            let q = JQuat::rotation_y(std::f32::consts::FRAC_PI_2);
            assert!(close(JMat4::rotation(q).transform_position(JVec3::new(0.0, 0.0, 1.0)), JVec3::new(1.0, 0.0, 0.0)));
            let q = JQuat::rotation_z(std::f32::consts::FRAC_PI_2);
            assert!(close(JMat4::rotation(q).transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(0.0, 1.0, 0.0)));
        }
    
        #[test]
        fn translate_then_rotate_multiplies_on_the_right() {
            // JOML: new Matrix4f().translate(1, 0, 0).rotateZ(90) moves (1, 0, 0) to (1, 1, 0).
            let m = JMat4::identity().translate(1.0, 0.0, 0.0).rotate_z(std::f32::consts::FRAC_PI_2);
            assert!(close(m.transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(1.0, 1.0, 0.0)));
        }
    
        #[test]
        fn invert_undoes() {
            let m = JMat4::identity().translate(1.0, 2.0, 3.0).rotate_x(0.7).rotate_y(-0.3).scale(2.0, 0.5, 1.5);
            let p = JVec3::new(0.3, -1.2, 4.0);
            assert!(close(m.invert().transform_position(m.transform_position(p)), p));
            assert!(close(m.mul(m.invert()).transform_position(p), p));
        }
    
        #[test]
        fn rotate_around_keeps_its_point() {
            let m = JMat4::identity().rotate_around(JQuat::rotation_y(1.1), 0.5, 0.25, 0.0);
            assert!(close(m.transform_position(JVec3::new(0.5, 0.25, 0.0)), JVec3::new(0.5, 0.25, 0.0)));
        }
    
        #[test]
        fn pose_stack_push_pop() {
            let mut p = JPoseStack::new();
            p.translate(1.0, 0.0, 0.0);
            p.push_pose();
            p.scale(2.0, 2.0, 2.0);
            assert!(close(p.last_pose().transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(3.0, 0.0, 0.0)));
            p.pop_pose();
            assert!(close(p.last_pose().transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(2.0, 0.0, 0.0)));
        }
    
        #[test]
        fn quaternion_matrix_agrees_with_axis_turns() {
            let q = JQuat::rotation_x(0.4).mul(JQuat::rotation_y(-0.9));
            let a = JMat4::rotation(q);
            let b = JMat4::identity().rotate_x(0.4).rotate_y(-0.9);
            let p = JVec3::new(1.0, 2.0, 3.0);
            assert!(close(a.transform_position(p), b.transform_position(p)));
        }
    
        #[test]
        fn set3x3_and_transpose() {
            let m3 = JMat3::from_cols(JVec3::new(0.0, 1.0, 0.0), JVec3::new(-1.0, 0.0, 0.0), JVec3::new(0.0, 0.0, 1.0));
            let m = JMat4::identity().set3x3(m3);
            assert!(close(m.transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(0.0, 1.0, 0.0)));
            assert!(close(m.transpose3x3().transform_position(JVec3::new(0.0, 1.0, 0.0)), JVec3::new(1.0, 0.0, 0.0)));
        }
    }

    
    // ---- Gson (com.google.gson): JSON values, a reader and Gson's writer ----
    // JsonElement, JsonObject, JsonArray and JsonPrimitive are all one Rust type. Numbers keep their text, as
    // Gson's LazilyParsedNumber does. Objects keep their insertion order, as Gson's LinkedTreeMap does.
    
    #[derive(Clone, Debug, PartialEq)]
    pub enum JJson {
        Null,
        Bool(bool),
        Num(String),
        Str(String),
        Arr(Vec<JJson>),
        Obj(Vec<(String, JJson)>),
    }
    
    /// A float or double as Java prints it (Float.toString / Double.toString).
    pub fn j_java_num(x: f64, float: bool) -> String {
        if x.is_nan() {
            return "NaN".into();
        }
        if x.is_infinite() {
            return if x > 0.0 { "Infinity".into() } else { "-Infinity".into() };
        }
        let a = x.abs();
        if x == 0.0 {
            return if x.is_sign_negative() { "-0.0".into() } else { "0.0".into() };
        }
        let digits = if float { format!("{:e}", x as f32) } else { format!("{:e}", x) };
        let (mant, exp) = digits.split_once('e').unwrap();
        let exp: i32 = exp.parse().unwrap();
        if (1e-3..1e7).contains(&a) {
            let plain = if float { format!("{}", x as f32) } else { format!("{}", x) };
            if plain.contains('.') { plain } else { plain + ".0" }
        } else {
            let mant = if mant.contains('.') { mant.to_string() } else { format!("{mant}.0") };
            format!("{mant}E{exp}")
        }
    }
    
    struct JJsonReader<'a> {
        s: &'a [char],
        i: usize,
    }
    
    impl JJsonReader<'_> {
        fn ws(&mut self) {
            while self.i < self.s.len() && self.s[self.i].is_whitespace() {
                self.i += 1;
            }
        }
        fn fail(&self, what: &str) -> ! {
            panic!("JsonSyntaxException: {what} at {}", self.i)
        }
        fn value(&mut self) -> JJson {
            self.ws();
            match self.s.get(self.i) {
                Some('{') => {
                    self.i += 1;
                    let mut members = Vec::new();
                    self.ws();
                    if self.s.get(self.i) == Some(&'}') {
                        self.i += 1;
                        return JJson::Obj(members);
                    }
                    loop {
                        self.ws();
                        let k = match self.s.get(self.i) {
                            Some('"') | Some('\'') => self.string(),
                            _ => self.bare(),
                        };
                        self.ws();
                        if !matches!(self.s.get(self.i), Some(':') | Some('=')) {
                            self.fail("expected ':'");
                        }
                        self.i += 1;
                        let v = self.value();
                        // A repeated name: Gson keeps the last one.
                        members.retain(|(n, _): &(String, JJson)| *n != k);
                        members.push((k, v));
                        self.ws();
                        match self.s.get(self.i) {
                            Some(',') | Some(';') => self.i += 1,
                            Some('}') => {
                                self.i += 1;
                                return JJson::Obj(members);
                            }
                            _ => self.fail("expected ',' or '}'"),
                        }
                    }
                }
                Some('[') => {
                    self.i += 1;
                    let mut items = Vec::new();
                    self.ws();
                    if self.s.get(self.i) == Some(&']') {
                        self.i += 1;
                        return JJson::Arr(items);
                    }
                    loop {
                        items.push(self.value());
                        self.ws();
                        match self.s.get(self.i) {
                            Some(',') | Some(';') => self.i += 1,
                            Some(']') => {
                                self.i += 1;
                                return JJson::Arr(items);
                            }
                            _ => self.fail("expected ',' or ']'"),
                        }
                    }
                }
                Some('"') | Some('\'') => JJson::Str(self.string()),
                Some(_) => {
                    let word = self.bare();
                    match word.as_str() {
                        "true" => JJson::Bool(true),
                        "false" => JJson::Bool(false),
                        "null" => JJson::Null,
                        w if w.parse::<f64>().is_ok() => JJson::Num(w.to_string()),
                        // Gson is lenient: an unquoted word is a string.
                        w => JJson::Str(w.to_string()),
                    }
                }
                None => self.fail("end of input"),
            }
        }
        fn bare(&mut self) -> String {
            let start = self.i;
            while self.i < self.s.len() && !matches!(self.s[self.i], ',' | ':' | '=' | ';' | '}' | ']' | '{' | '[' | '"' | '\'') && !self.s[self.i].is_whitespace() {
                self.i += 1;
            }
            if start == self.i {
                self.fail("expected a value");
            }
            self.s[start..self.i].iter().collect()
        }
        fn string(&mut self) -> String {
            let q = self.s[self.i];
            self.i += 1;
            let mut out = String::new();
            loop {
                let Some(&c) = self.s.get(self.i) else { self.fail("unterminated string") };
                self.i += 1;
                match c {
                    c if c == q => return out,
                    '\\' => {
                        let Some(&e) = self.s.get(self.i) else { self.fail("unterminated string") };
                        self.i += 1;
                        match e {
                            'n' => out.push('\n'),
                            't' => out.push('\t'),
                            'r' => out.push('\r'),
                            'b' => out.push('\u{8}'),
                            'f' => out.push('\u{c}'),
                            'u' => {
                                let hex: String = self.s[self.i..(self.i + 4).min(self.s.len())].iter().collect();
                                self.i += 4;
                                let unit = u16::from_str_radix(&hex, 16).unwrap_or_else(|_| self.fail("bad \\u escape"));
                                // A surrogate pair: two escapes make one char.
                                if (0xD800..0xDC00).contains(&unit) && self.s.get(self.i) == Some(&'\\') && self.s.get(self.i + 1) == Some(&'u') {
                                    let hex2: String = self.s[self.i + 2..(self.i + 6).min(self.s.len())].iter().collect();
                                    if let Ok(low) = u16::from_str_radix(&hex2, 16) {
                                        self.i += 6;
                                        out.extend(char::decode_utf16([unit, low]).map(|r| r.unwrap_or('\u{FFFD}')));
                                        continue;
                                    }
                                }
                                out.push(char::from_u32(unit as u32).unwrap_or('\u{FFFD}'));
                            }
                            other => out.push(other),
                        }
                    }
                    c => out.push(c),
                }
            }
        }
    }
    
    impl JJson {
        /// JsonParser.parseString: panics (Java throws JsonSyntaxException) on bad JSON.
        pub fn parse(s: &str) -> JJson {
            let chars: Vec<char> = s.chars().collect();
            let mut r = JJsonReader { s: &chars, i: 0 };
            let v = r.value();
            r.ws();
            if r.i != chars.len() {
                r.fail("more after the value");
            }
            v
        }
        pub fn object() -> JJson {
            JJson::Obj(Vec::new())
        }
        pub fn array() -> JJson {
            JJson::Arr(Vec::new())
        }
        pub fn int(v: i64) -> JJson {
            JJson::Num(v.to_string())
        }
        pub fn float(v: f32) -> JJson {
            JJson::Num(j_java_num(v as f64, true))
        }
        pub fn double(v: f64) -> JJson {
            JJson::Num(j_java_num(v, false))
        }
        pub fn str(v: &str) -> JJson {
            JJson::Str(v.to_string())
        }
        pub fn is_json_object(&self) -> bool {
            matches!(self, JJson::Obj(_))
        }
        pub fn is_json_array(&self) -> bool {
            matches!(self, JJson::Arr(_))
        }
        pub fn is_json_null(&self) -> bool {
            matches!(self, JJson::Null)
        }
        pub fn is_json_primitive(&self) -> bool {
            matches!(self, JJson::Bool(_) | JJson::Num(_) | JJson::Str(_))
        }
        pub fn is_string(&self) -> bool {
            matches!(self, JJson::Str(_))
        }
        pub fn is_number(&self) -> bool {
            matches!(self, JJson::Num(_))
        }
        pub fn is_boolean(&self) -> bool {
            matches!(self, JJson::Bool(_))
        }
        /// getAsJsonObject() / getAsJsonArray(): Java throws IllegalStateException on the wrong kind.
        pub fn as_object(&self) -> JJson {
            assert!(self.is_json_object(), "Not a JSON Object: {self}");
            self.clone()
        }
        pub fn as_array(&self) -> JJson {
            assert!(self.is_json_array(), "Not a JSON Array: {self}");
            self.clone()
        }
        fn members(&self) -> &Vec<(String, JJson)> {
            match self {
                JJson::Obj(m) => m,
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        /// The (name, value) pairs of an object, as entrySet() walks them.
        pub fn entries(&self) -> impl Iterator<Item = (&String, &JJson)> {
            self.members().iter().map(|(k, v)| (k, v))
        }
        pub fn keys(&self) -> impl Iterator<Item = &String> {
            self.members().iter().map(|(k, _)| k)
        }
        pub fn elements(&self) -> impl Iterator<Item = &JJson> {
            match self {
                JJson::Arr(a) => a.iter(),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        pub fn has(&self, k: &str) -> bool {
            self.members().iter().any(|(n, _)| n == k)
        }
        /// JsonObject.get: None (null) when absent.
        pub fn get(&self, k: &str) -> Option<JJson> {
            self.members().iter().find(|(n, _)| n == k).map(|(_, v)| v.clone())
        }
        /// JsonObject.getAsJsonObject(name) / getAsJsonArray(name): null when absent, a cast that throws on the wrong kind.
        pub fn member_object(&self, k: &str) -> Option<JJson> {
            self.get(k).map(|v| v.as_object())
        }
        pub fn member_array(&self, k: &str) -> Option<JJson> {
            self.get(k).map(|v| v.as_array())
        }
        /// JsonArray.get(i).
        pub fn at(&self, i: i32) -> JJson {
            match self {
                JJson::Arr(a) => a.get(i as usize).cloned().unwrap_or_else(|| panic!("Index {i} out of bounds for length {}", a.len())),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        pub fn size(&self) -> i32 {
            match self {
                JJson::Arr(a) => a.len() as i32,
                JJson::Obj(m) => m.len() as i32,
                _ => panic!("size() of a JSON primitive"),
            }
        }
        pub fn is_empty(&self) -> bool {
            self.size() == 0
        }
        /// The single value of a one-element array, as Gson's JsonArray.getAsX does.
        fn single(&self) -> &JJson {
            match self {
                JJson::Arr(a) if a.len() == 1 => &a[0],
                JJson::Arr(_) => panic!("Array must have size 1"),
                other => other,
            }
        }
        pub fn as_string(&self) -> String {
            match self.single() {
                JJson::Str(s) => s.clone(),
                JJson::Num(n) => n.clone(),
                JJson::Bool(b) => b.to_string(),
                other => panic!("UnsupportedOperationException: {other} as a string"),
            }
        }
        fn number_text(&self) -> String {
            match self.single() {
                JJson::Num(n) | JJson::Str(n) => n.trim().to_string(),
                other => panic!("UnsupportedOperationException: {other} as a number"),
            }
        }
        pub fn as_f64(&self) -> f64 {
            let t = self.number_text();
            t.parse().unwrap_or_else(|_| panic!("NumberFormatException: {t}"))
        }
        pub fn as_float(&self) -> f32 {
            self.as_f64() as f32
        }
        pub fn as_double(&self) -> f64 {
            self.as_f64()
        }
        /// getAsInt / getAsLong: a whole number, else the number cut to one.
        pub fn as_long(&self) -> i64 {
            let t = self.number_text();
            t.parse::<i64>().unwrap_or_else(|_| t.parse::<f64>().map(|f| f as i64).unwrap_or_else(|_| panic!("NumberFormatException: {t}")))
        }
        pub fn as_int(&self) -> i32 {
            self.as_long() as i32
        }
        pub fn as_bool(&self) -> bool {
            match self.single() {
                JJson::Bool(b) => *b,
                // Gson: Boolean.parseBoolean of the text
                other => other.as_string().eq_ignore_ascii_case("true"),
            }
        }
        pub fn add(&mut self, k: &str, v: JJson) {
            match self {
                JJson::Obj(m) => {
                    m.retain(|(n, _)| n != k);
                    m.push((k.to_string(), v));
                }
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        pub fn remove(&mut self, k: &str) -> Option<JJson> {
            match self {
                JJson::Obj(m) => {
                    let i = m.iter().position(|(n, _)| n == k)?;
                    Some(m.remove(i).1)
                }
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        pub fn push(&mut self, v: JJson) {
            match self {
                JJson::Arr(a) => a.push(v),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        fn write(&self, out: &mut String, pretty: bool, html: bool, depth: usize) {
            let nl = |out: &mut String, d: usize| {
                if pretty {
                    out.push('\n');
                    out.push_str(&"  ".repeat(d));
                }
            };
            match self {
                JJson::Null => out.push_str("null"),
                JJson::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
                JJson::Num(n) => out.push_str(n),
                JJson::Str(s) => j_json_quote(s, html, out),
                JJson::Arr(a) => {
                    out.push('[');
                    for (i, v) in a.iter().enumerate() {
                        if i > 0 {
                            out.push(',');
                        }
                        nl(out, depth + 1);
                        v.write(out, pretty, html, depth + 1);
                    }
                    if !a.is_empty() {
                        nl(out, depth);
                    }
                    out.push(']');
                }
                JJson::Obj(m) => {
                    out.push('{');
                    for (i, (k, v)) in m.iter().enumerate() {
                        if i > 0 {
                            out.push(',');
                        }
                        nl(out, depth + 1);
                        j_json_quote(k, html, out);
                        out.push(':');
                        if pretty {
                            out.push(' ');
                        }
                        v.write(out, pretty, html, depth + 1);
                    }
                    if !m.is_empty() {
                        nl(out, depth);
                    }
                    out.push('}');
                }
            }
        }
    }
    
    fn j_json_quote(s: &str, html: bool, out: &mut String) {
        out.push('"');
        for c in s.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                '\u{8}' => out.push_str("\\b"),
                '\u{c}' => out.push_str("\\f"),
                '<' | '>' | '&' | '=' | '\'' if html => out.push_str(&format!("\\u{:04x}", c as u32)),
                '\u{2028}' | '\u{2029}' => out.push_str(&format!("\\u{:04x}", c as u32)),
                c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                c => out.push(c),
            }
        }
        out.push('"');
    }
    
    /// JsonElement.toString: compact, without Gson's HTML escaping.
    impl std::fmt::Display for JJson {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut out = String::new();
            self.write(&mut out, false, false, 0);
            f.write_str(&out)
        }
    }
    
    /// A Gson (from `new Gson()` or a GsonBuilder): how it writes.
    #[derive(Clone, Copy, Debug)]
    pub struct JGson {
        pub pretty: bool,
        pub html: bool,
        pub nulls: bool,
    }
    
    impl Default for JGson {
        fn default() -> Self {
            JGson { pretty: false, html: true, nulls: false }
        }
    }
    
    impl JGson {
        /// Gson.toJson.
        pub fn to_json(&self, v: &JJson) -> String {
            let mut out = String::new();
            self.strip(v).write(&mut out, self.pretty, self.html, 0);
            out
        }
        /// Without serializeNulls, Gson leaves out null fields of objects.
        fn strip(&self, v: &JJson) -> JJson {
            match v {
                JJson::Obj(m) => JJson::Obj(m.iter().filter(|(_, x)| self.nulls || !x.is_json_null()).map(|(k, x)| (k.clone(), self.strip(x))).collect()),
                JJson::Arr(a) => JJson::Arr(a.iter().map(|x| self.strip(x)).collect()),
                other => other.clone(),
            }
        }
    }
    
    #[cfg(test)]
    mod jjson_tests {
        use super::*;
    
        #[test]
        fn read_and_write() {
            let j = JJson::parse("{\"a\": 1, \"b\": [1.5, \"x\", true, null], \"c\": {\"d\": -2e3}}");
            assert!(j.has("a") && !j.has("z"));
            assert_eq!(j.get("a").unwrap().as_int(), 1);
            assert_eq!(j.member_array("b").unwrap().at(0).as_float(), 1.5);
            assert_eq!(j.member_array("b").unwrap().size(), 4);
            assert_eq!(j.member_object("c").unwrap().get("d").unwrap().as_double(), -2000.0);
            assert_eq!(j.get("z"), None);
            assert_eq!(j.to_string(), "{\"a\":1,\"b\":[1.5,\"x\",true,null],\"c\":{\"d\":-2e3}}");
            let g = JGson { pretty: true, ..JGson::default() };
            assert_eq!(g.to_json(&JJson::parse("{\"a\":[1,2],\"b\":{},\"n\":null,\"s\":\"<x>\"}")),
                       "{\n  \"a\": [\n    1,\n    2\n  ],\n  \"b\": {},\n  \"s\": \"\\u003cx\\u003e\"\n}");
        }
    
        #[test]
        fn java_numbers() {
            assert_eq!(j_java_num(1.0, true), "1.0");
            assert_eq!(j_java_num(0.1, true), "0.1");
            assert_eq!(j_java_num(1.0e-4, false), "1.0E-4");
            assert_eq!(j_java_num(12345678.0, false), "1.2345678E7");
            assert_eq!(j_java_num(-0.75, false), "-0.75");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "xaerominimap");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
