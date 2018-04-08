#include <iostream>
using namespace std;

struct Point {
	int x;
	int y;
}

struct Shape {
	Shape(int type) {
		points = new Point[4];

		if(type == 1) {
			points[0].x = 0;
			points[0].y = 0;

			points[1].x = 0;
			points[1].y = 0;

			points[2].x = 0;
			points[2].y = 0;

			points[3].x = 0;
			points[3].y = 0;
		}
	}

	~Shape(){
		delete[] points;
	}

	Point* points;
}