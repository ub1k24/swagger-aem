# OpenAPIClient-php
Swagger AEM is an OpenAPI specification for Adobe Experience Manager (AEM) API

This PHP package is automatically generated by the [OpenAPI Generator](https://openapi-generator.tech) project:

- API version: 3.2.0-pre.0
- Build package: org.openapitools.codegen.languages.PhpClientCodegen
For more information, please visit [http://shinesolutions.com](http://shinesolutions.com)

## Requirements

PHP 5.5 and later

## Installation & Usage
### Composer

To install the bindings via [Composer](http://getcomposer.org/), add the following to `composer.json`:

```
{
  "repositories": [
    {
      "type": "git",
      "url": "https://github.com/GIT_USER_ID/GIT_REPO_ID.git"
    }
  ],
  "require": {
    "GIT_USER_ID/GIT_REPO_ID": "*@dev"
  }
}
```

Then run `composer install`

### Manual Installation

Download the files and include `autoload.php`:

```php
    require_once('/path/to/OpenAPIClient-php/vendor/autoload.php');
```

## Tests

To run the unit tests:

```
composer install
./vendor/bin/phpunit
```

## Getting Started

Please follow the [installation procedure](#installation--usage) and then run the following:

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Configure HTTP basic authorization: aemAuth
$config = OpenAPI\Client\Configuration::getDefaultConfiguration()
    ->setUsername('YOUR_USERNAME')
    ->setPassword('YOUR_PASSWORD');

$apiInstance = new OpenAPI\Client\Api\ConsoleApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client(),
    $config
);

try {
    $result = $apiInstance->getAemProductInfo();
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ConsoleApi->getAemProductInfo: ', $e->getMessage(), PHP_EOL;
}

?>
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ConsoleApi* | [**getAemProductInfo**](docs/Api/ConsoleApi.md#getaemproductinfo) | **GET** /system/console/status-productinfo.json | 
*ConsoleApi* | [**getConfigMgr**](docs/Api/ConsoleApi.md#getconfigmgr) | **GET** /system/console/configMgr | 
*ConsoleApi* | [**postBundle**](docs/Api/ConsoleApi.md#postbundle) | **POST** /system/console/bundles/{name} | 
*ConsoleApi* | [**postJmxRepository**](docs/Api/ConsoleApi.md#postjmxrepository) | **POST** /system/console/jmx/com.adobe.granite:type&#x3D;Repository/op/{action} | 
*ConsoleApi* | [**postSamlConfiguration**](docs/Api/ConsoleApi.md#postsamlconfiguration) | **POST** /system/console/configMgr/com.adobe.granite.auth.saml.SamlAuthenticationHandler | 
*CqApi* | [**getLoginPage**](docs/Api/CqApi.md#getloginpage) | **GET** /libs/granite/core/content/login.html | 
*CqApi* | [**postCqActions**](docs/Api/CqApi.md#postcqactions) | **POST** /.cqactions.html | 
*CrxApi* | [**getCrxdeStatus**](docs/Api/CrxApi.md#getcrxdestatus) | **GET** /crx/server/crx.default/jcr:root/.1.json | 
*CrxApi* | [**getInstallStatus**](docs/Api/CrxApi.md#getinstallstatus) | **GET** /crx/packmgr/installstatus.jsp | 
*CrxApi* | [**getPackageManagerServlet**](docs/Api/CrxApi.md#getpackagemanagerservlet) | **GET** /crx/packmgr/service/script.html | 
*CrxApi* | [**postPackageService**](docs/Api/CrxApi.md#postpackageservice) | **POST** /crx/packmgr/service.jsp | 
*CrxApi* | [**postPackageServiceJson**](docs/Api/CrxApi.md#postpackageservicejson) | **POST** /crx/packmgr/service/.json/{path} | 
*CrxApi* | [**postPackageUpdate**](docs/Api/CrxApi.md#postpackageupdate) | **POST** /crx/packmgr/update.jsp | 
*CrxApi* | [**postSetPassword**](docs/Api/CrxApi.md#postsetpassword) | **POST** /crx/explorer/ui/setpassword.jsp | 
*CustomApi* | [**getAemHealthCheck**](docs/Api/CustomApi.md#getaemhealthcheck) | **GET** /system/health | 
*CustomApi* | [**postConfigAemHealthCheckServlet**](docs/Api/CustomApi.md#postconfigaemhealthcheckservlet) | **POST** /apps/system/config/com.shinesolutions.healthcheck.hc.impl.ActiveBundleHealthCheck | 
*CustomApi* | [**postConfigAemPasswordReset**](docs/Api/CustomApi.md#postconfigaempasswordreset) | **POST** /apps/system/config/com.shinesolutions.aem.passwordreset.Activator | 
*SlingApi* | [**deleteAgent**](docs/Api/SlingApi.md#deleteagent) | **DELETE** /etc/replication/agents.{runmode}/{name} | 
*SlingApi* | [**deleteNode**](docs/Api/SlingApi.md#deletenode) | **DELETE** /{path}/{name} | 
*SlingApi* | [**getAgent**](docs/Api/SlingApi.md#getagent) | **GET** /etc/replication/agents.{runmode}/{name} | 
*SlingApi* | [**getAgents**](docs/Api/SlingApi.md#getagents) | **GET** /etc/replication/agents.{runmode}.-1.json | 
*SlingApi* | [**getAuthorizableKeystore**](docs/Api/SlingApi.md#getauthorizablekeystore) | **GET** /{intermediatePath}/{authorizableId}.ks.json | 
*SlingApi* | [**getKeystore**](docs/Api/SlingApi.md#getkeystore) | **GET** /{intermediatePath}/{authorizableId}/keystore/store.p12 | 
*SlingApi* | [**getNode**](docs/Api/SlingApi.md#getnode) | **GET** /{path}/{name} | 
*SlingApi* | [**getPackage**](docs/Api/SlingApi.md#getpackage) | **GET** /etc/packages/{group}/{name}-{version}.zip | 
*SlingApi* | [**getPackageFilter**](docs/Api/SlingApi.md#getpackagefilter) | **GET** /etc/packages/{group}/{name}-{version}.zip/jcr:content/vlt:definition/filter.tidy.2.json | 
*SlingApi* | [**getQuery**](docs/Api/SlingApi.md#getquery) | **GET** /bin/querybuilder.json | 
*SlingApi* | [**getTruststore**](docs/Api/SlingApi.md#gettruststore) | **GET** /etc/truststore/truststore.p12 | 
*SlingApi* | [**getTruststoreInfo**](docs/Api/SlingApi.md#gettruststoreinfo) | **GET** /libs/granite/security/truststore.json | 
*SlingApi* | [**postAgent**](docs/Api/SlingApi.md#postagent) | **POST** /etc/replication/agents.{runmode}/{name} | 
*SlingApi* | [**postAuthorizableKeystore**](docs/Api/SlingApi.md#postauthorizablekeystore) | **POST** /{intermediatePath}/{authorizableId}.ks.html | 
*SlingApi* | [**postAuthorizables**](docs/Api/SlingApi.md#postauthorizables) | **POST** /libs/granite/security/post/authorizables | 
*SlingApi* | [**postConfigAdobeGraniteSamlAuthenticationHandler**](docs/Api/SlingApi.md#postconfigadobegranitesamlauthenticationhandler) | **POST** /apps/system/config/com.adobe.granite.auth.saml.SamlAuthenticationHandler.config | 
*SlingApi* | [**postConfigApacheFelixJettyBasedHttpService**](docs/Api/SlingApi.md#postconfigapachefelixjettybasedhttpservice) | **POST** /apps/system/config/org.apache.felix.http | 
*SlingApi* | [**postConfigApacheHttpComponentsProxyConfiguration**](docs/Api/SlingApi.md#postconfigapachehttpcomponentsproxyconfiguration) | **POST** /apps/system/config/org.apache.http.proxyconfigurator.config | 
*SlingApi* | [**postConfigApacheSlingDavExServlet**](docs/Api/SlingApi.md#postconfigapacheslingdavexservlet) | **POST** /apps/system/config/org.apache.sling.jcr.davex.impl.servlets.SlingDavExServlet | 
*SlingApi* | [**postConfigApacheSlingGetServlet**](docs/Api/SlingApi.md#postconfigapacheslinggetservlet) | **POST** /apps/system/config/org.apache.sling.servlets.get.DefaultGetServlet | 
*SlingApi* | [**postConfigApacheSlingReferrerFilter**](docs/Api/SlingApi.md#postconfigapacheslingreferrerfilter) | **POST** /apps/system/config/org.apache.sling.security.impl.ReferrerFilter | 
*SlingApi* | [**postNode**](docs/Api/SlingApi.md#postnode) | **POST** /{path}/{name} | 
*SlingApi* | [**postNodeRw**](docs/Api/SlingApi.md#postnoderw) | **POST** /{path}/{name}.rw.html | 
*SlingApi* | [**postPath**](docs/Api/SlingApi.md#postpath) | **POST** /{path}/ | 
*SlingApi* | [**postQuery**](docs/Api/SlingApi.md#postquery) | **POST** /bin/querybuilder.json | 
*SlingApi* | [**postTreeActivation**](docs/Api/SlingApi.md#posttreeactivation) | **POST** /etc/replication/treeactivation.html | 
*SlingApi* | [**postTruststore**](docs/Api/SlingApi.md#posttruststore) | **POST** /libs/granite/security/post/truststore | 
*SlingApi* | [**postTruststorePKCS12**](docs/Api/SlingApi.md#posttruststorepkcs12) | **POST** /etc/truststore | 


## Documentation For Models

 - [InstallStatus](docs/Model/InstallStatus.md)
 - [InstallStatusStatus](docs/Model/InstallStatusStatus.md)
 - [KeystoreChainItems](docs/Model/KeystoreChainItems.md)
 - [KeystoreInfo](docs/Model/KeystoreInfo.md)
 - [KeystoreItems](docs/Model/KeystoreItems.md)
 - [SamlConfigurationInfo](docs/Model/SamlConfigurationInfo.md)
 - [SamlConfigurationProperties](docs/Model/SamlConfigurationProperties.md)
 - [SamlConfigurationPropertyItemsArray](docs/Model/SamlConfigurationPropertyItemsArray.md)
 - [SamlConfigurationPropertyItemsBoolean](docs/Model/SamlConfigurationPropertyItemsBoolean.md)
 - [SamlConfigurationPropertyItemsLong](docs/Model/SamlConfigurationPropertyItemsLong.md)
 - [SamlConfigurationPropertyItemsString](docs/Model/SamlConfigurationPropertyItemsString.md)
 - [TruststoreInfo](docs/Model/TruststoreInfo.md)
 - [TruststoreItems](docs/Model/TruststoreItems.md)


## Documentation For Authorization


## aemAuth

- **Type**: HTTP basic authentication


## Author

opensource@shinesolutions.com


