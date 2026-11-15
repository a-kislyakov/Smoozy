import bpy, os, sys
from math import radians

out = sys.argv[sys.argv.index("--") + 1]
os.makedirs(out, exist_ok=True)
V = bpy.app.version
NEW_COMPOSITOR = "compositing_node_group" in bpy.types.Scene.bl_rna.properties
STORED = []


def stored(idb, path, value):
    write(idb, path, value)
    STORED.append((idb, path, value))


def write(idb, path, value):
    if path.startswith('["'):
        idb[path[2:-2]] = value
    elif "." in path:
        parent, last = path.rsplit(".", 1)
        setattr(idb.path_resolve(parent), last, value)
    else:
        setattr(idb, path, value)


def fresh():
    bpy.ops.wm.read_factory_settings(use_empty=True)
    STORED.clear()
    one = bpy.data.scenes[0]
    one.name = "Scene"
    return one


def base(one, f0=1, f1=10):
    one.render.engine = "CYCLES"
    one.frame_start = f0
    one.frame_end = f1
    one.frame_current = f0
    one.render.resolution_x = 640
    one.render.resolution_y = 360
    one.render.resolution_percentage = 50
    one.render.filepath = "//render/" + one.name.replace(" ", "_") + "_####"
    stored(one, "cycles.samples", 100)
    if one.world is None:
        one.world = bpy.data.worlds.new("World")


def a_camera(one, name="Camera"):
    cam = bpy.data.objects.new(name, bpy.data.cameras.new(name))
    cam.location = (7, -7, 5)
    cam.rotation_euler = (radians(63), 0, radians(45))
    one.collection.objects.link(cam)
    one.camera = cam
    return cam


def cube_mesh(name):
    me = bpy.data.meshes.new(name)
    s = 1.0
    me.from_pydata([(-s, -s, -s), (s, -s, -s), (s, s, -s), (-s, s, -s), (-s, -s, s), (s, -s, s), (s, s, s), (-s, s, s)], [],
                   [(0, 3, 2, 1), (4, 5, 6, 7), (0, 1, 5, 4), (1, 2, 6, 5), (2, 3, 7, 6), (3, 0, 4, 7)])
    me.update()
    return me


def heavy_collection(name="Heavy", level=8, smooth=300):
    coll = bpy.data.collections.new(name)
    body = bpy.data.objects.new(name + "Model", cube_mesh(name + "Model"))
    sub = body.modifiers.new("Subdivision", "SUBSURF")
    sub.levels = level
    sub.render_levels = level
    smoothing = body.modifiers.new("Smooth", "SMOOTH")
    smoothing.iterations = smooth
    body.modifiers.new("Wave", "WAVE")
    coll.objects.link(body)
    return coll, body


def driver(idb, path, kind="AVERAGE", expression=None, index=-1):
    curve = idb.driver_add(path) if index < 0 else idb.driver_add(path, index)
    for modifier in list(curve.modifiers):
        curve.modifiers.remove(modifier)
    while len(curve.keyframe_points):
        curve.keyframe_points.remove(curve.keyframe_points[0], fast=True)
    curve.driver.type = kind
    if expression is not None:
        curve.driver.expression = expression
    return curve


def single(curve, name, idb, path, id_type):
    variable = curve.driver.variables.new()
    variable.name = name
    variable.type = "SINGLE_PROP"
    target = variable.targets[0]
    target.id_type = id_type
    target.id = idb
    target.data_path = path
    return variable


def context(curve, name, which, path):
    variable = curve.driver.variables.new()
    variable.name = name
    variable.type = "CONTEXT_PROP"
    variable.targets[0].context_property = which
    variable.targets[0].data_path = path
    return variable


def keys(idb, path, pairs, constant=False):
    for frame, value in pairs:
        write(idb, path, value)
        idb.keyframe_insert(data_path=path, frame=frame)
    if constant:
        for curve in curves_of(idb):
            if curve.data_path == path:
                for key in curve.keyframe_points:
                    key.interpolation = "CONSTANT"


def curves_of(idb):
    data = idb.animation_data
    if data is None or data.action is None:
        return []
    found = list(getattr(data.action, "fcurves", ()))
    for layer in getattr(data.action, "layers", ()):
        for strip in layer.strips:
            for bag in getattr(strip, "channelbags", ()):
                found.extend(bag.fcurves)
    return found


def save(name):
    for idb, path, value in STORED:
        write(idb, path, value)
    ids = set()
    for every in (bpy.data.scenes, bpy.data.objects, bpy.data.meshes, bpy.data.cameras, bpy.data.lights,
                  bpy.data.worlds, bpy.data.materials, bpy.data.node_groups, bpy.data.actions,
                  bpy.data.collections, bpy.data.libraries):
        ids.update(every)
    path = os.path.join(out, name + ".blend")
    if os.path.exists(path):
        os.remove(path)
    bpy.data.libraries.write(path, ids, path_remap="NONE", fake_user=False, compress=False)
    print("SMOOZY-DESK-MADE", name, flush=True)


# X01: the Creator's shape, heavy — eight one-frame scenes sharing a heavy
# model, each with a Background view layer whose samples copy Shot 1's.
one = fresh()
coll, _body = heavy_collection()
shots = []
for at in range(1, 9):
    shot = one if at == 1 else bpy.data.scenes.new("Shot")
    shot.name = "Shot %d" % at
    base(shot, at, at)
    shot.collection.children.link(coll)
    a_camera(shot, "Camera %d" % at)
    shot.view_layers[0].name = "ViewLayer"
    shot.view_layers.new("Background")
    stored(shot, 'view_layers["Background"].samples', 64 if at == 1 else 16)
    shots.append(shot)
for shot in shots[1:]:
    curve = driver(shot, 'view_layers["Background"].samples')
    single(curve, "s", shots[0], 'view_layers["Background"].samples', "SCENE")
save("X01 Eight one-frame shots copying Background samples, heavy")

# X02: a Background Scene the driver reads.
one = fresh()
base(one)
a_camera(one)
backdrop = bpy.data.scenes.new("Backdrop")
base(backdrop)
keys(backdrop, '["k"]', [(1, 1.0), (10, 10.0)])
one.background_set = backdrop
curve = driver(one, 'view_layers["ViewLayer"].samples')
single(curve, "k", backdrop, '["k"]', "SCENE")
save("X02 Driver reading the Background Scene")

# X03: a driver reading an object only another scene holds, keyed as a switch.
one = fresh()
base(one)
a_camera(one)
side = bpy.data.scenes.new("Side")
base(side)
holder = bpy.data.objects.new("SideEmpty", None)
side.collection.objects.link(holder)
keys(holder, '["on"]', [(1, 1.0), (4, 0.0), (8, 1.0)], constant=True)
curve = driver(one, 'view_layers["ViewLayer"].use')
single(curve, "on", holder, '["on"]', "OBJECT")
save("X03 Switch driven by another scene's object")

# X04: drivers only on paths the reading never reads, and keys it reads.
one = fresh()
base(one)
a_camera(one)
keys(one, "cycles.samples", [(1, 16), (10, 160)])
driver(one, "view_settings.exposure", "SCRIPTED", "frame / 5")
driver(one, "view_settings.gamma", "SCRIPTED", "1 + frame / 10")
save("X04 Drivers on exposure and gamma only, samples keyed")

# X05: heavy, thirty frames, a switch driven by a keyed property held at runs.
one = fresh()
base(one, 1, 30)
a_camera(one)
coll, _body = heavy_collection(smooth=200)
one.collection.children.link(coll)
second = one.view_layers.new("Second")
keys(one, '["on"]', [(1, 1.0), (10, 0.0), (20, 1.0)], constant=True)
curve = driver(one, 'view_layers["Second"].use')
single(curve, "on", one, '["on"]', "SCENE")
save("X05 Heavy, switch driven by a keyed property")

# X06: a heavy collection held through an instancing empty; samples copy another scene.
one = fresh()
base(one)
a_camera(one)
coll, _body = heavy_collection(smooth=200)
instancer = bpy.data.objects.new("Instancer", None)
instancer.instance_type = "COLLECTION"
instancer.instance_collection = coll
one.collection.objects.link(instancer)
source = bpy.data.scenes.new("Source")
base(source)
stored(source, "cycles.samples", 333)
curve = driver(one, "cycles.samples")
single(curve, "s", source, "cycles.samples", "SCENE")
save("X06 Heavy through a collection instance, samples from another scene")

# X07: a rigid-body world, so the moves leave nothing out.
one = fresh()
base(one)
a_camera(one)
falling = bpy.data.objects.new("Falling", cube_mesh("Falling"))
one.collection.objects.link(falling)
bpy.context.view_layer.objects.active = falling
falling.select_set(True)
bpy.ops.rigidbody.object_add()
source = bpy.data.scenes.new("Source")
base(source)
stored(source, "cycles.samples", 77)
curve = driver(one, "cycles.samples")
single(curve, "s", source, "cycles.samples", "SCENE")
save("X07 Rigid body world, samples from another scene")

# X08: an object whose modifier levels are keyed, so it is never left out.
one = fresh()
base(one)
a_camera(one)
body = bpy.data.objects.new("Keyed", cube_mesh("Keyed"))
one.collection.objects.link(body)
sub = body.modifiers.new("Subdivision", "SUBSURF")
keys(body, 'modifiers["Subdivision"].levels', [(1, 1), (10, 4)])
keys(one, '["a"]', [(1, 40.0), (10, 400.0)])
curve = driver(one, "cycles.samples")
single(curve, "a", one, '["a"]', "SCENE")
save("X08 Keyed modifier levels, samples from a keyed property")

# X09: the active view layer's keyed samples drive the scene's.
one = fresh()
base(one)
a_camera(one)
one.view_layers.new("Second")
keys(one, 'view_layers["ViewLayer"].samples', [(1, 8), (5, 8), (6, 32)], constant=True)
keys(one, 'view_layers["Second"].samples', [(1, 4), (10, 4)])
curve = driver(one, "cycles.samples")
context(curve, "s", "ACTIVE_VIEW_LAYER", "samples")
save("X09 Active view layer's keyed samples drive the scene's")

# X10: a compositor value read by a view-layer switch, through the tree itself.
one = fresh()
base(one)
a_camera(one)
if NEW_COMPOSITOR:
    tree = bpy.data.node_groups.new("Compositing", "CompositorNodeTree")
    one.compositing_node_group = tree
    tree.interface.new_socket("Image", in_out="OUTPUT", socket_type="NodeSocketColor")
    tree.nodes.new("NodeGroupOutput")
else:
    one.use_nodes = True
    tree = one.node_tree
value = tree.nodes.new("CompositorNodeBrightContrast")
value.name = "Gate"
keys(tree, 'nodes["Gate"].inputs[1].default_value', [(1, 1.0), (5, 0.0), (9, 1.0)], constant=True)
second = one.view_layers.new("Second")
curve = driver(one, 'view_layers["Second"].use')
single(curve, "g", tree, 'nodes["Gate"].inputs[1].default_value', "NODETREE")
save("X10 Switch driven by a keyed compositor value")

# X11: a fallback on a path that leads nowhere, and a missing data-block.
one = fresh()
base(one)
a_camera(one)
curve = driver(one, "cycles.samples", "SCRIPTED", "q * 2 + w")
variable = single(curve, "q", one, '["nothing_here"]', "SCENE")
variable.targets[0].use_fallback_value = True
variable.targets[0].fallback_value = 64.0
single(curve, "w", None, '["a"]', "OBJECT")
second = one.view_layers.new("Second")
curve = driver(one, 'view_layers["Second"].use')
variable = single(curve, "m", one, '["missing"]', "SCENE")
variable.targets[0].use_fallback_value = True
variable.targets[0].fallback_value = 1.0
save("X11 Fallback and missing data-block")

# X12: a driver that fails at some frames — division by zero on a keyed input.
one = fresh()
base(one)
a_camera(one)
keys(one, '["a"]', [(1, 1.0), (10, 10.0)])
second = one.view_layers.new("Second")
curve = driver(one, 'view_layers["Second"].use', "SCRIPTED", "1 / (a - 5)")
single(curve, "a", one, '["a"]', "SCENE")
curve = driver(one, "cycles.samples", "SCRIPTED", "100 / (a - 7)")
single(curve, "a", one, '["a"]', "SCENE")
save("X12 Drivers failing partway on a keyed input")

# X13: heavy, markers naming cameras, and a driver copying another scene.
one = fresh()
base(one, 1, 12)
first = a_camera(one, "CamA")
other = a_camera(one, "CamB")
one.camera = first
for name, frame, camera in (("A", 1, first), ("B", 7, other)):
    one.timeline_markers.new(name, frame=frame).camera = camera
coll, _body = heavy_collection(smooth=200)
one.collection.children.link(coll)
source = bpy.data.scenes.new("Source")
base(source)
stored(source, "cycles.samples", 222)
curve = driver(one, "cycles.samples")
single(curve, "s", source, "cycles.samples", "SCENE")
save("X13 Heavy with camera markers, samples from another scene")

# X14: a chain through an object's driven property to a scene's keyed property.
one = fresh()
base(one)
a_camera(one)
keys(one, '["a"]', [(1, 1.0), (5, 0.0), (8, 1.0)], constant=True)
carrier = bpy.data.objects.new("Carrier", None)
one.collection.objects.link(carrier)
carrier["mid"] = 0.0
curve = driver(carrier, '["mid"]')
single(curve, "a", one, '["a"]', "SCENE")
second = one.view_layers.new("Second")
curve = driver(one, 'view_layers["Second"].use')
single(curve, "m", carrier, '["mid"]', "OBJECT")
save("X14 Chain through an object's driven property")

# X15: a heavy object in a library, linked in; samples copy another scene.
library_path = os.path.join(out, "X15 library.blend")
one = fresh()
coll, body = heavy_collection("Linked", smooth=200)
if os.path.exists(library_path):
    os.remove(library_path)
bpy.data.libraries.write(library_path, {coll, body, body.data}, path_remap="NONE", fake_user=True, compress=False)
one = fresh()
base(one)
a_camera(one)
with bpy.data.libraries.load(library_path, link=True) as (source_data, target_data):
    target_data.collections = ["Linked"]
one.collection.children.link(target_data.collections[0])
source = bpy.data.scenes.new("Source")
base(source)
stored(source, "cycles.samples", 111)
curve = driver(one, "cycles.samples")
single(curve, "s", source, "cycles.samples", "SCENE")
save("X15 Linked heavy collection, samples from another scene")

# X16: a driver that fails partway on a switch that is also keyed (Sol's look).
one = fresh()
base(one)
a_camera(one)
second = one.view_layers.new("Second")
keys(one, 'view_layers["Second"].use', [(1, True), (10, True)], constant=True)
keys(one, '["p"]', [(1, 1.0), (2, 0.0)], constant=True)
curve = driver(one, 'view_layers["Second"].use', "SCRIPTED", "1 / p")
single(curve, "p", one, '["p"]', "SCENE")
save("X16 Failing driver on a keyed switch")

# X17: a setting read under its second name (Sol's look).
one = fresh()
base(one)
a_camera(one)
second = one.view_layers.new("Second")
keys(one, "cycles.samples", [(1, 10), (10, 100)])
curve = driver(one, 'view_layers["Second"].use', "SCRIPTED", "s < 50")
single(curve, "s", one, 'cycles["samples"]', "SCENE")
save("X17 Samples read under a second name")
print("SMOOZY-DESK-ALL-MADE", flush=True)
