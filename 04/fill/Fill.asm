// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.


// Idea: run an infinite loop that runs over the screen range and sets the 
// value to 1 (black) if the keyboard register is active.

// Initialize outer (infinite) loop
(INFLOOP)
  // Initialize inner loop counter to 0
  @i
  M=0

  (SCREENLOOP)
    // Test counter. If we're over the screen size, restart the screen loop
    // from the start
    @i
    D=M
    @8192
    D=A-D    // Test if 256*32 - @i == 0, if so, GOTO INFLOOP
    @INFLOOP
    D;JEQ

    //Read in keyboard value and set color accordingly
    @KBD
    D=M       // Load the current keyboard value
    @WHITE
    D;JEQ     // Jump to WHITE instructions if keyboard == 0;
    
    // Set fill color to black
    (BLACK)
      @color
      M=-1    // Black
      @FILL
      0;JMP  // Jump to filling instructions

    // Set fill color to white
    (WHITE)
      @color
      M=0     // White
      @FILL
      0;JMP  // Jump to filling instructions

    (FILL)
      // Calculate current location
      @SCREEN
      D=A
      @i
      D=D+M // Location we're currently filling
      @loc
      M=D   // Store location in @loc

      // Load color and set fill @loc with @color
      @color
      D=M
      @loc
      A=M   // Set Address to @loc
      M=D   // Fill with @color (stored in D)

    (INCREMENT)
      @i
      M=M+1
      @SCREENLOOP
      0;JMP

