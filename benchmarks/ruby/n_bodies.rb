# N-body simulation benchmark for Ruby

PI = 3.141592653589793
SOLAR_MASS = 4.0 * PI * PI
DAYS_PER_YEAR = 365.24

# Planet class
class Planet
  attr_accessor :x, :y, :z, :vx, :vy, :vz, :mass
  
  def initialize(x, y, z, vx, vy, vz, mass)
    @x = x
    @y = y
    @z = z
    @vx = vx
    @vy = vy
    @vz = vz
    @mass = mass
  end
end

# Initialize the solar system
def init_solar_system
  bodies = [
    # Sun
    Planet.new(
      0.0, 0.0, 0.0,
      0.0, 0.0, 0.0,
      SOLAR_MASS
    ),
    
    # Jupiter
    Planet.new(
      4.84143144246472090e+00,
      -1.16032004402742839e+00,
      -1.03622044471123109e-01,
      1.66007664274403694e-03 * DAYS_PER_YEAR,
      7.69901118419740425e-03 * DAYS_PER_YEAR,
      -6.90460016972063023e-05 * DAYS_PER_YEAR,
      9.54791938424326609e-04 * SOLAR_MASS
    ),
    
    # Saturn
    Planet.new(
      8.34336671824457987e+00,
      4.12479856412430479e+00,
      -4.03523417114321381e-01,
      -2.76742510726862411e-03 * DAYS_PER_YEAR,
      4.99852801234917238e-03 * DAYS_PER_YEAR,
      2.30417297573763929e-05 * DAYS_PER_YEAR,
      2.85885980666130812e-04 * SOLAR_MASS
    ),
    
    # Uranus
    Planet.new(
      1.28943695621391310e+01,
      -1.51111514016986312e+01,
      -2.23307578892655734e-01,
      2.96460137564761618e-03 * DAYS_PER_YEAR,
      2.37847173959480950e-03 * DAYS_PER_YEAR,
      -2.96589568540237556e-05 * DAYS_PER_YEAR,
      4.36624404335156298e-05 * SOLAR_MASS
    ),
    
    # Neptune
    Planet.new(
      1.53796971148509165e+01,
      -2.59193146099879641e+01,
      1.79258772950371181e-01,
      2.68067772490389322e-03 * DAYS_PER_YEAR,
      1.62824170038242295e-03 * DAYS_PER_YEAR,
      -9.51592254519715870e-05 * DAYS_PER_YEAR,
      5.15138902046611451e-05 * SOLAR_MASS
    )
  ]
  
  bodies
end

# Offset the momentum of the system
def offset_momentum(bodies)
  px = py = pz = 0.0
  
  bodies.each do |body|
    px += body.vx * body.mass
    py += body.vy * body.mass
    pz += body.vz * body.mass
  end
  
  bodies[0].vx = -px / SOLAR_MASS
  bodies[0].vy = -py / SOLAR_MASS
  bodies[0].vz = -pz / SOLAR_MASS
end

# Calculate energy of the system
def energy(bodies)
  e = 0.0
  
  bodies.each_with_index do |b, i|
    # Add kinetic energy for each body
    e += 0.5 * b.mass * (b.vx * b.vx + b.vy * b.vy + b.vz * b.vz)
    
    # Add potential energy between each body and other bodies
    ((i+1)...bodies.length).each do |j|
      b2 = bodies[j]
      dx = b.x - b2.x
      dy = b.y - b2.y
      dz = b.z - b2.z
      distance = Math.sqrt(dx*dx + dy*dy + dz*dz)
      e -= (b.mass * b2.mass) / distance
    end
  end
  
  e
end

# Advance the system by dt
def advance(bodies, dt)
  bodies.each_with_index do |b, i|
    ((i+1)...bodies.length).each do |j|
      b2 = bodies[j]
      dx = b.x - b2.x
      dy = b.y - b2.y
      dz = b.z - b2.z
      
      distance = Math.sqrt(dx*dx + dy*dy + dz*dz)
      mag = dt / (distance * distance * distance)
      
      b_mass_mag = b.mass * mag
      b2_mass_mag = b2.mass * mag
      
      b.vx -= dx * b2_mass_mag
      b.vy -= dy * b2_mass_mag
      b.vz -= dz * b2_mass_mag
      
      b2.vx += dx * b_mass_mag
      b2.vy += dy * b_mass_mag
      b2.vz += dz * b_mass_mag
    end
  end
  
  bodies.each do |body|
    body.x += dt * body.vx
    body.y += dt * body.vy
    body.z += dt * body.vz
  end
end

# Main function
def main
  n = 1000000  # Number of iterations
  bodies = init_solar_system
  
  start_time = Time.now
  
  offset_momentum(bodies)
  initial_energy = energy(bodies)
  puts "Initial energy: #{initial_energy}"
  
  n.times do
    advance(bodies, 0.01)
  end
  
  final_energy = energy(bodies)
  puts "Final energy: #{final_energy}"
  puts "Energy delta: #{final_energy - initial_energy}"
  
  elapsed = Time.now - start_time
  puts "Time taken: #{elapsed * 1000} ms"
end

main