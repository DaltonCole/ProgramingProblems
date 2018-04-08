#include <iostream>
#include <algorithm>
using namespace std;

#define N 1000001

int parent[N];
int depth[N]; // rank
int current = -1;

int UniqueNumber () { return ++current; }

int find(int parent[], int i) {
	while(i != parent[i]) {
		i = parent[i];
	}
	return i;
}
  
void Union(int parent[], int x, int y)
{
    int xset = find(parent, x);
    int yset = find(parent, y);

    if(xset == yset) {
    	return;
    } else if(depth[xset] > depth[yset]) {
    	parent[yset] = xset;
    } else {
    	parent[xset] = yset;
    	if(depth[xset] == depth[yset]) {
    		depth[yset]++;
    	}
    }

    return;
}

int main() {
	fill_n(parent, N, -1);
	generate_n(parent, N, UniqueNumber);

	int number, queries;
	char op;
	int a, b;
	int temp1, temp2;

	cin >> number >> queries;

	for(int i = 0; i < queries; i++) {
		cin >> op >> a >> b;

		if(op == '?') {
			temp1 = find(parent, a);
			temp2 = find(parent, b);

			if(temp2 == temp1) {
				cout << "yes" << endl;
			} else {
				cout << "no" << endl;
			}
		} else {
			Union(parent, a, b);
		}
	}

	return 0;
}