use std::fmt::Debug;

use common_utils::ext_traits::AsyncExt;
use error_stack::{report, IntoReport, ResultExt};
use masking::PeekInterface;
use maud::html;
use rand::{distributions::Uniform, prelude::Distribution};
use tokio::time as tokio;

use super::{
    consts, errors,
    types::{self, GetPaymentMethodDetails},
};
use crate::{configs::settings, routes::AppState};

pub async fn tokio_mock_sleep(delay: u64, tolerance: u64) {
    let mut rng = rand::thread_rng();
    // TODO: change this to `Uniform::try_from`
    // this would require changing the fn signature
    // to return a Result
    let effective_delay = Uniform::from((delay - tolerance)..(delay + tolerance));
    tokio::sleep(tokio::Duration::from_millis(
        effective_delay.sample(&mut rng),
    ))
    .await
}

pub async fn store_data_in_redis(
    state: &AppState,
    key: String,
    data: impl serde::Serialize + Debug,
    ttl: i64,
) -> types::DummyConnectorResult<()> {
    let redis_conn = state
        .store
        .get_redis_conn()
        .change_context(errors::DummyConnectorErrors::InternalServerError)
        .attach_printable("Failed to get redis connection")?;

    redis_conn
        .serialize_and_set_key_with_expiry(&key, data, ttl)
        .await
        .change_context(errors::DummyConnectorErrors::PaymentStoringError)
        .attach_printable("Failed to add data in redis")?;
    Ok(())
}

pub async fn get_payment_data_from_payment_id(
    state: &AppState,
    payment_id: String,
) -> types::DummyConnectorResult<types::DummyConnectorPaymentData> {
    let redis_conn = state
        .store
        .get_redis_conn()
        .change_context(errors::DummyConnectorErrors::InternalServerError)
        .attach_printable("Failed to get redis connection")?;

    redis_conn
        .get_and_deserialize_key::<types::DummyConnectorPaymentData>(
            payment_id.as_str(),
            "types DummyConnectorPaymentData",
        )
        .await
        .change_context(errors::DummyConnectorErrors::PaymentNotFound)
}

pub async fn get_payment_data_by_attempt_id(
    state: &AppState,
    attempt_id: String,
) -> types::DummyConnectorResult<types::DummyConnectorPaymentData> {
    let redis_conn = state
        .store
        .get_redis_conn()
        .change_context(errors::DummyConnectorErrors::InternalServerError)
        .attach_printable("Failed to get redis connection")?;

    redis_conn
        .get_and_deserialize_key::<String>(attempt_id.as_str(), "String")
        .await
        .async_and_then(|payment_id| async move {
            redis_conn
                .get_and_deserialize_key::<types::DummyConnectorPaymentData>(
                    payment_id.as_str(),
                    "DummyConnectorPaymentData",
                )
                .await
        })
        .await
        .change_context(errors::DummyConnectorErrors::PaymentNotFound)
}

pub fn get_authorize_page(
    payment_data: types::DummyConnectorPaymentData,
    return_url: String,
    dummy_connector_conf: &settings::DummyConnector,
) -> String {
    let mode = payment_data.payment_method_type.get_name();
    let image = payment_data
        .payment_method_type
        .get_image_link(dummy_connector_conf.assets_base_url.as_str());
    let connector_image = payment_data
        .connector
        .get_connector_image_link(dummy_connector_conf.assets_base_url.as_str());
    let currency = payment_data.currency.to_string();

    html! {
        head {
            title { "Authorize Payment" }
            style { (consts::THREE_DS_CSS) }
            link rel="icon" href=(connector_image) {}
        }
        body {
            div.heading {
                // img.logo src="../../../../../docs/imgs/logo.svg" alt="Bliver Logo" {}
                (maud::PreEscaped(
                    r#"<svg class="logo" version="1.1" id="Camada_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px"
                    viewBox="0 0 216 72" style="enable-background:new 0 0 216 72;" xml:space="preserve">
               <style type="text/css">
                   .st0{fill:none}
                   .st1{enable-background:new}
                   .st2{fill:#F46A35}
               </style>
               <rect x="3262.63" y="831.22" class="st0" width="0" height="2.34"/>
               <g>
                   <g class="st1">
                       <path d="M30.92,20.95c5.26,0,9.63,1.99,13.12,5.98c3.48,3.99,5.22,9.04,5.22,15.16c0,6.51-1.97,11.82-5.9,15.92
                           c-3.93,4.1-8.98,6.15-15.15,6.15c-6.17,0-11.23-2.02-15.19-6.06c-3.96-4.04-5.94-9.21-5.94-15.49V4.95h10.37V27.6
                           C20.27,23.17,24.76,20.95,30.92,20.95z M28.37,55.39c3.19,0,5.79-1.19,7.81-3.58c2.02-2.38,3.03-5.46,3.03-9.22
                           c0-3.82-1.01-6.92-3.03-9.3c-2.02-2.39-4.62-3.58-7.81-3.58c-3.24,0-5.87,1.19-7.89,3.58c-2.02,2.39-3.03,5.49-3.03,9.3
                           c0,3.76,1.01,6.84,3.03,9.22C22.5,54.2,25.13,55.39,28.37,55.39z"/>
                       <path d="M55.64,4.95h10.37v58.44H55.64V4.95z"/>
                       <path d="M84.31,5.84c1.2,1.15,1.79,2.62,1.79,4.42c0,1.8-0.6,3.26-1.79,4.38c-1.2,1.12-2.72,1.68-4.58,1.68
                           c-1.86,0-3.38-0.58-4.54-1.73c-1.17-1.15-1.75-2.6-1.75-4.34c0-1.8,0.58-3.27,1.75-4.42c1.17-1.15,2.68-1.73,4.54-1.73
                           C81.58,4.11,83.11,4.69,84.31,5.84z M74.54,21.79h10.45v41.6H74.54V21.79z"/>
                       <path d="M121.02,21.79h11.08l-15.15,41.6H104.6l-15.15-41.6h11.08l10.29,30.74L121.02,21.79z"/>
                       <path d="M175.56,42.68c0,1.29-0.11,2.39-0.32,3.28h-31.82c0.58,2.75,1.89,4.98,3.91,6.69c2.02,1.71,4.38,2.57,7.1,2.57
                           c2.29,0,4.33-0.63,6.14-1.9c1.81-1.26,3.08-2.85,3.83-4.76h10.45c-1.12,4.55-3.59,8.28-7.41,11.2c-3.83,2.92-8.19,4.38-13.08,4.38
                           c-6.17,0-11.26-2.02-15.27-6.06c-4.01-4.04-6.02-9.21-6.02-15.49c0-6.29,2.01-11.46,6.02-15.54c4.01-4.07,9.1-6.1,15.27-6.1
                           c6.11,0,11.16,2.04,15.15,6.1C173.49,31.13,175.51,36.33,175.56,42.68z M154.43,29.88c-2.45,0-4.61,0.7-6.5,2.11
                           c-1.89,1.4-3.26,3.28-4.11,5.64h21.61c-0.8-2.3-2.21-4.17-4.23-5.6S156.93,29.88,154.43,29.88z"/>
                       <path d="M191.83,63.39h-10.45V38.63c0-5.33,1.73-9.47,5.18-12.42c3.46-2.95,8.32-4.42,14.59-4.42v9.18
                           c-6.22,0-9.33,2.89-9.33,8.67V63.39z"/>
                   </g>
                   <g class="st1">
                       <path class="st2" d="M207.93,49.75c2.12,0,3.85,0.67,5.18,2.02c1.33,1.35,1.99,3.06,1.99,5.14c0,2.13-0.67,3.86-1.99,5.18
                           s-3.06,1.98-5.18,1.98c-2.07,0-3.78-0.67-5.1-2.02c-1.33-1.35-1.99-3.06-1.99-5.14c0-2.08,0.66-3.79,1.99-5.14
                           C204.16,50.42,205.86,49.75,207.93,49.75z"/>
                   </g>
               </g>
               </svg>"#)
                )
                h1 { "Test Payment Page" }
            }
            div.container {
                div.payment_details {
                    img src=(image) {}
                    div.border_horizontal {}
                    img src=(connector_image) {}
                }
                (maud::PreEscaped(
                    format!(r#"
                        <p class="disclaimer">
                            This is a test payment of <span id="amount"></span> {} using {}
                            <script>
                                document.getElementById("amount").innerHTML = ({} / 100).toFixed(2);
                            </script>
                        </p>
                        "#, currency, mode, payment_data.amount)
                    )
                )
                p { b { "Real money will not be debited for the payment." } " \
                        You can choose to simulate successful or failed payment while testing this payment." }
                div.user_action {
                    button.authorize onclick=(format!("window.location.href='{}?confirm=true'", return_url))
                        { "Complete Payment" }
                    button.reject onclick=(format!("window.location.href='{}?confirm=false'", return_url))
                        { "Reject Payment" }
                }
            }
            div.container {
                p.disclaimer { "What is this page?" }
                p { "This page is just a simulation for integration and testing purpose. \
                    In live mode, this page will not be displayed and the user will be taken to \
                    the Bank page (or) Google Pay cards popup (or) original payment method's page. \
                    Contact us for any queries."
                }
                div.contact {
                    div.contact_item.hover_cursor onclick=(dummy_connector_conf.slack_invite_url) {
                        img src="https://hyperswitch.io/logos/logo_slack.svg" alt="Slack Logo" {}
                    }
                    div.contact_item.hover_cursor onclick=(dummy_connector_conf.discord_invite_url) {
                        img src="https://hyperswitch.io/logos/logo_discord.svg" alt="Discord Logo" {}
                    }
                    div.border_vertical {}
                    div.contact_item.email {
                        p { "Or email us at" }
                        a href="mailto:callcenter@bliver.in" { "callcenter@bliver.in" }
                    }
                }
            }
        }
    }
    .into_string()
}

pub fn get_expired_page(dummy_connector_conf: &settings::DummyConnector) -> String {
    html! {
        head {
            title { "Authorize Payment" }
            style { (consts::THREE_DS_CSS) }
            link rel="icon" href="https://app.hyperswitch.io/HyperswitchFavicon.png" {}
        }
        body {
            div.heading {
                // img.logo src="https://app.hyperswitch.io/assets/Dark/hyperswitchLogoIconWithText.svg" alt="Hyperswitch Logo" {}
                (maud::PreEscaped(
                    r#"<svg class="logo" version="1.1" id="Camada_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px"
                    viewBox="0 0 216 72" style="enable-background:new 0 0 216 72;" xml:space="preserve">
               <style type="text/css">
                   .st0{fill:none}
                   .st1{enable-background:new}
                   .st2{fill:#F46A35}
               </style>
               <rect x="3262.63" y="831.22" class="st0" width="0" height="2.34"/>
               <g>
                   <g class="st1">
                       <path d="M30.92,20.95c5.26,0,9.63,1.99,13.12,5.98c3.48,3.99,5.22,9.04,5.22,15.16c0,6.51-1.97,11.82-5.9,15.92
                           c-3.93,4.1-8.98,6.15-15.15,6.15c-6.17,0-11.23-2.02-15.19-6.06c-3.96-4.04-5.94-9.21-5.94-15.49V4.95h10.37V27.6
                           C20.27,23.17,24.76,20.95,30.92,20.95z M28.37,55.39c3.19,0,5.79-1.19,7.81-3.58c2.02-2.38,3.03-5.46,3.03-9.22
                           c0-3.82-1.01-6.92-3.03-9.3c-2.02-2.39-4.62-3.58-7.81-3.58c-3.24,0-5.87,1.19-7.89,3.58c-2.02,2.39-3.03,5.49-3.03,9.3
                           c0,3.76,1.01,6.84,3.03,9.22C22.5,54.2,25.13,55.39,28.37,55.39z"/>
                       <path d="M55.64,4.95h10.37v58.44H55.64V4.95z"/>
                       <path d="M84.31,5.84c1.2,1.15,1.79,2.62,1.79,4.42c0,1.8-0.6,3.26-1.79,4.38c-1.2,1.12-2.72,1.68-4.58,1.68
                           c-1.86,0-3.38-0.58-4.54-1.73c-1.17-1.15-1.75-2.6-1.75-4.34c0-1.8,0.58-3.27,1.75-4.42c1.17-1.15,2.68-1.73,4.54-1.73
                           C81.58,4.11,83.11,4.69,84.31,5.84z M74.54,21.79h10.45v41.6H74.54V21.79z"/>
                       <path d="M121.02,21.79h11.08l-15.15,41.6H104.6l-15.15-41.6h11.08l10.29,30.74L121.02,21.79z"/>
                       <path d="M175.56,42.68c0,1.29-0.11,2.39-0.32,3.28h-31.82c0.58,2.75,1.89,4.98,3.91,6.69c2.02,1.71,4.38,2.57,7.1,2.57
                           c2.29,0,4.33-0.63,6.14-1.9c1.81-1.26,3.08-2.85,3.83-4.76h10.45c-1.12,4.55-3.59,8.28-7.41,11.2c-3.83,2.92-8.19,4.38-13.08,4.38
                           c-6.17,0-11.26-2.02-15.27-6.06c-4.01-4.04-6.02-9.21-6.02-15.49c0-6.29,2.01-11.46,6.02-15.54c4.01-4.07,9.1-6.1,15.27-6.1
                           c6.11,0,11.16,2.04,15.15,6.1C173.49,31.13,175.51,36.33,175.56,42.68z M154.43,29.88c-2.45,0-4.61,0.7-6.5,2.11
                           c-1.89,1.4-3.26,3.28-4.11,5.64h21.61c-0.8-2.3-2.21-4.17-4.23-5.6S156.93,29.88,154.43,29.88z"/>
                       <path d="M191.83,63.39h-10.45V38.63c0-5.33,1.73-9.47,5.18-12.42c3.46-2.95,8.32-4.42,14.59-4.42v9.18
                           c-6.22,0-9.33,2.89-9.33,8.67V63.39z"/>
                   </g>
                   <g class="st1">
                       <path class="st2" d="M207.93,49.75c2.12,0,3.85,0.67,5.18,2.02c1.33,1.35,1.99,3.06,1.99,5.14c0,2.13-0.67,3.86-1.99,5.18
                           s-3.06,1.98-5.18,1.98c-2.07,0-3.78-0.67-5.1-2.02c-1.33-1.35-1.99-3.06-1.99-5.14c0-2.08,0.66-3.79,1.99-5.14
                           C204.16,50.42,205.86,49.75,207.93,49.75z"/>
                   </g>
               </g>
               </svg>"#)
                )
                h1 { "Test Payment Page" }
            }
            div.container {
                p.disclaimer { "This link is not valid or it is expired" }
            }
            div.container {
                p.disclaimer { "What is this page?" }
                p { "This page is just a simulation for integration and testing purpose.\
                    In live mode, this is not visible. Contact us for any queries."
                }
                div.contact {
                    div.contact_item.hover_cursor onclick=(dummy_connector_conf.slack_invite_url) {
                        img src="https://hyperswitch.io/logos/logo_slack.svg" alt="Slack Logo" {}
                    }
                    div.contact_item.hover_cursor onclick=(dummy_connector_conf.discord_invite_url) {
                        img src="https://hyperswitch.io/logos/logo_discord.svg" alt="Discord Logo" {}
                    }
                    div.border_vertical {}
                    div.contact_item.email {
                        p { "Or email us at" }
                        a href="mailto:hyperswitch@juspay.in" { "hyperswitch@juspay.in" }
                    }
                }
            }
        }
    }
    .into_string()
}

pub trait ProcessPaymentAttempt {
    fn build_payment_data_from_payment_attempt(
        self,
        payment_attempt: types::DummyConnectorPaymentAttempt,
        redirect_url: String,
    ) -> types::DummyConnectorResult<types::DummyConnectorPaymentData>;
}

impl ProcessPaymentAttempt for types::DummyConnectorCard {
    fn build_payment_data_from_payment_attempt(
        self,
        payment_attempt: types::DummyConnectorPaymentAttempt,
        redirect_url: String,
    ) -> types::DummyConnectorResult<types::DummyConnectorPaymentData> {
        match self.get_flow_from_card_number()? {
            types::DummyConnectorCardFlow::NoThreeDS(status, error) => {
                if let Some(error) = error {
                    Err(error).into_report()?;
                }
                Ok(payment_attempt.build_payment_data(status, None, None))
            }
            types::DummyConnectorCardFlow::ThreeDS(_, _) => {
                Ok(payment_attempt.clone().build_payment_data(
                    types::DummyConnectorStatus::Processing,
                    Some(types::DummyConnectorNextAction::RedirectToUrl(redirect_url)),
                    payment_attempt.payment_request.return_url,
                ))
            }
        }
    }
}

impl types::DummyConnectorCard {
    pub fn get_flow_from_card_number(
        self,
    ) -> types::DummyConnectorResult<types::DummyConnectorCardFlow> {
        let card_number = self.number.peek();
        match card_number.as_str() {
            "4111111111111111" | "4242424242424242" | "5555555555554444" | "38000000000006"
            | "378282246310005" | "6011111111111117" => {
                Ok(types::DummyConnectorCardFlow::NoThreeDS(
                    types::DummyConnectorStatus::Succeeded,
                    None,
                ))
            }
            "5105105105105100" | "4000000000000002" => {
                Ok(types::DummyConnectorCardFlow::NoThreeDS(
                    types::DummyConnectorStatus::Failed,
                    Some(errors::DummyConnectorErrors::PaymentDeclined {
                        message: "Card declined",
                    }),
                ))
            }
            "4000000000009995" => Ok(types::DummyConnectorCardFlow::NoThreeDS(
                types::DummyConnectorStatus::Failed,
                Some(errors::DummyConnectorErrors::PaymentDeclined {
                    message: "Insufficient funds",
                }),
            )),
            "4000000000009987" => Ok(types::DummyConnectorCardFlow::NoThreeDS(
                types::DummyConnectorStatus::Failed,
                Some(errors::DummyConnectorErrors::PaymentDeclined {
                    message: "Lost card",
                }),
            )),
            "4000000000009979" => Ok(types::DummyConnectorCardFlow::NoThreeDS(
                types::DummyConnectorStatus::Failed,
                Some(errors::DummyConnectorErrors::PaymentDeclined {
                    message: "Stolen card",
                }),
            )),
            "4000003800000446" => Ok(types::DummyConnectorCardFlow::ThreeDS(
                types::DummyConnectorStatus::Succeeded,
                None,
            )),
            _ => Err(report!(errors::DummyConnectorErrors::CardNotSupported)
                .attach_printable("The card is not supported")),
        }
    }
}

impl ProcessPaymentAttempt for types::DummyConnectorWallet {
    fn build_payment_data_from_payment_attempt(
        self,
        payment_attempt: types::DummyConnectorPaymentAttempt,
        redirect_url: String,
    ) -> types::DummyConnectorResult<types::DummyConnectorPaymentData> {
        Ok(payment_attempt.clone().build_payment_data(
            types::DummyConnectorStatus::Processing,
            Some(types::DummyConnectorNextAction::RedirectToUrl(redirect_url)),
            payment_attempt.payment_request.return_url,
        ))
    }
}

impl ProcessPaymentAttempt for types::DummyConnectorPayLater {
    fn build_payment_data_from_payment_attempt(
        self,
        payment_attempt: types::DummyConnectorPaymentAttempt,
        redirect_url: String,
    ) -> types::DummyConnectorResult<types::DummyConnectorPaymentData> {
        Ok(payment_attempt.clone().build_payment_data(
            types::DummyConnectorStatus::Processing,
            Some(types::DummyConnectorNextAction::RedirectToUrl(redirect_url)),
            payment_attempt.payment_request.return_url,
        ))
    }
}

impl ProcessPaymentAttempt for types::DummyConnectorPaymentMethodData {
    fn build_payment_data_from_payment_attempt(
        self,
        payment_attempt: types::DummyConnectorPaymentAttempt,
        redirect_url: String,
    ) -> types::DummyConnectorResult<types::DummyConnectorPaymentData> {
        match self {
            Self::Card(card) => {
                card.build_payment_data_from_payment_attempt(payment_attempt, redirect_url)
            }
            Self::Wallet(wallet) => {
                wallet.build_payment_data_from_payment_attempt(payment_attempt, redirect_url)
            }
            Self::PayLater(pay_later) => {
                pay_later.build_payment_data_from_payment_attempt(payment_attempt, redirect_url)
            }
        }
    }
}

impl types::DummyConnectorPaymentData {
    pub fn process_payment_attempt(
        state: &AppState,
        payment_attempt: types::DummyConnectorPaymentAttempt,
    ) -> types::DummyConnectorResult<Self> {
        let redirect_url = format!(
            "{}/dummy-connector/authorize/{}",
            state.conf.server.base_url, payment_attempt.attempt_id
        );
        payment_attempt
            .clone()
            .payment_request
            .payment_method_data
            .build_payment_data_from_payment_attempt(payment_attempt, redirect_url)
    }
}
