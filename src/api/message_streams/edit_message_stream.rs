use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct EditMessageStreamRequest {
    #[serde(skip)]
    pub stream_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EditTemplateResponse {
    #[serde(rename = "ID")]
    pub id: String,
    pub name: String,
    pub description: String,
}

impl Endpoint for EditMessageStreamRequest {
    type Request = EditMessageStreamRequest;
    type Response = EditTemplateResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format! {"/message-streams/{}", self.stream_id}.into()
    }

    fn method(&self) -> http::Method {
        http::Method::PATCH
    }

    fn body(&self) -> &Self::Request {
        self
    }
}

#[cfg(test)]
mod tests {
    use httptest::matchers::request;
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use crate::reqwest::PostmarkClient;
    use crate::Query;

    use super::*;

    const NAME: &str = "Onboarding Email";
    const DESCRIPTION: &str = "Describing the message stream edited";

    #[tokio::test]
    pub async fn edit_message_stream() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("PATCH", "/message-streams/broadcast"))
                .respond_with(json_encoded(json!({
                    "ID": "broadcast",
                    "ServerID": 123457,
                    "Name": "Updated Dev Stream",
                    "Description": "Updating my dev transactional stream",
                    "MessageStreamType": "Transactional",
                    "CreatedAt": "2020-07-02T00:00:00-04:00",
                    "UpdatedAt": "2020-07-03T00:00:00-04:00",
                    "ArchivedAt": null,
                    "ExpectedPurgeDate": null,
                    "SubscriptionManagementConfiguration": {
                        "UnsubscribeHandlingType": "none"
                    }
                }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = EditMessageStreamRequest::builder()
            .stream_id(String::from("broadcast"))
            .name(String::from(NAME))
            .description(String::from(DESCRIPTION))
            .build();

        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({
                "Name": NAME,
                "Description": DESCRIPTION,
            })
        );

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }
}
