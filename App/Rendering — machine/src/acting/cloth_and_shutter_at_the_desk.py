"""Rendering L10b, Job files and outputs D21b-i: small desk scenes, actual material.
Run by the ignored Rust test; each Blender process has a 90-second limit.
No user files, UI, server, or Smoozy executable. Keeps evidence below --out.
"""
import argparse, hashlib, json, pathlib, subprocess
p = argparse.ArgumentParser()
p.add_argument('--blender', required=True)
p.add_argument('--out', required=True)
p.add_argument('--material', required=True)
a = p.parse_args()
root = pathlib.Path(a.out)
root.mkdir(parents=True, exist_ok=True)

def run(name, words):
    result = subprocess.run([a.blender, '--background', '--factory-startup', '--disable-autoexec', '--python-exit-code', '1', '-t', '2', *words], capture_output=True, timeout=90)
    (root / (name + '.log')).write_bytes(result.stdout + result.stderr)
    assert result.returncode == 0, (name, result.returncode, result.stderr[-2000:])

maker = r'''
import bpy, sys, math, os
from mathutils import Vector
out, kind = sys.argv[sys.argv.index('--')+1:]
bpy.ops.wm.read_factory_settings(use_empty=True)
s = bpy.context.scene
s.name = 'Scene'
s.render.engine = 'CYCLES'
s.cycles.device = 'CPU'
s.cycles.samples = 1
s.cycles.use_denoising = False
s.cycles.use_adaptive_sampling = False
s.render.resolution_x = s.render.resolution_y = 96
s.render.resolution_percentage = 100
s.render.image_settings.file_format = 'BMP'
s.render.image_settings.color_mode = 'RGB'
s.render.filepath = '//f_'
s.frame_start, s.frame_end = 20, 62
s.world = bpy.data.worlds.new('World')
s.world.color = (0.2,0.2,0.2)
material = bpy.data.materials.new('White')
material.diffuse_color = (0.8,0.8,0.8,1)
material.use_nodes=True
nodes=material.node_tree.nodes
nodes.clear()
e=nodes.new('ShaderNodeEmission'); e.inputs[0].default_value=(1,1,1,1)
o=nodes.new('ShaderNodeOutputMaterial');material.node_tree.links.new(e.outputs[0],o.inputs['Surface'])
bpy.ops.mesh.primitive_grid_add(x_subdivisions=10,y_subdivisions=10,size=2,location=(0,0,2))
cloth=bpy.context.object;cloth.name='Cloth'
cloth.data.materials.append(material)
g=cloth.vertex_groups.new(name='Pin')
g.add([v.index for v in cloth.data.vertices if v.co.y > 0.9],1,'REPLACE')
m=cloth.modifiers.new('Cloth','CLOTH'); m.settings.vertex_group_mass=g.name
m.point_cache.frame_start = {'early':1,'at':20,'after':30,'mixed':1,'video':1,'linked':1,'override':1,'instance':1}.get(kind,1)
m.point_cache.frame_end=100
if kind == 'mixed':
    bpy.ops.mesh.primitive_cube_add(size=.25,location=(2,0,4)); body=bpy.context.object;body.name='Body'
    bpy.ops.rigidbody.object_add(); s.rigidbody_world.point_cache.frame_start=20
    body.data.materials.append(material)
    bpy.ops.mesh.primitive_cube_add(size=.25,location=(-2,0,0)); zone=bpy.context.object;zone.name='Zone'
    zone.data.materials.append(material)
    tree=bpy.data.node_groups.new('Simulation','GeometryNodeTree')
    tree.interface.new_socket(name='Geometry',in_out='INPUT',socket_type='NodeSocketGeometry')
    tree.interface.new_socket(name='Geometry',in_out='OUTPUT',socket_type='NodeSocketGeometry')
    i=tree.nodes.new('NodeGroupInput');o=tree.nodes.new('NodeGroupOutput')
    si=tree.nodes.new('GeometryNodeSimulationInput');so=tree.nodes.new('GeometryNodeSimulationOutput');si.pair_with_output(so)
    move=tree.nodes.new('GeometryNodeTransform');move.inputs['Translation'].default_value=(0,0,.02)
    tree.links.new(i.outputs['Geometry'],si.inputs['Geometry']);tree.links.new(si.outputs['Geometry'],move.inputs['Geometry']);tree.links.new(move.outputs['Geometry'],so.inputs['Geometry']);tree.links.new(so.outputs['Geometry'],o.inputs['Geometry'])
    zone.modifiers.new('Zone','NODES').node_group=tree

if kind.startswith('blur-'):
    carrier=kind[5:]
    s.frame_end=30
    m.point_cache.frame_start=20
    if carrier!='cloth': bpy.data.objects.remove(cloth,do_unlink=True)
    if carrier in ('soft','rigid'):
        bpy.ops.mesh.primitive_ico_sphere_add(subdivisions=1,radius=.5,location=(0,0,3))
        ob=bpy.context.object;ob.name='Carrier';ob.data.materials.append(material)
        if carrier=='rigid':
            bpy.ops.rigidbody.object_add();s.rigidbody_world.point_cache.frame_start=20;s.rigidbody_world.point_cache.frame_end=100
        else:
            mod=ob.modifiers.new('Soft','SOFT_BODY');mod.settings.use_goal=False;mod.point_cache.frame_start=20;mod.point_cache.frame_end=100
    elif carrier in ('hair','particles'):
        bpy.ops.mesh.primitive_plane_add(size=1,location=(0,0,3));ob=bpy.context.object;ob.name='Carrier';ob.data.materials.append(material)
        ob.modifiers.new('Particles','PARTICLE_SYSTEM');ps=ob.particle_systems[0];st=ps.settings
        st.count=16;st.frame_start=20;st.frame_end=20;st.lifetime=100;st.normal_factor=1
        if carrier=='hair':
            # Stage 4's moving skin gives the strands motion beyond a bake's end.
            ob.keyframe_insert('location',frame=20)
            ob.location.x=2;ob.keyframe_insert('location',frame=80)
            st.type='HAIR';st.hair_length=1;st.hair_step=3;st.display_step=st.render_step
            bpy.context.view_layer.update();ps.use_hair_dynamics=True
        ps.point_cache.frame_start=20;ps.point_cache.frame_end=100
    elif carrier in ('zone','gncloth'):
        bpy.ops.mesh.primitive_grid_add(x_subdivisions=5,y_subdivisions=5,size=1,location=(0,0,3));ob=bpy.context.object;ob.name='Carrier';ob.data.materials.append(material)
        tree=bpy.data.node_groups.new('Simulation','GeometryNodeTree')
        tree.interface.new_socket(name='Geometry',in_out='INPUT',socket_type='NodeSocketGeometry');tree.interface.new_socket(name='Geometry',in_out='OUTPUT',socket_type='NodeSocketGeometry')
        i=tree.nodes.new('NodeGroupInput');o=tree.nodes.new('NodeGroupOutput')
        if carrier=='zone':
            si=tree.nodes.new('GeometryNodeSimulationInput');so=tree.nodes.new('GeometryNodeSimulationOutput');si.pair_with_output(so)
            move=tree.nodes.new('GeometryNodeTransform');move.inputs['Translation'].default_value=(0,0,-.1)
            tree.links.new(i.outputs['Geometry'],si.inputs['Geometry']);tree.links.new(si.outputs['Geometry'],move.inputs['Geometry']);tree.links.new(move.outputs['Geometry'],so.inputs['Geometry']);tree.links.new(so.outputs['Geometry'],o.inputs['Geometry'])
        else:
            assets=os.path.join(bpy.utils.resource_path('LOCAL'),'datafiles','assets','nodes','geometry_nodes_dynamics_assets.blend')
            with bpy.data.libraries.load(assets,link=False) as (src,dst):dst.node_groups=['Cloth Dynamics (Experimental)']
            group=tree.nodes.new('GeometryNodeGroup');group.node_tree=dst.node_groups[0]
            tree.links.new(i.outputs['Geometry'],group.inputs['Geometry']);tree.links.new(group.outputs['Geometry'],o.inputs['Geometry'])
        ob.modifiers.new('Simulation','NODES').node_group=tree
    elif carrier=='liquid':
        bpy.ops.mesh.primitive_cube_add(size=4,location=(0,0,2));ob=bpy.context.object;ob.name='Carrier';ob.data.materials.append(material)
        mod=ob.modifiers.new('Fluid','FLUID');mod.fluid_type='DOMAIN';ds=mod.domain_settings;ds.domain_type='LIQUID';ds.resolution_max=16;ds.cache_type='REPLAY';ds.cache_frame_start=20;ds.cache_frame_end=100;ds.cache_directory=os.path.join(out,'fluid');ds.use_mesh=True;ds.mesh_scale=1
        bpy.ops.mesh.primitive_ico_sphere_add(subdivisions=1,radius=.6,location=(0,0,2));flow=bpy.context.object;flow.name='Flow';flow.hide_render=True
        fm=flow.modifiers.new('Flow','FLUID');fm.fluid_type='FLOW';fm.flow_settings.flow_type='LIQUID';fm.flow_settings.flow_behavior='GEOMETRY'
camdata=bpy.data.cameras.new('Camera'); cam=bpy.data.objects.new('Camera',camdata);s.collection.objects.link(cam)
cam.location=(5,-8,5);cam.rotation_euler=(Vector((0,0,1.5))-cam.location).to_track_quat('-Z','Y').to_euler();camdata.type='ORTHO';camdata.ortho_scale=6;s.camera=cam
s.frame_set(20)
if kind in ('linked','override','instance'):
    library=os.path.join(out,'library.blend')
    if kind == 'instance':
        collection=bpy.data.collections.new('ClothCollection');s.collection.children.link(collection)
        for old in list(cloth.users_collection):old.objects.unlink(cloth)
        collection.objects.link(cloth)
        bpy.data.libraries.write(library,{collection})
        bpy.data.objects.remove(cloth,do_unlink=True)
        bpy.data.collections.remove(collection)
        with bpy.data.libraries.load(library,link=True) as (src,dst): dst.collections=['ClothCollection']
        inst=bpy.data.objects.new('Instance',None); inst.instance_type='COLLECTION';inst.instance_collection=dst.collections[0];s.collection.objects.link(inst)
    else:
        bpy.data.libraries.write(library,{cloth})
        bpy.data.objects.remove(cloth,do_unlink=True)
        with bpy.data.libraries.load(library,link=True) as (src,dst): dst.objects=['Cloth']
        linked=dst.objects[0];s.collection.objects.link(linked)
        if kind == 'override':
            overridden=linked.override_create(remap_local_usages=True)
            if overridden.name not in s.collection.objects: s.collection.objects.link(overridden)
bpy.ops.wm.save_as_mainfile(filepath=os.path.join(out,'scene.blend'))
'''
runner = r'''
import bpy, sys, os, json, hashlib
out, mode, material = sys.argv[sys.argv.index('--')+1:]
s=bpy.context.scene
exec(open(material).read())
# Production Step; the withdrawn cloth keeping is never applied. The
# reference still records the known early-start cloth limit.
folder=os.path.join(out,'pointcache')
os.makedirs(folder,exist_ok=True)
doc={'scene':'Scene','instructions':[
 {'do':'RedirectAnAddedWritePlace','settings':[{'carried_by':'EverythingCarryingAPointCache'}],'folders':[{'recorded':None,'folder':list(os.fsencode(folder))}]},
 {'do':'Step','lowest':60,'highest':62}]}
frames=[60,61,62]
def digest(dg=None):
    values={}
    if dg is None: dg=bpy.context.evaluated_depsgraph_get()
    for instance in dg.object_instances:
        obj=instance.object
        if obj.type=='MESH':
            ev=obj
            points=[tuple(round(float(x),6) for x in v.co) for v in ev.data.vertices]
            values[obj.name]=hashlib.sha256(repr((points,tuple(round(float(x),6) for row in ev.matrix_world for x in row))).encode()).hexdigest()
    return values
played={}; rendered={}
def rendering(scene,dg=None):
    if dg is not None and dg.mode == 'RENDER': rendered[str(scene.frame_current)]=digest(dg)
bpy.app.handlers.frame_change_post.append(rendering)
s.render.filepath=os.path.join(out,'f_')
if mode=='reference':
    for f in range(20,63):
        s.frame_set(f)
        if f in frames:
            played[str(f)]=digest()
            bpy.ops.render.render(write_still=True)
            # Use frame names like the command line.
            os.rename(s.render.filepath+'.bmp',os.path.join(out,'f_'+str(f).zfill(4)+'.bmp'))
else:
    if mode=='old':
        # Independent old route: play, then let the command line render.
        for f in range(min(s.frame_start,60),63+smoozy_simulation_tail(s)):s.frame_set(f)
    else:smoozy_carry_out(doc,1,2)
    # Frames are rendered by the command line after this script returns.
    def keep_result(scene):
        json.dump({'played':played,'rendered':rendered,'caches':[(o.name,m.point_cache.is_baked,m.point_cache.is_frame_skip,m.point_cache.use_disk_cache) for o in bpy.data.objects for m in o.modifiers if m.type=='CLOTH']},open(os.path.join(out,'result.json'),'w'))
    bpy.app.handlers.render_post.append(keep_result)
json.dump({'played':played,'rendered':rendered,'caches':[(o.name,m.point_cache.is_baked,m.point_cache.is_frame_skip,m.point_cache.use_disk_cache) for o in bpy.data.objects for m in o.modifiers if m.type=='CLOTH']},open(os.path.join(out,'result.json'),'w'))
'''
(root/'make.py').write_text(maker)
(root/'run.py').write_text(runner)
report={}
for kind in ['early','at','after','mixed','linked','override','instance']:
    case=root/kind;case.mkdir(exist_ok=True)
    run('make-'+kind,['--python',str(root/'make.py'),'--',str(case),kind])
    original={p:p.read_bytes() for p in case.glob('*.blend')}
    results={}
    for mode in ['reference','old','production']:
        here=case/mode;here.mkdir(exist_ok=True)
        run(kind+'-'+mode,[str(case/'scene.blend'),'--python',str(root/'run.py'),*(['-f','60,61,62'] if mode!='reference' else []),'--',str(here),mode,a.material])
        results[mode]=json.loads((here/'result.json').read_text())
    expected=results['reference']['rendered']
    production=results['production']['rendered']
    assert production==results['old']['rendered'],(kind,'production changed the unpreserved route',results)
    def pixels(mode,f):
        raw=(case/mode/('f_'+str(f).zfill(4)+'.bmp')).read_bytes()
        return raw[int.from_bytes(raw[10:14],'little'):]
    assert all(pixels('old',f)==pixels('production',f) for f in [60,61,62]),(kind,'unpreserved picture differs')
    old_matches=results['old']['rendered']==expected
    if kind in ['early','mixed','linked','override','instance']:
        assert not old_matches,(kind,'fault not reproduced',results)
        assert len(set(x['Cloth'] for x in results['old']['rendered'].values()))==1,(kind,'old cloth not frozen')
    else: assert old_matches,(kind,'control changed')
    assert all(p.read_bytes()==raw for p,raw in original.items()),(kind,'saved file changed')
    report[kind]={'old_matches':old_matches,'production_matches_stepping':production==expected,'production_matches_old':True,'production_cache':results['production']['caches'],'cache_files':len(list((case/'production'/'pointcache').glob('*')))}
    (root/'report.json').write_text(json.dumps(report,indent=2))
    print(kind,report[kind],flush=True)

# Actual animation output. The render handlers read the RENDER graph and the
# video control must keep the same geometry as the frame-by-frame reference.
video_runner = runner[:runner.index("if mode=='reference':")] + r"""
if mode=='old':
    for f in range(min(s.frame_start,60),63+smoozy_simulation_tail(s)):s.frame_set(f)
else:smoozy_carry_out(doc,1,2)
s.render.image_settings.media_type='VIDEO'
s.render.image_settings.file_format='FFMPEG'
s.render.ffmpeg.format='MPEG4';s.render.ffmpeg.codec='H264'
s.frame_start=60;s.frame_end=62
s.render.filepath=os.path.join(out,'video')
def keep_result(scene):json.dump(rendered,open(os.path.join(out,'video.json'),'w'))
bpy.app.handlers.render_post.append(keep_result)
"""
(root/'video.py').write_text(video_runner)
for kind in ['early','at']:
    for mode in ['old','production']:
        here=root/kind/('video-'+mode);here.mkdir(exist_ok=True)
        run(kind+'-video-'+mode,[str(root/kind/'scene.blend'),'--python',str(root/'video.py'),'-a','--',str(here),mode,a.material])
        values=json.loads((here/'video.json').read_text())
        expected=json.loads((root/kind/'reference'/'result.json').read_text())['rendered']
        assert (values==expected)==(kind=='at'),(kind,mode,'video geometry: early-start cloth remains a limit')
        assert any(x.stat().st_size>0 for x in here.glob('*.mp4')),'video was not written'
        report[kind]['video_'+mode+'_matches']=(values==expected)
        if mode=='production':
            assert values==json.loads((root/kind/'video-old'/'video.json').read_text()),(kind,'production video changed')
(root/'report.json').write_text(json.dumps(report,indent=2))

blur_runner=r"""
import bpy,sys,os,json,hashlib
out,mode,material,position,length,enabled=sys.argv[sys.argv.index('--')+1:]
s=bpy.context.scene
exec(open(material).read())
s.render.use_motion_blur=(enabled=='on');s.render.motion_blur_position=position;s.render.motion_blur_shutter=float(length)
# Replay remains Replay; each trial has its own working folder.
for ob in bpy.data.objects:
 for m in ob.modifiers:
  if m.type=='FLUID' and m.fluid_type=='DOMAIN':m.domain_settings.cache_directory=os.path.join(out,'fluid')
folder=os.path.join(out,'pointcache');os.makedirs(folder,exist_ok=True)
doc={'scene':'Scene','instructions':[{'do':'RedirectAnAddedWritePlace','settings':[{'carried_by':'EverythingCarryingAPointCache'}],'folders':[{'recorded':None,'folder':list(os.fsencode(folder))}]},{'do':'Step','lowest':30,'highest':30}]}
saved=[s.render.use_motion_blur,s.render.motion_blur_position,s.render.motion_blur_shutter]
if mode=='old': smoozy_simulation_tail=lambda scene:0
if mode=='reference':
 import math
 tail=0 if enabled!='on' or position=='END' else math.ceil(float(length)*(0.5 if position=='CENTER' else 1))
 for f in range(20,31+tail):s.frame_set(f)
else:smoozy_carry_out(doc,1,2)
assert saved==[s.render.use_motion_blur,s.render.motion_blur_position,s.render.motion_blur_shutter]
rows=[]
def watch(sc,dg=None):
 if dg is None or dg.mode!='RENDER':return
 values={}
 for inst in dg.object_instances:
  ob=inst.object
  if ob.type!='MESH':continue
  v=[tuple(round(float(x),6) for x in vert.co) for vert in ob.data.vertices]
  v.append(tuple(round(float(x),6) for row in inst.matrix_world for x in row))
  for ps in ob.particle_systems:
   v.extend(tuple(round(float(x),6) for x in pt.location) for pt in ps.particles)
   v.extend(tuple(round(float(x),6) for x in ps.co_hair(ob,particle_no=pi,step=ki)) for pi,pt in enumerate(ps.particles) for ki in range(len(pt.hair_keys)))
  values[ob.name]=hashlib.sha256(repr(v).encode()).hexdigest()
 rows.append([sc.frame_current+sc.frame_subframe,values]);json.dump(rows,open(os.path.join(out,'samples.json'),'w'))
bpy.app.handlers.frame_change_post.append(watch)
s.render.filepath=os.path.join(out,'blur_')
"""
(root/'blur.py').write_text(blur_runner)
blur_report={}
for carrier in ['cloth','soft','rigid','zone','gncloth','hair','particles','liquid']:
    case=root/('blur-'+carrier);case.mkdir(exist_ok=True)
    run('make-blur-'+carrier,['--python',str(root/'make.py'),'--',str(case),'blur-'+carrier])
    original=(case/'scene.blend').read_bytes()
    blur_report[carrier]={}
    for position,length,enabled in [('START',2.25,'on'),('CENTER',2.25,'on'),('END',2.25,'on'),('START',2.25,'off')]:
        label=position+'-'+enabled
        got={}
        for mode in ['old','fixed','reference']:
            here=case/(label+'-'+mode);here.mkdir(exist_ok=True)
            run('blur-'+carrier+'-'+label+'-'+mode,[str(case/'scene.blend'),'--python',str(root/'blur.py'),'-f','30','--',str(here),mode,a.material,position,str(length),enabled])
            got[mode]=json.loads((here/'samples.json').read_text())
        assert got['fixed']==got['reference'],(carrier,label,'fixed samples differ',got)
        def pixels(mode):
            raw=(case/(label+'-'+mode)/'blur_0030.bmp').read_bytes();return raw[int.from_bytes(raw[10:14],'little'):]
        assert pixels('fixed')==pixels('reference'),(carrier,label,'fixed picture differs')
        if position=='END' or enabled=='off':
            assert got['old']==got['fixed'],(carrier,label,'zero-tail control changed')
            assert pixels('old')==pixels('fixed'),(carrier,label,'zero-tail picture changed')
        after={mode:[v for t,v in rows if t>30] for mode,rows in got.items()}
        blur_report[carrier][label]={'old_matches':got['old']==got['reference'],'fixed_matches':True,'picture_old_matches':pixels('old')==pixels('reference'),'picture_fixed_matches':True,'after':after}
        (root/'blur-report.json').write_text(json.dumps(blur_report,indent=2))
    assert (case/'scene.blend').read_bytes()==original,(carrier,'saved blend changed')
    print('blur',carrier,{k:v['old_matches'] for k,v in blur_report[carrier].items()},flush=True)

# The complete production reading, emitted by the ignored Rust test, must
# carry each scene's saved shutter rather than a constant or a neighbour's.
reading=root/'reading.py'
if reading.exists():
    setup=root/'read-setup.py'
    setup.write_text("import bpy\na=bpy.context.scene\na.render.use_motion_blur=True\na.render.motion_blur_position='START'\na.render.motion_blur_shutter=2.25\nb=bpy.data.scenes.new('Other')\nb.render.use_motion_blur=True\nb.render.motion_blur_position='END'\nb.render.motion_blur_shutter=4\n")
    run('saved-shutter-reading',[str(root/'at'/'scene.blend'),'--python',str(setup),'--python',str(reading)])
    raw=(root/'saved-shutter-reading.log').read_text()
    doc=json.loads(raw.split('<<<SMOOZY-READING>>>',1)[1].split('<<<END-SMOOZY-READING>>>',1)[0])
    settings=doc['settings']
    assert {x['name']:x['motion_blur'] for x in settings['scenes']}=={'Scene':{'enabled':True,'position':'START','shutter':2.25},'Other':{'enabled':True,'position':'END','shutter':4.0}},settings
    print('saved shutter reading matches both scenes',flush=True)

# Stage 4's bake-at-end control: construct separate bakes ending at 30 and
# past it. The material must leave each supplied bake untouched. Preparing
# more frames cannot extend a supplied bake; liquid was already an exception.
baked_runner=blur_runner[:blur_runner.index("saved=[")] + r'''
end=int(mode)
s.frame_end=end
caches=[]
if s.rigidbody_world:caches.append(s.rigidbody_world.point_cache)
for ob in bpy.data.objects:
 for m in ob.modifiers:
  if m.type in ('CLOTH','SOFT_BODY'):caches.append(m.point_cache)
  if m.type=='NODES':
   m.bake_target='DISK';m.bake_directory=os.path.join(out,'nodes')
   for b in m.bakes:b.use_custom_simulation_frame_range=True;b.frame_start=20;b.frame_end=end
  if m.type=='FLUID' and m.fluid_type=='DOMAIN':
   m.domain_settings.cache_type='ALL';m.domain_settings.cache_frame_start=20;m.domain_settings.cache_frame_end=end
 for ps in ob.particle_systems:
  if not any(m.type=='FLUID' for m in ob.modifiers):caches.append(ps.point_cache)
for c in caches:c.frame_start=20;c.frame_end=end
for ob in bpy.data.objects:
 for m in ob.modifiers:
  if m.type=='NODES':
   bpy.context.view_layer.objects.active=ob
   for b in m.bakes:
    result=bpy.ops.object.geometry_node_bake_single(session_uid=ob.session_uid,modifier_name=m.name,bake_id=b.bake_id)
    assert result=={'FINISHED'},result
  if m.type=='FLUID' and m.fluid_type=='DOMAIN':
   with bpy.context.temp_override(object=ob,active_object=ob):
    result=bpy.ops.fluid.bake_all();assert result=={'FINISHED'},result
    assert m.domain_settings.has_cache_baked_data and m.domain_settings.has_cache_baked_mesh
if caches:
 result=bpy.ops.ptcache.bake_all(bake=True);assert result=={'FINISHED'},result
# Hair's cache needs its own operator context on this small fixture.
for ob in bpy.data.objects:
 for ps in ob.particle_systems:
  if ps.use_hair_dynamics:
   with bpy.context.temp_override(scene=s,object=ob,active_object=ob,point_cache=ps.point_cache):
    result=bpy.ops.ptcache.bake(bake=True);assert result=={'FINISHED'},result
# These bakes are the fixture's supplied state for the production Step.
# Save it to prove neither the file nor the cache flags/ranges are changed.
fixture=os.path.join(out,'baked.blend')
bpy.ops.wm.save_as_mainfile(filepath=fixture)
original=open(fixture,'rb').read()
before=[(c.is_baked,c.frame_start,c.frame_end) for c in caches]
assert all(c.is_baked for c in caches),before
smoozy_carry_out(doc,1,2)
assert before==[(c.is_baked,c.frame_start,c.frame_end) for c in caches]
assert original==open(fixture,'rb').read()
'''+blur_runner[blur_runner.index('rows=[]'):]
(root/'baked.py').write_text(baked_runner)
baked_report={}
for carrier in ['cloth','soft','rigid','zone','gncloth','hair','particles','liquid']:
    baked_report[carrier]={}
    for end in [30,33]:
        here=root/('baked-'+carrier+'-'+str(end));here.mkdir(exist_ok=True)
        run('baked-'+carrier+'-'+str(end),[str(root/('blur-'+carrier)/'scene.blend'),'--python',str(root/'baked.py'),'-f','30','--',str(here),str(end),a.material,'START','2.25','on'])
        rows=json.loads((here/'samples.json').read_text())
        after=[v for t,v in rows if t>30]
        # Say only what this scene did, by all geometry and particle values.
        at=next(v for t,v in rows if t==30)
        baked_report[carrier][str(end)]={'moves_after':any(v!=at for v in after),'samples':rows,'bake_unchanged':True}
        (root/'baked-report.json').write_text(json.dumps(baked_report,indent=2))
    print('supplied bakes',carrier,{end:v['moves_after'] for end,v in baked_report[carrier].items()},flush=True)
