cat ./dist/index.html > ./dist/tempindex
cat ./dist/tempindex | sed -i "s/\"\//\"\.\//g; s/'\//'\.\//g" ./dist/index.html;
rm ./dist//tempindex

for file in ./dist/*.wasm
do
    wasm-opt -Os -o "$file" "$file"
done
