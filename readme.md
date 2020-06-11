C:\Users\soh\Desktop\PROJECTS\jsvscsharp>clang -O -fsanitize=address main.cpp   && a.exe
   Creating library a.lib and object a.exp
30156ms to sort 50000 elements 10 times (Vector)
22360ms to sort 50000 elements 10 times (Array)

C:\Users\soh\Desktop\PROJECTS\jsvscsharp>clang -O main.cpp   && a.exe
9984ms to sort 50000 elements 10 times (Vector)
8859ms to sort 50000 elements 10 times (Array)

C:\Users\soh\Desktop\PROJECTS\jsvscsharp>dotnet run --configuration=Release
28766ms to sort 50000 elements 10 times  (List)
14687ms to sort 50000 elements 10 times (Array)

C:\Users\soh\Desktop\PROJECTS\jsvscsharp>node index.js
29131ms to sort 50000 elements 10 times


Conclusion:
Dotnet and node is similar in performance when using List.
Dotnet a factor 1.87 faster than node for bubblesort when using array in dotnet.

Todo:
- Check if it is possible to run node in "Release" configuration
- Check if performance can be improved in the javascript code somehow, e.g. using Object.seal or similar. 
