program HelloWorld;
{$mode delphi}
uses crt, queue, StrUtils, sysutils, classes, grids;

type
   prevType = array of integer;
   // neighbors = array of integer;
   // TStringArray = array of string;

var 
   i, j, n: integer;
   letterGrid: strGrid;
   heightGrid: intGrid;
   prev: intArr;


   // procedure solve(s, n:integer);

// function getNeighbors(): neighbors;

//    begin 
//       getNeighbors:=
//    end;



function solve(var lg:strGrid): intArr;
   var
      // node: integer;
      numberOfNodes: integer;
      sum: integer;
      number: integer;
      parents: array of integer;
      i, j, n: integer;
      split: array of AnsiString;


      maxX,maxY,countX, countY: integer;
      sX,sY: integer;
      eX,eY:integer;
      nX,nY:integer;
      s,e: intArr;

      q: tFifo;
      visited: intGrid; //tracks where you have been;
      prev: intArr; // tracks parent nodes
      node: tNode;
      neighbors: intGrid;
      next: intArr;
      nextH: integer;
      currH: integer;
      nextNum:integer;
      currNum:integer;

   begin
      //get the maxX and maxY
      countX:= Length(lg[0]);
      countY:= Length(lg);
      // writeln(maxX, ' by ', maxY);

      // find the S and E
      s:=findSorE(lg,'S');
      e:=findSorE(lg,'E');
      // writeln('s at: ',s[0],' ',s[1]);
      // writeln('e at: ',e[0],' ',e[1]);

      setlength(visited,countY,countX);
      setlength(prev,countY*countX); 
      initQueue(q); 
      enqueue(q,s[0],s[1]);

      for i:=Low(visited) to High(visited) do  
          begin  
            for j:= Low(visited[i]) to High(visited[i]) do  
               visited[i,j]:=0;
          end;  
      visited[s[0]][s[1]]:=1;

      while q.first <> Nil do
         begin
            // writeln(' ');
            // nX:=0;
            // nY:=0;
            SetLength(neighbors,0,0);
            dequeue(q,nX,nY);
            // writeln('queue length: ', q.length);
            neighbors := getNeighbors(nX,nY,countX-1,countY-1);
            // printGrid(lg);
            // printGrid(visited);
            // writeln('node: ',nX,', ',nY, ', ',lg[nY][nX], ': ',getGridNumberFromXY(nX,nY,countX));
            // writeln('neighbors: ');
            // printGrid(neighbors);
            currH:= letterToHeight(lg[nY][nX]);
            currNum:= getGridNumberFromXY(nX,nY,countX);
            // prev[currNum];

            for next in neighbors do
               begin
                  // writeln('-- n: ',next[0],', ',next[1], ', ', lg[next[1]][next[0]], ', ',visited[next[1]][next[0]]);
                  // printGrid(visited);
                  // writeln(visited[next[1]][next[0]]);
                  // writeln('after');
                  nextNum:= getGridNumberFromXY(next[0],next[1],countX);

                  if(visited[next[1]][next[0]]=0) then
                     begin 
                        // writeln('  -- not visited');
                        nextH:= letterToHeight(lg[next[1]][next[0]]);
                        // writeln('next: ',lg[next[0]][next[1]]);
                        // writeln('  -- heights: ',nextH,',',currH);
                        if( nextH <= currH+1 ) then
                           begin
                              // writeln(lg[next[0]][next[1]]);
                              // writeln('  --  -- postvisit');
                              enqueue(q,next[0],next[1]);
                              visited[next[1],next[0]]:=1;
                              prev[nextNum] := currNum;

                              // writeln('queue length: ', q.length);
                              // writeln('  --  -- postappend');
                           end;
                     end;
               end;
            // writeln('queue length: ', q.length);

         end;
      solve := prev;
   end;



procedure reconstructPath(prev:intArr;lg:strGrid);
   var 
      s,e, nextXY: intArr;
      maxX, countX: integer;
      sN: integer;
      curr: integer;
      counter: integer;
      next: integer;

   begin
      e:=findSorE(lg,'E');
      s:=findSorE(lg,'S');
      countX:= length(lg[0]); // the count
      maxX:= countX-1;
      // writeln('countX ', countX);
      // writeln('PATH to ', e[0],', ',e[1],', ', getGridNumberFromXY(e[0],e[1],countX));
      // printArray(prev);
      curr:=getGridNumberFromXY(e[0],e[1],countX);
      counter:=0;
      // printGrid(lg);
      sN:=getGridNumberFromXY(s[0],s[1],countX);
      while curr <> sN do
         begin
            next:= prev[curr];
            nextXY:= getXYFromGridNumber(next, countX);
            // writeln(nextXY[0], ',', nextXY[1],',   ',next);
            counter := counter+1;
            // writeln('curr: ', curr, ', #', counter,',  next: ',next, ',    ',nextXY[0],',',nextXY[1], lg[nextXY[1]][nextXY[0]]);
            curr:= next;
         end;
      // writeln(length(prev))
      writeln('PATH to ', e[0],', ',e[1],', ', getGridNumberFromXY(e[0],e[1],countX), ' counter: ',counter );

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
      theBs: intArr;
   begin
      //reading from file
      assign(input, 'input2.txt');
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
            // printArray(row);
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

   for i:=Low(letterGrid) to High(letterGrid) do  
      begin  
      for j:= Low(letterGrid[i]) to High(letterGrid[i]) do 
         if(letterGrid[i,j]='b') then
            begin
               writeln(i,',',j,' ', letterGrid[i,j]); 
               letterGrid[i,j]:='S';
               prev:=solve(letterGrid);
               reconstructPath(prev, letterGrid);
               setlength(prev,0);
            end;
      end;  
end.
