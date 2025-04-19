program NBodies;

{$mode objfpc}{$H+}

uses
  SysUtils, DateUtils, Math;

const
  PI = 3.141592653589793;
  SOLAR_MASS = 4.0 * PI * PI;
  DAYS_PER_YEAR = 365.24;

type
  TPlanet = record
    x, y, z: Double;
    vx, vy, vz: Double;
    mass: Double;
  end;
  TPlanetArray = array of TPlanet;

{ Initialize solar system }
function InitSolarSystem: TPlanetArray;
begin
  SetLength(Result, 5);
  
  // Sun
  Result[0].x := 0.0;
  Result[0].y := 0.0;
  Result[0].z := 0.0;
  Result[0].vx := 0.0;
  Result[0].vy := 0.0;
  Result[0].vz := 0.0;
  Result[0].mass := SOLAR_MASS;
  
  // Jupiter
  Result[1].x := 4.84143144246472090e+00;
  Result[1].y := -1.16032004402742839e+00;
  Result[1].z := -1.03622044471123109e-01;
  Result[1].vx := 1.66007664274403694e-03 * DAYS_PER_YEAR;
  Result[1].vy := 7.69901118419740425e-03 * DAYS_PER_YEAR;
  Result[1].vz := -6.90460016972063023e-05 * DAYS_PER_YEAR;
  Result[1].mass := 9.54791938424326609e-04 * SOLAR_MASS;
  
  // Saturn
  Result[2].x := 8.34336671824457987e+00;
  Result[2].y := 4.12479856412430479e+00;
  Result[2].z := -4.03523417114321381e-01;
  Result[2].vx := -2.76742510726862411e-03 * DAYS_PER_YEAR;
  Result[2].vy := 4.99852801234917238e-03 * DAYS_PER_YEAR;
  Result[2].vz := 2.30417297573763929e-05 * DAYS_PER_YEAR;
  Result[2].mass := 2.85885980666130812e-04 * SOLAR_MASS;
  
  // Uranus
  Result[3].x := 1.28943695621391310e+01;
  Result[3].y := -1.51111514016986312e+01;
  Result[3].z := -2.23307578892655734e-01;
  Result[3].vx := 2.96460137564761618e-03 * DAYS_PER_YEAR;
  Result[3].vy := 2.37847173959480950e-03 * DAYS_PER_YEAR;
  Result[3].vz := -2.96589568540237556e-05 * DAYS_PER_YEAR;
  Result[3].mass := 4.36624404335156298e-05 * SOLAR_MASS;
  
  // Neptune
  Result[4].x := 1.53796971148509165e+01;
  Result[4].y := -2.59193146099879641e+01;
  Result[4].z := 1.79258772950371181e-01;
  Result[4].vx := 2.68067772490389322e-03 * DAYS_PER_YEAR;
  Result[4].vy := 1.62824170038242295e-03 * DAYS_PER_YEAR;
  Result[4].vz := -9.51592254519715870e-05 * DAYS_PER_YEAR;
  Result[4].mass := 5.15138902046611451e-05 * SOLAR_MASS;
end;

{ Offset momentum of the sun }
procedure OffsetMomentum(var Bodies: TPlanetArray);
var
  px, py, pz: Double;
  i: Integer;
begin
  px := 0.0;
  py := 0.0;
  pz := 0.0;
  
  for i := 0 to Length(Bodies) - 1 do
  begin
    px := px + Bodies[i].vx * Bodies[i].mass;
    py := py + Bodies[i].vy * Bodies[i].mass;
    pz := pz + Bodies[i].vz * Bodies[i].mass;
  end;
  
  Bodies[0].vx := -px / SOLAR_MASS;
  Bodies[0].vy := -py / SOLAR_MASS;
  Bodies[0].vz := -pz / SOLAR_MASS;
end;

{ Calculate energy of the system }
function Energy(const Bodies: TPlanetArray): Double;
var
  e, dx, dy, dz, distance: Double;
  i, j: Integer;
begin
  e := 0.0;
  
  for i := 0 to Length(Bodies) - 1 do
  begin
    e := e + 0.5 * Bodies[i].mass * (
         Bodies[i].vx * Bodies[i].vx + 
         Bodies[i].vy * Bodies[i].vy + 
         Bodies[i].vz * Bodies[i].vz);
    
    for j := i + 1 to Length(Bodies) - 1 do
    begin
      dx := Bodies[i].x - Bodies[j].x;
      dy := Bodies[i].y - Bodies[j].y;
      dz := Bodies[i].z - Bodies[j].z;
      
      distance := Sqrt(dx*dx + dy*dy + dz*dz);
      e := e - (Bodies[i].mass * Bodies[j].mass) / distance;
    end;
  end;
  
  Result := e;
end;

{ Advance simulation by dt }
procedure Advance(var Bodies: TPlanetArray; dt: Double);
var
  dx, dy, dz, distance, mag, b_mass_mag, b2_mass_mag: Double;
  i, j: Integer;
begin
  // Update velocities
  for i := 0 to Length(Bodies) - 1 do
  begin
    for j := i + 1 to Length(Bodies) - 1 do
    begin
      dx := Bodies[i].x - Bodies[j].x;
      dy := Bodies[i].y - Bodies[j].y;
      dz := Bodies[i].z - Bodies[j].z;
      
      distance := Sqrt(dx*dx + dy*dy + dz*dz);
      mag := dt / (distance * distance * distance);
      
      b_mass_mag := Bodies[i].mass * mag;
      b2_mass_mag := Bodies[j].mass * mag;
      
      Bodies[i].vx := Bodies[i].vx - dx * b2_mass_mag;
      Bodies[i].vy := Bodies[i].vy - dy * b2_mass_mag;
      Bodies[i].vz := Bodies[i].vz - dz * b2_mass_mag;
      
      Bodies[j].vx := Bodies[j].vx + dx * b_mass_mag;
      Bodies[j].vy := Bodies[j].vy + dy * b_mass_mag;
      Bodies[j].vz := Bodies[j].vz + dz * b_mass_mag;
    end;
  end;
  
  // Update positions
  for i := 0 to Length(Bodies) - 1 do
  begin
    Bodies[i].x := Bodies[i].x + dt * Bodies[i].vx;
    Bodies[i].y := Bodies[i].y + dt * Bodies[i].vy;
    Bodies[i].z := Bodies[i].z + dt * Bodies[i].vz;
  end;
end;

var
  n: Integer;
  Bodies: TPlanetArray;
  StartTime: TDateTime;
  InitialEnergy, FinalEnergy, EnergyDelta: Double;
  i: Integer;
begin
  n := 1000000; // Number of iterations
  Bodies := InitSolarSystem;
  StartTime := Now;
  
  OffsetMomentum(Bodies);
  InitialEnergy := Energy(Bodies);
  WriteLn(Format('Initial energy: %.9f', [InitialEnergy]));
  
  for i := 1 to n do
  begin
    Advance(Bodies, 0.01);
  end;
  
  FinalEnergy := Energy(Bodies);
  EnergyDelta := FinalEnergy - InitialEnergy;
  WriteLn(Format('Final energy: %.9f', [FinalEnergy]));
  WriteLn(Format('Energy delta: %.9f', [EnergyDelta]));
  
  WriteLn('Time taken: ', MilliSecondsBetween(Now, StartTime), ' ms');
  
  // Approximate memory usage - this is a simplification
  WriteLn('Memory used: ', GetHeapStatus.TotalAllocated div 1024, ' KB');
end.