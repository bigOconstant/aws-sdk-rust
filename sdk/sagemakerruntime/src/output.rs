// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvokeEndpointAsyncOutput {
    /// <p>Identifier for an inference request. This will be the same as the <code>InferenceId</code> specified in the input. Amazon SageMaker will generate an identifier for you if you do not specify one.</p>
    pub inference_id: std::option::Option<std::string::String>,
    /// <p>The Amazon S3 URI where the inference response payload is stored.</p>
    pub output_location: std::option::Option<std::string::String>,
}
impl InvokeEndpointAsyncOutput {
    /// <p>Identifier for an inference request. This will be the same as the <code>InferenceId</code> specified in the input. Amazon SageMaker will generate an identifier for you if you do not specify one.</p>
    pub fn inference_id(&self) -> std::option::Option<&str> {
        self.inference_id.as_deref()
    }
    /// <p>The Amazon S3 URI where the inference response payload is stored.</p>
    pub fn output_location(&self) -> std::option::Option<&str> {
        self.output_location.as_deref()
    }
}
impl std::fmt::Debug for InvokeEndpointAsyncOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvokeEndpointAsyncOutput");
        formatter.field("inference_id", &self.inference_id);
        formatter.field("output_location", &self.output_location);
        formatter.finish()
    }
}
/// See [`InvokeEndpointAsyncOutput`](crate::output::InvokeEndpointAsyncOutput)
pub mod invoke_endpoint_async_output {
    /// A builder for [`InvokeEndpointAsyncOutput`](crate::output::InvokeEndpointAsyncOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) inference_id: std::option::Option<std::string::String>,
        pub(crate) output_location: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>Identifier for an inference request. This will be the same as the <code>InferenceId</code> specified in the input. Amazon SageMaker will generate an identifier for you if you do not specify one.</p>
        pub fn inference_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inference_id = Some(input.into());
            self
        }
        /// <p>Identifier for an inference request. This will be the same as the <code>InferenceId</code> specified in the input. Amazon SageMaker will generate an identifier for you if you do not specify one.</p>
        pub fn set_inference_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inference_id = input;
            self
        }
        /// <p>The Amazon S3 URI where the inference response payload is stored.</p>
        pub fn output_location(mut self, input: impl Into<std::string::String>) -> Self {
            self.output_location = Some(input.into());
            self
        }
        /// <p>The Amazon S3 URI where the inference response payload is stored.</p>
        pub fn set_output_location(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.output_location = input;
            self
        }
        /// Consumes the builder and constructs a [`InvokeEndpointAsyncOutput`](crate::output::InvokeEndpointAsyncOutput)
        pub fn build(self) -> crate::output::InvokeEndpointAsyncOutput {
            crate::output::InvokeEndpointAsyncOutput {
                inference_id: self.inference_id,
                output_location: self.output_location,
            }
        }
    }
}
impl InvokeEndpointAsyncOutput {
    /// Creates a new builder-style object to manufacture [`InvokeEndpointAsyncOutput`](crate::output::InvokeEndpointAsyncOutput)
    pub fn builder() -> crate::output::invoke_endpoint_async_output::Builder {
        crate::output::invoke_endpoint_async_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvokeEndpointOutput {
    /// <p>Includes the inference provided by the model.</p>
    /// <p>For information about the format of the response body, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/cdf-inference.html">Common Data Formats-Inference</a>.</p>
    pub body: std::option::Option<aws_smithy_types::Blob>,
    /// <p>The MIME type of the inference returned in the response body.</p>
    pub content_type: std::option::Option<std::string::String>,
    /// <p>Identifies the production variant that was invoked.</p>
    pub invoked_production_variant: std::option::Option<std::string::String>,
    /// <p>Provides additional information in the response about the inference returned by a model hosted at an Amazon SageMaker endpoint. The information is an opaque value that is forwarded verbatim. You could use this value, for example, to return an ID received in the <code>CustomAttributes</code> header of a request or other metadata that a service endpoint was programmed to produce. The value must consist of no more than 1024 visible US-ASCII characters as specified in <a href="https://tools.ietf.org/html/rfc7230#section-3.2.6">Section 3.3.6. Field Value Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). If the customer wants the custom attribute returned, the model must set the custom attribute to be included on the way back. </p>
    /// <p>The code in your model is responsible for setting or updating any custom attributes in the response. If your code does not set this value in the response, an empty value is returned. For example, if a custom attribute represents the trace ID, your model can prepend the custom attribute with <code>Trace ID:</code> in your post-processing function.</p>
    /// <p>This feature is currently supported in the Amazon Web Services SDKs but not in the Amazon SageMaker Python SDK.</p>
    pub custom_attributes: std::option::Option<std::string::String>,
}
impl InvokeEndpointOutput {
    /// <p>Includes the inference provided by the model.</p>
    /// <p>For information about the format of the response body, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/cdf-inference.html">Common Data Formats-Inference</a>.</p>
    pub fn body(&self) -> std::option::Option<&aws_smithy_types::Blob> {
        self.body.as_ref()
    }
    /// <p>The MIME type of the inference returned in the response body.</p>
    pub fn content_type(&self) -> std::option::Option<&str> {
        self.content_type.as_deref()
    }
    /// <p>Identifies the production variant that was invoked.</p>
    pub fn invoked_production_variant(&self) -> std::option::Option<&str> {
        self.invoked_production_variant.as_deref()
    }
    /// <p>Provides additional information in the response about the inference returned by a model hosted at an Amazon SageMaker endpoint. The information is an opaque value that is forwarded verbatim. You could use this value, for example, to return an ID received in the <code>CustomAttributes</code> header of a request or other metadata that a service endpoint was programmed to produce. The value must consist of no more than 1024 visible US-ASCII characters as specified in <a href="https://tools.ietf.org/html/rfc7230#section-3.2.6">Section 3.3.6. Field Value Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). If the customer wants the custom attribute returned, the model must set the custom attribute to be included on the way back. </p>
    /// <p>The code in your model is responsible for setting or updating any custom attributes in the response. If your code does not set this value in the response, an empty value is returned. For example, if a custom attribute represents the trace ID, your model can prepend the custom attribute with <code>Trace ID:</code> in your post-processing function.</p>
    /// <p>This feature is currently supported in the Amazon Web Services SDKs but not in the Amazon SageMaker Python SDK.</p>
    pub fn custom_attributes(&self) -> std::option::Option<&str> {
        self.custom_attributes.as_deref()
    }
}
impl std::fmt::Debug for InvokeEndpointOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvokeEndpointOutput");
        formatter.field("body", &"*** Sensitive Data Redacted ***");
        formatter.field("content_type", &self.content_type);
        formatter.field(
            "invoked_production_variant",
            &self.invoked_production_variant,
        );
        formatter.field("custom_attributes", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
/// See [`InvokeEndpointOutput`](crate::output::InvokeEndpointOutput)
pub mod invoke_endpoint_output {
    /// A builder for [`InvokeEndpointOutput`](crate::output::InvokeEndpointOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) body: std::option::Option<aws_smithy_types::Blob>,
        pub(crate) content_type: std::option::Option<std::string::String>,
        pub(crate) invoked_production_variant: std::option::Option<std::string::String>,
        pub(crate) custom_attributes: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>Includes the inference provided by the model.</p>
        /// <p>For information about the format of the response body, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/cdf-inference.html">Common Data Formats-Inference</a>.</p>
        pub fn body(mut self, input: aws_smithy_types::Blob) -> Self {
            self.body = Some(input);
            self
        }
        /// <p>Includes the inference provided by the model.</p>
        /// <p>For information about the format of the response body, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/cdf-inference.html">Common Data Formats-Inference</a>.</p>
        pub fn set_body(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
            self.body = input;
            self
        }
        /// <p>The MIME type of the inference returned in the response body.</p>
        pub fn content_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.content_type = Some(input.into());
            self
        }
        /// <p>The MIME type of the inference returned in the response body.</p>
        pub fn set_content_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.content_type = input;
            self
        }
        /// <p>Identifies the production variant that was invoked.</p>
        pub fn invoked_production_variant(mut self, input: impl Into<std::string::String>) -> Self {
            self.invoked_production_variant = Some(input.into());
            self
        }
        /// <p>Identifies the production variant that was invoked.</p>
        pub fn set_invoked_production_variant(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.invoked_production_variant = input;
            self
        }
        /// <p>Provides additional information in the response about the inference returned by a model hosted at an Amazon SageMaker endpoint. The information is an opaque value that is forwarded verbatim. You could use this value, for example, to return an ID received in the <code>CustomAttributes</code> header of a request or other metadata that a service endpoint was programmed to produce. The value must consist of no more than 1024 visible US-ASCII characters as specified in <a href="https://tools.ietf.org/html/rfc7230#section-3.2.6">Section 3.3.6. Field Value Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). If the customer wants the custom attribute returned, the model must set the custom attribute to be included on the way back. </p>
        /// <p>The code in your model is responsible for setting or updating any custom attributes in the response. If your code does not set this value in the response, an empty value is returned. For example, if a custom attribute represents the trace ID, your model can prepend the custom attribute with <code>Trace ID:</code> in your post-processing function.</p>
        /// <p>This feature is currently supported in the Amazon Web Services SDKs but not in the Amazon SageMaker Python SDK.</p>
        pub fn custom_attributes(mut self, input: impl Into<std::string::String>) -> Self {
            self.custom_attributes = Some(input.into());
            self
        }
        /// <p>Provides additional information in the response about the inference returned by a model hosted at an Amazon SageMaker endpoint. The information is an opaque value that is forwarded verbatim. You could use this value, for example, to return an ID received in the <code>CustomAttributes</code> header of a request or other metadata that a service endpoint was programmed to produce. The value must consist of no more than 1024 visible US-ASCII characters as specified in <a href="https://tools.ietf.org/html/rfc7230#section-3.2.6">Section 3.3.6. Field Value Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). If the customer wants the custom attribute returned, the model must set the custom attribute to be included on the way back. </p>
        /// <p>The code in your model is responsible for setting or updating any custom attributes in the response. If your code does not set this value in the response, an empty value is returned. For example, if a custom attribute represents the trace ID, your model can prepend the custom attribute with <code>Trace ID:</code> in your post-processing function.</p>
        /// <p>This feature is currently supported in the Amazon Web Services SDKs but not in the Amazon SageMaker Python SDK.</p>
        pub fn set_custom_attributes(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.custom_attributes = input;
            self
        }
        /// Consumes the builder and constructs a [`InvokeEndpointOutput`](crate::output::InvokeEndpointOutput)
        pub fn build(self) -> crate::output::InvokeEndpointOutput {
            crate::output::InvokeEndpointOutput {
                body: self.body,
                content_type: self.content_type,
                invoked_production_variant: self.invoked_production_variant,
                custom_attributes: self.custom_attributes,
            }
        }
    }
}
impl InvokeEndpointOutput {
    /// Creates a new builder-style object to manufacture [`InvokeEndpointOutput`](crate::output::InvokeEndpointOutput)
    pub fn builder() -> crate::output::invoke_endpoint_output::Builder {
        crate::output::invoke_endpoint_output::Builder::default()
    }
}
