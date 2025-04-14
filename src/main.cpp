#include <vector>
#include <algorithm>

extern "C" int count_primes(int max_number) {
    // Create sieve of Eratosthenes
    std::vector<bool> sieve(max_number + 1, true);
    sieve[0] = sieve[1] = false;

    for (int current = 2; ; ++current) {
        const int square = current * current;
        if (square > max_number) break;
        
        if (sieve[current]) {
            // If current is prime, mark all its multiples as non-prime
            for (int multiple = square; multiple <= max_number; multiple += current) {
                sieve[multiple] = false;
            }
        }
    }
    
    // Count remaining true values (primes)
    return std::count(sieve.begin(), sieve.end(), true);
}
