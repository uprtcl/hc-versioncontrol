use crate::{blob::Blob, tree::Tree};
use boolinator::Boolinator;
use hdk::{
  entry_definition::ValidatingEntryType,
  error::ZomeApiResult,
  holochain_core_types::{
    cas::content::Address, dna::entry_types::Sharing, entry::Entry, error::HolochainError,
    json::JsonString,
  },
  AGENT_ADDRESS,
};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Commit {
  context_address: Address,

  author_address: Address,
  message: String,
  content_address: Address,
  parent_commits_addresses: Vec<Address>,
}

impl Commit {
  fn new(
    context_address: &Address,
    author_address: &Address,
    message: &str,
    content_address: &Address,
    parent_commits_addresses: &Vec<Address>,
  ) -> Commit {
    Commit {
      context_address: context_address.to_owned(),
      author_address: author_address.to_owned(),
      message: message.to_owned(),
      content_address: content_address.to_owned(),
      parent_commits_addresses: parent_commits_addresses.to_owned(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub enum CommitContent {
  ContentBlob(Blob),
  ContentTree(Tree),
}

pub fn definition() -> ValidatingEntryType {
  entry!(
    name: "commit",
    description: "a commit object",
    sharing: Sharing::Public,
    native_type: Commit,

    validation_package: || {
      hdk::ValidationPackageDefinition::ChainFull
    },

    validation: |commit: Commit, _ctx: hdk::ValidationData| {
      Ok(())
    },

    links: []
  )
}

/** Zome exposed functions */

/**
 * Retrieves the metadata information of the commit with the given address
 */
pub fn handle_get_commit_info(commit_address: Address) -> ZomeApiResult<Option<Entry>> {
  hdk::get_entry(&commit_address)
}

/**
 * Retrieves the contents of the commit with the given address
 */
pub fn handle_get_commit_content(commit_address: Address) -> ZomeApiResult<Option<Entry>> {
  if let Some(Entry::App(_, commit_entry)) = hdk::get_entry(&commit_address)? {
    let commit = Commit::try_from(commit_entry)?;

    return hdk::get_entry(&(commit.content_address));
    /* 
    Useful when full commit contents should be retrieved, to iterate deep into the tree
    if let Some(Entry::App(_, content_entry)) = hdk::get_entry(&commit.content_address)? {
      match Blob::try_from(content_entry) {
        Ok(blob) => Ok(Some(CommitContent::ContentBlob(blob))) as ZomeApiResult<Option<CommitContent>>,
        Err(_) => Ok(Some(CommitContent::ContentTree(Tree::try_from(
          content_entry,
        )?))),
      };
    } */
  }

  Ok(None)
}

/** Helper functions */

/**
 * Creates a new commit in the given context_address with the given properties
 */
pub fn create_commit(
  context_address: Address,
  message: String,
  content_address: Address,
  parent_commits: Vec<Address>,
) -> ZomeApiResult<Address> {
  let commit_entry = Entry::App(
    "commit".into(),
    Commit::new(
      &context_address,
      &AGENT_ADDRESS,
      &message,
      &content_address,
      &parent_commits,
    )
    .into(),
  );

  hdk::commit_entry(&commit_entry)
}
