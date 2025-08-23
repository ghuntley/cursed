yeet "vibez"
yeet "renderz"
yeet "mathz"

vibez.spill("=== Simple Graphics Test ===")

fr fr Test 3D vertex generation
sus vertices [1000]renderz.Vertex
sus indices [3000]normie

vibez.spill("Testing cube vertex generation...")
sus success lit = renderz.renderz_generate_cube_vertices(vertices, indices)

ready (success) {
    vibez.spill("✓ Cube vertices generated successfully!")
    vibez.spill("First vertex position:", vertices[0].position.x, vertices[0].position.y, vertices[0].position.z)
    vibez.spill("First vertex normal:", vertices[0].normal.x, vertices[0].normal.y, vertices[0].normal.z)
    vibez.spill("First vertex texcoord:", vertices[0].tex_coord.x, vertices[0].tex_coord.y)
} otherwise {
    vibez.spill("✗ Failed to generate cube vertices")
}

vibez.spill("\nTesting sphere vertex generation...")
success = renderz.renderz_generate_sphere_vertices(vertices, indices, 1.0, 8, 8)

ready (success) {
    vibez.spill("✓ Sphere vertices generated successfully!")
    vibez.spill("First vertex position:", vertices[0].position.x, vertices[0].position.y, vertices[0].position.z)
    vibez.spill("First vertex normal:", vertices[0].normal.x, vertices[0].normal.y, vertices[0].normal.z)
} otherwise {
    vibez.spill("✗ Failed to generate sphere vertices")
}

vibez.spill("\nTesting matrix operations...")
sus eye renderz.Vec3 = {x: 0.0, y: 0.0, z: 5.0}
sus target renderz.Vec3 = {x: 0.0, y: 0.0, z: 0.0}
sus up renderz.Vec3 = {x: 0.0, y: 1.0, z: 0.0}

sus view_matrix renderz.Mat4 = renderz.renderz_look_at_matrix(eye, target, up)
vibez.spill("✓ Look-at matrix created")
vibez.spill("Matrix elements [0-3]:", view_matrix.m[0], view_matrix.m[1], view_matrix.m[2], view_matrix.m[3])

sus proj_matrix renderz.Mat4 = renderz.renderz_perspective_matrix(45.0, 1.333, 0.1, 100.0)
vibez.spill("✓ Perspective matrix created")
vibez.spill("Matrix elements [0,5,10]:", proj_matrix.m[0], proj_matrix.m[5], proj_matrix.m[10])

vibez.spill("\n=== Graphics system is working! ===")
