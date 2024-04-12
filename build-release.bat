@REM build a release version
cargo build --release

@REM move to output folder
copy /B/V/Y target\release\cube-infinifold.exe output\
copy /B/V/Y target\release\*.dll output\


@REM explorer D:\Cube-Infinifold\output\

start D:\Cube-Infinifold\output\cube-infinifold.exe 