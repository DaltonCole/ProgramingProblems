#include <iostream>
#include <list>
#include <iterator>

using namespace std;

bool combinable(char a, char b, char c) {
    if(a != b || a != c || b != c) {
        return true;
    }
    return false;
}

char resulting_color(char a, char b, char c) {
    if(a == b || a == c) {
        return a;
    }
    return b;
}

int main() {
    int cases;
    int stones;
    char color;
    list<char> colors;

    cin >> cases;

    for(int i = 0; i < cases; i++) {
        cin >> stones;
        // Get stone colors
        for(int j = 0; j < stones; j++) {
            cin >> color;
            colors.push_back(color);
        }

        auto first = colors.begin();
        auto second = next(first);
        auto third = next(second);

        // Combine stones
        while(colors.size() > 1) {
            if(combinable(*first, *second, *third)) {
                *first = resulting_color(*first, *second, *third);
                colors.erase(second);
                colors.erase(third);

                second = next(first);
                third = next(second);

                if(second == colors.end() || third == colors.end()) {
                    first = colors.begin();
                    second = next(first);
                    third = next(second);
                }
            } else {
                if(third == prev(colors.end()) && combinable(*first, *second, *third) == false) {
                    break;
                }
                first++;
                second++;
                third++;
            }
        }



        cout << "Case #" << (i + 1) << ": ";
        if(colors.size() == 1) { cout << 'Y' << endl;} 
        else { cout << 'N' << endl; }
        // Reset colors
        colors.clear();
    }
    return 0;
}
