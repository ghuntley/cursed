// N-body simulation benchmark for Swift

import Foundation

// Constants
let PI = 3.141592653589793
let SOLAR_MASS = 4.0 * PI * PI
let DAYS_PER_YEAR = 365.24

// Planet class
class Planet {
    var x, y, z: Double
    var vx, vy, vz: Double
    var mass: Double
    
    init(x: Double, y: Double, z: Double, vx: Double, vy: Double, vz: Double, mass: Double) {
        self.x = x
        self.y = y
        self.z = z
        self.vx = vx
        self.vy = vy
        self.vz = vz
        self.mass = mass
    }
}

// Initialize the solar system
func initSolarSystem() -> [Planet] {
    return [
        // Sun
        Planet(
            x: 0.0, y: 0.0, z: 0.0,
            vx: 0.0, vy: 0.0, vz: 0.0,
            mass: SOLAR_MASS
        ),
        
        // Jupiter
        Planet(
            x: 4.84143144246472090e+00,
            y: -1.16032004402742839e+00,
            z: -1.03622044471123109e-01,
            vx: 1.66007664274403694e-03 * DAYS_PER_YEAR,
            vy: 7.69901118419740425e-03 * DAYS_PER_YEAR,
            vz: -6.90460016972063023e-05 * DAYS_PER_YEAR,
            mass: 9.54791938424326609e-04 * SOLAR_MASS
        ),
        
        // Saturn
        Planet(
            x: 8.34336671824457987e+00,
            y: 4.12479856412430479e+00,
            z: -4.03523417114321381e-01,
            vx: -2.76742510726862411e-03 * DAYS_PER_YEAR,
            vy: 4.99852801234917238e-03 * DAYS_PER_YEAR,
            vz: 2.30417297573763929e-05 * DAYS_PER_YEAR,
            mass: 2.85885980666130812e-04 * SOLAR_MASS
        ),
        
        // Uranus
        Planet(
            x: 1.28943695621391310e+01,
            y: -1.51111514016986312e+01,
            z: -2.23307578892655734e-01,
            vx: 2.96460137564761618e-03 * DAYS_PER_YEAR,
            vy: 2.37847173959480950e-03 * DAYS_PER_YEAR,
            vz: -2.96589568540237556e-05 * DAYS_PER_YEAR,
            mass: 4.36624404335156298e-05 * SOLAR_MASS
        ),
        
        // Neptune
        Planet(
            x: 1.53796971148509165e+01,
            y: -2.59193146099879641e+01,
            z: 1.79258772950371181e-01,
            vx: 2.68067772490389322e-03 * DAYS_PER_YEAR,
            vy: 1.62824170038242295e-03 * DAYS_PER_YEAR,
            vz: -9.51592254519715870e-05 * DAYS_PER_YEAR,
            mass: 5.15138902046611451e-05 * SOLAR_MASS
        )
    ]
}

// Offset the momentum of the system
func offsetMomentum(_ bodies: [Planet]) {
    var px: Double = 0.0
    var py: Double = 0.0
    var pz: Double = 0.0
    
    for body in bodies {
        px += body.vx * body.mass
        py += body.vy * body.mass
        pz += body.vz * body.mass
    }
    
    bodies[0].vx = -px / SOLAR_MASS
    bodies[0].vy = -py / SOLAR_MASS
    bodies[0].vz = -pz / SOLAR_MASS
}

// Calculate energy of the system
func energy(_ bodies: [Planet]) -> Double {
    var e: Double = 0.0
    
    for i in 0..<bodies.count {
        let body = bodies[i]
        // Kinetic energy
        e += 0.5 * body.mass * (body.vx * body.vx + body.vy * body.vy + body.vz * body.vz)
        
        // Potential energy
        for j in (i+1)..<bodies.count {
            let bodyJ = bodies[j]
            let dx = body.x - bodyJ.x
            let dy = body.y - bodyJ.y
            let dz = body.z - bodyJ.z
            let distance = sqrt(dx*dx + dy*dy + dz*dz)
            e -= (body.mass * bodyJ.mass) / distance
        }
    }
    
    return e
}

// Advance the system by dt
func advance(_ bodies: [Planet], dt: Double) {
    // Update velocities based on interactions
    for i in 0..<bodies.count {
        let body = bodies[i]
        
        for j in (i+1)..<bodies.count {
            let bodyJ = bodies[j]
            let dx = body.x - bodyJ.x
            let dy = body.y - bodyJ.y
            let dz = body.z - bodyJ.z
            
            let distance = sqrt(dx*dx + dy*dy + dz*dz)
            let mag = dt / (distance * distance * distance)
            
            let bodyMassMag = body.mass * mag
            let bodyJMassMag = bodyJ.mass * mag
            
            body.vx -= dx * bodyJMassMag
            body.vy -= dy * bodyJMassMag
            body.vz -= dz * bodyJMassMag
            
            bodyJ.vx += dx * bodyMassMag
            bodyJ.vy += dy * bodyMassMag
            bodyJ.vz += dz * bodyMassMag
        }
    }
    
    // Update positions
    for body in bodies {
        body.x += dt * body.vx
        body.y += dt * body.vy
        body.z += dt * body.vz
    }
}

func main() {
    let n = 1000000  // Number of iterations
    let bodies = initSolarSystem()
    
    let startTime = Date()
    
    offsetMomentum(bodies)
    let initialEnergy = energy(bodies)
    print("Initial energy: \(initialEnergy)")
    
    // Run simulation
    for _ in 0..<n {
        advance(bodies, dt: 0.01)
    }
    
    let finalEnergy = energy(bodies)
    print("Final energy: \(finalEnergy)")
    print("Energy delta: \(finalEnergy - initialEnergy)")
    
    let elapsedTime = -startTime.timeIntervalSinceNow * 1000
    print("Time taken: \(elapsedTime) ms")
}

main()