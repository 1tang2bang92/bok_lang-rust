#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int64_t fibo(int64_t);

int main() {
    long long input = 0;
    for (;;) {
        scanf("%lld", &input);
        printf("%ld\n", fibo(input));
    }
    
    return 0;
}