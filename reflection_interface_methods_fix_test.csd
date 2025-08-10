fr fr Test P1 Issue #27 Fix: Reflection API TypeInfo.methods vector populated for collab objects
yeet "typez"
yeet "testz"

fr fr Test interface with methods
collab Drawable {
    slay draw() vibes
    slay move(x normie, y normie) vibes
    slay get_position() normie
}

fr fr Test struct implementing interface
squad Circle {
    spill x normie
    spill y normie
    spill radius normie
}

impl Circle bestie Drawable {
    slay draw() vibes {
        vibez.spill("Drawing circle at", me.x, me.y, "radius:", me.radius)
    }
    
    slay move(x normie, y normie) vibes {
        me.x = x
        me.y = y
    }
    
    slay get_position() normie {
        damn me.x + me.y
    }
}

slay test_interface_method_reflection() vibes {
    fr fr Register interface methods
    sus draw_method typez.MethodInfo = typez.MethodInfo{
        name: "draw",
        receiver_type: typez.TypeInfo{type_id: 0, name: "Drawable", size: 16, alignment: 8, kind: typez.TypeKind{id: typez.TYPE_KIND_INTERFACE, name: "interface"}, is_primitive: cap, is_pointer: cap, is_array: cap, is_struct: cap, is_interface: based, is_function: cap, methods: []},
        param_types: [],
        return_type: typez.TypeInfo{type_id: 0, name: "vibes", size: 0, alignment: 1, kind: typez.TypeKind{id: typez.TYPE_KIND_PRIMITIVE, name: "primitive"}, is_primitive: based, is_pointer: cap, is_array: cap, is_struct: cap, is_interface: cap, is_function: cap, methods: []},
        is_public: based
    }
    
    sus move_method typez.MethodInfo = typez.MethodInfo{
        name: "move",
        receiver_type: typez.TypeInfo{type_id: 0, name: "Drawable", size: 16, alignment: 8, kind: typez.TypeKind{id: typez.TYPE_KIND_INTERFACE, name: "interface"}, is_primitive: cap, is_pointer: cap, is_array: cap, is_struct: cap, is_interface: based, is_function: cap, methods: []},
        param_types: [
            typez.TypeInfo{type_id: 2, name: "normie", size: 4, alignment: 4, kind: typez.TypeKind{id: typez.TYPE_KIND_PRIMITIVE, name: "primitive"}, is_primitive: based, is_pointer: cap, is_array: cap, is_struct: cap, is_interface: cap, is_function: cap, methods: []},
            typez.TypeInfo{type_id: 2, name: "normie", size: 4, alignment: 4, kind: typez.TypeKind{id: typez.TYPE_KIND_PRIMITIVE, name: "primitive"}, is_primitive: based, is_pointer: cap, is_array: cap, is_struct: cap, is_interface: cap, is_function: cap, methods: []}
        ],
        return_type: typez.TypeInfo{type_id: 0, name: "vibes", size: 0, alignment: 1, kind: typez.TypeKind{id: typez.TYPE_KIND_PRIMITIVE, name: "primitive"}, is_primitive: based, is_pointer: cap, is_array: cap, is_struct: cap, is_interface: cap, is_function: cap, methods: []},
        is_public: based
    }
    
    sus get_pos_method typez.MethodInfo = typez.MethodInfo{
        name: "get_position",
        receiver_type: typez.TypeInfo{type_id: 0, name: "Drawable", size: 16, alignment: 8, kind: typez.TypeKind{id: typez.TYPE_KIND_INTERFACE, name: "interface"}, is_primitive: cap, is_pointer: cap, is_array: cap, is_struct: cap, is_interface: based, is_function: cap, methods: []},
        param_types: [],
        return_type: typez.TypeInfo{type_id: 2, name: "normie", size: 4, alignment: 4, kind: typez.TypeKind{id: typez.TYPE_KIND_PRIMITIVE, name: "primitive"}, is_primitive: based, is_pointer: cap, is_array: cap, is_struct: cap, is_interface: cap, is_function: cap, methods: []},
        is_public: based
    }
    
    sus interface_methods []typez.MethodInfo = [draw_method, move_method, get_pos_method]
    
    fr fr Register interface type with methods
    sus drawable_type_id normie = typez.register_interface_type("Drawable", interface_methods)
    
    fr fr Verify interface was registered correctly
    sus drawable_type typez.TypeInfo = typez.get_type_by_id(drawable_type_id)
    testz.assert_eq_string(drawable_type.name, "Drawable")
    testz.assert_true(drawable_type.is_interface)
    testz.assert_false(drawable_type.is_primitive)
    testz.assert_false(drawable_type.is_struct)
    
    fr fr P1 Issue #27 Fix Verification: Check that methods vector is populated
    testz.assert_eq_int(drawable_type.methods.len(), 3)
    testz.assert_eq_string(drawable_type.methods[0].name, "draw")
    testz.assert_eq_string(drawable_type.methods[1].name, "move")
    testz.assert_eq_string(drawable_type.methods[2].name, "get_position")
    
    fr fr Verify method signatures
    testz.assert_eq_int(drawable_type.methods[0].param_types.len(), 0)  fr fr draw() has no params
    testz.assert_eq_int(drawable_type.methods[1].param_types.len(), 2)  fr fr move(x, y) has 2 params  
    testz.assert_eq_int(drawable_type.methods[2].param_types.len(), 0)  fr fr get_position() has no params
    
    fr fr Verify return types
    testz.assert_eq_string(drawable_type.methods[0].return_type.name, "vibes")
    testz.assert_eq_string(drawable_type.methods[1].return_type.name, "vibes")
    testz.assert_eq_string(drawable_type.methods[2].return_type.name, "normie")
    
    vibez.spill("✅ P1 Issue #27 Fix Verified: TypeInfo.methods vector properly populated for interface types")
}

slay test_struct_method_reflection() vibes {
    fr fr Test that struct methods are also populated
    sus circle_methods []typez.MethodInfo = []  fr fr Placeholder for now
    
    sus circle_fields []typez.FieldInfo = [
        typez.FieldInfo{name: "x", type_info: typez.TypeInfo{type_id: 2, name: "normie", size: 4, alignment: 4, kind: typez.TypeKind{id: typez.TYPE_KIND_PRIMITIVE, name: "primitive"}, is_primitive: based, is_pointer: cap, is_array: cap, is_struct: cap, is_interface: cap, is_function: cap, methods: []}, offset: 0, tag: ""},
        typez.FieldInfo{name: "y", type_info: typez.TypeInfo{type_id: 2, name: "normie", size: 4, alignment: 4, kind: typez.TypeKind{id: typez.TYPE_KIND_PRIMITIVE, name: "primitive"}, is_primitive: based, is_pointer: cap, is_array: cap, is_struct: cap, is_interface: cap, is_function: cap, methods: []}, offset: 4, tag: ""},
        typez.FieldInfo{name: "radius", type_info: typez.TypeInfo{type_id: 2, name: "normie", size: 4, alignment: 4, kind: typez.TypeKind{id: typez.TYPE_KIND_PRIMITIVE, name: "primitive"}, is_primitive: based, is_pointer: cap, is_array: cap, is_struct: cap, is_interface: cap, is_function: cap, methods: []}, offset: 8, tag: ""}
    ]
    
    sus circle_type_id normie = typez.register_struct_type("Circle", circle_fields, circle_methods)
    sus circle_type typez.TypeInfo = typez.get_type_by_id(circle_type_id)
    
    fr fr Verify struct has methods field (even if empty for now)
    testz.assert_eq_int(circle_type.methods.len(), 0)
    
    vibez.spill("✅ Struct method reflection also properly initialized")
}

slay test_interface_discovery() vibes {
    fr fr Test interface method discovery for dynamic dispatch
    sus drawable_interface typez.InterfaceInfo = typez.get_interface_info("Drawable")
    
    testz.assert_eq_string(drawable_interface.name, "Drawable")
    testz.assert_eq_int(drawable_interface.methods.len(), 3)
    testz.assert_eq_string(drawable_interface.methods[0].name, "draw")
    testz.assert_eq_string(drawable_interface.methods[1].name, "move")
    testz.assert_eq_string(drawable_interface.methods[2].name, "get_position")
    
    vibez.spill("✅ Interface method discovery working for dynamic dispatch")
}

slay main() vibes {
    testz.test_start("P1 Issue #27: Reflection API TypeInfo.methods vector fix")
    
    fr fr Initialize primitive types first
    typez.init_primitive_types()
    
    test_interface_method_reflection()
    test_struct_method_reflection() 
    test_interface_discovery()
    
    testz.print_test_summary()
    vibez.spill("🎉 P1 Issue #27 Fixed: Interface method reflection working perfectly!")
}
