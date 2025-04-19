// N-body simulation benchmark adapted from The Computer Language Benchmarks Game

const PI = 3.141592653589793;
const SOLAR_MASS = 4.0 * PI * PI;
const DAYS_PER_YEAR = 365.24;

class Planet {
    constructor(x, y, z, vx, vy, vz, mass) {
        this.x = x;
        this.y = y;
        this.z = z;
        this.vx = vx;
        this.vy = vy;
        this.vz = vz;
        this.mass = mass;
    }
}

// Initialize solar system
function initSolarSystem() {
    const bodies = [];
    
    // Sun
    bodies.push(new Planet(
        0.0, 0.0, 0.0,
        0.0, 0.0, 0.0,
        SOLAR_MASS
    ));
    
    // Jupiter
    bodies.push(new Planet(
        4.84143144246472090e+00,
        -1.16032004402742839e+00,
        -1.03622044471123109e-01,
        1.66007664274403694e-03 * DAYS_PER_YEAR,
        7.69901118419740425e-03 * DAYS_PER_YEAR,
        -6.90460016972063023e-05 * DAYS_PER_YEAR,
        9.54791938424326609e-04 * SOLAR_MASS
    ));
    
    // Saturn
    bodies.push(new Planet(
        8.34336671824457987e+00,
        4.12479856412430479e+00,
        -4.03523417114321381e-01,
        -2.76742510726862411e-03 * DAYS_PER_YEAR,
        4.99852801234917238e-03 * DAYS_PER_YEAR,
        2.30417297573763929e-05 * DAYS_PER_YEAR,
        2.85885980666130812e-04 * SOLAR_MASS
    ));
    
    // Uranus
    bodies.push(new Planet(
        1.28943695621391310e+01,
        -1.51111514016986312e+01,
        -2.23307578892655734e-01,
        2.96460137564761618e-03 * DAYS_PER_YEAR,
        2.37847173959480950e-03 * DAYS_PER_YEAR,
        -2.96589568540237556e-05 * DAYS_PER_YEAR,
        4.36624404335156298e-05 * SOLAR_MASS
    ));
    
    // Neptune
    bodies.push(new Planet(
        1.53796971148509165e+01,
        -2.59193146099879641e+01,
        1.79258772950371181e-01,
        2.68067772490389322e-03 * DAYS_PER_YEAR,
        1.62824170038242295e-03 * DAYS_PER_YEAR,
        -9.51592254519715870e-05 * DAYS_PER_YEAR,
        5.15138902046611451e-05 * SOLAR_MASS
    ));
    
    return bodies;
}

// Offset momentum of the sun
function offsetMomentum(bodies) {
    let px = 0.0, py = 0.0, pz = 0.0;
    
    for (const body of bodies) {
        px += body.vx * body.mass;
        py += body.vy * body.mass;
        pz += body.vz * body.mass;
    }
    
    bodies[0].vx = -px / SOLAR_MASS;
    bodies[0].vy = -py / SOLAR_MASS;
    bodies[0].vz = -pz / SOLAR_MASS;
}

// Calculate energy of the system
function energy(bodies) {
    let e = 0.0;
    
    for (let i = 0; i < bodies.length; i++) {
        const b = bodies[i];
        e += 0.5 * b.mass * (b.vx*b.vx + b.vy*b.vy + b.vz*b.vz);
        
        for (let j = i + 1; j < bodies.length; j++) {
            const b2 = bodies[j];
            const dx = b.x - b2.x;
            const dy = b.y - b2.y;
            const dz = b.z - b2.z;
            const distance = Math.sqrt(dx*dx + dy*dy + dz*dz);
            e -= (b.mass * b2.mass) / distance;
        }
    }
    
    return e;
}

// Advance simulation by dt
function advance(bodies, dt) {
    for (let i = 0; i < bodies.length; i++) {
        const b = bodies[i];
        
        for (let j = i + 1; j < bodies.length; j++) {
            const b2 = bodies[j];
            const dx = b.x - b2.x;
            const dy = b.y - b2.y;
            const dz = b.z - b2.z;
            
            const distance = Math.sqrt(dx*dx + dy*dy + dz*dz);
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
    
    for (const b of bodies) {
        b.x += dt * b.vx;
        b.y += dt * b.vy;
        b.z += dt * b.vz;
    }
}

function main() {
    const n = 1000000; // Number of iterations
    const bodies = initSolarSystem();
    const startTime = Date.now();
    
    offsetMomentum(bodies);
    const initialEnergy = energy(bodies);
    console.log(`Initial energy: ${initialEnergy.toFixed(9)}`);
    
    for (let i = 0; i < n; i++) {
        advance(bodies, 0.01);
    }
    
    const finalEnergy = energy(bodies);
    console.log(`Final energy: ${finalEnergy.toFixed(9)}`);
    console.log(`Energy delta: ${(finalEnergy - initialEnergy).toFixed(9)}`);
    
    const elapsed = Date.now() - startTime;
    console.log(`Time taken: ${elapsed} ms`);
    
    // Get memory stats
    const memoryUsageInMB = process.memoryUsage().heapUsed / 1024 / 1024;
    console.log(`Memory used: ${Math.round(memoryUsageInMB * 1024)} KB`);
}

main();