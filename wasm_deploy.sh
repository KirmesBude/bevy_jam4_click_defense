cat ./dist/index.html > ./dist/tempindex
cat ./dist/tempindex | sed -i "s/\"\//\"\.\//g; s/'\//'\.\//g" ./dist/index.html;
rm ./dist//tempindex

<<comment
for file in ./dist/*.wasm
do
    wasm-opt -Os -o "$file" "$file"
done
comment

cd dist; zip -r ../bevy_jam4_click_defense.zip *