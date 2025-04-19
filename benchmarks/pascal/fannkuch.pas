program Fannkuch;

{$mode objfpc}{$H+}

uses
  SysUtils, DateUtils;

type
  TIntArray = array of Integer;

{ Reverse the first n elements of the array }
procedure Flip(var P: TIntArray; N: Integer);
var
  I, Temp: Integer;
begin
  for I := 0 to N div 2 - 1 do
  begin
    Temp := P[I];
    P[I] := P[N - I - 1];
    P[N - I - 1] := Temp;
  end;
end;

{ Count flips required to flip elements to get back to original order }
function FannkuchRedux(N: Integer): Integer;
var
  P, Perm, Count: TIntArray;
  I, J, Flips, K, Temp, PermCount, Sign, Checksum: Integer;
  Done: Boolean;
  FirstJ: Integer;
begin
  SetLength(P, N);
  SetLength(Perm, N);
  SetLength(Count, N);
  
  // Initialize permutation
  for I := 0 to N - 1 do
    P[I] := I;
  
  PermCount := 0;
  Sign := 1;
  FannkuchRedux := 0;
  Checksum := 0;
  Done := False;
  
  while not Done and (PermCount < 10000) do
  begin
    // Copy permutation to perm
    for I := 0 to N - 1 do
      Perm[I] := P[I] + 1;
    
    if P[0] <> 0 then
    begin
      // Count flips
      for I := 0 to N - 1 do
        Count[I] := 0;
      
      Flips := 0;
      while Perm[0] <> 1 do
      begin
        K := Perm[0] - 1;
        Flip(Perm, K);
        Inc(Flips);
        Perm[0] := K + 1;
      end;
      
      if Flips > FannkuchRedux then
        FannkuchRedux := Flips;
      
      Checksum := Checksum + Sign * Flips;
    end;
    
    // Generate next permutation
    Sign := -Sign;
    J := 1;
    while (J < N) and (P[J-1] >= P[J]) do
      Inc(J);
    
    if J >= N then
    begin
      Done := True;
      Continue;
    end;
    
    Inc(PermCount);
    
    // Flip elements
    for I := 0 to J div 2 - 1 do
    begin
      if I mod 2 = 0 then
      begin
        Temp := P[I];
        P[I] := P[J-I];
        P[J-I] := Temp;
      end
      else
      begin
        Temp := P[I];
        P[I] := P[J-I-1];
        P[J-I-1] := Temp;
      end;
    end;
    
    if J < 2 then
    begin
      // Special case for J < 2
      J := 1;
      for I := 1 to N - 1 do
        if P[I-1] > P[I] then
          J := I + 1;
      
      for I := 0 to J - 2 do
      begin
        K := I;
        Temp := P[I];
        while K < J - 1 do
        begin
          Inc(K);
          P[K-1] := P[K];
        end;
        P[J-1] := Temp;
      end;
    end
    else
    begin
      // Regular case
      Dec(J);
      FirstJ := P[J];
      for I := J downto 1 do
        P[I] := P[I-1];
      P[0] := FirstJ;
    end;
  end;
end;

var
  N, Result: Integer;
  StartTime: TDateTime;
begin
  N := 10;
  StartTime := Now;
  
  Result := FannkuchRedux(N);
  
  WriteLn('Fannkuch(', N, '): ', Result);
  
  WriteLn('Time taken: ', MilliSecondsBetween(Now, StartTime), ' ms');
  
  // Approximate memory usage - this is a simplification
  WriteLn('Memory used: ', GetHeapStatus.TotalAllocated div 1024, ' KB');
end.