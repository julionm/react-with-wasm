echo "removendo arquivo atual..."
rm -rf node_modules/my-wasm-lib

echo "updating the new one"
cp -r ../pkg node_modules/my-wasm-lib