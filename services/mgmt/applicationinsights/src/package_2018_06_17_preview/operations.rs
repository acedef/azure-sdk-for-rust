#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use snafu::{ResultExt, Snafu};
pub mod workbooks {
    use crate::models::*;
    use snafu::{ResultExt, Snafu};
    pub async fn list_by_resource_group(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        category: &str,
        tags: &Vec<&str>,
        source_id: &str,
        can_fetch_content: Option<bool>,
    ) -> std::result::Result<WorkbooksListResult, list_by_resource_group::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/microsoft.insights/workbooks",
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
        url.query_pairs_mut().append_pair("category", category);
        url.query_pairs_mut().append_pair("sourceId", source_id);
        if let Some(can_fetch_content) = can_fetch_content {
            url.query_pairs_mut()
                .append_pair("canFetchContent", can_fetch_content.to_string().as_str());
        }
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
                let rsp_value: WorkbooksListResult =
                    serde_json::from_slice(rsp_body).context(list_by_resource_group::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: WorkbookError =
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
                value: models::WorkbookError,
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
    pub async fn get(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> std::result::Result<Workbook, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/microsoft.insights/workbooks/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            resource_name
        );
        let mut url = url::Url::parse(url_str).context(get::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(get::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(get::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(get::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Workbook = serde_json::from_slice(rsp_body).context(get::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: WorkbookError =
                    serde_json::from_slice(rsp_body).context(get::DeserializeError { body: rsp_body.clone() })?;
                get::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::WorkbookError,
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
    pub async fn create_or_update(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
        source_id: &str,
        workbook_properties: &Workbook,
    ) -> std::result::Result<create_or_update::Response, create_or_update::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/microsoft.insights/workbooks/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            resource_name
        );
        let mut url = url::Url::parse(url_str).context(create_or_update::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PUT);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(create_or_update::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        url.query_pairs_mut().append_pair("sourceId", source_id);
        let req_body = azure_core::to_json(workbook_properties).context(create_or_update::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(create_or_update::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .context(create_or_update::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Workbook =
                    serde_json::from_slice(rsp_body).context(create_or_update::DeserializeError { body: rsp_body.clone() })?;
                Ok(create_or_update::Response::Ok200(rsp_value))
            }
            http::StatusCode::CREATED => {
                let rsp_body = rsp.body();
                let rsp_value: Workbook =
                    serde_json::from_slice(rsp_body).context(create_or_update::DeserializeError { body: rsp_body.clone() })?;
                Ok(create_or_update::Response::Created201(rsp_value))
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: WorkbookError =
                    serde_json::from_slice(rsp_body).context(create_or_update::DeserializeError { body: rsp_body.clone() })?;
                create_or_update::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod create_or_update {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(Workbook),
            Created201(Workbook),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::WorkbookError,
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
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
        source_id: &str,
        workbook_update_parameters: Option<&WorkbookUpdateParameters>,
    ) -> std::result::Result<Workbook, update::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/microsoft.insights/workbooks/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            resource_name
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
        url.query_pairs_mut().append_pair("sourceId", source_id);
        let req_body = if let Some(workbook_update_parameters) = workbook_update_parameters {
            azure_core::to_json(workbook_update_parameters).context(update::SerializeError)?
        } else {
            bytes::Bytes::from_static(azure_core::EMPTY_BODY)
        };
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(update::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(update::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Workbook = serde_json::from_slice(rsp_body).context(update::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: WorkbookError =
                    serde_json::from_slice(rsp_body).context(update::DeserializeError { body: rsp_body.clone() })?;
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
                value: models::WorkbookError,
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
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/microsoft.insights/workbooks/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            resource_name
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
            http::StatusCode::CREATED => Ok(delete::Response::Created201),
            http::StatusCode::NO_CONTENT => Ok(delete::Response::NoContent204),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: WorkbookError =
                    serde_json::from_slice(rsp_body).context(delete::DeserializeError { body: rsp_body.clone() })?;
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
            Created201,
            NoContent204,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::WorkbookError,
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
