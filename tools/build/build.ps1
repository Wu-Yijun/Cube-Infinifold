$ENABLE_CARGO_BUILD = $env:ENABLE_CARGO_BUILD
$ENABLE_CARGO_TEST = $env:ENABLE_CARGO_TEST
$ENABLE_PROGRAM_CHECK = $env:ENABLE_PROGRAM_CHECK
$EXECUTABLE_NAME = $env:EXECUTABLE_NAME
$EXECUTABLE_CHECK_NAME = $env:EXECUTABLE_CHECK_NAME
$SYSTEM = $env:ENV_SYSTEM
$PFX_CERTIFICATION_PASSWORD = $env:SECRET_PFX_CERTIFICATION_PASSWORD

# Build the project
if ( $ENABLE_CARGO_BUILD -eq 'true' ) {
  echo "Building the project..."
  cargo build --release
} else {
  echo "Skipping the build..."
  mkdir -p ./target/release
  echo "......" > ./target/release/${EXECUTABLE_NAME}.exe
  echo "......" > ./target/release/${EXECUTABLE_CHECK_NAME}.exe
}

# Cargo Test
if ( $ENABLE_CARGO_TEST -eq 'true' ) {
  echo "Running cargo test..."
  cargo test
} else {
  echo "Skipping cargo test..."
}

# List the files in the target/release directory
cp ${pwd}/ffmpeg/bin/*.dll ./target/release
echo $(tree ./target/release)

# sign the binary
$password = ConvertTo-SecureString -String $PFX_CERTIFICATION_PASSWORD -Force -AsPlainText
$cert = Get-PfxCertificate -FilePath ./tools/build/MyCert.pfx -Password $password
Set-AuthenticodeSignature -FilePath ./target/release/${EXECUTABLE_CHECK_NAME}.exe -Certificate $cert -TimeStampServer http://timestamp.digicert.com -HashAlgorithm SHA256

# copy and compress the binary and library into a zip file
cd ./target/release
mkdir -p ./${EXECUTABLE_NAME}_windows/libs
mkdir -p ./${EXECUTABLE_NAME}_windows/assets
mv ../../assets/ui ./${EXECUTABLE_NAME}_windows/assets/
mv ../../assets/version_files/${SYSTEM}/levels.json ./${EXECUTABLE_NAME}_windows/


mv *.exe ${EXECUTABLE_NAME}_windows/
mv *.dll ${EXECUTABLE_NAME}_windows/libs/
Compress-Archive -Path ${EXECUTABLE_NAME}_windows ${EXECUTABLE_NAME}_windows.zip
# mv ${EXECUTABLE_CHECK_NAME}.exe ${EXECUTABLE_NAME}_windows/

# Test the Rust program
if ( $ENABLE_PROGRAM_CHECK -eq 'true'){
  echo "Running the Check program..."
  ./${EXECUTABLE_NAME}_windows/${EXECUTABLE_CHECK_NAME}.exe
  $exit_code = $LASTEXITCODE
  if ($exit_code -ne 0) {
    Write-Output "Rust program failed with exit code $exit_code"
    exit 1
  }
} else {
  echo "Skipping the Check program..."
}

# delete the cube-infinifold_windows directory
Remove-Item -Path ./${EXECUTABLE_NAME}_windows -Recurse -Force

# decompress the zip file to root
Expand-Archive -Path ${EXECUTABLE_NAME}_windows.zip -Destination ../../