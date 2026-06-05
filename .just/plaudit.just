bin := "plaudit-cli"

login:
    cargo run -p {{bin}} -- login

logout:
    cargo run -p {{bin}} -- logout

me:
    cargo run -p {{bin}} -- me

files *args:
    cargo run -p {{bin}} -- files {{args}}

file id:
    cargo run -p {{bin}} -- file {{id}}

transcript id *args:
    cargo run -p {{bin}} -- transcript {{id}} {{args}}

summary id *args:
    cargo run -p {{bin}} -- summary {{id}} {{args}}

audio id:
    cargo run -p {{bin}} -- audio {{id}}

sync folder:
    cargo run -p {{bin}} -- sync {{folder}}
