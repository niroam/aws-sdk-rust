// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Information about a single retained message.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RetainedMessageSummary {
    /// <p>The topic name to which the retained message was published.</p>
    pub topic: std::option::Option<std::string::String>,
    /// <p>The size of the retained message's payload in bytes.</p>
    pub payload_size: i64,
    /// <p>The quality of service (QoS) level used to publish the retained message.</p>
    pub qos: i32,
    /// <p>The Epoch date and time, in milliseconds, when the retained message was stored by IoT.</p>
    pub last_modified_time: i64,
}
impl std::fmt::Debug for RetainedMessageSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RetainedMessageSummary");
        formatter.field("topic", &self.topic);
        formatter.field("payload_size", &self.payload_size);
        formatter.field("qos", &self.qos);
        formatter.field("last_modified_time", &self.last_modified_time);
        formatter.finish()
    }
}
/// See [`RetainedMessageSummary`](crate::model::RetainedMessageSummary)
pub mod retained_message_summary {
    /// A builder for [`RetainedMessageSummary`](crate::model::RetainedMessageSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) topic: std::option::Option<std::string::String>,
        pub(crate) payload_size: std::option::Option<i64>,
        pub(crate) qos: std::option::Option<i32>,
        pub(crate) last_modified_time: std::option::Option<i64>,
    }
    impl Builder {
        /// <p>The topic name to which the retained message was published.</p>
        pub fn topic(mut self, input: impl Into<std::string::String>) -> Self {
            self.topic = Some(input.into());
            self
        }
        /// <p>The topic name to which the retained message was published.</p>
        pub fn set_topic(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.topic = input;
            self
        }
        /// <p>The size of the retained message's payload in bytes.</p>
        pub fn payload_size(mut self, input: i64) -> Self {
            self.payload_size = Some(input);
            self
        }
        /// <p>The size of the retained message's payload in bytes.</p>
        pub fn set_payload_size(mut self, input: std::option::Option<i64>) -> Self {
            self.payload_size = input;
            self
        }
        /// <p>The quality of service (QoS) level used to publish the retained message.</p>
        pub fn qos(mut self, input: i32) -> Self {
            self.qos = Some(input);
            self
        }
        /// <p>The quality of service (QoS) level used to publish the retained message.</p>
        pub fn set_qos(mut self, input: std::option::Option<i32>) -> Self {
            self.qos = input;
            self
        }
        /// <p>The Epoch date and time, in milliseconds, when the retained message was stored by IoT.</p>
        pub fn last_modified_time(mut self, input: i64) -> Self {
            self.last_modified_time = Some(input);
            self
        }
        /// <p>The Epoch date and time, in milliseconds, when the retained message was stored by IoT.</p>
        pub fn set_last_modified_time(mut self, input: std::option::Option<i64>) -> Self {
            self.last_modified_time = input;
            self
        }
        /// Consumes the builder and constructs a [`RetainedMessageSummary`](crate::model::RetainedMessageSummary)
        pub fn build(self) -> crate::model::RetainedMessageSummary {
            crate::model::RetainedMessageSummary {
                topic: self.topic,
                payload_size: self.payload_size.unwrap_or_default(),
                qos: self.qos.unwrap_or_default(),
                last_modified_time: self.last_modified_time.unwrap_or_default(),
            }
        }
    }
}
impl RetainedMessageSummary {
    /// Creates a new builder-style object to manufacture [`RetainedMessageSummary`](crate::model::RetainedMessageSummary)
    pub fn builder() -> crate::model::retained_message_summary::Builder {
        crate::model::retained_message_summary::Builder::default()
    }
}