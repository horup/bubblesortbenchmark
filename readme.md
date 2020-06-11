

C:\Users\soh\Desktop\PROJECTS\jsvscsharp>dotnet run --configuration=Release
28500ms to sort 50000 elements 10 times  (List)
13782ms to sort 50000 elements 10 times (Array)

C:\Users\soh\Desktop\PROJECTS\jsvscsharp>node index.js
27613ms to sort 50000 elements 10 times

Conclusion:
Dotnet and node is similar in performance when using List.
Dotnet a factor 1.87 faster than node for bubblesort when using array in dotnet.

Todo:
- Check if it is possible to run node in "Release" configuration
- Check if performance can be improved in the javascript code somehow, e.g. using Object.seal or similar. 
