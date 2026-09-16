mod common;

// Participant contexts and their configuration are global (admin scoped) V5
// resources of the virtual connector: every test authenticates as the admin.
#[allow(clippy::unwrap_used)]
mod participants {
    use std::collections::HashMap;

    use edc_connector_client::{
        types::{
            dataspace_profile::{DataspaceProfile, DataspaceProtocol},
            participants::{
                NewParticipantContext, ParticipantContext, ParticipantContextConfig,
                ParticipantContextConfigPatch, ParticipantContextState,
            },
        },
        EdcConnectorApiVersion, Error, ManagementApiError, ManagementApiErrorDetailKind,
    };
    use reqwest::StatusCode;
    use rstest::rstest;
    use uuid::Uuid;

    use crate::common::{provider_virtual_edc, setup_admin_client, ClientParams};

    fn new_participant(id: &str) -> NewParticipantContext {
        NewParticipantContext::builder()
            .id(id)
            .identity(id)
            .property("region", "eu")
            .build()
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_create_and_get_a_participant_context(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);
        let id = format!("participant-{}", Uuid::new_v4());

        let response = client
            .participants(version)
            .create(&new_participant(&id))
            .await
            .unwrap();
        assert_eq!(&id, response.id());

        let ctx = client.participants(version).get(&id).await.unwrap();

        assert_eq!(id, ctx.id());
        assert_eq!(id, ctx.identity());
        assert_eq!(&ParticipantContextState::Created, ctx.state());
        assert_eq!(
            Some("eu".to_string()),
            ctx.property::<String>("region").unwrap()
        );

        client.participants(version).delete(&id).await.unwrap();
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_list_participant_contexts(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);
        let id = format!("participant-{}", Uuid::new_v4());

        client
            .participants(version)
            .create(&new_participant(&id))
            .await
            .unwrap();

        let participants = client.participants(version).list(0, 1000).await.unwrap();

        assert!(participants.iter().any(|p| p.id() == id));

        let first_page = client.participants(version).list(0, 1).await.unwrap();
        assert_eq!(1, first_page.len());

        client.participants(version).delete(&id).await.unwrap();
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_update_a_participant_context(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);
        let id = format!("participant-{}", Uuid::new_v4());

        client
            .participants(version)
            .create(&new_participant(&id))
            .await
            .unwrap();

        let updated = ParticipantContext::builder()
            .id(&id)
            .identity(format!("did:web:{id}"))
            .property("region", "us")
            .build();

        client.participants(version).update(&updated).await.unwrap();

        let ctx = client.participants(version).get(&id).await.unwrap();

        assert_eq!(format!("did:web:{id}"), ctx.identity());
        assert_eq!(
            Some("us".to_string()),
            ctx.property::<String>("region").unwrap()
        );

        client.participants(version).delete(&id).await.unwrap();
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_delete_a_participant_context(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);
        let id = format!("participant-{}", Uuid::new_v4());

        client
            .participants(version)
            .create(&new_participant(&id))
            .await
            .unwrap();

        client.participants(version).delete(&id).await.unwrap();

        let response = client.participants(version).get(&id).await;

        assert!(matches!(
            response,
            Err(Error::ManagementApi(ManagementApiError {
                status_code: StatusCode::NOT_FOUND,
                error_detail: ManagementApiErrorDetailKind::Parsed(..)
            }))
        ))
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_fail_to_get_a_participant_context_when_not_existing(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);

        let response = client
            .participants(version)
            .get(&Uuid::new_v4().to_string())
            .await;

        assert!(matches!(
            response,
            Err(Error::ManagementApi(ManagementApiError {
                status_code: StatusCode::NOT_FOUND,
                error_detail: ManagementApiErrorDetailKind::Parsed(..)
            }))
        ))
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_save_get_and_patch_the_participant_config(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);
        let id = format!("participant-{}", Uuid::new_v4());

        client
            .participants(version)
            .create(&new_participant(&id))
            .await
            .unwrap();

        let mut entries = HashMap::new();
        entries.insert("edc.participant.id".to_string(), id.clone());
        entries.insert("key1".to_string(), "value1".to_string());
        let mut private_entries = HashMap::new();
        private_entries.insert("secret".to_string(), "s3cret".to_string());

        client
            .participant_configs(version)
            .save(
                &id,
                &ParticipantContextConfig::builder()
                    .entries(entries)
                    .private_entries(private_entries)
                    .build(),
            )
            .await
            .unwrap();

        let config = client.participant_configs(version).get(&id).await.unwrap();

        assert_eq!(Some(&"value1".to_string()), config.entries().get("key1"));
        assert_eq!(Some(&id), config.entries().get("edc.participant.id"));
        assert_eq!(
            Some(&"s3cret".to_string()),
            config.private_entries().get("secret")
        );

        client
            .participant_configs(version)
            .patch(
                &id,
                &ParticipantContextConfigPatch::builder()
                    .entry("key2", "value2")
                    .remove_entry("key1")
                    .build(),
            )
            .await
            .unwrap();

        let config = client.participant_configs(version).get(&id).await.unwrap();

        assert_eq!(None, config.entries().get("key1"));
        assert_eq!(Some(&"value2".to_string()), config.entries().get("key2"));
        assert_eq!(Some(&id), config.entries().get("edc.participant.id"));

        client.participants(version).delete(&id).await.unwrap();
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_get_the_profiles_of_a_participant_context(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let participant_context = provider.participant_context.clone().unwrap();
        let client = setup_admin_client(provider, version);

        // The test stack enables every profile for every participant.
        let profiles = client
            .participants(version)
            .profiles(&participant_context)
            .await
            .unwrap();

        assert!(profiles
            .iter()
            .any(|profile| profile.name() == "http-dsp-profile-2025-1"));
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_associate_profiles_to_a_participant_context(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);
        let id = format!("participant-{}", Uuid::new_v4());
        let profile_name = format!("profile-{}", Uuid::new_v4());

        client
            .dataspace_profiles(version)
            .create(
                &DataspaceProfile::builder()
                    .name(&profile_name)
                    .protocol(
                        DataspaceProtocol::builder()
                            .version("2025-1")
                            .path(format!("/{profile_name}"))
                            .binding("HTTPS")
                            .namespace("https://w3id.org/dspace/2025/1/")
                            .build(),
                    )
                    .json_ld_context_url("https://w3id.org/dspace/2025/1/context.jsonld")
                    .build(),
            )
            .await
            .unwrap();

        client
            .participants(version)
            .create(&new_participant(&id))
            .await
            .unwrap();

        client
            .participants(version)
            .associate_profiles(&id, &[&profile_name])
            .await
            .unwrap();

        let profiles = client.participants(version).profiles(&id).await.unwrap();

        assert!(profiles
            .iter()
            .any(|profile| profile.name() == profile_name));

        client.participants(version).delete(&id).await.unwrap();
        client
            .dataspace_profiles(version)
            .delete(&profile_name)
            .await
            .unwrap();
    }
}
