//! The one piece of the check's reading script the remap material shares — the
//! walk over every path mark the check's kinds list that is not an ID's own
//! `filepath`.
//! **The rest of the script is not here**: it stays in
//! `smoozy-dependencies`' `acting::reading_script`, which re-exports this at
//! the path it always had.

/// **The walk over every path mark G6 lists that is not an ID's own
/// `filepath`** — and it is one text, shared verbatim by the check's reading,
/// the remap material and the fixing, so **a pair reaches exactly what the
/// check saw**. A test asserts all three carry it.
///
/// # Why this exists at all
///
/// **G6's list is about twenty-seven kinds and the eight collections reach
/// nine of them.** Every kind below has its own owner — a node, a modifier, a
/// strip, a surface, a cache layer — and none of them is a data-block in
/// `bpy.data`, so no walk over `bpy.data`'s collections reaches any of them.
/// The map does not close the gap either: it is keyed by ID, so a modifier's
/// path arrives under its **object**, and `BY_TYPE` names no object.
///
/// # Every Blender name below, with the publication it was read from
///
/// **Rule 2, and the same bundle and date as the rest of this file** —
/// Blender's own published Python API reference, **read on 2026-09-04**, the
/// path after each quote being the page inside that bundle. **Nothing here is
/// written from memory.**
///
/// - **`bpy.types.ShaderNodeScript.filepath`** — *"Shader script path"*
///   (`api/bpy.types.ShaderNodeScript.rst`). **G6's OSL script.**
/// - **`bpy.types.ShaderNodeTexIES.filepath`** — *"IES light path"*
///   (`api/bpy.types.ShaderNodeTexIES.rst`). **G6's IES profile.**
/// - **`bpy.types.CacheFileLayer.filepath`** — *"Path to the archive"*, held
///   in `CacheFile.layers`, *"Layers of the cache"*
///   (`api/bpy.types.CacheFileLayer.rst`, `api/bpy.types.CacheFile.rst`).
///   **G6's cache layer, and it is a second location beside the archive's
///   own** — one `CacheFile` carries its `filepath` **and** a layer path each,
///   so a walk over the collection alone reaches none of the layers.
/// - **`bpy.types.MultiresModifier.filepath`** — *"Path to external
///   displacements file"* (`api/bpy.types.MultiresModifier.rst`). **G6's
///   external displacement, the Multires half.**
/// - **`bpy.types.MeshCacheModifier.filepath`** — *"Path to external
///   displacements file"*, the same description word for word
///   (`api/bpy.types.MeshCacheModifier.rst`, read 2026-09-05). **G6's
///   external displacement, the Mesh Cache half**, which the spec names as
///   its own modifier: *"the external displacement files of the Multires and
///   Mesh Cache modifiers"*. **Its type value is quoted rather than
///   guessed**: `:MESH_CACHE: Mesh Cache.` in
///   `api/bpy_types_enum_items/object_modifier_type_items.rst`, read the same
///   day, which is the same page `:MULTIRES:` and `:OCEAN:` are taken from.
///   **Added 2026-09-05, out of round 1 of this module's code audit**: the
///   walk read the modifier's type and branched for two of the three, the map
///   could not cover for it — a modifier's path arrives under its object and
///   `BY_TYPE` names no object, so the entry was dropped — and nothing named
///   it an absence either.
/// - **`bpy.types.PointCache.filepath`** — *"Cache file path"*, with
///   `PointCache.point_caches` holding a `PointCacheItem` carrying the same
///   property (`api/bpy.types.PointCache.rst`,
///   `api/bpy.types.PointCacheItem.rst`). **G6's physics point cache.** Its
///   five owners are that page's own References list: `ClothModifier`,
///   `SoftBodyModifier` and `DynamicPaintSurface` each *"(readonly, never
///   None)"*, `ParticleSystem.point_cache`, and `RigidBodyWorld.point_cache`
///   reached from `Scene.rigidbody_world`.
/// - **`bpy.types.FluidDomainSettings.cache_directory`** — *"Directory that
///   contains fluid cache files"*, reached by `FluidModifier.domain_settings`
///   (`api/bpy.types.FluidDomainSettings.rst`,
///   `api/bpy.types.FluidModifier.rst`). **G6's fluid cache folder.**
/// - **`bpy.types.NodesModifier.bake_directory`** and
///   **`bpy.types.NodesModifierBake.directory`** — both *"Location on disk
///   where the bake data is stored"*, the second held in `NodesModifier.bakes`
///   (`api/bpy.types.NodesModifier.rst`,
///   `api/bpy.types.NodesModifierBake.rst`). **G6's geometry-node bake
///   folder, and it is two places rather than one.**
/// - **`bpy.types.BakeSettings.filepath`** — *"Image filepath to use when
///   saving externally"*, reached by `Scene.render` then `RenderSettings.bake`
///   (`api/bpy.types.BakeSettings.rst`, `api/bpy.types.RenderSettings.rst`).
///   **G6's bake output**, which is a row where a baked result exists to send.
/// - **`bpy.types.OceanModifier.filepath`** — *"Path to a folder to store
///   external baked images"* (`api/bpy.types.OceanModifier.rst`). **G6's
///   ocean bake folder.**
/// - **`bpy.types.DynamicPaintSurface.image_output_path`** — *"Directory to
///   save the textures"*, held in `DynamicPaintCanvasSettings.canvas_surfaces`
///   *"Paint surface list"*, reached by `DynamicPaintModifier.canvas_settings`
///   (`api/bpy.types.DynamicPaintSurface.rst`,
///   `api/bpy.types.DynamicPaintCanvasSettings.rst`). **G6's dynamic-paint
///   output folder**, and the same surface carries a point cache of its own.
/// - **`bpy.types.ImageStrip.directory`** and
///   **`bpy.types.MovieStrip.filepath`** — both *"(default "", never None,
///   blend relative `//` prefix supported)"* and **neither carries a
///   description sentence in the reference at all**
///   (`api/bpy.types.ImageStrip.rst`, `api/bpy.types.MovieStrip.rst`).
///   **G6's sequencer strip folder and file.** Both are reached by
///   `SequenceEditor.strips_all` — *"All strips, recursively including those
///   inside metastrips"* — and **never by `strips`**, which the same page
///   calls *"Top-level strips only"*, so a strip inside a meta strip would be
///   missed (`api/bpy.types.SequenceEditor.rst`).
/// - **`bpy.types.StripProxy.directory`** *"Location to store the proxy
///   files"* and **`bpy.types.StripProxy.filepath`** *"Location of custom
///   proxy file"*, and **`bpy.types.MovieClipProxy.directory`** *"Location to
///   store the proxy files"* reached by `MovieClip.proxy`
///   (`api/bpy.types.StripProxy.rst`, `api/bpy.types.MovieClipProxy.rst`).
///   **G6's proxy folder**, and it is three places.
/// - **`bpy.types.Modifier.type`**, whose values include `:MULTIRES:
///   Multiresolution.` and `:OCEAN: Ocean.`
///   (`api/bpy.types.Modifier.rst`,
///   `api/bpy_types_enum_items/object_modifier_type_items.rst`). **This is
///   how the two modifiers that both carry a bare `filepath` are told
///   apart**, and a third — `MeshCacheModifier` — carries the same property
///   name again, so `hasattr` would lump all three into one kind.
///
/// # Three kinds on G6's list produce no reference here, each a named absence
///
/// **Rule 2 forbids the alternative**, which is a walker written on a
/// remembered property name. Each of these is stated where the walk would
/// otherwise have reached it, with what breaks.
///
/// - **A brush icon: NO ROUTE ESTABLISHED, and this is the one flat
///   absence.** `bpy.types.Brush` carries **no path-shaped property at all**
///   in the reference — no `icon_filepath`, and no attribute whose name holds
///   `filepath`, `path`, `directory` or `folder`. The nearest thing that
///   exists is an operator argument, `bpy.ops.brush.asset_load_preview(*,
///   filepath="")` — *"Choose a preview image for the brush"*
///   (`api/bpy.ops.brush.rst`) — which is a thing you can do, not a thing you
///   can read back. The manual names the field — *"Image Path / Defines the
///   path to the image to use as custom icon."*
///   (`manual/grease_pencil/modes/draw/tool_settings/brushes.rst`) — and names
///   no property. **What breaks:** a file whose brush carries a custom icon
///   has that icon unfound and undisclosed, and a render that draws with such
///   a brush draws without the icon, which costs the picture nothing. **What
///   holds:** everything else on G6's list, and G7's own honesty — the check
///   never claims the list is all of them.
/// - **The record kept with a packed file: the route exists and no reference
///   is produced, deliberately.** `bpy.types.PackedFile` carries exactly two
///   members, `data` *"Raw data (bytes, exact content of the embedded file)"*
///   and `size`, and **no path** (`api/bpy.types.PackedFile.rst`); the only
///   packed record that holds one is `bpy.types.ImagePackedFile.filepath`
///   (`api/bpy.types.ImagePackedFile.rst`). **A packed file's bytes are inside
///   the `.blend` and travel with it**, and this project's own research
///   records that *a packed picture ignores its file on disk* (V8, ***Reading
///   a file 11***). Blender's own walker agrees: `file_path_foreach` defaults
///   to `flags={'SKIP_PACKED', 'SKIP_WEAK_REFERENCES'}`
///   (`api/bpy.types.BlendData.rst`). **What breaks:** the record's own text
///   is never shown and never sent. **What holds:** nothing is missing at the
///   render, because the bytes are in the file — a row here would ask the user
///   to find a file the render will not read.
/// - **The weak reference kept after appending: read, and never a row.** The
///   reading already carries every one of them for G27's exclusion. It is not
///   a location the render resolves: the reference calls it a *"Read-only
///   external reference to a linked data-block and its library file"*, its
///   owner property is marked `(readonly)`
///   (`api/bpy.types.LibraryWeakReference.rst`, `api/bpy.types.ID.rst`), and
///   Blender's own walker skips it by the same default flag as packed data.
///   **What breaks:** a weak reference's recorded library is never offered as
///   something to find or fix. **What holds:** it cannot be assigned, so a row
///   for it would carry an action that raises — the spec's *a row that cannot
///   be acted on is a control that does nothing*, in its worst form.
///
/// **And one property inside a kind that is reached**: a strip element's own
/// `bpy.types.StripElement.filename` is *"Name of the source file"* and is
/// **the one path-shaped property in this whole walk whose reference entry
/// carries no `blend relative //` clause** (`api/bpy.types.StripElement.rst`)
/// — it is a name and not a location. It is not read as a reference, because
/// the location is the strip's own `directory`, which is. **What breaks:** an
/// element naming a file outside its strip's folder, which the reference does
/// not establish is possible.
pub const THE_WALK_OVER_THE_NESTED_MARKS: &str = r#"
import bpy, os

# **G6's kinds that are not an ID's own `filepath`.** Each yield is
# (the kind's name, a label saying where it sits, the object carrying the
# setting, the attribute on that object). The check walks this to enumerate and
# the remap walks it to assign, so a pair reaches exactly what the check saw.

def smoozy_trees_with_owners():
    seen = set()
    frontier = []
    for tree in bpy.data.node_groups:
        frontier.append((tree, tree.name))
    for collection in ("materials", "worlds", "lights", "textures", "linestyles"):
        holder = getattr(bpy.data, collection, None)
        if holder is None:
            continue
        for owner in holder:
            tree = getattr(owner, "node_tree", None)
            if tree is not None:
                frontier.append((tree, owner.name))
    for scene in bpy.data.scenes:
        # From 5.3 every effect's group, and the older one-group name is never
        # read: it answers only the active effect's, and warns.
        # `Job files and outputs` R4 item 1. Before 5.3 a scene has no such stack.
        effects = getattr(scene, "compositor_effects", None)
        if effects is None:
            trees = [getattr(scene, "compositing_node_group", None)]
        else:
            trees = [effect.node_group for effect in effects]
        for tree in trees:
            if tree is not None:
                frontier.append((tree, scene.name))
    for obj in bpy.data.objects:
        for modifier in getattr(obj, "modifiers", ()):
            tree = getattr(modifier, "node_group", None)
            if tree is not None:
                frontier.append((tree, obj.name))
    while frontier:
        tree, owner_name = frontier.pop()
        key = id(tree)
        if key in seen:
            continue
        seen.add(key)
        yield tree, owner_name
        for node in getattr(tree, "nodes", ()):
            inner = getattr(node, "node_tree", None)
            if inner is not None:
                frontier.append((inner, owner_name))

def smoozy_point_cache_marks(cache, label, owner):
    # The active cache carries `filepath`, and `point_caches` carries one item
    # per cache with the same property. Both are walked; two naming the same
    # place group into one row, which is what K3's keying is for.
    if cache is None:
        return
    yield ("PhysicsPointCache", label, cache, "filepath", owner)
    for at, item in enumerate(getattr(cache, "point_caches", ())):
        yield ("PhysicsPointCache", label + "/point_caches[" + str(at) + "]", item, "filepath", owner)

# The kinds this walk reaches. A pair of one of these is applied by re-walking
# the generator; a kind absent from here is either an ID's own `filepath` or
# one of the named absences the Rust comment above states.
SMOOZY_NESTED_KINDS = (
    "OslScript",
    "IesProfile",
    "CacheLayer",
    "MultiresExternalDisplacement",
    "MeshCacheExternalDisplacement",
    "PhysicsPointCache",
    "FluidCacheFolder",
    "GeometryNodeBakeFolder",
    "BakeFolder",
    "OceanBakeFolder",
    "DynamicPaintOutputFolder",
    "SequencerStripFolder",
    "SequencerStripFile",
    "ProxyFolder",
)

def smoozy_nested_marks():
    # -- shader nodes: the OSL script and the IES profile.
    for tree, owner in smoozy_trees_with_owners():
        for node in getattr(tree, "nodes", ()):
            idname = getattr(node, "bl_idname", "")
            if idname == "ShaderNodeScript":
                yield ("OslScript", owner + "/" + node.name, node, "filepath", tree)
            elif idname == "ShaderNodeTexIES":
                yield ("IesProfile", owner + "/" + node.name, node, "filepath", tree)

    # -- a cache archive's layers, each a location beside the archive's own.
    for cache in bpy.data.cache_files:
        for at, layer in enumerate(getattr(cache, "layers", ())):
            yield ("CacheLayer", cache.name + "/layers[" + str(at) + "]", layer, "filepath", cache)

    # -- the bake output path, one per scene, and the rigid body world's cache.
    for scene in bpy.data.scenes:
        settings = getattr(scene, "render", None)
        bake = getattr(settings, "bake", None) if settings is not None else None
        if bake is not None:
            yield ("BakeFolder", scene.name, bake, "filepath", scene)
        world = getattr(scene, "rigidbody_world", None)
        if world is not None:
            for one in smoozy_point_cache_marks(getattr(world, "point_cache", None), scene.name, scene):
                yield one

    # -- everything an object carries.
    for obj in bpy.data.objects:
        for modifier in getattr(obj, "modifiers", ()):
            where = obj.name + "/" + modifier.name
            # Cloth and soft body each hold a point cache under this name.
            for one in smoozy_point_cache_marks(getattr(modifier, "point_cache", None), where, obj):
                yield one
            domain = getattr(modifier, "domain_settings", None)
            if domain is not None:
                yield ("FluidCacheFolder", where, domain, "cache_directory", obj)
            if hasattr(modifier, "bake_directory"):
                yield ("GeometryNodeBakeFolder", where, modifier, "bake_directory", obj)
            for at, bake in enumerate(getattr(modifier, "bakes", ())):
                yield ("GeometryNodeBakeFolder", where + "/bakes[" + str(at) + "]", bake, "directory", obj)
            # **Three modifier kinds carry a bare `filepath` and only the type
            # tells them apart**, so the type is read and never guessed at from
            # the property's presence.
            of_type = getattr(modifier, "type", "")
            if of_type == "OCEAN":
                yield ("OceanBakeFolder", where, modifier, "filepath", obj)
            elif of_type == "MULTIRES":
                yield ("MultiresExternalDisplacement", where, modifier, "filepath", obj)
            elif of_type == "MESH_CACHE":
                yield ("MeshCacheExternalDisplacement", where, modifier, "filepath", obj)
            canvas = getattr(modifier, "canvas_settings", None)
            if canvas is not None:
                for at, surface in enumerate(getattr(canvas, "canvas_surfaces", ())):
                    at_surface = where + "/surfaces[" + str(at) + "]"
                    yield ("DynamicPaintOutputFolder", at_surface, surface, "image_output_path", obj)
                    for one in smoozy_point_cache_marks(
                        getattr(surface, "point_cache", None), at_surface, obj
                    ):
                        yield one
        for system in getattr(obj, "particle_systems", ()):
            for one in smoozy_point_cache_marks(
                getattr(system, "point_cache", None), obj.name + "/" + system.name, obj
            ):
                yield one

    # -- the sequencer: a strip's folder, a movie strip's file, and the proxies.
    for scene in bpy.data.scenes:
        editor = getattr(scene, "sequence_editor", None)
        if editor is None:
            continue
        # `strips_all` and never `strips`: the second is top-level only, so a
        # strip inside a meta strip would go unread.
        for strip in getattr(editor, "strips_all", ()):
            where = scene.name + "/" + strip.name
            if hasattr(strip, "directory"):
                yield ("SequencerStripFolder", where, strip, "directory", scene)
            if hasattr(strip, "filepath"):
                yield ("SequencerStripFile", where, strip, "filepath", scene)
            proxy = getattr(strip, "proxy", None)
            if proxy is not None:
                yield ("ProxyFolder", where + "/proxy", proxy, "directory", scene)
                yield ("ProxyFolder", where + "/proxy", proxy, "filepath", scene)

    # -- a movie clip's own proxy folder.
    for clip in bpy.data.movieclips:
        proxy = getattr(clip, "proxy", None)
        if proxy is not None:
            yield ("ProxyFolder", clip.name + "/proxy", proxy, "directory", clip)
"#;
