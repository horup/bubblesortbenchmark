#include<iostream>
#include<vector>
#include<Windows.h>

void usingVector()
{
    int num = 50*1000;
    int rerun = 10;
    std::vector<int> array = std::vector<int>();
    for (auto i = 0; i < num; i++)
        array.push_back(num - i);

    int tick = GetTickCount();
    for (int r = 0; r < rerun; r++)
    {
        for (int i = 0; i < array.size(); i++)
            for (int j = 0; j < array.size(); j++)
            {
                if (array[i] < array[j])
                {
                    int org = array[i];
                    array[i] = array[j];
                    array[j] = org;
                }
            }
    }

    int took = GetTickCount() - tick;
    std::cout << took << "ms to sort " << num << " elements " << rerun << " times (Vector)\n";
}

void usingArray()
{
    int num = 50*1000;
    int rerun = 10;
    int* array = new int[num];
    for (auto i = 0; i < num; i++)
        array[i] = num - i;

    int tick = GetTickCount();
    for (int r = 0; r < rerun; r++)
    {
        for (int i = 0; i < num; i++)
            for (int j = 0; j < num; j++)
            {
                if (array[i] < array[j])
                {
                    int org = array[i];
                    array[i] = array[j];
                    array[j] = org;
                }
            }
    }

    int took = GetTickCount() - tick;
    std::cout << took << "ms to sort " << num << " elements " << rerun << " times (Array)\n";

    delete[] array;
}

int main()
{
    usingVector();
    usingArray();
    return 0;
}