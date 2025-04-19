program BinaryTrees;

{$mode objfpc}{$H+}

uses
  SysUtils, DateUtils;

type
  PTreeNode = ^TTreeNode;
  TTreeNode = record
    Item: Integer;
    Left: PTreeNode;
    Right: PTreeNode;
  end;

// Create a new tree with the given item value at the root
function NewTree(Item, Depth: Integer): PTreeNode;
begin
  New(Result);
  Result^.Item := Item;
  
  if Depth > 0 then
  begin
    Result^.Left := NewTree(2 * Item - 1, Depth - 1);
    Result^.Right := NewTree(2 * Item, Depth - 1);
  end
  else
  begin
    Result^.Left := nil;
    Result^.Right := nil;
  end;
end;

// Check the tree and return a checksum
function CheckTree(Tree: PTreeNode): Integer;
begin
  if Tree = nil then
    Result := 0
  else if Tree^.Left = nil then
    Result := Tree^.Item
  else
    Result := Tree^.Item + CheckTree(Tree^.Left) - CheckTree(Tree^.Right);
end;

// Free the tree
procedure DeleteTree(Tree: PTreeNode);
begin
  if Tree = nil then
    Exit;
    
  if Tree^.Left <> nil then
    DeleteTree(Tree^.Left);
    
  if Tree^.Right <> nil then
    DeleteTree(Tree^.Right);
    
  Dispose(Tree);
end;

var
  MinDepth, MaxDepth, StretchDepth, Depth, Iterations, I, Result, Check: Integer;
  StartTime: TDateTime;
  StretchTree, LongLivedTree, TreeA, TreeB: PTreeNode;

begin
  MinDepth := 4;
  MaxDepth := 12;
  StretchDepth := MaxDepth + 1;
  StartTime := Now;
  
  // Allocate and check a big tree
  StretchTree := NewTree(0, StretchDepth);
  WriteLn('stretch tree of depth ', StretchDepth, ' check: ', CheckTree(StretchTree));
  DeleteTree(StretchTree);
  
  // Allocate a long-lived binary tree
  LongLivedTree := NewTree(0, MaxDepth);
  
  // Check trees of increasing depth
  Depth := MinDepth;
  while Depth <= MaxDepth do
  begin
    Iterations := 1 shl (MaxDepth - Depth + MinDepth);
    Result := 0;
    
    for I := 1 to Iterations do
    begin
      TreeA := NewTree(I, Depth);
      TreeB := NewTree(-I, Depth);
      Result := Result + CheckTree(TreeA) + CheckTree(TreeB);
      DeleteTree(TreeA);
      DeleteTree(TreeB);
    end;
    
    WriteLn(Iterations * 2, ' trees of depth ', Depth, ' check: ', Result);
    Depth := Depth + 2;
  end;
  
  // Check the long-lived tree last
  WriteLn('long lived tree of depth ', MaxDepth, ' check: ', CheckTree(LongLivedTree));
  DeleteTree(LongLivedTree);
  
  WriteLn('Time taken: ', MilliSecondsBetween(Now, StartTime), ' ms');
  
  // Approximate memory usage - this is a simplification
  WriteLn('Memory used: ', GetHeapStatus.TotalAllocated div 1024, ' KB');
end.