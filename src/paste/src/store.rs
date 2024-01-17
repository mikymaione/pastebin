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

pub struct PastebinStore {
    connection: Connection,
}

impl PastebinStore {
    pub fn new(in_memory: bool) -> Result<Self> {
        Ok(
            Self {
                connection: if in_memory {
                    Connection::open_in_memory()?
                } else {
                    Connection::open("pastebin.db")?
                }
            }
        )
    }

    pub fn pastebin_create_table(&self) -> Result<usize> {
        let i = self.connection.execute(
            "create table if not exists pastebin (
            id integer primary key,
            content text
        )",
            [],
        )?;

        Ok(i)
    }

    pub fn pastebin_get(&self, id: i64) -> Result<Option<Pastebin>> {
        let mut stmt = self.connection.prepare(
            "SELECT id, content FROM pastebin WHERE id = ?1"
        )?;

        let mut rows = stmt.query_map([id], |row|
            Ok(
                Pastebin {
                    id: row.get(0)?,
                    content: row.get(1)?,
                }
            ),
        )?;

        let maybe_row = rows
            .next()
            .map_or(
                Ok(None),
                |v| v.map(Some),
            );

        Ok(maybe_row?)
    }

    pub fn pastebin_set(&self, req_body: String) -> Result<i64> {
        self.connection.execute(
            "INSERT INTO pastebin (content) VALUES (?1)",
            [req_body],
        )?;

        Ok(self.connection.last_insert_rowid())
    }

    pub fn pastebin_delete(&self, id: i64) -> Result<bool> {
        let r = self.connection.execute(
            "DELETE FROM pastebin WHERE id = ?1",
            [id],
        )?;

        Ok(r > 0)
    }
}