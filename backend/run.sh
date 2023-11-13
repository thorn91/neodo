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

export DATABASE_FILE_LOCATION=./db/database.sqlite?mode=rwc && export DATABASE_URL=sqlite://$DATABASE_FILE_LOCATION && cargo clean && sqlx database drop && sqlx database create && sqlx migrate run

sea-orm-cli migrate 
sea-orm-cli generate entity -o "domain/src/entity/" --lib