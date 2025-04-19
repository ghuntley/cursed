program StringProcessing;

{$mode objfpc}{$H+}

uses
  SysUtils, DateUtils, StrUtils, Math;

const
  Chars = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';

{ Process multiple strings }
function ProcessStrings(Count, Size: Integer): string;
var
  I: Integer;
  Str, Processed: string;
begin
  Result := '';
  
  for I := 1 to Count do
  begin
    Str := CreateRandomString(Size);
    Processed := ProcessString(Str);
    Result := Result + Processed;
  end;
end;

{ Create a random string of given size }
function CreateRandomString(Size: Integer): string;
var
  I, Idx: Integer;
begin
  SetLength(Result, Size);
  
  for I := 1 to Size do
  begin
    Idx := Random(Length(Chars)) + 1;
    Result[I] := Chars[Idx];
  end;
end;

{ Process a single string }
function ProcessString(Input: string): string;
var
  I, HalfLen: Integer;
  First, Rest: string;
  Reversed: string;
begin
  Result := Input;
  
  // Replace all vowels with their uppercase version
  Result := StringReplace(Result, 'a', 'A', [rfReplaceAll]);
  Result := StringReplace(Result, 'e', 'E', [rfReplaceAll]);
  Result := StringReplace(Result, 'i', 'I', [rfReplaceAll]);
  Result := StringReplace(Result, 'o', 'O', [rfReplaceAll]);
  Result := StringReplace(Result, 'u', 'U', [rfReplaceAll]);
  
  // Replace all digits with their doubled value
  for I := 0 to 9 do
  begin
    Result := StringReplace(Result, IntToStr(I), IntToStr(I * 2), [rfReplaceAll]);
  end;
  
  // Capitalize the first letter
  if Length(Result) > 0 then
  begin
    First := UpperCase(Result[1]);
    if Length(Result) > 1 then
      Rest := Copy(Result, 2, Length(Result) - 1)
    else
      Rest := '';
    Result := First + Rest;
  end;
  
  // Reverse the string
  Reversed := '';
  for I := Length(Result) downto 1 do
    Reversed := Reversed + Result[I];
  
  // Take the first half of the reversed string
  HalfLen := Length(Reversed) div 2;
  Result := Copy(Reversed, 1, HalfLen);
end;

var
  Small, Medium, Large: string;
  ResultLength: Integer;
  StartTime: TDateTime;
begin
  // Seed the random number generator
  Randomize;
  StartTime := Now;
  
  // Process strings of different sizes
  Small := ProcessStrings(10000, 10);   // 10,000 strings of length 10
  Medium := ProcessStrings(1000, 100);  // 1,000 strings of length 100
  Large := ProcessStrings(100, 1000);   // 100 strings of length 1,000
  
  ResultLength := Length(Small) + Length(Medium) + Length(Large);
  WriteLn('Processed string length: ', ResultLength);
  
  WriteLn('Time taken: ', MilliSecondsBetween(Now, StartTime), ' ms');
  
  // Approximate memory usage - this is a simplification
  WriteLn('Memory used: ', GetHeapStatus.TotalAllocated div 1024, ' KB');
end.