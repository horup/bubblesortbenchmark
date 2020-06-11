using System;

namespace csharp
{
    class Program
    {
        static void Main(string[] args)
        {
            int num = 50*1000;
            int retry = 10;
            int[] array = new int[num];
            for (var i = 0; i < num; i++)
                array[i] = array.Length - i;

            int tick = Environment.TickCount;

            for (int i = 0; i < array.Length; i++)
                for (int j = 0; j < array.Length; j++)
                {
                    if (array[i] < array[j])
                    {
                        int org = array[i];
                        array[i] = array[j];
                        array[j] = org;
                    }
                }

            int took = Environment.TickCount - tick;

            Console.WriteLine($"{took}ms to sort {array.Length} elements");
        }
    }
}
