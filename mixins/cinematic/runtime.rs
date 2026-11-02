// Cinematic Interactions, ported to Horizon by horizon-port from Cinematic Interaction-Fabric-26.3-0.18.30.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_cinematic). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "cinematic";
pub const NAME: &str = "Cinematic Interactions";
pub const VERSION: &str = "0.18.30";
pub const DESCRIPTION: &str = "First-person cinematic interactions with crafting tables, furnaces and other workstations.";

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

    /// `AddonMenuStation` (dev/dyslexic/cinematic/api/client/AddonMenuStation.java)
    pub mod addon_menu_station {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // AddonMenuStation.AddonMenuStation(4): not translated, its class isn't translated (it's generic).
        // AddonMenuStation.kind(0): not translated, its class isn't translated (it's generic).
    }

    /// `AddonScenes` (dev/dyslexic/cinematic/api/client/AddonScenes.java)
    pub mod addon_scenes {
        use super::*;
        // FACTORIES: not translated, it uses Factory, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct AddonScenes {
        }
        impl AddonScenes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // AddonScenes.register(2): not translated, it uses Factory, which isn't translated (it's an interface).
        // AddonScenes.available(1): not translated, it uses Minecraft's `BlockState`.
        // AddonScenes.create(4): not translated, it uses Minecraft's `BlockPos`.
    }

    /// `AddonScenes.Factory` (dev/dyslexic/cinematic/api/client/AddonScenes.java)
    pub mod addon_scenes_factory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // AddonScenes.Factory.create(4): not translated, it has no body.
    }

    /// `BrewingWatch` (dev/dyslexic/cinematic/brewing/BrewingWatch.java)
    pub mod brewing_watch {
        use super::*;
        pub const SLOTS: i32 = 5;
        pub const BOTTLES: i32 = 3;
        #[derive(Clone, Debug)]
        pub struct BrewingWatch {
        }
        impl BrewingWatch {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BrewingWatch.afterTick(4): not translated, it uses Minecraft's `Level`.
        // BrewingWatch.fingerprint(1): not translated, it uses Minecraft's `BrewingStandBlockEntity`.
        // BrewingWatch.contents(2): not translated, it uses BrewingContentsPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // BrewingWatch.chunkSent(2): not translated, it uses Minecraft's `ServerPlayer`.
    }

    /// `BrewingWatch.Fingerprinted` (dev/dyslexic/cinematic/brewing/BrewingWatch.java)
    pub mod brewing_watch_fingerprinted {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BrewingWatch.Fingerprinted.cinematic$fingerprint(0): not translated, it has no body.
        // BrewingWatch.Fingerprinted.cinematic$setFingerprint(1): not translated, it has no body.
    }

    /// `CampfireActions` (dev/dyslexic/cinematic/campfire/CampfireActions.java)
    pub mod campfire_actions {
        use super::*;
        pub const SPOTS: i32 = 4;
        // WATCHES: not translated, it uses Minecraft's `CampfireBlockEntity`.
        #[derive(Clone, Debug)]
        pub struct CampfireActions {
        }
        impl CampfireActions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CampfireActions.handle(4): not translated, it uses Minecraft's `ServerPlayer`.
        // CampfireActions.quickMove(4): not translated, it uses Minecraft's `ServerPlayer`.
        // CampfireActions.sendProgress(2): not translated, it uses Minecraft's `ServerPlayer`.
        // CampfireActions.tick(2): not translated, it uses Minecraft's `ServerLevel`.
        // CampfireActions.advanceStalledSlot(6): not translated, it uses Minecraft's `ServerLevel`.
    }

    /// `CampfireActions.CookingWatch` (dev/dyslexic/cinematic/campfire/CampfireActions.java)
    pub mod campfire_actions_cooking_watch {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `CachedCheck`. Its static parts are below.
        // CampfireActions.CookingWatch.reset(1): not translated, its class isn't translated (it uses Minecraft's `CachedCheck`).
    }

    /// `Cinematic` (dev/dyslexic/cinematic/Cinematic.java)
    pub mod cinematic {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ModInitializer`. Its static parts are below.
        pub const MODID: &str = "cinematic";
        // LOGGER: the logger. Log calls become comments.
        // Cinematic.onInitialize(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
        pub fn id(path: &str) -> JIdentifier {
            return JIdentifier::new("cinematic", &path);
        }
    }

    /// `ClientBridge` (dev/dyslexic/cinematic/ClientBridge.java)
    pub mod client_bridge {
        use super::*;
        // useStation: not translated, it uses Minecraft's `BlockPos`.
        // vanillaUse: not translated, it uses Minecraft's `BlockPos`.
        // useAt: not translated, it uses Minecraft's `Vec3`.
        pub static SWALLOW_SWING: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct ClientBridge {
        }
        impl ClientBridge {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `Anim` (dev/dyslexic/cinematic/client/Anim.java)
    pub mod anim {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Anim {
        }
        impl Anim {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn clamp01(t: f64) -> f64 {
            return if t < 0.0f64 { 0.0f64 } else { if t > 1.0f64 { 1.0f64 } else { t } };
        }
        pub fn window(t: f64, a: f64, b: f64) -> f64 {
            return anim::clamp01((t - a) / (b - a));
        }
        pub fn smoother(mut t: f64) -> f64 {
            t = anim::clamp01(t);
            return ((t * t) * t) * ((t * ((t * 6.0f64) - 15.0f64)) + 10.0f64);
        }
        pub fn in_out_cubic(mut t: f64) -> f64 {
            t = anim::clamp01(t);
            return if t < 0.5f64 { ((4.0f64 * t) * t) * t } else { 1.0f64 - (((((-2.0f64) * t) + 2.0f64).powf(3.0f64)) / 2.0f64) };
        }
        pub fn in_out_sine(t: f64) -> f64 {
            return (-(((std::f64::consts::PI * anim::clamp01(t)).cos()) - 1.0f64)) / 2.0f64;
        }
        pub fn out_cubic(mut t: f64) -> f64 {
            t = anim::clamp01(t);
            return 1.0f64 - ((1.0f64 - t).powf(3.0f64));
        }
        pub fn in_cubic(mut t: f64) -> f64 {
            t = anim::clamp01(t);
            return (t * t) * t;
        }
        pub fn out_back(mut t: f64, overshoot: f64) -> f64 {
            t = anim::clamp01(t);
            let c3: f64 = overshoot + 1.0f64;
            return (1.0f64 + (c3 * ((t - 1.0f64).powf(3.0f64)))) + (overshoot * ((t - 1.0f64).powf(2.0f64)));
        }
        pub fn hump(t: f64) -> f64 {
            return (std::f64::consts::PI * anim::clamp01(t)).sin();
        }
        // Anim.lerp(3): not translated, it uses Minecraft's `Vec3`.
        pub fn approach(current: f64, target: f64, rate: f64, dt: f64) -> f64 {
            return target + ((current - target) * (((-rate) * dt).exp()));
        }
    }

    /// `Anim.Spring3` (dev/dyslexic/cinematic/client/Anim.java)
    pub mod anim_spring3 {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Spring3 {
            pub x: f64,
            pub y: f64,
            pub z: f64,
            pub vx: f64,
            pub vy: f64,
            pub vz: f64,
        }
        impl Spring3 {
            pub fn new() -> Self {
                let mut s = Self { x: 0.0f64, y: 0.0f64, z: 0.0f64, vx: 0.0f64, vy: 0.0f64, vz: 0.0f64 };
                s
            }
            // Anim.Spring3.snap(1): not translated, it uses Minecraft's `Vec3`.
            // Anim.Spring3.pos(0): not translated, it uses Minecraft's `Vec3`.
            pub fn speed(&self) -> f64 {
                return (((self.vx * self.vx) + (self.vy * self.vy)) + (self.vz * self.vz)).sqrt();
            }
            // Anim.Spring3.update(3): not translated, it uses Minecraft's `Vec3`.
        }
    }

    /// `AnvilStation` (dev/dyslexic/cinematic/client/AnvilStation.java)
    pub mod anvil_station {
        use super::*;
        // The class itself isn't translated: it extends `MenuStation`. Its static parts are below.
        pub const HAND_Y: f64 = 0.062f64;
        pub const ITEM_REACH: f64 = 0.075f64;
        pub const MAIN_SIZE: f32 = 0.4f32;
        pub const ADDITION_SIZE: f32 = 0.3f32;
        pub const MAIN_TURN: f32 = 45.0f32;
        pub const TEXT_SCALE: f32 = 0.006f32;
        pub const FORGE_LENGTH: f64 = 2.05f64;
        pub static BLOWS: LazyLock<Vec<f64>> = LazyLock::new(|| vec![0.7f64, 1.02f64, 1.3f64]);
        pub const SWING: f64 = 0.09f64;
        pub static PUT_AWAY: LazyLock<f64> = LazyLock::new(|| ((*anvil_station::BLOWS)[2]) + 0.32f64);
        pub const ARM_REST: f32 = -0.98f32;
        pub const ARM_HIP: f32 = 0.2f32;
        pub const ARM_RAISED: f32 = -2.75f32;
        pub const ARM_IMPACT: f32 = -1.26f32;
        pub const ARM_REBOUND: f32 = -1.6f32;
        // AnvilStation.layoutEditor(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.AnvilStation(2): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.topOf(1): not translated, it uses Minecraft's `Vec3`.
        // AnvilStation.kind(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.center(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.touchSound(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.rightOf(1): not translated, it uses Minecraft's `Vec3`.
        // AnvilStation.mainPoint(2): not translated, it uses Minecraft's `Vec3`.
        // AnvilStation.additionPoint(2): not translated, it uses Minecraft's `Vec3`.
        // AnvilStation.restPoint(1): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.namePoint(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.costPoint(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.approach(1): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.worksAtAnAngle(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.lean(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.bodyPitch(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.rest(1): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.updateCursor(1): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.overName(1): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.clearCursor(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.handTarget(4): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.heldItemPoint(2): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.cost(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.affordable(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.tooExpensive(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.act(2): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.rename(1): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.sendName(1): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.nameText(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.renaming(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.forge(1): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.tick(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.anvilPoint(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.anvilPoint(2): not translated, it uses Minecraft's `Vec3`.
        // AnvilStation.swingDirection(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.handleDirection(1): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.impact(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.raised(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.rebound(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.hidden(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.mix(3): not translated, it uses Minecraft's `Vec3`.
        // AnvilStation.hammer(1): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.resultPos(2): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.handsOverride(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.handsStiffness(0): not translated, its class isn't translated (it extends `MenuStation`).
        pub fn since_blow(t: f64) -> f64 {
            let mut since: f64 = f64::MAX;
            for &blow in (*anvil_station::BLOWS).iter() {
                if t >= blow {
                    since = t - blow;
                }
            }
            return since;
        }
        // AnvilStation.cameraKick(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.flash(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.flashColor(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.frame(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.blow(5): not translated, it uses Minecraft's `Vec3`.
        // AnvilStation.ring(4): not translated, it uses Minecraft's `Vec3`.
        // AnvilStation.renderScene(4): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.drawForging(9): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // AnvilStation.drawForged(9): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // AnvilStation.drawPile(10): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // AnvilStation.geometryScale(1): not translated, it calls RemainingLayouts.scale, which isn't translated.
        // AnvilStation.headAxis(1): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.drawHammer(6): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.renderWorld(4): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.drawText(8): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // AnvilStation.drawContents(7): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // AnvilStation.drawOnlooker(7): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // AnvilStation.onlookerFx(4): not translated, it uses Minecraft's `BlockPos`.
        pub fn arm_pitch(t: f64) -> f32 {
            if t < 0.3f64 {
                return j_lerp((anim::smoother(t / 0.3f64) as f32), -0.98f32, 0.2f32);
            }
            if t < 0.55f64 {
                return j_lerp(((anim::out_cubic(anim::window(t, 0.3f64, 0.55f64))) as f32), 0.2f32, -2.75f32);
            }
            if t >= (*anvil_station::PUT_AWAY) {
                return j_lerp(((anim::in_out_cubic(anim::window(t, *anvil_station::PUT_AWAY, 2.05f64))) as f32), 0.2f32, -0.98f32);
            }
            for k in 0..((*anvil_station::BLOWS).len() as i32) {
                let blow: f64 = (*anvil_station::BLOWS)[k as usize];
                let wind_from: f64 = if k == 0 { 0.55f64 } else { ((*anvil_station::BLOWS)[(k - 1) as usize]) + 0.12f64 };
                if t < (blow - 0.09f64) {
                    return j_lerp(((anim::in_out_sine(anim::window(t, wind_from, blow - 0.09f64))) as f32), if k == 0 { -2.75f32 } else { -1.6f32 }, -2.75f32);
                }
                if t < blow {
                    return j_lerp(((anim::in_cubic(anim::window(t, blow - 0.09f64, blow))) as f32), -2.75f32, -1.26f32);
                }
                if t < (blow + 0.12f64) {
                    return j_lerp(((anim::out_cubic(anim::window(t, blow, blow + 0.12f64))) as f32), -1.26f32, -1.6f32);
                }
            }
            return j_lerp(((anim::in_cubic(anim::window(t, ((*anvil_station::BLOWS)[2]) + 0.12f64, *anvil_station::PUT_AWAY))) as f32), -1.6f32, 0.2f32);
        }
        pub fn hammer_shown(t: f64) -> f64 {
            return (anim::smoother(anim::window(t, 0.26f64, 0.36f64))) * (1.0f64 - (anim::smoother(anim::window(t, (*anvil_station::PUT_AWAY) - 0.1f64, *anvil_station::PUT_AWAY))));
        }
        // AnvilStation.tooltip(1): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.hint(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.beforeFrame(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.onEnd(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.busy(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.showsInventory(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.renderScreen(3): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.workSamples(0): not translated, its class isn't translated (it extends `MenuStation`).
        // AnvilStation.layoutHeldScale(0): not translated, its class isn't translated (it extends `MenuStation`).
    }

    /// `AnvilStation.Swing` (dev/dyslexic/cinematic/client/AnvilStation.java)
    pub mod anvil_station_swing {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // AnvilStation.Swing.Swing(2): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // AnvilStation.Swing.grip(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // AnvilStation.Swing.angle(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `AnvilStation.Target` (dev/dyslexic/cinematic/client/AnvilStation.java)
    pub mod anvil_station_target {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Target {
            None,
            Surface,
            Stack,
            Name,
        }
        impl Target {
            pub const VALUES: [Target; 4] = [Target::None, Target::Surface, Target::Stack, Target::Name, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Target::None => "NONE",
                    Target::Surface => "SURFACE",
                    Target::Stack => "STACK",
                    Target::Name => "NAME",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Target::None,
                    "SURFACE" => Target::Surface,
                    "STACK" => Target::Stack,
                    "NAME" => Target::Name,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `ArmMesh` (dev/dyslexic/cinematic/client/ArmMesh.java)
    pub mod arm_mesh {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Cube`. Its static parts are below.
        pub const UPPER: f32 = 8.0f32;
        pub const HAND: f32 = 4.0f32;
        // RIGHT_WIDE: not translated, it uses ArmMesh, which isn't translated (it uses Minecraft's `Cube`).
        // RIGHT_WIDE_SLEEVE: not translated, it uses ArmMesh, which isn't translated (it uses Minecraft's `Cube`).
        // LEFT_WIDE: not translated, it uses ArmMesh, which isn't translated (it uses Minecraft's `Cube`).
        // LEFT_WIDE_SLEEVE: not translated, it uses ArmMesh, which isn't translated (it uses Minecraft's `Cube`).
        // RIGHT_SLIM: not translated, it uses ArmMesh, which isn't translated (it uses Minecraft's `Cube`).
        // RIGHT_SLIM_SLEEVE: not translated, it uses ArmMesh, which isn't translated (it uses Minecraft's `Cube`).
        // LEFT_SLIM: not translated, it uses ArmMesh, which isn't translated (it uses Minecraft's `Cube`).
        // LEFT_SLIM_SLEEVE: not translated, it uses ArmMesh, which isn't translated (it uses Minecraft's `Cube`).
        // ArmMesh.ArmMesh(5): not translated, its class isn't translated (it uses Minecraft's `Cube`).
        // ArmMesh.arm(2): not translated, it uses ArmMesh, which isn't translated (it uses Minecraft's `Cube`).
        // ArmMesh.sleeve(2): not translated, it uses ArmMesh, which isn't translated (it uses Minecraft's `Cube`).
        // ArmMesh.render(5): not translated, its class isn't translated (it uses Minecraft's `Cube`).
        // ArmMesh.renderUpper(4): not translated, its class isn't translated (it uses Minecraft's `Cube`).
        // ArmMesh.renderForearm(4): not translated, its class isn't translated (it uses Minecraft's `Cube`).
        // ArmMesh.renderFloating(4): not translated, its class isn't translated (it uses Minecraft's `Cube`).
    }

    /// `BeaconStation` (dev/dyslexic/cinematic/client/BeaconStation.java)
    pub mod beacon_station {
        use super::*;
        // The class itself isn't translated: it extends `MenuStation`. Its static parts are below.
        pub const HAND_Y: f64 = 0.06f64;
        pub const ITEM_REACH: f64 = 0.07f64;
        pub const LIGHT_LENGTH: f64 = 3.0f64;
        pub const BURST: f64 = 0.62f64;
        pub static RING: LazyLock<JIdentifier> = LazyLock::new(|| cinematic::id("textures/fx/ring.png"));
        pub static RAY: LazyLock<JIdentifier> = LazyLock::new(|| cinematic::id("textures/fx/ray.png"));
        // BeaconStation.layoutEditor(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.BeaconStation(2): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.kind(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.center(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.touchSound(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.levels(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.payment(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.powers(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.open(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.chosen(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.active(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.ready(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.powerPos(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.paymentPos(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.beamAt(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.restPoint(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.approach(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.lean(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.leanFov(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.freeCamera(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.bodyPitch(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.rest(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.updateCursor(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.clearCursor(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.handTarget(4): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.heldItemPoint(2): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.handsOverride(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.handsStiffness(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.act(2): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.choose(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.light(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.burst(3): not translated, it uses Minecraft's `Vec3`.
        // BeaconStation.deny(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.color(1): not translated, it uses Minecraft's `Holder`.
        // BeaconStation.cameraKick(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.flash(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.flashColor(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.tick(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.frame(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.renderScene(4): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.burstColor(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.drawLight(3): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.drawBurst(7): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // BeaconStation.ray(10): not translated, it uses Minecraft's `VertexConsumer`.
        pub fn channel(v: f32) -> i32 {
            return (0i32).max((255i32).min(j_round_f(v * 255.0f32)));
        }
        // BeaconStation.renderWorld(4): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.drawPower(10): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.renderScreen(3): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.drawContents(6): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // BeaconStation.onlookerFx(6): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // BeaconStation.tooltip(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.hint(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.beforeFrame(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.onEnd(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.busy(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.showsInventory(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BeaconStation.workSamples(0): not translated, its class isn't translated (it extends `MenuStation`).
    }

    /// `BeaconStation.Icon` (dev/dyslexic/cinematic/client/BeaconStation.java)
    pub mod beacon_station_icon {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `TextureAtlasSprite`. Its static parts are below.
        // BeaconStation.Icon.Icon(9): not translated, its class isn't translated (it uses Minecraft's `TextureAtlasSprite`).
        // BeaconStation.Icon.sprite(0): not translated, its class isn't translated (it uses Minecraft's `TextureAtlasSprite`).
        // BeaconStation.Icon.at(0): not translated, its class isn't translated (it uses Minecraft's `TextureAtlasSprite`).
        // BeaconStation.Icon.half(0): not translated, its class isn't translated (it uses Minecraft's `TextureAtlasSprite`).
        // BeaconStation.Icon.brightness(0): not translated, its class isn't translated (it uses Minecraft's `TextureAtlasSprite`).
        // BeaconStation.Icon.alpha(0): not translated, its class isn't translated (it uses Minecraft's `TextureAtlasSprite`).
        // BeaconStation.Icon.label(0): not translated, its class isn't translated (it uses Minecraft's `TextureAtlasSprite`).
        // BeaconStation.Icon.labelColor(0): not translated, its class isn't translated (it uses Minecraft's `TextureAtlasSprite`).
        // BeaconStation.Icon.labelAt(0): not translated, its class isn't translated (it uses Minecraft's `TextureAtlasSprite`).
        // BeaconStation.Icon.rotation(0): not translated, its class isn't translated (it uses Minecraft's `TextureAtlasSprite`).
    }

    /// `BeaconStation.Power` (dev/dyslexic/cinematic/client/BeaconStation.java)
    pub mod beacon_station_power {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Holder`. Its static parts are below.
        // BeaconStation.Power.Power(4): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // BeaconStation.Power.effect(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // BeaconStation.Power.tier(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // BeaconStation.Power.secondary(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // BeaconStation.Power.upgrade(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
    }

    /// `BeaconStation.Target` (dev/dyslexic/cinematic/client/BeaconStation.java)
    pub mod beacon_station_target {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Target {
            None,
            Power,
            Payment,
            Top,
        }
        impl Target {
            pub const VALUES: [Target; 4] = [Target::None, Target::Power, Target::Payment, Target::Top, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Target::None => "NONE",
                    Target::Power => "POWER",
                    Target::Payment => "PAYMENT",
                    Target::Top => "TOP",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Target::None,
                    "POWER" => Target::Power,
                    "PAYMENT" => Target::Payment,
                    "TOP" => Target::Top,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `BookPages` (dev/dyslexic/cinematic/client/BookPages.java)
    pub mod book_pages {
        use super::*;
        pub const PAPER_U: i32 = 26;
        pub const PAPER_V: i32 = 8;
        pub const W: i32 = 131;
        pub const H: i32 = 164;
        pub const SCALE: i32 = 4;
        pub const TEXT_X: i32 = 10;
        pub const TEXT_Y: i32 = 22;
        pub const TEXT_W: i32 = 114;
        pub const LINES: i32 = 14;
        pub const POOL: i32 = 8;
        pub static BOOK: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "textures/gui/book.png"));
        pub const INK: i32 = -16777216;
        pub const PAGE_NUMBER: i32 = -6650778;
        // PAGES: not translated, it uses Page, which isn't translated (it extends Minecraft's `AbstractTexture`).
        pub static FRAME: JCell<i64> = JCell::new(0i64);
        // depth: not translated, it uses Minecraft's `GpuTexture`.
        // depthView: not translated, it uses Minecraft's `GpuTextureView`.
        // projection: not translated, it uses Minecraft's `ProjectionMatrixBuffer`.
        // noFog: not translated, it uses Minecraft's `FogRenderer`.
        #[derive(Clone, Debug)]
        pub struct BookPages {
        }
        impl BookPages {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn left_hand(index: i32) -> bool {
            return (index < 0) || ((index % 2) != 0);
        }
        // BookPages.texture(2): not translated, it uses BookPages.PAGES, which isn't translated (it uses Page, which isn't translated (it extends Minecraft's `AbstractTexture`)).
        // BookPages.prepare(3): not translated, it uses Minecraft's `FormattedText`.
        // BookPages.take(0): not translated, it uses Page, which isn't translated (it extends Minecraft's `AbstractTexture`).
        // BookPages.paint(4): not translated, it uses Page, which isn't translated (it extends Minecraft's `AbstractTexture`).
        // BookPages.forget(0): not translated, it uses BookPages.PAGES, which isn't translated (it uses Page, which isn't translated (it extends Minecraft's `AbstractTexture`)).
    }

    /// `BookPages.Page` (dev/dyslexic/cinematic/client/BookPages.java)
    pub mod book_pages_page {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractTexture`. Its static parts are below.
        // BookPages.Page.Page(1): not translated, its class isn't translated (it extends Minecraft's `AbstractTexture`).
    }

    /// `BrewingBottles` (dev/dyslexic/cinematic/client/BrewingBottles.java)
    pub mod brewing_bottles {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockEntityRenderer`. Its static parts are below.
        pub const SIZE: f32 = 0.42f32;
        // SHOWN: not translated, it uses Shown, which isn't translated (it uses Minecraft's `BlockPos`).
        // BrewingBottles.BrewingBottles(1): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BrewingBottles.createRenderState(0): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BrewingBottles.submit(4): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BrewingBottles.extractRenderState(5): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BrewingBottles.beginFrame(0): not translated, it uses BrewingBottles.SHOWN, which isn't translated (it uses Shown, which isn't translated (it uses Minecraft's `BlockPos`)).
        // BrewingBottles.renderAll(4): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // BrewingBottles.render(4): not translated, it uses Minecraft's `BrewingStandBlockEntity`.
        // BrewingBottles.draw(10): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
    }

    /// `BrewingBottles.Shown` (dev/dyslexic/cinematic/client/BrewingBottles.java)
    pub mod brewing_bottles_shown {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPos`. Its static parts are below.
        // BrewingBottles.Shown.Shown(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // BrewingBottles.Shown.pos(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // BrewingBottles.Shown.light(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `BrewingStation` (dev/dyslexic/cinematic/client/BrewingStation.java)
    pub mod brewing_station {
        use super::*;
        // The class itself isn't translated: it extends `MenuStation`. Its static parts are below.
        pub const HAND_Y: f64 = 0.055f64;
        pub const ITEM_REACH: f64 = 0.07f64;
        pub const BOTTLE_TOUCH_HALF_HEIGHT: f64 = 0.12f64;
        pub const LOOSE_TOUCH_HALF_HEIGHT: f64 = 0.065f64;
        pub static FEET: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![12.0f64, 8.0f64], vec![4.0f64, 4.0f64], vec![4.0f64, 12.0f64]]);
        pub const BELLY: f64 = 0.30874999426305294f64;
        pub const BOTTLES: i32 = 3;
        pub const INGREDIENT: i32 = 3;
        pub const FUEL: i32 = 4;
        pub const BREW_TIME: i32 = 400;
        pub static SLOT_KEYS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["bottle".to_string(), "bottle".to_string(), "bottle".to_string(), "ingredient".to_string(), "fuel".to_string()]);
        // BrewingStation.BrewingStation(2): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.layoutEditor(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.layoutItem(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.busy(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.showsInventory(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.beforeFrame(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.onEnd(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.renderScreen(3): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.layoutFrame(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.layoutHeldScale(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.workSamples(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.kind(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.center(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.touchSound(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.slotPoint(2): not translated, it uses Minecraft's `Vec3`.
        // BrewingStation.slotPoint(3): not translated, it uses Minecraft's `Vec3`.
        // BrewingStation.itemScale(1): not translated, it calls SpatialLayouts.current, which isn't translated.
        // BrewingStation.itemRotation(1): not translated, it calls SpatialLayouts.current, which isn't translated.
        // BrewingStation.restPoint(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.handle(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.approach(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.crouches(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.workOnTop(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.worksAtAnAngle(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.lean(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.bodyPitch(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.rest(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.updateCursor(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.nearStand(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.clearCursor(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.handTarget(4): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.heldItemPoint(2): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.act(2): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.ticks(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.progress(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.color(1): not translated, it uses Minecraft's `ItemStack`.
        // BrewingStation.tick(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.frame(0): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.renderScene(4): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.draw(9): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // BrewingStation.renderWorld(4): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.drawContents(7): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // BrewingStation.drawLoose(8): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // BrewingStation.tooltip(1): not translated, its class isn't translated (it extends `MenuStation`).
        // BrewingStation.hint(0): not translated, its class isn't translated (it extends `MenuStation`).
    }

    /// `BrewingStation.Target` (dev/dyslexic/cinematic/client/BrewingStation.java)
    pub mod brewing_station_target {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Target {
            None,
            Place,
            Item,
        }
        impl Target {
            pub const VALUES: [Target; 3] = [Target::None, Target::Place, Target::Item, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Target::None => "NONE",
                    Target::Place => "PLACE",
                    Target::Item => "ITEM",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Target::None,
                    "PLACE" => Target::Place,
                    "ITEM" => Target::Item,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `CameraRig` (dev/dyslexic/cinematic/client/CameraRig.java)
    pub mod camera_rig {
        use super::*;
        // last: not translated, it uses Pose, which isn't translated (it uses Minecraft's `Vec3`).
        pub static WEIGHT: JCell<f64> = JCell::new(0.0f64);
        pub static PARALLAX_X: JCell<f64> = JCell::new(0.0f64);
        pub static PARALLAX_Y: JCell<f64> = JCell::new(0.0f64);
        #[derive(Clone, Debug)]
        pub struct CameraRig {
        }
        impl CameraRig {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn weight() -> f64 {
            return camera_rig::WEIGHT.get();
        }
        // CameraRig.last(0): not translated, it uses Pose, which isn't translated (it uses Minecraft's `Vec3`).
        // CameraRig.override(3): not translated, it uses Minecraft's `Vec3`.
        // CameraRig.blend(3): not translated, it uses Minecraft's `Vec3`.
        // CameraRig.tether(3): not translated, it uses Minecraft's `Vec3`.
        // CameraRig.constrain(2): not translated, it uses Minecraft's `Vec3`.
        // CameraRig.kick(2): not translated, it uses Minecraft's `Vec3`.
        // CameraRig.breathe(2): not translated, it uses Minecraft's `Vec3`.
        // CameraRig.fov(1): not translated, it uses ClientStation, which isn't translated (it uses Minecraft's `BlockPos`).
        // CameraRig.reset(0): not translated, it uses CameraRig.last, which isn't translated (it uses Pose, which isn't translated (it uses Minecraft's `Vec3`)).
    }

    /// `CameraRig.Pose` (dev/dyslexic/cinematic/client/CameraRig.java)
    pub mod camera_rig_pose {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // CameraRig.Pose.Pose(4): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // CameraRig.Pose.pos(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // CameraRig.Pose.yaw(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // CameraRig.Pose.pitch(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // CameraRig.Pose.roll(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `CampfireHint` (dev/dyslexic/cinematic/client/CampfireHint.java)
    pub mod campfire_hint {
        use super::*;
        // SIT: not translated, it uses Minecraft's `KeyMapping`.
        pub const TEXT_ALPHA_CUTOFF: i32 = 8;
        pub static SHOWN: JCell<f64> = JCell::new(0.0f64);
        // checkedPos: not translated, it uses Minecraft's `BlockPos`.
        // checkedFrom: not translated, it uses Minecraft's `BlockPos`.
        pub static CHECKED_OK: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct CampfireHint {
        }
        impl CampfireHint {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CampfireHint.looked(0): not translated, it uses Minecraft's `BlockPos`.
        // CampfireHint.target(0): not translated, it uses Minecraft's `BlockPos`.
        // CampfireHint.tick(0): not translated, it uses CampfireHint.SIT, which isn't translated (it uses Minecraft's `KeyMapping`).
        // CampfireHint.render(1): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        pub fn argb(a: f32, r: i32, g: i32, b: i32) -> i32 {
            return (((((0i32).max((255i32).min(j_round_f(a * 255.0f32)))) << 24) | (r << 16)) | (g << 8)) | b;
        }
    }

    /// `CampfireStation` (dev/dyslexic/cinematic/client/CampfireStation.java)
    pub mod campfire_station {
        use super::*;
        // The class itself isn't translated: it extends `ClientStation`. Its static parts are below.
        pub const SEAT: f64 = 1.6f64;
        pub const EYE: f64 = 0.8f64;
        pub const HAND_Y: f64 = 0.055f64;
        pub const ITEM_REACH: f64 = 0.07f64;
        pub static RAIL_SAMPLES: LazyLock<Vec<f64>> = LazyLock::new(|| vec![-0.06f64, -0.03f64, 0.0f64, 0.03f64, 0.06f64]);
        pub const LEAN_IN: f64 = 0.85f64;
        // CampfireStation.layoutEditor(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.unhidden(1): not translated, it uses Minecraft's `Vec3`.
        // CampfireStation.CampfireStation(2): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.outward(2): not translated, it uses Minecraft's `Vec3`.
        // CampfireStation.sideOf(2): not translated, it uses Minecraft's `Direction`.
        // CampfireStation.kind(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.center(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.forward(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.right(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.side(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.touchSound(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.lightPos(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.vanillaSpot(3): not translated, it uses Minecraft's `Vec3`.
        // CampfireStation.spot(3): not translated, it uses Minecraft's `Vec3`.
        // CampfireStation.spot(1): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.food(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.applyProgress(1): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.cookingFraction(1): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.syncedProgress(1): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.syncedCookingTime(1): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.syncedLit(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.approach(1): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.lean(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.leanFov(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.workable(1): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.inFront(1): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.crouchesFor(1): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.sittable(2): not translated, it uses Minecraft's `LocalPlayer`.
        // CampfireStation.maxReach(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.bodyPitch(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.seated(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.at(4): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.kneeRight(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.kneeLeft(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.rest(1): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.renderBody(4): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.soul(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.lit(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.updateCursor(1): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.clearCursor(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.handTarget(4): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.heldItemPoint(2): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.click(2): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.renderWorld(4): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.tooltip(1): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.hint(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.stillThere(1): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.beforeFrame(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.onEnd(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.busy(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.showsInventory(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.renderScreen(3): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.renderCookingProgress(2): not translated, its class isn't translated (it extends `ClientStation`).
        pub fn color(alpha: f32, red: i32, green: i32, blue: i32) -> i32 {
            return (((((0i32).max((255i32).min(j_round_f(alpha * 255.0f32)))) << 24) | (red << 16)) | (green << 8)) | blue;
        }
        // CampfireStation.workSamples(0): not translated, its class isn't translated (it extends `ClientStation`).
        // CampfireStation.layoutHeldScale(0): not translated, its class isn't translated (it extends `ClientStation`).
    }

    /// `CampfireStation.Target` (dev/dyslexic/cinematic/client/CampfireStation.java)
    pub mod campfire_station_target {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Target {
            None,
            Place,
            Food,
        }
        impl Target {
            pub const VALUES: [Target; 3] = [Target::None, Target::Place, Target::Food, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Target::None => "NONE",
                    Target::Place => "PLACE",
                    Target::Food => "FOOD",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Target::None,
                    "PLACE" => Target::Place,
                    "FOOD" => Target::Food,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `CartographyStation` (dev/dyslexic/cinematic/client/CartographyStation.java)
    pub mod cartography_station {
        use super::*;
        // The class itself isn't translated: it extends `SurfaceStation`. Its static parts are below.
        pub const HAND_Y: f64 = 0.062f64;
        pub const ITEM_REACH: f64 = 0.075f64;
        pub const LEAN_BACK: f64 = 0.7f64;
        pub const LEAN_UP: f64 = 0.64f64;
        pub static LAYOUT: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![-0.04f64, 0.04f64], vec![0.34f64, -0.1f64]]);
        pub const MAP_SIZE: f32 = 0.5f32;
        pub const ADDITION_SIZE: f32 = 0.24f32;
        pub const WORK_LENGTH: f64 = 2.1f64;
        pub const BEAT: f64 = 1.55f64;
        pub static MAP_BACKGROUND: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "textures/map/map_background.png"));
        // CartographyStation.CartographyStation(2): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.kind(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.center(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.touchSound(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.slotPoint(3): not translated, it uses Minecraft's `Vec3`.
        // CartographyStation.heldItemScale(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.approach(1): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.worksAtAnAngle(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.lean(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.rest(1): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.updateCursor(1): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.clearCursor(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.handTarget(4): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.heldItemPoint(2): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.act(2): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.work(1): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.mapSize(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.additionSize(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.additionYaw(1): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.additionPos(1): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.handsOverride(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.frame(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.renderScene(4): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.drawWorking(6): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.drawMap(8): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // CartographyStation.renderWorld(4): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.drawContents(7): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // CartographyStation.tooltip(1): not translated, its class isn't translated (it extends `SurfaceStation`).
        // CartographyStation.hint(0): not translated, its class isn't translated (it extends `SurfaceStation`).
    }

    /// `CartographyStation.Target` (dev/dyslexic/cinematic/client/CartographyStation.java)
    pub mod cartography_station_target {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Target {
            None,
            Place,
            Item,
        }
        impl Target {
            pub const VALUES: [Target; 3] = [Target::None, Target::Place, Target::Item, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Target::None => "NONE",
                    Target::Place => "PLACE",
                    Target::Item => "ITEM",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Target::None,
                    "PLACE" => Target::Place,
                    "ITEM" => Target::Item,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `CartographyStation.Work` (dev/dyslexic/cinematic/client/CartographyStation.java)
    pub mod cartography_station_work {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Work {
            Zoom,
            Lock,
            Copy,
        }
        impl Work {
            pub const VALUES: [Work; 3] = [Work::Zoom, Work::Lock, Work::Copy, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Work::Zoom => "ZOOM",
                    Work::Lock => "LOCK",
                    Work::Copy => "COPY",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "ZOOM" => Work::Zoom,
                    "LOCK" => Work::Lock,
                    "COPY" => Work::Copy,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `CinematicClient` (dev/dyslexic/cinematic/client/CinematicClient.java)
    pub mod cinematic_client {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ClientModInitializer`. Its static parts are below.
        // CinematicClient.onInitializeClient(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // CinematicClient.screenOpening(1): not translated, it uses Minecraft's `Screen`.
        // CinematicClient.clear(0): not translated, it calls ClientOnly.clear, which isn't translated.
    }

    /// `CinematicKeys` (dev/dyslexic/cinematic/client/CinematicKeys.java)
    pub mod cinematic_keys {
        use super::*;
        // CATEGORY: not translated, it uses Minecraft's `Category`.
        #[derive(Clone, Debug)]
        pub struct CinematicKeys {
        }
        impl CinematicKeys {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `CinematicPipelines` (dev/dyslexic/cinematic/client/CinematicPipelines.java)
    pub mod cinematic_pipelines {
        use super::*;
        // HIGHLIGHT: not translated, it uses Minecraft's `RenderPipeline`.
        // FADING: not translated, it uses Minecraft's `RenderPipeline`.
        // GLOW: not translated, it uses Minecraft's `RenderPipeline`.
        // ENCHANT_CIRCLE: not translated, it uses Minecraft's `RenderPipeline`.
        #[derive(Clone, Debug)]
        pub struct CinematicPipelines {
        }
        impl CinematicPipelines {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CinematicPipelines.fading(1): not translated, it uses Minecraft's `RenderPipeline`.
        pub fn initialize() {
            // (`if` always false in Horizon: only the else part)
        }
    }

    /// `ClientBrewing` (dev/dyslexic/cinematic/client/ClientBrewing.java)
    pub mod client_brewing {
        use super::*;
        // STANDS: not translated, it uses Minecraft's `BlockPos`.
        #[derive(Clone, Debug)]
        pub struct ClientBrewing {
        }
        impl ClientBrewing {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ClientBrewing.apply(1): not translated, it uses BrewingContentsPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientBrewing.item(2): not translated, it uses Minecraft's `ItemStack`.
        // ClientBrewing.unloadChunk(1): not translated, it uses Minecraft's `ChunkPos`.
        // ClientBrewing.clear(0): not translated, it uses ClientBrewing.STANDS, which isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `ClientConfig` (dev/dyslexic/cinematic/client/ClientConfig.java)
    pub mod client_config {
        use super::*;
        // The class itself isn't translated: it uses Value, which isn't translated (it's generic). Its static parts are below.
        // ClientConfig.ClientConfig(1): not translated, its class isn't translated (it uses Value, which isn't translated (it's generic)).
        // ClientConfig.isLoaded(0): not translated, its class isn't translated (it uses Value, which isn't translated (it's generic)).
        // ClientConfig.load(0): not translated, its class isn't translated (it uses Value, which isn't translated (it's generic)).
        // ClientConfig.save(0): not translated, its class isn't translated (it uses Value, which isn't translated (it's generic)).
    }

    /// `ClientConfig.BooleanValue` (dev/dyslexic/cinematic/client/ClientConfig.java)
    pub mod client_config_boolean_value {
        use super::*;
        // The class itself isn't translated: it extends `Value`. Its static parts are below.
        // ClientConfig.BooleanValue.BooleanValue(1): not translated, its class isn't translated (it extends `Value`).
        // ClientConfig.BooleanValue.read(2): not translated, its class isn't translated (it extends `Value`).
    }

    /// `ClientConfig.Builder` (dev/dyslexic/cinematic/client/ClientConfig.java)
    pub mod client_config_builder {
        use super::*;
        // The class itself isn't translated: it uses `ArrayDeque` (java.util). Its static parts are below.
        // ClientConfig.Builder.translation(1): not translated, its class isn't translated (it uses `ArrayDeque` (java.util)).
        // ClientConfig.Builder.comment(1): not translated, its class isn't translated (it uses `ArrayDeque` (java.util)).
        // ClientConfig.Builder.push(1): not translated, its class isn't translated (it uses `ArrayDeque` (java.util)).
        // ClientConfig.Builder.pop(0): not translated, its class isn't translated (it uses `ArrayDeque` (java.util)).
        // ClientConfig.Builder.key(1): not translated, its class isn't translated (it uses `ArrayDeque` (java.util)).
        // ClientConfig.Builder.define(2): not translated, its class isn't translated (it uses `ArrayDeque` (java.util)).
        // ClientConfig.Builder.defineInRange(4): not translated, its class isn't translated (it uses `ArrayDeque` (java.util)).
        // ClientConfig.Builder.build(0): not translated, its class isn't translated (it uses `ArrayDeque` (java.util)).
    }

    /// `ClientConfig.IntValue` (dev/dyslexic/cinematic/client/ClientConfig.java)
    pub mod client_config_int_value {
        use super::*;
        // The class itself isn't translated: it extends `Value`. Its static parts are below.
        // ClientConfig.IntValue.IntValue(3): not translated, its class isn't translated (it extends `Value`).
        // ClientConfig.IntValue.set(1): not translated, its class isn't translated (it extends `Value`).
        // ClientConfig.IntValue.read(2): not translated, its class isn't translated (it extends `Value`).
    }

    /// `ClientConfig.Value` (dev/dyslexic/cinematic/client/ClientConfig.java)
    pub mod client_config_value {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ClientConfig.Value.Value(1): not translated, its class isn't translated (it's generic).
        // ClientConfig.Value.get(0): not translated, its class isn't translated (it's generic).
        // ClientConfig.Value.getDefault(0): not translated, its class isn't translated (it's generic).
        // ClientConfig.Value.set(1): not translated, its class isn't translated (it's generic).
        // ClientConfig.Value.read(2): not translated, it has no body.
    }

    /// `ClientEnchants` (dev/dyslexic/cinematic/client/ClientEnchants.java)
    pub mod client_enchants {
        use super::*;
        // TABLES: not translated, it uses Minecraft's `BlockPos`.
        #[derive(Clone, Debug)]
        pub struct ClientEnchants {
        }
        impl ClientEnchants {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ClientEnchants.get(1): not translated, it uses Minecraft's `BlockPos`.
        // ClientEnchants.apply(1): not translated, it uses EnchantDisplayPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientEnchants.flash(1): not translated, it uses Minecraft's `BlockPos`.
        // ClientEnchants.unloadChunk(1): not translated, it uses Minecraft's `ChunkPos`.
        // ClientEnchants.clear(0): not translated, it uses ClientEnchants.TABLES, which isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `ClientEnchants.Entry` (dev/dyslexic/cinematic/client/ClientEnchants.java)
    pub mod client_enchants_entry {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
    }

    /// `ClientFurnaces` (dev/dyslexic/cinematic/client/ClientFurnaces.java)
    pub mod client_furnaces {
        use super::*;
        // FURNACES: not translated, it uses Minecraft's `BlockPos`.
        // local: not translated, it uses Entry, which isn't translated (it uses Minecraft's `ItemStack`).
        // localPos: not translated, it uses Minecraft's `BlockPos`.
        pub static LOCAL_MENU_ID: JCell<i32> = JCell::new(-1);
        #[derive(Clone, Debug)]
        pub struct ClientFurnaces {
        }
        impl ClientFurnaces {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ClientFurnaces.get(1): not translated, it uses Minecraft's `BlockPos`.
        // ClientFurnaces.item(2): not translated, it uses Minecraft's `ItemStack`.
        // ClientFurnaces.apply(1): not translated, it uses FurnaceContentsPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientFurnaces.prune(1): not translated, it uses Minecraft's `ClientLevel`.
        // ClientFurnaces.unloadChunk(1): not translated, it uses Minecraft's `ChunkPos`.
        // ClientFurnaces.clear(0): not translated, it uses ClientFurnaces.FURNACES, which isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `ClientFurnaces.Entry` (dev/dyslexic/cinematic/client/ClientFurnaces.java)
    pub mod client_furnaces_entry {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
        // ClientFurnaces.Entry.Entry(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
    }

    /// `ClientMenus` (dev/dyslexic/cinematic/client/ClientMenus.java)
    pub mod client_menus {
        use super::*;
        // BLOCKS: not translated, it uses Minecraft's `BlockPos`.
        pub static MOVE_BY_ENTITY: LazyLock<JCell<JMap<i32, f64>>> = LazyLock::new(|| JCell::new(JMap::new()));
        #[derive(Clone, Debug)]
        pub struct ClientMenus {
        }
        impl ClientMenus {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ClientMenus.apply(1): not translated, it uses MenuDisplayPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientMenus.fx(1): not translated, it uses MenuFxPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientMenus.move(1): not translated, it uses MenuFxPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        pub fn since_move(entity_id: i32) -> f64 {
            let at: Option<f64> = { let a_0 = entity_id; client_menus::MOVE_BY_ENTITY.with(|o| o.get(&a_0).cloned()) };
            return if at.is_none() { 1.0e9f64 } else { scene::clock() - (at.expect("null")) };
        }
        pub fn forget_entity(entity_id: i32) {
            { let a_0 = entity_id; client_menus::MOVE_BY_ENTITY.with(|o| o.remove(&a_0)) };
        }
        // ClientMenus.unloadChunk(1): not translated, it uses Minecraft's `ChunkPos`.
        // ClientMenus.clear(0): not translated, it uses ClientMenus.BLOCKS, which isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `ClientMenus.Entry` (dev/dyslexic/cinematic/client/ClientMenus.java)
    pub mod client_menus_entry {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Direction`. Its static parts are below.
        // ClientMenus.Entry.moved(1): not translated, its class isn't translated (it uses Minecraft's `Direction`).
        // ClientMenus.Entry.taker(1): not translated, its class isn't translated (it uses Minecraft's `Direction`).
    }

    /// `ClientOnly` (dev/dyslexic/cinematic/client/ClientOnly.java)
    pub mod client_only {
        use super::*;
        // pendingPos: not translated, it uses Minecraft's `BlockPos`.
        pub static PENDING_KIND: LazyLock<JCell<Option<station_kind::StationKind>>> = LazyLock::new(|| JCell::new(None));
        pub static PENDING_AT: JCell<i64> = JCell::new(0i64);
        pub static ENABLED_FOR_CONNECTION: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct ClientOnly {
        }
        impl ClientOnly {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ClientOnly.onLogin(0): not translated, it uses ClientOnly.pendingPos, which isn't translated (it uses Minecraft's `BlockPos`).
        // ClientOnly.serverHasMod(0): not translated, it calls Minecraft's `Minecraft`.getInstance.
        // ClientOnly.onVanillaUse(2): not translated, it uses Minecraft's `BlockPos`.
        // ClientOnly.onScreenOpening(1): not translated, it uses Minecraft's `Screen`.
        // ClientOnly.clear(0): not translated, it uses ClientOnly.pendingPos, which isn't translated (it uses Minecraft's `BlockPos`).
        pub fn supported(kind: station_kind::StationKind) -> bool {
            return match kind {
                station_kind::StationKind::Table | station_kind::StationKind::Furnace | station_kind::StationKind::Smoker | station_kind::StationKind::BlastFurnace | station_kind::StationKind::Enchanting | station_kind::StationKind::Anvil | station_kind::StationKind::Grindstone | station_kind::StationKind::Smithing | station_kind::StationKind::Stonecutter | station_kind::StationKind::Loom | station_kind::StationKind::Cartography => true,
                _ => false,
            };
        }
        // ClientOnly.matches(2): not translated, it uses `Object`.
    }

    /// `ClientOnlyHintScreen` (dev/dyslexic/cinematic/client/ClientOnlyHintScreen.java)
    pub mod client_only_hint_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        pub const TEXT_WIDTH: i32 = 300;
        pub const TITLE_GAP: i32 = 14;
        pub const BUTTONS_GAP: i32 = 20;
        pub const BUTTON_SPACING: i32 = 8;
        pub const MESSAGE_COLOR: i32 = 14211288;
        // ClientOnlyHintScreen.ClientOnlyHintScreen(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ClientOnlyHintScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ClientOnlyHintScreen.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ClientOnlyHintScreen.onClose(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ClientOnlyHintScreen.getNarrationMessage(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `ClientOnlySmoke` (dev/dyslexic/cinematic/client/ClientOnlySmoke.java)
    pub mod client_only_smoke {
        use super::*;
        pub static ENABLED: LazyLock<bool> = LazyLock::new(|| std::env::var("cinematic.clientOnlySmoke").is_ok_and(|v| v.eq_ignore_ascii_case("true")));
        // TABLE: not translated, it uses Minecraft's `BlockPos`.
        // FURNACE: not translated, it uses Minecraft's `BlockPos`.
        // ANVIL: not translated, it uses Minecraft's `BlockPos`.
        // ENCHANTING: not translated, it uses Minecraft's `BlockPos`.
        pub static STAGE: JCell<i32> = JCell::new(0i32);
        pub static TICKS: JCell<i32> = JCell::new(0i32);
        // original: not translated, it uses Minecraft's `Vec3`.
        #[derive(Clone, Debug)]
        pub struct ClientOnlySmoke {
        }
        impl ClientOnlySmoke {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ClientOnlySmoke.onScreen(1): not translated, it uses Minecraft's `Screen`.
        // ClientOnlySmoke.tick(0): not translated, it uses Minecraft's `Minecraft`.
        // ClientOnlySmoke.clickAsPlayer(2): not translated, it uses Minecraft's `Minecraft`.
        pub fn advance() {
            client_only_smoke::STAGE.set(client_only_smoke::STAGE.get() + 1);
            client_only_smoke::TICKS.set(0);
        }
        // ClientOnlySmoke.fail(1): not translated, it calls Minecraft's `Minecraft`.getInstance.
    }

    /// `ClientPayloads` (dev/dyslexic/cinematic/client/ClientPayloads.java)
    pub mod client_payloads {
        use super::*;
        // AT_STATION: not translated, it uses AtStation, which isn't translated (it uses Minecraft's `BlockPos`).
        #[derive(Clone, Debug)]
        pub struct ClientPayloads {
        }
        impl ClientPayloads {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ClientPayloads.tableContents(1): not translated, it uses TableContentsPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientPayloads.furnaceContents(1): not translated, it uses FurnaceContentsPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientPayloads.furnaceState(1): not translated, it uses FurnaceStatePayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientPayloads.recipeChoices(1): not translated, it uses RecipeChoicePayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientPayloads.campfireProgress(1): not translated, it uses CampfireProgressPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientPayloads.close(1): not translated, it uses SessionClosePayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientPayloads.atStation(1): not translated, it's overloaded with the same number of parameters.
        // ClientPayloads.atStation(1): not translated, it's overloaded with the same number of parameters.
        // ClientPayloads.faceStations(1): not translated, it uses Minecraft's `ClientLevel`.
        // ClientPayloads.someoneAt(1): not translated, it uses Minecraft's `BlockPos`.
        // ClientPayloads.enchantDisplay(1): not translated, it uses EnchantDisplayPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientPayloads.enchantFx(1): not translated, it uses EnchantFxPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientPayloads.menuDisplay(1): not translated, it uses MenuDisplayPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientPayloads.menuFx(1): not translated, it uses MenuFxPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientPayloads.clear(0): not translated, it uses ClientPayloads.AT_STATION, which isn't translated (it uses AtStation, which isn't translated (it uses Minecraft's `BlockPos`)).
    }

    /// `ClientPayloads.AtStation` (dev/dyslexic/cinematic/client/ClientPayloads.java)
    pub mod client_payloads_at_station {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPos`. Its static parts are below.
        // ClientPayloads.AtStation.AtStation(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientPayloads.AtStation.kind(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientPayloads.AtStation.pos(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `ClientRecipes` (dev/dyslexic/cinematic/client/ClientRecipes.java)
    pub mod client_recipes {
        use super::*;
        // CRAFTING: not translated, it uses Minecraft's `RecipeHolder`.
        pub static REVISION: JCell<i32> = JCell::new(0i32);
        #[derive(Clone, Debug)]
        pub struct ClientRecipes {
        }
        impl ClientRecipes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ClientRecipes.crafting(0): not translated, it uses Minecraft's `RecipeHolder`.
        // ClientRecipes.match(2): not translated, it uses Minecraft's `RecipeHolder`.
        // ClientRecipes.clear(0): not translated, it uses ClientRecipes.CRAFTING, which isn't translated (it uses Minecraft's `RecipeHolder`).
        // ClientRecipes.accept(1): not translated, it uses RecipeSyncPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `ClientStation` (dev/dyslexic/cinematic/client/ClientStation.java)
    pub mod client_station {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPos`. Its static parts are below.
        // ClientStation.ClientStation(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.kind(0): not translated, it has no body.
        // ClientStation.sideDirection(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.side(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.forward(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.right(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.center(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.freeSide(4): not translated, it uses Minecraft's `Direction`.
        // ClientStation.sidesByNearness(4): not translated, it uses Minecraft's `Direction`.
        // ClientStation.approach(1): not translated, it has no body.
        // ClientStation.crouches(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.crouchesFor(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.workPoint(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.workOnTop(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.workSamples(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.workable(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.inFront(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.freeCamera(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.lean(0): not translated, it has no body.
        // ClientStation.view(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.turned(2): not translated, it uses Minecraft's `Vec3`.
        // ClientStation.worksAtAnAngle(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.leanFov(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.bodyPitch(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.rest(1): not translated, it has no body.
        // ClientStation.lateral(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.updateCursor(1): not translated, it has no body.
        // ClientStation.clearCursor(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.handTarget(4): not translated, it has no body.
        // ClientStation.heldItemPoint(2): not translated, it has no body.
        // ClientStation.heldItemTilt(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.handsOverride(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.maxReach(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.handsStiffness(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.click(2): not translated, it has no body.
        // ClientStation.drag(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.release(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.scroll(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.keyPressed(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.showsInventory(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.beforeFrame(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.tick(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.busy(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.onEnd(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.quickMove(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.cameraKick(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.flash(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.flashColor(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.stillThere(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.renderWorld(4): not translated, it has no body.
        // ClientStation.renderBody(4): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.renderScreen(3): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.choices(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.chose(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.tooltip(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.hint(0): not translated, it has no body.
        // ClientStation.lightPos(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.touchSound(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ClientStation.soundPoint(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `ClientTables` (dev/dyslexic/cinematic/client/ClientTables.java)
    pub mod client_tables {
        use super::*;
        // TABLES: not translated, it uses Minecraft's `BlockPos`.
        // local: not translated, it uses Entry, which isn't translated (it uses Minecraft's `ItemStack`).
        // localPos: not translated, it uses Minecraft's `BlockPos`.
        pub static LOCAL_MENU_ID: JCell<i32> = JCell::new(-1);
        #[derive(Clone, Debug)]
        pub struct ClientTables {
        }
        impl ClientTables {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ClientTables.get(1): not translated, it uses Minecraft's `BlockPos`.
        // ClientTables.apply(1): not translated, it uses TableContentsPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ClientTables.unloadChunk(1): not translated, it uses Minecraft's `ChunkPos`.
        // ClientTables.clear(0): not translated, it uses ClientTables.TABLES, which isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `ClientTables.Entry` (dev/dyslexic/cinematic/client/ClientTables.java)
    pub mod client_tables_entry {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
        // ClientTables.Entry.Entry(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // ClientTables.Entry.list(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
    }

    /// `FirstPersonBodyCompat` (dev/dyslexic/cinematic/client/compat/FirstPersonBodyCompat.java)
    pub mod first_person_body_compat {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct FirstPersonBodyCompat {
        }
        impl FirstPersonBodyCompat {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FirstPersonBodyCompat.hideLocalBody(0): not translated, it uses Minecraft's `Minecraft`.
        pub fn register() {
            // (`if` always false in Horizon: only the else part)
            // (`if` always false in Horizon: only the else part)
        }
        // FirstPersonBodyCompat.registerFirstPersonModel(0): not translated, a try/catch (ReflectiveOperationException, LinkageError).
        // FirstPersonBodyCompat.registerRealCamera(0): not translated, a try/catch (ReflectiveOperationException, LinkageError).
    }

    /// `CinematicJeiPlugin` (dev/dyslexic/cinematic/client/compat/jei/CinematicJeiPlugin.java)
    pub mod cinematic_jei_plugin {
        use super::*;
        // The class itself isn't translated: it implements `IModPlugin` (mezz.jei.api). Its static parts are below.
        // CinematicJeiPlugin.getPluginUid(0): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
        // CinematicJeiPlugin.onRuntimeAvailable(1): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
        // CinematicJeiPlugin.onRuntimeUnavailable(0): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
    }

    /// `JeiRecipeViewer` (dev/dyslexic/cinematic/client/compat/jei/JeiRecipeViewer.java)
    pub mod jei_recipe_viewer {
        use super::*;
        // The class itself isn't translated: it implements `RecipeViewer` (dev.dyslexic.cinematic.client.guide). Its static parts are below.
        pub static SOURCE_TYPES: LazyLock<Vec<JIdentifier>> = LazyLock::new(|| vec![JIdentifier::new("jeresources", "worldgen"), JIdentifier::new("jeresources", "mob"), JIdentifier::new("jeresources", "plant"), JIdentifier::new("jeresources", "dungeon"), JIdentifier::new("jeresources", "villager"), JIdentifier::new("jeresources", "enchantment")]);
        pub static PROFESSIONS: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("justenoughprofessions", "professions"));
        pub static NOT_STATION_RECIPES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["jeresources".to_string(), "justenoughprofessions".to_string()]);
        pub const MAX_OUTPUTS: i32 = 4000;
        // instance: not translated, it uses JeiRecipeViewer, which isn't translated (it implements `RecipeViewer` (dev.dyslexic.cinematic.client.guide)).
        // JeiRecipeViewer.JeiRecipeViewer(1): not translated, its class isn't translated (it implements `RecipeViewer` (dev.dyslexic.cinematic.client.guide)).
        // JeiRecipeViewer.available(1): not translated, it uses `IJeiRuntime` (mezz.jei.api.runtime).
        // JeiRecipeViewer.current(0): not translated, it uses RecipeViewer, which isn't translated (it's an interface).
        // JeiRecipeViewer.recipes(0): not translated, its class isn't translated (it implements `RecipeViewer` (dev.dyslexic.cinematic.client.guide)).
        // JeiRecipeViewer.focus(2): not translated, its class isn't translated (it implements `RecipeViewer` (dev.dyslexic.cinematic.client.guide)).
        // JeiRecipeViewer.stationCategories(1): not translated, its class isn't translated (it implements `RecipeViewer` (dev.dyslexic.cinematic.client.guide)).
        // JeiRecipeViewer.search(2): not translated, its class isn't translated (it implements `RecipeViewer` (dev.dyslexic.cinematic.client.guide)).
        // JeiRecipeViewer.stationOutputs(1): not translated, its class isn't translated (it implements `RecipeViewer` (dev.dyslexic.cinematic.client.guide)).
        // JeiRecipeViewer.collectOutputs(2): not translated, it's generic.
        // JeiRecipeViewer.stationRecipes(2): not translated, its class isn't translated (it implements `RecipeViewer` (dev.dyslexic.cinematic.client.guide)).
        // JeiRecipeViewer.collectRecipes(3): not translated, it's generic.
        // JeiRecipeViewer.showHowToGet(1): not translated, its class isn't translated (it implements `RecipeViewer` (dev.dyslexic.cinematic.client.guide)).
        // JeiRecipeViewer.showUses(1): not translated, its class isn't translated (it implements `RecipeViewer` (dev.dyslexic.cinematic.client.guide)).
        // JeiRecipeViewer.sources(1): not translated, its class isn't translated (it implements `RecipeViewer` (dev.dyslexic.cinematic.client.guide)).
        // JeiRecipeViewer.sourcePage(2): not translated, it's generic.
        // JeiRecipeViewer.professionPage(1): not translated, its class isn't translated (it implements `RecipeViewer` (dev.dyslexic.cinematic.client.guide)).
        // JeiRecipeViewer.professionPage(2): not translated, it's generic.
        // JeiRecipeViewer.isShowRecipeKey(2): not translated, its class isn't translated (it implements `RecipeViewer` (dev.dyslexic.cinematic.client.guide)).
        // JeiRecipeViewer.keyNames(0): not translated, its class isn't translated (it implements `RecipeViewer` (dev.dyslexic.cinematic.client.guide)).
        // JeiRecipeViewer.isShowUsesKey(2): not translated, its class isn't translated (it implements `RecipeViewer` (dev.dyslexic.cinematic.client.guide)).
    }

    /// `JeiRecipeViewer.JeiStationRecipe` (dev/dyslexic/cinematic/client/compat/jei/JeiRecipeViewer.java)
    pub mod jei_recipe_viewer_jei_station_recipe {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // JeiRecipeViewer.JeiStationRecipe.JeiStationRecipe(4): not translated, its class isn't translated (it's generic).
        // JeiRecipeViewer.JeiStationRecipe.layout(0): not translated, its class isn't translated (it's generic).
        // JeiRecipeViewer.JeiStationRecipe.category(0): not translated, its class isn't translated (it's generic).
        // JeiRecipeViewer.JeiStationRecipe.recipeId(0): not translated, its class isn't translated (it's generic).
        // JeiRecipeViewer.JeiStationRecipe.inputs(0): not translated, its class isn't translated (it's generic).
        // JeiRecipeViewer.JeiStationRecipe.width(0): not translated, its class isn't translated (it's generic).
        // JeiRecipeViewer.JeiStationRecipe.height(0): not translated, its class isn't translated (it's generic).
        // JeiRecipeViewer.JeiStationRecipe.draw(5): not translated, its class isn't translated (it's generic).
        // JeiRecipeViewer.JeiStationRecipe.open(0): not translated, its class isn't translated (it's generic).
    }

    /// `CrafterStation` (dev/dyslexic/cinematic/client/CrafterStation.java)
    pub mod crafter_station {
        use super::*;
        // The class itself isn't translated: it extends `MenuStation`. Its static parts are below.
        pub const HAND_Y: f64 = 0.062f64;
        pub const ITEM_REACH: f64 = 0.075f64;
        pub const UPRIGHT_REST_OUT: f64 = 0.18f64;
        pub const UPRIGHT_REST_SPREAD: f64 = 0.38f64;
        pub const UPRIGHT_REST_DROP: f64 = 0.28f64;
        pub const PITCH: f64 = 0.1875f64;
        pub const RESULT: i32 = 45;
        pub static DISABLED_SLOT: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "container/crafter/disabled_slot"));
        // CrafterStation.CrafterStation(3): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.layoutEditor(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.layoutItem(1): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.busy(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.showsInventory(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.beforeFrame(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.onEnd(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.renderScreen(3): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.layoutFrame(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.layoutHeldScale(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.workSamples(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.kind(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.flat(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.normal(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.rows(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.center(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.touchSound(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.lightPos(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.cell(1): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.restPoint(1): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.resultPoint(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.approach(1): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.crouches(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.workOnTop(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.worksAtAnAngle(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.lean(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.bodyPitch(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.rest(1): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.updateCursor(1): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.clearCursor(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.handTarget(4): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.heldItemPoint(2): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.act(2): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.tick(0): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.renderScene(4): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.drawGrid(9): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // CrafterStation.drawGrid(10): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // CrafterStation.renderWorld(4): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.drawShut(6): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.drawContents(8): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // CrafterStation.tooltip(1): not translated, its class isn't translated (it extends `MenuStation`).
        // CrafterStation.hint(0): not translated, its class isn't translated (it extends `MenuStation`).
    }

    /// `CrafterStation.Target` (dev/dyslexic/cinematic/client/CrafterStation.java)
    pub mod crafter_station_target {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Target {
            None,
            Cell,
            Result,
        }
        impl Target {
            pub const VALUES: [Target; 3] = [Target::None, Target::Cell, Target::Result, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Target::None => "NONE",
                    Target::Cell => "CELL",
                    Target::Result => "RESULT",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Target::None,
                    "CELL" => Target::Cell,
                    "RESULT" => Target::Result,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `EditorIcon` (dev/dyslexic/cinematic/client/EditorIcon.java)
    pub mod editor_icon {
        use super::*;
        pub static GEAR: LazyLock<Vec<String>> = LazyLock::new(|| vec![".....####.....".to_string(), ".....####.....".to_string(), "..##.####.##..".to_string(), "..##########..".to_string(), "...########...".to_string(), "##############".to_string(), "#####....#####".to_string(), "#####....#####".to_string(), "##############".to_string(), "...########...".to_string(), "..##########..".to_string(), "..##.####.##..".to_string(), ".....####.....".to_string(), ".....####.....".to_string()]);
        #[derive(Clone, Debug)]
        pub struct EditorIcon {
        }
        impl EditorIcon {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EditorIcon.draw(2): needs the game, so it's in game_side.rs.
    }

    /// `EnchantFx` (dev/dyslexic/cinematic/client/EnchantFx.java)
    pub mod enchant_fx {
        use super::*;
        pub static CIRCLE: LazyLock<JIdentifier> = LazyLock::new(|| cinematic::id("textures/fx/magic_circle.png"));
        pub static PIXEL_CIRCLE: LazyLock<JIdentifier> = LazyLock::new(|| cinematic::id("textures/fx/enchant_circle_pixel.png"));
        // PIXEL_RENDER: not translated, it uses Minecraft's `RenderType`.
        pub const OUTER_LIFT: f64 = 0.012f64;
        pub const INNER_LIFT: f64 = 0.02f64;
        pub const HINT_LIFT: f64 = 0.028f64;
        pub const SOCKET_LIFT: f64 = 0.016f64;
        pub static GLOW: LazyLock<JIdentifier> = LazyLock::new(|| cinematic::id("textures/fx/glow.png"));
        pub static BEAM: LazyLock<JIdentifier> = LazyLock::new(|| cinematic::id("textures/fx/beam.png"));
        pub const LAPIS_RADIUS: f64 = 0.115f64;
        pub const LAPIS_RIGHT: f64 = 0.44f64;
        pub const LAPIS_NEAR: f64 = 0.18f64;
        pub const LAPIS_LIFT: f64 = 0.27f64;
        pub static RANDOM: LazyLock<JCell<JRandom>> = LazyLock::new(|| JCell::new(JRandom::new()));
        pub static QUAD_UV: LazyLock<Vec<Vec<f32>>> = LazyLock::new(|| vec![vec![0.0f32, 0.0f32], vec![1.0f32, 0.0f32], vec![1.0f32, 1.0f32], vec![0.0f32, 1.0f32]]);
        pub static BOTH_WAYS: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| vec![vec![0, 1, 2, 3], vec![3, 2, 1, 0]]);
        pub static PALETTE: LazyLock<Vec<Vec<f32>>> = LazyLock::new(|| vec![vec![0.75f32, 0.45f32, 1.0f32], vec![0.45f32, 0.8f32, 1.0f32], vec![1.0f32, 0.92f32, 1.0f32]]);
        #[derive(Clone, Debug)]
        pub struct EnchantFx {
        }
        impl EnchantFx {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EnchantFx.top(1): not translated, it uses Minecraft's `Vec3`.
        // EnchantFx.itemCenter(3): not translated, it uses Minecraft's `Vec3`.
        // EnchantFx.lapisCorner(2): not translated, it uses Minecraft's `Vec3`.
        // EnchantFx.lapisGhost(5): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // EnchantFx.lapisGhost(7): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // EnchantFx.crystalLine(9): not translated, it uses Minecraft's `VertexConsumer`.
        // EnchantFx.crystalVertex(8): not translated, it uses Minecraft's `VertexConsumer`.
        // EnchantFx.pixelQuad(6): not translated, it uses Minecraft's `VertexConsumer`.
        // EnchantFx.circle(8): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // EnchantFx.circleBillboard(8): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // EnchantFx.quad(7): not translated, it uses Minecraft's `VertexConsumer`.
        pub fn clamp(v: f32) -> i32 {
            return (0i32).max((255i32).min(j_round_f(v * 255.0f32)));
        }
        // EnchantFx.circle(9): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // EnchantFx.disc(10): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // EnchantFx.billboard(10): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // EnchantFx.beam(10): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // EnchantFx.stream(3): not translated, it uses Minecraft's `Vec3`.
        // EnchantFx.spark(4): needs the game, so it's in game_side.rs.
        // EnchantFx.burst(2): not translated, it uses Minecraft's `Vec3`.
        // EnchantFx.sparkle(2): not translated, it uses Minecraft's `Vec3`.
        // EnchantFx.gather(2): not translated, it uses Minecraft's `Vec3`.
        // EnchantFx.shelves(2): not translated, it uses Minecraft's `Vec3`.
    }

    /// `EnchantFx.CirclePalette` (dev/dyslexic/cinematic/client/EnchantFx.java)
    pub mod enchant_fx_circle_palette {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // EnchantFx.CirclePalette.CirclePalette(2): not translated, its class isn't translated (its constants carry values).
    }

    /// `EnchantRenderer` (dev/dyslexic/cinematic/client/EnchantRenderer.java)
    pub mod enchant_renderer {
        use super::*;
        pub const MAX_DISTANCE_SQR: f64 = 2304.0f64;
        // DRAWN: not translated, it uses Minecraft's `BlockPos`.
        #[derive(Clone, Debug)]
        pub struct EnchantRenderer {
        }
        impl EnchantRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EnchantRenderer.renderAll(5): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // EnchantRenderer.visibleBox(1): not translated, it uses Minecraft's `AABB`.
        // EnchantRenderer.drawCircle(7): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
    }

    /// `EnchantStation` (dev/dyslexic/cinematic/client/EnchantStation.java)
    pub mod enchant_station {
        use super::*;
        // The class itself isn't translated: it extends `ClientStation`. Its static parts are below.
        pub const OFFER_HIGH: f64 = 0.92f64;
        pub const OFFER_LOW: f64 = 0.74f64;
        pub const OFFER_FAR: f64 = -0.12f64;
        pub const OFFER_NEAR: f64 = 0.02f64;
        pub const OFFER_SPREAD: f64 = 0.4f64;
        pub const HAND_Y: f64 = 0.062f64;
        pub const ITEM_REACH: f64 = 0.075f64;
        pub const RITUAL_LENGTH: f64 = 1.5f64;
        pub const RITUAL_PEAK: f64 = 0.5f64;
        pub const ITEM_SIZE: f32 = 0.34f32;
        // EnchantStation.EnchantStation(2): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.kind(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.center(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.touchSound(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.layoutEditor(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.layoutFrame(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.layoutItem(1): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.layoutPoint(1): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.layoutNormal(1): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.layoutAxis(2): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.busy(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.showsInventory(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.beforeFrame(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.renderScreen(3): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.workSamples(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.layoutHeldScale(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.layoutGhost(5): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.itemPosition(3): not translated, it uses Minecraft's `Vec3`.
        // EnchantStation.lapisPosition(2): not translated, it uses Minecraft's `Vec3`.
        // EnchantStation.approach(1): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.lean(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.leanFov(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.bodyPitch(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.rest(1): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.menu(0): not translated, it uses Minecraft's `EnchantmentMenu`.
        // EnchantStation.item(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.lapis(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.offered(1): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.affordable(1): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.itemPos(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.sigilPos(1): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.corner(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.guiRadius(2): not translated, it uses Minecraft's `Vec3`.
        // EnchantStation.under(3): not translated, it uses Minecraft's `Vec3`.
        // EnchantStation.updateCursor(1): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.clearCursor(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.handTarget(4): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.heldItemPoint(2): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.handsOverride(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.click(2): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.slotClick(4): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.startRitual(2): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.ritualTime(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.cameraKick(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.flash(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.flashColor(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.onEnd(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.tick(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.source(0): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.renderScene(4): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.renderWorld(4): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.drawLapisCount(6): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // EnchantStation.drawSigil(10): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.drawFloating(7): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // EnchantStation.drawLapis(7): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // EnchantStation.drawLapis(8): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // EnchantStation.tooltip(1): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.offerTooltip(2): not translated, its class isn't translated (it extends `ClientStation`).
        // EnchantStation.hint(0): not translated, its class isn't translated (it extends `ClientStation`).
    }

    /// `EnchantStation.Target` (dev/dyslexic/cinematic/client/EnchantStation.java)
    pub mod enchant_station_target {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Target {
            None,
            Surface,
            Item,
            Lapis,
            Offer,
        }
        impl Target {
            pub const VALUES: [Target; 5] = [Target::None, Target::Surface, Target::Item, Target::Lapis, Target::Offer, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Target::None => "NONE",
                    Target::Surface => "SURFACE",
                    Target::Item => "ITEM",
                    Target::Lapis => "LAPIS",
                    Target::Offer => "OFFER",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Target::None,
                    "SURFACE" => Target::Surface,
                    "ITEM" => Target::Item,
                    "LAPIS" => Target::Lapis,
                    "OFFER" => Target::Offer,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `FieldGuide` (dev/dyslexic/cinematic/client/FieldGuide.java)
    pub mod field_guide {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `EditBox`. Its static parts are below.
        pub const W: i32 = 260;
        pub const H: i32 = 168;
        pub const COVER: i32 = 7;
        pub const SPINE: i32 = 6;
        pub const PAGE_W: i32 = 120;
        pub const COLS: i32 = 6;
        pub const ROWS: i32 = 4;
        pub const CELL: i32 = 18;
        pub const PAGE_SIZE: i32 = 24;
        pub const GRID_Y: i32 = 42;
        pub const TOGGLE_W: i32 = 30;
        pub const TOGGLE_H: i32 = 28;
        pub const CLOSE_W: i32 = 14;
        // FieldGuide.FieldGuide(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.text(2): not translated, it uses Minecraft's `Component`.
        // FieldGuide.viewer(0): not translated, it uses RecipeViewer, which isn't translated (it's an interface).
        // FieldGuide.present(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.tick(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.refresh(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.select(1): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.professionOf(1): not translated, it uses Minecraft's `Component`.
        // FieldGuide.showsSources(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.showsProfession(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.pages(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.place(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.lx(1): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.ly(1): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.leftPage(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.rightPage(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.inBook(2): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.inToggle(2): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.over(2): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.render(3): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.drawToggle(3): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.drawCover(2): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.drawTabs(5): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        pub fn over_close(mx: i32, my: i32) -> bool {
            return (((mx >= 244) && (mx < 258)) && (my >= (-12))) && (my < 2);
        }
        // FieldGuide.drawLeftPage(5): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.gridX(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.entryAt(2): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.recipeButtons(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.drawRecipePage(6): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.drawSourcesPage(5): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.drawProfessionPage(5): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.professionButton(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.drawButton(6): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.canHint(1): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.hint(1): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.click(3): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.scroll(3): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.charTyped(1): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.keyPressed(1): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.viewerKey(3): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.hoveredEntry(1): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.tooltip(2): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.fold(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.rim(5): needs the game, so it's in game_side.rs.
        // FieldGuide.fit(3): not translated, it uses Minecraft's `Font`.
        pub fn lighter(rgb: i32) -> i32 {
            let r: i32 = (255i32).min(((rgb >> 16) & 0xFF) + 28);
            let gr: i32 = (255i32).min(((rgb >> 8) & 0xFF) + 28);
            let b: i32 = (255i32).min((rgb & 0xFF) + 28);
            return ((r << 16) | (gr << 8)) | b;
        }
        pub fn color(alpha: f32, rgb: i32) -> i32 {
            return ((j_clamp(j_round_f(alpha * 255.0f32), 0, 255)) << 24) | (rgb & 16777215);
        }
        // FieldGuide.togglePoint(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.point(2): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.searchPoint(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.entryPoint(1): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.tabPoint(1): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.buttonPoint(1): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.closePoint(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.sourcePoint(1): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.opened(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.selected(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.recipeCount(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.sourceCount(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.profession(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.professionPageReady(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.tabName(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.itemCount(0): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
        // FieldGuide.indexOf(1): not translated, its class isn't translated (it uses Minecraft's `EditBox`).
    }

    /// `FieldGuide.Button` (dev/dyslexic/cinematic/client/FieldGuide.java)
    pub mod field_guide_button {
        use super::*;
        // The class itself isn't translated: a function kept in a variable or field (only parameters can be). Its static parts are below.
        // FieldGuide.Button.Button(2): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // FieldGuide.Button.over(2): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
    }

    /// `FieldGuide.Tab` (dev/dyslexic/cinematic/client/FieldGuide.java)
    pub mod field_guide_tab {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
    }

    /// `FrameProbe` (dev/dyslexic/cinematic/client/FrameProbe.java)
    pub mod frame_probe {
        use super::*;
        pub static FRAMES: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| Vec::new());
        pub static SEGMENT: LazyLock<JCell<Option<String>>> = LazyLock::new(|| JCell::new(None));
        pub static AFTER_SHOT: JCell<i32> = JCell::new(0i32);
        #[derive(Clone, Debug)]
        pub struct FrameProbe {
        }
        impl FrameProbe {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn shot_taken() {
            frame_probe::AFTER_SHOT.set(2);
        }
        // FrameProbe.sample(1): not translated, it uses Minecraft's `Camera`.
        pub fn report() {
            let n: i32 = ((*frame_probe::FRAMES).len() as i32);
            if (frame_probe::SEGMENT.get().is_some()) && (n >= 12) {
                let dt: Vec<f64> = (1..n).map(|i: i32| (((*frame_probe::FRAMES)[i as usize])[1]) - (((*frame_probe::FRAMES)[(i - 1) as usize])[1])).filter(|&d: &f64| !d.is_nan()).collect::<Vec<_>>();
                let mean: f64 = ({ let avg_0: Vec<f64> = dt.iter().cloned().map(|v| v as f64).collect(); if avg_0.is_empty() { None } else { Some(avg_0.iter().sum::<f64>() / avg_0.len() as f64) } }).unwrap_or(0.0f64);
                let sd: f64 = (({ let avg_1: Vec<f64> = dt.iter().cloned().map(|d: f64| (d - mean) * (d - mean)).map(|v| v as f64).collect(); if avg_1.is_empty() { None } else { Some(avg_1.iter().sum::<f64>() / avg_1.len() as f64) } }).unwrap_or(0.0f64)).sqrt();
                let max: f64 = (dt.iter().cloned().reduce(|p, q| if j_compare_f(p as f64, q as f64) >= 0 { p } else { q })).unwrap_or(0.0f64);
                // log: "[Probe] {} frames={} time={} dt={}±{}ms max={}ms | camera {} | turn {} | right hand {} | left hand {}"
            }
        }
        // FrameProbe.evenness(2): not translated, it calls `Arrays` (java.util).sort.
        pub fn sq(v: f64) -> f64 {
            return v * v;
        }
    }

    /// `FreeLook` (dev/dyslexic/cinematic/client/FreeLook.java)
    pub mod free_look {
        use super::*;
        // LOOK: not translated, it uses Minecraft's `KeyMapping`.
        pub const START_TURN: f32 = 38.0f32;
        pub const START_PITCH: f32 = 24.0f32;
        pub const START_DISTANCE: f64 = 2.5f64;
        pub const NEAREST: f64 = 1.3f64;
        pub const FURTHEST: f64 = 4.5f64;
        pub const RATE: f64 = 5.5f64;
        pub static WANTED: JCell<bool> = JCell::new(false);
        pub static AMOUNT: JCell<f64> = JCell::new(0.0f64);
        pub static YAW: JCell<f32> = JCell::new(0.0f32);
        pub static PITCH: JCell<f32> = JCell::new(0.0f32);
        pub static DISTANCE: JCell<f64> = JCell::new(2.5f64);
        pub static GRABBED: JCell<bool> = JCell::new(false);
        pub static SAVED_X: JCell<f64> = JCell::new(0.0f64);
        pub static SAVED_Y: JCell<f64> = JCell::new(0.0f64);
        pub static LAST_X: JCell<f64> = JCell::new(0.0f64);
        pub static LAST_Y: JCell<f64> = JCell::new(0.0f64);
        pub static THIRD: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct FreeLook {
        }
        impl FreeLook {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn amount() -> f64 {
            return anim::smoother(free_look::AMOUNT.get());
        }
        pub fn looking() -> bool {
            return free_look::WANTED.get() || (free_look::AMOUNT.get() > 0.02f64);
        }
        pub fn shows_body() -> bool {
            return free_look::THIRD.get();
        }
        // FreeLook.keyPressed(2): not translated, it uses FreeLook.LOOK, which isn't translated (it uses Minecraft's `KeyMapping`).
        // FreeLook.keyReleased(2): not translated, it uses FreeLook.LOOK, which isn't translated (it uses Minecraft's `KeyMapping`).
        pub fn stop() {
            free_look::WANTED.set(false);
        }
        pub fn scroll(amount: f64) {
            free_look::DISTANCE.set(j_clamp(free_look::DISTANCE.get() - (amount * 0.35f64), 1.3f64, 4.5f64));
        }
        // FreeLook.hint(0): not translated, it uses Minecraft's `Component`.
        // FreeLook.frame(1): not translated, it uses Minecraft's `Minecraft`.
        // FreeLook.outroFar(1): not translated, it uses Minecraft's `Minecraft`.
        // FreeLook.grab(1): not translated, it uses Minecraft's `Minecraft`.
        // FreeLook.release(1): not translated, it uses Minecraft's `Minecraft`.
        // FreeLook.reset(0): not translated, it uses Minecraft's `Minecraft`.
        // FreeLook.apply(1): not translated, it uses Pose, which isn't translated (it uses Minecraft's `Vec3`).
        // FreeLook.focus(2): not translated, it uses Minecraft's `Vec3`.
    }

    /// `FurnaceGridEditor` (dev/dyslexic/cinematic/client/FurnaceGridEditor.java)
    pub mod furnace_grid_editor {
        use super::*;
        // The class itself isn't translated: it implements `LayoutEditor`. Its static parts are below.
        pub const W: i32 = 180;
        pub const H: i32 = 342;
        pub const ROW_Y: i32 = 112;
        pub const ROW_H: i32 = 24;
        pub static FIELDS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["offset_u".to_string(), "offset_v".to_string(), "width".to_string(), "height".to_string(), "out".to_string(), "item_size".to_string(), "click_width".to_string(), "click_height".to_string(), "item_u".to_string(), "item_v".to_string()]);
        pub static REGIONS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["input".to_string(), "fuel".to_string(), "result".to_string(), "cooking".to_string(), "burning".to_string()]);
        pub static BASIC: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0, 1, 2, 3, 4, 5]);
        pub static ADVANCED: LazyLock<Vec<i32>> = LazyLock::new(|| vec![6, 7, 8, 9]);
        pub static GAUGE: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0, 1, 2, 3, 4]);
        // FurnaceGridEditor.FurnaceGridEditor(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.text(2): not translated, it uses Minecraft's `Component`.
        // FurnaceGridEditor.editing(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.visible(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.amount(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.fields(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        pub fn percent(field: i32) -> bool {
            return ((((field == 2) || (field == 3)) || (field == 5)) || (field == 6)) || (field == 7);
        }
        pub fn min(field: i32) -> f64 {
            return if furnace_grid_editor::percent(field) { 50.0f64 } else { if field == 4 { -2.0f64 } else { if field >= 8 { -3.0f64 } else { -6.0f64 } } };
        }
        pub fn max(field: i32) -> f64 {
            return if furnace_grid_editor::percent(field) { 150.0f64 } else { -furnace_grid_editor::min(field) };
        }
        pub fn step(field: i32) -> f64 {
            return if furnace_grid_editor::percent(field) { 1.0f64 } else { 0.125f64 };
        }
        // FurnaceGridEditor.layout(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.lx(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.ly(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.inPanel(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.toggle(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.over(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.frame(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.open(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.cancel(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.save(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.change(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.slide(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.click(3): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.drag(3): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.release(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.scroll(3): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.keyPressed(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.tooltip(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.render(3): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.selector(5): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.button(9): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.glass(4): needs the game, so it's in game_side.rs.
        pub fn color(a: f64, r: i32, g: i32, b: i32) -> i32 {
            return ((((j_clamp((j_round_d(a * 255.0f64) as i32), 0, 255)) << 24) | (r << 16)) | (g << 8)) | b;
        }
        // FurnaceGridEditor.renderWorld(4): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.togglePoint(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.controlPoint(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // FurnaceGridEditor.sliderPoint(2): not translated, its class isn't translated (it implements `LayoutEditor`).
    }

    /// `FurnaceLayouts` (dev/dyslexic/cinematic/client/FurnaceLayouts.java)
    pub mod furnace_layouts {
        use super::*;
        pub static JSON: LazyLock<JGson> = LazyLock::new(|| JGson { pretty: true, ..JGson::default() });
        pub const FILE: &str = "cinematic-furnace-layouts.json";
        // profiles: not translated, it uses `FurnaceAppearance`.
        pub static SELECTED: LazyLock<JCell<Vec<i32>>> = LazyLock::new(|| JCell::new(vec![0i32; 3]));
        // current: not translated, it uses `FurnaceAppearance`.
        // preview: not translated, it uses `FurnaceAppearance`.
        pub static LOADED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct FurnaceLayouts {
        }
        impl FurnaceLayouts {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FurnaceLayouts.index(1): not translated, a switch on StationKind (or null).
        // FurnaceLayouts.defaults(0): not translated, it uses `FurnaceAppearance`.
        pub fn path() -> std::path::PathBuf {
            return j_config_dir().join("cinematic-furnace-layouts.json");
        }
        // FurnaceLayouts.load(0): not translated, its body couldn't be read: a labelled statement.
        // FurnaceLayouts.current(1): not translated, it uses `FurnaceAppearance`.
        // FurnaceLayouts.profiles(1): not translated, it uses `FurnaceAppearance`.
        // FurnaceLayouts.selected(1): not translated, it calls FurnaceLayouts.load, which isn't translated.
        // FurnaceLayouts.preview(2): not translated, it uses `FurnaceAppearance`.
        // FurnaceLayouts.cancelPreview(1): not translated, it uses FurnaceLayouts.preview, which isn't translated (it uses `FurnaceAppearance`).
        // FurnaceLayouts.frame(1): not translated, it calls FurnaceLayouts.load, which isn't translated.
        // FurnaceLayouts.save(3): not translated, it uses `FurnaceAppearance`.
    }

    /// `FurnaceLayouts.Saved` (dev/dyslexic/cinematic/client/FurnaceLayouts.java)
    pub mod furnace_layouts_saved {
        use super::*;
        // The class itself isn't translated: it uses `FurnaceAppearance`. Its static parts are below.
        // FurnaceLayouts.Saved.Saved(3): not translated, its class isn't translated (it uses `FurnaceAppearance`).
        // FurnaceLayouts.Saved.version(0): not translated, its class isn't translated (it uses `FurnaceAppearance`).
        // FurnaceLayouts.Saved.selected(0): not translated, its class isn't translated (it uses `FurnaceAppearance`).
        // FurnaceLayouts.Saved.profiles(0): not translated, its class isn't translated (it uses `FurnaceAppearance`).
    }

    /// `FurnaceRenderer` (dev/dyslexic/cinematic/client/FurnaceRenderer.java)
    pub mod furnace_renderer {
        use super::*;
        pub const MAX_DISTANCE_SQR: f64 = 2304.0f64;
        pub static FUEL_GLOW: LazyLock<Vec<i32>> = LazyLock::new(|| vec![1]);
        pub static INPUT_GLOW: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0]);
        pub static COOKING_GLOW: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0, 2]);
        // UP: not translated, it uses Minecraft's `Vec3`.
        #[derive(Clone, Debug)]
        pub struct FurnaceRenderer {
        }
        impl FurnaceRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FurnaceRenderer.renderAll(5): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // FurnaceRenderer.drawBed(10): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // FurnaceRenderer.drawGlow(10): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // FurnaceRenderer.radial(6): not translated, it uses Minecraft's `VertexConsumer`.
        pub fn rgb(red: f64, green: f64, blue: f64) -> Vec<i32> {
            return vec![(red as i32), ((red * green) as i32), ((red * blue) as i32)];
        }
    }

    /// `FurnaceStation` (dev/dyslexic/cinematic/client/FurnaceStation.java)
    pub mod furnace_station {
        use super::*;
        // The class itself isn't translated: it extends `ClientStation`. Its static parts are below.
        pub static LOW: LazyLock<furnace_station_staging::Staging> = LazyLock::new(|| furnace_station_staging::Staging::new(true, 0.9f64, 0.5f64, 0.1f64, 0.44f64, -3.5f32, 32.0f32, furnace_zones_face::Face::Front, 0.2f64, 0.09f64, 0.36f64));
        pub static TALL: LazyLock<furnace_station_staging::Staging> = LazyLock::new(|| furnace_station_staging::Staging::new(false, 0.7f64, 1.15f64, 0.08f64, 0.97f64, -2.0f32, 45.0f32, furnace_zones_face::Face::Top, 0.15f64, 0.13f64, 0.06f64));
        pub const ITEM_REACH: f64 = 0.07f64;
        pub const HAND_Y: f64 = 0.062f64;
        pub const SUNK_REST_U: f64 = 0.18f64;
        pub const SUNK_REST_V: f64 = 0.14f64;
        pub const SUNK_REST_OUT: f64 = 0.06f64;
        pub static BOTH_WAYS: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| vec![vec![0, 1, 2, 3], vec![3, 2, 1, 0]]);
        pub static ONE_WAY: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| vec![vec![0, 1, 2, 3]]);
        // FurnaceStation.FurnaceStation(3): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.cookingInput(1): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.showHint(1): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.drawHint(4): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.kind(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.geometry(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.busy(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.showsInventory(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.beforeFrame(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.onEnd(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.workSamples(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.key(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.center(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.lightPos(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.touchSound(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.stillThere(1): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.approach(1): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.crouches(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.workOnTop(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.bodyPitch(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.worksAtAnAngle(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.lean(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.rest(1): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.sunk(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.updateCursor(1): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.pick(3): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.zoneHit(1): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.clearCursor(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.handTarget(4): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.heldItemPoint(2): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.heldItemTilt(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.heldItemScale(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.click(2): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.accepts(2): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.cookable(1): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.applyState(1): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.tick(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.burn(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.cook(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.renderWorld(4): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.gauge(8): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.glowQuad(10): not translated, it uses Minecraft's `VertexConsumer`.
        // FurnaceStation.quad(11): not translated, it uses Minecraft's `VertexConsumer`.
        // FurnaceStation.quad(12): not translated, it uses Minecraft's `VertexConsumer`.
        // FurnaceStation.tooltip(1): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.hint(0): not translated, its class isn't translated (it extends `ClientStation`).
        // FurnaceStation.renderScreen(3): not translated, its class isn't translated (it extends `ClientStation`).
    }

    /// `FurnaceStation.Staging` (dev/dyslexic/cinematic/client/FurnaceStation.java)
    pub mod furnace_station_staging {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Staging {
            pub crouch: bool,
            pub out: f64,
            pub up: f64,
            pub side: f64,
            pub look_v: f64,
            pub roll: f32,
            pub body_pitch: f32,
            pub rest_face: furnace_zones_face::Face,
            pub rest_u: f64,
            pub rest_v: f64,
            pub rest_out: f64,
        }
        impl Staging {
            pub fn new(crouch: bool, out: f64, up: f64, side: f64, look_v: f64, roll: f32, body_pitch: f32, rest_face: furnace_zones_face::Face, rest_u: f64, rest_v: f64, rest_out: f64) -> Self {
                let mut s = Self { crouch: crouch, out: out, up: up, side: side, look_v: look_v, roll: roll, body_pitch: body_pitch, rest_face: rest_face, rest_u: rest_u, rest_v: rest_v, rest_out: rest_out };
                s.crouch = crouch;
                s.out = out;
                s.up = up;
                s.side = side;
                s.look_v = look_v;
                s.roll = roll;
                s.body_pitch = body_pitch;
                s.rest_face = rest_face;
                s.rest_u = rest_u;
                s.rest_v = rest_v;
                s.rest_out = rest_out;
                s
            }
            pub fn crouch(&self) -> bool {
                return self.crouch;
            }
            pub fn out(&self) -> f64 {
                return self.out;
            }
            pub fn up(&self) -> f64 {
                return self.up;
            }
            pub fn side(&self) -> f64 {
                return self.side;
            }
            pub fn look_v(&self) -> f64 {
                return self.look_v;
            }
            pub fn roll(&self) -> f32 {
                return self.roll;
            }
            pub fn body_pitch(&self) -> f32 {
                return self.body_pitch;
            }
            pub fn rest_face(&self) -> furnace_zones_face::Face {
                return self.rest_face;
            }
            pub fn rest_u(&self) -> f64 {
                return self.rest_u;
            }
            pub fn rest_v(&self) -> f64 {
                return self.rest_v;
            }
            pub fn rest_out(&self) -> f64 {
                return self.rest_out;
            }
        }
    }

    /// `FurnaceStation.ZoneHit` (dev/dyslexic/cinematic/client/FurnaceStation.java)
    pub mod furnace_station_zone_hit {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // FurnaceStation.ZoneHit.ZoneHit(2): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FurnaceStation.ZoneHit.index(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // FurnaceStation.ZoneHit.world(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `Fx` (dev/dyslexic/cinematic/client/Fx.java)
    pub mod fx {
        use super::*;
        pub static RANDOM: LazyLock<JCell<JRandom>> = LazyLock::new(|| JCell::new(JRandom::new()));
        pub static WHITE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "textures/misc/white.png"));
        // HIGHLIGHT: not translated, it uses Minecraft's `RenderType`.
        // GLOWS: not translated, it uses Minecraft's `RenderType`.
        pub static SPARKS: LazyLock<JCell<Vec<fx_spark::Spark>>> = LazyLock::new(|| JCell::new(Vec::new()));
        pub const MAX_SPARKS: i32 = 900;
        pub static QUAD_UV: LazyLock<Vec<Vec<f32>>> = LazyLock::new(|| vec![vec![0.0f32, 0.0f32], vec![1.0f32, 0.0f32], vec![1.0f32, 1.0f32], vec![0.0f32, 1.0f32]]);
        pub static BOTH_WAYS: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| vec![vec![0, 1, 2, 3], vec![3, 2, 1, 0]]);
        #[derive(Clone, Debug)]
        pub struct Fx {
        }
        impl Fx {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Fx.flatGlow(0): not translated, it uses Minecraft's `RenderType`.
        // Fx.glowVertex(9): not translated, it uses Minecraft's `VertexConsumer`.
        // Fx.glow(1): not translated, it uses Minecraft's `RenderType`.
        // Fx.spark(9): needs the game, so it's in game_side.rs.
        // Fx.renderSparks(4): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        pub fn clear_sparks() {
            fx::SPARKS.with(|o| o.clear());
        }
        // Fx.shade(6): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // Fx.facePatch(10): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // Fx.streak(9): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // Fx.quad(7): not translated, it uses Minecraft's `VertexConsumer`.
        pub fn channel(v: f32) -> i32 {
            return (0i32).max((255i32).min(j_round_f(v * 255.0f32)));
        }
        // Fx.spread(1): needs the game, so it's in game_side.rs.
        // Fx.hotSparks(5): not translated, it uses Minecraft's `Vec3`.
        // Fx.blowSparks(3): not translated, it uses Minecraft's `Vec3`.
        // Fx.flakes(3): not translated, it uses Minecraft's `Vec3`.
        // Fx.fountain(5): not translated, it uses Minecraft's `Vec3`.
        // Fx.motes(7): not translated, it uses Minecraft's `Vec3`.
        // Fx.steam(3): not translated, it uses Minecraft's `Vec3`.
        // Fx.sprite(1): not translated, it uses Minecraft's `TextureAtlasSprite`.
        // Fx.atlas(1): not translated, it uses Minecraft's `VertexConsumer`.
        // Fx.box(10): not translated, it uses Minecraft's `VertexConsumer`.
        // Fx.face(9): not translated, it uses Minecraft's `VertexConsumer`.
    }

    /// `Fx.Spark` (dev/dyslexic/cinematic/client/Fx.java)
    pub mod fx_spark {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Spark {
            pub x: f64,
            pub y: f64,
            pub z: f64,
            pub vx: f64,
            pub vy: f64,
            pub vz: f64,
            pub age: f64,
            pub life: f64,
            pub gravity: f64,
            pub streak: f64,
            pub size: f32,
            pub r: f32,
            pub g: f32,
            pub b: f32,
            pub phase: i32,
        }
        impl Spark {
            pub fn new() -> Self {
                let mut s = Self { x: 0.0f64, y: 0.0f64, z: 0.0f64, vx: 0.0f64, vy: 0.0f64, vz: 0.0f64, age: 0.0f64, life: 0.0f64, gravity: 0.0f64, streak: 0.0f64, size: 0.0f32, r: 0.0f32, g: 0.0f32, b: 0.0f32, phase: 0i32 };
                s
            }
        }
    }

    /// `GrindstoneStation` (dev/dyslexic/cinematic/client/GrindstoneStation.java)
    pub mod grindstone_station {
        use super::*;
        // The class itself isn't translated: it extends `MenuStation`. Its static parts are below.
        pub const HALF: f64 = 0.375f64;
        pub const WIDTH: f64 = 0.25f64;
        pub const HAND_Y: f64 = 0.062f64;
        pub const ITEM_REACH: f64 = 0.07f64;
        pub const ITEM_SIZE: f32 = 0.3f32;
        pub const TOOL_TURN: f32 = -45.0f32;
        pub const SLOT_SIDE: f64 = 0.1f64;
        pub const SLANT: f32 = 25.0f32;
        pub const GRIND_LENGTH: f64 = 2.4f64;
        pub const CONTACT: f64 = 0.6f64;
        pub const STROKE: f64 = 0.433f64;
        pub const CLEAN_AT: f64 = 1.5f64;
        pub const LET_GO: f64 = 1.9f64;
        // GrindstoneStation.layoutEditor(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.GrindstoneStation(3): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.kind(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.center(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.touchSound(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.lightPos(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.wheelCenter(2): not translated, it uses Minecraft's `Vec3`.
        // GrindstoneStation.rightOf(1): not translated, it uses Minecraft's `Vec3`.
        // GrindstoneStation.restPoint(3): not translated, it uses Minecraft's `Vec3`.
        // GrindstoneStation.restPoint(1): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.approach(1): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.worksAtAnAngle(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.lean(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.leanFov(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.bodyPitch(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.rest(1): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.updateCursor(1): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.overWheel(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.under(3): not translated, it uses Minecraft's `Vec3`.
        // GrindstoneStation.clearCursor(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.handTarget(4): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.heldItemPoint(2): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.act(2): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.grind(1): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.removesEnchantments(1): not translated, it uses Minecraft's `ItemStack`.
        pub fn stroke(t: f64) -> f64 {
            let s: f64 = anim::window(t, 0.6f64, 1.9f64);
            let fade: f64 = anim::smoother(s * 8.0f64) * (anim::smoother((1.0f64 - s) * 8.0f64));
            return (0.075f64 * (((std::f64::consts::PI * (t - 0.6f64)) / 0.433f64).sin())) * fade;
        }
        // GrindstoneStation.blade(1): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.blade(2): not translated, it uses Minecraft's `Vec3`.
        // GrindstoneStation.bite(4): not translated, it uses Minecraft's `Vec3`.
        // GrindstoneStation.heldPos(2): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.heldPos(4): not translated, it uses Minecraft's `Vec3`.
        // GrindstoneStation.heldYaw(3): not translated, it uses Minecraft's `Direction`.
        pub fn held_tilt(t: f64) -> f32 {
            return (((-9.0f64) * (anim::in_out_cubic(anim::window(t, 0.28f64, 0.6f64)))) as f32);
        }
        // GrindstoneStation.handle(2): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.lifted(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.lifted(2): not translated, it uses Minecraft's `Vec3`.
        // GrindstoneStation.inspectPos(2): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.handsOverride(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.cameraKick(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.tick(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.frame(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.grindFx(6): not translated, it uses Minecraft's `Vec3`.
        // GrindstoneStation.sound(4): not translated, it uses Minecraft's `Vec3`.
        // GrindstoneStation.renderScene(4): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.drawMove(5): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.drawGrinding(11): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // GrindstoneStation.drawLifted(9): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // GrindstoneStation.drawLying(9): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // GrindstoneStation.drawRim(7): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // GrindstoneStation.rimQuad(13): not translated, it uses Minecraft's `VertexConsumer`.
        // GrindstoneStation.renderWorld(4): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.drawContents(8): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // GrindstoneStation.drawOnlooker(8): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // GrindstoneStation.onlookerFx(5): not translated, it uses Minecraft's `BlockPos`.
        pub fn arm_sway(t: f64) -> f32 {
            return ((grindstone_station::stroke(t) * 2.4f64) as f32);
        }
        // GrindstoneStation.tooltip(1): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.hint(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.stillThere(1): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.beforeFrame(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.onEnd(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.busy(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.showsInventory(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.renderScreen(3): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.workSamples(0): not translated, its class isn't translated (it extends `MenuStation`).
        // GrindstoneStation.layoutHeldScale(0): not translated, its class isn't translated (it extends `MenuStation`).
    }

    /// `GrindstoneStation.Target` (dev/dyslexic/cinematic/client/GrindstoneStation.java)
    pub mod grindstone_station_target {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Target {
            None,
            Place,
            Item,
        }
        impl Target {
            pub const VALUES: [Target; 3] = [Target::None, Target::Place, Target::Item, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Target::None => "NONE",
                    Target::Place => "PLACE",
                    Target::Item => "ITEM",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Target::None,
                    "PLACE" => Target::Place,
                    "ITEM" => Target::Item,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `RecipeViewer` (dev/dyslexic/cinematic/client/guide/RecipeViewer.java)
    pub mod recipe_viewer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // RecipeViewer.search(2): not translated, it has no body.
        // RecipeViewer.stationOutputs(1): not translated, it has no body.
        // RecipeViewer.stationRecipes(2): not translated, it has no body.
        // RecipeViewer.showHowToGet(1): not translated, it has no body.
        // RecipeViewer.showUses(1): not translated, it has no body.
        // RecipeViewer.sources(1): not translated, it has no body.
        // RecipeViewer.professionPage(1): not translated, it has no body.
        // RecipeViewer.isShowRecipeKey(2): not translated, it has no body.
        // RecipeViewer.isShowUsesKey(2): not translated, it has no body.
        // RecipeViewer.keyNames(0): not translated, it has no body.
    }

    /// `RecipeViewer.Icon` (dev/dyslexic/cinematic/client/guide/RecipeViewer.java)
    pub mod recipe_viewer_icon {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // RecipeViewer.Icon.draw(3): not translated, it has no body.
    }

    /// `RecipeViewer.SourcePage` (dev/dyslexic/cinematic/client/guide/RecipeViewer.java)
    pub mod recipe_viewer_source_page {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Component`. Its static parts are below.
        // RecipeViewer.SourcePage.SourcePage(4): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // RecipeViewer.SourcePage.title(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // RecipeViewer.SourcePage.entries(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // RecipeViewer.SourcePage.open(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // RecipeViewer.SourcePage.icon(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
    }

    /// `RecipeViewer.StationRecipe` (dev/dyslexic/cinematic/client/guide/RecipeViewer.java)
    pub mod recipe_viewer_station_recipe {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // RecipeViewer.StationRecipe.category(0): not translated, it has no body.
        // RecipeViewer.StationRecipe.recipeId(0): not translated, it has no body.
        // RecipeViewer.StationRecipe.inputs(0): not translated, it has no body.
        // RecipeViewer.StationRecipe.width(0): not translated, it has no body.
        // RecipeViewer.StationRecipe.height(0): not translated, it has no body.
        // RecipeViewer.StationRecipe.draw(5): not translated, it has no body.
        // RecipeViewer.StationRecipe.open(0): not translated, it has no body.
    }

    /// `RecipeViewers` (dev/dyslexic/cinematic/client/guide/RecipeViewers.java)
    pub mod recipe_viewers {
        use super::*;
        pub static FAILED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct RecipeViewers {
        }
        impl RecipeViewers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RecipeViewers.get(0): not translated, it uses RecipeViewer, which isn't translated (it's an interface).
    }

    /// `HammerModel` (dev/dyslexic/cinematic/client/HammerModel.java)
    pub mod hammer_model {
        use super::*;
        pub static LOCATION: LazyLock<JIdentifier> = LazyLock::new(|| cinematic::id("prop/hammer"));
        pub const SCALE: f32 = 0.3f32;
        // GRIP: not translated, it uses Minecraft's `Vec3`.
        // STRIKE_ALONG: not translated, it uses HammerModel.GRIP, which isn't translated (it uses Minecraft's `Vec3`).
        pub const STRIKE_ACROSS: f64 = (0.112500004f32 as f64);
        pub const ARM_SCALE: f32 = 0.39000002f32;
        pub static WRIST: LazyLock<f64> = LazyLock::new(|| 18.0f64.to_radians());
        #[derive(Clone, Debug)]
        pub struct HammerModel {
        }
        impl HammerModel {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // HammerModel.model(0): not translated, it uses Minecraft's `ItemStackRenderState`.
        // HammerModel.render(7): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // HammerModel.renderInArm(4): not translated, it uses Minecraft's `SubmitNodeCollector`.
        // HammerModel.draw(6): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // HammerModel.orient(4): not translated, it uses Minecraft's `Vec3`.
    }

    /// `Hands` (dev/dyslexic/cinematic/client/Hands.java)
    pub mod hands {
        use super::*;
        pub const ARM_SCALE: f32 = 0.55f32;
        pub const FLOATING_LENGTH: f64 = 0.24750000536441805f64;
        pub const NATURAL_SEGMENT: f64 = (0.275f32 as f64);
        pub static RIGHT: LazyLock<anim_spring3::Spring3> = LazyLock::new(|| anim_spring3::Spring3::new());
        pub static LEFT: LazyLock<anim_spring3::Spring3> = LazyLock::new(|| anim_spring3::Spring3::new());
        // outroRight: not translated, it uses Minecraft's `Vec3`.
        // outroLeft: not translated, it uses Minecraft's `Vec3`.
        // fromRight: not translated, it uses Minecraft's `Vec3`.
        // fromLeft: not translated, it uses Minecraft's `Vec3`.
        pub static TAP_AT: JCell<f64> = JCell::new(-10.0f64);
        pub static LEFT_WORKS: JCell<bool> = JCell::new(false);
        pub static RIGHT_LANDED: JCell<bool> = JCell::new(false);
        pub static LEFT_LANDED: JCell<bool> = JCell::new(false);
        pub const ARM_RADIUS: f64 = 0.07875000149011611f64;
        #[derive(Clone, Debug)]
        pub struct Hands {
        }
        impl Hands {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Hands.begin(0): not translated, it uses Hands.fromLeft, which isn't translated (it uses Minecraft's `Vec3`).
        // Hands.enterActive(0): not translated, it calls Spring3.snap, which isn't translated.
        // Hands.beginTransition(0): not translated, it uses Hands.fromRight, which isn't translated (it uses Minecraft's `Vec3`).
        // Hands.beginOutro(0): not translated, it uses Hands.outroRight, which isn't translated (it uses Minecraft's `Vec3`).
        pub fn tap() {
            hands::TAP_AT.set(scene::clock());
        }
        // Hands.shoulder(3): not translated, it uses Minecraft's `Vec3`.
        // Hands.hides(3): not translated, it uses Minecraft's `Vec3`.
        // Hands.closest(4): not translated, it uses Minecraft's `Vec3`.
        // Hands.hidden(3): not translated, it uses Minecraft's `Vec3`.
        // Hands.update(2): not translated, it uses Minecraft's `Vec3`.
        // Hands.arrive(5): not translated, it uses Minecraft's `Vec3`.
        // Hands.depart(5): not translated, it uses Minecraft's `Vec3`.
        // Hands.follow(4): not translated, it uses ClientStation, which isn't translated (it uses Minecraft's `BlockPos`).
        // Hands.position(1): not translated, it uses Minecraft's `Vec3`.
        // Hands.heldPoint(0): not translated, it uses Minecraft's `Vec3`.
        // Hands.carried(0): not translated, it uses Minecraft's `ItemStack`.
        // Hands.itemInHand(0): not translated, it uses ClientStation, which isn't translated (it uses Minecraft's `BlockPos`).
        // Hands.render(5): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // Hands.drawArm(8): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // Hands.drawArm(11): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // Hands.drawBentArm(11): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // Hands.bentPoints(4): not translated, it uses Minecraft's `Vec3`.
        // Hands.drawBentPiece(12): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
    }

    /// `InventoryEffects` (dev/dyslexic/cinematic/client/InventoryEffects.java)
    pub mod inventory_effects {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
        // InventoryEffects.InventoryEffects(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // InventoryEffects.supported(2): not translated, FabricLoader.getModContainer of a computed id.
        // InventoryEffects.failed(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // InventoryEffects.beginFrame(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // InventoryEffects.drawer(5): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // InventoryEffects.endFrame(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // InventoryEffects.world(4): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // InventoryEffects.action(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // InventoryEffects.inventoryFingerprint(0): not translated, it uses Minecraft's `LocalPlayer`.
        // InventoryEffects.tick(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // InventoryEffects.carried(5): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
    }

    /// `InventoryEffects.Bridge` (dev/dyslexic/cinematic/client/InventoryEffects.java)
    pub mod inventory_effects_bridge {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // InventoryEffects.Bridge.tick(5): not translated, it has no body.
        // InventoryEffects.Bridge.carried(5): not translated, it has no body.
    }

    /// `InventoryEffects.Change` (dev/dyslexic/cinematic/client/InventoryEffects.java)
    pub mod inventory_effects_change {
        use super::*;
        // The class itself isn't translated: it uses Target, which isn't translated (it uses Minecraft's `ItemStack`). Its static parts are below.
        // InventoryEffects.Change.Change(3): not translated, its class isn't translated (it uses Target, which isn't translated (it uses Minecraft's `ItemStack`)).
        // InventoryEffects.Change.target(0): not translated, its class isn't translated (it uses Target, which isn't translated (it uses Minecraft's `ItemStack`)).
        // InventoryEffects.Change.take(0): not translated, its class isn't translated (it uses Target, which isn't translated (it uses Minecraft's `ItemStack`)).
        // InventoryEffects.Change.put(0): not translated, its class isn't translated (it uses Target, which isn't translated (it uses Minecraft's `ItemStack`)).
    }

    /// `InventoryEffects.InteractionsBridge` (dev/dyslexic/cinematic/client/InventoryEffects.java)
    pub mod inventory_effects_interactions_bridge {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct InteractionsBridge {
        }
        impl InteractionsBridge {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // InventoryEffects.InteractionsBridge.tick(1): not translated, it uses Minecraft's `ItemStack`.
            // InventoryEffects.InteractionsBridge.carried(5): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        }
    }

    /// `InventoryEffects.ParticlesBridge` (dev/dyslexic/cinematic/client/InventoryEffects.java)
    pub mod inventory_effects_particles_bridge {
        use super::*;
        // The class itself isn't translated: it implements `Bridge`. Its static parts are below.
        // InventoryEffects.ParticlesBridge.tick(5): not translated, its class isn't translated (it implements `Bridge`).
        // InventoryEffects.ParticlesBridge.slot(1): not translated, it uses Minecraft's `Slot`.
        // InventoryEffects.ParticlesBridge.carried(5): not translated, its class isn't translated (it implements `Bridge`).
    }

    /// `InventoryEffects.Target` (dev/dyslexic/cinematic/client/InventoryEffects.java)
    pub mod inventory_effects_target {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
        // InventoryEffects.Target.Target(5): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // InventoryEffects.Target.id(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // InventoryEffects.Target.stack(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // InventoryEffects.Target.x(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // InventoryEffects.Target.y(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // InventoryEffects.Target.hovered(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
    }

    /// `InventoryEffectsProbe` (dev/dyslexic/cinematic/client/InventoryEffectsProbe.java)
    pub mod inventory_effects_probe {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct InventoryEffectsProbe {
        }
        impl InventoryEffectsProbe {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // InventoryEffectsProbe.defaults(0): not translated, it uses `InventoryParticlesConfig` (net.lopymine.ip.config).
        // InventoryEffectsProbe.clear(0): not translated, it uses `InventoryParticlesRenderer` (net.lopymine.ip.renderer).
        // InventoryEffectsProbe.particles(0): not translated, it uses `InventoryParticlesRenderer` (net.lopymine.ip.renderer).
        // InventoryEffectsProbe.cursor(0): not translated, it uses Minecraft's `Item`.
        pub fn has_interactions() -> bool {
            return false;
        }
        // InventoryEffectsProbe.physicsItem(0): not translated, it uses Minecraft's `Item`.
        // InventoryEffectsProbe.angle(0): not translated, it calls `CursorItemRenderer` (net.lopymine.ipi.renderer).getInstance.
        // InventoryEffectsProbe.x(0): not translated, it calls `InventoryParticlesRenderer` (net.lopymine.ip.renderer).getInstance.
        // InventoryEffectsProbe.y(0): not translated, it calls `InventoryParticlesRenderer` (net.lopymine.ip.renderer).getInstance.
        // InventoryEffectsProbe.near(2): not translated, it calls `InventoryParticlesRenderer` (net.lopymine.ip.renderer).getInstance.
        // InventoryEffectsProbe.actionOnly(0): not translated, it calls InventoryEffectsProbe.defaults, which isn't translated.
        // InventoryEffectsProbe.allSlots(0): not translated, it calls InventoryEffectsProbe.actionOnly, which isn't translated.
        // InventoryEffectsProbe.disable(0): not translated, it uses `InventoryParticleConfig` (net.lopymine.ip.config.sub).
    }

    /// `IrisCompatibility` (dev/dyslexic/cinematic/client/IrisCompatibility.java)
    pub mod iris_compatibility {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct IrisCompatibility {
        }
        impl IrisCompatibility {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // IrisCompatibility.initialize(0): not translated, a try/catch (ReflectiveOperationException, RuntimeException).
        // IrisCompatibility.copyEntityBindings(0): not translated, it uses `Class`.
        // IrisCompatibility.copyBinding(2): not translated, it uses Minecraft's `RenderPipeline`.
        // IrisCompatibility.program(2): not translated, it uses `Object`.
    }

    /// `ItemModels` (dev/dyslexic/cinematic/client/ItemModels.java)
    pub mod item_models {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ItemModels {
        }
        impl ItemModels {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ItemModels.drawStatic(5): not translated, it uses Minecraft's `ItemStack`.
        // ItemModels.draw(5): not translated, it uses Minecraft's `ItemStackRenderState`.
        // ItemModels.drawSpecial(5): not translated, it uses Minecraft's `LayerRenderState`.
        // ItemModels.drawModel(2): not translated, it's generic.
    }

    /// `ItemPlacer` (dev/dyslexic/cinematic/client/ItemPlacer.java)
    pub mod item_placer {
        use super::*;
        pub const FLAT_SIZE: f32 = 0.205f32;
        pub const BLOCK_SIZE: f32 = 0.15f32;
        // FADING_TYPES: not translated, it uses Minecraft's `RenderType`.
        pub static RANDOM: LazyLock<JRandom> = LazyLock::new(|| JRandom::new());
        #[derive(Clone, Debug)]
        pub struct ItemPlacer {
        }
        impl ItemPlacer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ItemPlacer.render(10): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // ItemPlacer.render(11): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // ItemPlacer.renderOne(10): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // ItemPlacer.render(12): not translated, its body couldn't be read: an anonymous class.
        // ItemPlacer.fadingType(1): not translated, it uses Minecraft's `RenderType`.
        // ItemPlacer.opacity(2): not translated, its body couldn't be read: an anonymous class.
        // ItemPlacer.restingBounds(2): not translated, it uses Minecraft's `AABB`.
        // ItemPlacer.awayYaw(1): not translated, it uses Minecraft's `Direction`.
        pub fn lying(yaw: f32, tilt: f32) -> JQuat {
            return (JQuat::rotation_y(yaw.to_radians())).mul(JQuat::rotation_x(((-90.0f32) + tilt).to_radians()));
        }
        // ItemPlacer.drawOriented(8): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // ItemPlacer.model(2): not translated, it uses Minecraft's `ItemStackRenderState`.
        // ItemPlacer.isBlock(1): not translated, it uses Minecraft's `ItemStack`.
        pub fn flat_step(scale: f32) -> f32 {
            return ((0.205f32 * scale) / 16.0f32) + 0.0015f32;
        }
        pub fn copies(count: i32, block: bool) -> i32 {
            if block {
                return 1;
            } else {
                return if count >= 33 { 4 } else { if count >= 17 { 3 } else { if count >= 2 { 2 } else { 1 } } };
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
    }

    /// `JukeboxStation` (dev/dyslexic/cinematic/client/JukeboxStation.java)
    pub mod jukebox_station {
        use super::*;
        // The class itself isn't translated: it extends `ClientStation`. Its static parts are below.
        pub const RECORD_SIZE: f32 = 0.6f32;
        pub const DISC_R: f64 = (0.22500001f32 as f64);
        pub const INSERT: f64 = 1.3f64;
        pub const EJECT: f64 = 1.15f64;
        pub const LINGER: f64 = 0.55f64;
        pub const INSERT_SEND: f64 = 0.62f64;
        pub const EJECT_SEND: f64 = 0.16f64;
        // JukeboxStation.layoutEditor(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.JukeboxStation(2): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.worthIt(2): not translated, it uses Minecraft's `LocalPlayer`.
        // JukeboxStation.isRecord(1): not translated, it uses Minecraft's `ItemStack`.
        // JukeboxStation.kind(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.center(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.worksAtAnAngle(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.touchSound(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.playing(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.inside(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.approach(1): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.lean(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.leanFov(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.bodyPitch(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.rest(1): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.updateCursor(1): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.handTarget(4): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.heldItemPoint(2): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.click(2): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.begin(3): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.progress(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.tick(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.busy(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.recordPoint(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.recordTurn(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.handsOverride(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.edge(1): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.handsStiffness(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.renderWorld(4): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.hint(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.tooltip(1): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.moveName(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.beforeFrame(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.onEnd(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.showsInventory(0): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.renderScreen(3): not translated, its class isn't translated (it extends `ClientStation`).
        // JukeboxStation.workSamples(0): not translated, its class isn't translated (it extends `ClientStation`).
    }

    /// `JukeboxStation.Move` (dev/dyslexic/cinematic/client/JukeboxStation.java)
    pub mod jukebox_station_move {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Move {
            None,
            Insert,
            Eject,
        }
        impl Move {
            pub const VALUES: [Move; 3] = [Move::None, Move::Insert, Move::Eject, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Move::None => "NONE",
                    Move::Insert => "INSERT",
                    Move::Eject => "EJECT",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Move::None,
                    "INSERT" => Move::Insert,
                    "EJECT" => Move::Eject,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `KeyLabels` (dev/dyslexic/cinematic/client/KeyLabels.java)
    pub mod key_labels {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct KeyLabels {
        }
        impl KeyLabels {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // KeyLabels.of(1): not translated, it uses Minecraft's `Component`.
    }

    /// `KneeLayer` (dev/dyslexic/cinematic/client/KneeLayer.java)
    pub mod knee_layer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RenderLayer`. Its static parts are below.
        pub const THIGH: f32 = -2.1f32;
        pub const KNEE: f32 = 1.5f32;
        pub static BENDS: JCell<bool> = JCell::new(true);
        // KneeLayer.KneeLayer(2): not translated, its class isn't translated (it extends Minecraft's `RenderLayer`).
        // KneeLayer.sitting(1): not translated, it calls ClientPayloads.atStation, which isn't translated.
        // KneeLayer.submit(6): not translated, its class isn't translated (it extends Minecraft's `RenderLayer`).
        // KneeLayer.submitParts(7): not translated, it uses Minecraft's `SubmitNodeCollector`.
        // KneeLayer.submitArmor(8): not translated, its class isn't translated (it extends Minecraft's `RenderLayer`).
        // KneeLayer.armorModel(1): not translated, its body couldn't be read: an anonymous class.
        // KneeLayer.part(4): not translated, it uses Minecraft's `ModelPart`.
    }

    /// `LayoutEditor` (dev/dyslexic/cinematic/client/LayoutEditor.java)
    pub mod layout_editor {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LayoutEditor.editing(0): not translated, it has no body.
        // LayoutEditor.visible(0): not translated, it has no body.
        // LayoutEditor.over(2): not translated, it has no body.
        // LayoutEditor.click(3): not translated, it has no body.
        // LayoutEditor.drag(3): not translated, it has no body.
        // LayoutEditor.scroll(3): not translated, it has no body.
        // LayoutEditor.keyPressed(1): not translated, it has no body.
        // LayoutEditor.release(0): not translated, it has no body.
    }

    /// `LayoutFiles` (dev/dyslexic/cinematic/client/LayoutFiles.java)
    pub mod layout_files {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LayoutFiles {
        }
        impl LayoutFiles {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LayoutFiles.write(2): not translated, a catch block that uses its exception.
    }

    /// `LayoutProfiles` (dev/dyslexic/cinematic/client/LayoutProfiles.java)
    pub mod layout_profiles {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LayoutProfiles {
        }
        impl LayoutProfiles {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LayoutProfiles.name(1): not translated, it uses Minecraft's `Component`.
        // LayoutProfiles.value(1): not translated, it uses Minecraft's `Component`.
        // LayoutProfiles.selected(1): not translated, it calls FurnaceLayouts.selected, which isn't translated.
        // LayoutProfiles.select(2): not translated, it calls LayoutProfiles.selected, which isn't translated.
    }

    /// `LecternStation` (dev/dyslexic/cinematic/client/LecternStation.java)
    pub mod lectern_station {
        use super::*;
        // The class itself isn't translated: it extends `MenuStation`. Its static parts are below.
        // LecternStation.layoutEditor(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.LecternStation(2): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.kind(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.center(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.restPoint(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.touchSound(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.showsInventory(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.drawsBookAt(1): not translated, it uses Minecraft's `BlockPos`.
        // LecternStation.book(0): not translated, its class isn't translated (it extends `MenuStation`).
        pub fn page_of(spread: i32) -> i32 {
            return (0i32).max((2 * spread) - 1);
        }
        // LecternStation.sendPage(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.deny(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.taking(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.turn(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.take(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.fold(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.lift(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.beforeFrame(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.frame(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.tick(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.spread(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.pageCenter(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.approach(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.lean(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.leanFov(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.bodyPitch(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.rest(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.updateCursor(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.clearCursor(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.handTarget(4): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.heldItemPoint(2): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.handsOverride(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.handsStiffness(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.act(2): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.scroll(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.keyPressed(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.renderScene(4): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.renderWorld(4): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.hint(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.onEnd(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.busy(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.renderScreen(3): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.tooltip(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LecternStation.workSamples(0): not translated, its class isn't translated (it extends `MenuStation`).
    }

    /// `LecternStation.Target` (dev/dyslexic/cinematic/client/LecternStation.java)
    pub mod lectern_station_target {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Target {
            None,
            Left,
            Right,
        }
        impl Target {
            pub const VALUES: [Target; 3] = [Target::None, Target::Left, Target::Right, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Target::None => "NONE",
                    Target::Left => "LEFT",
                    Target::Right => "RIGHT",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Target::None,
                    "LEFT" => Target::Left,
                    "RIGHT" => Target::Right,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `LegMesh` (dev/dyslexic/cinematic/client/LegMesh.java)
    pub mod leg_mesh {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Cube`. Its static parts are below.
        pub const HALF: f32 = 6.0f32;
        pub const KNEE_OVERLAP: f32 = 2.0f32;
        pub const INSET: f32 = 0.02f32;
        // RIGHT: not translated, it uses LegMesh, which isn't translated (it uses Minecraft's `Cube`).
        // RIGHT_PANTS: not translated, it uses LegMesh, which isn't translated (it uses Minecraft's `Cube`).
        // LEFT: not translated, it uses LegMesh, which isn't translated (it uses Minecraft's `Cube`).
        // LEFT_PANTS: not translated, it uses LegMesh, which isn't translated (it uses Minecraft's `Cube`).
        // LegMesh.LegMesh(3): not translated, its class isn't translated (it uses Minecraft's `Cube`).
        // LegMesh.draw(10): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // LegMesh.segment(8): not translated, it uses Minecraft's `VertexConsumer`.
    }

    /// `LoomFlags` (dev/dyslexic/cinematic/client/LoomFlags.java)
    pub mod loom_flags {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LoomFlags {
        }
        impl LoomFlags {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `LoomFlags.Flag` (dev/dyslexic/cinematic/client/LoomFlags.java)
    pub mod loom_flags_flag {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `DyeColor`. Its static parts are below.
        // LoomFlags.Flag.Flag(5): not translated, its class isn't translated (it uses Minecraft's `DyeColor`).
        // LoomFlags.Flag.x(0): not translated, its class isn't translated (it uses Minecraft's `DyeColor`).
        // LoomFlags.Flag.y(0): not translated, its class isn't translated (it uses Minecraft's `DyeColor`).
        // LoomFlags.Flag.width(0): not translated, its class isn't translated (it uses Minecraft's `DyeColor`).
        // LoomFlags.Flag.base(0): not translated, its class isn't translated (it uses Minecraft's `DyeColor`).
        // LoomFlags.Flag.layers(0): not translated, its class isn't translated (it uses Minecraft's `DyeColor`).
    }

    /// `LoomFlags.Renderer` (dev/dyslexic/cinematic/client/LoomFlags.java)
    pub mod loom_flags_renderer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `PictureInPictureRenderer`. Its static parts are below.
        // LoomFlags.Renderer.getRenderStateClass(0): not translated, its class isn't translated (it extends Minecraft's `PictureInPictureRenderer`).
        // LoomFlags.Renderer.getTranslateY(2): not translated, its class isn't translated (it extends Minecraft's `PictureInPictureRenderer`).
        // LoomFlags.Renderer.renderToTexture(3): not translated, its class isn't translated (it extends Minecraft's `PictureInPictureRenderer`).
        // LoomFlags.Renderer.getTextureLabel(0): not translated, its class isn't translated (it extends Minecraft's `PictureInPictureRenderer`).
    }

    /// `LoomFlags.State` (dev/dyslexic/cinematic/client/LoomFlags.java)
    pub mod loom_flags_state {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `PictureInPictureRenderState`. Its static parts are below.
        // LoomFlags.State.State(8): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // LoomFlags.State.State(7): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // LoomFlags.State.scale(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // LoomFlags.State.flags(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // LoomFlags.State.model(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // LoomFlags.State.x0(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // LoomFlags.State.y0(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // LoomFlags.State.x1(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // LoomFlags.State.y1(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // LoomFlags.State.scissorArea(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
        // LoomFlags.State.bounds(0): not translated, its class isn't translated (it implements Minecraft's `PictureInPictureRenderState`).
    }

    /// `LoomStation` (dev/dyslexic/cinematic/client/LoomStation.java)
    pub mod loom_station {
        use super::*;
        // The class itself isn't translated: it extends `MenuStation`. Its static parts are below.
        pub const HAND_Y: f64 = 0.062f64;
        pub const ITEM_REACH: f64 = 0.075f64;
        pub const EDIT_HAND_DROP: f64 = 0.24f64;
        pub const EDIT_HAND_OUT: f64 = 0.12f64;
        pub const CLOTH_WIDTH: f64 = 0.42f64;
        pub const CLOTH_LENGTH: f64 = 0.84f64;
        pub static CORNERS: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![0.0f64, 0.0f64], vec![0.37f64, 0.34f64], vec![-0.37f64, 0.34f64]]);
        pub const WEAVE_LENGTH: f64 = 2.4f64;
        pub const WEAVE_FROM: f64 = 0.2f64;
        pub const WEAVE_TO: f64 = 1.85f64;
        pub const PASSES: i32 = 6;
        pub const BEAT: f64 = 2.0f64;
        pub const COLS: i32 = 5;
        pub const ROWS: i32 = 4;
        pub const CELL_W: i32 = 22;
        pub const CELL_H: i32 = 38;
        pub const PAD: i32 = 6;
        pub const HEADER: i32 = 13;
        // LoomStation.LoomStation(2): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.kind(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.center(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.touchSound(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.slotPoint(3): not translated, it uses Minecraft's `Vec3`.
        // LoomStation.restPoint(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.layoutEditor(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.layoutFrame(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.layoutItem(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.busy(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.showsInventory(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.beforeFrame(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.onEnd(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.workSamples(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.layoutHeldScale(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.clothAcross(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.clothNear(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.clothWidth(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.clothLength(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.itemRotation(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.layoutGhost(5): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.approach(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.lean(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.rest(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.patterns(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.full(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.baseOf(1): not translated, it uses Minecraft's `DyeColor`.
        // LoomStation.dye(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.scroll(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.updateCursor(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.clearCursor(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.handTarget(4): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.heldItemPoint(2): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.act(2): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.weave(2): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.woven(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.shuttleAcross(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.weaveLine(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.handsOverride(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.frame(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.tick(0): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.rgb(1): not translated, it uses Minecraft's `DyeColor`.
        // LoomStation.renderScene(4): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.drawWeaving(5): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.drawCloth(14): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // LoomStation.layer(14): not translated, it uses Minecraft's `VertexConsumer`.
        // LoomStation.renderWorld(4): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.renderScreen(3): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.renderBoard(3): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.boardCell(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.patternIndex(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.drawFlags(6): not translated, its class isn't translated (it extends `MenuStation`).
        pub fn argb(a: f32, r: i32, g: i32, b: i32) -> i32 {
            return ((((j_clamp(j_round_f(a * 255.0f32), 0, 255)) << 24) | (r << 16)) | (g << 8)) | b;
        }
        // LoomStation.drawContents(7): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // LoomStation.tooltip(1): not translated, its class isn't translated (it extends `MenuStation`).
        // LoomStation.hint(0): not translated, its class isn't translated (it extends `MenuStation`).
    }

    /// `LoomStation.Target` (dev/dyslexic/cinematic/client/LoomStation.java)
    pub mod loom_station_target {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Target {
            None,
            Place,
            Item,
            Pattern,
        }
        impl Target {
            pub const VALUES: [Target; 4] = [Target::None, Target::Place, Target::Item, Target::Pattern, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Target::None => "NONE",
                    Target::Place => "PLACE",
                    Target::Item => "ITEM",
                    Target::Pattern => "PATTERN",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Target::None,
                    "PLACE" => Target::Place,
                    "ITEM" => Target::Item,
                    "PATTERN" => Target::Pattern,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `MenuRenderer` (dev/dyslexic/cinematic/client/MenuRenderer.java)
    pub mod menu_renderer {
        use super::*;
        pub const MAX_DISTANCE_SQR: f64 = 2304.0f64;
        #[derive(Clone, Debug)]
        pub struct MenuRenderer {
        }
        impl MenuRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MenuRenderer.renderAll(5): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // MenuRenderer.onlookerFx(9): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
    }

    /// `MenuStation` (dev/dyslexic/cinematic/client/MenuStation.java)
    pub mod menu_station {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MenuStation.MenuStation(4): not translated, its class isn't translated (it's generic).
        // MenuStation.menu(0): not translated, its class isn't translated (it's generic).
        // MenuStation.slot(1): not translated, its class isn't translated (it's generic).
        // MenuStation.slotClick(3): not translated, its class isn't translated (it's generic).
        // MenuStation.accepts(2): not translated, its class isn't translated (it's generic).
        // MenuStation.slotFor(2): not translated, its class isn't translated (it's generic).
        // MenuStation.fits(2): not translated, its class isn't translated (it's generic).
        // MenuStation.startMove(3): not translated, its class isn't translated (it's generic).
        // MenuStation.showMove(1): not translated, its class isn't translated (it's generic).
        // MenuStation.moveTime(0): not translated, its class isn't translated (it's generic).
        // MenuStation.moving(0): not translated, its class isn't translated (it's generic).
        // MenuStation.frame(0): not translated, its class isn't translated (it's generic).
        // MenuStation.cancelMove(0): not translated, its class isn't translated (it's generic).
        // MenuStation.deny(0): not translated, its class isn't translated (it's generic).
        // MenuStation.denyShake(0): not translated, its class isn't translated (it's generic).
        // MenuStation.click(2): not translated, its class isn't translated (it's generic).
        // MenuStation.busy(0): not translated, its class isn't translated (it's generic).
        // MenuStation.act(2): not translated, it has no body.
        // MenuStation.onEnd(0): not translated, its class isn't translated (it's generic).
        // MenuStation.renderScene(4): not translated, it has no body.
        // MenuStation.leftBehind(0): not translated, its class isn't translated (it's generic).
        // MenuStation.restPoint(1): not translated, it has no body.
        // MenuStation.drawLeaving(4): not translated, its class isn't translated (it's generic).
        // MenuStation.outro(0): not translated, its class isn't translated (it's generic).
    }

    /// `ModMenuIntegration` (dev/dyslexic/cinematic/client/ModMenuIntegration.java)
    pub mod mod_menu_integration {
        use super::*;
        // The class itself isn't translated: it implements `ModMenuApi` (com.terraformersmc.modmenu.api). Its static parts are below.
        // ModMenuIntegration.getModConfigScreenFactory(0): not translated, its class isn't translated (it implements `ModMenuApi` (com.terraformersmc.modmenu.api)).
    }

    /// `MultiplayerSettingsScreen` (dev/dyslexic/cinematic/client/MultiplayerSettingsScreen.java)
    pub mod multiplayer_settings_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        // MultiplayerSettingsScreen.MultiplayerSettingsScreen(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // MultiplayerSettingsScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // MultiplayerSettingsScreen.label(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // MultiplayerSettingsScreen.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // MultiplayerSettingsScreen.onClose(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `OpenBook` (dev/dyslexic/cinematic/client/OpenBook.java)
    pub mod open_book {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        pub const PAGE_W: f64 = 0.4f64;
        pub const PAGE_H: f64 = 0.500763358778626f64;
        pub const GUTTER: f64 = 0.007f64;
        pub const MARGIN: f64 = 0.014f64;
        pub const COVER: f64 = 0.012f64;
        pub const STACK: f64 = 0.032f64;
        pub const LEAF: f64 = 0.0012f64;
        pub const TURN: f64 = 0.62f64;
        pub const RIFFLE: f64 = 0.34f64;
        pub const COLS: i32 = 16;
        pub const ROWS: i32 = 6;
        pub const DEEP: f64 = 0.42100000000000004f64;
        pub const TALL: f64 = 0.528763358778626f64;
        pub const THICK: f64 = 0.056f64;
        pub static BOOK_GUI: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "textures/gui/book.png"));
        pub const PU0: f32 = 0.2734375f32;
        pub const PU1: f32 = 0.296875f32;
        pub const PV0: f32 = 0.234375f32;
        pub const PV1: f32 = 0.3515625f32;
        pub static LEATHER: LazyLock<open_book_look::Look> = LazyLock::new(|| open_book_look::Look::new("entity/enchantment/enchanting_table_book", 0.015625f32, 0.03125f32, 0.140625f32, 0.25f32, 0.015625f32, 0.03125f32, 0.140625f32, 0.25f32));
        // OpenBook.place(4): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.read(1): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.lastSpread(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.pageCount(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.openAt(1): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.share(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.turn(1): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.follow(1): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.riffle(1): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.turning(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.settle(3): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.preparePages(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.leaves(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.still(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.under(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.corner(1): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.rest(1): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.pageCenter(1): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.at(3): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.turningHands(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.foldCorner(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.stack(1): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        pub fn surface(x: f64, stack: f64) -> f64 {
            let k: f64 = 1.0f64.min(x / 0.096f64);
            return (stack * (0.42f64 + (0.58f64 * (((k * std::f64::consts::PI) / 2.0f64).sin())))) + (0.004f64 * ((std::f64::consts::PI * (1.0f64.min(x / 0.40700000000000003f64))).sin()));
        }
        // OpenBook.lying(4): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        pub fn angle(leaf: &open_book_leaf::Leaf, x: f64, v: f64) -> f64 {
            let lead: f64 = 0.75f64 - (0.5f64 * (v / 0.500763358778626f64));
            let out: f64 = (x / 0.40700000000000003f64).powf(1.5f64);
            return 0.0f64.max(std::f64::consts::PI.min((std::f64::consts::PI * leaf.p()) + ((leaf.curl() * lead) * out)));
        }
        // OpenBook.leafPoints(1): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.foldLeaf(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.render(4): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.boardUv(2): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.drawCover(4): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.drawStack(6): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.drop(1): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.drawLeafFace(7): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.drawCoverFace(5): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.drawShut(4): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.slab(11): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.drawGrid(9): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // OpenBook.reverse(1): needs the game, so it's in game_side.rs.
        // OpenBook.face(11): not translated, it uses Minecraft's `VertexConsumer`.
        // OpenBook.quad(8): not translated, it uses Minecraft's `VertexConsumer`.
    }

    /// `OpenBook.Leaf` (dev/dyslexic/cinematic/client/OpenBook.java)
    pub mod open_book_leaf {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Leaf {
            pub p: f64,
            pub curl: f64,
            pub right: i32,
            pub left: i32,
            pub lift: f64,
        }
        impl Leaf {
            pub fn new(p: f64, curl: f64, right: i32, left: i32, lift: f64) -> Self {
                let mut s = Self { p: p, curl: curl, right: right, left: left, lift: lift };
                s.p = p;
                s.curl = curl;
                s.right = right;
                s.left = left;
                s.lift = lift;
                s
            }
            pub fn p(&self) -> f64 {
                return self.p;
            }
            pub fn curl(&self) -> f64 {
                return self.curl;
            }
            pub fn right(&self) -> i32 {
                return self.right;
            }
            pub fn left(&self) -> i32 {
                return self.left;
            }
            pub fn lift(&self) -> f64 {
                return self.lift;
            }
        }
    }

    /// `OpenBook.Look` (dev/dyslexic/cinematic/client/OpenBook.java)
    pub mod open_book_look {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Look {
            pub texture: String,
            pub u0: f32,
            pub v0: f32,
            pub u1: f32,
            pub v1: f32,
            pub su0: f32,
            pub sv0: f32,
            pub su1: f32,
            pub sv1: f32,
        }
        impl Look {
            pub fn new(texture: &str, u0: f32, v0: f32, u1: f32, v1: f32, su0: f32, sv0: f32, su1: f32, sv1: f32) -> Self {
                let mut s = Self { texture: String::new(), u0: u0, v0: v0, u1: u1, v1: v1, su0: su0, sv0: sv0, su1: su1, sv1: sv1 };
                s.texture = texture.to_string();
                s.u0 = u0;
                s.v0 = v0;
                s.u1 = u1;
                s.v1 = v1;
                s.su0 = su0;
                s.sv0 = sv0;
                s.su1 = su1;
                s.sv1 = sv1;
                s
            }
            pub fn texture(&self) -> String {
                return self.texture.to_string();
            }
            pub fn u0(&self) -> f32 {
                return self.u0;
            }
            pub fn v0(&self) -> f32 {
                return self.v0;
            }
            pub fn u1(&self) -> f32 {
                return self.u1;
            }
            pub fn v1(&self) -> f32 {
                return self.v1;
            }
            pub fn su0(&self) -> f32 {
                return self.su0;
            }
            pub fn sv0(&self) -> f32 {
                return self.sv0;
            }
            pub fn su1(&self) -> f32 {
                return self.su1;
            }
            pub fn sv1(&self) -> f32 {
                return self.sv1;
            }
        }
    }

    /// `OpenBook.Side` (dev/dyslexic/cinematic/client/OpenBook.java)
    pub mod open_book_side {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Side {
            None,
            Left,
            Right,
        }
        impl Side {
            pub const VALUES: [Side; 3] = [Side::None, Side::Left, Side::Right, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Side::None => "NONE",
                    Side::Left => "LEFT",
                    Side::Right => "RIGHT",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Side::None,
                    "LEFT" => Side::Left,
                    "RIGHT" => Side::Right,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `OpenBook.Turn` (dev/dyslexic/cinematic/client/OpenBook.java)
    pub mod open_book_turn {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Turn {
            pub at: f64,
            pub length: f64,
            pub from: i32,
            pub dir: i32,
        }
        impl Turn {
            pub fn new(at: f64, length: f64, from: i32, dir: i32) -> Self {
                let mut s = Self { at: at, length: length, from: from, dir: dir };
                s.at = at;
                s.length = length;
                s.from = from;
                s.dir = dir;
                s
            }
            pub fn phase(&self) -> f64 {
                return anim::clamp01((scene::clock() - self.at) / self.length);
            }
            pub fn at(&self) -> f64 {
                return self.at;
            }
            pub fn length(&self) -> f64 {
                return self.length;
            }
            pub fn from(&self) -> i32 {
                return self.from;
            }
            pub fn dir(&self) -> i32 {
                return self.dir;
            }
        }
    }

    /// `OpenBook.Turned` (dev/dyslexic/cinematic/client/OpenBook.java)
    pub mod open_book_turned {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Turned {
            Yes,
            Wait,
            NoPage,
        }
        impl Turned {
            pub const VALUES: [Turned; 3] = [Turned::Yes, Turned::Wait, Turned::NoPage, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Turned::Yes => "YES",
                    Turned::Wait => "WAIT",
                    Turned::NoPage => "NO_PAGE",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "YES" => Turned::Yes,
                    "WAIT" => Turned::Wait,
                    "NO_PAGE" => Turned::NoPage,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `Overlay` (dev/dyslexic/cinematic/client/Overlay.java)
    pub mod overlay {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Overlay {
        }
        impl Overlay {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Overlay.bars(0): not translated, a `yield` outside a switch expression's last statement.
        // Overlay.render(1): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
    }

    /// `Picking` (dev/dyslexic/cinematic/client/Picking.java)
    pub mod picking {
        use super::*;
        pub static VIEW_PROJ: JCell<JMat4> = JCell::new(JMat4::IDENTITY);
        pub static INVERSE: LazyLock<JMat4> = LazyLock::new(|| JMat4::IDENTITY);
        // camera: not translated, it uses Minecraft's `Vec3`.
        pub static VALID: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct Picking {
        }
        impl Picking {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Picking.capture(3): not translated, it uses Minecraft's `Vec3`.
        // Picking.camera(0): not translated, it uses Minecraft's `Vec3`.
        // Picking.mouseNdc(0): not translated, it uses Minecraft's `Minecraft`.
        // Picking.mouseGui(0): not translated, it uses Minecraft's `Minecraft`.
        // Picking.cursorRay(0): not translated, it uses Minecraft's `Vec3`.
        // Picking.cursorOnPlane(1): not translated, it uses Minecraft's `Vec3`.
        // Picking.cursorOnPlane(2): not translated, it uses Minecraft's `Vec3`.
        // Picking.toGui(1): not translated, it uses Minecraft's `Vec3`.
    }

    /// `PropLayer` (dev/dyslexic/cinematic/client/PropLayer.java)
    pub mod prop_layer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RenderLayer`. Its static parts are below.
        // PropLayer.PropLayer(1): not translated, its class isn't translated (it extends Minecraft's `RenderLayer`).
        // PropLayer.hammer(1): not translated, it's overloaded with the same number of parameters.
        // PropLayer.hammer(1): not translated, it's overloaded with the same number of parameters.
        // PropLayer.submit(6): not translated, its class isn't translated (it extends Minecraft's `RenderLayer`).
    }

    /// `QuickMoves` (dev/dyslexic/cinematic/client/QuickMoves.java)
    pub mod quick_moves {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct QuickMoves {
        }
        impl QuickMoves {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // QuickMoves.sendVanilla(3): not translated, it uses Minecraft's `Minecraft`.
    }

    /// `Reach` (dev/dyslexic/cinematic/client/Reach.java)
    pub mod reach {
        use super::*;
        pub const SURFACE_HIGH: f64 = 1.42f64;
        pub const FACE_HIGH: f64 = 1.85f64;
        pub const LOW: f64 = -0.6f64;
        pub const FAR: f64 = 1.5f64;
        pub const FURTHER_OUT: f64 = 0.9f64;
        pub const ROUTE_RADIUS: i32 = 10;
        pub static BESIDE_ANGLES: LazyLock<Vec<f64>> = LazyLock::new(|| vec![55.0f64, 40.0f64, 70.0f64]);
        pub static BESIDE_REACH: LazyLock<Vec<f64>> = LazyLock::new(|| vec![0.95f64, 1.1f64, 1.25f64, 1.4f64]);
        pub const HARM_MARGIN: f64 = 0.12f64;
        pub const CAMERA_AHEAD: f64 = 1.05f64;
        pub const CAMERA_DOWN: f64 = 0.95f64;
        pub const CAMERA_UP: f64 = 0.16f64;
        pub const LEAN_OVER: f64 = 1.2f64;
        pub const LEAN_OVER_GAP: f64 = 0.45f64;
        pub const LEAN_OVER_RISE: f64 = 0.35f64;
        #[derive(Clone, Debug)]
        pub struct Reach {
        }
        impl Reach {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Reach.plan(2): not translated, it uses ClientStation, which isn't translated (it uses Minecraft's `BlockPos`).
        // Reach.planHere(2): not translated, it uses ClientStation, which isn't translated (it uses Minecraft's `BlockPos`).
        // Reach.planBeside(2): not translated, it uses ClientStation, which isn't translated (it uses Minecraft's `BlockPos`).
        // Reach.judge(4): not translated, it uses ClientStation, which isn't translated (it uses Minecraft's `BlockPos`).
        // Reach.candidates(2): not translated, it uses Minecraft's `Vec3`.
        // Reach.firm(3): not translated, it uses Minecraft's `Level`.
        // Reach.standingOn(2): not translated, it uses Minecraft's `BlockPos`.
        // Reach.hidden(3): not translated, it uses Minecraft's `BlockHitResult`.
        // Reach.inReach(3): needs the game, so it's in game_side.rs.
        // Reach.walkable(3): not translated, it uses Minecraft's `Level`.
        // Reach.clear(4): not translated, it uses Minecraft's `Level`.
        // Reach.standable(3): not translated, it uses Minecraft's `Level`.
        // Reach.harmless(1): not translated, it uses Minecraft's `BlockState`.
        // Reach.tethered(3): not translated, it uses Pose, which isn't translated (it uses Minecraft's `Vec3`).
        // Reach.cameraPose(3): not translated, it uses Pose, which isn't translated (it uses Minecraft's `Vec3`).
        // Reach.lookPoint(2): not translated, it uses Minecraft's `Vec3`.
    }

    /// `Reach.Plan` (dev/dyslexic/cinematic/client/Reach.java)
    pub mod reach_plan {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // Reach.Plan.Plan(6): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // Reach.Plan.scene(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // Reach.Plan.path(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // Reach.Plan.feet(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // Reach.Plan.why(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // Reach.Plan.reason(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // Reach.Plan.blocker(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `Reach.Reason` (dev/dyslexic/cinematic/client/Reach.java)
    pub mod reach_reason {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Reason {
            TooHigh,
            TooLow,
            TooFar,
            NoWay,
            OnTop,
            WrongSide,
            Covered,
            Blocked,
            NoSeat,
        }
        impl Reason {
            pub const VALUES: [Reason; 9] = [Reason::TooHigh, Reason::TooLow, Reason::TooFar, Reason::NoWay, Reason::OnTop, Reason::WrongSide, Reason::Covered, Reason::Blocked, Reason::NoSeat, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Reason::TooHigh => "TOO_HIGH",
                    Reason::TooLow => "TOO_LOW",
                    Reason::TooFar => "TOO_FAR",
                    Reason::NoWay => "NO_WAY",
                    Reason::OnTop => "ON_TOP",
                    Reason::WrongSide => "WRONG_SIDE",
                    Reason::Covered => "COVERED",
                    Reason::Blocked => "BLOCKED",
                    Reason::NoSeat => "NO_SEAT",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "TOO_HIGH" => Reason::TooHigh,
                    "TOO_LOW" => Reason::TooLow,
                    "TOO_FAR" => Reason::TooFar,
                    "NO_WAY" => Reason::NoWay,
                    "ON_TOP" => Reason::OnTop,
                    "WRONG_SIDE" => Reason::WrongSide,
                    "COVERED" => Reason::Covered,
                    "BLOCKED" => Reason::Blocked,
                    "NO_SEAT" => Reason::NoSeat,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `Reach.Routes` (dev/dyslexic/cinematic/client/Reach.java)
    pub mod reach_routes {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Level`. Its static parts are below.
        // Reach.Routes.Routes(2): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // Reach.Routes.to(1): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // Reach.Routes.step(3): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // Reach.Routes.middle(1): not translated, its class isn't translated (it uses Minecraft's `Level`).
    }

    /// `ReachNotice` (dev/dyslexic/cinematic/client/ReachNotice.java)
    pub mod reach_notice {
        use super::*;
        pub const FADE_IN: f64 = 0.12f64;
        pub const HOLD: f64 = 1.9f64;
        pub const FADE_OUT: f64 = 0.4f64;
        pub const TEXT_ALPHA_CUTOFF: i32 = 8;
        pub const FROM_BOTTOM: i32 = 70;
        // line: not translated, it uses Minecraft's `Component`.
        pub static SHOWN_AT: JCell<f64> = JCell::new(-100.0f64);
        #[derive(Clone, Debug)]
        pub struct ReachNotice {
        }
        impl ReachNotice {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ReachNotice.show(2): not translated, it uses Minecraft's `Component`.
        // ReachNotice.text(0): not translated, it uses ReachNotice.line, which isn't translated (it uses Minecraft's `Component`).
        pub fn alpha() -> f32 {
            let age: f64 = scene::clock() - reach_notice::SHOWN_AT.get();
            if (!(age < 0.0f64)) && (!(age > 2.42f64)) {
                let k: f64 = if age < 0.12f64 { age / 0.12f64 } else { if age > 2.02f64 { 1.0f64 - (((age - 0.12f64) - 1.9f64) / 0.4f64) } else { 1.0f64 } };
                return (anim::smoother(k) as f32);
            } else {
                return 0.0f32;
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        // ReachNotice.render(1): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        pub fn argb(a: f32, r: i32, g: i32, b: i32) -> i32 {
            return (((((0i32).max((255i32).min(j_round_f(a * 255.0f32)))) << 24) | (r << 16)) | (g << 8)) | b;
        }
    }

    /// `RecipeChoiceStrip` (dev/dyslexic/cinematic/client/RecipeChoiceStrip.java)
    pub mod recipe_choice_strip {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
        pub const CELL: i32 = 20;
        pub const PAD: i32 = 4;
        pub const TITLE: i32 = 11;
        // RecipeChoiceStrip.set(2): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // RecipeChoiceStrip.clear(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // RecipeChoiceStrip.active(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // RecipeChoiceStrip.selected(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // RecipeChoiceStrip.selectedResult(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // RecipeChoiceStrip.render(3): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // RecipeChoiceStrip.over(2): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // RecipeChoiceStrip.at(2): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // RecipeChoiceStrip.click(2): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // RecipeChoiceStrip.send(3): not translated, it uses Minecraft's `BlockPos`.
        // RecipeChoiceStrip.tooltip(2): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
    }

    /// `RemainingLayouts` (dev/dyslexic/cinematic/client/RemainingLayouts.java)
    pub mod remaining_layouts {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct RemainingLayouts {
        }
        impl RemainingLayouts {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn supports(kind: station_kind::StationKind) -> bool {
            return match kind {
                station_kind::StationKind::Anvil | station_kind::StationKind::Grindstone | station_kind::StationKind::Stonecutter | station_kind::StationKind::Campfire | station_kind::StationKind::Lectern | station_kind::StationKind::Beacon | station_kind::StationKind::Jukebox => true,
                _ => false,
            };
        }
        pub fn billboard(kind: station_kind::StationKind, slot: i32) -> bool {
            return (((kind == station_kind::StationKind::Beacon) && (slot > 0)) || ((kind == station_kind::StationKind::Anvil) && (slot == 2))) || ((kind == station_kind::StationKind::Stonecutter) && (slot == 2));
        }
        // RemainingLayouts.frame(3): not translated, it uses Minecraft's `BlockPos`.
        // RemainingLayouts.frameAt(3): not translated, it uses Minecraft's `Vec3`.
        // RemainingLayouts.base(2): not translated, it uses Minecraft's `Vec3`.
        // RemainingLayouts.half(2): not translated, a switch expression giving a double[].
        // RemainingLayouts.point(4): not translated, it uses Minecraft's `Vec3`.
        // RemainingLayouts.transform(5): not translated, it uses Minecraft's `Vec3`.
        // RemainingLayouts.scale(2): not translated, it calls SpatialLayouts.current, which isn't translated.
        // RemainingLayouts.rotation(2): not translated, it calls SpatialLayouts.current, which isn't translated.
        // RemainingLayouts.samples(2): not translated, it uses Minecraft's `Vec3`.
        // RemainingLayouts.item(2): not translated, it uses Minecraft's `ItemStack`.
        // RemainingLayouts.ghost(6): not translated, it uses SpatialTarget, which isn't translated (it's an interface).
    }

    /// `Scene` (dev/dyslexic/cinematic/client/Scene.java)
    pub mod scene {
        use super::*;
        pub const OUTRO_LENGTH: f64 = 1.05f64;
        pub const LEAN_LENGTH: f64 = 1.2f64;
        pub const QUICK_LEAN_LENGTH: f64 = 0.4f64;
        pub static LEAN_LENGTH_2: JCell<f64> = JCell::new(1.2f64);
        pub static QUICK: JCell<bool> = JCell::new(false);
        // pendingPos: not translated, it uses Minecraft's `BlockPos`.
        pub static PENDING_KIND: LazyLock<JCell<Option<station_kind::StationKind>>> = LazyLock::new(|| JCell::new(None));
        pub static PENDING_AT: JCell<f64> = JCell::new(0.0f64);
        // landingPos: not translated, it uses Minecraft's `BlockPos`.
        pub static LANDING_KIND: LazyLock<JCell<Option<station_kind::StationKind>>> = LazyLock::new(|| JCell::new(None));
        pub static LANDING_TICKS: JCell<i32> = JCell::new(0i32);
        pub const LANDING_WAIT: i32 = 30;
        // leftPos: not translated, it uses Minecraft's `BlockPos`.
        pub static LEFT_AT: JCell<f64> = JCell::new(-1000.0f64);
        pub const WALK_SHARE: f64 = 0.3f64;
        pub const STEP_BACK: f64 = 0.9f64;
        pub const UI_FADE_OUT: f64 = 0.22f64;
        pub static PHASE: JCell<scene_phase::Phase> = JCell::new(scene_phase::Phase::None);
        // station: not translated, it uses ClientStation, which isn't translated (it uses Minecraft's `BlockPos`).
        pub static TIME: JCell<f64> = JCell::new(0.0f64);
        pub static INTRO_LENGTH: JCell<f64> = JCell::new(1.2f64);
        pub static WALK_SHARE_2: JCell<f64> = JCell::new(0.0f64);
        pub static ARRIVED_AT: JCell<f64> = JCell::new(-1.0f64);
        pub static WALK_ESTIMATE: JCell<f64> = JCell::new(0.0f64);
        pub static CROUCH: JCell<bool> = JCell::new(false);
        pub static STEP_BACK_2: JCell<bool> = JCell::new(false);
        pub static OUTRO_SECONDS: JCell<f64> = JCell::new(0.0f64);
        pub static BEST_DISTANCE: JCell<f64> = JCell::new(0.0f64);
        pub static PROGRESS_AT: JCell<f64> = JCell::new(0.0f64);
        pub static REFUSED: LazyLock<JCell<String>> = LazyLock::new(|| JCell::new("".to_string()));
        pub static FRAME_DT: JCell<f64> = JCell::new(0.0f64);
        pub static CLOCK: JCell<f64> = JCell::new(0.0f64);
        pub static LAST_NANOS: JCell<i64> = JCell::new(0i64);
        // path: a game value, so it's in game_side.rs.
        pub static WAYPOINT: JCell<i32> = JCell::new(0i32);
        pub static LEAVE_SENT: JCell<bool> = JCell::new(false);
        pub static CLIENT_ONLY: JCell<bool> = JCell::new(false);
        pub static CLIENT_MENU_ID: JCell<i32> = JCell::new(-1);
        // savedCamera: not translated, it uses Minecraft's `CameraType`.
        // outroFrom: not translated, it uses Pose, which isn't translated (it uses Minecraft's `Vec3`).
        pub static OUTRO_WEIGHT: JCell<f64> = JCell::new(1.0f64);
        pub static LAST_HURT_TIME: JCell<i32> = JCell::new(0i32);
        pub static FALLEN: JCell<f64> = JCell::new(0.0f64);
        pub static OVER_PANEL: JCell<bool> = JCell::new(false);
        pub static CURSOR_ENGAGED: JCell<bool> = JCell::new(false);
        pub static MOUSE_START_X: JCell<f64> = JCell::new(0.0f64);
        pub static MOUSE_START_Y: JCell<f64> = JCell::new(0.0f64);
        pub static CONSULTING: JCell<bool> = JCell::new(false);
        // neighbor: not translated, it uses Minecraft's `BlockPos`.
        pub static NEIGHBOR_KIND: LazyLock<JCell<Option<station_kind::StationKind>>> = LazyLock::new(|| JCell::new(None));
        pub static TRANSITION: JCell<bool> = JCell::new(false);
        // transitionFrom: not translated, it uses Pose, which isn't translated (it uses Minecraft's `Vec3`).
        pub const NEIGHBOR_REACH: f64 = 4.5f64;
        #[derive(Clone, Debug)]
        pub struct Scene {
        }
        impl Scene {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn clock() -> f64 {
            return scene::CLOCK.get();
        }
        // Scene.active(0): not translated, it uses Scene.station, which isn't translated (it uses ClientStation, which isn't translated (it uses Minecraft's `BlockPos`)).
        // Scene.consult(1): not translated, `instanceof`.
        // Scene.consulted(0): not translated, it calls Scene.stillAt, which isn't translated.
        // Scene.stillAt(0): not translated, it uses Minecraft's `Minecraft`.
        // Scene.clientOnly(0): not translated, it calls Scene.active, which isn't translated.
        pub fn intro_progress() -> f64 {
            if scene::PHASE.get() != scene_phase::Phase::Intro {
                return if scene::PHASE.get() == scene_phase::Phase::None { 0.0f64 } else { 1.0f64 };
            } else {
                return if scene::ARRIVED_AT.get() < 0.0f64 { 0.3f64 * (0.98f64.min(scene::TIME.get() / (0.2f64.max(scene::WALK_ESTIMATE.get())))) } else { scene::WALK_SHARE_2.get() + ((1.0f64 - scene::WALK_SHARE_2.get()) * (anim::clamp01((scene::TIME.get() - scene::ARRIVED_AT.get()) / scene::LEAN_LENGTH_2.get()))) };
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        pub fn arrived() -> bool {
            return (scene::PHASE.get() != scene_phase::Phase::Intro) || (scene::ARRIVED_AT.get() >= 0.0f64);
        }
        // Scene.introSecondsUntil(1): not translated, it calls Settings.approachSpeed, which isn't translated.
        // Scene.station(1): not translated, it's generic.
        // Scene.begin(2): not translated, it uses Minecraft's `BlockPos`.
        // Scene.beginClientOnly(2): not translated, it uses Minecraft's `BlockPos`.
        // Scene.choose(3): not translated, it uses Minecraft's `BlockPos`.
        // Scene.startIntro(2): not translated, it uses Plan, which isn't translated (it uses Minecraft's `Vec3`).
        // Scene.findNeighbor(0): not translated, it uses Scene.neighbor, which isn't translated (it uses Minecraft's `BlockPos`).
        // Scene.moveOver(0): not translated, it uses Minecraft's `Minecraft`.
        pub fn released() -> f64 {
            return if scene::TRANSITION.get() && (scene::WALK_SHARE_2.get() > 0.0f64) { anim::smoother(if (scene::PHASE.get() == scene_phase::Phase::Intro) && (scene::ARRIVED_AT.get() < 0.0f64) { scene::TIME.get() / 0.45f64 } else { 1.0f64 }) } else { 0.0f64 };
        }
        // Scene.create(5): not translated, it uses ClientStation, which isn't translated (it uses Minecraft's `BlockPos`).
        // Scene.leave(0): not translated, it uses Scene.leftPos, which isn't translated (it uses Minecraft's `BlockPos`).
        // Scene.serverClosed(0): not translated, it calls Scene.leave, which isn't translated.
        // Scene.abort(0): not translated, it calls Scene.sendLeave, which isn't translated.
        // Scene.fallBack(0): not translated, it uses ClientStation, which isn't translated (it uses Minecraft's `BlockPos`).
        // Scene.refuse(3): not translated, it uses ClientStation, which isn't translated (it uses Minecraft's `BlockPos`).
        // Scene.openWindow(2): not translated, it uses Minecraft's `BlockPos`.
        // Scene.window(2): not translated, it uses StationActionPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // Scene.drop(0): not translated, it calls Scene.sendLeave, which isn't translated.
        // Scene.finish(1): not translated, it uses Minecraft's `Minecraft`.
        // Scene.sendLeave(0): not translated, it uses Minecraft's `LocalPlayer`.
        // Scene.enterActive(0): not translated, it uses Minecraft's `Minecraft`.
        // Scene.frame(0): not translated, it uses Minecraft's `Minecraft`.
        // Scene.airborne(1): not translated, it uses Minecraft's `LocalPlayer`.
        // Scene.tickLanding(1): not translated, it uses Minecraft's `Minecraft`.
        // Scene.tick(0): not translated, it uses Minecraft's `Minecraft`.
        // Scene.steer(1): not translated, it uses Minecraft's `LocalPlayer`.
        // Scene.movementInput(1): not translated, it uses Minecraft's `ClientInput`.
        // Scene.updateCursor(0): not translated, it calls Scene.active, which isn't translated.
        // Scene.sound(3): not translated, it uses Minecraft's `SoundEvent`.
        // Scene.yawForward(1): not translated, it uses Minecraft's `Vec3`.
    }

    /// `Scene.Choice` (dev/dyslexic/cinematic/client/Scene.java)
    pub mod scene_choice {
        use super::*;
        // The class itself isn't translated: it uses ClientStation, which isn't translated (it uses Minecraft's `BlockPos`). Its static parts are below.
        // Scene.Choice.Choice(2): not translated, its class isn't translated (it uses ClientStation, which isn't translated (it uses Minecraft's `BlockPos`)).
        // Scene.Choice.station(0): not translated, its class isn't translated (it uses ClientStation, which isn't translated (it uses Minecraft's `BlockPos`)).
        // Scene.Choice.plan(0): not translated, its class isn't translated (it uses ClientStation, which isn't translated (it uses Minecraft's `BlockPos`)).
    }

    /// `Scene.Phase` (dev/dyslexic/cinematic/client/Scene.java)
    pub mod scene_phase {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Phase {
            None,
            Intro,
            Active,
            Outro,
        }
        impl Phase {
            pub const VALUES: [Phase; 4] = [Phase::None, Phase::Intro, Phase::Active, Phase::Outro, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Phase::None => "NONE",
                    Phase::Intro => "INTRO",
                    Phase::Active => "ACTIVE",
                    Phase::Outro => "OUTRO",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Phase::None,
                    "INTRO" => Phase::Intro,
                    "ACTIVE" => Phase::Active,
                    "OUTRO" => Phase::Outro,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `SceneBuffers` (dev/dyslexic/cinematic/client/SceneBuffers.java)
    pub mod scene_buffers {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SceneBuffers.getBuffer(1): not translated, it has no body.
    }

    /// `SceneBuffers.BufferSource` (dev/dyslexic/cinematic/client/SceneBuffers.java)
    pub mod scene_buffers_buffer_source {
        use super::*;
        // The class itself isn't translated: it implements `SceneBuffers`. Its static parts are below.
        // SceneBuffers.BufferSource.BufferSource(1): not translated, its class isn't translated (it implements `SceneBuffers`).
        // SceneBuffers.BufferSource.getBuffer(1): not translated, its class isn't translated (it implements `SceneBuffers`).
        // SceneBuffers.BufferSource.endBatch(0): not translated, its class isn't translated (it implements `SceneBuffers`).
        // SceneBuffers.BufferSource.endBatch(1): not translated, its class isn't translated (it implements `SceneBuffers`).
    }

    /// `SceneBuffers.Recording` (dev/dyslexic/cinematic/client/SceneBuffers.java)
    pub mod scene_buffers_recording {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `VertexConsumer`. Its static parts are below.
        // SceneBuffers.Recording.addVertex(3): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // SceneBuffers.Recording.setColor(4): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // SceneBuffers.Recording.setColor(1): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // SceneBuffers.Recording.setUv(2): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // SceneBuffers.Recording.setUv1(2): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // SceneBuffers.Recording.setUv2(2): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // SceneBuffers.Recording.setUv3(2): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // SceneBuffers.Recording.setNormal(3): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // SceneBuffers.Recording.setLineWidth(1): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
    }

    /// `SceneBuffers.Vertex` (dev/dyslexic/cinematic/client/SceneBuffers.java)
    pub mod scene_buffers_vertex {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Vertex {
            pub x: f32,
            pub y: f32,
            pub z: f32,
            pub u: f32,
            pub v: f32,
            pub uv3x: f32,
            pub uv3y: f32,
            pub nx: f32,
            pub ny: f32,
            pub nz: f32,
            pub line: f32,
            pub color: i32,
            pub overlay: i32,
            pub light: i32,
            pub attributes: i32,
        }
        impl Vertex {
            pub fn new() -> Self {
                let mut s = Self { x: 0.0f32, y: 0.0f32, z: 0.0f32, u: 0.0f32, v: 0.0f32, uv3x: 0.0f32, uv3y: 0.0f32, nx: 0.0f32, ny: 0.0f32, nz: 0.0f32, line: 1.0f32, color: -1, overlay: 0i32, light: 0i32, attributes: 0i32 };
                s
            }
            // SceneBuffers.Vertex.write(1): not translated, it uses Minecraft's `VertexConsumer`.
        }
    }

    /// `SceneFrameAccess` (dev/dyslexic/cinematic/client/SceneFrameAccess.java)
    pub mod scene_frame_access {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SceneFrameAccess.cinematic$getScene(0): not translated, it has no body.
        // SceneFrameAccess.cinematic$setScene(1): not translated, it has no body.
    }

    /// `SceneRenderer` (dev/dyslexic/cinematic/client/SceneRenderer.java)
    pub mod scene_renderer {
        use super::*;
        // levelCamera: not translated, it uses Minecraft's `Camera`.
        // collecting: not translated, it uses Minecraft's `SubmitNodeStorage`.
        // frustum: not translated, it uses Minecraft's `Frustum`.
        #[derive(Clone, Debug)]
        pub struct SceneRenderer {
        }
        impl SceneRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SceneRenderer.outlineNeighbor(4): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // SceneRenderer.beginLevel(3): not translated, it uses Minecraft's `Camera`.
        // SceneRenderer.collector(0): not translated, it uses Minecraft's `SubmitNodeCollector`.
        // SceneRenderer.beginExtraction(0): not translated, it calls BrewingBottles.beginFrame, which isn't translated.
        // SceneRenderer.extract(0): not translated, it uses Minecraft's `Minecraft`.
        // SceneRenderer.append(2): not translated, it uses Minecraft's `LevelRenderState`.
        // SceneRenderer.render(0): not translated, it uses Minecraft's `Minecraft`.
    }

    /// `SceneScreen` (dev/dyslexic/cinematic/client/SceneScreen.java)
    pub mod scene_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        pub const SLOT: i32 = 18;
        pub const PAD: i32 = 5;
        pub const ROW_GAP: i32 = 4;
        pub const INNER: i32 = 162;
        pub const FIELD_W: i32 = 170;
        // SceneScreen.SceneScreen(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.tick(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.isPauseScreen(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.extractBackground(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        pub fn presence() -> f32 {
            if scene::TRANSITION.get() && (scene::PHASE.get() == scene_phase::Phase::Intro) {
                return 1.0f32;
            } else {
                return if scene::PHASE.get() == scene_phase::Phase::Outro { ((1.0f64 - (anim::in_out_sine(scene::TIME.get() / 0.22f64))) as f32) } else { ((anim::out_cubic(scene::TIME.get() / 0.45f64)) as f32) };
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        // SceneScreen.working(0): not translated, it uses Scene.station, which isn't translated (it uses ClientStation, which isn't translated (it uses Minecraft's `BlockPos`)).
        // SceneScreen.editor(0): not translated, it uses LayoutEditor, which isn't translated (it's an interface).
        // SceneScreen.edit(3): not translated, `instanceof`.
        // SceneScreen.editing(0): not translated, `instanceof`.
        // SceneScreen.commitEdit(1): not translated, `instanceof`.
        // SceneScreen.openField(3): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.closeField(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.drawField(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.layout(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.overPanel(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.slotAt(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        pub fn inventory_index(menu_slot: i32) -> i32 {
            return if menu_slot >= 36 { menu_slot - 36 } else { menu_slot };
        }
        // SceneScreen.menuSlot(2): not translated, it uses Minecraft's `LocalPlayer`.
        // SceneScreen.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.drawPanel(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.drawSlot(8): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.drawHints(3): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.drawLookHint(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.drawMoveOver(3): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.drawTooltip(6): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        pub fn argb(a: f32, r: i32, g: i32, b: i32) -> i32 {
            let alpha: i32 = j_clamp(j_round_f(a * 255.0f32), 0, 255);
            return (((alpha << 24) | (r << 16)) | (g << 8)) | b;
        }
        // SceneScreen.guideShown(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.hoveredPanelItem(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.mouseClicked(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.charTyped(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.mouseDragged(3): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.mouseReleased(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.keyPressed(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.keyReleased(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        pub fn logical_button(button: i32) -> i32 {
            return if button == 1 { 0 } else { if button == 3 { 1 } else { 2 } };
        }
        pub fn physical_button(button: i32) -> i32 {
            return if button == 0 { 1 } else { if button == 1 { 3 } else { 2 } };
        }
        // SceneScreen.mouseClicked(3): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.mouseScrolled(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.charTyped(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.mouseDragged(5): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.mouseReleased(3): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.keyPressed(3): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.keyReleased(3): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SceneScreen.onClose(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `SceneText` (dev/dyslexic/cinematic/client/SceneText.java)
    pub mod scene_text {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SceneText {
        }
        impl SceneText {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SceneText.draw(11): not translated, it's overloaded with the same number of parameters.
        // SceneText.draw(11): not translated, it's overloaded with the same number of parameters.
        // SceneText.draw(11): not translated, it's overloaded with the same number of parameters.
    }

    /// `ServerListHint` (dev/dyslexic/cinematic/client/ServerListHint.java)
    pub mod server_list_hint {
        use super::*;
        pub const ACCENT: i32 = 9225454;
        pub const MUTED: i32 = 11053224;
        pub const GAP: i32 = 8;
        #[derive(Clone, Debug)]
        pub struct ServerListHint {
        }
        impl ServerListHint {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ServerListHint.onScreenInit(2): not translated, it uses Minecraft's `Minecraft`.
        // ServerListHint.seen(0): not translated, it uses BooleanValue, which isn't translated (it extends `Value`).
        // ServerListHint.addStateLine(2): not translated, it uses Minecraft's `Minecraft`.
    }

    /// `Settings` (dev/dyslexic/cinematic/client/Settings.java)
    pub mod settings {
        use super::*;
        // SPEC: not translated, it uses ClientConfig, which isn't translated (it uses Value, which isn't translated (it's generic)).
        // HEIGHT_LOGIC: not translated, it uses BooleanValue, which isn't translated (it extends `Value`).
        // SCENES: not translated, it uses BooleanValue, which isn't translated (it extends `Value`).
        // CONTROL_HINTS: not translated, it uses BooleanValue, which isn't translated (it extends `Value`).
        // CAMPFIRE_INVITE: not translated, it uses BooleanValue, which isn't translated (it extends `Value`).
        // APPROACH_SPEED: not translated, it uses IntValue, which isn't translated (it extends `Value`).
        // LEAVE_SPEED: not translated, it uses IntValue, which isn't translated (it extends `Value`).
        // LEAN_STRENGTH: not translated, it uses IntValue, which isn't translated (it extends `Value`).
        // SCENE_ZOOM: not translated, it uses IntValue, which isn't translated (it extends `Value`).
        // QUICK_MODE: not translated, it uses IntValue, which isn't translated (it extends `Value`).
        // QUICK_WINDOW: not translated, it uses IntValue, which isn't translated (it extends `Value`).
        // ARM_STYLE: not translated, it uses IntValue, which isn't translated (it extends `Value`).
        // LOOK_TOGGLES: not translated, it uses BooleanValue, which isn't translated (it extends `Value`).
        // LETTERBOX: not translated, it uses BooleanValue, which isn't translated (it extends `Value`).
        // LIVING_CAMERA: not translated, it uses BooleanValue, which isn't translated (it extends `Value`).
        // CAMERA_SHAKE: not translated, it uses BooleanValue, which isn't translated (it extends `Value`).
        // SCREEN_FLASH: not translated, it uses BooleanValue, which isn't translated (it extends `Value`).
        // REDUCE_MOTION: not translated, it uses BooleanValue, which isn't translated (it extends `Value`).
        // CLIENT_ONLY_MODE: not translated, it uses BooleanValue, which isn't translated (it extends `Value`).
        // SERVER_HINT_SEEN: not translated, it uses BooleanValue, which isn't translated (it extends `Value`).
        // DEVELOPER_MODE: not translated, it uses BooleanValue, which isn't translated (it extends `Value`).
        #[derive(Clone, Debug)]
        pub struct Settings {
        }
        impl Settings {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn key(kind: station_kind::StationKind) -> String {
            return kind.java_name().to_lowercase();
        }
        // Settings.get(1): not translated, it uses BooleanValue, which isn't translated (it extends `Value`).
        // Settings.percent(1): not translated, it uses IntValue, which isn't translated (it extends `Value`).
        // Settings.approachSpeed(0): not translated, it calls Settings.percent, which isn't translated.
        // Settings.leaveSpeed(0): not translated, it calls Settings.percent, which isn't translated.
        // Settings.leanStrength(0): not translated, it calls Settings.percent, which isn't translated.
        // Settings.sceneZoom(0): not translated, it calls Settings.percent, which isn't translated.
        // Settings.heightLogic(0): not translated, it calls Settings.get, which isn't translated.
        // Settings.scene(1): not translated, it calls Settings.get, which isn't translated.
        // Settings.controlHints(0): not translated, it calls Settings.get, which isn't translated.
        // Settings.clientOnlyMode(0): not translated, it calls Settings.get, which isn't translated.
        // Settings.developerMode(0): not translated, it calls Settings.get, which isn't translated.
        // Settings.campfireInvite(0): not translated, it calls Settings.get, which isn't translated.
        // Settings.quickMode(0): not translated, it uses Settings.SPEC, which isn't translated (it uses ClientConfig, which isn't translated (it uses Value, which isn't translated (it's generic))).
        // Settings.armStyle(0): not translated, it uses Settings.SPEC, which isn't translated (it uses ClientConfig, which isn't translated (it uses Value, which isn't translated (it's generic))).
        // Settings.quickWindow(0): not translated, it uses Settings.SPEC, which isn't translated (it uses ClientConfig, which isn't translated (it uses Value, which isn't translated (it's generic))).
        // Settings.lookToggles(0): not translated, it calls Settings.get, which isn't translated.
        // Settings.letterbox(0): not translated, it calls Settings.get, which isn't translated.
        // Settings.livingCamera(0): not translated, it calls Settings.get, which isn't translated.
        // Settings.cameraShake(0): not translated, it calls Settings.get, which isn't translated.
        // Settings.screenFlash(0): not translated, it calls Settings.get, which isn't translated.
    }

    /// `SettingsScreen` (dev/dyslexic/cinematic/client/SettingsScreen.java)
    pub mod settings_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        pub const ROW_WIDTH: i32 = 310;
        pub const CONTROL: i32 = 120;
        pub const NOTE: i32 = -5725546;
        pub const HEADER: i32 = -2047872;
        // GROUPS: not translated, it uses `Object`.
        // SettingsScreen.SettingsScreen(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SettingsScreen.bool(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SettingsScreen.number(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SettingsScreen.put(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SettingsScreen.profile(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SettingsScreen.apply(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SettingsScreen.preset(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SettingsScreen.choose(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SettingsScreen.toggle(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SettingsScreen.toggle(5): not translated, its body couldn't be read: an anonymous class.
        // SettingsScreen.choice(4): not translated, its body couldn't be read: an anonymous class.
        // SettingsScreen.slider(6): not translated, its body couldn't be read: an anonymous class.
        // SettingsScreen.presetOption(0): not translated, its body couldn't be read: an anonymous class.
        // SettingsScreen.profileOption(1): not translated, its body couldn't be read: an anonymous class.
        // SettingsScreen.options(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SettingsScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SettingsScreen.fill(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SettingsScreen.flush(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SettingsScreen.stations(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SettingsScreen.tabTitle(1): not translated, it uses Minecraft's `Component`.
        // SettingsScreen.showDeveloper(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SettingsScreen.tick(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SettingsScreen.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SettingsScreen.onClose(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // SettingsScreen.value(3): not translated, it uses Minecraft's `Component`.
        // SettingsScreen.percent(1): needs the game, so it's in game_side.rs.
        // SettingsScreen.icon(1): not translated, it uses Minecraft's `ItemLike`.
    }

    /// `SettingsScreen.Header` (dev/dyslexic/cinematic/client/SettingsScreen.java)
    pub mod settings_screen_header {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Component`. Its static parts are below.
        // SettingsScreen.Header.Header(3): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // SettingsScreen.Header.Header(2): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // SettingsScreen.Header.on(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // SettingsScreen.Header.title(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // SettingsScreen.Header.developer(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
    }

    /// `SettingsScreen.IntSlider` (dev/dyslexic/cinematic/client/SettingsScreen.java)
    pub mod settings_screen_int_slider {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractSliderButton`. Its static parts are below.
        // SettingsScreen.IntSlider.IntSlider(4): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
        // SettingsScreen.IntSlider.current(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
        // SettingsScreen.IntSlider.updateMessage(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
        // SettingsScreen.IntSlider.applyValue(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
    }

    /// `SettingsScreen.Option` (dev/dyslexic/cinematic/client/SettingsScreen.java)
    pub mod settings_screen_option {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Component`. Its static parts are below.
        // SettingsScreen.Option.Option(3): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // SettingsScreen.Option.control(0): not translated, it has no body.
        // SettingsScreen.Option.changed(0): not translated, it has no body.
        // SettingsScreen.Option.reset(0): not translated, it has no body.
        // SettingsScreen.Option.matches(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
    }

    /// `SettingsScreen.Preset` (dev/dyslexic/cinematic/client/SettingsScreen.java)
    pub mod settings_screen_preset {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // SettingsScreen.Preset.Preset(8): not translated, its class isn't translated (its constants carry values).
        // SettingsScreen.Preset.title(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `SettingsScreen.Row` (dev/dyslexic/cinematic/client/SettingsScreen.java)
    pub mod settings_screen_row {
        use super::*;
        // The class itself isn't translated: it extends `Entry` (java.util.Map). Its static parts are below.
        // SettingsScreen.Row.Row(5): not translated, its class isn't translated (it extends `Entry` (java.util.Map)).
        // SettingsScreen.Row.extractContent(5): not translated, its class isn't translated (it extends `Entry` (java.util.Map)).
        // SettingsScreen.Row.children(0): not translated, its class isn't translated (it extends `Entry` (java.util.Map)).
        // SettingsScreen.Row.narratables(0): not translated, its class isn't translated (it extends `Entry` (java.util.Map)).
    }

    /// `SettingsScreen.Row.Style` (dev/dyslexic/cinematic/client/SettingsScreen.java)
    pub mod settings_screen_row_style {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Style {
            Header,
            Note,
            Option,
            Pair,
            Bar,
        }
        impl Style {
            pub const VALUES: [Style; 5] = [Style::Header, Style::Note, Style::Option, Style::Pair, Style::Bar, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Style::Header => "HEADER",
                    Style::Note => "NOTE",
                    Style::Option => "OPTION",
                    Style::Pair => "PAIR",
                    Style::Bar => "BAR",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "HEADER" => Style::Header,
                    "NOTE" => Style::Note,
                    "OPTION" => Style::Option,
                    "PAIR" => Style::Pair,
                    "BAR" => Style::Bar,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `SettingsScreen.Rows` (dev/dyslexic/cinematic/client/SettingsScreen.java)
    pub mod settings_screen_rows {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ContainerObjectSelectionList`. Its static parts are below.
        // SettingsScreen.Rows.Rows(4): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // SettingsScreen.Rows.set(1): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // SettingsScreen.Rows.getRowWidth(0): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // SettingsScreen.Rows.scrollBarX(0): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
    }

    /// `SettingsScreen.StationButton` (dev/dyslexic/cinematic/client/SettingsScreen.java)
    pub mod settings_screen_station_button {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Plain`. Its static parts are below.
        // SettingsScreen.StationButton.StationButton(1): not translated, its class isn't translated (it extends Minecraft's `Plain`).
        // SettingsScreen.StationButton.getMessage(0): not translated, its class isn't translated (it extends Minecraft's `Plain`).
        // SettingsScreen.StationButton.extractContents(4): not translated, its class isn't translated (it extends Minecraft's `Plain`).
    }

    /// `SettingsScreen.StationOption` (dev/dyslexic/cinematic/client/SettingsScreen.java)
    pub mod settings_screen_station_option {
        use super::*;
        // The class itself isn't translated: it extends `Option`. Its static parts are below.
        // SettingsScreen.StationOption.StationOption(1): not translated, its class isn't translated (it extends `Option`).
        // SettingsScreen.StationOption.control(0): not translated, its body couldn't be read: an inner class creation.
        // SettingsScreen.StationOption.changed(0): not translated, its class isn't translated (it extends `Option`).
        // SettingsScreen.StationOption.reset(0): not translated, its class isn't translated (it extends `Option`).
    }

    /// `SettingsScreen.Tab` (dev/dyslexic/cinematic/client/SettingsScreen.java)
    pub mod settings_screen_tab {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Tab {
            General,
            Stations,
            Camera,
            Advanced,
        }
        impl Tab {
            pub const VALUES: [Tab; 4] = [Tab::General, Tab::Stations, Tab::Camera, Tab::Advanced, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Tab::General => "GENERAL",
                    Tab::Stations => "STATIONS",
                    Tab::Camera => "CAMERA",
                    Tab::Advanced => "ADVANCED",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "GENERAL" => Tab::General,
                    "STATIONS" => Tab::Stations,
                    "CAMERA" => Tab::Camera,
                    "ADVANCED" => Tab::Advanced,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `ShelfBooks` (dev/dyslexic/cinematic/client/ShelfBooks.java)
    pub mod shelf_books {
        use super::*;
        pub static SPINES: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| vec![vec![1, 2, 5, 7], vec![6, 1, 10, 7], vec![11, 1, 15, 7], vec![1, 9, 5, 15], vec![6, 10, 10, 15], vec![11, 9, 15, 15]]);
        pub const TEXTURE: &str = "block/chiseled_bookshelf_occupied";
        pub const DEPTH: f64 = 0.32f64;
        pub const PROUD: f64 = 0.004f64;
        pub const CLEAR: f64 = 0.37f64;
        // UP: not translated, it uses Minecraft's `Vec3`.
        // FLIGHTS: not translated, it uses Flight, which isn't translated (it uses Minecraft's `BlockPos`).
        // LOCAL: not translated, it uses Minecraft's `BlockPos`.
        // HIDDEN: not translated, it uses Minecraft's `BlockPos`.
        #[derive(Clone, Debug)]
        pub struct ShelfBooks {
        }
        impl ShelfBooks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ShelfBooks.basis(3): not translated, it uses Minecraft's `Vec3`.
        // ShelfBooks.out(1): not translated, it uses Minecraft's `Vec3`.
        // ShelfBooks.right(1): not translated, it uses Minecraft's `Vec3`.
        // ShelfBooks.face(5): not translated, it uses Minecraft's `Vec3`.
        // ShelfBooks.spine(3): not translated, it uses Minecraft's `Vec3`.
        // ShelfBooks.inSlot(4): not translated, it uses Minecraft's `BlockPos`.
        pub fn look(slot: i32) -> open_book_look::Look {
            let r = &(*shelf_books::SPINES)[slot as usize];
            let column: f32 = ((r[0] + 1) as f32);
            return open_book_look::Look::new("block/chiseled_bookshelf_occupied", (column + 0.3f32) / 16.0f32, (r[1] as f32) / 16.0f32, (column + 0.7f32) / 16.0f32, (r[3] as f32) / 16.0f32, (r[0] as f32) / 16.0f32, (r[1] as f32) / 16.0f32, (r[2] as f32) / 16.0f32, (r[3] as f32) / 16.0f32);
        }
        // ShelfBooks.reading(4): not translated, it uses Pose, which isn't translated (it uses Minecraft's `Vec3`).
        // ShelfBooks.held(2): not translated, it uses Minecraft's `Entity`.
        // ShelfBooks.atChest(5): not translated, it uses Minecraft's `Entity`.
        // ShelfBooks.draw(8): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // ShelfBooks.fly(1): not translated, it uses Minecraft's `BlockPos`.
        // ShelfBooks.flight(2): not translated, it uses Minecraft's `BlockPos`.
        // ShelfBooks.keepEmpty(2): not translated, it uses Minecraft's `BlockPos`.
        // ShelfBooks.cue(1): not translated, it uses MenuFxPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ShelfBooks.renderAll(5): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // ShelfBooks.applyMasks(1): not translated, it uses Minecraft's `BlockPos`.
        // ShelfBooks.reading(1): not translated, it uses ShelfBooks.FLIGHTS, which isn't translated (it uses Flight, which isn't translated (it uses Minecraft's `BlockPos`)).
        // ShelfBooks.forgetEntity(1): not translated, List.removeIf on a list kept in a changing static.
        // ShelfBooks.unloadChunk(1): not translated, it uses Minecraft's `ChunkPos`.
        // ShelfBooks.clear(0): not translated, it uses ShelfBooks.LOCAL, which isn't translated (it uses Minecraft's `BlockPos`).
        // ShelfBooks.emptyMask(1): not translated, it uses Minecraft's `BlockPos`.
    }

    /// `ShelfBooks.Flight` (dev/dyslexic/cinematic/client/ShelfBooks.java)
    pub mod shelf_books_flight {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPos`. Its static parts are below.
        // ShelfBooks.Flight.Flight(7): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ShelfBooks.Flight.length(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ShelfBooks.Flight.time(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ShelfBooks.Flight.pose(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `ShelfBooks.Key` (dev/dyslexic/cinematic/client/ShelfBooks.java)
    pub mod shelf_books_key {
        use super::*;
        // The class itself isn't translated: a function kept in a variable or field (only parameters can be). Its static parts are below.
        // ShelfBooks.Key.Key(2): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // ShelfBooks.Key.t(0): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // ShelfBooks.Key.pose(0): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
    }

    /// `ShelfBooks.Pose` (dev/dyslexic/cinematic/client/ShelfBooks.java)
    pub mod shelf_books_pose {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // ShelfBooks.Pose.Pose(4): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // ShelfBooks.Pose.to(2): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // ShelfBooks.Pose.moved(1): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // ShelfBooks.Pose.scaled(1): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // ShelfBooks.Pose.shut(1): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // ShelfBooks.Pose.axis(4): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // ShelfBooks.Pose.apply(1): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // ShelfBooks.Pose.at(3): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // ShelfBooks.Pose.origin(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // ShelfBooks.Pose.turn(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // ShelfBooks.Pose.size(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // ShelfBooks.Pose.fold(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `ShelfLayoutBooks` (dev/dyslexic/cinematic/client/ShelfLayoutBooks.java)
    pub mod shelf_layout_books {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockEntityRenderer`. Its static parts are below.
        // book: not translated, it uses OpenBook, which isn't translated (it uses Minecraft's `Vec3`).
        // VISIBLE_SHELVES: not translated, it uses Minecraft's `BlockPos`.
        pub static RESTORING_UNTIL: JCell<f64> = JCell::new(-1.0f64);
        // ShelfLayoutBooks.drawsRestingBooks(0): not translated, it calls ClientOnly.serverHasMod, which isn't translated.
        // ShelfLayoutBooks.layoutChanged(1): not translated, it uses Minecraft's `Minecraft`.
        // ShelfLayoutBooks.unloadChunk(1): not translated, it uses Minecraft's `ChunkPos`.
        // ShelfLayoutBooks.clear(0): not translated, it uses ShelfLayoutBooks.VISIBLE_SHELVES, which isn't translated (it uses Minecraft's `BlockPos`).
        // ShelfLayoutBooks.ShelfLayoutBooks(1): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // ShelfLayoutBooks.createRenderState(0): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // ShelfLayoutBooks.extractRenderState(5): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // ShelfLayoutBooks.submit(4): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // ShelfLayoutBooks.renderAll(5): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // ShelfLayoutBooks.render(5): not translated, it uses Minecraft's `ChiseledBookShelfBlockEntity`.
    }

    /// `ShelfStation` (dev/dyslexic/cinematic/client/ShelfStation.java)
    pub mod shelf_station {
        use super::*;
        // The class itself isn't translated: it extends `ClientStation`. Its static parts are below.
        pub const GRIP: f64 = 0.24f64;
        pub const OUT: f64 = 0.5f64;
        pub const TAKEN: f64 = 0.95f64;
        pub const BAGGED: f64 = 0.8f64;
        pub const PUT_AT: f64 = 0.34f64;
        pub const PUT_HOME: f64 = 0.62f64;
        pub const PUT_END: f64 = 0.82f64;
        pub const OPEN_OUT: f64 = 0.52f64;
        pub const OPEN_HELD: f64 = 1.05f64;
        pub const OPEN_END: f64 = 1.45f64;
        pub const CLOSE_SHUT: f64 = 0.45f64;
        pub const CLOSE_AT: f64 = 0.95f64;
        pub const CLOSE_HOME: f64 = 1.25f64;
        pub const CLOSE_END: f64 = 1.42f64;
        pub const CLOSE_GONE: f64 = 0.9f64;
        pub const PEEK: f64 = 0.05f64;
        pub const READ_DISTANCE: f64 = 0.52f64;
        pub const READ_DROP: f64 = 0.04f64;
        pub const READ_SCALE: f64 = 0.86f64;
        // ShelfStation.layoutEditor(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.layoutItem(1): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.layoutFrame(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.workSamples(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.renderScreen(3): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.layoutGhost(5): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.ShelfStation(2): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.kind(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.center(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.lightPos(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.touchSound(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.showsInventory(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.slotAt(3): not translated, it uses Minecraft's `BlockPos`.
        // ShelfStation.bookIn(1): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.occupied(1): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.readable(1): not translated, it uses Minecraft's `ItemStack`.
        // ShelfStation.shelfBook(1): needs the game, so it's in game_side.rs.
        // ShelfStation.slotPoint(1): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.approach(1): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.crouches(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.workOnTop(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.lean(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.leanFov(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.bodyPitch(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.rest(1): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.updateCursor(1): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.clearCursor(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.handTarget(4): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.heldItemPoint(2): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.click(2): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.deny(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.begin(3): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.take(2): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.put(3): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.open(1): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.close(1): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.turn(1): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.scroll(1): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.keyPressed(1): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.busy(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.send(4): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.tick(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.beforeFrame(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.gone(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.home(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.ownId(0): not translated, it uses Minecraft's `LocalPlayer`.
        // ShelfStation.onEnd(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.handPose(2): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.lowPose(1): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.lying(3): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.pose(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.handsOverride(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.handsStiffness(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.renderBooks(4): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.renderWorld(4): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.tooltip(1): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.hint(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.reading(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.spread(0): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.pageCenter(1): not translated, its class isn't translated (it extends `ClientStation`).
        // ShelfStation.moveName(0): not translated, its class isn't translated (it extends `ClientStation`).
    }

    /// `ShelfStation.Move` (dev/dyslexic/cinematic/client/ShelfStation.java)
    pub mod shelf_station_move {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Move {
            None,
            Take,
            Put,
            Open,
            Close,
        }
        impl Move {
            pub const VALUES: [Move; 5] = [Move::None, Move::Take, Move::Put, Move::Open, Move::Close, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Move::None => "NONE",
                    Move::Take => "TAKE",
                    Move::Put => "PUT",
                    Move::Open => "OPEN",
                    Move::Close => "CLOSE",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Move::None,
                    "TAKE" => Move::Take,
                    "PUT" => Move::Put,
                    "OPEN" => Move::Open,
                    "CLOSE" => Move::Close,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `ShelfStation.Target` (dev/dyslexic/cinematic/client/ShelfStation.java)
    pub mod shelf_station_target {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Target {
            None,
            Book,
            Place,
            Page,
        }
        impl Target {
            pub const VALUES: [Target; 4] = [Target::None, Target::Book, Target::Place, Target::Page, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Target::None => "NONE",
                    Target::Book => "BOOK",
                    Target::Place => "PLACE",
                    Target::Page => "PAGE",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Target::None,
                    "BOOK" => Target::Book,
                    "PLACE" => Target::Place,
                    "PAGE" => Target::Page,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `Showcase` (dev/dyslexic/cinematic/client/Showcase.java)
    pub mod showcase {
        use super::*;
        pub static ENABLED: LazyLock<bool> = LazyLock::new(|| std::env::var("cinematic.showcase").is_ok_and(|v| v.eq_ignore_ascii_case("true")));
        pub static FROM: LazyLock<String> = LazyLock::new(|| std::env::var("cinematic.showcase.from").unwrap_or_else(|_| "0".to_string()));
        pub static ONLY: LazyLock<bool> = LazyLock::new(|| std::env::var("cinematic.showcase.only").is_ok_and(|v| v.eq_ignore_ascii_case("true")));
        pub static EDITOR_BATCH: LazyLock<bool> = LazyLock::new(|| std::env::var("cinematic.showcase.editorBatch").is_ok_and(|v| v.eq_ignore_ascii_case("true")));
        pub static WORLD: LazyLock<String> = LazyLock::new(|| std::env::var("cinematic.showcase.world").unwrap_or_else(|_| "cinematic-showcase".to_string()));
        // TABLE: not translated, it uses Minecraft's `BlockPos`.
        // FURNACE: not translated, it uses Minecraft's `BlockPos`.
        // SMOKER: not translated, it uses Minecraft's `BlockPos`.
        // BLAST: not translated, it uses Minecraft's `BlockPos`.
        // ENCHANT: not translated, it uses Minecraft's `BlockPos`.
        // ANVIL: not translated, it uses Minecraft's `BlockPos`.
        // GRINDSTONE: not translated, it uses Minecraft's `BlockPos`.
        // SMITHING: not translated, it uses Minecraft's `BlockPos`.
        // STONECUTTER: not translated, it uses Minecraft's `BlockPos`.
        // LOOM: not translated, it uses Minecraft's `BlockPos`.
        // CARTOGRAPHY: not translated, it uses Minecraft's `BlockPos`.
        // BREWING: not translated, it uses Minecraft's `BlockPos`.
        // CRAFTER: not translated, it uses Minecraft's `BlockPos`.
        // CAMPFIRE: not translated, it uses Minecraft's `BlockPos`.
        // SOUL_CAMPFIRE: not translated, it uses Minecraft's `BlockPos`.
        // LECTERN: not translated, it uses Minecraft's `BlockPos`.
        // BEACON: not translated, it uses Minecraft's `BlockPos`.
        // SHELF: not translated, it uses Minecraft's `BlockPos`.
        // SHELF_SLOTS: not translated, it uses Minecraft's `Vec3`.
        // JUKEBOX: not translated, it uses Minecraft's `BlockPos`.
        pub const HEIGHTS_X: i32 = -10;
        pub static REQUESTED: JCell<bool> = JCell::new(false);
        pub static T: JCell<i32> = JCell::new(-1);
        pub static WAIT: JCell<i32> = JCell::new(0i32);
        // STEPS: not translated, it uses `Deque` (java.util).
        pub static REPORT: LazyLock<JCell<Vec<String>>> = LazyLock::new(|| JCell::new(Vec::new()));
        pub static INTRO_SHOTS: LazyLock<Vec<f64>> = LazyLock::new(|| vec![0.15f64, 0.35f64, 0.55f64, 0.75f64, 0.92f64]);
        pub static OUTRO_SHOTS: LazyLock<Vec<f64>> = LazyLock::new(|| vec![0.1f64, 0.25f64, 0.45f64, 0.65f64, 0.85f64]);
        pub static INTRO_SHOT: JCell<i32> = JCell::new(0i32);
        pub static OUTRO_SHOT: JCell<i32> = JCell::new(0i32);
        pub static MOVE_SHOTS: LazyLock<Vec<f64>> = LazyLock::new(|| vec![0.15f64, 0.35f64, 0.55f64, 0.75f64, 0.95f64]);
        pub static MOVE_SHOT: JCell<i32> = JCell::new(0i32);
        pub static ACT: LazyLock<JCell<String>> = LazyLock::new(|| JCell::new("0".to_string()));
        pub static WAITING_FOR_ACTIVE: JCell<bool> = JCell::new(false);
        pub static WAITING_TICKS: JCell<i32> = JCell::new(0i32);
        pub static DONE: JCell<bool> = JCell::new(false);
        pub static BEEF_PACE: LazyLock<Vec<i64>> = LazyLock::new(|| vec![0i64; 3]);
        // smith: not translated, it uses Minecraft's `RemotePlayer`.
        pub const SMITH_ID: i32 = 424242;
        #[derive(Clone, Debug)]
        pub struct Showcase {
        }
        impl Showcase {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Showcase.onScreen(1): not translated, it uses Minecraft's `Screen`.
        // Showcase.tick(0): not translated, it uses Minecraft's `Minecraft`.
        // Showcase.runScript(0): not translated, it uses Minecraft's `Minecraft`.
        // Showcase.setup(0): not translated, it uses Minecraft's `Minecraft`.
        // Showcase.use(3): not translated, it uses Minecraft's `BlockPos`.
        // Showcase.stopHere(1): not translated, it uses Minecraft's `Minecraft`.
        // Showcase.nextAct(6): not translated, it uses Minecraft's `Vec3`.
        // Showcase.inventoryEffectsScript(0): not translated, `instanceof`.
        // Showcase.drawerPoint(1): not translated, it uses Minecraft's `Minecraft`.
        // Showcase.recipeSyncChecks(0): not translated, it uses Minecraft's `ClientLevel`.
        // Showcase.encodeRecipes(2): not translated, it uses Minecraft's `ServerPlayer`.
        // Showcase.tableScript(0): not translated, it calls Showcase.recipeSyncChecks, which isn't translated.
        // Showcase.tableGridEditorScript(0): not translated, it uses `TableLayout`.
        // Showcase.editorSliderStep(2): not translated, it uses TableStation, which isn't translated (it extends `ClientStation`).
        // Showcase.editorControl(2): not translated, it uses TableStation, which isn't translated (it extends `ClientStation`).
        // Showcase.tableGridAlignmentScript(0): not translated, it uses Minecraft's `Vec3`.
        // Showcase.tableRecipeBookScript(0): not translated, it uses Minecraft's `ItemStack`.
        // Showcase.tableRecipeLayoutScript(0): not translated, it uses Minecraft's `ServerPlayer`.
        // Showcase.recipeControl(2): not translated, it uses TableStation, which isn't translated (it extends `ClientStation`).
        // Showcase.recipeSearch(1): not translated, it calls Showcase.recipeControl, which isn't translated.
        // Showcase.pointRecipe(1): not translated, it uses TableStation, which isn't translated (it extends `ClientStation`).
        // Showcase.tableVisibilityScript(0): not translated, it uses Minecraft's `Direction`.
        // Showcase.checkTableOpacity(2): not translated, it uses TableStation, which isn't translated (it extends `ClientStation`).
        // Showcase.tableGlintVisibilityScript(0): not translated, it uses Minecraft's `ServerPlayer`.
        // Showcase.tableSpecialVisibilityScript(0): not translated, it uses Minecraft's `ServerPlayer`.
        // Showcase.table(0): not translated, it uses TableStation, which isn't translated (it extends `ClientStation`).
        // Showcase.cellPoint(2): not translated, it uses Minecraft's `Vec3`.
        // Showcase.point(2): not translated, it uses Minecraft's `Vec3`.
        // Showcase.clickCell(3): not translated, it uses Minecraft's `Direction`.
        // Showcase.clickResult(0): not translated, it uses TableStation, which isn't translated (it extends `ClientStation`).
        // Showcase.furnace(0): not translated, it uses FurnaceStation, which isn't translated (it extends `ClientStation`).
        // Showcase.furnaceEditorScript(0): not translated, it uses FurnaceStation, which isn't translated (it extends `ClientStation`).
        // Showcase.surfaceEditorScript(0): not translated, a function kept in a variable or field (only parameters can be).
        // Showcase.spatialEditorScript(0): not translated, a function kept in a variable or field (only parameters can be).
        // Showcase.spatialControl(2): not translated, it uses SpatialEditor, which isn't translated (it implements `LayoutEditor`).
        // Showcase.spatialSlider(2): not translated, it uses SpatialEditor, which isn't translated (it implements `LayoutEditor`).
        // Showcase.surfaceControl(2): not translated, it uses SurfaceStation, which isn't translated (it's generic).
        // Showcase.surfaceSlider(2): not translated, it uses SurfaceStation, which isn't translated (it's generic).
        // Showcase.furnaceEditorControl(2): not translated, it calls Showcase.furnace, which isn't translated.
        // Showcase.furnaceEditorSliderNow(2): not translated, it calls Showcase.furnace, which isn't translated.
        // Showcase.furnaceEditorSlider(2): not translated, it calls Showcase.furnaceEditorSliderNow, which isn't translated.
        // Showcase.developerOptionsScript(0): not translated, it uses Minecraft's `Minecraft`.
        // Showcase.profileWidgets(2): not translated, it uses Minecraft's `GuiEventListener`.
        // Showcase.verifyProfileControls(0): not translated, it uses `Object`.
        // Showcase.clickSettingsSection(2): not translated, it uses Minecraft's `GuiEventListener`.
        // Showcase.furnaceScript(0): not translated, it calls Showcase.furnaceEditorScript, which isn't translated.
        // Showcase.smokerScript(0): not translated, it calls Showcase.furnaceEditorScript, which isn't translated.
        // Showcase.blastScript(0): not translated, it calls Showcase.furnaceEditorScript, which isn't translated.
        // Showcase.pointZone(2): not translated, it uses Minecraft's `BlockPos`.
        // Showcase.pointLedge(1): not translated, it uses Minecraft's `BlockPos`.
        // Showcase.clickZone(2): not translated, it uses FurnaceStation, which isn't translated (it extends `ClientStation`).
        // Showcase.enchantScript(0): not translated, it calls Showcase.spatialEditorScript, which isn't translated.
        // Showcase.enchant(0): not translated, it uses EnchantStation, which isn't translated (it extends `ClientStation`).
        // Showcase.checkLapisCount(2): not translated, it calls Showcase.enchant, which isn't translated.
        // Showcase.clickEnchant(2): not translated, it uses EnchantStation, which isn't translated (it extends `ClientStation`).
        // Showcase.smithyAct(2): not translated, it uses Minecraft's `BlockPos`.
        // Showcase.moveShots(2): not translated, String.format with `%03d`.
        // Showcase.anvil(0): not translated, it uses AnvilStation, which isn't translated (it extends `MenuStation`).
        // Showcase.clickAnvil(2): not translated, it uses AnvilStation, which isn't translated (it extends `MenuStation`).
        // Showcase.anvilScript(0): not translated, it calls Showcase.spatialEditorScript, which isn't translated.
        // Showcase.grindstone(0): not translated, it uses GrindstoneStation, which isn't translated (it extends `MenuStation`).
        // Showcase.clickGrindstone(2): not translated, it uses GrindstoneStation, which isn't translated (it extends `MenuStation`).
        // Showcase.grindstoneScript(0): not translated, it calls Showcase.spatialEditorScript, which isn't translated.
        // Showcase.smithing(0): not translated, it uses SmithingStation, which isn't translated (it extends `SurfaceStation`).
        // Showcase.clickSmithing(2): not translated, it uses SmithingStation, which isn't translated (it extends `SurfaceStation`).
        // Showcase.laySmithing(4): not translated, it uses Minecraft's `Item`.
        // Showcase.smithingScript(0): not translated, it calls Showcase.surfaceEditorScript, which isn't translated.
        // Showcase.stonecutter(0): not translated, it uses StonecutterStation, which isn't translated (it extends `MenuStation`).
        // Showcase.clickStonecutter(2): not translated, it uses StonecutterStation, which isn't translated (it extends `MenuStation`).
        // Showcase.shape(1): not translated, it uses Minecraft's `Item`.
        // Showcase.stonecutterScript(0): not translated, it calls Showcase.spatialEditorScript, which isn't translated.
        // Showcase.restock(0): not translated, it uses Minecraft's `ServerPlayer`.
        // Showcase.stash(0): not translated, it uses Minecraft's `LocalPlayer`.
        // Showcase.carried(0): not translated, it uses Minecraft's `ItemStack`.
        // Showcase.loom(0): not translated, it uses LoomStation, which isn't translated (it extends `MenuStation`).
        // Showcase.clickLoom(2): not translated, it uses LoomStation, which isn't translated (it extends `MenuStation`).
        // Showcase.pointPattern(1): not translated, it uses LoomStation, which isn't translated (it extends `MenuStation`).
        // Showcase.loomScript(0): not translated, it calls Showcase.spatialEditorScript, which isn't translated.
        // Showcase.cartography(0): not translated, it uses CartographyStation, which isn't translated (it extends `SurfaceStation`).
        // Showcase.clickCartography(2): not translated, it uses CartographyStation, which isn't translated (it extends `SurfaceStation`).
        // Showcase.cartographyScript(0): not translated, it calls Showcase.surfaceEditorScript, which isn't translated.
        // Showcase.brewing(0): not translated, it uses BrewingStation, which isn't translated (it extends `MenuStation`).
        // Showcase.clickBrewing(2): not translated, it uses BrewingStation, which isn't translated (it extends `MenuStation`).
        // Showcase.brewingScript(0): not translated, it calls Showcase.spatialEditorScript, which isn't translated.
        // Showcase.crafter(0): not translated, it uses CrafterStation, which isn't translated (it extends `MenuStation`).
        // Showcase.clickCrafter(2): not translated, it uses CrafterStation, which isn't translated (it extends `MenuStation`).
        // Showcase.crafterScript(0): not translated, it calls Showcase.spatialEditorScript, which isn't translated.
        // Showcase.cookedBeef(2): not translated, it uses Minecraft's `ServerPlayer`.
        // Showcase.campfire(0): not translated, it uses CampfireStation, which isn't translated (it extends `ClientStation`).
        // Showcase.clickCampfire(2): not translated, it uses CampfireStation, which isn't translated (it extends `ClientStation`).
        // Showcase.campfireAct(0): not translated, it uses Minecraft's `ServerPlayer`.
        // Showcase.campfireHintFadeScript(1): not translated, it calls Minecraft's `Minecraft`.getInstance.
        // Showcase.campfireScript(0): not translated, it calls Showcase.spatialEditorScript, which isn't translated.
        // Showcase.soulCampfireAct(0): not translated, it uses Minecraft's `ServerPlayer`.
        // Showcase.soulCampfireScript(0): not translated, it calls Showcase.spatialEditorScript, which isn't translated.
        // Showcase.notesBook(0): not translated, it uses Minecraft's `ItemStack`.
        // Showcase.storyBook(0): not translated, it uses Minecraft's `ItemStack`.
        // Showcase.lectern(0): not translated, it uses LecternStation, which isn't translated (it extends `MenuStation`).
        // Showcase.lecternAct(0): not translated, it calls Showcase.nextAct, which isn't translated.
        // Showcase.lecternPage(1): not translated, it uses Minecraft's `ServerPlayer`.
        // Showcase.lecternScript(0): not translated, it calls Showcase.spatialEditorScript, which isn't translated.
        // Showcase.shelf(0): not translated, it uses ShelfStation, which isn't translated (it extends `ClientStation`).
        // Showcase.clickShelf(2): not translated, it uses ShelfStation, which isn't translated (it extends `ClientStation`).
        // Showcase.shelfItem(2): not translated, it uses Minecraft's `ItemStack`.
        // Showcase.hold(2): not translated, it uses Minecraft's `ItemStack`.
        // Showcase.shelfAct(0): not translated, it uses Minecraft's `Minecraft`.
        // Showcase.shelfScript(0): not translated, it calls Showcase.spatialEditorScript, which isn't translated.
        // Showcase.beacon(0): not translated, it uses BeaconStation, which isn't translated (it extends `MenuStation`).
        // Showcase.clickBeacon(2): not translated, it uses BeaconStation, which isn't translated (it extends `MenuStation`).
        // Showcase.beaconAct(0): not translated, it calls Showcase.nextAct, which isn't translated.
        // Showcase.beaconScript(0): not translated, it calls Showcase.spatialEditorScript, which isn't translated.
        // Showcase.farSpot(0): not translated, it uses CampfireStation, which isn't translated (it extends `ClientStation`).
        // Showcase.heightSpot(1): not translated, it uses Minecraft's `BlockPos`.
        // Showcase.buildHeights(1): not translated, it uses Minecraft's `ServerLevel`.
        // Showcase.standLooking(2): not translated, it uses Minecraft's `Vec3`.
        // Showcase.closeWindow(0): not translated, it uses Minecraft's `Minecraft`.
        // Showcase.cameraOverEyes(0): not translated, it uses Minecraft's `Minecraft`.
        // Showcase.heightCase(8): not translated, it uses Minecraft's `BlockPos`.
        // Showcase.heightCase(9): not translated, it uses Minecraft's `BlockPos`.
        // Showcase.heightAct(0): not translated, it calls Showcase.developerOptionsScript, which isn't translated.
        // Showcase.optionsAct(0): not translated, it calls Showcase.stopHere, which isn't translated.
        // Showcase.transitionAct(0): not translated, it calls Showcase.stopHere, which isn't translated.
        // Showcase.layoutAct(0): not translated, it calls Showcase.stopHere, which isn't translated.
        // Showcase.lookAct(0): not translated, it calls Showcase.stopHere, which isn't translated.
        // Showcase.jukeboxAct(0): not translated, it calls Showcase.stopHere, which isn't translated.
        // Showcase.jukeboxOperations(2): not translated, it calls Showcase.closeWindow, which isn't translated.
        // Showcase.quickAct(0): not translated, it uses Minecraft's `Minecraft`.
        // Showcase.smithAt(5): not translated, it uses Minecraft's `Vec3`.
        // Showcase.smithMoves(6): not translated, it uses Minecraft's `BlockPos`.
        // Showcase.smithShelf(3): not translated, it uses Minecraft's `ItemStack`.
        // Showcase.watchAct(0): not translated, it calls Showcase.stopHere, which isn't translated.
        // Showcase.useAt(3): not translated, it uses Minecraft's `BlockPos`.
        // Showcase.paceCase(4): not translated, it uses Minecraft's `Vec3`.
        // Showcase.clickWidget(1): not translated, it uses Minecraft's `Minecraft`.
        // Showcase.noticeCase(7): not translated, it uses Minecraft's `BlockPos`.
        // Showcase.thirdPersonShots(0): not translated, it uses Minecraft's `ServerPlayer`.
        // Showcase.poseShot(3): not translated, it uses Minecraft's `BlockPos`.
        // Showcase.step(2): not translated, it uses Showcase.STEPS, which isn't translated (it uses `Deque` (java.util)).
        // Showcase.pointAt(1): not translated, it uses Minecraft's `Vec3`.
        // Showcase.pointGui(2): not translated, it uses Minecraft's `Minecraft`.
        // Showcase.pointAside(0): not translated, it uses Minecraft's `Minecraft`.
        // Showcase.hoverDrawer(0): not translated, it uses Minecraft's `Minecraft`.
        // Showcase.screenClick(1): not translated, it uses Minecraft's `Minecraft`.
        // Showcase.inventoryClick(2): not translated, it uses Minecraft's `Minecraft`.
        // Showcase.invClick(2): not translated, it uses Minecraft's `LocalPlayer`.
        // Showcase.invClickItem(1): not translated, it uses Minecraft's `Item`.
        pub fn check(name: &str, passed: bool, details: &str) {
            let line: String = format!("check {}{}: {}", if passed { "passed " } else { "FAILED " }, name, details);
            { let a_0 = line.to_string(); showcase::REPORT.with(|o| o.push(a_0)) };
            // log: "[Showcase] {}"
        }
        // Showcase.describe(1): needs the game, so it's in game_side.rs.
        // Showcase.report(1): not translated, it uses Minecraft's `Minecraft`.
        // Showcase.shot(1): not translated, it uses Minecraft's `Minecraft`.
        // Showcase.finish(1): not translated, it uses Minecraft's `Minecraft`.
        // Showcase.server(1): not translated, it uses Minecraft's `ServerPlayer`.
        // Showcase.cmd(1): not translated, it calls Showcase.server, which isn't translated.
        // Showcase.modelParts(1): not translated, it uses Minecraft's `BlockStateModelPart`.
        // Showcase.testKey(1): not translated, it uses Minecraft's `KeyEvent`.
    }

    /// `Showcase.Step` (dev/dyslexic/cinematic/client/Showcase.java)
    pub mod showcase_step {
        use super::*;
        // The class itself isn't translated: a function kept in a variable or field (only parameters can be). Its static parts are below.
        // Showcase.Step.Step(2): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // Showcase.Step.delay(0): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // Showcase.Step.action(0): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
    }

    /// `SmithingStation` (dev/dyslexic/cinematic/client/SmithingStation.java)
    pub mod smithing_station {
        use super::*;
        // The class itself isn't translated: it extends `SurfaceStation`. Its static parts are below.
        pub const HAND_Y: f64 = 0.062f64;
        pub const ITEM_REACH: f64 = 0.075f64;
        pub const LEAN_BACK: f64 = 0.7f64;
        pub const LEAN_UP: f64 = 0.62f64;
        pub static LAYOUT: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![-0.26f64, -0.12f64], vec![0.0f64, 0.06f64], vec![0.27f64, -0.12f64]]);
        pub static SIZES: LazyLock<Vec<f32>> = LazyLock::new(|| vec![0.22f32, 0.34f32, 0.2f32]);
        pub const SMITH_LENGTH: f64 = 2.6f64;
        pub const TEMPLATE_DOWN: f64 = 0.38f64;
        pub const MATERIAL_UP: f64 = 0.5f64;
        pub const MATERIAL_DOWN: f64 = 0.76f64;
        pub const PRESS: f64 = 0.95f64;
        pub const PEAK: f64 = 1.35f64;
        pub const COOLED: f64 = 2.15f64;
        pub const ON_GEAR: f64 = 0.025f64;
        pub const ON_TEMPLATE: f64 = 0.041f64;
        pub static SLOT_KEYS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["template".to_string(), "base".to_string(), "addition".to_string()]);
        // SmithingStation.SmithingStation(2): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.kind(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.center(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.touchSound(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.slotPoint(3): not translated, it uses Minecraft's `Vec3`.
        // SmithingStation.heldItemScale(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.approach(1): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.worksAtAnAngle(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.lean(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.rest(1): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.updateCursor(1): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.clearCursor(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.handTarget(4): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.heldItemPoint(2): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.act(2): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.smith(1): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.moltenColor(1): not translated, it uses Minecraft's `ItemStack`.
        pub fn fuse(t: f64) -> f64 {
            return anim::smoother(anim::window(t, 1.0f64, 1.32f64));
        }
        // SmithingStation.templateHeight(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.materialHeight(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.movingAxis(5): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.templatePos(1): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.materialPos(1): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.turn(6): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.heat(1): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.resultPos(2): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.handsOverride(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.cameraKick(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.flash(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.flashColor(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.frame(0): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.renderScene(4): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.drawLaid(9): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // SmithingStation.drawLaid(10): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // SmithingStation.renderWorld(4): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.drawContents(7): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // SmithingStation.onlookerFx(4): not translated, it uses Minecraft's `BlockPos`.
        // SmithingStation.tooltip(1): not translated, its class isn't translated (it extends `SurfaceStation`).
        // SmithingStation.hint(0): not translated, its class isn't translated (it extends `SurfaceStation`).
    }

    /// `SmithingStation.Target` (dev/dyslexic/cinematic/client/SmithingStation.java)
    pub mod smithing_station_target {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Target {
            None,
            Place,
            Item,
        }
        impl Target {
            pub const VALUES: [Target; 3] = [Target::None, Target::Place, Target::Item, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Target::None => "NONE",
                    Target::Place => "PLACE",
                    Target::Item => "ITEM",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Target::None,
                    "PLACE" => Target::Place,
                    "ITEM" => Target::Item,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `SpatialAppearance` (dev/dyslexic/cinematic/client/SpatialAppearance.java)
    pub mod spatial_appearance {
        use super::*;
        // The class itself isn't translated: it uses `Region`. Its static parts are below.
        pub const FLOATING_BOUND: f64 = 0.8f64;
        // VANILLA: not translated, it uses SpatialAppearance, which isn't translated (it uses `Region`).
        // SpatialAppearance.SpatialAppearance(2): not translated, its class isn't translated (it uses `Region`).
        // SpatialAppearance.region(1): not translated, its class isn't translated (it uses `Region`).
        // SpatialAppearance.with(2): not translated, its class isn't translated (it uses `Region`).
        // SpatialAppearance.blend(2): not translated, its class isn't translated (it uses `Region`).
        pub fn count(kind: station_kind::StationKind) -> i32 {
            return match kind {
                station_kind::StationKind::Crafter => 9,
                station_kind::StationKind::ChiseledBookshelf => 6,
                station_kind::StationKind::Brewing | station_kind::StationKind::Enchanting => 5,
                station_kind::StationKind::Loom | station_kind::StationKind::Anvil | station_kind::StationKind::Grindstone | station_kind::StationKind::Stonecutter => 3,
                station_kind::StationKind::Campfire => 4,
                station_kind::StationKind::Lectern | station_kind::StationKind::Jukebox => 1,
                station_kind::StationKind::Beacon => 8,
                _ => {
                panic!("{}", format!("Not a spatial layout: {}", kind.java_name()));
                },
            };
        }
        // SpatialAppearance.axis(3): not translated, its class isn't translated (it uses `Region`).
        // SpatialAppearance.rotation(1): not translated, its class isn't translated (it uses `Region`).
        // SpatialAppearance.scale(1): not translated, its class isn't translated (it uses `Region`).
        // SpatialAppearance.half(2): not translated, its class isn't translated (it uses `Region`).
        // SpatialAppearance.point(2): not translated, its class isn't translated (it uses `Region`).
        // SpatialAppearance.contains(3): not translated, its class isn't translated (it uses `Region`).
        // SpatialAppearance.group(0): not translated, its class isn't translated (it uses `Region`).
        // SpatialAppearance.slots(0): not translated, its class isn't translated (it uses `Region`).
    }

    /// `SpatialAppearance.Frame` (dev/dyslexic/cinematic/client/SpatialAppearance.java)
    pub mod spatial_appearance_frame {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPos`. Its static parts are below.
        // SpatialAppearance.Frame.Frame(8): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // SpatialAppearance.Frame.of(4): not translated, it uses Frame, which isn't translated (it uses Minecraft's `BlockPos`).
        // SpatialAppearance.Frame.base(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // SpatialAppearance.Frame.kind(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // SpatialAppearance.Frame.pos(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // SpatialAppearance.Frame.side(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // SpatialAppearance.Frame.grid(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // SpatialAppearance.Frame.origin(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // SpatialAppearance.Frame.across(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // SpatialAppearance.Frame.up(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // SpatialAppearance.Frame.normal(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `SpatialEditor` (dev/dyslexic/cinematic/client/SpatialEditor.java)
    pub mod spatial_editor {
        use super::*;
        // The class itself isn't translated: it implements `LayoutEditor`. Its static parts are below.
        pub const W: i32 = 180;
        pub const H: i32 = 362;
        pub const ROW_Y: i32 = 108;
        pub const ROW_H: i32 = 22;
        pub static FIELDS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["offset_x".to_string(), "offset_z".to_string(), "rotation".to_string(), "width".to_string(), "depth".to_string(), "height".to_string(), "item_size".to_string()]);
        pub static FIELDS_INDEX: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0, 1, 2, 3, 4, 5, 6]);
        // SpatialEditor.SpatialEditor(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.text(2): not translated, it uses Minecraft's `Component`.
        // SpatialEditor.editing(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.visible(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.presence(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.fields(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        pub fn percent(field: i32) -> bool {
            return ((field == 3) || (field == 4)) || (field == 6);
        }
        pub fn min(field: i32) -> f64 {
            return if spatial_editor::percent(field) { 50.0f64 } else { if field == 2 { -180.0f64 } else { if field == 5 { -2.0f64 } else { -6.0f64 } } };
        }
        pub fn max(field: i32) -> f64 {
            return if spatial_editor::percent(field) { 150.0f64 } else { -spatial_editor::min(field) };
        }
        pub fn step(field: i32) -> f64 {
            return if (!spatial_editor::percent(field)) && (field != 2) { 0.125f64 } else { 1.0f64 };
        }
        // SpatialEditor.fieldKey(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.regionName(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.layout(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.lx(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.ly(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.inPanel(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.toggle(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.over(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.frame(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.open(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.cancel(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.save(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.change(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.slide(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.click(3): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.drag(3): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.release(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.scroll(3): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.keyPressed(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.tooltip(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.render(3): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.selector(5): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.button(9): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.glass(4): needs the game, so it's in game_side.rs.
        pub fn color(a: f64, r: i32, g: i32, b: i32) -> i32 {
            return ((((j_clamp((j_round_d(a * 255.0f64) as i32), 0, 255)) << 24) | (r << 16)) | (g << 8)) | b;
        }
        // SpatialEditor.renderWorld(4): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.patch(11): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // SpatialEditor.togglePoint(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.controlPoint(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SpatialEditor.sliderPoint(2): not translated, its class isn't translated (it implements `LayoutEditor`).
    }

    /// `SpatialLayouts` (dev/dyslexic/cinematic/client/SpatialLayouts.java)
    pub mod spatial_layouts {
        use super::*;
        pub static JSON: LazyLock<JGson> = LazyLock::new(|| JGson { pretty: true, ..JGson::default() });
        pub const FILE: &str = "cinematic-spatial-layouts.json";
        // profiles: not translated, it uses SpatialAppearance, which isn't translated (it uses `Region`).
        pub const TYPES: i32 = 12;
        pub static SELECTED: LazyLock<JCell<Vec<i32>>> = LazyLock::new(|| JCell::new(vec![0i32; 12]));
        // current: not translated, it uses SpatialAppearance, which isn't translated (it uses `Region`).
        // preview: not translated, it uses SpatialAppearance, which isn't translated (it uses `Region`).
        pub static LOADED: JCell<bool> = JCell::new(false);
        pub static CUSTOM_SHELF: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct SpatialLayouts {
        }
        impl SpatialLayouts {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn custom_shelf() -> bool {
            return spatial_layouts::CUSTOM_SHELF.get();
        }
        pub fn index(kind: station_kind::StationKind) -> i32 {
            return match kind {
                station_kind::StationKind::Crafter => 0,
                station_kind::StationKind::ChiseledBookshelf => 1,
                station_kind::StationKind::Brewing => 2,
                station_kind::StationKind::Loom => 3,
                station_kind::StationKind::Enchanting => 4,
                station_kind::StationKind::Anvil => 5,
                station_kind::StationKind::Grindstone => 6,
                station_kind::StationKind::Stonecutter => 7,
                station_kind::StationKind::Campfire => 8,
                station_kind::StationKind::Lectern => 9,
                station_kind::StationKind::Beacon => 10,
                station_kind::StationKind::Jukebox => 11,
                _ => {
                panic!("{}", format!("Not a spatial layout: {}", kind.java_name()));
                },
            };
        }
        // SpatialLayouts.defaults(0): not translated, it uses SpatialAppearance, which isn't translated (it uses `Region`).
        // SpatialLayouts.vanillaRow(0): not translated, it uses SpatialAppearance, which isn't translated (it uses `Region`).
        pub fn path() -> std::path::PathBuf {
            return j_config_dir().join("cinematic-spatial-layouts.json");
        }
        // SpatialLayouts.load(0): not translated, its body couldn't be read: a labelled statement.
        // SpatialLayouts.current(1): not translated, it uses SpatialAppearance, which isn't translated (it uses `Region`).
        // SpatialLayouts.profiles(1): not translated, it uses SpatialAppearance, which isn't translated (it uses `Region`).
        // SpatialLayouts.selected(1): not translated, it calls SpatialLayouts.load, which isn't translated.
        // SpatialLayouts.preview(2): not translated, it uses SpatialAppearance, which isn't translated (it uses `Region`).
        // SpatialLayouts.cancelPreview(1): not translated, it uses SpatialLayouts.preview, which isn't translated (it uses SpatialAppearance, which isn't translated (it uses `Region`)).
        // SpatialLayouts.frame(1): not translated, it calls SpatialLayouts.load, which isn't translated.
        // SpatialLayouts.save(3): not translated, it uses SpatialAppearance, which isn't translated (it uses `Region`).
    }

    /// `SpatialLayouts.Saved` (dev/dyslexic/cinematic/client/SpatialLayouts.java)
    pub mod spatial_layouts_saved {
        use super::*;
        // The class itself isn't translated: it uses SpatialAppearance, which isn't translated (it uses `Region`). Its static parts are below.
        // SpatialLayouts.Saved.Saved(3): not translated, its class isn't translated (it uses SpatialAppearance, which isn't translated (it uses `Region`)).
        // SpatialLayouts.Saved.version(0): not translated, its class isn't translated (it uses SpatialAppearance, which isn't translated (it uses `Region`)).
        // SpatialLayouts.Saved.selected(0): not translated, its class isn't translated (it uses SpatialAppearance, which isn't translated (it uses `Region`)).
        // SpatialLayouts.Saved.profiles(0): not translated, its class isn't translated (it uses SpatialAppearance, which isn't translated (it uses `Region`)).
    }

    /// `SpatialTarget` (dev/dyslexic/cinematic/client/SpatialTarget.java)
    pub mod spatial_target {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SpatialTarget.layoutStation(0): not translated, its class isn't translated (it's an interface).
        // SpatialTarget.layoutEditor(0): not translated, it has no body.
        // SpatialTarget.layoutFrame(0): not translated, its class isn't translated (it's an interface).
        // SpatialTarget.layoutItem(1): not translated, its class isn't translated (it's an interface).
        // SpatialTarget.geometry(0): not translated, its class isn't translated (it's an interface).
        // SpatialTarget.layoutPoint(1): not translated, its class isn't translated (it's an interface).
        // SpatialTarget.layoutNormal(1): not translated, its class isn't translated (it's an interface).
        // SpatialTarget.layoutAxis(2): not translated, its class isn't translated (it's an interface).
        // SpatialTarget.layoutSamples(1): not translated, its class isn't translated (it's an interface).
        // SpatialTarget.layoutPick(0): not translated, its class isn't translated (it's an interface).
        // SpatialTarget.volumePick(0): not translated, its class isn't translated (it's an interface).
        // SpatialTarget.layoutHeldScale(0): not translated, its class isn't translated (it's an interface).
        // SpatialTarget.layoutGhost(5): not translated, its class isn't translated (it's an interface).
    }

    /// `StationDisplayLayouts` (dev/dyslexic/cinematic/client/StationDisplayLayouts.java)
    pub mod station_display_layouts {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct StationDisplayLayouts {
        }
        impl StationDisplayLayouts {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // StationDisplayLayouts.campfire(3): not translated, it uses Minecraft's `CampfireRenderState`.
        // StationDisplayLayouts.customLectern(0): not translated, it calls ClientOnly.serverHasMod, which isn't translated.
        // StationDisplayLayouts.lectern(2): not translated, it uses Minecraft's `LecternRenderState`.
    }

    /// `StationModels` (dev/dyslexic/cinematic/client/StationModels.java)
    pub mod station_models {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `WrapperBlockStateModel`. Its static parts are below.
        // MODELS: not translated, it uses Minecraft's `BlockState`.
        // StationModels.StationModels(2): not translated, its class isn't translated (it extends Fabric's `WrapperBlockStateModel`).
        // StationModels.wrap(2): not translated, it uses Minecraft's `BlockStateModel`.
        // StationModels.displayed(2): not translated, its class isn't translated (it extends Fabric's `WrapperBlockStateModel`).
        // StationModels.collectParts(2): not translated, its class isn't translated (it extends Fabric's `WrapperBlockStateModel`).
        // StationModels.emitQuads(6): not translated, its class isn't translated (it extends Fabric's `WrapperBlockStateModel`).
        // StationModels.createGeometryKey(4): not translated, its class isn't translated (it extends Fabric's `WrapperBlockStateModel`).
    }

    /// `StonecutterStation` (dev/dyslexic/cinematic/client/StonecutterStation.java)
    pub mod stonecutter_station {
        use super::*;
        // The class itself isn't translated: it extends `MenuStation`. Its static parts are below.
        pub const HAND_Y: f64 = 0.062f64;
        pub const ITEM_REACH: f64 = 0.07f64;
        pub const TABLE: f64 = 0.5625f64;
        pub const BLADE: f64 = 0.4375f64;
        pub const DIAL_WIDE: f64 = 0.72f64;
        pub const DIAL_HIGH: f64 = 0.5f64;
        pub const DIAL_SLOTS: i32 = 9;
        pub const SHAPE_SIZE: f32 = 0.15f32;
        pub const PIECE_SIZE: f32 = 0.2f32;
        pub const CUT_LENGTH: f64 = 1.05f64;
        pub const CUT_ALL_LENGTH: f64 = 1.5f64;
        pub const BITE: f64 = 0.26f64;
        pub static DIAL_FITS: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![0.72f64, 0.0f64, 0.0f64], vec![0.5f64, 0.08f64, 0.04f64], vec![0.42f64, 0.14f64, 0.26f64], vec![0.34f64, 0.2f64, 0.42f64]]);
        pub const SHAPE_ROOM: f64 = 0.07f64;
        pub const FIT_EVERY: f64 = 0.5f64;
        pub static FITS_DIAL: JCell<bool> = JCell::new(true);
        // StonecutterStation.layoutEditor(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.StonecutterStation(2): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.topOf(1): not translated, it uses Minecraft's `Vec3`.
        // StonecutterStation.rightOf(1): not translated, it uses Minecraft's `Vec3`.
        // StonecutterStation.kind(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.center(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.touchSound(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.lightPos(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.stackPoint(2): not translated, it uses Minecraft's `Vec3`.
        // StonecutterStation.restPoint(1): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.approach(1): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.lean(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.bodyPitch(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.rest(1): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.shapes(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.shapeResult(1): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.visible(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.step(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.angle(1): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.onDial(1): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.shapePoint(1): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.fit(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.dialFit(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.free(1): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.dialPoint(3): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.scroll(1): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.updateCursor(1): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.clearCursor(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.handTarget(4): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.heldItemPoint(2): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.act(2): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.cut(2): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.tick(0): not translated, its class isn't translated (it extends `MenuStation`).
        pub fn out_at(all: bool) -> f64 {
            return if all { 1.15f64 } else { 0.72f64 };
        }
        // StonecutterStation.along(1): not translated, its class isn't translated (it extends `MenuStation`).
        pub fn along_2(t: f64, all: bool) -> f64 {
            return (-0.46f64) + (0.76f64 * (anim::in_out_sine(anim::window(t, 0.22f64, stonecutter_station::out_at(all)))));
        }
        // StonecutterStation.piecePath(4): not translated, it uses Minecraft's `Vec3`.
        // StonecutterStation.piecePos(2): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.catchPoint(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.handsOverride(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.cameraKick(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.cutting(1): not translated, its class isn't translated (it extends `MenuStation`).
        pub fn cutting_2(t: f64, all: bool) -> bool {
            return ((t >= 0.26f64) && ((stonecutter_station::along_2(t, all).abs()) < 0.4375f64)) && (t < stonecutter_station::out_at(all));
        }
        // StonecutterStation.frame(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.cutFx(6): not translated, it uses Minecraft's `Vec3`.
        // StonecutterStation.dust(4): not translated, it uses Minecraft's `Vec3`.
        // StonecutterStation.renderScene(4): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.drawPiece(5): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.drawPieceAt(12): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // StonecutterStation.drawStack(7): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // StonecutterStation.renderWorld(4): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.shapeEnd(1): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.drawCount(6): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // StonecutterStation.drawContents(7): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // StonecutterStation.drawOnlooker(7): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // StonecutterStation.onlookerFx(4): not translated, it uses Minecraft's `BlockPos`.
        pub fn arm_turns(t: f64) -> Vec<f32> {
            let push: f64 = anim::window(stonecutter_station::along_2(t, false), -0.46f64, 0.3f64);
            let reach: f64 = (anim::smoother(anim::window(t, 0.26f64, stonecutter_station::out_at(false)))) * (1.0f64 - (anim::smoother(anim::window(t, stonecutter_station::out_at(false), 1.05f64))));
            return vec![(((-0.25f64) + (0.6f64 * push)) as f32), (((-0.12f64) + (0.36f64 * reach)) as f32)];
        }
        // StonecutterStation.tooltip(1): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.hint(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.beforeFrame(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.onEnd(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.busy(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.showsInventory(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.renderScreen(3): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.workSamples(0): not translated, its class isn't translated (it extends `MenuStation`).
        // StonecutterStation.layoutHeldScale(0): not translated, its class isn't translated (it extends `MenuStation`).
    }

    /// `StonecutterStation.Target` (dev/dyslexic/cinematic/client/StonecutterStation.java)
    pub mod stonecutter_station_target {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Target {
            None,
            Place,
            Stack,
            Shape,
        }
        impl Target {
            pub const VALUES: [Target; 4] = [Target::None, Target::Place, Target::Stack, Target::Shape, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Target::None => "NONE",
                    Target::Place => "PLACE",
                    Target::Stack => "STACK",
                    Target::Shape => "SHAPE",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Target::None,
                    "PLACE" => Target::Place,
                    "STACK" => Target::Stack,
                    "SHAPE" => Target::Shape,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `SurfaceEditor` (dev/dyslexic/cinematic/client/SurfaceEditor.java)
    pub mod surface_editor {
        use super::*;
        // The class itself isn't translated: it implements `LayoutEditor`. Its static parts are below.
        pub const W: i32 = 180;
        pub const H: i32 = 362;
        pub const ROW_Y: i32 = 108;
        pub const ROW_H: i32 = 22;
        pub static FIELDS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["offset_x".to_string(), "offset_z".to_string(), "rotation".to_string(), "width".to_string(), "depth".to_string(), "height".to_string(), "item_size".to_string()]);
        pub static FIELDS_INDEX: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0, 1, 2, 3, 4, 5, 6]);
        // SurfaceEditor.SurfaceEditor(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.text(2): not translated, it uses Minecraft's `Component`.
        // SurfaceEditor.editing(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.visible(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.fields(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        pub fn percent(field: i32) -> bool {
            return ((field == 3) || (field == 4)) || (field == 6);
        }
        pub fn min(field: i32) -> f64 {
            return if surface_editor::percent(field) { 50.0f64 } else { if field == 2 { -180.0f64 } else { if field == 5 { -2.0f64 } else { -6.0f64 } } };
        }
        pub fn max(field: i32) -> f64 {
            return if surface_editor::percent(field) { 150.0f64 } else { -surface_editor::min(field) };
        }
        pub fn step(field: i32) -> f64 {
            return if (!surface_editor::percent(field)) && (field != 2) { 0.125f64 } else { 1.0f64 };
        }
        // SurfaceEditor.regionName(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.layout(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.lx(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.ly(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.inPanel(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.toggle(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.over(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.frame(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.open(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.cancel(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.save(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.change(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.slide(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.click(3): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.drag(3): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.release(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.scroll(3): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.keyPressed(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.tooltip(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.render(3): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.selector(5): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.button(9): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.glass(4): needs the game, so it's in game_side.rs.
        pub fn color(a: f64, r: i32, g: i32, b: i32) -> i32 {
            return ((((j_clamp((j_round_d(a * 255.0f64) as i32), 0, 255)) << 24) | (r << 16)) | (g << 8)) | b;
        }
        // SurfaceEditor.renderWorld(4): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.patch(11): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // SurfaceEditor.togglePoint(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.controlPoint(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // SurfaceEditor.sliderPoint(2): not translated, its class isn't translated (it implements `LayoutEditor`).
    }

    /// `SurfaceLayouts` (dev/dyslexic/cinematic/client/SurfaceLayouts.java)
    pub mod surface_layouts {
        use super::*;
        pub static JSON: LazyLock<JGson> = LazyLock::new(|| JGson { pretty: true, ..JGson::default() });
        pub const FILE: &str = "cinematic-surface-layouts.json";
        // profiles: not translated, it uses `SurfaceAppearance`.
        pub static SELECTED: LazyLock<JCell<Vec<i32>>> = LazyLock::new(|| JCell::new(vec![0i32; 2]));
        // current: not translated, it uses `SurfaceAppearance`.
        // preview: not translated, it uses `SurfaceAppearance`.
        pub static LOADED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct SurfaceLayouts {
        }
        impl SurfaceLayouts {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn index(kind: station_kind::StationKind) -> i32 {
            return match kind {
                station_kind::StationKind::Smithing => 0,
                station_kind::StationKind::Cartography => 1,
                _ => {
                panic!("{}", format!("Not a tabletop: {}", kind.java_name()));
                },
            };
        }
        // SurfaceLayouts.defaults(0): not translated, it uses `SurfaceAppearance`.
        pub fn path() -> std::path::PathBuf {
            return j_config_dir().join("cinematic-surface-layouts.json");
        }
        // SurfaceLayouts.load(0): not translated, its body couldn't be read: a labelled statement.
        // SurfaceLayouts.current(1): not translated, it uses `SurfaceAppearance`.
        // SurfaceLayouts.profiles(1): not translated, it uses `SurfaceAppearance`.
        // SurfaceLayouts.selected(1): not translated, it calls SurfaceLayouts.load, which isn't translated.
        // SurfaceLayouts.preview(2): not translated, it uses `SurfaceAppearance`.
        // SurfaceLayouts.cancelPreview(1): not translated, it uses SurfaceLayouts.preview, which isn't translated (it uses `SurfaceAppearance`).
        // SurfaceLayouts.frame(1): not translated, it calls SurfaceLayouts.load, which isn't translated.
        // SurfaceLayouts.save(3): not translated, it uses `SurfaceAppearance`.
    }

    /// `SurfaceLayouts.Saved` (dev/dyslexic/cinematic/client/SurfaceLayouts.java)
    pub mod surface_layouts_saved {
        use super::*;
        // The class itself isn't translated: it uses `SurfaceAppearance`. Its static parts are below.
        // SurfaceLayouts.Saved.Saved(3): not translated, its class isn't translated (it uses `SurfaceAppearance`).
        // SurfaceLayouts.Saved.version(0): not translated, its class isn't translated (it uses `SurfaceAppearance`).
        // SurfaceLayouts.Saved.selected(0): not translated, its class isn't translated (it uses `SurfaceAppearance`).
        // SurfaceLayouts.Saved.profiles(0): not translated, its class isn't translated (it uses `SurfaceAppearance`).
    }

    /// `SurfaceStation` (dev/dyslexic/cinematic/client/SurfaceStation.java)
    pub mod surface_station {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // SurfaceStation.SurfaceStation(4): not translated, its class isn't translated (it's generic).
        // SurfaceStation.geometry(0): not translated, its class isn't translated (it's generic).
        // SurfaceStation.restPoint(1): not translated, its class isn't translated (it's generic).
        // SurfaceStation.surfaceScale(1): not translated, its class isn't translated (it's generic).
        // SurfaceStation.surfaceRotation(1): not translated, its class isn't translated (it's generic).
        // SurfaceStation.surfaceRight(1): not translated, its class isn't translated (it's generic).
        // SurfaceStation.surfaceOut(1): not translated, its class isn't translated (it's generic).
        // SurfaceStation.busy(0): not translated, its class isn't translated (it's generic).
        // SurfaceStation.showsInventory(0): not translated, its class isn't translated (it's generic).
        // SurfaceStation.beforeFrame(0): not translated, its class isn't translated (it's generic).
        // SurfaceStation.onEnd(0): not translated, its class isn't translated (it's generic).
        // SurfaceStation.renderScreen(3): not translated, its class isn't translated (it's generic).
        // SurfaceStation.workSamples(0): not translated, its class isn't translated (it's generic).
        // SurfaceStation.surfaceSlot(1): not translated, its class isn't translated (it's generic).
        // SurfaceStation.heldItemScale(1): not translated, its class isn't translated (it's generic).
        // SurfaceStation.heldItemScale(0): not translated, it has no body.
    }

    /// `TableGridEditor` (dev/dyslexic/cinematic/client/TableGridEditor.java)
    pub mod table_grid_editor {
        use super::*;
        // The class itself isn't translated: it implements `LayoutEditor`. Its static parts are below.
        pub const W: i32 = 164;
        pub const H: i32 = 290;
        pub const ROW_Y: i32 = 64;
        pub const ROW_H: i32 = 25;
        pub static FIELDS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["offset_x".to_string(), "offset_z".to_string(), "rotation".to_string(), "width".to_string(), "depth".to_string(), "height".to_string(), "item_size".to_string()]);
        pub static MIN: LazyLock<Vec<f64>> = LazyLock::new(|| vec![-6.0f64, -6.0f64, -180.0f64, 50.0f64, 50.0f64, -2.0f64, 50.0f64]);
        pub static MAX: LazyLock<Vec<f64>> = LazyLock::new(|| vec![6.0f64, 6.0f64, 180.0f64, 170.0f64, 170.0f64, 2.0f64, 150.0f64]);
        // TableGridEditor.TableGridEditor(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.text(2): not translated, it uses Minecraft's `Component`.
        // TableGridEditor.editing(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.visible(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.layout(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.frame(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.lx(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.ly(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.inPanel(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.toggle(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.over(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.open(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.cancel(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.save(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.change(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.slide(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.click(3): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.drag(3): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.release(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.scroll(3): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.keyPressed(1): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.tooltip(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.render(3): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.button(9): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.glass(4): needs the game, so it's in game_side.rs.
        pub fn color(a: f64, r: i32, g: i32, b: i32) -> i32 {
            return ((((j_clamp((j_round_d(a * 255.0f64) as i32), 0, 255)) << 24) | (r << 16)) | (g << 8)) | b;
        }
        // TableGridEditor.renderWorld(3): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.quad(7): not translated, it uses Minecraft's `VertexConsumer`.
        // TableGridEditor.togglePoint(0): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.controlPoint(2): not translated, its class isn't translated (it implements `LayoutEditor`).
        // TableGridEditor.sliderPoint(2): not translated, its class isn't translated (it implements `LayoutEditor`).
    }

    /// `TableLayouts` (dev/dyslexic/cinematic/client/TableLayouts.java)
    pub mod table_layouts {
        use super::*;
        pub static JSON: LazyLock<JGson> = LazyLock::new(|| JGson { pretty: true, ..JGson::default() });
        pub const FILE: &str = "cinematic-table-layouts.json";
        // profiles: not translated, it uses `TableLayout`.
        pub static SELECTED: JCell<i32> = JCell::new(0i32);
        pub static LOADED: JCell<bool> = JCell::new(false);
        // preview: not translated, it uses `TableLayout`.
        // current: not translated, it uses `TableLayout`.
        #[derive(Clone, Debug)]
        pub struct TableLayouts {
        }
        impl TableLayouts {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn path() -> std::path::PathBuf {
            return j_config_dir().join("cinematic-table-layouts.json");
        }
        // TableLayouts.load(0): not translated, Gson.fromJson into something other than a class (a TypeToken).
        // TableLayouts.current(0): not translated, it uses `TableLayout`.
        // TableLayouts.selected(0): not translated, it calls TableLayouts.load, which isn't translated.
        // TableLayouts.profiles(0): not translated, it uses `TableLayout`.
        // TableLayouts.preview(1): not translated, it uses `TableLayout`.
        // TableLayouts.cancelPreview(0): not translated, it uses TableLayouts.preview, which isn't translated (it uses `TableLayout`).
        // TableLayouts.frame(1): not translated, it calls TableLayouts.load, which isn't translated.
        // TableLayouts.close(2): not translated, it uses `TableLayout`.
        // TableLayouts.save(2): not translated, it uses `TableLayout`.
    }

    /// `TableLayouts.Saved` (dev/dyslexic/cinematic/client/TableLayouts.java)
    pub mod table_layouts_saved {
        use super::*;
        // The class itself isn't translated: it uses `TableLayout`. Its static parts are below.
        // TableLayouts.Saved.Saved(3): not translated, its class isn't translated (it uses `TableLayout`).
        // TableLayouts.Saved.version(0): not translated, its class isn't translated (it uses `TableLayout`).
        // TableLayouts.Saved.selected(0): not translated, its class isn't translated (it uses `TableLayout`).
        // TableLayouts.Saved.profiles(0): not translated, its class isn't translated (it uses `TableLayout`).
    }

    /// `TableRecipeBook` (dev/dyslexic/cinematic/client/TableRecipeBook.java)
    pub mod table_recipe_book {
        use super::*;
        // The class itself isn't translated: it uses TableStation, which isn't translated (it extends `ClientStation`). Its static parts are below.
        pub const WIDTH: i32 = 128;
        pub const HEIGHT: i32 = 226;
        pub const CELL: i32 = 27;
        pub const COLS: i32 = 4;
        pub const ROWS: i32 = 4;
        pub const LAY_Y: i32 = 205;
        pub const LAY_H: i32 = 15;
        pub const GRID_X: i32 = 10;
        pub const GRID_Y: i32 = 61;
        pub const PAGE_SIZE: i32 = 16;
        // BOOK: not translated, it uses Minecraft's `ItemStack`.
        // TableRecipeBook.TableRecipeBook(1): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.text(2): not translated, it uses Minecraft's `Component`.
        // TableRecipeBook.tick(0): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.filter(0): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.pages(0): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.localX(1): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.localY(1): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.inPanel(2): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.inToggle(2): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.over(2): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.show(1): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.fold(0): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.entryAt(2): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.render(3): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.layable(0): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.inLayButton(2): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.maxLayout(0): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.layOut(1): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.drawLayButton(4): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.glass(4): needs the game, so it's in game_side.rs.
        // TableRecipeBook.rim(5): needs the game, so it's in game_side.rs.
        pub fn color(alpha: f32, r: i32, g: i32, b: i32) -> i32 {
            return ((((j_clamp(j_round_f(alpha * 255.0f32), 0, 255)) << 24) | (r << 16)) | (g << 8)) | b;
        }
        // TableRecipeBook.click(3): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.scroll(3): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.charTyped(2): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.keyPressed(3): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.tooltip(2): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.logicalCell(1): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.renderWorld(4): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.drawRim(7): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // TableRecipeBook.strip(11): not translated, it uses Minecraft's `VertexConsumer`.
        // TableRecipeBook.togglePoint(0): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.layPoint(0): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.controlPoint(2): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
        // TableRecipeBook.recipePoint(1): not translated, its class isn't translated (it uses TableStation, which isn't translated (it extends `ClientStation`)).
    }

    /// `TableRecipeGuide` (dev/dyslexic/cinematic/client/TableRecipeGuide.java)
    pub mod table_recipe_guide {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Ingredient`. Its static parts are below.
        pub const SEARCH_STEPS: i32 = 20000;
        // TableRecipeGuide.ingredient(2): not translated, it uses Minecraft's `Ingredient`.
        // TableRecipeGuide.empty(1): not translated, it uses Minecraft's `Ingredient`.
        // TableRecipeGuide.TableRecipeGuide(1): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.entry(1): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.tick(0): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.select(1): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.actual(1): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.refresh(0): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.layout(0): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.gridEmpty(0): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.shapedLayout(4): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.score(1): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.layoutMoves(0): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.workOutMoves(0): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.takeBackMoves(0): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.stuck(0): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.fits(2): not translated, it uses Minecraft's `ItemStack`.
        // TableRecipeGuide.layableAfterTakingBack(0): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.logicalOf(1): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.maxLayout(0): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.workOutMax(0): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.kindOf(2): not translated, it uses Minecraft's `ItemStack`.
        // TableRecipeGuide.assign(9): not translated, it uses Minecraft's `ItemStack`.
        // TableRecipeGuide.place(8): not translated, it uses Minecraft's `ItemStack`.
        // TableRecipeGuide.ghost(1): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.wrong(1): not translated, its class isn't translated (it uses Minecraft's `Ingredient`).
        // TableRecipeGuide.units(1): needs the game, so it's in game_side.rs.
        // TableRecipeGuide.match(2): not translated, it uses Minecraft's `Ingredient`.
        // TableRecipeGuide.augment(5): not translated, it uses Minecraft's `Ingredient`.
    }

    /// `TableRecipeGuide.Entry` (dev/dyslexic/cinematic/client/TableRecipeGuide.java)
    pub mod table_recipe_guide_entry {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `RecipeHolder`. Its static parts are below.
        // TableRecipeGuide.Entry.Entry(2): not translated, its class isn't translated (it uses Minecraft's `RecipeHolder`).
        // TableRecipeGuide.Entry.Entry(3): not translated, its class isn't translated (it uses Minecraft's `RecipeHolder`).
    }

    /// `TableRecipeGuide.MaxLayout` (dev/dyslexic/cinematic/client/TableRecipeGuide.java)
    pub mod table_recipe_guide_max_layout {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct MaxLayout {
            pub sets: i32,
            pub current: i32,
            pub moves: Vec<Vec<i32>>,
        }
        impl MaxLayout {
            pub fn new(sets: i32, current: i32, moves: &[Vec<i32>]) -> Self {
                let mut s = Self { sets: sets, current: current, moves: Vec::new() };
                s.sets = sets;
                s.current = current;
                s.moves = moves.to_vec();
                s
            }
            pub fn sets(&self) -> i32 {
                return self.sets;
            }
            pub fn current(&self) -> i32 {
                return self.current;
            }
            pub fn moves(&self) -> Vec<Vec<i32>> {
                return self.moves.to_vec();
            }
        }
    }

    /// `TableRenderer` (dev/dyslexic/cinematic/client/TableRenderer.java)
    pub mod table_renderer {
        use super::*;
        pub const MAX_DISTANCE_SQR: f64 = 4096.0f64;
        #[derive(Clone, Debug)]
        pub struct TableRenderer {
        }
        impl TableRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TableRenderer.renderAll(5): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
        // TableRenderer.gathering(1): not translated, it uses TableStation, which isn't translated (it extends `ClientStation`).
        // TableRenderer.draw(8): not translated, it uses SceneBuffers, which isn't translated (it's an interface).
    }

    /// `TableStation` (dev/dyslexic/cinematic/client/TableStation.java)
    pub mod table_station {
        use super::*;
        // The class itself isn't translated: it extends `ClientStation`. Its static parts are below.
        pub const HAND_Y: f64 = 0.062f64;
        pub const REST_BACK: f64 = 0.4f64;
        pub const REST_SIDE: f64 = 0.24f64;
        pub const ITEM_REACH: f64 = 0.075f64;
        pub const LEAN_BACK: f64 = 0.86f64;
        pub const LEAN_UP: f64 = 0.72f64;
        pub const LOOK_BACK: f64 = -0.04f64;
        pub const CRAFT_LENGTH: f64 = 0.5f64;
        pub const CRAFT_MID: f64 = 0.2f64;
        pub const RESULT_SIZE: f32 = 0.2f32;
        pub const OCCLUDED_OPACITY: f32 = 0.28f32;
        pub const FADE_RATE: f64 = 14.0f64;
        pub const LAY_MOVE: f64 = 0.36f64;
        pub const LAY_DROP: f64 = 0.64f64;
        pub const TAKE: i32 = -2;
        pub const REPLAN: i32 = -3;
        pub const REPLAN_WAIT: f64 = 1.5f64;
        pub const OCCLUSION_MOVE: f64 = 0.06f64;
        pub static CELL_SAMPLES: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| vec![vec![0.0f64, 0.0f64], vec![-0.3f64, -0.3f64], vec![0.3f64, -0.3f64], vec![-0.3f64, 0.3f64], vec![0.3f64, 0.3f64]]);
        // TableStation.refreshResult(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.TableStation(2): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.kind(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.center(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.approach(1): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.busy(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.showsInventory(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.workPoint(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.workSamples(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.onEnd(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.surfaceY(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.layOut(1): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.quickMove(1): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.onCell(2): not translated, it uses Minecraft's `ItemStack`.
        // TableStation.nextLay(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.layProgress(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.layPath(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.replan(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.beforeFrame(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.cellPoint(1): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.layHands(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.drawLayItem(4): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.worksAtAnAngle(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.lean(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.rest(1): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.updateCursor(1): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.clearCursor(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.updateOcclusion(1): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.inTheWayCopy(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.itemOpacity(1): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.hasFadedItems(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.handTarget(4): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.heldItemPoint(2): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.handsOverride(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.click(2): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.drag(1): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.release(1): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.cancelSpread(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.spreadable(1): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.spreadCells(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.spreadShare(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.spread(1): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.send(4): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.vanillaMenu(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.menuCell(1): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.clickVanilla(3): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.placeVanilla(3): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.craft(1): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.craftProgress(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.tick(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.renderWorld(4): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.drawSpreadPreview(4): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.renderScreen(3): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.drawHighlight(3): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.drawHighlightCell(4): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.highlightStrip(8): not translated, it uses Minecraft's `VertexConsumer`.
        // TableStation.resultPosition(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.resultRadiusGui(0): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.drawResult(4): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.tooltip(1): not translated, its class isn't translated (it extends `ClientStation`).
        // TableStation.hint(0): not translated, its class isn't translated (it extends `ClientStation`).
    }

    /// `VertexConsumerWrapper` (dev/dyslexic/cinematic/client/VertexConsumerWrapper.java)
    pub mod vertex_consumer_wrapper {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `VertexConsumer`. Its static parts are below.
        // VertexConsumerWrapper.VertexConsumerWrapper(1): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // VertexConsumerWrapper.addVertex(3): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // VertexConsumerWrapper.setColor(4): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // VertexConsumerWrapper.setColor(1): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // VertexConsumerWrapper.setUv(2): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // VertexConsumerWrapper.setUv1(2): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // VertexConsumerWrapper.setUv2(2): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // VertexConsumerWrapper.setNormal(3): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // VertexConsumerWrapper.setUv3(2): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
        // VertexConsumerWrapper.setLineWidth(1): not translated, its class isn't translated (it implements Minecraft's `VertexConsumer`).
    }

    /// `EnchantActions` (dev/dyslexic/cinematic/enchant/EnchantActions.java)
    pub mod enchant_actions {
        use super::*;
        // SHOWN: not translated, it uses `UUID` (java.util).
        #[derive(Clone, Debug)]
        pub struct EnchantActions {
        }
        impl EnchantActions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EnchantActions.close(2): not translated, it uses Minecraft's `ServerPlayer`.
        // EnchantActions.tick(2): not translated, it uses Minecraft's `ServerPlayer`.
        // EnchantActions.show(4): not translated, it uses Minecraft's `ServerPlayer`.
        // EnchantActions.onEnchant(1): not translated, it uses Minecraft's `ServerPlayer`.
    }

    /// `FurnaceActions` (dev/dyslexic/cinematic/furnace/FurnaceActions.java)
    pub mod furnace_actions {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct FurnaceActions {
        }
        impl FurnaceActions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FurnaceActions.handle(4): not translated, it uses Minecraft's `ServerPlayer`.
        // FurnaceActions.load(5): not translated, it uses Minecraft's `ServerPlayer`.
        // FurnaceActions.quickMove(4): not translated, it uses Minecraft's `ServerPlayer`.
        // FurnaceActions.takeResult(5): not translated, it uses Minecraft's `ServerPlayer`.
        // FurnaceActions.sendState(2): not translated, it uses Minecraft's `ServerPlayer`.
    }

    /// `FurnaceActions.Outcome` (dev/dyslexic/cinematic/furnace/FurnaceActions.java)
    pub mod furnace_actions_outcome {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Outcome {
            None,
            Placed,
            Taken,
            Refused,
        }
        impl Outcome {
            pub const VALUES: [Outcome; 4] = [Outcome::None, Outcome::Placed, Outcome::Taken, Outcome::Refused, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Outcome::None => "NONE",
                    Outcome::Placed => "PLACED",
                    Outcome::Taken => "TAKEN",
                    Outcome::Refused => "REFUSED",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Outcome::None,
                    "PLACED" => Outcome::Placed,
                    "TAKEN" => Outcome::Taken,
                    "REFUSED" => Outcome::Refused,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `FurnaceLayout` (dev/dyslexic/cinematic/furnace/FurnaceLayout.java)
    pub mod furnace_layout {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `RecipeType`. Its static parts are below.
        // FURNACE: not translated, it uses FurnaceLayout, which isn't translated (it uses Minecraft's `RecipeType`).
        // SMOKER: not translated, it uses FurnaceLayout, which isn't translated (it uses Minecraft's `RecipeType`).
        // BLAST_FURNACE: not translated, it uses FurnaceLayout, which isn't translated (it uses Minecraft's `RecipeType`).
        // FurnaceLayout.FurnaceLayout(6): not translated, its class isn't translated (it uses Minecraft's `RecipeType`).
        pub fn px(pixels: f64) -> f64 {
            return pixels / 16.0f64;
        }
        // FurnaceLayout.of(1): not translated, it uses FurnaceLayout, which isn't translated (it uses Minecraft's `RecipeType`).
        // FurnaceLayout.zoneAt(3): not translated, its class isn't translated (it uses Minecraft's `RecipeType`).
        // FurnaceLayout.zone(1): not translated, its class isn't translated (it uses Minecraft's `RecipeType`).
        // FurnaceLayout.usesTop(0): not translated, its class isn't translated (it uses Minecraft's `RecipeType`).
        // FurnaceLayout.center(1): not translated, its class isn't translated (it uses Minecraft's `RecipeType`).
    }

    /// `FurnaceLayout.Bed` (dev/dyslexic/cinematic/furnace/FurnaceLayout.java)
    pub mod furnace_layout_bed {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Bed {
            pub slot: i32,
            pub face: furnace_zones_face::Face,
            pub u0: f64,
            pub u1: f64,
            pub floor: f64,
            pub depth: f64,
            pub max_copies: i32,
            pub tilt: f32,
        }
        impl Bed {
            pub fn new(slot: i32, face: furnace_zones_face::Face, u0: f64, u1: f64, floor: f64, depth: f64, max_copies: i32, tilt: f32) -> Self {
                let mut s = Self { slot: slot, face: face, u0: u0, u1: u1, floor: floor, depth: depth, max_copies: max_copies, tilt: tilt };
                s.slot = slot;
                s.face = face;
                s.u0 = u0;
                s.u1 = u1;
                s.floor = floor;
                s.depth = depth;
                s.max_copies = max_copies;
                s.tilt = tilt;
                s
            }
            pub fn slot(&self) -> i32 {
                return self.slot;
            }
            pub fn face(&self) -> furnace_zones_face::Face {
                return self.face;
            }
            pub fn u0(&self) -> f64 {
                return self.u0;
            }
            pub fn u1(&self) -> f64 {
                return self.u1;
            }
            pub fn floor(&self) -> f64 {
                return self.floor;
            }
            pub fn depth(&self) -> f64 {
                return self.depth;
            }
            pub fn max_copies(&self) -> i32 {
                return self.max_copies;
            }
            pub fn tilt(&self) -> f32 {
                return self.tilt;
            }
        }
    }

    /// `FurnaceLayout.Glow` (dev/dyslexic/cinematic/furnace/FurnaceLayout.java)
    pub mod furnace_layout_glow {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Glow {
            pub face: furnace_zones_face::Face,
            pub u0: f64,
            pub u1: f64,
            pub v0: f64,
            pub v1: f64,
            pub fire: bool,
        }
        impl Glow {
            pub fn new(face: furnace_zones_face::Face, u0: f64, u1: f64, v0: f64, v1: f64, fire: bool) -> Self {
                let mut s = Self { face: face, u0: u0, u1: u1, v0: v0, v1: v1, fire: fire };
                s.face = face;
                s.u0 = u0;
                s.u1 = u1;
                s.v0 = v0;
                s.v1 = v1;
                s.fire = fire;
                s
            }
            pub fn face(&self) -> furnace_zones_face::Face {
                return self.face;
            }
            pub fn u0(&self) -> f64 {
                return self.u0;
            }
            pub fn u1(&self) -> f64 {
                return self.u1;
            }
            pub fn v0(&self) -> f64 {
                return self.v0;
            }
            pub fn v1(&self) -> f64 {
                return self.v1;
            }
            pub fn fire(&self) -> bool {
                return self.fire;
            }
        }
    }

    /// `FurnaceLayout.Zone` (dev/dyslexic/cinematic/furnace/FurnaceLayout.java)
    pub mod furnace_layout_zone {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Zone {
            pub slot: i32,
            pub face: furnace_zones_face::Face,
            pub u0: f64,
            pub u1: f64,
            pub v0: f64,
            pub v1: f64,
            pub highlight: Vec<f64>,
        }
        impl Zone {
            pub fn new(slot: i32, face: furnace_zones_face::Face, u0: f64, u1: f64, v0: f64, v1: f64, highlight: &[f64]) -> Self {
                let mut s = Self { slot: slot, face: face, u0: u0, u1: u1, v0: v0, v1: v1, highlight: Vec::new() };
                s.slot = slot;
                s.face = face;
                s.u0 = u0;
                s.u1 = u1;
                s.v0 = v0;
                s.v1 = v1;
                s.highlight = highlight.to_vec();
                s
            }
            pub fn contains(&self, f: furnace_zones_face::Face, u: f64, v: f64) -> bool {
                return ((((f == self.face) && (u >= self.u0)) && (u <= self.u1)) && (v >= self.v0)) && (v <= self.v1);
            }
            pub fn slot(&self) -> i32 {
                return self.slot;
            }
            pub fn face(&self) -> furnace_zones_face::Face {
                return self.face;
            }
            pub fn u0(&self) -> f64 {
                return self.u0;
            }
            pub fn u1(&self) -> f64 {
                return self.u1;
            }
            pub fn v0(&self) -> f64 {
                return self.v0;
            }
            pub fn v1(&self) -> f64 {
                return self.v1;
            }
            pub fn highlight(&self) -> Vec<f64> {
                return self.highlight.to_vec();
            }
        }
    }

    /// `FurnaceWatch` (dev/dyslexic/cinematic/furnace/FurnaceWatch.java)
    pub mod furnace_watch {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct FurnaceWatch {
        }
        impl FurnaceWatch {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FurnaceWatch.afterTick(4): not translated, it uses Minecraft's `Level`.
        // FurnaceWatch.shown(1): not translated, it uses Minecraft's `BlockState`.
        // FurnaceWatch.contentsFingerprint(2): not translated, it uses Minecraft's `Container`.
        // FurnaceWatch.fingerprint(1): not translated, it uses Minecraft's `AbstractFurnaceBlockEntity`.
        // FurnaceWatch.contents(2): not translated, it uses FurnaceContentsPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // FurnaceWatch.sync(3): not translated, it uses Minecraft's `ServerLevel`.
        // FurnaceWatch.chunkSent(2): not translated, it uses Minecraft's `ServerPlayer`.
    }

    /// `FurnaceWatch.Fingerprinted` (dev/dyslexic/cinematic/furnace/FurnaceWatch.java)
    pub mod furnace_watch_fingerprinted {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // FurnaceWatch.Fingerprinted.cinematic$fingerprint(0): not translated, it has no body.
        // FurnaceWatch.Fingerprinted.cinematic$setFingerprint(1): not translated, it has no body.
    }

    /// `FurnaceZones` (dev/dyslexic/cinematic/furnace/FurnaceZones.java)
    pub mod furnace_zones {
        use super::*;
        pub const INPUT: i32 = 0;
        pub const FUEL: i32 = 1;
        pub const RESULT: i32 = 2;
        #[derive(Clone, Debug)]
        pub struct FurnaceZones {
        }
        impl FurnaceZones {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FurnaceZones.right(1): not translated, it uses Minecraft's `Direction`.
        // FurnaceZones.point(5): not translated, it uses Minecraft's `Vec3`.
        // FurnaceZones.top(5): not translated, it uses Minecraft's `Vec3`.
        // FurnaceZones.point(6): not translated, it uses Minecraft's `Vec3`.
    }

    /// `FurnaceZones.Face` (dev/dyslexic/cinematic/furnace/FurnaceZones.java)
    pub mod furnace_zones_face {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Face {
            Front,
            Top,
        }
        impl Face {
            pub const VALUES: [Face; 2] = [Face::Front, Face::Top, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Face::Front => "FRONT",
                    Face::Top => "TOP",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "FRONT" => Face::Front,
                    "TOP" => Face::Top,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `AtStationPayload` (dev/dyslexic/cinematic/net/AtStationPayload.java)
    pub mod at_station_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // AtStationPayload.AtStationPayload(3): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // AtStationPayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // AtStationPayload.entityId(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // AtStationPayload.pos(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // AtStationPayload.kind(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `BrewingContentsPayload` (dev/dyslexic/cinematic/net/BrewingContentsPayload.java)
    pub mod brewing_contents_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // BrewingContentsPayload.BrewingContentsPayload(2): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // BrewingContentsPayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // BrewingContentsPayload.pos(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // BrewingContentsPayload.items(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `CampfireProgressPayload` (dev/dyslexic/cinematic/net/CampfireProgressPayload.java)
    pub mod campfire_progress_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // CampfireProgressPayload.CampfireProgressPayload(4): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // CampfireProgressPayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // CampfireProgressPayload.pos(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // CampfireProgressPayload.lit(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // CampfireProgressPayload.progress(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // CampfireProgressPayload.total(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `EnchantDisplayPayload` (dev/dyslexic/cinematic/net/EnchantDisplayPayload.java)
    pub mod enchant_display_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // EnchantDisplayPayload.EnchantDisplayPayload(4): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // EnchantDisplayPayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // EnchantDisplayPayload.pos(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // EnchantDisplayPayload.item(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // EnchantDisplayPayload.lapis(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // EnchantDisplayPayload.side(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `EnchantFxPayload` (dev/dyslexic/cinematic/net/EnchantFxPayload.java)
    pub mod enchant_fx_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // EnchantFxPayload.EnchantFxPayload(2): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // EnchantFxPayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // EnchantFxPayload.pos(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // EnchantFxPayload.entityId(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `FurnaceContentsPayload` (dev/dyslexic/cinematic/net/FurnaceContentsPayload.java)
    pub mod furnace_contents_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // FurnaceContentsPayload.FurnaceContentsPayload(2): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // FurnaceContentsPayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // FurnaceContentsPayload.pos(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // FurnaceContentsPayload.items(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `FurnaceStatePayload` (dev/dyslexic/cinematic/net/FurnaceStatePayload.java)
    pub mod furnace_state_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // FurnaceStatePayload.FurnaceStatePayload(5): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // FurnaceStatePayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // FurnaceStatePayload.pos(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // FurnaceStatePayload.litTime(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // FurnaceStatePayload.litDuration(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // FurnaceStatePayload.cookingProgress(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // FurnaceStatePayload.cookingTotalTime(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `MenuDisplayPayload` (dev/dyslexic/cinematic/net/MenuDisplayPayload.java)
    pub mod menu_display_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // MenuDisplayPayload.MenuDisplayPayload(4): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // MenuDisplayPayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // MenuDisplayPayload.pos(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // MenuDisplayPayload.kind(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // MenuDisplayPayload.side(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // MenuDisplayPayload.items(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `MenuFxPayload` (dev/dyslexic/cinematic/net/MenuFxPayload.java)
    pub mod menu_fx_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // MenuFxPayload.MenuFxPayload(7): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // MenuFxPayload.beat(3): not translated, it uses MenuFxPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // MenuFxPayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // MenuFxPayload.pos(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // MenuFxPayload.kind(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // MenuFxPayload.entityId(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // MenuFxPayload.cue(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // MenuFxPayload.variant(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // MenuFxPayload.items(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // MenuFxPayload.result(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `MenuFxPayload.Cue` (dev/dyslexic/cinematic/net/MenuFxPayload.java)
    pub mod menu_fx_payload_cue {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Cue {
            Beat,
            Move,
            Pull,
            Put,
            Read,
            ReadEnd,
        }
        impl Cue {
            pub const VALUES: [Cue; 6] = [Cue::Beat, Cue::Move, Cue::Pull, Cue::Put, Cue::Read, Cue::ReadEnd, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Cue::Beat => "BEAT",
                    Cue::Move => "MOVE",
                    Cue::Pull => "PULL",
                    Cue::Put => "PUT",
                    Cue::Read => "READ",
                    Cue::ReadEnd => "READ_END",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "BEAT" => Cue::Beat,
                    "MOVE" => Cue::Move,
                    "PULL" => Cue::Pull,
                    "PUT" => Cue::Put,
                    "READ" => Cue::Read,
                    "READ_END" => Cue::ReadEnd,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `Network` (dev/dyslexic/cinematic/net/Network.java)
    pub mod network {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Network {
        }
        impl Network {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Network.enumCodec(1): not translated, it's generic.
        // Network.register(0): not translated, it calls Fabric's `PayloadTypeRegistry`.serverboundPlay.
    }

    /// `PacketDistributor` (dev/dyslexic/cinematic/net/PacketDistributor.java)
    pub mod packet_distributor {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PacketDistributor {
        }
        impl PacketDistributor {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // PacketDistributor.sendToPlayer(2): not translated, it uses Minecraft's `ServerPlayer`.
        // PacketDistributor.sendToPlayersTrackingChunk(3): not translated, it uses Minecraft's `ServerLevel`.
        // PacketDistributor.sendToPlayersTrackingEntityAndSelf(2): not translated, it uses Minecraft's `ServerPlayer`.
        // PacketDistributor.sendToServer(1): not translated, it uses Minecraft's `CustomPacketPayload`.
    }

    /// `ProtocolPayload` (dev/dyslexic/cinematic/net/ProtocolPayload.java)
    pub mod protocol_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ProtocolPayload.ProtocolPayload(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ProtocolPayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `RecipeChoicePayload` (dev/dyslexic/cinematic/net/RecipeChoicePayload.java)
    pub mod recipe_choice_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // RecipeChoicePayload.RecipeChoicePayload(4): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // RecipeChoicePayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // RecipeChoicePayload.pos(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // RecipeChoicePayload.kind(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // RecipeChoicePayload.results(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // RecipeChoicePayload.selected(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `RecipeSyncPayload` (dev/dyslexic/cinematic/net/RecipeSyncPayload.java)
    pub mod recipe_sync_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // RecipeSyncPayload.RecipeSyncPayload(2): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // RecipeSyncPayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // RecipeSyncPayload.vanillaSerializer(1): not translated, it uses Minecraft's `Recipe`.
        // RecipeSyncPayload.reference(1): not translated, it uses Minecraft's `RecipeHolder`.
        // RecipeSyncPayload.send(1): not translated, it uses Minecraft's `ServerPlayer`.
        // RecipeSyncPayload.reset(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // RecipeSyncPayload.recipes(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `SessionClosePayload` (dev/dyslexic/cinematic/net/SessionClosePayload.java)
    pub mod session_close_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // SessionClosePayload.SessionClosePayload(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // SessionClosePayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `StationActionPayload` (dev/dyslexic/cinematic/net/StationActionPayload.java)
    pub mod station_action_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // StationActionPayload.StationActionPayload(7): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // StationActionPayload.of(3): not translated, it uses StationActionPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // StationActionPayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // StationActionPayload.pos(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // StationActionPayload.kind(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // StationActionPayload.action(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // StationActionPayload.index(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // StationActionPayload.button(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // StationActionPayload.shift(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // StationActionPayload.side(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `StationActionPayload.Action` (dev/dyslexic/cinematic/net/StationActionPayload.java)
    pub mod station_action_payload_action {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Action {
            Begin,
            Click,
            Paint,
            Craft,
            Leave,
            Window,
            Place,
            Spread,
            Quick,
            Choose,
        }
        impl Action {
            pub const VALUES: [Action; 10] = [Action::Begin, Action::Click, Action::Paint, Action::Craft, Action::Leave, Action::Window, Action::Place, Action::Spread, Action::Quick, Action::Choose, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Action::Begin => "BEGIN",
                    Action::Click => "CLICK",
                    Action::Paint => "PAINT",
                    Action::Craft => "CRAFT",
                    Action::Leave => "LEAVE",
                    Action::Window => "WINDOW",
                    Action::Place => "PLACE",
                    Action::Spread => "SPREAD",
                    Action::Quick => "QUICK",
                    Action::Choose => "CHOOSE",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "BEGIN" => Action::Begin,
                    "CLICK" => Action::Click,
                    "PAINT" => Action::Paint,
                    "CRAFT" => Action::Craft,
                    "LEAVE" => Action::Leave,
                    "WINDOW" => Action::Window,
                    "PLACE" => Action::Place,
                    "SPREAD" => Action::Spread,
                    "QUICK" => Action::Quick,
                    "CHOOSE" => Action::Choose,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `TableContentsPayload` (dev/dyslexic/cinematic/net/TableContentsPayload.java)
    pub mod table_contents_payload {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Minecraft's `Type`.
        // CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // TableContentsPayload.TableContentsPayload(3): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // TableContentsPayload.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // TableContentsPayload.pos(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // TableContentsPayload.items(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // TableContentsPayload.facing(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `JukeboxActions` (dev/dyslexic/cinematic/station/JukeboxActions.java)
    pub mod jukebox_actions {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct JukeboxActions {
        }
        impl JukeboxActions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // JukeboxActions.handle(4): not translated, it uses Minecraft's `ServerPlayer`.
        // JukeboxActions.show(2): not translated, it uses Minecraft's `ServerPlayer`.
        // JukeboxActions.vanillaUse(2): not translated, it uses Minecraft's `ServerPlayer`.
    }

    /// `MenuActions` (dev/dyslexic/cinematic/station/MenuActions.java)
    pub mod menu_actions {
        use super::*;
        // SHOWN: not translated, it uses `UUID` (java.util).
        // LAST_FX: not translated, it uses `UUID` (java.util).
        pub const FX_COOLDOWN: i32 = 10;
        #[derive(Clone, Debug)]
        pub struct MenuActions {
        }
        impl MenuActions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MenuActions.open(2): not translated, it uses Minecraft's `ServerPlayer`.
        // MenuActions.provider(3): not translated, it uses Minecraft's `MenuProvider`.
        // MenuActions.close(2): not translated, it uses Minecraft's `ServerPlayer`.
        // MenuActions.quickMove(3): not translated, it uses Minecraft's `ServerPlayer`.
        // MenuActions.beaconPayment(2): not translated, it uses Minecraft's `AbstractContainerMenu`.
        // MenuActions.isOpen(2): not translated, it's overloaded with the same number of parameters.
        // MenuActions.isOpen(2): not translated, it's overloaded with the same number of parameters.
        pub fn inputs(kind: station_kind::StationKind) -> i32 {
            return match kind {
                station_kind::StationKind::Anvil | station_kind::StationKind::Grindstone | station_kind::StationKind::Cartography => 2,
                station_kind::StationKind::Smithing | station_kind::StationKind::Loom => 3,
                station_kind::StationKind::Stonecutter | station_kind::StationKind::Beacon => 1,
                station_kind::StationKind::Brewing => 5,
                station_kind::StationKind::Crafter => 9,
                _ => 0,
            };
        }
        // MenuActions.tick(2): not translated, it uses Minecraft's `ServerPlayer`.
        // MenuActions.show(3): not translated, it uses Minecraft's `ServerPlayer`.
        pub fn result_slot(kind: station_kind::StationKind) -> i32 {
            return match kind {
                station_kind::StationKind::Anvil | station_kind::StationKind::Grindstone => 2,
                _ => -1,
                station_kind::StationKind::Stonecutter => 1,
            };
        }
        // MenuActions.handle(3): not translated, it uses Minecraft's `ServerPlayer`.
    }

    /// `RecipeChoices` (dev/dyslexic/cinematic/station/RecipeChoices.java)
    pub mod recipe_choices {
        use super::*;
        pub static POLYMORPH: LazyLock<bool> = LazyLock::new(|| false);
        // LAST_INPUT: not translated, it uses `UUID` (java.util).
        #[derive(Clone, Debug)]
        pub struct RecipeChoices {
        }
        impl RecipeChoices {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RecipeChoices.forget(1): not translated, it uses Minecraft's `ServerPlayer`.
        // RecipeChoices.clear(0): not translated, it uses RecipeChoices.LAST_INPUT, which isn't translated (it uses `UUID` (java.util)).
        // RecipeChoices.fingerprint(2): not translated, it uses Minecraft's `ServerPlayer`.
        pub fn available() -> bool {
            return *recipe_choices::POLYMORPH;
        }
        // RecipeChoices.matching(3): not translated, it's generic.
        // RecipeChoices.chosen(1): not translated, it uses Minecraft's `Player`.
        // RecipeChoices.furnaceChoice(2): not translated, it uses Minecraft's `AbstractFurnaceBlockEntity`.
        // RecipeChoices.cookingType(1): not translated, it uses Minecraft's `RecipeType`.
        // RecipeChoices.sync(2): not translated, it uses Minecraft's `ServerPlayer`.
        // RecipeChoices.choose(4): not translated, it uses Minecraft's `ServerPlayer`.
    }

    /// `RecipeChoices.Polymorph` (dev/dyslexic/cinematic/station/RecipeChoices.java)
    pub mod recipe_choices_polymorph {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Polymorph {
        }
        impl Polymorph {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RecipeChoices.Polymorph.chosen(1): not translated, it's overloaded with the same number of parameters.
        // RecipeChoices.Polymorph.chosen(1): not translated, it's overloaded with the same number of parameters.
        // RecipeChoices.Polymorph.choose(2): not translated, it's overloaded with the same number of parameters.
        // RecipeChoices.Polymorph.choose(2): not translated, it's overloaded with the same number of parameters.
    }

    /// `Sessions` (dev/dyslexic/cinematic/station/Sessions.java)
    pub mod sessions {
        use super::*;
        // SESSIONS: not translated, it uses `UUID` (java.util).
        pub const REACH_SQR: f64 = 36.0f64;
        #[derive(Clone, Debug)]
        pub struct Sessions {
        }
        impl Sessions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Sessions.get(1): not translated, it uses Minecraft's `ServerPlayer`.
        // Sessions.handle(2): not translated, it uses Minecraft's `ServerPlayer`.
        // Sessions.begin(4): not translated, it uses Minecraft's `ServerPlayer`.
        // Sessions.openWindow(4): not translated, it uses Minecraft's `ServerPlayer`.
        // Sessions.end(2): not translated, it uses Minecraft's `ServerPlayer`.
        // Sessions.tick(1): not translated, it uses Minecraft's `ServerPlayer`.
        // Sessions.endAllAt(2): not translated, it uses Minecraft's `ServerLevel`.
        // Sessions.clear(0): not translated, it uses Sessions.SESSIONS, which isn't translated (it uses `UUID` (java.util)).
        // Sessions.sendAtStation(2): not translated, it uses Minecraft's `ServerPlayer`.
        // Sessions.valid(3): not translated, it uses Minecraft's `ServerPlayer`.
    }

    /// `Sessions.Session` (dev/dyslexic/cinematic/station/Sessions.java)
    pub mod sessions_session {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ResourceKey`. Its static parts are below.
        // Sessions.Session.Session(5): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // Sessions.Session.dimension(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // Sessions.Session.pos(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // Sessions.Session.side(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // Sessions.Session.kind(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // Sessions.Session.addonId(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
    }

    /// `ShelfActions` (dev/dyslexic/cinematic/station/ShelfActions.java)
    pub mod shelf_actions {
        use super::*;
        // SHOWN: not translated, it uses `UUID` (java.util).
        // READING: not translated, it uses `UUID` (java.util).
        #[derive(Clone, Debug)]
        pub struct ShelfActions {
        }
        impl ShelfActions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ShelfActions.handle(4): not translated, it uses Minecraft's `ServerPlayer`.
        // ShelfActions.take(6): not translated, it uses Minecraft's `ServerPlayer`.
        // ShelfActions.put(6): not translated, it uses Minecraft's `ServerPlayer`.
        // ShelfActions.sound(2): not translated, it uses Minecraft's `SoundEvent`.
        // ShelfActions.cue(5): not translated, it uses Minecraft's `ServerPlayer`.
        // ShelfActions.show(2): not translated, it uses Minecraft's `ServerPlayer`.
        // ShelfActions.tick(2): not translated, it uses Minecraft's `ServerPlayer`.
        // ShelfActions.fingerprint(1): not translated, it uses Minecraft's `ItemStack`.
        // ShelfActions.close(2): not translated, it uses Minecraft's `ServerPlayer`.
        // ShelfActions.vanillaUse(3): not translated, it uses Minecraft's `ServerPlayer`.
    }

    /// `StationEvents` (dev/dyslexic/cinematic/station/StationEvents.java)
    pub mod station_events {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct StationEvents {
        }
        impl StationEvents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // StationEvents.register(0): not translated, it uses Fabric's `UseBlockCallback`.EVENT.
        // StationEvents.neighbourNotify(2): not translated, it uses Minecraft's `ServerLevel`.
        // StationEvents.chunkSent(3): not translated, it uses Minecraft's `ServerPlayer`.
    }

    /// `StationKind` (dev/dyslexic/cinematic/station/StationKind.java)
    pub mod station_kind {
        use super::*;
        // ANY_SIDE: not translated, it uses Minecraft's `Direction`.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum StationKind {
            Table,
            Furnace,
            Smoker,
            BlastFurnace,
            Enchanting,
            Anvil,
            Grindstone,
            Smithing,
            Stonecutter,
            Loom,
            Cartography,
            Brewing,
            Crafter,
            Campfire,
            Lectern,
            Beacon,
            Jukebox,
            ChiseledBookshelf,
            Addon,
        }
        impl StationKind {
            pub const VALUES: [StationKind; 19] = [StationKind::Table, StationKind::Furnace, StationKind::Smoker, StationKind::BlastFurnace, StationKind::Enchanting, StationKind::Anvil, StationKind::Grindstone, StationKind::Smithing, StationKind::Stonecutter, StationKind::Loom, StationKind::Cartography, StationKind::Brewing, StationKind::Crafter, StationKind::Campfire, StationKind::Lectern, StationKind::Beacon, StationKind::Jukebox, StationKind::ChiseledBookshelf, StationKind::Addon, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    StationKind::Table => "TABLE",
                    StationKind::Furnace => "FURNACE",
                    StationKind::Smoker => "SMOKER",
                    StationKind::BlastFurnace => "BLAST_FURNACE",
                    StationKind::Enchanting => "ENCHANTING",
                    StationKind::Anvil => "ANVIL",
                    StationKind::Grindstone => "GRINDSTONE",
                    StationKind::Smithing => "SMITHING",
                    StationKind::Stonecutter => "STONECUTTER",
                    StationKind::Loom => "LOOM",
                    StationKind::Cartography => "CARTOGRAPHY",
                    StationKind::Brewing => "BREWING",
                    StationKind::Crafter => "CRAFTER",
                    StationKind::Campfire => "CAMPFIRE",
                    StationKind::Lectern => "LECTERN",
                    StationKind::Beacon => "BEACON",
                    StationKind::Jukebox => "JUKEBOX",
                    StationKind::ChiseledBookshelf => "CHISELED_BOOKSHELF",
                    StationKind::Addon => "ADDON",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "TABLE" => StationKind::Table,
                    "FURNACE" => StationKind::Furnace,
                    "SMOKER" => StationKind::Smoker,
                    "BLAST_FURNACE" => StationKind::BlastFurnace,
                    "ENCHANTING" => StationKind::Enchanting,
                    "ANVIL" => StationKind::Anvil,
                    "GRINDSTONE" => StationKind::Grindstone,
                    "SMITHING" => StationKind::Smithing,
                    "STONECUTTER" => StationKind::Stonecutter,
                    "LOOM" => StationKind::Loom,
                    "CARTOGRAPHY" => StationKind::Cartography,
                    "BREWING" => StationKind::Brewing,
                    "CRAFTER" => StationKind::Crafter,
                    "CAMPFIRE" => StationKind::Campfire,
                    "LECTERN" => StationKind::Lectern,
                    "BEACON" => StationKind::Beacon,
                    "JUKEBOX" => StationKind::Jukebox,
                    "CHISELED_BOOKSHELF" => StationKind::ChiseledBookshelf,
                    "ADDON" => StationKind::Addon,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
        impl StationKind {
            // StationKind.matches(1): not translated, it uses Minecraft's `BlockState`.
            // StationKind.isFurnace(0): not translated, `this` used as a value.
            // StationKind.usesMenu(0): not translated, `this` used as a value.
            // StationKind.startsOnUse(0): not translated, `this` used as a value.
            // StationKind.sides(1): not translated, it uses Minecraft's `Direction`.
            // StationKind.usable(3): not translated, it uses Minecraft's `BlockGetter`.
            // StationKind.usedOn(2): not translated, it uses Minecraft's `BlockState`.
        }
        // StationKind.of(1): not translated, it uses Minecraft's `BlockState`.
        // StationKind.roomAt(2): not translated, it uses Minecraft's `BlockGetter`.
    }

    /// `TableActions` (dev/dyslexic/cinematic/table/TableActions.java)
    pub mod table_actions {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TableActions {
        }
        impl TableActions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TableActions.handle(4): not translated, it uses Minecraft's `ServerPlayer`.
        // TableActions.click(6): not translated, it uses Minecraft's `ServerPlayer`.
        // TableActions.paint(4): not translated, it uses Minecraft's `ServerPlayer`.
        // TableActions.spread(4): not translated, it uses Minecraft's `ServerPlayer`.
        // TableActions.place(6): not translated, it uses Minecraft's `ServerPlayer`.
        // TableActions.placeOne(3): not translated, it uses Minecraft's `ItemStack`.
        // TableActions.craft(4): not translated, it uses Minecraft's `ServerPlayer`.
        // TableActions.finish(4): not translated, it uses Minecraft's `ServerPlayer`.
        // TableActions.contents(2): not translated, it uses TableContentsPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // TableActions.sync(2): not translated, it uses Minecraft's `ServerLevel`.
        // TableActions.intoWindow(2): not translated, it uses Minecraft's `ServerPlayer`.
        // TableActions.spill(2): not translated, it uses Minecraft's `ServerLevel`.
    }

    /// `TableCrafting` (dev/dyslexic/cinematic/table/TableCrafting.java)
    pub mod table_crafting {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TableCrafting {
        }
        impl TableCrafting {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TableCrafting.match(3): not translated, it uses Minecraft's `Level`.
        // TableCrafting.match(4): not translated, it uses Minecraft's `Level`.
        // TableCrafting.craftOnce(5): not translated, it uses Minecraft's `ItemStack`.
        // TableCrafting.canFit(2): not translated, it uses Minecraft's `Inventory`.
    }

    /// `TableCrafting.Match` (dev/dyslexic/cinematic/table/TableCrafting.java)
    pub mod table_crafting_match {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `RecipeHolder`. Its static parts are below.
        // TableCrafting.Match.Match(3): not translated, its class isn't translated (it uses Minecraft's `RecipeHolder`).
        // TableCrafting.Match.recipe(0): not translated, its class isn't translated (it uses Minecraft's `RecipeHolder`).
        // TableCrafting.Match.input(0): not translated, its class isn't translated (it uses Minecraft's `RecipeHolder`).
        // TableCrafting.Match.result(0): not translated, its class isn't translated (it uses Minecraft's `RecipeHolder`).
    }

    /// `TableGrid` (dev/dyslexic/cinematic/table/TableGrid.java)
    pub mod table_grid {
        use super::*;
        pub const CELLS: i32 = 9;
        pub const CELL: f64 = 0.1875f64;
        pub const GRID_MIN: f64 = 0.21875f64;
        pub const GRID_MAX: f64 = 0.78125f64;
        #[derive(Clone, Debug)]
        pub struct TableGrid {
        }
        impl TableGrid {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TableGrid.isTable(1): not translated, it uses Minecraft's `BlockState`.
        pub fn cell_x(index: i32) -> f64 {
            return 0.21875f64 + (0.1875f64 * (((index % 3) as f64) + 0.5f64));
        }
        pub fn cell_z(index: i32) -> f64 {
            return 0.21875f64 + (0.1875f64 * (((index / 3) as f64) + 0.5f64));
        }
        pub fn cell_at(local_x: f64, local_z: f64) -> i32 {
            if (((!(local_x < 0.21875f64)) && (!(local_z < 0.21875f64))) && (!(local_x >= 0.78125f64))) && (!(local_z >= 0.78125f64)) {
                let gx: f64 = (local_x - 0.21875f64) / 0.1875f64;
                let gz: f64 = (local_z - 0.21875f64) / 0.1875f64;
                return (gx as i32) + ((gz as i32) * 3);
            } else {
                return -1;
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        // TableGrid.worldIndex(3): not translated, it uses Minecraft's `Direction`.
        // TableGrid.oriented(2): not translated, it uses Minecraft's `ItemStack`.
        // TableGrid.sideFacing(3): not translated, it uses Minecraft's `Direction`.
    }

    /// `TableStore` (dev/dyslexic/cinematic/table/TableStore.java)
    pub mod table_store {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SavedData`. Its static parts are below.
        // TableStore.get(1): not translated, it uses TableStore, which isn't translated (it extends Minecraft's `SavedData`).
        // TableStore.get(1): not translated, it uses TableStore, which isn't translated (it extends Minecraft's `SavedData`).
        // TableStore.getOrCreate(1): not translated, its class isn't translated (it extends Minecraft's `SavedData`).
        // TableStore.remove(1): not translated, its class isn't translated (it extends Minecraft's `SavedData`).
        // TableStore.cleanup(1): not translated, its class isn't translated (it extends Minecraft's `SavedData`).
        // TableStore.inChunk(1): not translated, its class isn't translated (it extends Minecraft's `SavedData`).
        // TableStore.positions(0): not translated, its class isn't translated (it extends Minecraft's `SavedData`).
        // TableStore.save(2): not translated, its class isn't translated (it extends Minecraft's `SavedData`).
        // TableStore.load(2): not translated, it uses TableStore, which isn't translated (it extends Minecraft's `SavedData`).
    }

    /// `TableStore.Table` (dev/dyslexic/cinematic/table/TableStore.java)
    pub mod table_store_table {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `NonNullList`. Its static parts are below.
        // TableStore.Table.isEmpty(0): not translated, its class isn't translated (it uses Minecraft's `NonNullList`).
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
        assert_eq!(ID, "cinematic");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
