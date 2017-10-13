#include <iostream>
#include <algorithm>
#include <list>
#include <unordered_map>

using namespace std;

int main() {
	int inputs;
	int number;
	int temp;

	list<int> not_sorted;
	unordered_map<int, int> mapping;

	//list<int>::iterator it;// = find(ilist.begin(), ilist.end(), 1);

	cin >> inputs;

	int small = 1;
	int large = inputs;

	for(int i = 0; i < inputs; i++) {
		cin >> number;

		not_sorted.push_back(number);
		mapping[number] = i;
	}

	for(int i = 0; i < inputs; i++) {
		if(i % 2 == 0) {
			//it = find(not_sorted.begin(), not_sorted.end(), small);
			temp = 0;

			for(list<int>::iterator it = not_sorted.begin(); it != not_sorted.end(); ++it){
			    if(*it == small) {
			    	not_sorted.erase(it);
			    	break;
			    }
			    temp++;
			}

			small++;

			cout << temp << endl;
		} else {
			temp = -1;

			for(list<int>::iterator it = not_sorted.end(); it != not_sorted.begin(); --it){
			    if(*it == large) {
			    	not_sorted.erase(it);
			    	break;
			    }
			    temp++;
			}

			large--;

			cout << temp << endl;
		}
		
	}

	return 0;
}