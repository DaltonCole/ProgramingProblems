#include <iostream>
#include <vector>
#include <cmath>
using namespace std;

struct rectangle {
	int x1;
	int y1;
	int x2;
	int y2;
};

struct circle {
	int x;
	int y;
	int r;
};

int main() {
	//double PI = 3.1415926535897;

	int shapes, points;
	vector<rectangle> rect;
	vector<circle> cir;
	int temp1, temp2, temp3, temp4;
	int x, y;
	string shape;
	rectangle r;
	circle c;
	double distance;
	int count;

	cin >> shapes;

	for(int i = 0; i < shapes; i++) {
		cin >> shape;

		if(shape == "rectangle") {
			cin >> temp1 >> temp2 >> temp3 >> temp4;
			r.x1 = temp1;
			r.y1 = temp2;
			r.x2 = temp3;
			r.y2 = temp4;
			rect.push_back(r);
		} else {
			cin >> temp1 >> temp2 >> temp3;
			c.x = temp1;
			c.y = temp2;
			c.r = temp3;
			cir.push_back(c);
		}
	}	

	cin >> points;

	for(int i = 0; i < points; i++) {
		count = 0;
		cin >> x >> y;

		for(auto it : rect) {
			if(it.x1 <= x && it.x2 >= x && it.y1 <= y && it.y2 >= y) {
				count++;
			}
		}

		for(auto it : cir) {
			if(it.r >= sqrt(pow((x - it.x), 2) + pow((y - it.y), 2))) {
				count++;
			}
		}
		cout << count << endl;
	}

	return 0;
}