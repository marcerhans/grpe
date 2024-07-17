#pragma once

#include <stdint.h>
#include <stdbool.h>

/**
 * Disables partial raw mode and restores previous settings.
 */
void disablePartialRawMode();

/**
 * Configures terminal settings to be more interactive.
 * 
 * Disables:
 * - ICRNL (Disable interpretation of newline/carriage return.)
 * - ECHO  (Disable echoing of characters.)
 * - ICANON (Disable 'canonical mode' meaning inputs
 *           are read without pressing 'Enter'.)
 */
void enablePartialRawMode();

/**
 * Sets an exit-handler.
 */
void setExitHandler();

/**
 * Fetches the next input char from stdin.
 * Blocks until character has been read.
 * 
 * @return True if successful. False on error.
 * @param buf Buffer for read character.
 */
bool getNextChar(char * const buf);