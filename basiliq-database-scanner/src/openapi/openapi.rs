use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use getset::Getters;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPILicense {
    name: String,
    url: String,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPIContact {
    name: String,
    url: String,
    email: String,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPIInfo {
    title: String,
    description: String,
    terms_of_service: String,
    contact: OpenAPIContact,
    license: OpenAPILicense,
    version: String,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPIServerVariable {
    #[serde(rename = "enum")]
    list: Vec<String>,
    default: String,
    description: String,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPIServer {
    url: String,
    description: String,
	#[serde(skip_serializing_if = "HashMap::is_empty")]
    variables: HashMap<String, OpenAPIServerVariable>,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPIComponents {
    schemas: Value,
    responses: Value,
	#[serde(skip_serializing_if = "HashMap::is_empty")]
    parameters: HashMap<String, OpenAPIParameter>,
	#[serde(skip_serializing_if = "HashMap::is_empty")]
    examples: HashMap<String, OpenAPIExample>,
	#[serde(skip_serializing_if = "HashMap::is_empty")]
    request_bodies: HashMap<String, OpenAPIRequestBody>,
    headers: Value,
	#[serde(skip_serializing_if = "HashMap::is_empty")]
    security_schemes: HashMap<String, OpenAPISecurity>,
    links: Value,
    callbacks: Value,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPIPathObject {
    #[serde(rename = "$ref")]
    reference: String,
    summary: String,
    description: String,
	#[serde(skip_serializing_if = "Option::is_none")]
    get: Option<OpenAPIOperation>,
	#[serde(skip_serializing_if = "Option::is_none")]
    put: Option<OpenAPIOperation>,
	#[serde(skip_serializing_if = "Option::is_none")]
    post: Option<OpenAPIOperation>,
	#[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<OpenAPIOperation>,
	#[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OpenAPIOperation>,
	#[serde(skip_serializing_if = "Option::is_none")]
    head: Option<OpenAPIOperation>,
	#[serde(skip_serializing_if = "Option::is_none")]
    patch: Option<OpenAPIOperation>,
	#[serde(skip_serializing_if = "Option::is_none")]
    trace: Option<OpenAPIOperation>,
    servers: Vec<OpenAPIServer>,
    parameters: Value,
}

impl OpenAPIPathObject {
    pub fn operation(&self, method: &str) -> &Option<OpenAPIOperation> {
        match method {
            "get" => &self.get,
            "put" => &self.put,
            "post" => &self.post,
            "delete" => &self.delete,
            "options" => &self.options,
            "head" => &self.head,
            "patch" => &self.patch,
            "trace" => &self.trace,
            _ => &None,
        }
    }
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPIExternalDocs {
	#[serde(skip_serializing_if = "String::is_empty")]
    description: String,
	#[serde(skip_serializing_if = "String::is_empty")]
    url: String,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPIParameter {
	#[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(rename = "in")]
	#[serde(skip_serializing_if = "String::is_empty")]
    location: String,
	#[serde(skip_serializing_if = "String::is_empty")]
    description: String,
    required: bool,
    deprecated: bool,
    allow_empty_value: bool,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPIMediaType {
    schema: Value,
    example: Value,
    examples: Value,
    encoding: Value,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPIRequestBody {
	#[serde(skip_serializing_if = "String::is_empty")]
    description: String,
	#[serde(skip_serializing_if = "HashMap::is_empty")]
    content: HashMap<String, OpenAPIMediaType>,
    required: bool,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPIExample {
	#[serde(skip_serializing_if = "String::is_empty")]
    summary: String,
	#[serde(skip_serializing_if = "String::is_empty")]
    description: String,
    value: Value,
	#[serde(skip_serializing_if = "String::is_empty")]
    external_value: String,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPITag {
	#[serde(skip_serializing_if = "String::is_empty")]
    name: String,
	#[serde(skip_serializing_if = "String::is_empty")]
    description: String,
    external_docs: OpenAPIExternalDocs,
}

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPIOAuthFlowAuthorizationCode {
	#[serde(skip_serializing_if = "String::is_empty")]
    authorization_url: String,
	#[serde(skip_serializing_if = "String::is_empty")]
    token_url: String,
	#[serde(skip_serializing_if = "String::is_empty")]
    refresh_url: String,
	#[serde(skip_serializing_if = "HashMap::is_empty")]
    scopes_url: HashMap<String, String>,
}

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPIOAuthFlowImplicit {
	#[serde(skip_serializing_if = "String::is_empty")]
    authorization_url: String,
	#[serde(skip_serializing_if = "String::is_empty")]
    refresh_url: String,
	#[serde(skip_serializing_if = "HashMap::is_empty")]
    scopes_url: HashMap<String, String>,
}

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPIOAuthFlowClientCrendentials {
	#[serde(skip_serializing_if = "String::is_empty")]
    token_url: String,
	#[serde(skip_serializing_if = "String::is_empty")]
    refresh_url: String,
	#[serde(skip_serializing_if = "HashMap::is_empty")]
    scopes_url: HashMap<String, String>,
}

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPIOAuthFlowPassword {
	#[serde(skip_serializing_if = "String::is_empty")]
    token_url: String,
	#[serde(skip_serializing_if = "String::is_empty")]
    refresh_url: String,
	#[serde(skip_serializing_if = "HashMap::is_empty")]
    scopes_url: HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPIOAuthFlows {
    implicit: OpenAPIOAuthFlowImplicit,
    password: OpenAPIOAuthFlowPassword,
    client_credentials: OpenAPIOAuthFlowClientCrendentials,
    authorization_code: OpenAPIOAuthFlowAuthorizationCode,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPISecurity {
    #[serde(rename = "type")]
	#[serde(skip_serializing_if = "String::is_empty")]
    security_type: String,
	#[serde(skip_serializing_if = "String::is_empty")]
    description: String,
	#[serde(skip_serializing_if = "String::is_empty")]
    name: String,
	#[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "in")]
    location: String,
	#[serde(skip_serializing_if = "String::is_empty")]
    scheme: String,
	#[serde(skip_serializing_if = "String::is_empty")]
    bearer_format: String,
	#[serde(skip_serializing_if = "Option::is_none")]
    flows: Option<OpenAPIOAuthFlows>,
	#[serde(skip_serializing_if = "String::is_empty")]
    open_id_connect_url: String,
}

pub type OpenAPISecurityScope = Option<Vec<HashMap<String, Vec<String>>>>;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPIOperation {
    tags: Vec<String>,
	#[serde(skip_serializing_if = "String::is_empty")]
    summary: String,
	#[serde(skip_serializing_if = "String::is_empty")]
    description: String,
    external_docs: OpenAPIExternalDocs,
	#[serde(skip_serializing_if = "String::is_empty")]
    operation_id: String,
    parameters: Vec<OpenAPIParameter>,
    request_body: OpenAPIRequestBody,
    responses: Value,
    callbacks: Value,
    deprecated: bool,
    security: OpenAPISecurityScope,
    servers: Vec<OpenAPIServer>,
}

impl OpenAPIOperation {
    pub fn securities(&self) -> &OpenAPISecurityScope {
        &self.security
    }
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OpenAPIDocument {
	#[serde(skip_serializing_if = "String::is_empty")]
    openapi: String,
    info: OpenAPIInfo,
	servers: Vec<OpenAPIServer>,
	#[serde(skip_serializing_if = "HashMap::is_empty")]
    paths: HashMap<String, OpenAPIPathObject>,
	components: OpenAPIComponents,
	#[serde(skip_serializing_if = "HashMap::is_empty")]
    security: HashMap<String, OpenAPISecurity>,
    tags: Vec<OpenAPITag>,
    external_docs: OpenAPIExternalDocs,
}
