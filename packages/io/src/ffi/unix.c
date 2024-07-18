#include <stdlib.h> // exit,atexit.
#include <termios.h> // struct termios,tcgetattr,tcsetattr,ECHO,TCSAFLUSH,etc.
#include <unistd.h> // read,STDIN_FILENO.
#include <stdio.h> // perror, NULL.
#include <errno.h> // errno macro,EAGAIN

#include "unix.h"

static const char* DEFAULT_ERROR_MSG = "Terminal I/O Failed";
static struct termios orig_termios;

void error(const char* s) {
  if (s == NULL) {
    s = DEFAULT_ERROR_MSG;
  }

  perror(s);
  exit(1);
}

void disablePartialRawMode() {
  puts("\x1B[?1002l");

  if (tcsetattr(STDIN_FILENO, TCSAFLUSH, &orig_termios) == -1) {
    error(NULL);
  }
}

void enablePartialRawMode() {
  tcgetattr(STDIN_FILENO, &orig_termios);
  struct termios raw = orig_termios;
  raw.c_iflag &= ~(ICRNL | IXON);
  raw.c_lflag &= ~(ECHO | ICANON | IEXTEN | ISIG );
  raw.c_cc[VMIN] = 0; // Only return from read when at least one character is ready.
  raw.c_cc[VTIME] = 1; // Time to wait for input in deciseconds (i.e. 1/10:th seconds). Zero is infinite.
  tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw);

  // Also enable mouse tracking via ANSI escape codes (xterm).
  puts("\x1B[?1002h"); // Track button presses + movement while pressed.
}

void setExitHandler() {
  atexit(disablePartialRawMode);
}

bool getNextChar(char * const buf) {
    return read(STDIN_FILENO, buf, 1) == 1;
}