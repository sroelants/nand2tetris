// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.


// Idea: Run a loop R0 times in which we add R1 to itself, storing the
// intermediate results in R2. After every iteration, we decrement the
// counter from i=R0 to 0. When i=0 we jump to the end.


  // Set counter i to value in R0
  @R0
  D=M
  @i
  M=D

  // Initialize intermediate result to 0
  @R2
  M=0

(LOOP)
  // Loop condition
  @i
  D=M
  @END
  D;JEQ // If i == 0, GOTO END

  // Loop body
  @R1 
  D=M // Load value to add
  @R2 // Load temporary result
  M=D+M // Add R1 to the temporary result (stored in R2)

  // Decrement counter
  @i
  M=M-1
  @LOOP
  0;JMP // GOTO LOOP

// End program with infinite loop.
(END)
  @END
  0;JMP


