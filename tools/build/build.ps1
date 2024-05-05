# $ENV_ENABLE_CARGO_BUILD = ${{ env.ENABLE_CARGO_BUILD }}
# $ENV_ENABLE_CARGO_TEST = ${{ env.ENABLE_CARGO_TEST }}
# $ENV_ENABLE_PROGRAM_CHECK = ${{ env.ENABLE_PROGRAM_CHECK }}
# $ENV_EXECUTABLE_NAME = ${{ env.EXECUTABLE_NAME }}
# $ENV_EXECUTABLE_CHECK_NAME = ${{ env.EXECUTABLE_CHECK_NAME }}
# $ENV_SYSTEM = 'windows'
# $SECRET_PFX_CERTIFICATION_PASSWORD = ${{ secrets.PFX_CERTIFICATION_PASSWORD }}


# Build the project
if ( $ENV_ENABLE_CARGO_BUILD -eq 'true' ) {
  cargo build --release
  echo . > ./target/release/${ENV_EXECUTABLE_NAME}.exe
} else {
  mkdir -p ./target/release
  echo . > ./target/release/${ENV_EXECUTABLE_NAME}.exe
  echo . > ./target/release/${ENV_EXECUTABLE_CHECK_NAME}.exe
}

# Cargo Test
if ( $ENV_ENABLE_CARGO_TEST -eq 'true' ) {
  cargo test
}

# List the files in the target/release directory
cp ${pwd}/ffmpeg/bin/*.dll ./target/release
Tree ./target/release

# sign the binary
$password = ConvertTo-SecureString -String $SECRET_PFX_CERTIFICATION_PASSWORD -Force -AsPlainText
$cert = Get-PfxCertificate -FilePath MyCert.pfx -Password $password
Set-AuthenticodeSignature -FilePath target/release/${ENV_EXECUTABLE_CHECK_NAME}.exe -Certificate $cert -TimeStampServer http://timestamp.digicert.com -HashAlgorithm SHA256

# copy and compress the binary and library into a zip file
cd ./target/release
mkdir -p ./${ENV_EXECUTABLE_NAME}_windows/libs
mkdir -p ./${ENV_EXECUTABLE_NAME}_windows/assets
mv ../../assets/ui ./${ENV_EXECUTABLE_NAME}_windows/assets/
mv ../../assets/version_files/$ENV_SYSTEM/levels.json ./${ENV_EXECUTABLE_NAME}_windows/

mv ${ENV_EXECUTABLE_NAME}.exe ${ENV_EXECUTABLE_NAME}_windows/
mv *.dll ${ENV_EXECUTABLE_NAME}_windows/libs/
Compress-Archive -Path ${ENV_EXECUTABLE_NAME}_windows ${ENV_EXECUTABLE_NAME}_windows.zip
mv ${ENV_EXECUTABLE_CHECK_NAME}.exe ${ENV_EXECUTABLE_NAME}_windows/

# Test the Rust program
if ( $ENV_ENABLE_PROGRAM_CHECK -eq 'true'){
  ./${ENV_EXECUTABLE_NAME}_windows/${ENV_EXECUTABLE_CHECK_NAME}.exe
  $exit_code = $LASTEXITCODE
  if ($exit_code -ne 0) {
    Write-Output "Rust program failed with exit code $exit_code"
    exit 1
  }
}

# delete the cube-infinifold_windows directory
Remove-Item -Path ./${ENV_EXECUTABLE_NAME}_windows -Recurse -Force

# decompress the zip file to root
Expand-Archive -Path ${ENV_EXECUTABLE_NAME}_windows.zip -Destination ../../