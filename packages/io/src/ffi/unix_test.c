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
    setExitHandler();
    initialize();

    while (buf != 'q') {
        getChar(&buf);
        printf("Typed character: %c\n", buf);
        usleep(1000);       
    }

    terminate();
    return 0;
}
