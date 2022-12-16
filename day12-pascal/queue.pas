
Unit queue;
{$mode delphi}

Interface

  Type 
    pNode = ^tNode;
    tNode = Record
      x,y: integer;
      next:  pNode;
    End;

    tFifo = Record
      first, last: pNode;
      length: integer;
    End;
  Procedure initQueue(Var fifo: tFifo);
  Procedure enqueue(Var fifo: tFifo; x,y: integer);
  Function dequeue(Var fifo: tFifo; Var x,y: integer): boolean;

Implementation

  Procedure initQueue(Var fifo: tFifo);
  Begin
    fifo.first := Nil;
    fifo.last := Nil;
    fifo.length := 0;
  End;

  Procedure enqueue(Var fifo: tFifo; x,y: integer);
  Var 
    node: pNode;
  Begin
    // writeln('1');
    new(node);
    // node^.value := value;
    node^.x := x;
    node^.y := y;
    node^.next := Nil;
    If fifo.first = Nil
      Then
        begin
        fifo.first := node;
        fifo.length := 1;
        end
    Else
      begin
        fifo.last^.next := node;
        fifo.length := fifo.length+1;
      end;
    fifo.last := node;
  End;

  Function dequeue(Var fifo: tFifo; Var x,y: integer): boolean;
  Var 
    node: pNode;
  Begin
    If fifo.first = Nil
      Then
        begin
          dequeue := false;
          fifo.length := 0;
        end
    Else
      Begin
        node := fifo.first;
        fifo.first := fifo.first^.next;
        x := node^.x;
        y := node^.y;
        dispose(node);
        dequeue := true;
        fifo.length := fifo.length-1;
      End
  End;

// Procedure testFifo;

// Var 
//   fifo: tFifo;
// Procedure testpop(expectEmpty: boolean; expectedValue: integer);

// Var 
//   i: integer;
// Begin
//   If popFifo(fifo, i)
//     Then
//     If expectEmpty
//       Then
//       writeln('Error! Expected empty, got ', i, '.')
//   Else
//     If i = expectedValue
//       Then
//       writeln('Ok, got ', i, '.')
//   Else
//     writeln('Error! Expected ', expectedValue, ', got ', i, '.')
//   Else
//     If expectEmpty
//       Then
//       writeln('Ok, fifo is empty.')
//   Else
//     writeln('Error! Expected ', expectedValue, ', found fifo empty.')
// End;
// Begin
//   initFifo(fifo);
//   pushFifo(fifo, 2);
//   pushFifo(fifo, 3);
//   pushFifo(fifo, 5);
//   testpop(false, 2);
//   pushFifo(fifo, 7);
//   testpop(false, 3);
//   testpop(false, 5);
//   pushFifo(fifo, 11);
//   testpop(false, 7);
//   testpop(false, 11);
//   pushFifo(fifo, 13);
//   testpop(false, 13);
//   testpop(true, 0);
//   pushFifo(fifo, 17);
//   testpop(false, 17);
//   testpop(true, 0)
// End;

Begin
  // writeln('Testing fifo implementation ...');
  // testFifo;
  // writeln('Testing finished.')
End.
