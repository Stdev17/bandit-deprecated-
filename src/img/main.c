#include "crc.h"

int main() {

    unsigned char test [5] = {'h', 'e', 'l', 'l', 'o'};
    crc(test, 5, 0);
    return 0;
}