set shell := ["powershell.exe", "-c"]

alias r := run

run:
    cargo run --bin turret-game --features bevy/dynamic_linking

edit:
    cargo run --bin voxel-map-editor --features bevy/dynamic_linking
