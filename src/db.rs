use rusqlite::{Connection, OptionalExtension, Row, params};

use crate::models::{Article, NewArticle};

const SEED_JSON: &str = include_str!("../seed.json");

/// Opens the database, creates the schema and seeds it on first run.
pub fn open(path: &str) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS articles (
            id                INTEGER PRIMARY KEY AUTOINCREMENT,
            title             TEXT NOT NULL,
            category          TEXT NOT NULL,
            short_description TEXT NOT NULL,
            content           TEXT NOT NULL,
            image_url         TEXT,
            created_at        TEXT NOT NULL
        );",
    )?;

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM articles", [], |r| r.get(0))?;
    if count == 0 {
        let seed: Vec<NewArticle> = serde_json::from_str(SEED_JSON).expect("invalid seed.json");
        for a in seed {
            conn.execute(
                "INSERT INTO articles (title, category, short_description, content, image_url, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![a.title, a.category, a.short_description, a.content, a.image_url, a.created_at],
            )?;
        }
        println!("Seeded database with initial articles");
    }
    Ok(conn)
}

const SELECT: &str =
    "SELECT id, title, category, short_description, content, image_url, created_at FROM articles";

fn map_row(row: &Row) -> rusqlite::Result<Article> {
    Ok(Article {
        id: row.get(0)?,
        title: row.get(1)?,
        category: row.get(2)?,
        short_description: row.get(3)?,
        content: row.get(4)?,
        image_url: row.get(5)?,
        created_at: row.get(6)?,
    })
}

pub fn all_articles(conn: &Connection) -> rusqlite::Result<Vec<Article>> {
    let mut stmt = conn.prepare(&format!("{SELECT} ORDER BY created_at DESC"))?;
    stmt.query_map([], map_row)?.collect()
}

pub fn article_by_id(conn: &Connection, id: i64) -> rusqlite::Result<Option<Article>> {
    conn.query_row(&format!("{SELECT} WHERE id = ?1"), [id], map_row)
        .optional()
}
