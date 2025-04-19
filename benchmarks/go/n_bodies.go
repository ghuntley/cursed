// N-body simulation benchmark adapted from The Computer Language Benchmarks Game

package main

import (
	"fmt"
	"math"
	"runtime"
	"time"
)

const (
	PI           = 3.141592653589793
	SOLAR_MASS   = 4.0 * PI * PI
	DAYS_PER_YEAR = 365.24
)

// Planet structure
type Planet struct {
	x, y, z       float64
	vx, vy, vz    float64
	mass          float64
}

// Initialize solar system
func initSolarSystem() []Planet {
	bodies := make([]Planet, 5)

	// Sun
	bodies[0] = Planet{
		x:    0.0,
		y:    0.0,
		z:    0.0,
		vx:   0.0,
		vy:   0.0,
		vz:   0.0,
		mass: SOLAR_MASS,
	}

	// Jupiter
	bodies[1] = Planet{
		x:    4.84143144246472090e+00,
		y:    -1.16032004402742839e+00,
		z:    -1.03622044471123109e-01,
		vx:   1.66007664274403694e-03 * DAYS_PER_YEAR,
		vy:   7.69901118419740425e-03 * DAYS_PER_YEAR,
		vz:   -6.90460016972063023e-05 * DAYS_PER_YEAR,
		mass: 9.54791938424326609e-04 * SOLAR_MASS,
	}

	// Saturn
	bodies[2] = Planet{
		x:    8.34336671824457987e+00,
		y:    4.12479856412430479e+00,
		z:    -4.03523417114321381e-01,
		vx:   -2.76742510726862411e-03 * DAYS_PER_YEAR,
		vy:   4.99852801234917238e-03 * DAYS_PER_YEAR,
		vz:   2.30417297573763929e-05 * DAYS_PER_YEAR,
		mass: 2.85885980666130812e-04 * SOLAR_MASS,
	}

	// Uranus
	bodies[3] = Planet{
		x:    1.28943695621391310e+01,
		y:    -1.51111514016986312e+01,
		z:    -2.23307578892655734e-01,
		vx:   2.96460137564761618e-03 * DAYS_PER_YEAR,
		vy:   2.37847173959480950e-03 * DAYS_PER_YEAR,
		vz:   -2.96589568540237556e-05 * DAYS_PER_YEAR,
		mass: 4.36624404335156298e-05 * SOLAR_MASS,
	}

	// Neptune
	bodies[4] = Planet{
		x:    1.53796971148509165e+01,
		y:    -2.59193146099879641e+01,
		z:    1.79258772950371181e-01,
		vx:   2.68067772490389322e-03 * DAYS_PER_YEAR,
		vy:   1.62824170038242295e-03 * DAYS_PER_YEAR,
		vz:   -9.51592254519715870e-05 * DAYS_PER_YEAR,
		mass: 5.15138902046611451e-05 * SOLAR_MASS,
	}

	return bodies
}

// Offset momentum of the sun
func offsetMomentum(bodies []Planet) {
	var px, py, pz float64

	for _, body := range bodies {
		px += body.vx * body.mass
		py += body.vy * body.mass
		pz += body.vz * body.mass
	}

	bodies[0].vx = -px / SOLAR_MASS
	bodies[0].vy = -py / SOLAR_MASS
	bodies[0].vz = -pz / SOLAR_MASS
}

// Calculate energy of the system
func energy(bodies []Planet) float64 {
	e := 0.0
	for i := 0; i < len(bodies); i++ {
		b := &bodies[i]
		e += 0.5 * b.mass * (b.vx*b.vx + b.vy*b.vy + b.vz*b.vz)

		for j := i + 1; j < len(bodies); j++ {
			b2 := &bodies[j]
			dx := b.x - b2.x
			dy := b.y - b2.y
			dz := b.z - b2.z
			distance := math.Sqrt(dx*dx + dy*dy + dz*dz)
			e -= (b.mass * b2.mass) / distance
		}
	}
	return e
}

// Advance simulation by dt
func advance(bodies []Planet, dt float64) {
	for i := 0; i < len(bodies); i++ {
		b := &bodies[i]

		for j := i + 1; j < len(bodies); j++ {
			b2 := &bodies[j]
			dx := b.x - b2.x
			dy := b.y - b2.y
			dz := b.z - b2.z

			distance := math.Sqrt(dx*dx + dy*dy + dz*dz)
			mag := dt / (distance * distance * distance)

			b_mass_mag := b.mass * mag
			b2_mass_mag := b2.mass * mag

			b.vx -= dx * b2_mass_mag
			b.vy -= dy * b2_mass_mag
			b.vz -= dz * b2_mass_mag

			b2.vx += dx * b_mass_mag
			b2.vy += dy * b_mass_mag
			b2.vz += dz * b_mass_mag
		}
	}

	for i := range bodies {
		b := &bodies[i]
		b.x += dt * b.vx
		b.y += dt * b.vy
		b.z += dt * b.vz
	}
}

func main() {
	n := 1000000 // Number of iterations
	bodies := initSolarSystem()
	startTime := time.Now()

	offsetMomentum(bodies)
	initialEnergy := energy(bodies)
	fmt.Printf("Initial energy: %.9f\n", initialEnergy)

	for i := 0; i < n; i++ {
		advance(bodies, 0.01)
	}

	finalEnergy := energy(bodies)
	fmt.Printf("Final energy: %.9f\n", finalEnergy)
	fmt.Printf("Energy delta: %.9f\n", finalEnergy-initialEnergy)

	elapsed := time.Since(startTime).Milliseconds()
	fmt.Printf("Time taken: %d ms\n", elapsed)

	// Get memory stats
	var mem runtime.MemStats
	runtime.ReadMemStats(&mem)
	fmt.Printf("Memory used: %d KB\n", mem.Alloc/1024)
}