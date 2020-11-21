#include <iostream>
#include <vector>
#include "nn.h"

using namespace std;

void training_data(vector<vector<int> > train_data, vector<int> train_label) {
	int value;

	for(int i = 0; i < 60000; i++) {
		vector<int> data(51, 0);
		for(int j = 0; j < 51; j++) {
			cin >> value;
			data[j] = value;
		}
		cin >> value;
		train_label.push_back(value);
		train_data.push_back(data);
	}
}

int main() {
	vector<vector<int> > train_data;
	vector<int> train_label;

	training_data(train_data, train_label);

	Neural_network nn(train_data, train_label);

	nn.print();

	return 0;
}