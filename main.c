#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int64_t foo();

int main() {
    long long input = 0;
    for (;;) {
        //scanf("%lld", &input);
        printf("%ld\n", foo());
    }
    
    return 0;
}