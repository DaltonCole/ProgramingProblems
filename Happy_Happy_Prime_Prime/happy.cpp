#include <iostream>
using namespace std;

bool happy(int p, int tries)
{
    int sum = 0, smaller_number = p;
    bool is_happy = false;

    while(smaller_number > 0)
    {
        sum += (smaller_number % 10)*(smaller_number % 10);
        smaller_number /= 10;
    }

    if(sum != 1 && tries < 100)
    {
        is_happy = happy(sum, tries+1);
    }

    if(sum == 1)
    {
        is_happy = true;
    }

    return is_happy;
}


int main()
{
    int lines, line_number, p;
    bool is_prime = false, is_happy = false;

    cin >> lines;

    for(int i = 0; i < lines; i++)
    {
        cin >> line_number >> p;

        for(int i = 2; i < p/2; i++)
        {
            if(p % i == 0)
            {
                is_prime = false;
                break;
            }
            is_prime = true;
        }

        if(is_prime == true)
        {
            is_happy = happy(p, 0);
        }

        cout << line_number << " " << p;

        if(is_happy == true && is_prime == true)
        {
            cout << " YES" << endl;
        }
        else
        {
            cout << " NO" << endl;
        }

        is_prime = false;
        is_happy = false;

    }

    return 0;
}
