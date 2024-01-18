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