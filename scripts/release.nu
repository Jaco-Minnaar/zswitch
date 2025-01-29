#!/usr/bin/env nu

def create_release [tag: string, file: string] {
    (gh release create $"v($tag)" --generate-notes $file )
}

let target = "./target/wasm32-wasip1/release/zswitch.wasm"
let tag = cargo info zswitch --color never | lines | filter {|s| str starts-with "version"} | split row ' ' | find -r '\d+\.\d+\.\d+(-.*)?' | first

let assets_path = create_release $tag $target
