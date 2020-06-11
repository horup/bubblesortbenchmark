using System;
using System.Collections.Generic;
namespace csharp
{
    class Program
    {
        static void Main(string[] args)
        {
            int num = 50*1000;
            int rerun = 10;
            List<int> array = new List<int>();
            for (var i = 0; i < num; i++)
                array.Add(array.Count - i);

            int tick = Environment.TickCount;

            for (int r = 0; r < rerun; r++)
            {
                for (int i = 0; i < array.Count; i++)
                    for (int j = 0; j < array.Count; j++)
                    {
                        if (array[i] < array[j])
                        {
                            int org = array[i];
                            array[i] = array[j];
                            array[j] = org;
                        }
                    }
            }

            int took = Environment.TickCount - tick;

            Console.WriteLine($"{took}ms to sort {array.Count} elements {rerun} times");
        }
    }
}
