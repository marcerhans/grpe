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
 * - IXON (Disable Ctrl-s and -q.)
 * - ECHO  (Disable echoing of characters.)
 * - ICANON (Disable 'canonical mode' meaning inputs
 *           are read without pressing 'Enter'.)
 * - IEXTEN (Disable Ctrl-v.)
 * - ISIG (Disable SIGNALs like Ctrl-c and -z.)
 */
void enablePartialRawMode();

/**
 * Sets an exit-handler.
 */
void setExitHandler();

/**
 * Fetches the next input char from stdin.
 * Blocks for 1/10:th of a second or until character has been read
 * before timing out.
 * 
 * @return True if successful. False on timeout.
 * @param buf Buffer for read character.
 */
bool getNextChar(char * const buf);