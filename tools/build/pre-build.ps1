$VCINSTALLDIR = $(& "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe" -latest -property installationPath)
Add-Content $env:GITHUB_ENV "LIBCLANG_PATH=${VCINSTALLDIR}\VC\Tools\LLVM\x64\bin`n"
Invoke-WebRequest "${env:FFMPEG_DOWNLOAD_URL}" -OutFile ffmpeg-release-full-shared.7z
7z x ffmpeg-release-full-shared.7z
mkdir ffmpeg
mv ffmpeg-*/* ffmpeg/
Add-Content $env:GITHUB_ENV "FFMPEG_DIR=${pwd}\ffmpeg`n"
Add-Content $env:GITHUB_PATH "${pwd}\ffmpeg\bin`n"