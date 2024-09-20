#include <stdio.h>
#include <unistd.h>

#include "unix.h"

int main() {
    char buf = 0;
    initialize();

    while (buf != 'q') {
        getChar(&buf);
        printf("Typed character: %c\n", buf);
        usleep(1000);
    }

    terminate();
    return 0;
}
