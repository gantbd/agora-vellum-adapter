//! Effect-free identity conversion between Agora and Vellum.
//!
//! This crate deliberately accepts no Vellum client, authority, witness,
//! locator, content, or persistence type.
#![forbid(unsafe_code)]

use agora_protocol::v2::VellumDocumentTargetV1;
use vellum_protocol::{WorkspaceDocumentIdV1, WorkspaceIdV1};

/// A malformed scalar identity supplied outside either canonical wire codec.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConversionErrorV1 {
    InvalidWorkspace,
    InvalidDocument,
}

/// Projects one exact Vellum document revision into Agora's authority-free DTO.
pub fn from_vellum(
    document: WorkspaceDocumentIdV1,
    source_revision: u64,
) -> VellumDocumentTargetV1 {
    VellumDocumentTargetV1 {
        workspace_id: document.workspace().get(),
        document_id: document.get(),
        source_revision,
    }
}

/// Reconstructs Vellum's public restart-stable identities from an Agora DTO.
///
/// The result is still only identity plus historical revision. It is not read
/// or edit authority and cannot be used as a live `VersionWitness`.
pub fn to_vellum(
    target: VellumDocumentTargetV1,
) -> Result<(WorkspaceDocumentIdV1, u64), ConversionErrorV1> {
    let workspace =
        WorkspaceIdV1::new(target.workspace_id).ok_or(ConversionErrorV1::InvalidWorkspace)?;
    let document = WorkspaceDocumentIdV1::new(workspace, target.document_id)
        .ok_or(ConversionErrorV1::InvalidDocument)?;
    Ok((document, target.source_revision))
}
