// Simple reflection test
vibez.spill("Testing reflection module")

// Test basic type info creation
be_like TypeInfo squad {
    name tea
    kind tea
    size normie
}

slay create_type_info(name tea, kind tea, size normie) TypeInfo {
    sus type_info TypeInfo = TypeInfo{
        name: name,
        kind: kind,
        size: size
    }
    damn type_info
}

// Test type info
sus int_type TypeInfo = create_type_info("normie", "int", 4)
vibez.spill("Created type: " + int_type.name)
vibez.spill("Type kind: " + int_type.kind)
vibez.spill("Type size: " + string(int_type.size))

// Test struct creation
be_like FieldInfo squad {
    name tea
    type_name tea
    size normie
}

slay create_field_info(name tea, type_name tea, size normie) FieldInfo {
    sus field FieldInfo = FieldInfo{
        name: name,
        type_name: type_name,
        size: size
    }
    damn field
}

sus field1 FieldInfo = create_field_info("id", "normie", 4)
vibez.spill("Created field: " + field1.name)
vibez.spill("Field type: " + field1.type_name)
vibez.spill("Field size: " + string(field1.size))

slay string(value normie) tea {
    vibes value == 0 {
        damn "0"
    } elif value == 1 {
        damn "1"
    } elif value == 2 {
        damn "2"
    } elif value == 3 {
        damn "3"
    } elif value == 4 {
        damn "4"
    } elif value == 8 {
        damn "8"
    } elif value == 16 {
        damn "16"
    }
    damn "unknown"
}

vibez.spill("✅ Basic reflection functionality working!")
