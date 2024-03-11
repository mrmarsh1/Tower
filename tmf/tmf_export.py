bl_info = {
    "name": "TMFExport",
    "description": "Exports scene to a Tower Engine mesh data format",
    "version": (1, 0),
    "blender": (4,0,0),
    "location": "File > Export",
    "category": "Import-Export"
}

import bpy
import struct
import math

def export_objects(filepath, split_angle):
    for obj in bpy.context.scene.objects:
            obj.select_set(False);

    objects = []
    for obj in bpy.data.objects:
        if obj.type == 'MESH':
            objects.append(obj)

    for original_obj in objects:
        obj = original_obj.copy()
        obj.data = original_obj.data.copy()

        bpy.context.scene.collection.objects.link(obj)        

        obj_name = None
        vertices = []
        normals = []
        uv0 = []
        indices = []

        bpy.context.view_layer.objects.active = obj

        obj.rotation_euler.x = math.radians(-90)

        bpy.context.active_object.select_set(True);
        bpy.ops.object.transform_apply(location=True, rotation=True, scale=True)
        bpy.context.active_object.select_set(False);

        bpy.ops.object.mode_set(mode='EDIT')
        bpy.ops.mesh.select_all(action='SELECT')
        # triangulate
        bpy.ops.mesh.quads_convert_to_tris(quad_method='BEAUTY')
        bpy.ops.object.mode_set(mode='OBJECT')

        edge_split_modifier = obj.modifiers.new(name="EdgeSplit", type='EDGE_SPLIT')
        edge_split_modifier.split_angle = math.radians(split_angle)
        edge_split_modifier.use_edge_sharp = True
        
        bpy.ops.object.modifier_apply(modifier=edge_split_modifier.name)

        # -----

        obj_name = original_obj.name.encode('utf-8')

        mesh = obj.data

        #mesh.calc_normals_split()

        uv_layer = None
        if mesh.uv_layers.active:
            uv_layer = mesh.uv_layers.active.data

        for vertex in mesh.vertices:
            vertices.append(vertex.co)

        for polygon in mesh.polygons:
            for loop_index in polygon.loop_indices:
                vertex_index = mesh.loops[loop_index].vertex_index
                uv0.append(uv_layer[loop_index].uv)

            for loop_index in polygon.loop_indices:
                vertex_index = mesh.loops[loop_index].vertex_index
                normals.append(mesh.vertices[vertex_index].normal)

        for polygon in mesh.polygons:
            for vertex_index in polygon.vertices:
                indices.append(vertex_index)

        with open(filepath, "wb") as f:
            f.write(struct.pack('i', len(obj_name)))
            f.write(obj_name)
            
            f.write(struct.pack('i', len(vertices)))
            for vertex in vertices:
                f.write(struct.pack('fff', *vertex))
            
            f.write(struct.pack('i', len(normals)))
            for normal in normals:
                f.write(struct.pack('fff', *normal))

            f.write(struct.pack('i', len(uv0)))
            for uv in uv0:
                f.write(struct.pack('ff', *uv))

            f.write(struct.pack('i', len(indices)))
            for index in indices:
                f.write(struct.pack('i', index))

        bpy.data.objects.remove(obj, do_unlink=True)

        break

def menu_func(self, context):
    self.layout.operator(TMFExport.bl_idname, text="Tower Mesh Format")

class TMFExport(bpy.types.Operator):
    bl_idname = "object.tmfexport"
    bl_label = "Export"
    bl_description = "Tower Engine mesh exporter"

    filepath: bpy.props.StringProperty(subtype="FILE_PATH")
    split_angle: bpy.props.IntProperty(name="Edge smooth angle", default=60)

    def execute(self, context):
        path = self.filepath
        if not path.endswith(".tmf"):
            path += ".tmf"
        export_objects(path, self.split_angle)
        return {'FINISHED'}

    def invoke(self, context, event):
        context.window_manager.fileselect_add(self)
        return {'RUNNING_MODAL'}

def register():
    bpy.utils.register_class(TMFExport)
    bpy.types.TOPBAR_MT_file_export.append(menu_func)

def unregister():
    bpy.utils.unregister_class(TMFExport)
    bpy.type.TOPBAR_MT_file_export.remove(menu_func)
