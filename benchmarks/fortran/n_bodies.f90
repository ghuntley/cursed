! N-body simulation benchmark adapted from The Computer Language Benchmarks Game

program n_bodies
  use iso_fortran_env, only: dp => real64, stdout => output_unit
  implicit none

  ! Constants
  real(dp), parameter :: PI = 3.141592653589793_dp
  real(dp), parameter :: SOLAR_MASS = 4.0_dp * PI * PI
  real(dp), parameter :: DAYS_PER_YEAR = 365.24_dp

  ! Planet type
  type :: planet
    real(dp) :: x, y, z
    real(dp) :: vx, vy, vz
    real(dp) :: mass
  end type planet

  ! Variables
  integer, parameter :: n = 1000000 ! Number of iterations
  type(planet), dimension(5) :: bodies
  real(dp) :: initial_energy, final_energy, energy_delta
  integer :: i, start_time, end_time, count_rate

  ! Initialize solar system
  call init_solar_system(bodies)
  call system_clock(start_time, count_rate)

  call offset_momentum(bodies)
  initial_energy = energy(bodies)
  write(stdout, '(A,F20.9)') "Initial energy: ", initial_energy

  do i = 1, n
    call advance(bodies, 0.01_dp)
  end do

  final_energy = energy(bodies)
  energy_delta = final_energy - initial_energy
  write(stdout, '(A,F20.9)') "Final energy: ", final_energy
  write(stdout, '(A,F20.9)') "Energy delta: ", energy_delta

  call system_clock(end_time)
  write(stdout, '(A,I0,A)') "Time taken: ", (end_time - start_time) * 1000 / count_rate, " ms"

contains

  ! Initialize solar system
  subroutine init_solar_system(bodies)
    type(planet), dimension(5), intent(out) :: bodies

    ! Sun
    bodies(1) = planet(
      0.0_dp, 0.0_dp, 0.0_dp,
      0.0_dp, 0.0_dp, 0.0_dp,
      SOLAR_MASS)

    ! Jupiter
    bodies(2) = planet(
      4.84143144246472090e+00_dp, -1.16032004402742839e+00_dp, -1.03622044471123109e-01_dp,
      1.66007664274403694e-03_dp * DAYS_PER_YEAR, 7.69901118419740425e-03_dp * DAYS_PER_YEAR, -6.90460016972063023e-05_dp * DAYS_PER_YEAR,
      9.54791938424326609e-04_dp * SOLAR_MASS)

    ! Saturn
    bodies(3) = planet(
      8.34336671824457987e+00_dp, 4.12479856412430479e+00_dp, -4.03523417114321381e-01_dp,
      -2.76742510726862411e-03_dp * DAYS_PER_YEAR, 4.99852801234917238e-03_dp * DAYS_PER_YEAR, 2.30417297573763929e-05_dp * DAYS_PER_YEAR,
      2.85885980666130812e-04_dp * SOLAR_MASS)

    ! Uranus
    bodies(4) = planet(
      1.28943695621391310e+01_dp, -1.51111514016986312e+01_dp, -2.23307578892655734e-01_dp,
      2.96460137564761618e-03_dp * DAYS_PER_YEAR, 2.37847173959480950e-03_dp * DAYS_PER_YEAR, -2.96589568540237556e-05_dp * DAYS_PER_YEAR,
      4.36624404335156298e-05_dp * SOLAR_MASS)

    ! Neptune
    bodies(5) = planet(
      1.53796971148509165e+01_dp, -2.59193146099879641e+01_dp, 1.79258772950371181e-01_dp,
      2.68067772490389322e-03_dp * DAYS_PER_YEAR, 1.62824170038242295e-03_dp * DAYS_PER_YEAR, -9.51592254519715870e-05_dp * DAYS_PER_YEAR,
      5.15138902046611451e-05_dp * SOLAR_MASS)
  end subroutine init_solar_system

  ! Offset momentum of the sun
  subroutine offset_momentum(bodies)
    type(planet), dimension(5), intent(inout) :: bodies
    real(dp) :: px, py, pz
    integer :: i

    px = 0.0_dp
    py = 0.0_dp
    pz = 0.0_dp

    do i = 1, size(bodies)
      px = px + bodies(i)%vx * bodies(i)%mass
      py = py + bodies(i)%vy * bodies(i)%mass
      pz = pz + bodies(i)%vz * bodies(i)%mass
    end do

    bodies(1)%vx = -px / SOLAR_MASS
    bodies(1)%vy = -py / SOLAR_MASS
    bodies(1)%vz = -pz / SOLAR_MASS
  end subroutine offset_momentum

  ! Calculate energy of the system
  function energy(bodies) result(e)
    type(planet), dimension(5), intent(in) :: bodies
    real(dp) :: e, dx, dy, dz, distance
    integer :: i, j

    e = 0.0_dp

    do i = 1, size(bodies)
      e = e + 0.5_dp * bodies(i)%mass * (
          bodies(i)%vx * bodies(i)%vx + &
          bodies(i)%vy * bodies(i)%vy + &
          bodies(i)%vz * bodies(i)%vz)

      do j = i+1, size(bodies)
        dx = bodies(i)%x - bodies(j)%x
        dy = bodies(i)%y - bodies(j)%y
        dz = bodies(i)%z - bodies(j)%z

        distance = sqrt(dx*dx + dy*dy + dz*dz)
        e = e - (bodies(i)%mass * bodies(j)%mass) / distance
      end do
    end do
  end function energy

  ! Advance simulation by dt
  subroutine advance(bodies, dt)
    type(planet), dimension(5), intent(inout) :: bodies
    real(dp), intent(in) :: dt
    real(dp) :: dx, dy, dz, distance, mag, b_mass_mag, b2_mass_mag
    integer :: i, j

    ! Update velocities
    do i = 1, size(bodies)
      do j = i+1, size(bodies)
        dx = bodies(i)%x - bodies(j)%x
        dy = bodies(i)%y - bodies(j)%y
        dz = bodies(i)%z - bodies(j)%z

        distance = sqrt(dx*dx + dy*dy + dz*dz)
        mag = dt / (distance * distance * distance)

        b_mass_mag = bodies(i)%mass * mag
        b2_mass_mag = bodies(j)%mass * mag

        bodies(i)%vx = bodies(i)%vx - dx * b2_mass_mag
        bodies(i)%vy = bodies(i)%vy - dy * b2_mass_mag
        bodies(i)%vz = bodies(i)%vz - dz * b2_mass_mag

        bodies(j)%vx = bodies(j)%vx + dx * b_mass_mag
        bodies(j)%vy = bodies(j)%vy + dy * b_mass_mag
        bodies(j)%vz = bodies(j)%vz + dz * b_mass_mag
      end do
    end do

    ! Update positions
    do i = 1, size(bodies)
      bodies(i)%x = bodies(i)%x + dt * bodies(i)%vx
      bodies(i)%y = bodies(i)%y + dt * bodies(i)%vy
      bodies(i)%z = bodies(i)%z + dt * bodies(i)%vz
    end do
  end subroutine advance

end program n_bodies