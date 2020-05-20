/**
 * Adobe Experience Manager (AEM) API
 * Swagger AEM is an OpenAPI specification for Adobe Experience Manager (AEM) API
 *
 * OpenAPI spec version: 3.2.0-pre.0
 * Contact: opensource@shinesolutions.com
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 *
 * OpenAPI Generator version: 3.2.1-SNAPSHOT
 *
 * Do not edit the class manually.
 *
 */

(function(root, factory) {
  if (typeof define === 'function' && define.amd) {
    // AMD. Register as an anonymous module.
    define(['ApiClient', 'model/SamlConfigurationProperties'], factory);
  } else if (typeof module === 'object' && module.exports) {
    // CommonJS-like environments that support module.exports, like Node.
    module.exports = factory(require('../ApiClient'), require('./SamlConfigurationProperties'));
  } else {
    // Browser globals (root is window)
    if (!root.NodeSwaggerAem) {
      root.NodeSwaggerAem = {};
    }
    root.NodeSwaggerAem.SamlConfigurationInfo = factory(root.NodeSwaggerAem.ApiClient, root.NodeSwaggerAem.SamlConfigurationProperties);
  }
}(this, function(ApiClient, SamlConfigurationProperties) {
  'use strict';




  /**
   * The SamlConfigurationInfo model module.
   * @module model/SamlConfigurationInfo
   * @version 0.9.0
   */

  /**
   * Constructs a new <code>SamlConfigurationInfo</code>.
   * @alias module:model/SamlConfigurationInfo
   * @class
   */
  var exports = function() {
    var _this = this;







  };

  /**
   * Constructs a <code>SamlConfigurationInfo</code> from a plain JavaScript object, optionally creating a new instance.
   * Copies all relevant properties from <code>data</code> to <code>obj</code> if supplied or a new instance if not.
   * @param {Object} data The plain JavaScript object bearing properties of interest.
   * @param {module:model/SamlConfigurationInfo} obj Optional instance to populate.
   * @return {module:model/SamlConfigurationInfo} The populated <code>SamlConfigurationInfo</code> instance.
   */
  exports.constructFromObject = function(data, obj) {
    if (data) {
      obj = obj || new exports();

      if (data.hasOwnProperty('pid')) {
        obj['pid'] = ApiClient.convertToType(data['pid'], 'String');
      }
      if (data.hasOwnProperty('title')) {
        obj['title'] = ApiClient.convertToType(data['title'], 'String');
      }
      if (data.hasOwnProperty('description')) {
        obj['description'] = ApiClient.convertToType(data['description'], 'String');
      }
      if (data.hasOwnProperty('bundle_location')) {
        obj['bundle_location'] = ApiClient.convertToType(data['bundle_location'], 'String');
      }
      if (data.hasOwnProperty('service_location')) {
        obj['service_location'] = ApiClient.convertToType(data['service_location'], 'String');
      }
      if (data.hasOwnProperty('properties')) {
        obj['properties'] = SamlConfigurationProperties.constructFromObject(data['properties']);
      }
    }
    return obj;
  }

  /**
   * Persistent Identity (PID)
   * @member {String} pid
   */
  exports.prototype['pid'] = undefined;
  /**
   * Title
   * @member {String} title
   */
  exports.prototype['title'] = undefined;
  /**
   * Title
   * @member {String} description
   */
  exports.prototype['description'] = undefined;
  /**
   * needed for configuration binding
   * @member {String} bundle_location
   */
  exports.prototype['bundle_location'] = undefined;
  /**
   * needed for configuraiton binding
   * @member {String} service_location
   */
  exports.prototype['service_location'] = undefined;
  /**
   * @member {module:model/SamlConfigurationProperties} properties
   */
  exports.prototype['properties'] = undefined;



  return exports;
}));

