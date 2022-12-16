program HelloWorld;
{$mode delphi}
uses crt, queue, StrUtils, sysutils, classes, grids;

type
   prevType = array of integer;
   // neighbors = array of integer;
   // TStringArray = array of string;

var 
   q: tFifo;
   visited: array of integer;
   heights: array of integer;
   prev: intGrid;
   numberOfNodes: integer;
   sum: integer;
   number: integer;
   parents: array of integer;
   i, j, n: integer;
   split: array of AnsiString;
   SY: TStringList;
   S: string;
   A: TStringArray;


   letterGrid: strGrid;
   heightGrid: intGrid;


   // procedure solve(s, n:integer);

// function getNeighbors(): neighbors;

//    begin 
//       getNeighbors:=
//    end;



function solve(var lg:strGrid): intGrid;
   var
      // node: integer;
      maxX,maxY: integer;
      sX,sY: integer;
      s,e: intArr;
      eX,eY:integer;
      nX,nY:integer;
      q: tFifo;
      visited: intGrid; //tracks where you have been;
      prev: intGrid; // tracks parent nodes
      node: tNode;
      neighbors: intGrid;
      next: intArr;
      nextH: integer;
      currH: integer;
      nextNum:integer;
      currNum:integer;

   begin
      //get the maxX and maxY
      maxX:= Length(lg[0]);
      maxY:= Length(lg);
      writeln(maxX, ' by ', maxY);

      // find the S and E
      s:=findSorE(lg,'S');
      e:=findSorE(lg,'E');
      writeln('s at: ',s[0],' ',s[1]);
      writeln('e at: ',e[0],' ',e[1]);

      setlength(visited,maxY,maxX);
      setlength(prev,maxY,maxX);  
      enqueue(q,s[0],s[1]);

      for i:=Low(visited) to High(visited) do  
          begin  
            for j:= Low(visited[i]) to High(visited[i]) do  
               visited[i,j]:=0;
          end;  
      visited[s[0]][s[1]]:=1;

      while q.first <> Nil do
         begin
            writeln(' ');
            // nX:=0;
            // nY:=0;
            SetLength(neighbors,0,0);
            dequeue(q,nX,nY);
            // writeln('queue length: ', q.length);
            neighbors := getNeighbors(nX,nY,maxX-1,maxY-1);
            printGrid(lg);
            printGrid(visited);
            writeln('node: ',nX,', ',nY, ', ',lg[nY][nX]);
            writeln('neighbors: ');
            // printGrid(neighbors);
            currH:= letterToHeight(lg[nY][nX]);
            currNum:= getGridNumberFromXY(nY,nX,maxX);
            // prev[currNum];

            for next in neighbors do
               begin
                  writeln('-- n: ',next[0],', ',next[1], ', ', lg[next[1]][next[0]], ', ',visited[next[1]][next[0]]);
                  // printGrid(visited);
                  // writeln(visited[next[1]][next[0]]);
                  // writeln('after');
                  nextNum:= getGridNumberFromXY(next[0],next[1],maxX);

                  if(visited[next[1]][next[0]]=0) then
                     begin 
                        writeln('  -- not visited');
                        nextH:= letterToHeight(lg[next[1]][next[0]]);
                        // writeln('next: ',lg[next[0]][next[1]]);
                        writeln('  -- heights: ',nextH,',',currH);
                        if( nextH <= currH+1 ) then
                           begin
                              // writeln(lg[next[0]][next[1]]);
                              // writeln('  --  -- postvisit');
                              enqueue(q,next[0],next[1]);
                              visited[next[1],next[0]]:=1;
                              prev[nextNum] = currNum;

                              // writeln('queue length: ', q.length);
                              // writeln('  --  -- postappend');
                           end;
                     end;
               end;
            writeln('queue length: ', q.length);

         end;
      solve := prev;
   end;



procedure reconstructPath(prev:intGrid;lg:strGrid);
   var 
      s,e: intArr;
   begin
      e:=findSorE(lg,'E');
      writeln('');
      writeln('PATH to ', e[0],', ',e[1]);
      printGrid(prev);
      writeln(length(prev))
   end;

procedure bfs(s,e:integer);
   begin

   end;

function readFromFile():strGrid;
   var 
      //read the line
      input: text;
      singleline: string;
      len: integer;
      row: strArr;
      intRow: intArr;
      thegrid: strGrid;
   begin
      //reading from file
      assign(input, 'start.txt');
      reset(input);
      while not eof(input) do
      
         begin
            readln(input, singleline); // get single line and store in singleline
            len := Length(singleline); // get the length
            for i:= 0 to len-1 do //foreach character, append
               begin
                  gridAppend(row,singleline[i+1]);
                  // gridAppend(intRow,Ord(singleline[i+1])-97);
                  // writeln(letterToHeight(singleline[i+1]));
               end;
            printArray(row);
            // printArray(intRow);
            gridAppend(thegrid,row);
            setlength(row,0); //reset the internal array
            setlength(intRow,0); //reset the internal array

         end;
      close(input);
      readFromFile:=thegrid;
   end;

begin
   letterGrid:=readFromFile();
   writeln(length(letterGrid));

   prev:=solve(letterGrid);
   reconstructPath(prev, letterGrid);
   printArray(prev);
   writeln('in');
end.
