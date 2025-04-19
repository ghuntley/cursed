#!/usr/bin/env php
<?php
// N-body simulation benchmark adapted from The Computer Language Benchmarks Game

define('PI', 3.141592653589793);
define('SOLAR_MASS', 4.0 * PI * PI);
define('DAYS_PER_YEAR', 365.24);

class Planet {
    public $x, $y, $z;
    public $vx, $vy, $vz;
    public $mass;
    
    public function __construct($x, $y, $z, $vx, $vy, $vz, $mass) {
        $this->x = $x;
        $this->y = $y;
        $this->z = $z;
        $this->vx = $vx;
        $this->vy = $vy;
        $this->vz = $vz;
        $this->mass = $mass;
    }
}

// Initialize solar system
function initSolarSystem() {
    $bodies = [];
    
    // Sun
    $bodies[] = new Planet(
        0.0, 0.0, 0.0,
        0.0, 0.0, 0.0,
        SOLAR_MASS
    );
    
    // Jupiter
    $bodies[] = new Planet(
        4.84143144246472090e+00,
        -1.16032004402742839e+00,
        -1.03622044471123109e-01,
        1.66007664274403694e-03 * DAYS_PER_YEAR,
        7.69901118419740425e-03 * DAYS_PER_YEAR,
        -6.90460016972063023e-05 * DAYS_PER_YEAR,
        9.54791938424326609e-04 * SOLAR_MASS
    );
    
    // Saturn
    $bodies[] = new Planet(
        8.34336671824457987e+00,
        4.12479856412430479e+00,
        -4.03523417114321381e-01,
        -2.76742510726862411e-03 * DAYS_PER_YEAR,
        4.99852801234917238e-03 * DAYS_PER_YEAR,
        2.30417297573763929e-05 * DAYS_PER_YEAR,
        2.85885980666130812e-04 * SOLAR_MASS
    );
    
    // Uranus
    $bodies[] = new Planet(
        1.28943695621391310e+01,
        -1.51111514016986312e+01,
        -2.23307578892655734e-01,
        2.96460137564761618e-03 * DAYS_PER_YEAR,
        2.37847173959480950e-03 * DAYS_PER_YEAR,
        -2.96589568540237556e-05 * DAYS_PER_YEAR,
        4.36624404335156298e-05 * SOLAR_MASS
    );
    
    // Neptune
    $bodies[] = new Planet(
        1.53796971148509165e+01,
        -2.59193146099879641e+01,
        1.79258772950371181e-01,
        2.68067772490389322e-03 * DAYS_PER_YEAR,
        1.62824170038242295e-03 * DAYS_PER_YEAR,
        -9.51592254519715870e-05 * DAYS_PER_YEAR,
        5.15138902046611451e-05 * SOLAR_MASS
    );
    
    return $bodies;
}

// Offset momentum of the sun
function offsetMomentum($bodies) {
    $px = $py = $pz = 0.0;
    
    foreach ($bodies as $body) {
        $px += $body->vx * $body->mass;
        $py += $body->vy * $body->mass;
        $pz += $body->vz * $body->mass;
    }
    
    $bodies[0]->vx = -$px / SOLAR_MASS;
    $bodies[0]->vy = -$py / SOLAR_MASS;
    $bodies[0]->vz = -$pz / SOLAR_MASS;
}

// Calculate energy of the system
function energy($bodies) {
    $e = 0.0;
    $n = count($bodies);
    
    for ($i = 0; $i < $n; $i++) {
        $b = $bodies[$i];
        $e += 0.5 * $b->mass * ($b->vx * $b->vx + $b->vy * $b->vy + $b->vz * $b->vz);
        
        for ($j = $i + 1; $j < $n; $j++) {
            $b2 = $bodies[$j];
            $dx = $b->x - $b2->x;
            $dy = $b->y - $b2->y;
            $dz = $b->z - $b2->z;
            $distance = sqrt($dx * $dx + $dy * $dy + $dz * $dz);
            $e -= ($b->mass * $b2->mass) / $distance;
        }
    }
    
    return $e;
}

// Advance simulation by dt
function advance($bodies, $dt) {
    $n = count($bodies);
    
    for ($i = 0; $i < $n; $i++) {
        $b = $bodies[$i];
        
        for ($j = $i + 1; $j < $n; $j++) {
            $b2 = $bodies[$j];
            $dx = $b->x - $b2->x;
            $dy = $b->y - $b2->y;
            $dz = $b->z - $b2->z;
            
            $distance = sqrt($dx * $dx + $dy * $dy + $dz * $dz);
            $mag = $dt / ($distance * $distance * $distance);
            
            $b_mass_mag = $b->mass * $mag;
            $b2_mass_mag = $b2->mass * $mag;
            
            $b->vx -= $dx * $b2_mass_mag;
            $b->vy -= $dy * $b2_mass_mag;
            $b->vz -= $dz * $b2_mass_mag;
            
            $b2->vx += $dx * $b_mass_mag;
            $b2->vy += $dy * $b_mass_mag;
            $b2->vz += $dz * $b_mass_mag;
        }
    }
    
    foreach ($bodies as $body) {
        $body->x += $dt * $body->vx;
        $body->y += $dt * $body->vy;
        $body->z += $dt * $body->vz;
    }
}

function main() {
    $n = 1000000; // Number of iterations
    $bodies = initSolarSystem();
    $startTime = microtime(true);
    
    offsetMomentum($bodies);
    $initialEnergy = energy($bodies);
    printf("Initial energy: %.9f\n", $initialEnergy);
    
    for ($i = 0; $i < $n; $i++) {
        advance($bodies, 0.01);
    }
    
    $finalEnergy = energy($bodies);
    printf("Final energy: %.9f\n", $finalEnergy);
    printf("Energy delta: %.9f\n", $finalEnergy - $initialEnergy);
    
    $elapsed = (microtime(true) - $startTime) * 1000;
    echo "Time taken: $elapsed ms\n";
    
    // Get memory stats
    $memoryUsed = memory_get_peak_usage(true) / 1024;
    echo "Memory used: $memoryUsed KB\n";
}

main();