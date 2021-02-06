#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use snafu::{ResultExt, Snafu};
pub mod cognitive_services_accounts {
    use crate::models::*;
    use snafu::{ResultExt, Snafu};
    pub async fn get_properties(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<CognitiveServicesAccount, get_properties::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.CognitiveServices/accounts/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            account_name
        );
        let mut url = url::Url::parse(url_str).context(get_properties::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(get_properties::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(get_properties::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .context(get_properties::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: CognitiveServicesAccount =
                    serde_json::from_slice(rsp_body).context(get_properties::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: Error =
                    serde_json::from_slice(rsp_body).context(get_properties::DeserializeError { body: rsp_body.clone() })?;
                get_properties::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod get_properties {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::Error,
            },
            ParseUrlError {
                source: url::ParseError,
            },
            BuildRequestError {
                source: http::Error,
            },
            ExecuteRequestError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            SerializeError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
    pub async fn create(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        parameters: &CognitiveServicesAccountCreateParameters,
        subscription_id: &str,
    ) -> std::result::Result<create::Response, create::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.CognitiveServices/accounts/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            account_name
        );
        let mut url = url::Url::parse(url_str).context(create::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PUT);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(create::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = azure_core::to_json(parameters).context(create::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(create::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(create::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: CognitiveServicesAccount =
                    serde_json::from_slice(rsp_body).context(create::DeserializeError { body: rsp_body.clone() })?;
                Ok(create::Response::Ok200(rsp_value))
            }
            http::StatusCode::CREATED => {
                let rsp_body = rsp.body();
                let rsp_value: CognitiveServicesAccount =
                    serde_json::from_slice(rsp_body).context(create::DeserializeError { body: rsp_body.clone() })?;
                Ok(create::Response::Created201(rsp_value))
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: Error = serde_json::from_slice(rsp_body).context(create::DeserializeError { body: rsp_body.clone() })?;
                create::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod create {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(CognitiveServicesAccount),
            Created201(CognitiveServicesAccount),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::Error,
            },
            ParseUrlError {
                source: url::ParseError,
            },
            BuildRequestError {
                source: http::Error,
            },
            ExecuteRequestError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            SerializeError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
    pub async fn update(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        body: &CognitiveServicesAccountUpdateParameters,
        subscription_id: &str,
    ) -> std::result::Result<CognitiveServicesAccount, update::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.CognitiveServices/accounts/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            account_name
        );
        let mut url = url::Url::parse(url_str).context(update::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PATCH);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(update::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = azure_core::to_json(body).context(update::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(update::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(update::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: CognitiveServicesAccount =
                    serde_json::from_slice(rsp_body).context(update::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: Error = serde_json::from_slice(rsp_body).context(update::DeserializeError { body: rsp_body.clone() })?;
                update::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod update {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::Error,
            },
            ParseUrlError {
                source: url::ParseError,
            },
            BuildRequestError {
                source: http::Error,
            },
            ExecuteRequestError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            SerializeError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.CognitiveServices/accounts/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            account_name
        );
        let mut url = url::Url::parse(url_str).context(delete::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::DELETE);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(delete::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(delete::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(delete::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => Ok(delete::Response::Ok200),
            http::StatusCode::NO_CONTENT => Ok(delete::Response::NoContent204),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: Error = serde_json::from_slice(rsp_body).context(delete::DeserializeError { body: rsp_body.clone() })?;
                delete::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod delete {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::Error,
            },
            ParseUrlError {
                source: url::ParseError,
            },
            BuildRequestError {
                source: http::Error,
            },
            ExecuteRequestError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            SerializeError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
    pub async fn list_by_resource_group(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<CognitiveServicesAccountListResult, list_by_resource_group::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.CognitiveServices/accounts",
            operation_config.base_path(),
            subscription_id,
            resource_group_name
        );
        let mut url = url::Url::parse(url_str).context(list_by_resource_group::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(list_by_resource_group::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(list_by_resource_group::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .context(list_by_resource_group::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: CognitiveServicesAccountListResult =
                    serde_json::from_slice(rsp_body).context(list_by_resource_group::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: Error =
                    serde_json::from_slice(rsp_body).context(list_by_resource_group::DeserializeError { body: rsp_body.clone() })?;
                list_by_resource_group::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list_by_resource_group {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::Error,
            },
            ParseUrlError {
                source: url::ParseError,
            },
            BuildRequestError {
                source: http::Error,
            },
            ExecuteRequestError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            SerializeError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
    pub async fn list(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
    ) -> std::result::Result<CognitiveServicesAccountListResult, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.CognitiveServices/accounts",
            operation_config.base_path(),
            subscription_id
        );
        let mut url = url::Url::parse(url_str).context(list::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(list::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(list::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: CognitiveServicesAccountListResult =
                    serde_json::from_slice(rsp_body).context(list::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: Error = serde_json::from_slice(rsp_body).context(list::DeserializeError { body: rsp_body.clone() })?;
                list::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::Error,
            },
            ParseUrlError {
                source: url::ParseError,
            },
            BuildRequestError {
                source: http::Error,
            },
            ExecuteRequestError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            SerializeError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
    pub async fn list_keys(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<CognitiveServicesAccountKeys, list_keys::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.CognitiveServices/accounts/{}/listKeys",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            account_name
        );
        let mut url = url::Url::parse(url_str).context(list_keys::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(list_keys::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.header(http::header::CONTENT_LENGTH, 0);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(list_keys::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(list_keys::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: CognitiveServicesAccountKeys =
                    serde_json::from_slice(rsp_body).context(list_keys::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: Error = serde_json::from_slice(rsp_body).context(list_keys::DeserializeError { body: rsp_body.clone() })?;
                list_keys::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list_keys {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::Error,
            },
            ParseUrlError {
                source: url::ParseError,
            },
            BuildRequestError {
                source: http::Error,
            },
            ExecuteRequestError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            SerializeError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
    pub async fn regenerate_key(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        body: &RegenerateKeyParameters,
        subscription_id: &str,
    ) -> std::result::Result<CognitiveServicesAccountKeys, regenerate_key::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.CognitiveServices/accounts/{}/regenerateKey",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            account_name
        );
        let mut url = url::Url::parse(url_str).context(regenerate_key::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(regenerate_key::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = azure_core::to_json(body).context(regenerate_key::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(regenerate_key::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .context(regenerate_key::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: CognitiveServicesAccountKeys =
                    serde_json::from_slice(rsp_body).context(regenerate_key::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: Error =
                    serde_json::from_slice(rsp_body).context(regenerate_key::DeserializeError { body: rsp_body.clone() })?;
                regenerate_key::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod regenerate_key {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::Error,
            },
            ParseUrlError {
                source: url::ParseError,
            },
            BuildRequestError {
                source: http::Error,
            },
            ExecuteRequestError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            SerializeError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
    pub async fn list_skus(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<CognitiveServicesAccountEnumerateSkusResult, list_skus::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.CognitiveServices/accounts/{}/skus",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            account_name
        );
        let mut url = url::Url::parse(url_str).context(list_skus::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(list_skus::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(list_skus::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(list_skus::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: CognitiveServicesAccountEnumerateSkusResult =
                    serde_json::from_slice(rsp_body).context(list_skus::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: Error = serde_json::from_slice(rsp_body).context(list_skus::DeserializeError { body: rsp_body.clone() })?;
                list_skus::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list_skus {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::Error,
            },
            ParseUrlError {
                source: url::ParseError,
            },
            BuildRequestError {
                source: http::Error,
            },
            ExecuteRequestError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            SerializeError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
}
