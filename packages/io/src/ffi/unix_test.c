/**
 * Just some code to test that the IO C-library does indeed do SOMETHING right.
 * 
 * gcc unix_test.c unix.c -o unix_test.bin && ./unix_test.bin
 */

#include <stdio.h>
#include <unistd.h>

#include "unix.h"

int main() {
    char buf = 0;
    initialize();

    while (getChar(&buf) && buf != 'q') {
        printf("Typed character: %c\n", buf);
    }

    terminate();
    return 0;
}
