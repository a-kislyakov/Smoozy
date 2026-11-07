//! The kinds a reference can be — `Dependency Upload & Remap` §3's list, with
//! the import-node path and the folder-shaped kinds. **Everything else of
//! the check is not here**: its judgements stay in `smoozy-dependencies`'
//! `check`, which re-exports [`Kind`] at the path it always had. The kind
//! moved because the remap material's pairs carry one.

use serde::{Deserialize, Serialize};

/// **G6: the kinds enumerated are the spec's list**, plus G8's import-node
/// path, read off the node itself in every version. **Write-only destinations
/// are not rows** — they are `WriteOnlyDestination`, which stayed in
/// `smoozy_dependencies::check`, and the redirect's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Kind {
    Library,
    /// The weak reference kept after appending.
    LibraryWeakReference,
    Image,
    MovieClip,
    Sound,
    Font,
    Volume,
    CacheArchive,
    CacheLayer,
    Text,
    OslScript,
    IesProfile,
    /// **The spec names two modifiers individually** — *"the external
    /// displacement files of the Multires and Mesh Cache modifiers"* — and
    /// one variant covering both hid half the kind: the script's walk had no
    /// Mesh Cache branch, and the guard that holds every kind against the
    /// script passed on the Multires half. Split 2026-09-05, out of round 1
    /// of this module's code audit.
    MultiresExternalDisplacement,
    MeshCacheExternalDisplacement,
    PhysicsPointCache,
    FluidCacheFolder,
    GeometryNodeBakeFolder,
    /// The bake output path read as a place a baked result already sits —
    /// `Job files and outputs` D20's first write place, a row where a baked
    /// result exists to send (G6), and one of M19's seven inferred kinds.
    BakeFolder,
    OceanBakeFolder,
    DynamicPaintOutputFolder,
    SequencerStripFolder,
    SequencerStripFile,
    ProxyFolder,
    BrushIcon,
    /// The record kept with a packed file.
    PackedFileRecord,
    /// **G8: an import node's path, read off the node itself** — six kinds of
    /// import node in 4.5 to 5.2 and seven in 5.3, plus the sound route in 5.2
    /// and 5.3. In 4.5 that path takes no part in the file's own path
    /// housekeeping, so the node walk is the only route to it.
    ImportNodePath,
}

impl Kind {
    /// **Every kind, so a walk over them is not a list somebody keeps.**
    pub const ALL: [Kind; 26] = [
        Kind::Library,
        Kind::LibraryWeakReference,
        Kind::Image,
        Kind::MovieClip,
        Kind::Sound,
        Kind::Font,
        Kind::Volume,
        Kind::CacheArchive,
        Kind::CacheLayer,
        Kind::Text,
        Kind::OslScript,
        Kind::IesProfile,
        Kind::MultiresExternalDisplacement,
        Kind::MeshCacheExternalDisplacement,
        Kind::PhysicsPointCache,
        Kind::FluidCacheFolder,
        Kind::GeometryNodeBakeFolder,
        Kind::BakeFolder,
        Kind::OceanBakeFolder,
        Kind::DynamicPaintOutputFolder,
        Kind::SequencerStripFolder,
        Kind::SequencerStripFile,
        Kind::ProxyFolder,
        Kind::BrushIcon,
        Kind::PackedFileRecord,
        Kind::ImportNodePath,
    ];

    /// **G6: a recorded write place that is also read — a cache or a bake —
    /// is a row where a baked result exists to send**, per the spec's
    /// found-and-sent rule; where its uploaded copy is read from is M22a's.
    pub fn is_a_write_place_that_is_also_read(self) -> bool {
        matches!(
            self,
            Kind::PhysicsPointCache
                | Kind::FluidCacheFolder
                | Kind::GeometryNodeBakeFolder
                | Kind::BakeFolder
                | Kind::OceanBakeFolder
                | Kind::DynamicPaintOutputFolder
                | Kind::ProxyFolder
        )
    }

    /// **F28: the kinds whose recorded location names a folder rather than a
    /// file.** G6's folder-shaped kinds — a physics point cache, a fluid
    /// cache folder, a geometry-node bake folder, an ocean bake folder, a
    /// dynamic-paint output folder, a sequencer strip folder and a proxy
    /// folder — with the bake output path beside them, G6 making that a row
    /// on the Creator's ruling of 2026-08-22 (M22a).
    ///
    /// **A reference of one of these is matched on the final *named* segment
    /// of its recorded location** against a directory entry's own name
    /// ([`crate::bytes::RecordedLocation::final_named_segment`]), under F9's
    /// byte equality and F10's fold exactly as a file is. Everything else on
    /// G6's list names a file.
    pub fn names_a_folder(self) -> bool {
        matches!(
            self,
            Kind::PhysicsPointCache
                | Kind::FluidCacheFolder
                | Kind::GeometryNodeBakeFolder
                | Kind::BakeFolder
                | Kind::OceanBakeFolder
                | Kind::DynamicPaintOutputFolder
                | Kind::SequencerStripFolder
                | Kind::ProxyFolder
        )
    }
}
