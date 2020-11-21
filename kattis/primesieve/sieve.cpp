// Sieve of Eratosthenes
#include <iostream>
#include <bitset>
using namespace std;

#define N 100000010

bitset<N> primes;

int main() {
    long n, q, temp;

    //bitset<100000010> primes;
    
    long total_primes = 0;

    cin >> n >> q;

    primes[0] = true;
    primes[1] = true;

    for(long i = 2; i < 10011; i++) {
        if(primes[i] == false) {
            for(long j = i * 2; j <= n; j += i) {
                primes[j] = true;
            }
        }
    }
    
    total_primes = primes.count();

    cout << (n - total_primes + 1) << endl;

    for(long i = 0; i < q; i++) {
        cin >> temp;
        cout << !primes[temp] << endl;
    }
    
    return 0;
}