// N-body simulation benchmark adapted from The Computer Language Benchmarks Game

using System;

class NBodies
{
    const double PI = 3.141592653589793;
    const double SOLAR_MASS = 4.0 * PI * PI;
    const double DAYS_PER_YEAR = 365.24;
    
    class Planet
    {
        public double x, y, z;
        public double vx, vy, vz;
        public double mass;
        
        public Planet(double x, double y, double z, double vx, double vy, double vz, double mass)
        {
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
    static Planet[] InitSolarSystem()
    {
        Planet[] bodies = new Planet[5];
        
        // Sun
        bodies[0] = new Planet(
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
            SOLAR_MASS
        );
        
        // Jupiter
        bodies[1] = new Planet(
            4.84143144246472090e+00,
            -1.16032004402742839e+00,
            -1.03622044471123109e-01,
            1.66007664274403694e-03 * DAYS_PER_YEAR,
            7.69901118419740425e-03 * DAYS_PER_YEAR,
            -6.90460016972063023e-05 * DAYS_PER_YEAR,
            9.54791938424326609e-04 * SOLAR_MASS
        );
        
        // Saturn
        bodies[2] = new Planet(
            8.34336671824457987e+00,
            4.12479856412430479e+00,
            -4.03523417114321381e-01,
            -2.76742510726862411e-03 * DAYS_PER_YEAR,
            4.99852801234917238e-03 * DAYS_PER_YEAR,
            2.30417297573763929e-05 * DAYS_PER_YEAR,
            2.85885980666130812e-04 * SOLAR_MASS
        );
        
        // Uranus
        bodies[3] = new Planet(
            1.28943695621391310e+01,
            -1.51111514016986312e+01,
            -2.23307578892655734e-01,
            2.96460137564761618e-03 * DAYS_PER_YEAR,
            2.37847173959480950e-03 * DAYS_PER_YEAR,
            -2.96589568540237556e-05 * DAYS_PER_YEAR,
            4.36624404335156298e-05 * SOLAR_MASS
        );
        
        // Neptune
        bodies[4] = new Planet(
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
    
    // Offset momentum of the sun
    static void OffsetMomentum(Planet[] bodies)
    {
        double px = 0.0, py = 0.0, pz = 0.0;
        
        foreach (var body in bodies)
        {
            px += body.vx * body.mass;
            py += body.vy * body.mass;
            pz += body.vz * body.mass;
        }
        
        bodies[0].vx = -px / SOLAR_MASS;
        bodies[0].vy = -py / SOLAR_MASS;
        bodies[0].vz = -pz / SOLAR_MASS;
    }
    
    // Calculate energy of the system
    static double Energy(Planet[] bodies)
    {
        double e = 0.0;
        
        for (int i = 0; i < bodies.Length; i++)
        {
            Planet b = bodies[i];
            e += 0.5 * b.mass * (b.vx*b.vx + b.vy*b.vy + b.vz*b.vz);
            
            for (int j = i + 1; j < bodies.Length; j++)
            {
                Planet b2 = bodies[j];
                double dx = b.x - b2.x;
                double dy = b.y - b2.y;
                double dz = b.z - b2.z;
                double distance = Math.Sqrt(dx*dx + dy*dy + dz*dz);
                e -= (b.mass * b2.mass) / distance;
            }
        }
        
        return e;
    }
    
    // Advance simulation by dt
    static void Advance(Planet[] bodies, double dt)
    {
        for (int i = 0; i < bodies.Length; i++)
        {
            Planet b = bodies[i];
            
            for (int j = i + 1; j < bodies.Length; j++)
            {
                Planet b2 = bodies[j];
                double dx = b.x - b2.x;
                double dy = b.y - b2.y;
                double dz = b.z - b2.z;
                
                double distance = Math.Sqrt(dx*dx + dy*dy + dz*dz);
                double mag = dt / (distance * distance * distance);
                
                double b_mass_mag = b.mass * mag;
                double b2_mass_mag = b2.mass * mag;
                
                b.vx -= dx * b2_mass_mag;
                b.vy -= dy * b2_mass_mag;
                b.vz -= dz * b2_mass_mag;
                
                b2.vx += dx * b_mass_mag;
                b2.vy += dy * b_mass_mag;
                b2.vz += dz * b_mass_mag;
            }
        }
        
        foreach (var body in bodies)
        {
            body.x += dt * body.vx;
            body.y += dt * body.vy;
            body.z += dt * body.vz;
        }
    }
    
    public static void Main()
    {
        int n = 1000000; // Number of iterations
        Planet[] bodies = InitSolarSystem();
        DateTime startTime = DateTime.Now;
        
        OffsetMomentum(bodies);
        double initialEnergy = Energy(bodies);
        Console.WriteLine("Initial energy: {0:F9}", initialEnergy);
        
        for (int i = 0; i < n; i++)
        {
            Advance(bodies, 0.01);
        }
        
        double finalEnergy = Energy(bodies);
        Console.WriteLine("Final energy: {0:F9}", finalEnergy);
        Console.WriteLine("Energy delta: {0:F9}", finalEnergy - initialEnergy);
        
        TimeSpan elapsed = DateTime.Now - startTime;
        Console.WriteLine("Time taken: {0} ms", elapsed.TotalMilliseconds);
        
        // Get memory stats
        long memoryUsed = GC.GetTotalMemory(true) / 1024;
        Console.WriteLine("Memory used: {0} KB", memoryUsed);
    }
}