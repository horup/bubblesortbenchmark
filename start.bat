clang -O -fsanitize=address main.cpp && a.exe
clang -O main.cpp && a.exe
dotnet run --configuration=Release
node index.js
rustc -O main.rs -o rust.exe
./rust.exe