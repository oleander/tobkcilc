set dotenv-load := true

install: setup
  . .espup.sh && cargo run -r
setup:
  espup install -t $MCU -f .espup.sh
clean:
  cargo clean
