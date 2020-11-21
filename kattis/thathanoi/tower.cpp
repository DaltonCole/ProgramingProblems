#include <iostream>
#include <unordered_map>
#include <vector>
#include <algorithm>

using namespace std;

unordered_map<string, int> turns;

vector<int> l;
vector<int> m;
vector<int> r;

int total_moves;

string make_string() {
    string tmp = "";
    for(const auto& i : l) {
        tmp += (i - '0');
    }
    tmp += ' ';
    for(const auto& i : m) {
        tmp += (i - '0');
    }
    tmp += ' ';
    for(const auto& i : r) {
        tmp += (i - '0');
    }

    return tmp;
}

void move(char from, char to) {
    // cout << "Moving " << from << " to " << to << endl;
    if(from == 'a' && to == 'b') {
        m.push_back(l.back());
        l.pop_back();
    } else if(from == 'a' && to == 'c') {
        r.push_back(l.back());
        l.pop_back();
    } else if(from == 'b' && to == 'a') {
        l.push_back(m.back());
        m.pop_back();
    } else if(from == 'b' && to == 'c') {
        r.push_back(m.back());
        m.pop_back();
    } else if(from == 'c' && to == 'a') {
        l.push_back(r.back());
        r.pop_back();
    } else if(from == 'c' && to == 'b') {
        m.push_back(r.back());
        r.pop_back();
    }

    turns[make_string()] = total_moves;
    total_moves++;
}

void tower(int n, char from, char to, char aux) {
    if(n == 1) {
        move(from, to);            
        return;
    }
    tower(n - 1, from, aux, to);
    move(from, to);
    tower(n - 1, aux, to, from);
}


int main() {
    vector<int> a, b, c;
    int tmp, count, max_disk = 0;

    cin >> count;

    for(int i = 0; i < count; i++) {
        cin >> tmp;
        a.push_back(tmp);
        max_disk = max(max_disk, tmp);
    }
    cin >> count;
    for(int i = 0; i < count; i++) {
        cin >> tmp;
        b.push_back(tmp);
        max_disk = max(max_disk, tmp);
    }
    cin >> count;
    for(int i = 0; i < count; i++) {
        cin >> tmp;
        c.push_back(tmp);
        max_disk = max(max_disk, tmp);
    }

    for(int i = max_disk; i > 0; i--) {
        l.push_back(i);
    }

    tower(max_disk, 'a', 'c', 'b');

    l = a;
    m = b;
    r = c;

    string s = make_string();

    if(turns.find(s) != turns.end()) {
        cout << (total_moves - turns[s] - 1) << endl;
    } else {
        cout << "No" << endl;
    }

    return 0;
}
