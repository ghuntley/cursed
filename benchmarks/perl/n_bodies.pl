#!/usr/bin/env perl
# N-body simulation benchmark adapted from The Computer Language Benchmarks Game

use strict;
use warnings;
use Time::HiRes qw(gettimeofday tv_interval);
use Math::Trig;

use constant PI => 3.141592653589793;
use constant SOLAR_MASS => 4.0 * PI * PI;
use constant DAYS_PER_YEAR => 365.24;

# Planet structure
package Planet;

sub new {
    my ($class, $x, $y, $z, $vx, $vy, $vz, $mass) = @_;
    return bless {
        x => $x, y => $y, z => $z,
        vx => $vx, vy => $vy, vz => $vz,
        mass => $mass
    }, $class;
}

package main;

# Initialize solar system
sub init_solar_system {
    my @bodies;
    
    # Sun
    push @bodies, Planet->new(
        0.0, 0.0, 0.0,
        0.0, 0.0, 0.0,
        SOLAR_MASS
    );
    
    # Jupiter
    push @bodies, Planet->new(
        4.84143144246472090e+00,
        -1.16032004402742839e+00,
        -1.03622044471123109e-01,
        1.66007664274403694e-03 * DAYS_PER_YEAR,
        7.69901118419740425e-03 * DAYS_PER_YEAR,
        -6.90460016972063023e-05 * DAYS_PER_YEAR,
        9.54791938424326609e-04 * SOLAR_MASS
    );
    
    # Saturn
    push @bodies, Planet->new(
        8.34336671824457987e+00,
        4.12479856412430479e+00,
        -4.03523417114321381e-01,
        -2.76742510726862411e-03 * DAYS_PER_YEAR,
        4.99852801234917238e-03 * DAYS_PER_YEAR,
        2.30417297573763929e-05 * DAYS_PER_YEAR,
        2.85885980666130812e-04 * SOLAR_MASS
    );
    
    # Uranus
    push @bodies, Planet->new(
        1.28943695621391310e+01,
        -1.51111514016986312e+01,
        -2.23307578892655734e-01,
        2.96460137564761618e-03 * DAYS_PER_YEAR,
        2.37847173959480950e-03 * DAYS_PER_YEAR,
        -2.96589568540237556e-05 * DAYS_PER_YEAR,
        4.36624404335156298e-05 * SOLAR_MASS
    );
    
    # Neptune
    push @bodies, Planet->new(
        1.53796971148509165e+01,
        -2.59193146099879641e+01,
        1.79258772950371181e-01,
        2.68067772490389322e-03 * DAYS_PER_YEAR,
        1.62824170038242295e-03 * DAYS_PER_YEAR,
        -9.51592254519715870e-05 * DAYS_PER_YEAR,
        5.15138902046611451e-05 * SOLAR_MASS
    );
    
    return @bodies;
}

# Offset momentum of the sun
sub offset_momentum {
    my $bodies = shift;
    my ($px, $py, $pz) = (0.0, 0.0, 0.0);
    
    for my $body (@$bodies) {
        $px += $body->{vx} * $body->{mass};
        $py += $body->{vy} * $body->{mass};
        $pz += $body->{vz} * $body->{mass};
    }
    
    $bodies->[0]{vx} = -$px / SOLAR_MASS;
    $bodies->[0]{vy} = -$py / SOLAR_MASS;
    $bodies->[0]{vz} = -$pz / SOLAR_MASS;
}

# Calculate energy of the system
sub energy {
    my $bodies = shift;
    my $e = 0.0;
    
    for (my $i = 0; $i < @$bodies; $i++) {
        my $b = $bodies->[$i];
        $e += 0.5 * $b->{mass} * ($b->{vx}*$b->{vx} + $b->{vy}*$b->{vy} + $b->{vz}*$b->{vz});
        
        for (my $j = $i + 1; $j < @$bodies; $j++) {
            my $b2 = $bodies->[$j];
            my $dx = $b->{x} - $b2->{x};
            my $dy = $b->{y} - $b2->{y};
            my $dz = $b->{z} - $b2->{z};
            my $distance = sqrt($dx*$dx + $dy*$dy + $dz*$dz);
            $e -= ($b->{mass} * $b2->{mass}) / $distance;
        }
    }
    
    return $e;
}

# Advance simulation by dt
sub advance {
    my ($bodies, $dt) = @_;
    
    for (my $i = 0; $i < @$bodies; $i++) {
        my $b = $bodies->[$i];
        
        for (my $j = $i + 1; $j < @$bodies; $j++) {
            my $b2 = $bodies->[$j];
            my $dx = $b->{x} - $b2->{x};
            my $dy = $b->{y} - $b2->{y};
            my $dz = $b->{z} - $b2->{z};
            
            my $distance = sqrt($dx*$dx + $dy*$dy + $dz*$dz);
            my $mag = $dt / ($distance * $distance * $distance);
            
            my $b_mass_mag = $b->{mass} * $mag;
            my $b2_mass_mag = $b2->{mass} * $mag;
            
            $b->{vx} -= $dx * $b2_mass_mag;
            $b->{vy} -= $dy * $b2_mass_mag;
            $b->{vz} -= $dz * $b2_mass_mag;
            
            $b2->{vx} += $dx * $b_mass_mag;
            $b2->{vy} += $dy * $b_mass_mag;
            $b2->{vz} += $dz * $b_mass_mag;
        }
    }
    
    for my $body (@$bodies) {
        $body->{x} += $dt * $body->{vx};
        $body->{y} += $dt * $body->{vy};
        $body->{z} += $dt * $body->{vz};
    }
}

# Main
my $n = 1000000; # Number of iterations
my @bodies = init_solar_system();
my $start_time = [gettimeofday];

offset_momentum(\@bodies);
my $initial_energy = energy(\@bodies);
printf "Initial energy: %.9f\n", $initial_energy;

for (my $i = 0; $i < $n; $i++) {
    advance(\@bodies, 0.01);
}

my $final_energy = energy(\@bodies);
printf "Final energy: %.9f\n", $final_energy;
printf "Energy delta: %.9f\n", $final_energy - $initial_energy;

my $elapsed = tv_interval($start_time) * 1000;
printf "Time taken: %.2f ms\n", $elapsed;

# Perl doesn't have a standard way to get memory usage
print "Memory monitoring not available for Perl implementation\n";