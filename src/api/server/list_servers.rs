use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::api::server::{GetServerResponse, ServerIdOrName};
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct ListServerRequest {
    #[serde(skip)]
    #[builder(default = 1)]
    pub count: isize,
    #[serde(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<ServerIdOrName>,
    #[serde(skip)]
    #[builder(default = 0)]
    pub offset: isize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListServerResponse {
    pub total_count: isize,
    pub servers: Vec<GetServerResponse>,
}

impl Endpoint for ListServerRequest {
    type Request = ListServerRequest;
    type Response = ListServerResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        if let Some(name) = &self.name {
            format!(
                "/servers?count={}&offset={}&name={}",
                self.count, self.offset, &name
            )
            .into()
        } else {
            format!("/servers?count={}&offset={}", self.count, self.offset).into()
        }
    }

    fn body(&self) -> &Self::Request {
        self
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

#[cfg(test)]
mod tests {
    use httptest::{Expectation, responders::*, Server};
    use httptest::matchers::request;
    use serde_json::json;

    use crate::Query;
    use crate::reqwest::PostmarkClient;

    use super::*;

    const SERVER_NAME: &str = "TEST-NAME";
    #[tokio::test]
    pub async fn get_server() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                format!("/servers"),
            ))
            .respond_with(json_encoded(json!({
              "TotalCount": 2,
              "Servers": [
                {
                  "ID": 1,
                  "Name": "Production01",
                  "ApiTokens": [
                    "server token"
                  ],
                  "Color": "red",
                  "SmtpApiActivated": true,
                  "RawEmailEnabled": false,
                  "DeliveryType": "Live",
                  "ServerLink": "https://postmarkapp.com/servers/1/streams",
                  "InboundAddress": "yourhash@inbound.postmarkapp.com",
                  "InboundHookUrl": "http://inboundhook.example.com/inbound",
                  "BounceHookUrl": "http://bouncehook.example.com/bounce",
                  "OpenHookUrl": "http://openhook.example.com/open",
                  "DeliveryHookUrl": "http://hooks.example.com/delivery",
                  "PostFirstOpenOnly": true,
                  "InboundDomain": "",
                  "InboundHash": "yourhash",
                  "InboundSpamThreshold": 5,
                  "TrackOpens": false,
                  "TrackLinks": "None",
                  "IncludeBounceContentInHook": true,
                  "ClickHookUrl": "http://hooks.example.com/click",
                  "EnableSmtpApiErrorHooks": false
                },
                {
                  "ID": 2,
                  "Name": "Production02",
                  "ApiTokens": [
                    "server token"
                  ],
                  "Color": "green",
                  "SmtpApiActivated": true,
                  "RawEmailEnabled": false,
                  "DeliveryType": "Sandbox",
                  "ServerLink": "https://postmarkapp.com/servers/2/streams",
                  "InboundAddress": "yourhash@inbound.postmarkapp.com",
                  "InboundHookUrl": "",
                  "BounceHookUrl": "",
                  "OpenHookUrl": "",
                  "DeliveryHookUrl": "http://hooks.example.com/delivery",
                  "PostFirstOpenOnly": false,
                  "InboundDomain": "",
                  "InboundHash": "yourhash",
                  "InboundSpamThreshold": 0,
                  "TrackOpens": true,
                  "TrackLinks": "HtmlAndText",
                  "IncludeBounceContentInHook": false,
                  "ClickHookUrl": "",
                  "EnableSmtpApiErrorHooks": false
                }
              ]
            }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = ListServerRequest::builder()
            .name(Some(ServerIdOrName::ServerName(String::from(SERVER_NAME))))
            .build();

        print!("{}\n", req.endpoint());

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }
}
