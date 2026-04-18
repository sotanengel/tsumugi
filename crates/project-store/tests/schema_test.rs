use project_store::migrate;
use rusqlite::Connection;

#[test]
fn migrate_creates_tables() {
    let conn = Connection::open_in_memory().unwrap();
    migrate(&conn).unwrap();

    // Verify tables exist by querying them
    conn.execute_batch("SELECT * FROM projects LIMIT 0").unwrap();
    conn.execute_batch("SELECT * FROM tracks LIMIT 0").unwrap();
    conn.execute_batch("SELECT * FROM clips LIMIT 0").unwrap();
}

#[test]
fn migrate_is_idempotent() {
    let conn = Connection::open_in_memory().unwrap();
    migrate(&conn).unwrap();
    migrate(&conn).unwrap(); // Should not fail on second run
}

#[test]
fn schema_version_is_tracked() {
    let conn = Connection::open_in_memory().unwrap();
    migrate(&conn).unwrap();

    let version: i64 = conn
        .query_row("SELECT MAX(version) FROM schema_version", [], |row| {
            row.get(0)
        })
        .unwrap();
    assert_eq!(version, 1);
}
