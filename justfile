# OpenDeck uses the folder name (not the manifest PluginUUID field) as the plugin identity,
# so this must not collide with the upstream plugin's folder name.
id := "4296d983-7e3c-4b6e-a76c-dc722e81a362.sdPlugin"

release next=`git cliff --bumped-version | tr -d "v"`: (bump next) package (tag next)

package: build-linux build-win collect zip

bump next=`git cliff --bumped-version | tr -d "v"`:
    git diff --cached --exit-code

    echo "We will bump version to {{next}}, press any key"
    read ans

    sed -i 's/"Version": ".*"/"Version": "{{next}}"/g' manifest.json
    sed -i 's/^version = ".*"$/version = "{{next}}"/g' Cargo.toml

tag next=`git cliff --bumped-version | tr -d "v"`:
    echo "Generating changelog"
    git cliff -o CHANGELOG.md --tag v{{next}}

    echo "We will now commit the changes, please review before pressing any key"
    read ans

    git add .
    git commit -m "chore(release): v{{next}}"
    git tag "v{{next}}"

build-linux:
    cargo build --release --target x86_64-unknown-linux-gnu --target-dir target/plugin-linux

build-win:
    cargo build --release --target x86_64-pc-windows-msvc --target-dir target/plugin-win

clean:
    sudo rm -rf target/

collect:
    rm -rf build
    mkdir -p build/{{id}}
    cp -r assets build/{{id}}
    cp manifest.json build/{{id}}
    cp target/plugin-linux/x86_64-unknown-linux-gnu/release/opendeck-akp153 build/{{id}}/opendeck-akp153-linux
    cp target/plugin-win/x86_64-pc-windows-gnu/release/opendeck-akp153.exe build/{{id}}/opendeck-akp153-win.exe

[working-directory: "build"]
zip:
    zip -r opendeck-mirabox-293v3.plugin.zip {{id}}/
