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
    int result = 0;
    initialize();

    while ((result = getChar(&buf, false)) != 1 && buf != 'q') {
        if (result == 2) {
            continue;
        }

        printf("Typed character: %c (binary: ", buf);
        for (int i = 7; i >= 0; --i) {
            printf("%d", (buf >> i) & 0x01);
        }
        printf(")\n");
    }

    terminate();
    return 0;
}
