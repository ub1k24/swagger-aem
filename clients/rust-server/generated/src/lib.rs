#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


extern crate futures;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

// Logically this should be in the client and server modules, but rust doesn't allow `macro_use` from a module.
#[cfg(any(feature = "client", feature = "server"))]
#[macro_use]
extern crate hyper;

extern crate swagger;

#[macro_use]
extern crate url;

use futures::Stream;
use std::io::Error;

#[allow(unused_imports)]
use std::collections::HashMap;

pub use futures::Future;

#[cfg(any(feature = "client", feature = "server"))]
mod mimetypes;

pub use swagger::{ApiError, ContextWrapper};

pub const BASE_PATH: &'static str = "/";
pub const API_VERSION: &'static str = "3.2.0-pre.0";


#[derive(Debug, PartialEq)]
pub enum GetAemProductInfoResponse {
    /// Default response
    DefaultResponse ( Vec<String> ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetConfigMgrResponse {
    /// OK
    OK ( String ) ,
    /// Unexpected error.
    UnexpectedError ,
}

#[derive(Debug, PartialEq)]
pub enum PostBundleResponse {
    /// Default response
    DefaultResponse ,
}

#[derive(Debug, PartialEq)]
pub enum PostJmxRepositoryResponse {
    /// Default response
    DefaultResponse ,
}

#[derive(Debug, PartialEq)]
pub enum PostSamlConfigurationResponse {
    /// Retrieved AEM SAML Configuration
    RetrievedAEMSAMLConfiguration ( models::SamlConfigurationInfo ) ,
    /// Default response
    DefaultResponse ( String ) ,
    /// Default response
    DefaultResponse ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetLoginPageResponse {
    /// Default response
    DefaultResponse ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum PostCqActionsResponse {
    /// Default response
    DefaultResponse ,
}

#[derive(Debug, PartialEq)]
pub enum GetCrxdeStatusResponse {
    /// CRXDE is enabled
    CRXDEIsEnabled ( String ) ,
    /// CRXDE is disabled
    CRXDEIsDisabled ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetInstallStatusResponse {
    /// Retrieved CRX package manager install status
    RetrievedCRXPackageManagerInstallStatus ( models::InstallStatus ) ,
    /// Default response
    DefaultResponse ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetPackageManagerServletResponse {
    /// Package Manager Servlet is disabled
    PackageManagerServletIsDisabled ( String ) ,
    /// Package Manager Servlet is active
    PackageManagerServletIsActive ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum PostPackageServiceResponse {
    /// Default response
    DefaultResponse ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum PostPackageServiceJsonResponse {
    /// Default response
    DefaultResponse ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum PostPackageUpdateResponse {
    /// Default response
    DefaultResponse ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum PostSetPasswordResponse {
    /// Default response
    DefaultResponse ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetAemHealthCheckResponse {
    /// Default response
    DefaultResponse ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum PostConfigAemHealthCheckServletResponse {
    /// Default response
    DefaultResponse ,
}

#[derive(Debug, PartialEq)]
pub enum PostConfigAemPasswordResetResponse {
    /// Default response
    DefaultResponse ,
}

#[derive(Debug, PartialEq)]
pub enum DeleteAgentResponse {
    /// Default response
    DefaultResponse ,
}

#[derive(Debug, PartialEq)]
pub enum DeleteNodeResponse {
    /// Default response
    DefaultResponse ,
}

#[derive(Debug, PartialEq)]
pub enum GetAgentResponse {
    /// Default response
    DefaultResponse ,
}

#[derive(Debug, PartialEq)]
pub enum GetAgentsResponse {
    /// Default response
    DefaultResponse ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetAuthorizableKeystoreResponse {
    /// Retrieved Authorizable Keystore info
    RetrievedAuthorizableKeystoreInfo ( models::KeystoreInfo ) ,
    /// Default response
    DefaultResponse ( String ) ,
}

pub enum GetKeystoreResponse {
    /// Default response
    DefaultResponse ( swagger::ByteArray ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetNodeResponse {
    /// Default response
    DefaultResponse ,
}

pub enum GetPackageResponse {
    /// Default response
    DefaultResponse ( swagger::ByteArray ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetPackageFilterResponse {
    /// Default response
    DefaultResponse ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetQueryResponse {
    /// Default response
    DefaultResponse ( String ) ,
}

pub enum GetTruststoreResponse {
    /// Default response
    DefaultResponse ( swagger::ByteArray ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetTruststoreInfoResponse {
    /// Retrieved AEM Truststore info
    RetrievedAEMTruststoreInfo ( models::TruststoreInfo ) ,
    /// Default response
    DefaultResponse ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum PostAgentResponse {
    /// Default response
    DefaultResponse ,
}

#[derive(Debug, PartialEq)]
pub enum PostAuthorizableKeystoreResponse {
    /// Retrieved Authorizable Keystore info
    RetrievedAuthorizableKeystoreInfo ( models::KeystoreInfo ) ,
    /// Default response
    DefaultResponse ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum PostAuthorizablesResponse {
    /// Default response
    DefaultResponse ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum PostConfigAdobeGraniteSamlAuthenticationHandlerResponse {
    /// Default response
    DefaultResponse ,
}

#[derive(Debug, PartialEq)]
pub enum PostConfigApacheFelixJettyBasedHttpServiceResponse {
    /// Default response
    DefaultResponse ,
}

#[derive(Debug, PartialEq)]
pub enum PostConfigApacheHttpComponentsProxyConfigurationResponse {
    /// Default response
    DefaultResponse ,
}

#[derive(Debug, PartialEq)]
pub enum PostConfigApacheSlingDavExServletResponse {
    /// Default response
    DefaultResponse ,
}

#[derive(Debug, PartialEq)]
pub enum PostConfigApacheSlingGetServletResponse {
    /// Default response
    DefaultResponse ,
}

#[derive(Debug, PartialEq)]
pub enum PostConfigApacheSlingReferrerFilterResponse {
    /// Default response
    DefaultResponse ,
}

#[derive(Debug, PartialEq)]
pub enum PostNodeResponse {
    /// Default response
    DefaultResponse ,
}

#[derive(Debug, PartialEq)]
pub enum PostNodeRwResponse {
    /// Default response
    DefaultResponse ,
}

#[derive(Debug, PartialEq)]
pub enum PostPathResponse {
    /// Default response
    DefaultResponse ,
}

#[derive(Debug, PartialEq)]
pub enum PostQueryResponse {
    /// Default response
    DefaultResponse ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum PostTreeActivationResponse {
    /// Default response
    DefaultResponse ,
}

#[derive(Debug, PartialEq)]
pub enum PostTruststoreResponse {
    /// Default response
    DefaultResponse ( String ) ,
}

#[derive(Debug, PartialEq)]
pub enum PostTruststorePKCS12Response {
    /// Default response
    DefaultResponse ( String ) ,
}


/// API
pub trait Api<C> {


    fn get_aem_product_info(&self, context: &C) -> Box<Future<Item=GetAemProductInfoResponse, Error=ApiError>>;


    fn get_config_mgr(&self, context: &C) -> Box<Future<Item=GetConfigMgrResponse, Error=ApiError>>;


    fn post_bundle(&self, name: String, action: String, context: &C) -> Box<Future<Item=PostBundleResponse, Error=ApiError>>;


    fn post_jmx_repository(&self, action: String, context: &C) -> Box<Future<Item=PostJmxRepositoryResponse, Error=ApiError>>;


    fn post_saml_configuration(&self, post: Option<bool>, apply: Option<bool>, delete: Option<bool>, action: Option<String>, location: Option<String>, path: Option<&Vec<String>>, service_ranking: Option<i32>, idp_url: Option<String>, idp_cert_alias: Option<String>, idp_http_redirect: Option<bool>, service_provider_entity_id: Option<String>, assertion_consumer_service_url: Option<String>, sp_private_key_alias: Option<String>, key_store_password: Option<String>, default_redirect_url: Option<String>, user_id_attribute: Option<String>, use_encryption: Option<bool>, create_user: Option<bool>, add_group_memberships: Option<bool>, group_membership_attribute: Option<String>, default_groups: Option<&Vec<String>>, name_id_format: Option<String>, synchronize_attributes: Option<&Vec<String>>, handle_logout: Option<bool>, logout_url: Option<String>, clock_tolerance: Option<i32>, digest_method: Option<String>, signature_method: Option<String>, user_intermediate_path: Option<String>, propertylist: Option<&Vec<String>>, context: &C) -> Box<Future<Item=PostSamlConfigurationResponse, Error=ApiError>>;


    fn get_login_page(&self, context: &C) -> Box<Future<Item=GetLoginPageResponse, Error=ApiError>>;


    fn post_cq_actions(&self, authorizable_id: String, changelog: String, context: &C) -> Box<Future<Item=PostCqActionsResponse, Error=ApiError>>;


    fn get_crxde_status(&self, context: &C) -> Box<Future<Item=GetCrxdeStatusResponse, Error=ApiError>>;


    fn get_install_status(&self, context: &C) -> Box<Future<Item=GetInstallStatusResponse, Error=ApiError>>;


    fn get_package_manager_servlet(&self, context: &C) -> Box<Future<Item=GetPackageManagerServletResponse, Error=ApiError>>;


    fn post_package_service(&self, cmd: String, context: &C) -> Box<Future<Item=PostPackageServiceResponse, Error=ApiError>>;


    fn post_package_service_json(&self, path: String, cmd: String, group_name: Option<String>, package_name: Option<String>, package_version: Option<String>, _charset_: Option<String>, force: Option<bool>, recursive: Option<bool>, package: Option<swagger::ByteArray>, context: &C) -> Box<Future<Item=PostPackageServiceJsonResponse, Error=ApiError>>;


    fn post_package_update(&self, group_name: String, package_name: String, version: String, path: String, filter: Option<String>, _charset_: Option<String>, context: &C) -> Box<Future<Item=PostPackageUpdateResponse, Error=ApiError>>;


    fn post_set_password(&self, old: String, plain: String, verify: String, context: &C) -> Box<Future<Item=PostSetPasswordResponse, Error=ApiError>>;


    fn get_aem_health_check(&self, tags: Option<String>, combine_tags_or: Option<bool>, context: &C) -> Box<Future<Item=GetAemHealthCheckResponse, Error=ApiError>>;


    fn post_config_aem_health_check_servlet(&self, bundles_ignored: Option<&Vec<String>>, bundles_ignored_type_hint: Option<String>, context: &C) -> Box<Future<Item=PostConfigAemHealthCheckServletResponse, Error=ApiError>>;


    fn post_config_aem_password_reset(&self, pwdreset_authorizables: Option<&Vec<String>>, pwdreset_authorizables_type_hint: Option<String>, context: &C) -> Box<Future<Item=PostConfigAemPasswordResetResponse, Error=ApiError>>;


    fn delete_agent(&self, runmode: String, name: String, context: &C) -> Box<Future<Item=DeleteAgentResponse, Error=ApiError>>;


    fn delete_node(&self, path: String, name: String, context: &C) -> Box<Future<Item=DeleteNodeResponse, Error=ApiError>>;


    fn get_agent(&self, runmode: String, name: String, context: &C) -> Box<Future<Item=GetAgentResponse, Error=ApiError>>;


    fn get_agents(&self, runmode: String, context: &C) -> Box<Future<Item=GetAgentsResponse, Error=ApiError>>;


    fn get_authorizable_keystore(&self, intermediate_path: String, authorizable_id: String, context: &C) -> Box<Future<Item=GetAuthorizableKeystoreResponse, Error=ApiError>>;


    fn get_keystore(&self, intermediate_path: String, authorizable_id: String, context: &C) -> Box<Future<Item=GetKeystoreResponse, Error=ApiError>>;


    fn get_node(&self, path: String, name: String, context: &C) -> Box<Future<Item=GetNodeResponse, Error=ApiError>>;


    fn get_package(&self, group: String, name: String, version: String, context: &C) -> Box<Future<Item=GetPackageResponse, Error=ApiError>>;


    fn get_package_filter(&self, group: String, name: String, version: String, context: &C) -> Box<Future<Item=GetPackageFilterResponse, Error=ApiError>>;


    fn get_query(&self, path: String, p_limit: f64, _1_property: String, _1_property_value: String, context: &C) -> Box<Future<Item=GetQueryResponse, Error=ApiError>>;


    fn get_truststore(&self, context: &C) -> Box<Future<Item=GetTruststoreResponse, Error=ApiError>>;


    fn get_truststore_info(&self, context: &C) -> Box<Future<Item=GetTruststoreInfoResponse, Error=ApiError>>;


    fn post_agent(&self, runmode: String, name: String, jcrcontentcqdistribute: Option<bool>, jcrcontentcqdistribute_type_hint: Option<String>, jcrcontentcqname: Option<String>, jcrcontentcqtemplate: Option<String>, jcrcontentenabled: Option<bool>, jcrcontentjcrdescription: Option<String>, jcrcontentjcrlast_modified: Option<String>, jcrcontentjcrlast_modified_by: Option<String>, jcrcontentjcrmixin_types: Option<String>, jcrcontentjcrtitle: Option<String>, jcrcontentlog_level: Option<String>, jcrcontentno_status_update: Option<bool>, jcrcontentno_versioning: Option<bool>, jcrcontentprotocol_connect_timeout: Option<f64>, jcrcontentprotocol_http_connection_closed: Option<bool>, jcrcontentprotocol_http_expired: Option<String>, jcrcontentprotocol_http_headers: Option<&Vec<String>>, jcrcontentprotocol_http_headers_type_hint: Option<String>, jcrcontentprotocol_http_method: Option<String>, jcrcontentprotocol_https_relaxed: Option<bool>, jcrcontentprotocol_interface: Option<String>, jcrcontentprotocol_socket_timeout: Option<f64>, jcrcontentprotocol_version: Option<String>, jcrcontentproxy_ntlm_domain: Option<String>, jcrcontentproxy_ntlm_host: Option<String>, jcrcontentproxy_host: Option<String>, jcrcontentproxy_password: Option<String>, jcrcontentproxy_port: Option<f64>, jcrcontentproxy_user: Option<String>, jcrcontentqueue_batch_max_size: Option<f64>, jcrcontentqueue_batch_mode: Option<String>, jcrcontentqueue_batch_wait_time: Option<f64>, jcrcontentretry_delay: Option<String>, jcrcontentreverse_replication: Option<bool>, jcrcontentserialization_type: Option<String>, jcrcontentslingresource_type: Option<String>, jcrcontentssl: Option<String>, jcrcontenttransport_ntlm_domain: Option<String>, jcrcontenttransport_ntlm_host: Option<String>, jcrcontenttransport_password: Option<String>, jcrcontenttransport_uri: Option<String>, jcrcontenttransport_user: Option<String>, jcrcontenttrigger_distribute: Option<bool>, jcrcontenttrigger_modified: Option<bool>, jcrcontenttrigger_on_off_time: Option<bool>, jcrcontenttrigger_receive: Option<bool>, jcrcontenttrigger_specific: Option<bool>, jcrcontentuser_id: Option<String>, jcrprimary_type: Option<String>, operation: Option<String>, context: &C) -> Box<Future<Item=PostAgentResponse, Error=ApiError>>;


    fn post_authorizable_keystore(&self, intermediate_path: String, authorizable_id: String, operation: Option<String>, current_password: Option<String>, new_password: Option<String>, re_password: Option<String>, key_password: Option<String>, key_store_pass: Option<String>, alias: Option<String>, new_alias: Option<String>, remove_alias: Option<String>, cert_chain: Option<swagger::ByteArray>, pk: Option<swagger::ByteArray>, key_store: Option<swagger::ByteArray>, context: &C) -> Box<Future<Item=PostAuthorizableKeystoreResponse, Error=ApiError>>;


    fn post_authorizables(&self, authorizable_id: String, intermediate_path: String, create_user: Option<String>, create_group: Option<String>, reppassword: Option<String>, profilegiven_name: Option<String>, context: &C) -> Box<Future<Item=PostAuthorizablesResponse, Error=ApiError>>;


    fn post_config_adobe_granite_saml_authentication_handler(&self, key_store_password: Option<String>, key_store_password_type_hint: Option<String>, service_ranking: Option<i32>, service_ranking_type_hint: Option<String>, idp_http_redirect: Option<bool>, idp_http_redirect_type_hint: Option<String>, create_user: Option<bool>, create_user_type_hint: Option<String>, default_redirect_url: Option<String>, default_redirect_url_type_hint: Option<String>, user_id_attribute: Option<String>, user_id_attribute_type_hint: Option<String>, default_groups: Option<&Vec<String>>, default_groups_type_hint: Option<String>, idp_cert_alias: Option<String>, idp_cert_alias_type_hint: Option<String>, add_group_memberships: Option<bool>, add_group_memberships_type_hint: Option<String>, path: Option<&Vec<String>>, path_type_hint: Option<String>, synchronize_attributes: Option<&Vec<String>>, synchronize_attributes_type_hint: Option<String>, clock_tolerance: Option<i32>, clock_tolerance_type_hint: Option<String>, group_membership_attribute: Option<String>, group_membership_attribute_type_hint: Option<String>, idp_url: Option<String>, idp_url_type_hint: Option<String>, logout_url: Option<String>, logout_url_type_hint: Option<String>, service_provider_entity_id: Option<String>, service_provider_entity_id_type_hint: Option<String>, assertion_consumer_service_url: Option<String>, assertion_consumer_service_url_type_hint: Option<String>, handle_logout: Option<bool>, handle_logout_type_hint: Option<String>, sp_private_key_alias: Option<String>, sp_private_key_alias_type_hint: Option<String>, use_encryption: Option<bool>, use_encryption_type_hint: Option<String>, name_id_format: Option<String>, name_id_format_type_hint: Option<String>, digest_method: Option<String>, digest_method_type_hint: Option<String>, signature_method: Option<String>, signature_method_type_hint: Option<String>, user_intermediate_path: Option<String>, user_intermediate_path_type_hint: Option<String>, context: &C) -> Box<Future<Item=PostConfigAdobeGraniteSamlAuthenticationHandlerResponse, Error=ApiError>>;


    fn post_config_apache_felix_jetty_based_http_service(&self, org_apache_felix_https_nio: Option<bool>, org_apache_felix_https_nio_type_hint: Option<String>, org_apache_felix_https_keystore: Option<String>, org_apache_felix_https_keystore_type_hint: Option<String>, org_apache_felix_https_keystore_password: Option<String>, org_apache_felix_https_keystore_password_type_hint: Option<String>, org_apache_felix_https_keystore_key: Option<String>, org_apache_felix_https_keystore_key_type_hint: Option<String>, org_apache_felix_https_keystore_key_password: Option<String>, org_apache_felix_https_keystore_key_password_type_hint: Option<String>, org_apache_felix_https_truststore: Option<String>, org_apache_felix_https_truststore_type_hint: Option<String>, org_apache_felix_https_truststore_password: Option<String>, org_apache_felix_https_truststore_password_type_hint: Option<String>, org_apache_felix_https_clientcertificate: Option<String>, org_apache_felix_https_clientcertificate_type_hint: Option<String>, org_apache_felix_https_enable: Option<bool>, org_apache_felix_https_enable_type_hint: Option<String>, org_osgi_service_http_port_secure: Option<String>, org_osgi_service_http_port_secure_type_hint: Option<String>, context: &C) -> Box<Future<Item=PostConfigApacheFelixJettyBasedHttpServiceResponse, Error=ApiError>>;


    fn post_config_apache_http_components_proxy_configuration(&self, proxy_host: Option<String>, proxy_host_type_hint: Option<String>, proxy_port: Option<i32>, proxy_port_type_hint: Option<String>, proxy_exceptions: Option<&Vec<String>>, proxy_exceptions_type_hint: Option<String>, proxy_enabled: Option<bool>, proxy_enabled_type_hint: Option<String>, proxy_user: Option<String>, proxy_user_type_hint: Option<String>, proxy_password: Option<String>, proxy_password_type_hint: Option<String>, context: &C) -> Box<Future<Item=PostConfigApacheHttpComponentsProxyConfigurationResponse, Error=ApiError>>;


    fn post_config_apache_sling_dav_ex_servlet(&self, alias: Option<String>, alias_type_hint: Option<String>, dav_create_absolute_uri: Option<bool>, dav_create_absolute_uri_type_hint: Option<String>, context: &C) -> Box<Future<Item=PostConfigApacheSlingDavExServletResponse, Error=ApiError>>;


    fn post_config_apache_sling_get_servlet(&self, json_maximumresults: Option<String>, json_maximumresults_type_hint: Option<String>, enable_html: Option<bool>, enable_html_type_hint: Option<String>, enable_txt: Option<bool>, enable_txt_type_hint: Option<String>, enable_xml: Option<bool>, enable_xml_type_hint: Option<String>, context: &C) -> Box<Future<Item=PostConfigApacheSlingGetServletResponse, Error=ApiError>>;


    fn post_config_apache_sling_referrer_filter(&self, allow_empty: Option<bool>, allow_empty_type_hint: Option<String>, allow_hosts: Option<String>, allow_hosts_type_hint: Option<String>, allow_hosts_regexp: Option<String>, allow_hosts_regexp_type_hint: Option<String>, filter_methods: Option<String>, filter_methods_type_hint: Option<String>, context: &C) -> Box<Future<Item=PostConfigApacheSlingReferrerFilterResponse, Error=ApiError>>;


    fn post_node(&self, path: String, name: String, operation: Option<String>, delete_authorizable: Option<String>, file: Option<swagger::ByteArray>, context: &C) -> Box<Future<Item=PostNodeResponse, Error=ApiError>>;


    fn post_node_rw(&self, path: String, name: String, add_members: Option<String>, context: &C) -> Box<Future<Item=PostNodeRwResponse, Error=ApiError>>;


    fn post_path(&self, path: String, jcrprimary_type: String, name: String, context: &C) -> Box<Future<Item=PostPathResponse, Error=ApiError>>;


    fn post_query(&self, path: String, p_limit: f64, _1_property: String, _1_property_value: String, context: &C) -> Box<Future<Item=PostQueryResponse, Error=ApiError>>;


    fn post_tree_activation(&self, ignoredeactivated: bool, onlymodified: bool, path: String, context: &C) -> Box<Future<Item=PostTreeActivationResponse, Error=ApiError>>;


    fn post_truststore(&self, operation: Option<String>, new_password: Option<String>, re_password: Option<String>, key_store_type: Option<String>, remove_alias: Option<String>, certificate: Option<swagger::ByteArray>, context: &C) -> Box<Future<Item=PostTruststoreResponse, Error=ApiError>>;


    fn post_truststore_pkcs12(&self, truststore_p12: Option<swagger::ByteArray>, context: &C) -> Box<Future<Item=PostTruststorePKCS12Response, Error=ApiError>>;

}

/// API without a `Context`
pub trait ApiNoContext {


    fn get_aem_product_info(&self) -> Box<Future<Item=GetAemProductInfoResponse, Error=ApiError>>;


    fn get_config_mgr(&self) -> Box<Future<Item=GetConfigMgrResponse, Error=ApiError>>;


    fn post_bundle(&self, name: String, action: String) -> Box<Future<Item=PostBundleResponse, Error=ApiError>>;


    fn post_jmx_repository(&self, action: String) -> Box<Future<Item=PostJmxRepositoryResponse, Error=ApiError>>;


    fn post_saml_configuration(&self, post: Option<bool>, apply: Option<bool>, delete: Option<bool>, action: Option<String>, location: Option<String>, path: Option<&Vec<String>>, service_ranking: Option<i32>, idp_url: Option<String>, idp_cert_alias: Option<String>, idp_http_redirect: Option<bool>, service_provider_entity_id: Option<String>, assertion_consumer_service_url: Option<String>, sp_private_key_alias: Option<String>, key_store_password: Option<String>, default_redirect_url: Option<String>, user_id_attribute: Option<String>, use_encryption: Option<bool>, create_user: Option<bool>, add_group_memberships: Option<bool>, group_membership_attribute: Option<String>, default_groups: Option<&Vec<String>>, name_id_format: Option<String>, synchronize_attributes: Option<&Vec<String>>, handle_logout: Option<bool>, logout_url: Option<String>, clock_tolerance: Option<i32>, digest_method: Option<String>, signature_method: Option<String>, user_intermediate_path: Option<String>, propertylist: Option<&Vec<String>>) -> Box<Future<Item=PostSamlConfigurationResponse, Error=ApiError>>;


    fn get_login_page(&self) -> Box<Future<Item=GetLoginPageResponse, Error=ApiError>>;


    fn post_cq_actions(&self, authorizable_id: String, changelog: String) -> Box<Future<Item=PostCqActionsResponse, Error=ApiError>>;


    fn get_crxde_status(&self) -> Box<Future<Item=GetCrxdeStatusResponse, Error=ApiError>>;


    fn get_install_status(&self) -> Box<Future<Item=GetInstallStatusResponse, Error=ApiError>>;


    fn get_package_manager_servlet(&self) -> Box<Future<Item=GetPackageManagerServletResponse, Error=ApiError>>;


    fn post_package_service(&self, cmd: String) -> Box<Future<Item=PostPackageServiceResponse, Error=ApiError>>;


    fn post_package_service_json(&self, path: String, cmd: String, group_name: Option<String>, package_name: Option<String>, package_version: Option<String>, _charset_: Option<String>, force: Option<bool>, recursive: Option<bool>, package: Option<swagger::ByteArray>) -> Box<Future<Item=PostPackageServiceJsonResponse, Error=ApiError>>;


    fn post_package_update(&self, group_name: String, package_name: String, version: String, path: String, filter: Option<String>, _charset_: Option<String>) -> Box<Future<Item=PostPackageUpdateResponse, Error=ApiError>>;


    fn post_set_password(&self, old: String, plain: String, verify: String) -> Box<Future<Item=PostSetPasswordResponse, Error=ApiError>>;


    fn get_aem_health_check(&self, tags: Option<String>, combine_tags_or: Option<bool>) -> Box<Future<Item=GetAemHealthCheckResponse, Error=ApiError>>;


    fn post_config_aem_health_check_servlet(&self, bundles_ignored: Option<&Vec<String>>, bundles_ignored_type_hint: Option<String>) -> Box<Future<Item=PostConfigAemHealthCheckServletResponse, Error=ApiError>>;


    fn post_config_aem_password_reset(&self, pwdreset_authorizables: Option<&Vec<String>>, pwdreset_authorizables_type_hint: Option<String>) -> Box<Future<Item=PostConfigAemPasswordResetResponse, Error=ApiError>>;


    fn delete_agent(&self, runmode: String, name: String) -> Box<Future<Item=DeleteAgentResponse, Error=ApiError>>;


    fn delete_node(&self, path: String, name: String) -> Box<Future<Item=DeleteNodeResponse, Error=ApiError>>;


    fn get_agent(&self, runmode: String, name: String) -> Box<Future<Item=GetAgentResponse, Error=ApiError>>;


    fn get_agents(&self, runmode: String) -> Box<Future<Item=GetAgentsResponse, Error=ApiError>>;


    fn get_authorizable_keystore(&self, intermediate_path: String, authorizable_id: String) -> Box<Future<Item=GetAuthorizableKeystoreResponse, Error=ApiError>>;


    fn get_keystore(&self, intermediate_path: String, authorizable_id: String) -> Box<Future<Item=GetKeystoreResponse, Error=ApiError>>;


    fn get_node(&self, path: String, name: String) -> Box<Future<Item=GetNodeResponse, Error=ApiError>>;


    fn get_package(&self, group: String, name: String, version: String) -> Box<Future<Item=GetPackageResponse, Error=ApiError>>;


    fn get_package_filter(&self, group: String, name: String, version: String) -> Box<Future<Item=GetPackageFilterResponse, Error=ApiError>>;


    fn get_query(&self, path: String, p_limit: f64, _1_property: String, _1_property_value: String) -> Box<Future<Item=GetQueryResponse, Error=ApiError>>;


    fn get_truststore(&self) -> Box<Future<Item=GetTruststoreResponse, Error=ApiError>>;


    fn get_truststore_info(&self) -> Box<Future<Item=GetTruststoreInfoResponse, Error=ApiError>>;


    fn post_agent(&self, runmode: String, name: String, jcrcontentcqdistribute: Option<bool>, jcrcontentcqdistribute_type_hint: Option<String>, jcrcontentcqname: Option<String>, jcrcontentcqtemplate: Option<String>, jcrcontentenabled: Option<bool>, jcrcontentjcrdescription: Option<String>, jcrcontentjcrlast_modified: Option<String>, jcrcontentjcrlast_modified_by: Option<String>, jcrcontentjcrmixin_types: Option<String>, jcrcontentjcrtitle: Option<String>, jcrcontentlog_level: Option<String>, jcrcontentno_status_update: Option<bool>, jcrcontentno_versioning: Option<bool>, jcrcontentprotocol_connect_timeout: Option<f64>, jcrcontentprotocol_http_connection_closed: Option<bool>, jcrcontentprotocol_http_expired: Option<String>, jcrcontentprotocol_http_headers: Option<&Vec<String>>, jcrcontentprotocol_http_headers_type_hint: Option<String>, jcrcontentprotocol_http_method: Option<String>, jcrcontentprotocol_https_relaxed: Option<bool>, jcrcontentprotocol_interface: Option<String>, jcrcontentprotocol_socket_timeout: Option<f64>, jcrcontentprotocol_version: Option<String>, jcrcontentproxy_ntlm_domain: Option<String>, jcrcontentproxy_ntlm_host: Option<String>, jcrcontentproxy_host: Option<String>, jcrcontentproxy_password: Option<String>, jcrcontentproxy_port: Option<f64>, jcrcontentproxy_user: Option<String>, jcrcontentqueue_batch_max_size: Option<f64>, jcrcontentqueue_batch_mode: Option<String>, jcrcontentqueue_batch_wait_time: Option<f64>, jcrcontentretry_delay: Option<String>, jcrcontentreverse_replication: Option<bool>, jcrcontentserialization_type: Option<String>, jcrcontentslingresource_type: Option<String>, jcrcontentssl: Option<String>, jcrcontenttransport_ntlm_domain: Option<String>, jcrcontenttransport_ntlm_host: Option<String>, jcrcontenttransport_password: Option<String>, jcrcontenttransport_uri: Option<String>, jcrcontenttransport_user: Option<String>, jcrcontenttrigger_distribute: Option<bool>, jcrcontenttrigger_modified: Option<bool>, jcrcontenttrigger_on_off_time: Option<bool>, jcrcontenttrigger_receive: Option<bool>, jcrcontenttrigger_specific: Option<bool>, jcrcontentuser_id: Option<String>, jcrprimary_type: Option<String>, operation: Option<String>) -> Box<Future<Item=PostAgentResponse, Error=ApiError>>;


    fn post_authorizable_keystore(&self, intermediate_path: String, authorizable_id: String, operation: Option<String>, current_password: Option<String>, new_password: Option<String>, re_password: Option<String>, key_password: Option<String>, key_store_pass: Option<String>, alias: Option<String>, new_alias: Option<String>, remove_alias: Option<String>, cert_chain: Option<swagger::ByteArray>, pk: Option<swagger::ByteArray>, key_store: Option<swagger::ByteArray>) -> Box<Future<Item=PostAuthorizableKeystoreResponse, Error=ApiError>>;


    fn post_authorizables(&self, authorizable_id: String, intermediate_path: String, create_user: Option<String>, create_group: Option<String>, reppassword: Option<String>, profilegiven_name: Option<String>) -> Box<Future<Item=PostAuthorizablesResponse, Error=ApiError>>;


    fn post_config_adobe_granite_saml_authentication_handler(&self, key_store_password: Option<String>, key_store_password_type_hint: Option<String>, service_ranking: Option<i32>, service_ranking_type_hint: Option<String>, idp_http_redirect: Option<bool>, idp_http_redirect_type_hint: Option<String>, create_user: Option<bool>, create_user_type_hint: Option<String>, default_redirect_url: Option<String>, default_redirect_url_type_hint: Option<String>, user_id_attribute: Option<String>, user_id_attribute_type_hint: Option<String>, default_groups: Option<&Vec<String>>, default_groups_type_hint: Option<String>, idp_cert_alias: Option<String>, idp_cert_alias_type_hint: Option<String>, add_group_memberships: Option<bool>, add_group_memberships_type_hint: Option<String>, path: Option<&Vec<String>>, path_type_hint: Option<String>, synchronize_attributes: Option<&Vec<String>>, synchronize_attributes_type_hint: Option<String>, clock_tolerance: Option<i32>, clock_tolerance_type_hint: Option<String>, group_membership_attribute: Option<String>, group_membership_attribute_type_hint: Option<String>, idp_url: Option<String>, idp_url_type_hint: Option<String>, logout_url: Option<String>, logout_url_type_hint: Option<String>, service_provider_entity_id: Option<String>, service_provider_entity_id_type_hint: Option<String>, assertion_consumer_service_url: Option<String>, assertion_consumer_service_url_type_hint: Option<String>, handle_logout: Option<bool>, handle_logout_type_hint: Option<String>, sp_private_key_alias: Option<String>, sp_private_key_alias_type_hint: Option<String>, use_encryption: Option<bool>, use_encryption_type_hint: Option<String>, name_id_format: Option<String>, name_id_format_type_hint: Option<String>, digest_method: Option<String>, digest_method_type_hint: Option<String>, signature_method: Option<String>, signature_method_type_hint: Option<String>, user_intermediate_path: Option<String>, user_intermediate_path_type_hint: Option<String>) -> Box<Future<Item=PostConfigAdobeGraniteSamlAuthenticationHandlerResponse, Error=ApiError>>;


    fn post_config_apache_felix_jetty_based_http_service(&self, org_apache_felix_https_nio: Option<bool>, org_apache_felix_https_nio_type_hint: Option<String>, org_apache_felix_https_keystore: Option<String>, org_apache_felix_https_keystore_type_hint: Option<String>, org_apache_felix_https_keystore_password: Option<String>, org_apache_felix_https_keystore_password_type_hint: Option<String>, org_apache_felix_https_keystore_key: Option<String>, org_apache_felix_https_keystore_key_type_hint: Option<String>, org_apache_felix_https_keystore_key_password: Option<String>, org_apache_felix_https_keystore_key_password_type_hint: Option<String>, org_apache_felix_https_truststore: Option<String>, org_apache_felix_https_truststore_type_hint: Option<String>, org_apache_felix_https_truststore_password: Option<String>, org_apache_felix_https_truststore_password_type_hint: Option<String>, org_apache_felix_https_clientcertificate: Option<String>, org_apache_felix_https_clientcertificate_type_hint: Option<String>, org_apache_felix_https_enable: Option<bool>, org_apache_felix_https_enable_type_hint: Option<String>, org_osgi_service_http_port_secure: Option<String>, org_osgi_service_http_port_secure_type_hint: Option<String>) -> Box<Future<Item=PostConfigApacheFelixJettyBasedHttpServiceResponse, Error=ApiError>>;


    fn post_config_apache_http_components_proxy_configuration(&self, proxy_host: Option<String>, proxy_host_type_hint: Option<String>, proxy_port: Option<i32>, proxy_port_type_hint: Option<String>, proxy_exceptions: Option<&Vec<String>>, proxy_exceptions_type_hint: Option<String>, proxy_enabled: Option<bool>, proxy_enabled_type_hint: Option<String>, proxy_user: Option<String>, proxy_user_type_hint: Option<String>, proxy_password: Option<String>, proxy_password_type_hint: Option<String>) -> Box<Future<Item=PostConfigApacheHttpComponentsProxyConfigurationResponse, Error=ApiError>>;


    fn post_config_apache_sling_dav_ex_servlet(&self, alias: Option<String>, alias_type_hint: Option<String>, dav_create_absolute_uri: Option<bool>, dav_create_absolute_uri_type_hint: Option<String>) -> Box<Future<Item=PostConfigApacheSlingDavExServletResponse, Error=ApiError>>;


    fn post_config_apache_sling_get_servlet(&self, json_maximumresults: Option<String>, json_maximumresults_type_hint: Option<String>, enable_html: Option<bool>, enable_html_type_hint: Option<String>, enable_txt: Option<bool>, enable_txt_type_hint: Option<String>, enable_xml: Option<bool>, enable_xml_type_hint: Option<String>) -> Box<Future<Item=PostConfigApacheSlingGetServletResponse, Error=ApiError>>;


    fn post_config_apache_sling_referrer_filter(&self, allow_empty: Option<bool>, allow_empty_type_hint: Option<String>, allow_hosts: Option<String>, allow_hosts_type_hint: Option<String>, allow_hosts_regexp: Option<String>, allow_hosts_regexp_type_hint: Option<String>, filter_methods: Option<String>, filter_methods_type_hint: Option<String>) -> Box<Future<Item=PostConfigApacheSlingReferrerFilterResponse, Error=ApiError>>;


    fn post_node(&self, path: String, name: String, operation: Option<String>, delete_authorizable: Option<String>, file: Option<swagger::ByteArray>) -> Box<Future<Item=PostNodeResponse, Error=ApiError>>;


    fn post_node_rw(&self, path: String, name: String, add_members: Option<String>) -> Box<Future<Item=PostNodeRwResponse, Error=ApiError>>;


    fn post_path(&self, path: String, jcrprimary_type: String, name: String) -> Box<Future<Item=PostPathResponse, Error=ApiError>>;


    fn post_query(&self, path: String, p_limit: f64, _1_property: String, _1_property_value: String) -> Box<Future<Item=PostQueryResponse, Error=ApiError>>;


    fn post_tree_activation(&self, ignoredeactivated: bool, onlymodified: bool, path: String) -> Box<Future<Item=PostTreeActivationResponse, Error=ApiError>>;


    fn post_truststore(&self, operation: Option<String>, new_password: Option<String>, re_password: Option<String>, key_store_type: Option<String>, remove_alias: Option<String>, certificate: Option<swagger::ByteArray>) -> Box<Future<Item=PostTruststoreResponse, Error=ApiError>>;


    fn post_truststore_pkcs12(&self, truststore_p12: Option<swagger::ByteArray>) -> Box<Future<Item=PostTruststorePKCS12Response, Error=ApiError>>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<'a, C> where Self: Sized {
    /// Binds this API to a context.
    fn with_context(self: &'a Self, context: C) -> ContextWrapper<'a, Self, C>;
}

impl<'a, T: Api<C> + Sized, C> ContextWrapperExt<'a, C> for T {
    fn with_context(self: &'a T, context: C) -> ContextWrapper<'a, T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

impl<'a, T: Api<C>, C> ApiNoContext for ContextWrapper<'a, T, C> {


    fn get_aem_product_info(&self) -> Box<Future<Item=GetAemProductInfoResponse, Error=ApiError>> {
        self.api().get_aem_product_info(&self.context())
    }


    fn get_config_mgr(&self) -> Box<Future<Item=GetConfigMgrResponse, Error=ApiError>> {
        self.api().get_config_mgr(&self.context())
    }


    fn post_bundle(&self, name: String, action: String) -> Box<Future<Item=PostBundleResponse, Error=ApiError>> {
        self.api().post_bundle(name, action, &self.context())
    }


    fn post_jmx_repository(&self, action: String) -> Box<Future<Item=PostJmxRepositoryResponse, Error=ApiError>> {
        self.api().post_jmx_repository(action, &self.context())
    }


    fn post_saml_configuration(&self, post: Option<bool>, apply: Option<bool>, delete: Option<bool>, action: Option<String>, location: Option<String>, path: Option<&Vec<String>>, service_ranking: Option<i32>, idp_url: Option<String>, idp_cert_alias: Option<String>, idp_http_redirect: Option<bool>, service_provider_entity_id: Option<String>, assertion_consumer_service_url: Option<String>, sp_private_key_alias: Option<String>, key_store_password: Option<String>, default_redirect_url: Option<String>, user_id_attribute: Option<String>, use_encryption: Option<bool>, create_user: Option<bool>, add_group_memberships: Option<bool>, group_membership_attribute: Option<String>, default_groups: Option<&Vec<String>>, name_id_format: Option<String>, synchronize_attributes: Option<&Vec<String>>, handle_logout: Option<bool>, logout_url: Option<String>, clock_tolerance: Option<i32>, digest_method: Option<String>, signature_method: Option<String>, user_intermediate_path: Option<String>, propertylist: Option<&Vec<String>>) -> Box<Future<Item=PostSamlConfigurationResponse, Error=ApiError>> {
        self.api().post_saml_configuration(post, apply, delete, action, location, path, service_ranking, idp_url, idp_cert_alias, idp_http_redirect, service_provider_entity_id, assertion_consumer_service_url, sp_private_key_alias, key_store_password, default_redirect_url, user_id_attribute, use_encryption, create_user, add_group_memberships, group_membership_attribute, default_groups, name_id_format, synchronize_attributes, handle_logout, logout_url, clock_tolerance, digest_method, signature_method, user_intermediate_path, propertylist, &self.context())
    }


    fn get_login_page(&self) -> Box<Future<Item=GetLoginPageResponse, Error=ApiError>> {
        self.api().get_login_page(&self.context())
    }


    fn post_cq_actions(&self, authorizable_id: String, changelog: String) -> Box<Future<Item=PostCqActionsResponse, Error=ApiError>> {
        self.api().post_cq_actions(authorizable_id, changelog, &self.context())
    }


    fn get_crxde_status(&self) -> Box<Future<Item=GetCrxdeStatusResponse, Error=ApiError>> {
        self.api().get_crxde_status(&self.context())
    }


    fn get_install_status(&self) -> Box<Future<Item=GetInstallStatusResponse, Error=ApiError>> {
        self.api().get_install_status(&self.context())
    }


    fn get_package_manager_servlet(&self) -> Box<Future<Item=GetPackageManagerServletResponse, Error=ApiError>> {
        self.api().get_package_manager_servlet(&self.context())
    }


    fn post_package_service(&self, cmd: String) -> Box<Future<Item=PostPackageServiceResponse, Error=ApiError>> {
        self.api().post_package_service(cmd, &self.context())
    }


    fn post_package_service_json(&self, path: String, cmd: String, group_name: Option<String>, package_name: Option<String>, package_version: Option<String>, _charset_: Option<String>, force: Option<bool>, recursive: Option<bool>, package: Option<swagger::ByteArray>) -> Box<Future<Item=PostPackageServiceJsonResponse, Error=ApiError>> {
        self.api().post_package_service_json(path, cmd, group_name, package_name, package_version, _charset_, force, recursive, package, &self.context())
    }


    fn post_package_update(&self, group_name: String, package_name: String, version: String, path: String, filter: Option<String>, _charset_: Option<String>) -> Box<Future<Item=PostPackageUpdateResponse, Error=ApiError>> {
        self.api().post_package_update(group_name, package_name, version, path, filter, _charset_, &self.context())
    }


    fn post_set_password(&self, old: String, plain: String, verify: String) -> Box<Future<Item=PostSetPasswordResponse, Error=ApiError>> {
        self.api().post_set_password(old, plain, verify, &self.context())
    }


    fn get_aem_health_check(&self, tags: Option<String>, combine_tags_or: Option<bool>) -> Box<Future<Item=GetAemHealthCheckResponse, Error=ApiError>> {
        self.api().get_aem_health_check(tags, combine_tags_or, &self.context())
    }


    fn post_config_aem_health_check_servlet(&self, bundles_ignored: Option<&Vec<String>>, bundles_ignored_type_hint: Option<String>) -> Box<Future<Item=PostConfigAemHealthCheckServletResponse, Error=ApiError>> {
        self.api().post_config_aem_health_check_servlet(bundles_ignored, bundles_ignored_type_hint, &self.context())
    }


    fn post_config_aem_password_reset(&self, pwdreset_authorizables: Option<&Vec<String>>, pwdreset_authorizables_type_hint: Option<String>) -> Box<Future<Item=PostConfigAemPasswordResetResponse, Error=ApiError>> {
        self.api().post_config_aem_password_reset(pwdreset_authorizables, pwdreset_authorizables_type_hint, &self.context())
    }


    fn delete_agent(&self, runmode: String, name: String) -> Box<Future<Item=DeleteAgentResponse, Error=ApiError>> {
        self.api().delete_agent(runmode, name, &self.context())
    }


    fn delete_node(&self, path: String, name: String) -> Box<Future<Item=DeleteNodeResponse, Error=ApiError>> {
        self.api().delete_node(path, name, &self.context())
    }


    fn get_agent(&self, runmode: String, name: String) -> Box<Future<Item=GetAgentResponse, Error=ApiError>> {
        self.api().get_agent(runmode, name, &self.context())
    }


    fn get_agents(&self, runmode: String) -> Box<Future<Item=GetAgentsResponse, Error=ApiError>> {
        self.api().get_agents(runmode, &self.context())
    }


    fn get_authorizable_keystore(&self, intermediate_path: String, authorizable_id: String) -> Box<Future<Item=GetAuthorizableKeystoreResponse, Error=ApiError>> {
        self.api().get_authorizable_keystore(intermediate_path, authorizable_id, &self.context())
    }


    fn get_keystore(&self, intermediate_path: String, authorizable_id: String) -> Box<Future<Item=GetKeystoreResponse, Error=ApiError>> {
        self.api().get_keystore(intermediate_path, authorizable_id, &self.context())
    }


    fn get_node(&self, path: String, name: String) -> Box<Future<Item=GetNodeResponse, Error=ApiError>> {
        self.api().get_node(path, name, &self.context())
    }


    fn get_package(&self, group: String, name: String, version: String) -> Box<Future<Item=GetPackageResponse, Error=ApiError>> {
        self.api().get_package(group, name, version, &self.context())
    }


    fn get_package_filter(&self, group: String, name: String, version: String) -> Box<Future<Item=GetPackageFilterResponse, Error=ApiError>> {
        self.api().get_package_filter(group, name, version, &self.context())
    }


    fn get_query(&self, path: String, p_limit: f64, _1_property: String, _1_property_value: String) -> Box<Future<Item=GetQueryResponse, Error=ApiError>> {
        self.api().get_query(path, p_limit, _1_property, _1_property_value, &self.context())
    }


    fn get_truststore(&self) -> Box<Future<Item=GetTruststoreResponse, Error=ApiError>> {
        self.api().get_truststore(&self.context())
    }


    fn get_truststore_info(&self) -> Box<Future<Item=GetTruststoreInfoResponse, Error=ApiError>> {
        self.api().get_truststore_info(&self.context())
    }


    fn post_agent(&self, runmode: String, name: String, jcrcontentcqdistribute: Option<bool>, jcrcontentcqdistribute_type_hint: Option<String>, jcrcontentcqname: Option<String>, jcrcontentcqtemplate: Option<String>, jcrcontentenabled: Option<bool>, jcrcontentjcrdescription: Option<String>, jcrcontentjcrlast_modified: Option<String>, jcrcontentjcrlast_modified_by: Option<String>, jcrcontentjcrmixin_types: Option<String>, jcrcontentjcrtitle: Option<String>, jcrcontentlog_level: Option<String>, jcrcontentno_status_update: Option<bool>, jcrcontentno_versioning: Option<bool>, jcrcontentprotocol_connect_timeout: Option<f64>, jcrcontentprotocol_http_connection_closed: Option<bool>, jcrcontentprotocol_http_expired: Option<String>, jcrcontentprotocol_http_headers: Option<&Vec<String>>, jcrcontentprotocol_http_headers_type_hint: Option<String>, jcrcontentprotocol_http_method: Option<String>, jcrcontentprotocol_https_relaxed: Option<bool>, jcrcontentprotocol_interface: Option<String>, jcrcontentprotocol_socket_timeout: Option<f64>, jcrcontentprotocol_version: Option<String>, jcrcontentproxy_ntlm_domain: Option<String>, jcrcontentproxy_ntlm_host: Option<String>, jcrcontentproxy_host: Option<String>, jcrcontentproxy_password: Option<String>, jcrcontentproxy_port: Option<f64>, jcrcontentproxy_user: Option<String>, jcrcontentqueue_batch_max_size: Option<f64>, jcrcontentqueue_batch_mode: Option<String>, jcrcontentqueue_batch_wait_time: Option<f64>, jcrcontentretry_delay: Option<String>, jcrcontentreverse_replication: Option<bool>, jcrcontentserialization_type: Option<String>, jcrcontentslingresource_type: Option<String>, jcrcontentssl: Option<String>, jcrcontenttransport_ntlm_domain: Option<String>, jcrcontenttransport_ntlm_host: Option<String>, jcrcontenttransport_password: Option<String>, jcrcontenttransport_uri: Option<String>, jcrcontenttransport_user: Option<String>, jcrcontenttrigger_distribute: Option<bool>, jcrcontenttrigger_modified: Option<bool>, jcrcontenttrigger_on_off_time: Option<bool>, jcrcontenttrigger_receive: Option<bool>, jcrcontenttrigger_specific: Option<bool>, jcrcontentuser_id: Option<String>, jcrprimary_type: Option<String>, operation: Option<String>) -> Box<Future<Item=PostAgentResponse, Error=ApiError>> {
        self.api().post_agent(runmode, name, jcrcontentcqdistribute, jcrcontentcqdistribute_type_hint, jcrcontentcqname, jcrcontentcqtemplate, jcrcontentenabled, jcrcontentjcrdescription, jcrcontentjcrlast_modified, jcrcontentjcrlast_modified_by, jcrcontentjcrmixin_types, jcrcontentjcrtitle, jcrcontentlog_level, jcrcontentno_status_update, jcrcontentno_versioning, jcrcontentprotocol_connect_timeout, jcrcontentprotocol_http_connection_closed, jcrcontentprotocol_http_expired, jcrcontentprotocol_http_headers, jcrcontentprotocol_http_headers_type_hint, jcrcontentprotocol_http_method, jcrcontentprotocol_https_relaxed, jcrcontentprotocol_interface, jcrcontentprotocol_socket_timeout, jcrcontentprotocol_version, jcrcontentproxy_ntlm_domain, jcrcontentproxy_ntlm_host, jcrcontentproxy_host, jcrcontentproxy_password, jcrcontentproxy_port, jcrcontentproxy_user, jcrcontentqueue_batch_max_size, jcrcontentqueue_batch_mode, jcrcontentqueue_batch_wait_time, jcrcontentretry_delay, jcrcontentreverse_replication, jcrcontentserialization_type, jcrcontentslingresource_type, jcrcontentssl, jcrcontenttransport_ntlm_domain, jcrcontenttransport_ntlm_host, jcrcontenttransport_password, jcrcontenttransport_uri, jcrcontenttransport_user, jcrcontenttrigger_distribute, jcrcontenttrigger_modified, jcrcontenttrigger_on_off_time, jcrcontenttrigger_receive, jcrcontenttrigger_specific, jcrcontentuser_id, jcrprimary_type, operation, &self.context())
    }


    fn post_authorizable_keystore(&self, intermediate_path: String, authorizable_id: String, operation: Option<String>, current_password: Option<String>, new_password: Option<String>, re_password: Option<String>, key_password: Option<String>, key_store_pass: Option<String>, alias: Option<String>, new_alias: Option<String>, remove_alias: Option<String>, cert_chain: Option<swagger::ByteArray>, pk: Option<swagger::ByteArray>, key_store: Option<swagger::ByteArray>) -> Box<Future<Item=PostAuthorizableKeystoreResponse, Error=ApiError>> {
        self.api().post_authorizable_keystore(intermediate_path, authorizable_id, operation, current_password, new_password, re_password, key_password, key_store_pass, alias, new_alias, remove_alias, cert_chain, pk, key_store, &self.context())
    }


    fn post_authorizables(&self, authorizable_id: String, intermediate_path: String, create_user: Option<String>, create_group: Option<String>, reppassword: Option<String>, profilegiven_name: Option<String>) -> Box<Future<Item=PostAuthorizablesResponse, Error=ApiError>> {
        self.api().post_authorizables(authorizable_id, intermediate_path, create_user, create_group, reppassword, profilegiven_name, &self.context())
    }


    fn post_config_adobe_granite_saml_authentication_handler(&self, key_store_password: Option<String>, key_store_password_type_hint: Option<String>, service_ranking: Option<i32>, service_ranking_type_hint: Option<String>, idp_http_redirect: Option<bool>, idp_http_redirect_type_hint: Option<String>, create_user: Option<bool>, create_user_type_hint: Option<String>, default_redirect_url: Option<String>, default_redirect_url_type_hint: Option<String>, user_id_attribute: Option<String>, user_id_attribute_type_hint: Option<String>, default_groups: Option<&Vec<String>>, default_groups_type_hint: Option<String>, idp_cert_alias: Option<String>, idp_cert_alias_type_hint: Option<String>, add_group_memberships: Option<bool>, add_group_memberships_type_hint: Option<String>, path: Option<&Vec<String>>, path_type_hint: Option<String>, synchronize_attributes: Option<&Vec<String>>, synchronize_attributes_type_hint: Option<String>, clock_tolerance: Option<i32>, clock_tolerance_type_hint: Option<String>, group_membership_attribute: Option<String>, group_membership_attribute_type_hint: Option<String>, idp_url: Option<String>, idp_url_type_hint: Option<String>, logout_url: Option<String>, logout_url_type_hint: Option<String>, service_provider_entity_id: Option<String>, service_provider_entity_id_type_hint: Option<String>, assertion_consumer_service_url: Option<String>, assertion_consumer_service_url_type_hint: Option<String>, handle_logout: Option<bool>, handle_logout_type_hint: Option<String>, sp_private_key_alias: Option<String>, sp_private_key_alias_type_hint: Option<String>, use_encryption: Option<bool>, use_encryption_type_hint: Option<String>, name_id_format: Option<String>, name_id_format_type_hint: Option<String>, digest_method: Option<String>, digest_method_type_hint: Option<String>, signature_method: Option<String>, signature_method_type_hint: Option<String>, user_intermediate_path: Option<String>, user_intermediate_path_type_hint: Option<String>) -> Box<Future<Item=PostConfigAdobeGraniteSamlAuthenticationHandlerResponse, Error=ApiError>> {
        self.api().post_config_adobe_granite_saml_authentication_handler(key_store_password, key_store_password_type_hint, service_ranking, service_ranking_type_hint, idp_http_redirect, idp_http_redirect_type_hint, create_user, create_user_type_hint, default_redirect_url, default_redirect_url_type_hint, user_id_attribute, user_id_attribute_type_hint, default_groups, default_groups_type_hint, idp_cert_alias, idp_cert_alias_type_hint, add_group_memberships, add_group_memberships_type_hint, path, path_type_hint, synchronize_attributes, synchronize_attributes_type_hint, clock_tolerance, clock_tolerance_type_hint, group_membership_attribute, group_membership_attribute_type_hint, idp_url, idp_url_type_hint, logout_url, logout_url_type_hint, service_provider_entity_id, service_provider_entity_id_type_hint, assertion_consumer_service_url, assertion_consumer_service_url_type_hint, handle_logout, handle_logout_type_hint, sp_private_key_alias, sp_private_key_alias_type_hint, use_encryption, use_encryption_type_hint, name_id_format, name_id_format_type_hint, digest_method, digest_method_type_hint, signature_method, signature_method_type_hint, user_intermediate_path, user_intermediate_path_type_hint, &self.context())
    }


    fn post_config_apache_felix_jetty_based_http_service(&self, org_apache_felix_https_nio: Option<bool>, org_apache_felix_https_nio_type_hint: Option<String>, org_apache_felix_https_keystore: Option<String>, org_apache_felix_https_keystore_type_hint: Option<String>, org_apache_felix_https_keystore_password: Option<String>, org_apache_felix_https_keystore_password_type_hint: Option<String>, org_apache_felix_https_keystore_key: Option<String>, org_apache_felix_https_keystore_key_type_hint: Option<String>, org_apache_felix_https_keystore_key_password: Option<String>, org_apache_felix_https_keystore_key_password_type_hint: Option<String>, org_apache_felix_https_truststore: Option<String>, org_apache_felix_https_truststore_type_hint: Option<String>, org_apache_felix_https_truststore_password: Option<String>, org_apache_felix_https_truststore_password_type_hint: Option<String>, org_apache_felix_https_clientcertificate: Option<String>, org_apache_felix_https_clientcertificate_type_hint: Option<String>, org_apache_felix_https_enable: Option<bool>, org_apache_felix_https_enable_type_hint: Option<String>, org_osgi_service_http_port_secure: Option<String>, org_osgi_service_http_port_secure_type_hint: Option<String>) -> Box<Future<Item=PostConfigApacheFelixJettyBasedHttpServiceResponse, Error=ApiError>> {
        self.api().post_config_apache_felix_jetty_based_http_service(org_apache_felix_https_nio, org_apache_felix_https_nio_type_hint, org_apache_felix_https_keystore, org_apache_felix_https_keystore_type_hint, org_apache_felix_https_keystore_password, org_apache_felix_https_keystore_password_type_hint, org_apache_felix_https_keystore_key, org_apache_felix_https_keystore_key_type_hint, org_apache_felix_https_keystore_key_password, org_apache_felix_https_keystore_key_password_type_hint, org_apache_felix_https_truststore, org_apache_felix_https_truststore_type_hint, org_apache_felix_https_truststore_password, org_apache_felix_https_truststore_password_type_hint, org_apache_felix_https_clientcertificate, org_apache_felix_https_clientcertificate_type_hint, org_apache_felix_https_enable, org_apache_felix_https_enable_type_hint, org_osgi_service_http_port_secure, org_osgi_service_http_port_secure_type_hint, &self.context())
    }


    fn post_config_apache_http_components_proxy_configuration(&self, proxy_host: Option<String>, proxy_host_type_hint: Option<String>, proxy_port: Option<i32>, proxy_port_type_hint: Option<String>, proxy_exceptions: Option<&Vec<String>>, proxy_exceptions_type_hint: Option<String>, proxy_enabled: Option<bool>, proxy_enabled_type_hint: Option<String>, proxy_user: Option<String>, proxy_user_type_hint: Option<String>, proxy_password: Option<String>, proxy_password_type_hint: Option<String>) -> Box<Future<Item=PostConfigApacheHttpComponentsProxyConfigurationResponse, Error=ApiError>> {
        self.api().post_config_apache_http_components_proxy_configuration(proxy_host, proxy_host_type_hint, proxy_port, proxy_port_type_hint, proxy_exceptions, proxy_exceptions_type_hint, proxy_enabled, proxy_enabled_type_hint, proxy_user, proxy_user_type_hint, proxy_password, proxy_password_type_hint, &self.context())
    }


    fn post_config_apache_sling_dav_ex_servlet(&self, alias: Option<String>, alias_type_hint: Option<String>, dav_create_absolute_uri: Option<bool>, dav_create_absolute_uri_type_hint: Option<String>) -> Box<Future<Item=PostConfigApacheSlingDavExServletResponse, Error=ApiError>> {
        self.api().post_config_apache_sling_dav_ex_servlet(alias, alias_type_hint, dav_create_absolute_uri, dav_create_absolute_uri_type_hint, &self.context())
    }


    fn post_config_apache_sling_get_servlet(&self, json_maximumresults: Option<String>, json_maximumresults_type_hint: Option<String>, enable_html: Option<bool>, enable_html_type_hint: Option<String>, enable_txt: Option<bool>, enable_txt_type_hint: Option<String>, enable_xml: Option<bool>, enable_xml_type_hint: Option<String>) -> Box<Future<Item=PostConfigApacheSlingGetServletResponse, Error=ApiError>> {
        self.api().post_config_apache_sling_get_servlet(json_maximumresults, json_maximumresults_type_hint, enable_html, enable_html_type_hint, enable_txt, enable_txt_type_hint, enable_xml, enable_xml_type_hint, &self.context())
    }


    fn post_config_apache_sling_referrer_filter(&self, allow_empty: Option<bool>, allow_empty_type_hint: Option<String>, allow_hosts: Option<String>, allow_hosts_type_hint: Option<String>, allow_hosts_regexp: Option<String>, allow_hosts_regexp_type_hint: Option<String>, filter_methods: Option<String>, filter_methods_type_hint: Option<String>) -> Box<Future<Item=PostConfigApacheSlingReferrerFilterResponse, Error=ApiError>> {
        self.api().post_config_apache_sling_referrer_filter(allow_empty, allow_empty_type_hint, allow_hosts, allow_hosts_type_hint, allow_hosts_regexp, allow_hosts_regexp_type_hint, filter_methods, filter_methods_type_hint, &self.context())
    }


    fn post_node(&self, path: String, name: String, operation: Option<String>, delete_authorizable: Option<String>, file: Option<swagger::ByteArray>) -> Box<Future<Item=PostNodeResponse, Error=ApiError>> {
        self.api().post_node(path, name, operation, delete_authorizable, file, &self.context())
    }


    fn post_node_rw(&self, path: String, name: String, add_members: Option<String>) -> Box<Future<Item=PostNodeRwResponse, Error=ApiError>> {
        self.api().post_node_rw(path, name, add_members, &self.context())
    }


    fn post_path(&self, path: String, jcrprimary_type: String, name: String) -> Box<Future<Item=PostPathResponse, Error=ApiError>> {
        self.api().post_path(path, jcrprimary_type, name, &self.context())
    }


    fn post_query(&self, path: String, p_limit: f64, _1_property: String, _1_property_value: String) -> Box<Future<Item=PostQueryResponse, Error=ApiError>> {
        self.api().post_query(path, p_limit, _1_property, _1_property_value, &self.context())
    }


    fn post_tree_activation(&self, ignoredeactivated: bool, onlymodified: bool, path: String) -> Box<Future<Item=PostTreeActivationResponse, Error=ApiError>> {
        self.api().post_tree_activation(ignoredeactivated, onlymodified, path, &self.context())
    }


    fn post_truststore(&self, operation: Option<String>, new_password: Option<String>, re_password: Option<String>, key_store_type: Option<String>, remove_alias: Option<String>, certificate: Option<swagger::ByteArray>) -> Box<Future<Item=PostTruststoreResponse, Error=ApiError>> {
        self.api().post_truststore(operation, new_password, re_password, key_store_type, remove_alias, certificate, &self.context())
    }


    fn post_truststore_pkcs12(&self, truststore_p12: Option<swagger::ByteArray>) -> Box<Future<Item=PostTruststorePKCS12Response, Error=ApiError>> {
        self.api().post_truststore_pkcs12(truststore_p12, &self.context())
    }

}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use self::client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

pub mod models;
