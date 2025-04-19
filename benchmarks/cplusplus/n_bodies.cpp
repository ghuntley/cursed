// N-body simulation benchmark for C++

#include <iostream>
#include <vector>
#include <cmath>
#include <chrono>

// Constants
const double PI = 3.141592653589793;
const double SOLAR_MASS = 4.0 * PI * PI;
const double DAYS_PER_YEAR = 365.24;

class Planet {
public:
    double x, y, z;
    double vx, vy, vz;
    double mass;
    
    Planet(double x, double y, double z, double vx, double vy, double vz, double mass)
        : x(x), y(y), z(z), vx(vx), vy(vy), vz(vz), mass(mass) {}
};

// Initialize the solar system
std::vector<Planet> initSolarSystem() {
    std::vector<Planet> bodies;
    
    // Sun
    bodies.emplace_back(
        0.0, 0.0, 0.0,
        0.0, 0.0, 0.0,
        SOLAR_MASS
    );
    
    // Jupiter
    bodies.emplace_back(
        4.84143144246472090e+00,
        -1.16032004402742839e+00,
        -1.03622044471123109e-01,
        1.66007664274403694e-03 * DAYS_PER_YEAR,
        7.69901118419740425e-03 * DAYS_PER_YEAR,
        -6.90460016972063023e-05 * DAYS_PER_YEAR,
        9.54791938424326609e-04 * SOLAR_MASS
    );
    
    // Saturn
    bodies.emplace_back(
        8.34336671824457987e+00,
        4.12479856412430479e+00,
        -4.03523417114321381e-01,
        -2.76742510726862411e-03 * DAYS_PER_YEAR,
        4.99852801234917238e-03 * DAYS_PER_YEAR,
        2.30417297573763929e-05 * DAYS_PER_YEAR,
        2.85885980666130812e-04 * SOLAR_MASS
    );
    
    // Uranus
    bodies.emplace_back(
        1.28943695621391310e+01,
        -1.51111514016986312e+01,
        -2.23307578892655734e-01,
        2.96460137564761618e-03 * DAYS_PER_YEAR,
        2.37847173959480950e-03 * DAYS_PER_YEAR,
        -2.96589568540237556e-05 * DAYS_PER_YEAR,
        4.36624404335156298e-05 * SOLAR_MASS
    );
    
    // Neptune
    bodies.emplace_back(
        1.53796971148509165e+01,
        -2.59193146099879641e+01,
        1.79258772950371181e-01,
        2.68067772490389322e-03 * DAYS_PER_YEAR,
        1.62824170038242295e-03 * DAYS_PER_YEAR,
        -9.51592254519715870e-05 * DAYS_PER_YEAR,
        5.15138902046611451e-05 * SOLAR_MASS
    );
    
    return bodies;
}

// Offset the momentum of the system
void offsetMomentum(std::vector<Planet>& bodies) {
    double px = 0.0, py = 0.0, pz = 0.0;
    
    for (auto& body : bodies) {
        px += body.vx * body.mass;
        py += body.vy * body.mass;
        pz += body.vz * body.mass;
    }
    
    bodies[0].vx = -px / SOLAR_MASS;
    bodies[0].vy = -py / SOLAR_MASS;
    bodies[0].vz = -pz / SOLAR_MASS;
}

// Calculate energy of the system
double energy(const std::vector<Planet>& bodies) {
    double e = 0.0;
    
    for (size_t i = 0; i < bodies.size(); ++i) {
        const auto& body = bodies[i];
        
        // Kinetic energy
        e += 0.5 * body.mass * (body.vx * body.vx + body.vy * body.vy + body.vz * body.vz);
        
        // Potential energy with all other bodies
        for (size_t j = i + 1; j < bodies.size(); ++j) {
            const auto& other = bodies[j];
            double dx = body.x - other.x;
            double dy = body.y - other.y;
            double dz = body.z - other.z;
            double distance = std::sqrt(dx*dx + dy*dy + dz*dz);
            e -= (body.mass * other.mass) / distance;
        }
    }
    
    return e;
}

// Advance the system by dt
void advance(std::vector<Planet>& bodies, double dt) {
    // Update velocities based on gravity
    for (size_t i = 0; i < bodies.size(); ++i) {
        auto& body = bodies[i];
        
        for (size_t j = i + 1; j < bodies.size(); ++j) {
            auto& other = bodies[j];
            double dx = body.x - other.x;
            double dy = body.y - other.y;
            double dz = body.z - other.z;
            
            double distance = std::sqrt(dx*dx + dy*dy + dz*dz);
            double mag = dt / (distance * distance * distance);
            
            double body_mass_mag = body.mass * mag;
            double other_mass_mag = other.mass * mag;
            
            body.vx -= dx * other_mass_mag;
            body.vy -= dy * other_mass_mag;
            body.vz -= dz * other_mass_mag;
            
            other.vx += dx * body_mass_mag;
            other.vy += dy * body_mass_mag;
            other.vz += dz * body_mass_mag;
        }
    }
    
    // Update positions
    for (auto& body : bodies) {
        body.x += dt * body.vx;
        body.y += dt * body.vy;
        body.z += dt * body.vz;
    }
}

int main() {
    const int n = 1000000;  // Number of iterations
    auto bodies = initSolarSystem();
    
    auto startTime = std::chrono::high_resolution_clock::now();
    
    offsetMomentum(bodies);
    double initialEnergy = energy(bodies);
    std::cout << "Initial energy: " << initialEnergy << std::endl;
    
    // Run simulation
    for (int i = 0; i < n; ++i) {
        advance(bodies, 0.01);
    }
    
    double finalEnergy = energy(bodies);
    std::cout << "Final energy: " << finalEnergy << std::endl;
    std::cout << "Energy delta: " << finalEnergy - initialEnergy << std::endl;
    
    auto endTime = std::chrono::high_resolution_clock::now();
    auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(endTime - startTime);
    std::cout << "Time taken: " << elapsed.count() << " ms" << std::endl;
    
    return 0;
}