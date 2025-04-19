// N-body simulation benchmark adapted from The Computer Language Benchmarks Game

use std::time::Instant;

const PI: f64 = 3.141592653589793;
const SOLAR_MASS: f64 = 4.0 * PI * PI;
const DAYS_PER_YEAR: f64 = 365.24;

// Planet structure
struct Planet {
    x: f64, y: f64, z: f64,
    vx: f64, vy: f64, vz: f64,
    mass: f64,
}

// Initialize solar system
fn init_solar_system() -> Vec<Planet> {
    let mut bodies = Vec::with_capacity(5);
    
    // Sun
    bodies.push(Planet {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
        mass: SOLAR_MASS,
    });
    
    // Jupiter
    bodies.push(Planet {
        x: 4.84143144246472090e+00,
        y: -1.16032004402742839e+00,
        z: -1.03622044471123109e-01,
        vx: 1.66007664274403694e-03 * DAYS_PER_YEAR,
        vy: 7.69901118419740425e-03 * DAYS_PER_YEAR,
        vz: -6.90460016972063023e-05 * DAYS_PER_YEAR,
        mass: 9.54791938424326609e-04 * SOLAR_MASS,
    });
    
    // Saturn
    bodies.push(Planet {
        x: 8.34336671824457987e+00,
        y: 4.12479856412430479e+00,
        z: -4.03523417114321381e-01,
        vx: -2.76742510726862411e-03 * DAYS_PER_YEAR,
        vy: 4.99852801234917238e-03 * DAYS_PER_YEAR,
        vz: 2.30417297573763929e-05 * DAYS_PER_YEAR,
        mass: 2.85885980666130812e-04 * SOLAR_MASS,
    });
    
    // Uranus
    bodies.push(Planet {
        x: 1.28943695621391310e+01,
        y: -1.51111514016986312e+01,
        z: -2.23307578892655734e-01,
        vx: 2.96460137564761618e-03 * DAYS_PER_YEAR,
        vy: 2.37847173959480950e-03 * DAYS_PER_YEAR,
        vz: -2.96589568540237556e-05 * DAYS_PER_YEAR,
        mass: 4.36624404335156298e-05 * SOLAR_MASS,
    });
    
    // Neptune
    bodies.push(Planet {
        x: 1.53796971148509165e+01,
        y: -2.59193146099879641e+01,
        z: 1.79258772950371181e-01,
        vx: 2.68067772490389322e-03 * DAYS_PER_YEAR,
        vy: 1.62824170038242295e-03 * DAYS_PER_YEAR,
        vz: -9.51592254519715870e-05 * DAYS_PER_YEAR,
        mass: 5.15138902046611451e-05 * SOLAR_MASS,
    });
    
    bodies
}

// Offset momentum of the sun
fn offset_momentum(bodies: &mut [Planet]) {
    let mut px = 0.0;
    let mut py = 0.0;
    let mut pz = 0.0;
    
    for body in bodies.iter() {
        px += body.vx * body.mass;
        py += body.vy * body.mass;
        pz += body.vz * body.mass;
    }
    
    bodies[0].vx = -px / SOLAR_MASS;
    bodies[0].vy = -py / SOLAR_MASS;
    bodies[0].vz = -pz / SOLAR_MASS;
}

// Calculate energy of the system
fn energy(bodies: &[Planet]) -> f64 {
    let mut e = 0.0;
    
    for i in 0..bodies.len() {
        let b = &bodies[i];
        e += 0.5 * b.mass * (b.vx*b.vx + b.vy*b.vy + b.vz*b.vz);
        
        for j in (i+1)..bodies.len() {
            let b2 = &bodies[j];
            let dx = b.x - b2.x;
            let dy = b.y - b2.y;
            let dz = b.z - b2.z;
            let distance = (dx*dx + dy*dy + dz*dz).sqrt();
            e -= (b.mass * b2.mass) / distance;
        }
    }
    
    e
}

// Advance simulation by dt
fn advance(bodies: &mut [Planet], dt: f64) {
    for i in 0..bodies.len() {
        for j in (i+1)..bodies.len() {
            let dx = bodies[i].x - bodies[j].x;
            let dy = bodies[i].y - bodies[j].y;
            let dz = bodies[i].z - bodies[j].z;
            
            let distance = (dx*dx + dy*dy + dz*dz).sqrt();
            let mag = dt / (distance * distance * distance);
            
            let b_mass_mag = bodies[i].mass * mag;
            let b2_mass_mag = bodies[j].mass * mag;
            
            bodies[i].vx -= dx * b2_mass_mag;
            bodies[i].vy -= dy * b2_mass_mag;
            bodies[i].vz -= dz * b2_mass_mag;
            
            bodies[j].vx += dx * b_mass_mag;
            bodies[j].vy += dy * b_mass_mag;
            bodies[j].vz += dz * b_mass_mag;
        }
    }
    
    for body in bodies.iter_mut() {
        body.x += dt * body.vx;
        body.y += dt * body.vy;
        body.z += dt * body.vz;
    }
}

fn main() {
    let n = 1_000_000; // Number of iterations
    let mut bodies = init_solar_system();
    let start_time = Instant::now();
    
    offset_momentum(&mut bodies);
    let initial_energy = energy(&bodies);
    println!("Initial energy: {:.9}", initial_energy);
    
    for _ in 0..n {
        advance(&mut bodies, 0.01);
    }
    
    let final_energy = energy(&bodies);
    println!("Final energy: {:.9}", final_energy);
    println!("Energy delta: {:.9}", final_energy - initial_energy);
    
    let elapsed = start_time.elapsed();
    println!("Time taken: {} ms", elapsed.as_millis());
    
    // Get approximate memory usage
    let memory_usage = std::mem::size_of::<Planet>() * bodies.len() / 1024;
    println!("Memory used: {} KB", memory_usage);
}