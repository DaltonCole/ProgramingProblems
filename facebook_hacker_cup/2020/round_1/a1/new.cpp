#include <iostream>
#include <vector>
#include <algorithm>
#include <utility>

using namespace std;

const int MOD = 1000000007;

int big_mod1(int a, int b, int mod) {
    int res = 0;
    a = a % mod;
    while(b > 0) {
        if(b % 2 == 1) {
            res = (res + a) % mod;
        }
        a = (a * 2) % mod;
        b /= 2;
    }
    return res % mod;
}

int big_mod2(const int a, const int z, int m) {
    const int r = m % a;
    const int q = m / a;
    m = a * q + r;
    //cout << a << " " << z << " " << q << " " << r << endl;
    //cout << (a * (z % q)) << endl;
    //cout << (r * (z / q)) << endl;
    int result = (a * (z % q)) - (r * (z / q));
    while(result < 0) {
        result += MOD;
    }
    while(result >= MOD) {
        result -= MOD;
    }
    //cout << result << endl;
    return result;
}

int big_mod(long long a, long long b, long long m) {
    return ((a % m) * (b % m)) % m;
}

int perimetric() {
    int n, k, w, tmp;
    vector<int> l;
    vector<int> h;
    int al, bl, cl, dl;
    int ah, bh, ch, dh;

    // Read input
    cin >> n >> k >> w;
    for(int i = 0; i < k; i++) {
        cin >> tmp;
        l.push_back(tmp);
    }
    cin >> al >> bl >> cl >> dl;
    for(int i = 0; i < k; i++) {
        cin >> tmp;
        h.push_back(tmp);
    }
    cin >> ah >> bh >> ch >> dh;
    
    // Calculate remaining l and h
    for(int i = k; i < n; i++) {
        // l
        tmp = ((al * l[i-2] + bl * l[i-1] + cl) % dl) + 1;
        l.push_back(tmp);
        tmp = ((ah * h[i-2] + bh * h[i-1] + ch) % dh) + 1;
        h.push_back(tmp);
    }

    // Make bottom-left and top-right coordinates
    int left = 0;
    int right = 0;
    // Total and previous perimeters
    int total = 1;
    int previous_total = 0;

    for(uint i = 0; i < l.size(); i++) {
        // Rooms intersect/touch
        if(l[i] <= right) {
            // Remove the last wall
            previous_total -= h[i-1];
            // Move left to be where right was
            left = right;
            // New right most wall
            right = (l[i] + w);
            // Add top and bottom walls
            previous_total += (2 * (right - left));
            // Add right wall
            previous_total += h[i];
            // Add remainder of old left wall
            previous_total += abs(h[i] - h[i-1]);
        }
        // Rooms DON'T intersect
        else {
            // New room, add top/bottom and left/right walls
            previous_total += (2 * w) + (2 * h[i]);
            left = l[i];
            right = l[i] + w;
        }
        cout << "P: " << previous_total << endl;
        //cout << "Total: " << endl;
        //cout << total << endl;
        //cout << MOD << endl;
        total = big_mod(total, previous_total, MOD);
        //cout << total << endl;
    }

    return total;
}


int main() {
    int cases;
    int tmp;

    cin >> cases;

    for(int i = 0; i < cases; i++) {
        tmp = perimetric();
        cout << "Case #" << (i+1) << ": " << tmp << endl;
    }

    return 0;
}
