#!/usr/bin/env python3
# N-bodies simulation benchmark adapted from The Computer Language Benchmarks Game

import math
import time
import resource

PI = 3.141592653589793
SOLAR_MASS = 4 * PI * PI
DAYS_PER_YEAR = 365.24

class Body:
    def __init__(self, x, y, z, vx, vy, vz, mass):
        self.x = x
        self.y = y
        self.z = z
        self.vx = vx
        self.vy = vy
        self.vz = vz
        self.mass = mass

def advance(bodies, dt):
    n = len(bodies)
    
    for i in range(n):
        b = bodies[i]
        for j in range(i+1, n):
            b2 = bodies[j]
            dx = b.x - b2.x
            dy = b.y - b2.y
            dz = b.z - b2.z
            
            distance = math.sqrt(dx*dx + dy*dy + dz*dz)
            mag = dt / (distance * distance * distance)
            
            b_mass_mag = b.mass * mag
            b2_mass_mag = b2.mass * mag
            
            b.vx -= dx * b2_mass_mag
            b.vy -= dy * b2_mass_mag
            b.vz -= dz * b2_mass_mag
            
            b2.vx += dx * b_mass_mag
            b2.vy += dy * b_mass_mag
            b2.vz += dz * b_mass_mag
    
    for body in bodies:
        body.x += dt * body.vx
        body.y += dt * body.vy
        body.z += dt * body.vz

def energy(bodies):
    e = 0.0
    n = len(bodies)
    
    for i in range(n):
        b = bodies[i]
        e += 0.5 * b.mass * (b.vx*b.vx + b.vy*b.vy + b.vz*b.vz)
        
        for j in range(i+1, n):
            b2 = bodies[j]
            dx = b.x - b2.x
            dy = b.y - b2.y
            dz = b.z - b2.z
            
            distance = math.sqrt(dx*dx + dy*dy + dz*dz)
            e -= (b.mass * b2.mass) / distance
    
    return e

def offset_momentum(bodies):
    px = py = pz = 0.0
    
    for body in bodies:
        px += body.vx * body.mass
        py += body.vy * body.mass
        pz += body.vz * body.mass
    
    bodies[0].vx = -px / SOLAR_MASS
    bodies[0].vy = -py / SOLAR_MASS
    bodies[0].vz = -pz / SOLAR_MASS

def main():
    start_time = time.time()
    
    # Initialize bodies
    sun = Body(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, SOLAR_MASS)
    
    jupiter = Body(
        4.84143144246472090e+00,
        -1.16032004402742839e+00,
        -1.03622044471123109e-01,
        1.66007664274403694e-03 * DAYS_PER_YEAR,
        7.69901118419740425e-03 * DAYS_PER_YEAR,
        -6.90460016972063023e-05 * DAYS_PER_YEAR,
        9.54791938424326609e-04 * SOLAR_MASS
    )
    
    saturn = Body(
        8.34336671824457987e+00,
        4.12479856412430479e+00,
        -4.03523417114321381e-01,
        -2.76742510726862411e-03 * DAYS_PER_YEAR,
        4.99852801234917238e-03 * DAYS_PER_YEAR,
        2.30417297573763929e-05 * DAYS_PER_YEAR,
        2.85885980666130812e-04 * SOLAR_MASS
    )
    
    uranus = Body(
        1.28943695621391310e+01,
        -1.51111514016986312e+01,
        -2.23307578892655734e-01,
        2.96460137564761618e-03 * DAYS_PER_YEAR,
        2.37847173959480950e-03 * DAYS_PER_YEAR,
        -2.96589568540237556e-05 * DAYS_PER_YEAR,
        4.36624404335156298e-05 * SOLAR_MASS
    )
    
    neptune = Body(
        1.53796971148509165e+01,
        -2.59193146099879641e+01,
        1.79258772950371181e-01,
        2.68067772490389322e-03 * DAYS_PER_YEAR,
        1.62824170038242295e-03 * DAYS_PER_YEAR,
        -9.51592254519715870e-05 * DAYS_PER_YEAR,
        5.15138902046611451e-05 * SOLAR_MASS
    )
    
    bodies = [sun, jupiter, saturn, uranus, neptune]
    
    # Offset the momentum of the system
    offset_momentum(bodies)
    
    # Calculate initial energy
    initial_energy = energy(bodies)
    print(f"Initial energy: {initial_energy}")
    
    # Run simulation
    steps = 1000000
    for _ in range(steps):
        advance(bodies, 0.01)
    
    # Calculate final energy
    final_energy = energy(bodies)
    print(f"Final energy: {final_energy}")
    print(f"Energy delta: {abs(final_energy - initial_energy)}")
    
    elapsed = (time.time() - start_time) * 1000
    print(f"Time taken: {elapsed:.2f} ms")
    
    # Get memory usage
    mem_usage = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    print(f"Memory used: {mem_usage / 1024:.2f} MB")

if __name__ == "__main__":
    main()