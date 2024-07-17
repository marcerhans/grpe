#pragma once

void disablePartialRawMode();

/**
 * Configures terminal settings to be more interactive.
 * 
 * Disables:
 * ICRNL  - Disable interpretation of newline/carriage return.
 * ECHO   - Disable echoing of characters.
 * ICANON - Disable 'canonical mode' meaning inputs
 *          are read without pressing 'Enter'.
 */
void enablePartialRawMode();

void setExitHandler();