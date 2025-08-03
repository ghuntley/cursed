#!/bin/bash

# Test compilation speed performance

set -e

# Create test programs of different sizes
cat > small_program.csd << 'EOF'
vibez.spill("Small program")
EOF

cat > medium_program.csd << 'EOF'
fr fr Medium-sized program
squad Point {
    spill x meal
    spill y meal
}

slay distance(p1 Point, p2 Point) meal {
    sus dx meal = p1.x - p2.x
    sus dy meal = p1.y - p2.y
    damn dx * dx + dy * dy
}

sus points []Point = []
bestie i := 0; i < 100; i = i + 1 {
    points.push(Point{x: i.(meal), y: (i * 2).(meal)})
}

bestie i := 0; i < points.len() - 1; i = i + 1 {
    sus dist meal = distance(points[i], points[i + 1])
    vibez.spill("Distance:", dist)
}
EOF

cat > large_program.csd << 'EOF'
fr fr Large program with many functions and structs

squad Vector3 {
    spill x meal
    spill y meal
    spill z meal
}

squad Matrix3x3 {
    spill m00 meal
    spill m01 meal
    spill m02 meal
    spill m10 meal
    spill m11 meal
    spill m12 meal
    spill m20 meal
    spill m21 meal
    spill m22 meal
}

slay vector_add(a Vector3, b Vector3) Vector3 {
    damn Vector3{x: a.x + b.x, y: a.y + b.y, z: a.z + b.z}
}

slay vector_multiply(v Vector3, scalar meal) Vector3 {
    damn Vector3{x: v.x * scalar, y: v.y * scalar, z: v.z * scalar}
}

slay vector_magnitude(v Vector3) meal {
    damn (v.x * v.x + v.y * v.y + v.z * v.z)
}

slay matrix_multiply_vector(m Matrix3x3, v Vector3) Vector3 {
    damn Vector3{
        x: m.m00 * v.x + m.m01 * v.y + m.m02 * v.z,
        y: m.m10 * v.x + m.m11 * v.y + m.m12 * v.z,
        z: m.m20 * v.x + m.m21 * v.y + m.m22 * v.z
    }
}

slay complex_calculation() {
    sus vectors []Vector3 = []
    
    bestie i := 0; i < 1000; i = i + 1 {
        vectors.push(Vector3{
            x: i.(meal),
            y: (i * 2).(meal),
            z: (i * 3).(meal)
        })
    }
    
    sus identity Matrix3x3 = Matrix3x3{
        m00: 1.0, m01: 0.0, m02: 0.0,
        m10: 0.0, m11: 1.0, m12: 0.0,
        m20: 0.0, m21: 0.0, m22: 1.0
    }
    
    bestie i := 0; i < vectors.len(); i = i + 1 {
        sus transformed Vector3 = matrix_multiply_vector(identity, vectors[i])
        sus magnitude meal = vector_magnitude(transformed)
        
        bestie magnitude > 100.0 {
            vibez.spill("Large vector magnitude:", magnitude)
        }
    }
}

complex_calculation()
vibez.spill("Complex calculation completed")
EOF

# Test compilation speed for different program sizes
echo "Testing compilation speed..."

echo "Small program compilation:"
time ./cursed-unified --compile small_program.csd
rm -f small_program

echo "Medium program compilation:"
time ./cursed-unified --compile medium_program.csd
rm -f medium_program

echo "Large program compilation:"
time ./cursed-unified --compile large_program.csd
rm -f large_program

# Cleanup
rm -f small_program.csd medium_program.csd large_program.csd

exit 0
