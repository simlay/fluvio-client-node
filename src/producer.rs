use crate::CLIENT_NOT_FOUND_ERROR_MSG;

use log::debug;
use fluvio::TopicProducer;
use fluvio::FluvioError;

use node_bindgen::derive::node_bindgen;
use node_bindgen::core::NjError;
use node_bindgen::core::val::JsEnv;
use node_bindgen::core::TryIntoJs;
use node_bindgen::sys::napi_value;
use node_bindgen::core::JSClass;

pub struct TopicProducerWrapper {
    client: TopicProducer,
}

impl TopicProducerWrapper {
    pub fn new(client: TopicProducer) -> Self {
        Self { client }
    }
}

impl TryIntoJs for TopicProducerWrapper {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        debug!("converting FluvioWrapper to js");
        let new_instance = TopicProducerJS::new_instance(js_env, vec![])?;
        debug!("instance created");
        TopicProducerJS::unwrap_mut(js_env, new_instance)?.set_client(self.client);
        Ok(new_instance)
    }
}

pub struct TopicProducerJS {
    inner: Option<TopicProducer>,
}

#[node_bindgen]
impl TopicProducerJS {
    #[node_bindgen(constructor)]
    pub fn new() -> Self {
        Self { inner: None }
    }

    pub fn set_client(&mut self, client: TopicProducer) {
        self.inner.replace(client);
    }

    #[node_bindgen]
    async fn send_record(&self, data: String, partition: i32) -> Result<(), FluvioError> {
        if let Some(client) = &self.inner {
            client.send_record(data.into_bytes(), partition).await?;
            Ok(())
        } else {
            Err(FluvioError::Other(CLIENT_NOT_FOUND_ERROR_MSG.to_owned()))
        }
    }
}
