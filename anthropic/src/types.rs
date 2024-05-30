//! Module for types used in the API.
use std::pin::Pin;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use tokio_stream::Stream;

use crate::error::AnthropicError;
use crate::DEFAULT_MODEL;

#[derive(Clone, Serialize, Default, Debug, Builder, PartialEq)]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "AnthropicError"))]
pub struct CompleteRequest {
    /// The prompt to complete.
    pub prompt: String,
    /// The model to use.
    #[builder(default = "DEFAULT_MODEL")]
    pub model: Model,
    /// The number of tokens to sample.
    pub max_tokens_to_sample: usize,
    /// The stop sequences to use.
    pub stop_sequences: Option<Vec<String>>,
    /// Whether to incrementally stream the response.
    #[builder(default = "false")]
    pub stream: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CompleteResponse {
    pub completion: String,
    pub stop_reason: Option<StopReason>,
}

/// Parsed server side events stream until a [StopReason::StopSequence] is received from server.
pub type CompleteResponseStream = Pin<Box<dyn Stream<Item = Result<CompleteResponse, AnthropicError>> + Send>>;
#[derive(Clone, Serialize, Default, Debug, Builder, PartialEq)]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "AnthropicError"))]
pub struct MessagesRequest {
    /// The model to use.
    #[builder(default = "DEFAULT_MODEL")]
    pub model: Model,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// Amount of randomness injected into the response.
    ///
    /// Defaults to `1.0`. Ranges from `0.0` to `1.0`. Use `temperature` closer to `0.0`
    /// for analytical / multiple choice, and closer to `1.0` for creative and
    /// generative tasks.
    ///
    /// Note that even with `temperature` of `0.0`, the results will not be fully
    /// deterministic.
    ///    

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    ///   Only sample from the top K options for each subsequent token.
    ///   
    ///    Used to remove "long tail" low probability responses.
    ///    [Learn more technical details here](https://towardsdatascience.com/how-to-sample-from-language-models-682bceb97277).
    ///   
    ///    Recommended for advanced use cases only. You usually only need to use
    ///    `temperature`.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<usize>,

    /// The messages to send.
    pub messages: Vec<Message>,
    /// The number of tokens to sample.
    pub max_tokens: u32,

    /// The stop sequences to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub enum Model {
    #[serde(rename = "claude-3-opus-20240229")]
    Claude3Opus20240229,
    #[serde(rename = "claude-3-sonnet-20240229")]
    Claude3Sonnet20240229,
    #[default]
    #[serde(rename = "claude-3-haiku-20240307")]
    Claude3Haiku20240307,
    #[serde(rename = "claude-2.1")]
    Claude21,
    #[serde(rename = "claude-2.0")]
    Claude20,
    #[serde(rename = "claude-instant-1.2")]
    ClaudeInstant12,
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    MaxTokens,

    StopSequence,

    EndTurn,

    ToolUse,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct MessagesResponse {
    pub id: String,
    pub content: Vec<MessageResponse>,
    pub model: Model,
    pub stop_reason: Option<StopReason>,
    pub stop_sequence: Option<String>,
    pub usage: Usage,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Assistant,
}
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Content {
    Text(String),
    // Image{

    // }
    // Audio,
    // Video,
    // File,
}
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]

pub struct Message {
    pub role: Role,
    pub content: Content,
}
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct MessageResponse {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub text: String,
}
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}
