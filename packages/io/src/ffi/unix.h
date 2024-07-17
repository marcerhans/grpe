#pragma once

#include <stdlib.h> // exit,atexit.
#include <termios.h> // struct termios,tcgetattr,tcsetattr,ECHO,TCSAFLUSH,etc.
#include <unistd.h> // read,STDIN_FILENO.
#include <stdio.h> // perror, NULL.
#include <errno.h> // errno macro,EAGAIN

const char* DEFAULT_ERROR_MSG = "Terminal I/O Failed";
struct termios orig_termios;

void error(const char* s) {
  if (s == NULL) {
    s = DEFAULT_ERROR_MSG;
  }

  perror(s);
  exit(1);
}

void disablePartialRawMode() {
  if (tcsetattr(STDIN_FILENO, TCSAFLUSH, &orig_termios) == -1) {
    error(NULL);
  }
}

/**
 * Configures terminal settings to be more interactive.
 * 
 * Disables:
 * ICRNL  - Disable interpretation of newline/carriage return.
 * ECHO   - Disable echoing of characters.
 * ICANON - Disable 'canonical mode' meaning inputs
 *          are read without pressing 'Enter'.
 */
void enablePartialRawMode() {
  tcgetattr(STDIN_FILENO, &orig_termios);
  struct termios raw = orig_termios;
  // raw.c_iflag &= ~(ICRNL);
  raw.c_lflag &= ~(ECHO | ICANON);
  raw.c_cc[VMIN] = 1; // Wait for at least 1 byte(s) to have been written.
  raw.c_cc[VTIME] = 1; // Time to wait for input in deciseconds (i.e. 1/10:th seconds).
  tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw);
}

void setExitHandler() {
  atexit(disablePartialRawMode);
}

/**
 * Just an example.
 */
#ifdef FALSE
int main() {
  setExitHandler();
  enablePartialRawMode();

  char c;

  while (1) {
    if (read(STDIN_FILENO, &c, 1) == -1) {
      error(NULL);
    }

    if (c == 'q') {
      break;
    }
  }

  return 0;
}
#endif