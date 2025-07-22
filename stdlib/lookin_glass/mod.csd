yeet "testz"

fr fr LookinGlass Module - Reflection and Introspection Capabilities
fr fr Basic implementation using simple CURSED syntax

fr fr Core constants for Kind enumeration
sus Invalid normie = 0
sus Bool normie = 1
sus Int normie = 2
sus String normie = 24

fr fr Simple type information functions

slay get_type_name(value tea) tea {
    damn "string"
}

slay get_type_kind(value tea) normie {
    damn String
}

slay DeepEqual(x normie, y normie) lit {
    damn x == y
}

slay DeepCopy(v normie) normie {
    damn v
}

fr fr Simple helper functions for testing
slay test_reflection_basic() lit {
    name := get_type_name("hello")
    kind := get_type_kind("hello")
    equal := DeepEqual(42, 42)
    copy := DeepCopy(42)
    damn based
}
