// N-body simulation benchmark for Kotlin

import kotlin.math.sqrt
import kotlin.system.measureTimeMillis

// Constants
const val PI = 3.141592653589793
const val SOLAR_MASS = 4.0 * PI * PI
const val DAYS_PER_YEAR = 365.24

// Planet class
class Planet(
    var x: Double, var y: Double, var z: Double,
    var vx: Double, var vy: Double, var vz: Double,
    val mass: Double
)

// Initialize the solar system
fun initSolarSystem(): List<Planet> {
    return listOf(
        // Sun
        Planet(
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
            SOLAR_MASS
        ),
        
        // Jupiter
        Planet(
            4.84143144246472090e+00,
            -1.16032004402742839e+00,
            -1.03622044471123109e-01,
            1.66007664274403694e-03 * DAYS_PER_YEAR,
            7.69901118419740425e-03 * DAYS_PER_YEAR,
            -6.90460016972063023e-05 * DAYS_PER_YEAR,
            9.54791938424326609e-04 * SOLAR_MASS
        ),
        
        // Saturn
        Planet(
            8.34336671824457987e+00,
            4.12479856412430479e+00,
            -4.03523417114321381e-01,
            -2.76742510726862411e-03 * DAYS_PER_YEAR,
            4.99852801234917238e-03 * DAYS_PER_YEAR,
            2.30417297573763929e-05 * DAYS_PER_YEAR,
            2.85885980666130812e-04 * SOLAR_MASS
        ),
        
        // Uranus
        Planet(
            1.28943695621391310e+01,
            -1.51111514016986312e+01,
            -2.23307578892655734e-01,
            2.96460137564761618e-03 * DAYS_PER_YEAR,
            2.37847173959480950e-03 * DAYS_PER_YEAR,
            -2.96589568540237556e-05 * DAYS_PER_YEAR,
            4.36624404335156298e-05 * SOLAR_MASS
        ),
        
        // Neptune
        Planet(
            1.53796971148509165e+01,
            -2.59193146099879641e+01,
            1.79258772950371181e-01,
            2.68067772490389322e-03 * DAYS_PER_YEAR,
            1.62824170038242295e-03 * DAYS_PER_YEAR,
            -9.51592254519715870e-05 * DAYS_PER_YEAR,
            5.15138902046611451e-05 * SOLAR_MASS
        )
    )
}

// Offset the momentum of the system
fun offsetMomentum(bodies: List<Planet>) {
    var px = 0.0
    var py = 0.0
    var pz = 0.0
    
    bodies.forEach { body ->
        px += body.vx * body.mass
        py += body.vy * body.mass
        pz += body.vz * body.mass
    }
    
    bodies[0].vx = -px / SOLAR_MASS
    bodies[0].vy = -py / SOLAR_MASS
    bodies[0].vz = -pz / SOLAR_MASS
}

// Calculate energy of the system
fun energy(bodies: List<Planet>): Double {
    var e = 0.0
    
    for (i in bodies.indices) {
        val body = bodies[i]
        
        // Kinetic energy
        e += 0.5 * body.mass * (body.vx * body.vx + body.vy * body.vy + body.vz * body.vz)
        
        // Potential energy with other bodies
        for (j in i + 1 until bodies.size) {
            val other = bodies[j]
            val dx = body.x - other.x
            val dy = body.y - other.y
            val dz = body.z - other.z
            val distance = sqrt(dx*dx + dy*dy + dz*dz)
            e -= (body.mass * other.mass) / distance
        }
    }
    
    return e
}

// Advance the system by dt
fun advance(bodies: List<Planet>, dt: Double) {
    // Calculate interactions between pairs of bodies
    for (i in bodies.indices) {
        val body = bodies[i]
        
        for (j in i + 1 until bodies.size) {
            val other = bodies[j]
            val dx = body.x - other.x
            val dy = body.y - other.y
            val dz = body.z - other.z
            
            val distance = sqrt(dx*dx + dy*dy + dz*dz)
            val mag = dt / (distance * distance * distance)
            
            val bodyMassMag = body.mass * mag
            val otherMassMag = other.mass * mag
            
            body.vx -= dx * otherMassMag
            body.vy -= dy * otherMassMag
            body.vz -= dz * otherMassMag
            
            other.vx += dx * bodyMassMag
            other.vy += dy * bodyMassMag
            other.vz += dz * bodyMassMag
        }
    }
    
    // Update positions
    bodies.forEach { body ->
        body.x += dt * body.vx
        body.y += dt * body.vy
        body.z += dt * body.vz
    }
}

fun main() {
    val n = 1000000  // Number of iterations
    val bodies = initSolarSystem()
    
    val totalTime = measureTimeMillis {
        offsetMomentum(bodies)
        val initialEnergy = energy(bodies)
        println("Initial energy: $initialEnergy")
        
        repeat(n) {
            advance(bodies, 0.01)
        }
        
        val finalEnergy = energy(bodies)
        println("Final energy: $finalEnergy")
        println("Energy delta: ${finalEnergy - initialEnergy}")
    }
    
    println("Time taken: $totalTime ms")
}