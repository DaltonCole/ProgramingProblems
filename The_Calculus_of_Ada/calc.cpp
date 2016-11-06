#include <iostream>
#include <vector>

using namespace std;

bool unique(vector<int> v) {
	int temp = v[0];
	
	for(int i = 1; i < v.size(); i++) {
		if(temp != v[i]) {
			return false;
		}
	}
	
	return true;
}


int main() {
	int num, temp, back, level;
	vector<int> v;
	vector<int> u;
	vector<int> t;
	
	cin >> num;
	
	for(int i = 0; i < num; i++) {
		cin >> temp;
		v.push_back(temp);
	}
	
	back = v.back();
	level = 0;
	
	while(true) {
		level++;
		for(int i = 0; i < v.size() - 1; i++){
			u.push_back(v[i+1] - v[i]);
		}
		if(unique(u) == true) {
			t.push_back(u.back());
			break;
		} else {
			t.push_back(u.back());
			v = u;
			u.clear();
		}
	}
	
	num = 0;
	
	for(auto it : t) {
		num += it;
	}
	
	cout << level << " " << back + num << endl;
	
	
	
	return 0;
}


