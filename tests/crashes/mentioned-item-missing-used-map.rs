// Regression test for a panic in `UsageMap::for_each_inlined_used_item` at
// compiler/rustc_monomorphize/src/collector.rs:304 where `used_map.get(&item)`
// returns `None` and `.unwrap()` panics.
//
// Root cause
// ----------
// `used_map` entries are only inserted when `collect_items_rec` runs with
// `CollectionMode::UsedItems`. If a `LocalCopy` (inlined) mono item is visited
// *first* via `CollectionMode::MentionedItems` it enters the global `visited`
// set. Later, when a parent item lists it as a *used* sub-item, the guard at
// collector.rs:583 sees it is already visited and skips
// `collect_items_rec(item, UsedItems)` -- so the item never gets a `used_map`
// entry. Partitioning's `get_reachable_inlined_items` then recurses into that
// item and calls `used_map.get(&item).unwrap()` → ICE.
//
// Required scenario
// -----------------
//   root_a  (GloballyShared) --used-->  middle (LocalCopy)
//                                           --used-->  leaf (LocalCopy)
//   root_b  (GloballyShared) --mentioned-->  leaf
//
// `root_b` contains `if false { leaf() }`.  The `MentionedItems` MIR pass runs
// before any optimization and records `leaf` in `root_b`'s mentioned_items.
// The later `SimplifyCfg-final` pass removes the dead branch, so `leaf` does
// NOT appear in `root_b`'s `used_items` during monomorphization collection.
//
// At `-Copt-level=1` the MIR inliner is disabled (mir_opt_level 1 → Inline
// pass returns false), so the chain root_a→middle→leaf is preserved in
// `used_map`. Plain `#[inline]` (not `#[inline(always)]`) is used to avoid the
// `ForceInline` pass which runs unconditionally and would collapse the chain.
//
// Race trigger
// ------------
// If `root_b`'s MentionedItems traversal runs in a parallel thread *before*
// `middle`'s UsedItems traversal, `leaf` enters `visited` but not `used_map`.
// When `middle` is later processed as UsedItems it lists `leaf` as a used
// sub-item, but the `visited` guard skips `collect_items_rec(leaf, UsedItems)`
// -- `leaf` never gets a `used_map` entry. Partitioning then panics when
// traversing root_a→middle→leaf via `for_each_inlined_used_item`.
//
// This ordering is rare on x86_64 but occurs more often on s390x.
// Using `-Ccodegen-units=4 -Zthreads=8` maximises the chance of triggering it.
//
//@ known-bug: unknown
//@ compile-flags: -Copt-level=1 -Ccodegen-units=4 -Zthreads=8

#![crate_type = "lib"]

/// Leaf inlined function (`LocalCopy`).
/// This item ends up with no `used_map` entry when the race fires:
/// it is visited via MentionedItems (from `root_b`) before it is
/// visited via UsedItems (from `middle`).
#[inline]
pub fn leaf() -> u32 {
    42
}

/// Middle inlined function (`LocalCopy`).
/// `used_map[middle]` = [leaf], so partitioning recurses into `leaf`
/// and calls `for_each_inlined_used_item(leaf)` → panic.
#[inline]
pub fn middle() -> u32 {
    leaf()
}

/// Root A — non-inline (`GloballyShared`).
/// Used-items chain: root_a → middle → leaf.
/// Partitioning path: get_reachable_inlined_items(root_a)
///   → for_each_inlined_used_item(root_a) finds `middle` (LocalCopy)
///   → for_each_inlined_used_item(middle) finds `leaf` (LocalCopy)
///   → for_each_inlined_used_item(leaf) → used_map[leaf].unwrap() → ICE
pub fn root_a() -> u32 {
    middle()
}

/// Root B — non-inline (`GloballyShared`).
/// `if false` is folded by `SimplifyConstCondition`/`SimplifyCfg-final`
/// in the optimized (post-analysis) MIR pipeline, removing the call to
/// `leaf()` from the body seen by the used-items collector. However, the
/// `MentionedItems` pass ran before any optimization and already recorded
/// `leaf` in `root_b`'s `mentioned_items` list, which is what causes `leaf`
/// to be processed via `CollectionMode::MentionedItems` and enter `visited`
/// before `middle`'s UsedItems traversal claims it.
pub fn root_b() -> u32 {
    if false { leaf() } else { 0 }
}
