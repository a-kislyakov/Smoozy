# A two-file project with a set linked into a shot — the fixture the 38's
# piece 7 made at the desk, because no test scene before it linked a library
# (`Audits/Build/Step 10c/The 38, piece 7 — what landed.md`).
#
#   assets/tex/checker.png   a red-and-white checker, 64 by 64
#   assets/set.blend         collection "Set": a cube whose material reads
#                            //tex/checker.png (relative to set.blend)
#   shots/shot.blend         camera, sun and a grey floor of its own, and the
#                            collection "Set" linked from //../assets/set.blend
#                            and placed by one collection instance
#
# The shot renders the checkered cube only where the library is found AND the
# library's own texture is found. A shot whose library is missing renders the
# floor alone; a library found whose texture is missing renders the cube in
# Blender's missing-texture pink.
#
# Made with Blender 4.5.12 so every Blender Smoozy offers opens it:
#   "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b \
#       --factory-startup --python make.py
# It writes beside itself and over what is there.

import os

import bpy

HERE = os.path.dirname(os.path.abspath(__file__))
ASSETS = os.path.join(HERE, "assets")
SHOTS = os.path.join(HERE, "shots")
os.makedirs(os.path.join(ASSETS, "tex"), exist_ok=True)
os.makedirs(SHOTS, exist_ok=True)


def empty_file():
    bpy.ops.wm.read_factory_settings(use_empty=True)
    # No `.blend1` beside a file saved twice.
    bpy.context.preferences.filepaths.save_version = 0


# ------------------------------------------------------------ the texture
empty_file()
SIZE = 64
image = bpy.data.images.new("checker", SIZE, SIZE)
pixels = []
for y in range(SIZE):
    for x in range(SIZE):
        red = ((x // 8) + (y // 8)) % 2 == 0
        pixels.extend((0.9, 0.05, 0.05, 1.0) if red else (0.95, 0.95, 0.95, 1.0))
image.pixels = pixels
image.filepath_raw = os.path.join(ASSETS, "tex", "checker.png")
image.file_format = "PNG"
image.save()

# ------------------------------------------------------------ the set
empty_file()
bpy.ops.wm.save_as_mainfile(filepath=os.path.join(ASSETS, "set.blend"))
the_set = bpy.data.collections.new("Set")
bpy.context.scene.collection.children.link(the_set)
mesh = bpy.data.meshes.new("Crate")
mesh.from_pydata(
    [(-1, -1, 0), (1, -1, 0), (1, 1, 0), (-1, 1, 0), (-1, -1, 2), (1, -1, 2), (1, 1, 2), (-1, 1, 2)],
    [],
    [(0, 3, 2, 1), (4, 5, 6, 7), (0, 1, 5, 4), (1, 2, 6, 5), (2, 3, 7, 6), (3, 0, 4, 7)],
)
mesh.update()
uv = mesh.uv_layers.new(name="UVMap")
corners = [(0, 0), (1, 0), (1, 1), (0, 1)]
for polygon in mesh.polygons:
    for at, loop in enumerate(polygon.loop_indices):
        uv.data[loop].uv = corners[at % 4]
crate = bpy.data.objects.new("Crate", mesh)
the_set.objects.link(crate)
material = bpy.data.materials.new("Checker")
material.use_nodes = True
nodes = material.node_tree.nodes
texture = nodes.new("ShaderNodeTexImage")
texture.image = bpy.data.images.load(os.path.join(ASSETS, "tex", "checker.png"))
texture.image.filepath = "//tex/checker.png"
shader = nodes.get("Principled BSDF")
material.node_tree.links.new(texture.outputs["Color"], shader.inputs["Base Color"])
mesh.materials.append(material)
bpy.ops.wm.save_as_mainfile(filepath=os.path.join(ASSETS, "set.blend"), relative_remap=False)

# ------------------------------------------------------------ the shot
empty_file()
scene = bpy.context.scene
bpy.ops.wm.save_as_mainfile(filepath=os.path.join(SHOTS, "shot.blend"))
floor_mesh = bpy.data.meshes.new("Floor")
floor_mesh.from_pydata([(-6, -6, 0), (6, -6, 0), (6, 6, 0), (-6, 6, 0)], [], [(0, 1, 2, 3)])
floor_mesh.update()
floor = bpy.data.objects.new("Floor", floor_mesh)
scene.collection.objects.link(floor)
camera = bpy.data.objects.new("Camera", bpy.data.cameras.new("Camera"))
camera.location = (6.0, -6.0, 4.5)
camera.rotation_euler = (1.1, 0.0, 0.785)
scene.collection.objects.link(camera)
scene.camera = camera
sun = bpy.data.objects.new("Sun", bpy.data.lights.new("Sun", type="SUN"))
sun.rotation_euler = (0.6, 0.2, 0.4)
sun.data.energy = 3.0
scene.collection.objects.link(sun)
with bpy.data.libraries.load(os.path.join(ASSETS, "set.blend"), link=True, relative=True) as (
    data_from,
    data_to,
):
    data_to.collections = ["Set"]
placed = bpy.data.objects.new("Set", None)
placed.instance_type = "COLLECTION"
placed.instance_collection = data_to.collections[0]
scene.collection.objects.link(placed)
scene.render.engine = "CYCLES"
scene.cycles.samples = 16
scene.cycles.device = "CPU"
scene.render.resolution_x = 640
scene.render.resolution_y = 360
scene.render.resolution_percentage = 100
scene.frame_start = 1
scene.frame_end = 1
bpy.ops.wm.save_as_mainfile(filepath=os.path.join(SHOTS, "shot.blend"), relative_remap=False)

print("SMOOZY-FIXTURE library:", [library.filepath for library in bpy.data.libraries])
