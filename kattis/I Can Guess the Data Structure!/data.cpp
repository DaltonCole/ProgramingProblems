#include <iostream>
#include <queue>
#include <stack>
#include <vector>

using namespace std;

struct Compare {
	bool operator() (int a, int b) {
		return a < b;
	}
};

int main() {
	int cases;

	int command;
	int input;
	int temp;

	stack<int> s;
	queue<int> q;
	priority_queue<int, vector<int>, Compare> pq;

	bool possible_s;
	bool possible_q;
	bool possible_pq;


	while(cin >> cases) {
		possible_pq = possible_q = possible_s = true;

		for(int i = 0; i < cases; i++) {
			cin >> command >> input;

			if(command == 1) {
				if(possible_s) {
					s.push(input);
				}
				if(possible_q) {
					q.push(input);
				}
				if(possible_pq) {
					pq.push(input);
				}
			} else {
				if(s.empty() == true) {
					possible_pq = possible_q = possible_s = false;
					break;
				}

				if(possible_s) {
					if(s.empty()) {
						possible_s = false;
					} else {
						// Check for stack
						temp = s.top();
						s.pop();

						if(temp != input) {
							possible_s = false;
						}
					}
				}

				if(possible_q) {
					if(q.empty()) {
						possible_q = false;
					} else {
						// Check for queue
						temp = q.front();
						q.pop();

						if(temp != input) {
							possible_q = false;
						}
					}
				}

				if(possible_pq) {
					if(pq.empty()) {
						possible_pq = false;
					} else {
						// Check for priority queue
						temp = pq.top();
						pq.pop();

						if(temp != input) {
							possible_pq = false;
						}
					}
				}
			}
		}

		s = stack<int> ();
		q = queue<int> ();
		pq = priority_queue<int, vector<int>, Compare> ();

		if(!possible_pq && !possible_q && !possible_s) {
			cout << "impossible";
		} else if(possible_s && !possible_q && !possible_pq) {
			cout << "stack";
		} else if(!possible_s && possible_q && !possible_pq) {
			cout << "queue";
		} else if(!possible_s && !possible_q && possible_pq) {
			cout << "priority queue";
		} else {
			cout << "not sure";
		}
		cout << endl;

	}

	return 0;
}