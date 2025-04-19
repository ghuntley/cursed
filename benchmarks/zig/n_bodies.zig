// N-body simulation benchmark for Zig

const std = @import("std");
const math = std.math;
const Timer = std.time.Timer;

// Constants
const PI = 3.141592653589793;
const SOLAR_MASS = 4.0 * PI * PI;
const DAYS_PER_YEAR = 365.24;

// Planet structure
const Planet = struct {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    mass: f64,
};

// Initialize the solar system
fn initSolarSystem() [5]Planet {
    return [5]Planet{
        // Sun
        Planet{
            .x = 0.0,
            .y = 0.0,
            .z = 0.0,
            .vx = 0.0,
            .vy = 0.0,
            .vz = 0.0,
            .mass = SOLAR_MASS,
        },
        // Jupiter
        Planet{
            .x = 4.84143144246472090e+00,
            .y = -1.16032004402742839e+00,
            .z = -1.03622044471123109e-01,
            .vx = 1.66007664274403694e-03 * DAYS_PER_YEAR,
            .vy = 7.69901118419740425e-03 * DAYS_PER_YEAR,
            .vz = -6.90460016972063023e-05 * DAYS_PER_YEAR,
            .mass = 9.54791938424326609e-04 * SOLAR_MASS,
        },
        // Saturn
        Planet{
            .x = 8.34336671824457987e+00,
            .y = 4.12479856412430479e+00,
            .z = -4.03523417114321381e-01,
            .vx = -2.76742510726862411e-03 * DAYS_PER_YEAR,
            .vy = 4.99852801234917238e-03 * DAYS_PER_YEAR,
            .vz = 2.30417297573763929e-05 * DAYS_PER_YEAR,
            .mass = 2.85885980666130812e-04 * SOLAR_MASS,
        },
        // Uranus
        Planet{
            .x = 1.28943695621391310e+01,
            .y = -1.51111514016986312e+01,
            .z = -2.23307578892655734e-01,
            .vx = 2.96460137564761618e-03 * DAYS_PER_YEAR,
            .vy = 2.37847173959480950e-03 * DAYS_PER_YEAR,
            .vz = -2.96589568540237556e-05 * DAYS_PER_YEAR,
            .mass = 4.36624404335156298e-05 * SOLAR_MASS,
        },
        // Neptune
        Planet{
            .x = 1.53796971148509165e+01,
            .y = -2.59193146099879641e+01,
            .z = 1.79258772950371181e-01,
            .vx = 2.68067772490389322e-03 * DAYS_PER_YEAR,
            .vy = 1.62824170038242295e-03 * DAYS_PER_YEAR,
            .vz = -9.51592254519715870e-05 * DAYS_PER_YEAR,
            .mass = 5.15138902046611451e-05 * SOLAR_MASS,
        },
    };
}

// Offset the momentum of the system
fn offsetMomentum(bodies: *[5]Planet) void {
    var px: f64 = 0.0;
    var py: f64 = 0.0;
    var pz: f64 = 0.0;
    
    for (bodies) |body| {
        px += body.vx * body.mass;
        py += body.vy * body.mass;
        pz += body.vz * body.mass;
    }
    
    bodies[0].vx = -px / SOLAR_MASS;
    bodies[0].vy = -py / SOLAR_MASS;
    bodies[0].vz = -pz / SOLAR_MASS;
}

// Calculate energy of the system
fn energy(bodies: *const [5]Planet) f64 {
    var e: f64 = 0.0;
    
    for (bodies, 0..) |b, i| {
        // Kinetic energy
        e += 0.5 * b.mass * (b.vx * b.vx + b.vy * b.vy + b.vz * b.vz);
        
        // Potential energy with other bodies
        var j: usize = i + 1;
        while (j < bodies.len) : (j += 1) {
            const b2 = bodies[j];
            const dx = b.x - b2.x;
            const dy = b.y - b2.y;
            const dz = b.z - b2.z;
            const distance = math.sqrt(dx*dx + dy*dy + dz*dz);
            e -= (b.mass * b2.mass) / distance;
        }
    }
    
    return e;
}

// Advance the system by dt
fn advance(bodies: *[5]Planet, dt: f64) void {
    // Update velocities based on interactions
    for (bodies, 0..) |*b, i| {
        var j: usize = i + 1;
        while (j < bodies.len) : (j += 1) {
            const b2 = &bodies[j];
            const dx = b.x - b2.x;
            const dy = b.y - b2.y;
            const dz = b.z - b2.z;
            
            const distance = math.sqrt(dx*dx + dy*dy + dz*dz);
            const mag = dt / (distance * distance * distance);
            
            const b_mass_mag = b.mass * mag;
            const b2_mass_mag = b2.mass * mag;
            
            b.vx -= dx * b2_mass_mag;
            b.vy -= dy * b2_mass_mag;
            b.vz -= dz * b2_mass_mag;
            
            b2.vx += dx * b_mass_mag;
            b2.vy += dy * b_mass_mag;
            b2.vz += dz * b_mass_mag;
        }
    }
    
    // Update positions
    for (bodies) |*body| {
        body.x += dt * body.vx;
        body.y += dt * body.vy;
        body.z += dt * body.vz;
    }
}

pub fn main() !void {
    const n = 1000000;  // Number of iterations
    var bodies = initSolarSystem();
    
    var timer = try Timer.start();
    const start_time = timer.lap();
    
    offsetMomentum(&bodies);
    const initial_energy = energy(&bodies);
    try std.io.getStdOut().writer().print("Initial energy: {d:.9}\n", .{initial_energy});
    
    // Run simulation
    var i: u32 = 0;
    while (i < n) : (i += 1) {
        advance(&bodies, 0.01);
    }
    
    const final_energy = energy(&bodies);
    try std.io.getStdOut().writer().print("Final energy: {d:.9}\n", .{final_energy});
    try std.io.getStdOut().writer().print("Energy delta: {d:.9}\n", .{final_energy - initial_energy});
    
    const end_time = timer.lap();
    const elapsed = @intToFloat(f64, end_time - start_time) / std.time.ns_per_ms;
    try std.io.getStdOut().writer().print("Time taken: {d:.2} ms\n", .{elapsed});
}