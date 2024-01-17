#[cfg(test)]
mod tests {
    use anyhow::Result;

    use paste::store::PastebinStore;

    #[test]
    fn full() -> Result<()> {
        let store = PastebinStore::new(true)?;

        let r = store.pastebin_create_table()?;
        assert_eq!(r, 0);

        let id = store.pastebin_set(
            String::from("Ciao mi chiamo Michele Maione"),
        )?;
        assert_eq!(id, 1);

        let p = store.pastebin_get(id)?;
        assert_eq!(p.is_some(), true);
        assert_eq!(p.unwrap().id, id);

        let deleted = store.pastebin_delete(id)?;
        assert_eq!(deleted, true);

        let p = store.pastebin_get(id)?;
        assert_eq!(p.is_none(), true);

        Ok(())
    }
}