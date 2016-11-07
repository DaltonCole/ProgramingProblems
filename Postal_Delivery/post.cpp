#include <iostream>
#include <queue>
using namespace std;

struct place {
	int location;
	int mail;

	place & operator =(const place & rhs) {
		location = rhs.location;
		mail = rhs.mail;

		return *this;
	}
};

struct neg_place {
	bool operator () (place a, place b) {
		return a.location > b.location;
	}
};

struct pos_place {
	bool operator ()(place a, place b) {
		return a.location < b.location;
	}
};

int main() {
	int num_places;
	int capacity;
	int temp1, temp2;
	int current_mail;
	long distance = 0;
	place places;
	priority_queue<place, vector<place>, neg_place> n_place;
	priority_queue<place, vector<place>, pos_place> p_place;

	cin >> num_places >> capacity;

	for(int i = 0; i < num_places; i++) {
		cin >> temp1 >> temp2;
		places.location = temp1;
		places.mail = temp2;
		if(places.location < 0) {
			n_place.push(places);
		} else if(places.location > 0) {
			p_place.push(places);
		}
	}

	current_mail = 0;

	while(n_place.empty() == false) {
		places = n_place.top();
		n_place.pop();

		if(current_mail == 0) {
			current_mail = capacity;
			distance += (-2 * places.location);
			current_mail -= places.mail;
			if(current_mail < 0) {
				places.mail = -current_mail;
				current_mail = 0;
				n_place.push(places);
			}
		} else {
			current_mail -= places.mail;
			if(current_mail < 0) {
				places.mail = -current_mail;
				current_mail = 0;
				n_place.push(places);
			}
		}
	}

	current_mail = 0;

	while(p_place.empty() == false) {
		places = p_place.top();
		p_place.pop();

		if(current_mail == 0) {
			current_mail = capacity;
			distance += (2 * places.location);
			current_mail -= places.mail;
			if(current_mail < 0) {
				places.mail = -current_mail;
				current_mail = 0;
				p_place.push(places);
			}
		} else {
			current_mail -= places.mail;
			if(current_mail < 0) {
				places.mail = -current_mail;
				current_mail = 0;
				p_place.push(places);
			}
		}
	}

	cout << distance << endl;

	return 0;
}