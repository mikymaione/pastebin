/*
MIT License

Copyright (c) 2024 Michele Maione

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
use anyhow::Result;
use rusqlite::Connection;

use crate::paste::Pastebin;

fn get_connection() -> Result<Connection> {
    let c = Connection::open("pastebin.db")?;

    Ok(c)
}

pub fn pastebin_create_table() -> Result<usize> {
    let conn = get_connection().unwrap();

    let i = conn.execute(
        "create table if not exists pastebin (
            id integer primary key,
            content text
        )",
        [],
    )?;

    Ok(i)
}

pub fn pastebin_get(id: i64) -> Result<Pastebin> {
    let conn = get_connection()?;

    let mut stmt = conn.prepare(
        "SELECT id, content FROM pastebin WHERE id = ?1"
    )?;

    let mut rows = stmt.query_map([id], |row| {
        Ok(
            Pastebin {
                id: row.get(0)?,
                content: row.get(1)?,
            }
        )
    })?;

    Ok(rows.next().unwrap().unwrap())
}

pub fn pastebin_set(req_body: String) -> Result<i64> {
    let conn = get_connection()?;

    conn.execute(
        "INSERT INTO pastebin (content) VALUES (?1)",
        [req_body],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn pastebin_delete(id: i64) -> Result<bool> {
    let conn = get_connection()?;

    let r = conn.execute(
        "DELETE FROM pastebin WHERE id = ?1",
        [id],
    )?;

    Ok(r > 0)
}