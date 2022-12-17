
Unit grids;

Interface

  Type 
    intArr = array of integer;
    strArr = array of string;
    intGrid = array of intArr;
    strGrid = array of strArr;
    // coords = array of integer;
    // neighborSet = array of coords;


  procedure gridAppend(var Arr:strArr;  Value:string          );overload;
  procedure gridAppend(var Arr:intArr;  Value:integer         );overload;
  procedure gridAppend(var Arr:intGrid; Value:intArr );overload;
  procedure gridAppend(var Arr:strGrid; Value:strArr );overload;
  procedure printGrid(a:intGrid);overload;
  procedure printGrid(a:strGrid);overload;
  procedure printArray(a:strArr);overload;
  procedure printArray(a:intArr);overload;
  function letterToHeight(s:string):integer;
  function findSorE(var a:strGrid; se:string):intArr;
  function getNeighbors(x,y,maxX,maxY:integer):intGrid;
  function getGridNumberFromXY(x,y,countX:integer):integer;
  function getXYFromGridNumber(n,countX:integer):intArr;


Implementation
uses sysutils, Math;

  function getNeighbors(x,y,maxX,maxY:integer):intGrid;
    var
      a,b,c,d: array [0..1] of integer;
      neighbors: intGrid;
    begin
        a[0]:=x; //bottom
        a[1]:=y+1;     

        b[0]:=x;// top
        b[1]:=y-1; 

        c[0]:=x-1; //left
        c[1]:=y;

        d[0]:=x+1; //right
        d[1]:=y;
        // writeln('maxX: ',maxX, ' maxY: ',maxY);
        setlength(neighbors,0,0);
        if(x>0) then
          gridAppend(neighbors,c);
        if(y>0) then
          gridAppend(neighbors,b);
        if(x<maxX) then
          gridAppend(neighbors,d);
        if(y<maxY) then
          gridAppend(neighbors,a);
      getNeighbors:=neighbors;
    end;

  function letterToHeight(s:string):integer;
    begin
      if(CompareStr(s,'S')=0) then
        letterToHeight:=0
      else if(CompareStr(s,'E')=0) then
        letterToHeight:=26
      else 
        letterToHeight:=Ord(s[1])-97;
    end;
  procedure gridAppend(var Arr:strArr;Value:string);overload;
    begin
        SetLength(Arr, Length(Arr)+1);
        // writeln(Value);
        Arr[High(Arr)] := Value;
        // writeln(Arr[High(Arr)])
    end;

  procedure gridAppend(var Arr:intArr;Value:integer);overload;
    begin
        SetLength(Arr, Length(Arr)+1);
        Arr[High(Arr)] := Value;
    end;
  procedure gridAppend(var Arr:intGrid; Value:intArr);overload;
    begin
        SetLength(Arr, Length(Arr)+1);
        Arr[High(Arr)] := Value;
    end;
  procedure gridAppend(var Arr:strGrid; Value:strArr);overload;
    begin
        SetLength(Arr, Length(Arr)+1);
        Arr[High(Arr)] := Value;
    end;

  procedure printGrid(a:intGrid);

    var
      i, j: integer;  
    begin  
        for i:=Low(a) to High(a) do  
          begin  
            for j:= Low(a[i]) to High(a[i]) do  
              write(a[i,j]:4,' ');  
            writeln;
          end;  
    end;
  procedure printGrid(a:strGrid);
    var
      i, j: integer;  
    begin  
        for i:=Low(a) to High(a) do  
          begin  
            for j:= Low(a[i]) to High(a[i]) do  
              write(a[i,j]:4,' ');  
            writeln;
          end;  
    end;

  procedure printArray(a:strArr);overload;
    var 
      Len,i :integer;
    begin
      Len := Length(a);
      for i:= 0 to Len-1 do //foreach character, append
         begin
            write(a[i]:4,' ');  
         end;
      writeln;  
    end;

  procedure printArray(a:intArr);overload;
    var 
      Len,i :integer;
    begin
      Len := Length(a);
      for i:= 0 to Len-1 do //foreach character, append
         begin
            write(a[i]:4,' ');  
         end;
      writeln;  
    end;

  function getGridNumberFromXY(x,y,countX:integer):integer;
    //countX should be 4 not 3
    // 0  1  2  3
    // 4  5  6  7
    // 8  9 10 11

    // 0  1  2  3 
    // 4  5  6  7
    // 8  9 10 11

    // g = x + y * countX
    // 5 = 1,1
    // 7 = 3,1
    // 8 = 0,2

    begin
      getGridNumberFromXY:= x + y * countX
    end;

  function getXYFromGridNumber(n,countX:integer):intArr;
    var
      o : intArr;
    begin
      setlength(o,2);
      o[0]:= n mod countX;
      o[1]:= Floor(n/countX);
      // writeln('o[0]: ',o[0],' n: ',n, ' countX: ', countX );
      getXYFromGridNumber:=o;
    end;

  function findSorE(var a:strGrid; se:string):intArr;
    var
      i, j: integer;  
      // o: array [0..1] of integer;
      o: intArr;
    begin  
        setlength(o,2);
        for i:=Low(a) to High(a) do  
          begin  
            for j:= Low(a[i]) to High(a[i]) do 
              begin
                // writeln(a[i,j],i,j) ;
                if(a[i,j]=se) then
                  begin
                    o[0]:=j;
                    o[1]:=i;
                    // writeln(i,j);
                    findSorE:=o;
                  end
              end
            // writeln;
          end;  
      
    end;
Begin
End.
