@ECHO OFF 
cd ..
cargo build --release
set "fname=iced-template.exe"
rd /s /q out
mkdir out

xcopy ".\libs\win_x64\*" "out\" /E /I /Q
xcopy .\plugins .\out\plugins /E /I /H /K /Q
xcopy .\migrations .\out\migrations /E /I /H /K /Q
copy /y .\target\release\%fname% .\out\%fname%
copy /y .\app.db .\out\app.db

echo the iced-template.exe into ./out
start out