use crate::{engines::KvsEngine, KvsError, KvsResult};
use sled::{Db, Tree};

#[derive(Clone)]
pub struct SledStore(Db);

impl SledStore {
    pub fn new(db: Db) -> Self {
        SledStore(db)
    }
}

impl KvsEngine for SledStore {
    fn set(&mut self, key: String, val: String) -> KvsResult<()> {
        let tree: &Tree = &self.0;
        tree.insert(key, val.into_bytes()).map(|_| ())?;
        tree.flush()?;

        Ok(())
    }

    fn get(&self, key: String) -> KvsResult<String> {
        let tree: &Tree = &self.0;

        Ok(tree
            .get(key)?
            .map(|i_vec| AsRef::<[u8]>::as_ref(&i_vec).to_vec())
            .map(String::from_utf8)
            .transpose()?
            .expect("Is None"))
    }

    fn remove(&mut self, key: String) -> KvsResult<()> {
        let tree: &Tree = &self.0;
        tree.remove(key)?.ok_or(KvsError::NotFound)?;
        tree.flush()?;

        Ok(())
    }
}
