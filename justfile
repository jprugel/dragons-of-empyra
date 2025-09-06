set shell := ["powershell.exe", "-c"]

alias r := client

client:
    cargo run --bin turret-game --features bevy/dynamic_linking

editor:
    cargo run --bin voxel-map-editor --features bevy/dynamic_linking
