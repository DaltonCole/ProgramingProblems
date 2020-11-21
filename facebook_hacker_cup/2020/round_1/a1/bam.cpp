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

    

    // Previous total perimeter
    int prev_total_perimeter = 0;
    // Current perimeter
    int curr_perimeter = 0;
    // Complete total
    int total = 1;

    // Right coordinate
    int right = 0;
    int left = 0;
    int top = 0;

    for(uint i = 0; i < l.size(); i++) {
        // Same room
        if(l[i] <= right) {
            // Smaller room, add area
            if(h[i] <= top) {
                prev_total_perimeter += (2 * (l[i] + w - right));
                right = l[i] + w; 
            }
            // Bigger room, start new section
            else {
                // Find height of intersection point
                top = 0;
                for(int j = i - 1; j >= 0; j--) {
                    if(l[j] + w < l[i]) {
                        break;
                    }
                    top = max(top, h[j]);
                }
                //cout << top << endl;
                // Remove extra perimeter
                prev_total_perimeter -= ((2 * (right - l[i])) + top);
                left = l[i];
                right = l[i] + w;
                prev_total_perimeter += ((2 * (right - left)) + h[i] + (h[i] - top));
                top = h[i];
            }
        }
        // New room
        else {
            left = l[i];
            right = l[i] + w;
            top = h[i];
            prev_total_perimeter += ((2 * (right - left)) + (2 * top));
        }
        total = big_mod(total, prev_total_perimeter, MOD);
        cout << (i+1) << ": (" << l[i] << ", 0) (" << (l[i] + w) << ", " << h[i] << ")" 
            << " -> " << prev_total_perimeter << endl;
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
