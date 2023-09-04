/*
 * Description: ???
 *
 * Copyright (C) 2023 Danny McClanahan <dmcC2@hypnicjerk.ai>
 * SPDX-License-Identifier: Apache-2.0
 *
 * Licensed under the Apache License, Version 2.0 (see LICENSE).
 */

//! ???

/* These clippy lint descriptions are purely non-functional and do not affect the functionality
 * or correctness of the code. */
// #![warn(missing_docs)]

/* Note: run clippy with: rustup run nightly cargo-clippy! */
#![deny(unsafe_code)]
/* Ensure any doctest warnings fails the doctest! */
#![doc(test(attr(deny(warnings))))]
/* Enable all clippy lints except for many of the pedantic ones. It's a shame this needs to be
 * copied and pasted across crates, but there doesn't appear to be a way to include inner
 * attributes from a common source. */
#![deny(
  clippy::all,
  clippy::default_trait_access,
  clippy::expl_impl_clone_on_copy,
  clippy::if_not_else,
  clippy::needless_continue,
  clippy::single_match_else,
  clippy::unseparated_literal_suffix,
  clippy::used_underscore_binding
)]
/* It is often more clear to show that nothing is being moved. */
#![allow(clippy::match_ref_pats)]
/* Subjective style. */
#![allow(
  clippy::derived_hash_with_manual_eq,
  clippy::len_without_is_empty,
  clippy::redundant_field_names,
  clippy::too_many_arguments,
  clippy::single_component_path_imports,
  clippy::double_must_use
)]
/* Default isn't as big a deal as people seem to think it is. */
#![allow(clippy::new_without_default, clippy::new_ret_no_self)]
/* Arc<Mutex> can be more clear than needing to grok Orderings. */
#![allow(clippy::mutex_atomic)]

use store::Store;
use task_executor::Executor;
use workunit_store::{Level, WorkunitStore};

use std::path::Path;

pub struct Handles {
  executor: Executor,
  fs_store: Store,
  workunit_store: WorkunitStore,
}

impl Handles {
  pub fn new(
    log_starting_workunits: bool,
    max_level: Level,
    local_store_path: impl AsRef<Path>,
  ) -> Result<Self, String> {
    let exe = Executor::new();
    let workunit_store = WorkunitStore::new(log_starting_workunits, max_level);
    let local_store = Store::local_only(exe.clone(), local_store_path)?;
    Ok(Self {
      executor: exe,
      fs_store: local_store,
      workunit_store,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use tempfile::tempdir;

  #[tokio::test]
  async fn it_works() {
    let local_store_path = tempdir().unwrap();
    let _handles = Handles::new(true, Level::Debug, local_store_path.path()).unwrap();
  }
}
