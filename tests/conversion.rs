use agora_protocol::v2::VellumDocumentTargetV1;
use agora_vellum_adapter::{ConversionErrorV1, from_vellum, to_vellum};
use vellum_protocol::{WorkspaceDocumentIdV1, WorkspaceIdV1};

#[test]
fn exact_public_vellum_identities_and_source_revision_round_trip() {
    let workspace = WorkspaceIdV1::new(7).unwrap();
    let document = WorkspaceDocumentIdV1::new(workspace, 11).unwrap();
    let target = from_vellum(document, 13);
    assert_eq!(
        target,
        VellumDocumentTargetV1 {
            workspace_id: 7,
            document_id: 11,
            source_revision: 13,
        }
    );
    let (recovered, revision) = to_vellum(target).unwrap();
    assert_eq!(recovered, document);
    assert_eq!(revision, 13);
}

#[test]
fn zero_source_revision_is_historical_identity_not_a_live_witness() {
    let document = WorkspaceDocumentIdV1::new(WorkspaceIdV1::new(2).unwrap(), 3).unwrap();
    let target = from_vellum(document, 0);
    assert_eq!(to_vellum(target), Ok((document, 0)));
}

#[test]
fn malformed_direct_dtos_fail_closed_before_vellum_identity_construction() {
    assert_eq!(
        to_vellum(VellumDocumentTargetV1 {
            workspace_id: 0,
            document_id: 1,
            source_revision: 0,
        }),
        Err(ConversionErrorV1::InvalidWorkspace)
    );
    assert_eq!(
        to_vellum(VellumDocumentTargetV1 {
            workspace_id: 1,
            document_id: 0,
            source_revision: 0,
        }),
        Err(ConversionErrorV1::InvalidDocument)
    );
}
