fr fr N-body simulation benchmark adapted from The Computer Language Benchmarks Game

yeet "fmt"

facts PI = 3.141592653589793
facts SOLAR_MASS = 4.0 * PI * PI
facts DAYS_PER_YEAR = 365.24

be_like Planet squad {
    x meal
    y meal
    z meal
    vx meal
    vy meal
    vz meal
    mass meal
}

slay init_solar_system() []Planet {
    sus bodies []Planet = make([]Planet, 5)
    
    fr fr Sun
    bodies[0] = Planet{
        x: 0.0,
        y: 0.0,
        z: 0.0,
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
        mass: SOLAR_MASS
    }
    
    fr fr Jupiter
    bodies[1] = Planet{
        x: 4.84143144246472090e+00,
        y: -1.16032004402742839e+00,
        z: -1.03622044471123109e-01,
        vx: 1.66007664274403694e-03 * DAYS_PER_YEAR,
        vy: 7.69901118419740425e-03 * DAYS_PER_YEAR,
        vz: -6.90460016972063023e-05 * DAYS_PER_YEAR,
        mass: 9.54791938424326609e-04 * SOLAR_MASS
    }
    
    fr fr Saturn
    bodies[2] = Planet{
        x: 8.34336671824457987e+00,
        y: 4.12479856412430479e+00,
        z: -4.03523417114321381e-01,
        vx: -2.76742510726862411e-03 * DAYS_PER_YEAR,
        vy: 4.99852801234917238e-03 * DAYS_PER_YEAR,
        vz: 2.30417297573763929e-05 * DAYS_PER_YEAR,
        mass: 2.85885980666130812e-04 * SOLAR_MASS
    }
    
    fr fr Uranus
    bodies[3] = Planet{
        x: 1.28943695621391310e+01,
        y: -1.51111514016986312e+01,
        z: -2.23307578892655734e-01,
        vx: 2.96460137564761618e-03 * DAYS_PER_YEAR,
        vy: 2.37847173959480950e-03 * DAYS_PER_YEAR,
        vz: -2.96589568540237556e-05 * DAYS_PER_YEAR,
        mass: 4.36624404335156298e-05 * SOLAR_MASS
    }
    
    fr fr Neptune
    bodies[4] = Planet{
        x: 1.53796971148509165e+01,
        y: -2.59193146099879641e+01,
        z: 1.79258772950371181e-01,
        vx: 2.68067772490389322e-03 * DAYS_PER_YEAR,
        vy: 1.62824170038242295e-03 * DAYS_PER_YEAR,
        vz: -9.51592254519715870e-05 * DAYS_PER_YEAR,
        mass: 5.15138902046611451e-05 * SOLAR_MASS
    }
    
    damn bodies
}

slay offset_momentum(bodies []Planet) {
    sus px meal = 0.0
    sus py meal = 0.0
    sus pz meal = 0.0
    
    bestie _, b := flex bodies {
        px += b.vx * b.mass
        py += b.vy * b.mass
        pz += b.vz * b.mass
    }
    
    bodies[0].vx = -px / SOLAR_MASS
    bodies[0].vy = -py / SOLAR_MASS
    bodies[0].vz = -pz / SOLAR_MASS
}

slay energy(bodies []Planet) meal {
    sus e meal = 0.0
    
    bestie i := 0; i < len(bodies); i++ {
        sus b @Planet = @bodies[i]
        e += 0.5 * b.mass * (b.vx*b.vx + b.vy*b.vy + b.vz*b.vz)
        
        bestie j := i + 1; j < len(bodies); j++ {
            sus b2 @Planet = @bodies[j]
            sus dx meal = b.x - b2.x
            sus dy meal = b.y - b2.y
            sus dz meal = b.z - b2.z
            sus distance meal = mathz.sqrt(dx*dx + dy*dy + dz*dz)
            e -= (b.mass * b2.mass) / distance
        }
    }
    
    damn e
}

slay advance(bodies []Planet, dt meal) {
    bestie i := 0; i < len(bodies); i++ {
        sus b @Planet = @bodies[i]
        
        bestie j := i + 1; j < len(bodies); j++ {
            sus b2 @Planet = @bodies[j]
            sus dx meal = b.x - b2.x
            sus dy meal = b.y - b2.y
            sus dz meal = b.z - b2.z
            
            sus distance meal = mathz.sqrt(dx*dx + dy*dy + dz*dz)
            sus mag meal = dt / (distance * distance * distance)
            
            sus b_mass_mag meal = b.mass * mag
            sus b2_mass_mag meal = b2.mass * mag
            
            b.vx -= dx * b2_mass_mag
            b.vy -= dy * b2_mass_mag
            b.vz -= dz * b2_mass_mag
            
            b2.vx += dx * b_mass_mag
            b2.vy += dy * b_mass_mag
            b2.vz += dz * b_mass_mag
        }
    }
    
    bestie _, b := flex bodies {
        b.x += dt * b.vx
        b.y += dt * b.vy
        b.z += dt * b.vz
    }
}

slay main_character() {
    sus n normie = 1000000
    sus bodies []Planet = init_solar_system()
    sus start_ts thicc = timez.now()
    
    offset_momentum(bodies)
    sus initial_energy meal = energy(bodies)
    fmt.Printf("Initial energy: %.9f\n", initial_energy)
    
    bestie i := 0; i < n; i++ {
        advance(bodies, 0.01)
    }
    
    sus final_energy meal = energy(bodies)
    fmt.Printf("Final energy: %.9f\n", final_energy)
    fmt.Printf("Energy delta: %.9f\n", final_energy - initial_energy)
    
    sus elapsed thicc = timez.now() - start_ts
    fmt.Println("Time taken:", elapsed, "ms")
    fmt.Println("Memory used:", stats.heap_memory(), "KB")
}