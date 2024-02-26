#include <cstdint>

extern "C" uint64_t fibonacci(uint64_t n) {
    if (n <= 1) {
        return n;
    }
    uint64_t a = 0, b = 1;
    for (uint64_t i = 2; i <= n; i++) {
        uint64_t c = a + b;
        a = b;
        b = c;
    }
    return b;
}