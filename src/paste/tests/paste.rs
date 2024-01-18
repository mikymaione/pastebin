/*
MIT License

Copyright (c) 2024 Michele Maione

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
#[cfg(test)]
mod tests {
    use anyhow::Result;

    use paste::store::PastebinStore;

    #[test]
    fn full() -> Result<()> {
        let store = PastebinStore::new(true)?;

        // first element of autoinc is always 1
        let id = store.add(
            String::from("Ciao mi chiamo Michele Maione"),
        )?;
        assert_eq!(id, 1);

        // get this paste
        let p = store.get(id)?;
        assert_eq!(p.is_some(), true);
        assert_eq!(p.unwrap().id, id);

        // delete this paste
        let deleted = store.delete(id)?;
        assert_eq!(deleted, true);

        // check this paste is none
        let p = store.get(id)?;
        assert_eq!(p.is_none(), true);

        Ok(())
    }
}