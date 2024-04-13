@REM build a release version
cargo build --release

@REM move to output folder
copy /B/V/Y target\release\cube-infinifold.exe bin\
copy /B/V/Y target\release\*.dll bin\


@REM explorer D:\Cube-Infinifold\output\

start D:\Cube-Infinifold\bin\cube-infinifold.exe 