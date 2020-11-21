#include <iostream>
#include <vector>
#include <stdio.h>      /* printf, scanf, puts, NULL */
#include <stdlib.h>     /* srand, rand */
#include <time.h>       /* time */

using namespace std;


class Neural_network {
	public:
		Neural_network(vector<vector<int> > data, vector<int> label);
		int find_sign(vector<int> v);
		void train();
		void print();
		int classify(vector<int> data);
		bool test(vector<int> data, int label){return label == classify(data)? true : false;}

	private:
		vector<vector<int> > edge;
		vector<vector<int> > train_data;
		vector<int> train_label;
};

Neural_network::Neural_network(vector<vector<int> > data, vector<int> label) {
	srand (time(NULL));
	bool tmp;
	train_data = data;
	train_label = label;

	for(int i = 0; i < 150; i++) {
		vector<int> single_edge(51, -1);
		for(int j = 0; j < 51; j++) {
			tmp = rand() % 2;
			if(tmp == true) {
				single_edge[j] = 1;
			}
		}
		edge.push_back(single_edge);
	}
}

int Neural_network::find_sign(vector<int> v) {
	int sum = 0;
	for(const auto& i : v) {
		sum += i;
	}
	return sum < 0 ? -1 : 1;
}

void Neural_network::train() {
	for(int i = 0; i < data.size(); i++) {
		for(int z = 0; z < 10; z++) {
			if(label[i] == z) {continue;}
			for(int j = 0; j < 15; j++) {
				
			}
		}
	}
}

void Neural_network::print() {
	for(int i = 0; i < 150; i++) {
		for(int j = 0; j < 51; j++) {
			cout << edge[i][j] << " ";
		}
		cout << endl;
	}
}

int Neural_network::classify(vector<int> data) {
	// Perception layer
	vector<int> sign(150, 0);
	for(int i = 0; i < 150; i++) {
		for(int j = 0; j < 51; j++) {
			sign[i] += data[j] * edge[i][j];
		}
	}
	for(int i = 0; i < 150; i++) {
		sign[i] = sign[i] < 0 ? -1 : 1;
	}
	// Sum nodes
	vector<int> sums(10, 0);
	for(int i = 0; i < 10; i++) {
		for(int j = 0; j < 15; j++) {
			sums[i] += sign[(15 * i) + j];
		}
	}

	// Arg max
	int index = 0;
	for(int i = 1; i < 10; i++) {
		if(sums[i] > sums[index]) {
			index = i;
		}
	}

	return index;
}