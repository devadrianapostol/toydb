#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn put_get_recover() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let path = dir.path().join("test.db");

        // First session
        {
            let mut db = DB::open(&path)?;
            db.put("name".to_string(), "grok".to_string())?;
        } // dropped → flush

        // Recover
        let db = DB::open(&path)?;
        assert_eq!(db.get("name"), Some(&"grok".to_string()));
        Ok(())
    }
}