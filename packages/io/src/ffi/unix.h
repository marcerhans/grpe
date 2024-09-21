/**
 * Documentation regarding manipulating the terminal
 * is quite arcane... A lot of convoluted history >.<
 * 
 * This code is only going to be "good enough".
 * Might not be portable. Target terminal is xterm.
 */

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
 * - ISIG (Disable SIGNALs like Ctrl-c and -z.) NOTE: Currently not set.
 * - OPOST (Do not interpret output, like converting '\n' to '\r\n'.) NOTE: Currently not set.
 */
void enablePartialRawMode();

/**
 * Sets an exit-handler.
 * Preferably called before anything else for better cleanup.
 * Only call once!
 */
void setExitHandler();

/**
 * Fetches the most recent input char from stdin. Blocking.
 * 
 * @return True if successful. False if not initialized, on error,
 *         or IO manager has received a SIGINT/SIGTERM.
 * @param buf Buffer for read character.
 */
bool getChar(char * const buf);

/**
 * Initializes threads for maintaining the read/write buffer.
 * 
 * Must be called before calling 'getNextChar'.
 */
void initialize();

/**
 * Terminates the runtime code.
 * 
 * Should be called before finalizing the runtime.
 */
void terminate();