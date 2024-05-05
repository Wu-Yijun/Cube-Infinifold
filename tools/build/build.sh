# .so for linux, .dylib for mac
if [ "$ENV_SYSTEM" = "mac" ]; then
  LIB_EXTENSION=".dylib"
else if [ "$ENV_SYSTEM" = "linux" ]; then
  LIB_EXTENSION=".so"
else
  echo "Unsupported system"
  exit 1
fi

# Build the project
if [ "$ENABLE_CARGO_BUILD" = "true" ]; then
  echo "Building project"
  cargo build --release
  touch ./target/release/${EXECUTABLE_NAME}
else
  # Create binary file if not building
  echo "Skipping build"
  mkdir -p ./target/release
  touch ./target/release/${EXECUTABLE_NAME}
  touch ./target/release/${EXECUTABLE_CHECK_NAME}
  touch ./target/release/empty${LIB_EXTENSION}
fi

# Copy the binary file to the target directory
if [ "$ENV_SYSTEM" = "mac" ]; then
  # Copy the dylib files to the target directory
  ffmpeg_path=$(brew --prefix ffmpeg)
  cp $ffmpeg_path/lib/*.dylib ./target/release
else if [ "$ENV_SYSTEM" = "linux" ]; then
  # Copy the so files to the target directory
  cp /usr/lib/x86_64-linux-gnu/libav*.so ./target/release
else
  echo "Unsupported system"
  exit 1
fi

echo $(ls ./target/release)

# Run the tests
if [ "$ENABLE_CARGO_TEST" = "true" ]; then
  echo "Running tests"
  cargo test
else
  echo "Skipping tests"
fi

# copy and compress the binary and library into a zip file
cd ./target/release
mkdir -p ./${EXECUTABLE_NAME}_${ENV_SYSTEM}/libs
mv ./${EXECUTABLE_NAME} ./${EXECUTABLE_NAME}_${ENV_SYSTEM}/
mv ./${EXECUTABLE_CHECK_NAME} ./${EXECUTABLE_NAME}_${ENV_SYSTEM}/
mv ./*${LIB_EXTENSION} ./${EXECUTABLE_NAME}_${ENV_SYSTEM}/libs/
zip -r ./${EXECUTABLE_NAME}_${ENV_SYSTEM}.zip ./${EXECUTABLE_NAME}_${ENV_SYSTEM}

# Run the Check program
if [ "$ENABLE_PROGRAM_CHECK" = "true" ]; then
  echo "Running Rust program"
  ./${EXECUTABLE_NAME}_${ENV_SYSTEM}/${EXECUTABLE_CHECK_NAME}
  exit_code=$?
  if [ $exit_code -ne 0 ]; then
    echo "Rust program failed with exit code $exit_code"
    exit 1
  fi
else
  echo "Skipping Rust program"
fi

# delete the ${EXECUTABLE_NAME}_${ENV_SYSTEM} directory
rm -rf ./target/release/${EXECUTABLE_NAME}_${ENV_SYSTEM}

# Decompress the zip file to root
unzip ./${EXECUTABLE_NAME}_${ENV_SYSTEM}.zip -d ../../