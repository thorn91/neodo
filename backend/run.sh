# Terminal 1 - For server run.
cargo watch -q -c -w src/ -x "run" 
MAIN_PID=$!
echo $MAIN_PID

# Terminal 2 - For test.
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
TEST_PID=$!
echo $TEST_PID

# SQXL Migration
sqlx migrate add users

mkdir db && sea-orm-cli migrate 
sea-orm-cli generate entity -o "entity" --lib -with-serde both