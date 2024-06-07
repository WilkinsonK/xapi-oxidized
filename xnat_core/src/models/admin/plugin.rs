use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use oxinat_derive::ModelField;

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "beanName")]
pub struct BeanName(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "dataModelBeans")]
pub struct DataModelBeans(Vec<String>);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "entityPackages")]
pub struct EntityPackages(Vec<String>);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename = "extendedAttributes")]
pub struct ExtendedAttributes {
    empty: Option<bool>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "id")]
pub struct ID(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "log4jPropertiesFile")]
pub struct Log4JPropertiesFile(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "name")]
pub struct Name(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "pluginClass")]
pub struct PluginClass(String);

#[derive(Clone, Debug, Serialize, ModelField)]
#[serde(rename = "version")]
pub struct Version(String);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plugin {
    #[serde(rename = "beanName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bean_name: Option<BeanName>,
    #[serde(rename = "dataModelBeans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_model_beans: Option<DataModelBeans>,
    #[serde(rename = "entityPackages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_packages: Option<EntityPackages>,
    #[serde(rename = "extendedAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_attributes: Option<ExtendedAttributes>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(rename = "log4jPropertiesFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log4j_properties_file: Option<Log4JPropertiesFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(rename = "pluginClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_class: Option<PluginClass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plugins {
    #[serde(flatten)]
    pub plugins: HashMap<String, Plugin>
}
