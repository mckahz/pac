use std::collections::HashMap;

use crate::{
    ast::{
        canonical::{self, Type},
        ModuleName, Name,
    },
    report::error::tipe::Error,
};

pub struct Scheme(Vec<Name>, Type);

pub type Context = HashMap<Name, Scheme>;

pub fn type_check(modules: &HashMap<ModuleName, canonical::Module>) -> Result<(), Error> {
    Ok(())
}
