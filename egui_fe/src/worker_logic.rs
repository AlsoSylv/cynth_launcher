use async_channel::Sender;
use launcher_core::account::auth::{
    authorization_token_response, device_response, minecraft_ownership_response,
    minecraft_profile_response, minecraft_response, refresh_token_response, xbox_response,
    xbox_security_token_response,
};
use launcher_core::account::types::Account;
use launcher_core::types::{AssetIndexJson, Version, VersionJson, VersionManifest};
use launcher_core::Error;
use reqwest::Client;
use std::path::Path;
use std::sync::Arc;

pub const CLIENT_ID: &str = "04bc8538-fc3c-4490-9e61-a2b3f4cbcf5c";

pub enum Message {
    Versions,
    Auth,
}

pub enum Response {
    Versions(Result<VersionManifest, Error>),
    Version(Result<Box<VersionJson>, Error>),
    Libraries(Result<String, Error>, Arc<Version>),
    AssetIndex(Result<AssetIndexJson, Error>, Arc<Version>),
    Asset(Result<(), Error>, Arc<Version>),
    Jar(Result<String, Error>, Arc<Version>),
    Auth(Result<Account, Error>),
    JavaMajorVersion(Result<u32, Error>),
    DefaultJavaVersion(Result<u32, Error>),
}

#[derive(Clone)]
pub struct State {
    pub client: Client,
    pub launcher_core: Arc<launcher_core::AsyncLauncher>,
    pub tx: Sender<EarlyMessage>,
}

pub enum EarlyMessage {
    LinkCode((String, String)),
}

pub fn worker_event_loop(
    message: Message,
    state: &State,
) -> impl std::future::Future<Output = Response> {
    let client = state.client.clone();
    let launcher_core = state.launcher_core.clone();
    let tx = state.tx.clone();
    let path = Path::new("./");
    async move {
        match message {
            Message::Versions => {
                let versions = launcher_core
                    .get_version_manifest(&path.join("versions"))
                    .await;
                Response::Versions(versions)
            }
            Message::Auth => {
                let result =
                    auth_or_refresh(&client, &tx, Path::new("./refresh.txt"), CLIENT_ID).await;
                Response::Auth(result)
            }
        }
    }
}

async fn auth_or_refresh(
    client: &Client,
    tx: &Sender<EarlyMessage>,
    path: &Path,
    client_id: &str,
) -> Result<Account, Error> {
    let exists = tokio::fs::try_exists(path).await?;
    let auth_res = if exists {
        let token = tokio::fs::read_to_string(path).await?;
        refresh_token_response(client, &token, client_id)
            .await?
            .into()
    } else {
        // https://wiki.vg/Microsoft_Authentication_Scheme

        let device_response = device_response(client, client_id).await?;

        let code = device_response.user_code;
        let ms_url = device_response.verification_uri;

        tx.send(EarlyMessage::LinkCode((ms_url, code)))
            .await
            .unwrap();

        loop {
            let device_code = &device_response.device_code;
            let auth_hook = authorization_token_response(client, device_code, client_id).await;
            if let Ok(t) = auth_hook {
                break t;
            }
        }
    };

    let xbox_response = xbox_response(client, &auth_res.access_token).await?;

    let xbox_secure_token_res = xbox_security_token_response(client, &xbox_response.token).await?;

    let claims = &xbox_secure_token_res.display_claims;
    let token = &xbox_secure_token_res.token;
    let mc_res = minecraft_response(claims, token, client).await?;

    let ownership_check = minecraft_ownership_response(&mc_res.access_token, client).await?;

    if ownership_check.items.is_empty() {
        todo!("Is this worth checking?")
    }

    let profile = minecraft_profile_response(&mc_res.access_token, client).await?;

    tokio::fs::write(path, &auth_res.refresh_token).await?;

    use std::time::{Duration, SystemTime};

    let expires_in = Duration::from_secs(auth_res.expires_in);
    let system_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let combined_duration = system_time + expires_in;

    let account = Account {
        active: true,
        expiry: combined_duration.as_secs(),
        access_token: mc_res.access_token,
        profile,
    };

    Ok(account)
}
