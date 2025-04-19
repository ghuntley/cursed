// N-body simulation benchmark adapted from The Computer Language Benchmarks Game

#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <time.h>

#define PI 3.141592653589793
#define SOLAR_MASS (4.0 * PI * PI)
#define DAYS_PER_YEAR 365.24

typedef struct {
    double x, y, z;
    double vx, vy, vz;
    double mass;
} Planet;

// Initialize solar system
void init_solar_system(Planet bodies[]) {
    // Sun
    bodies[0] = (Planet){
        .x = 0, .y = 0, .z = 0,
        .vx = 0, .vy = 0, .vz = 0,
        .mass = SOLAR_MASS
    };
    
    // Jupiter
    bodies[1] = (Planet){
        .x = 4.84143144246472090e+00,
        .y = -1.16032004402742839e+00,
        .z = -1.03622044471123109e-01,
        .vx = 1.66007664274403694e-03 * DAYS_PER_YEAR,
        .vy = 7.69901118419740425e-03 * DAYS_PER_YEAR,
        .vz = -6.90460016972063023e-05 * DAYS_PER_YEAR,
        .mass = 9.54791938424326609e-04 * SOLAR_MASS
    };
    
    // Saturn
    bodies[2] = (Planet){
        .x = 8.34336671824457987e+00,
        .y = 4.12479856412430479e+00,
        .z = -4.03523417114321381e-01,
        .vx = -2.76742510726862411e-03 * DAYS_PER_YEAR,
        .vy = 4.99852801234917238e-03 * DAYS_PER_YEAR,
        .vz = 2.30417297573763929e-05 * DAYS_PER_YEAR,
        .mass = 2.85885980666130812e-04 * SOLAR_MASS
    };
    
    // Uranus
    bodies[3] = (Planet){
        .x = 1.28943695621391310e+01,
        .y = -1.51111514016986312e+01,
        .z = -2.23307578892655734e-01,
        .vx = 2.96460137564761618e-03 * DAYS_PER_YEAR,
        .vy = 2.37847173959480950e-03 * DAYS_PER_YEAR,
        .vz = -2.96589568540237556e-05 * DAYS_PER_YEAR,
        .mass = 4.36624404335156298e-05 * SOLAR_MASS
    };
    
    // Neptune
    bodies[4] = (Planet){
        .x = 1.53796971148509165e+01,
        .y = -2.59193146099879641e+01,
        .z = 1.79258772950371181e-01,
        .vx = 2.68067772490389322e-03 * DAYS_PER_YEAR,
        .vy = 1.62824170038242295e-03 * DAYS_PER_YEAR,
        .vz = -9.51592254519715870e-05 * DAYS_PER_YEAR,
        .mass = 5.15138902046611451e-05 * SOLAR_MASS
    };
}

// Offset momentum of the sun
void offset_momentum(Planet bodies[], int n) {
    double px = 0.0, py = 0.0, pz = 0.0;
    
    for (int i = 0; i < n; i++) {
        px += bodies[i].vx * bodies[i].mass;
        py += bodies[i].vy * bodies[i].mass;
        pz += bodies[i].vz * bodies[i].mass;
    }
    
    bodies[0].vx = -px / SOLAR_MASS;
    bodies[0].vy = -py / SOLAR_MASS;
    bodies[0].vz = -pz / SOLAR_MASS;
}

// Calculate energy of the system
double energy(Planet bodies[], int n) {
    double e = 0.0;
    
    for (int i = 0; i < n; i++) {
        e += 0.5 * bodies[i].mass * (
            bodies[i].vx * bodies[i].vx +
            bodies[i].vy * bodies[i].vy +
            bodies[i].vz * bodies[i].vz);
        
        for (int j = i + 1; j < n; j++) {
            double dx = bodies[i].x - bodies[j].x;
            double dy = bodies[i].y - bodies[j].y;
            double dz = bodies[i].z - bodies[j].z;
            double distance = sqrt(dx * dx + dy * dy + dz * dz);
            e -= (bodies[i].mass * bodies[j].mass) / distance;
        }
    }
    
    return e;
}

// Advance simulation by dt
void advance(Planet bodies[], int n, double dt) {
    for (int i = 0; i < n; i++) {
        for (int j = i + 1; j < n; j++) {
            double dx = bodies[i].x - bodies[j].x;
            double dy = bodies[i].y - bodies[j].y;
            double dz = bodies[i].z - bodies[j].z;
            
            double distance = sqrt(dx * dx + dy * dy + dz * dz);
            double mag = dt / (distance * distance * distance);
            
            double b_mass_mag = bodies[i].mass * mag;
            double b2_mass_mag = bodies[j].mass * mag;
            
            bodies[i].vx -= dx * b2_mass_mag;
            bodies[i].vy -= dy * b2_mass_mag;
            bodies[i].vz -= dz * b2_mass_mag;
            
            bodies[j].vx += dx * b_mass_mag;
            bodies[j].vy += dy * b_mass_mag;
            bodies[j].vz += dz * b_mass_mag;
        }
    }
    
    for (int i = 0; i < n; i++) {
        bodies[i].x += dt * bodies[i].vx;
        bodies[i].y += dt * bodies[i].vy;
        bodies[i].z += dt * bodies[i].vz;
    }
}

int main() {
    const int n = 1000000; // Number of iterations
    const int num_bodies = 5;
    Planet bodies[5];
    clock_t start = clock();
    
    init_solar_system(bodies);
    offset_momentum(bodies, num_bodies);
    
    double initial_energy = energy(bodies, num_bodies);
    printf("Initial energy: %.9f\n", initial_energy);
    
    for (int i = 0; i < n; i++) {
        advance(bodies, num_bodies, 0.01);
    }
    
    double final_energy = energy(bodies, num_bodies);
    printf("Final energy: %.9f\n", final_energy);
    printf("Energy delta: %.9f\n", final_energy - initial_energy);
    
    // Calculate elapsed time
    clock_t end = clock();
    double elapsed = (double)(end - start) * 1000.0 / CLOCKS_PER_SEC;
    printf("Time taken: %.2f ms\n", elapsed);
    
    // Note: C doesn't have a standard way to get memory usage
    printf("Memory monitoring not available for C implementation\n");
    
    return 0;
}