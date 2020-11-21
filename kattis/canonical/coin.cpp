#include <iostream>
#include <vector>
#include <limits>

using namespace std;

int greedyCoins(const vector<int> & coins, int value) {
	int coin_count = 0;
	int index = coins.size() - 1;

	while(value != 0) {
		while(value >= coins[index]) {
			value -= coins[index];
			coin_count++;
		}
		index -= 1;
	}

	return coin_count;
}

int minCoins(const vector<int> & coins, const int value) {
	if(value == 0) {
		return 0;
	}

	int res = numeric_limits<int>::max() - 1;
	int temp_res;

	for(auto coin : coins) {
		if(coin <= value) {
			temp_res = minCoins(coins, value - coin);

			if(temp_res + 1 < res) {
				res = temp_res + 1;
			}
		}
	}

	return res;
}

void canonical(const vector<int> & coins) {
	int max_sum = coins[coins.size() - 1] + coins[coins.size() - 2];

	// 0 and 1 are trivial
	for(int i = 2; i < max_sum; i++) {
		cout << i << endl;
		//cout << minCoins(coins, i) << endl;
		//cout << greedyCoins(coins, i) << endl << endl;
		if(minCoins(coins, i) != greedyCoins(coins, i)) {
			cout << "non-canonical" << endl;
			return;
		}
	}

	cout << "canonical" << endl;
	return;
}

int main() {
	int cases;
	int temp;
	vector<int> coins;

	cin >> cases;

	for(int i = 0; i < cases; i++) {
		cin >> temp;
		coins.push_back(temp);
	}

	canonical(coins);

	return 0;
}