use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    basic::{BasicRequestTokenError, BasicTokenResponse},
    AuthUrl, AuthorizationCode, ClientId, CsrfToken, RedirectUrl, RefreshToken, TokenResponse,
    TokenUrl,
};
use oauth2::{ClientSecret, Scope};
use std::path::PathBuf;
use url::Url;

use crate::error::Error;
use crate::token::local_server;
use crate::token::Token;

type RequestTokenError = BasicRequestTokenError<oauth2::reqwest::Error<reqwest::Error>>;

#[derive(Debug)]
pub(super) struct Authorizer {
    client: BasicClient,
    certs_dir: PathBuf,
}

impl Authorizer {
    pub(super) fn new(
        app_key: String,
        secret: String,
        redirect_url: String,
        certs_dir: PathBuf,
    ) -> Self {
        let app_key = ClientId::new(app_key);
        let secret = ClientSecret::new(secret);
        let auth_url = AuthUrl::new("https://api.schwabapi.com/v1/oauth/authorize".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://api.schwabapi.com/v1/oauth/token".to_string())
            .expect("Invalid token endpoint URL");
        let redirect_url = RedirectUrl::new(redirect_url).expect("Invalid redirect URL");

        let client = BasicClient::new(app_key, Some(secret), auth_url, Some(token_url))
            .set_redirect_uri(redirect_url);
        Authorizer { client, certs_dir }
    }

    async fn authorize(&self) -> Result<Token, RequestTokenError> {
        let (auth_url, csrf_token) = self.auth_code_url();

        match open::that(auth_url.as_ref()) {
            Ok(()) => println!("Opened '{auth_url}' successfully."),
            Err(err) => {
                print!("An error occurred when opening '{auth_url}': {err}");
                println!("Please Open this URL in your browser manually\n{auth_url}",);
            }
        }

        let auth_code = Self::auth_code(csrf_token, self.certs_dir.clone()).await;

        let token_result = self.refresh_token(auth_code).await?;
        // dbg!(&token_result);
        let token = Token {
            refresh: token_result
                .refresh_token()
                .expect("should have refresh_token")
                .secret()
                .to_string(),
            refresh_expires_in: chrono::Utc::now()
                .checked_add_signed(super::REFRESH_TOKEN_LIFETIME)
                .expect("refresh_expires_in"),
            access: token_result.access_token().secret().to_string(),
            access_expires_in: chrono::Utc::now()
                .checked_add_signed(super::ACCESS_TOKEN_LIFETIME)
                .expect("access_expires_in"),
            type_: token_result.token_type().as_ref().to_string(),
        };

        Ok(token)
    }

    fn auth_code_url(&self) -> (Url, CsrfToken) {
        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("readonly".to_string()))
            .url();
        (auth_url, csrf_token)
    }

    async fn auth_code(csrf_state: CsrfToken, certs_dir: PathBuf) -> AuthorizationCode {
        let code = local_server::local_server(csrf_state, certs_dir).await;

        AuthorizationCode::new(code)
    }

    async fn refresh_token(
        &self,
        auth_code: AuthorizationCode,
    ) -> Result<BasicTokenResponse, RequestTokenError> {
        self.client
            .exchange_code(auth_code)
            .request_async(async_http_client)
            .await
    }

    pub(super) async fn access_token(
        &self,
        refresh_token: &str,
    ) -> Result<BasicTokenResponse, RequestTokenError> {
        let refresh_token = RefreshToken::new(refresh_token.to_string());
        self.client
            .exchange_refresh_token(&refresh_token)
            .request_async(async_http_client)
            .await
    }

    pub(super) async fn save(&self, path: PathBuf) -> Result<Token, Error> {
        let token = self
            .authorize()
            .await
            .map_err(|e| Error::Token(e.to_string()))?;
        token.save(path)?;
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use std::{borrow::Cow, collections::HashMap};

    const REDIRECT_URL: &str = "https://127.0.0.1:8080";
    fn client_id_static() -> &'static str {
        #[allow(clippy::option_env_unwrap)]
        option_env!("SCHWAB_API_KEY").expect("There should be SCHWAB API KEY")
    }

    fn secret_static() -> &'static str {
        #[allow(clippy::option_env_unwrap)]
        option_env!("SCHWAB_SECRET").expect("There should be SCHWAB SECRET")
    }

    #[tokio::test]
    #[ignore = "Testing manually for browser verification. Should be --nocapture"]
    async fn test_auth() {
        let auth = Authorizer::new(
            client_id_static().to_string(),
            secret_static().to_string(),
            REDIRECT_URL.to_string(),
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/certs"),
        );

        let token = auth.authorize().await.unwrap();
        dbg!(&token);

        // test refresh access token
        let access_token = auth.access_token(&token.refresh).await.unwrap();
        dbg!(&access_token);
    }

    #[test]
    fn test_get_auth_code_url() {
        const CLIENTID: &str = "CLIENTID";
        const SECRET: &str = "SECRET";
        let auth = Authorizer::new(
            CLIENTID.to_string(),
            SECRET.to_string(),
            REDIRECT_URL.to_string(),
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/certs"),
        );

        let (auth_url, csrf_token) = auth.auth_code_url();

        println!("{auth_url:?}");
        assert_eq!(auth_url.scheme(), "https");
        assert_eq!(auth_url.host_str().unwrap(), "api.schwabapi.com");
        assert_eq!(auth_url.path(), "/v1/oauth/authorize");
        let pairs: HashMap<_, _> = auth_url.query_pairs().into_iter().collect();
        assert_eq!(pairs.len(), 5);
        assert_eq!(
            pairs.get(&Cow::Borrowed("state")).unwrap(),
            &Cow::Borrowed(csrf_token.secret().as_str())
        );
        assert_eq!(
            pairs.get(&Cow::Borrowed("response_type")).unwrap(),
            &Cow::Borrowed("code")
        );
        assert_eq!(
            pairs.get(&Cow::Borrowed("client_id")).unwrap(),
            &Cow::Borrowed(CLIENTID)
        );
        assert_eq!(
            pairs.get(&Cow::Borrowed("redirect_uri")).unwrap(),
            &Cow::Borrowed(REDIRECT_URL)
        );
        assert_eq!(
            pairs.get(&Cow::Borrowed("scope")).unwrap(),
            &Cow::Borrowed("readonly")
        );
        assert!(!csrf_token.secret().is_empty());
    }

    #[tokio::test]
    #[ignore = "If the test is performed manually on Linux, it may fail for HTTPS."]
    async fn test_get_auth_code() {
        let auth_code = tokio::spawn(Authorizer::auth_code(
            CsrfToken::new("CSRF".to_string()),
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/certs"),
        ));
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        let body = client
            .get("https://127.0.0.1:8080/?state=CSRF&code=code")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        assert_eq!(auth_code.await.unwrap().secret(), "code");
        assert_eq!(body, "Schwab returned the following code:\ncode\nYou can now safely close this browser window.");
    }
}
