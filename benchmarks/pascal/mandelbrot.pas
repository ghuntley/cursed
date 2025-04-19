program Mandelbrot;

{$mode objfpc}{$H+}

uses
  SysUtils, DateUtils;

const
  WIDTH = 800;
  HEIGHT = 800;
  MAX_ITERATIONS = 100;

type
  TResultArray = array of array of Integer;

{ Calculate the Mandelbrot set }
function CalculateMandelbrot(MaxIterations: Integer): TResultArray;
var
  x, y, iteration: Integer;
  cx, cy, zx, zy, temp: Double;
begin
  SetLength(Result, HEIGHT, WIDTH);
  
  for y := 0 to HEIGHT - 1 do
  begin
    for x := 0 to WIDTH - 1 do
    begin
      cx := (x - WIDTH / 2.0) * 4.0 / WIDTH;
      cy := (y - HEIGHT / 2.0) * 4.0 / HEIGHT;
      
      zx := 0.0;
      zy := 0.0;
      iteration := 0;
      
      while ((zx * zx + zy * zy <= 4.0) and (iteration < MaxIterations)) do
      begin
        temp := zx * zx - zy * zy + cx;
        zy := 2.0 * zx * zy + cy;
        zx := temp;
        Inc(iteration);
      end;
      
      Result[y, x] := iteration;
    end;
  end;
end;

{ Count non-black pixels in the result }
function CountNonBlack(const Result: TResultArray; MaxIterations: Integer): Integer;
var
  x, y: Integer;
begin
  CountNonBlack := 0;
  
  for y := 0 to HEIGHT - 1 do
  begin
    for x := 0 to WIDTH - 1 do
    begin
      if Result[y, x] < MaxIterations then
        Inc(CountNonBlack);
    end;
  end;
end;

var
  Result: TResultArray;
  Count: Integer;
  StartTime: TDateTime;
begin
  StartTime := Now;
  
  Result := CalculateMandelbrot(MAX_ITERATIONS);
  Count := CountNonBlack(Result, MAX_ITERATIONS);
  
  WriteLn('Mandelbrot set calculation finished.');
  WriteLn('Image size: ', WIDTH, ' x ', HEIGHT);
  WriteLn('Maximum iterations: ', MAX_ITERATIONS);
  WriteLn('Non-black pixels: ', Count);
  
  WriteLn('Time taken: ', MilliSecondsBetween(Now, StartTime), ' ms');
  
  // Approximate memory usage - this is a simplification
  WriteLn('Memory used: ', GetHeapStatus.TotalAllocated div 1024, ' KB');
end.