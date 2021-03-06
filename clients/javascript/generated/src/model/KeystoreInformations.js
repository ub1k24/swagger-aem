/**
 * Adobe Experience Manager (AEM) API
 * Swagger AEM is an OpenAPI specification for Adobe Experience Manager (AEM) API
 *
 * OpenAPI spec version: 2.1.0
 * Contact: opensource@shinesolutions.com
 *
 * NOTE: This class is auto generated by the swagger code generator program.
 * https://github.com/swagger-api/swagger-codegen.git
 * Do not edit the class manually.
 *
 */


import ApiClient from '../ApiClient';
import KeystoreItems from './KeystoreItems';





/**
* The KeystoreInformations model module.
* @module model/KeystoreInformations
* @version 0.9.0
*/
export default class KeystoreInformations {
    /**
    * Constructs a new <code>KeystoreInformations</code>.
    * @alias module:model/KeystoreInformations
    * @class
    */

    constructor() {
        

        
        

        

        
    }

    /**
    * Constructs a <code>KeystoreInformations</code> from a plain JavaScript object, optionally creating a new instance.
    * Copies all relevant properties from <code>data</code> to <code>obj</code> if supplied or a new instance if not.
    * @param {Object} data The plain JavaScript object bearing properties of interest.
    * @param {module:model/KeystoreInformations} obj Optional instance to populate.
    * @return {module:model/KeystoreInformations} The populated <code>KeystoreInformations</code> instance.
    */
    static constructFromObject(data, obj) {
        if (data) {
            obj = obj || new KeystoreInformations();

            
            
            

            if (data.hasOwnProperty('aliases')) {
                obj['aliases'] = ApiClient.convertToType(data['aliases'], [KeystoreItems]);
            }
            if (data.hasOwnProperty('exists')) {
                obj['exists'] = ApiClient.convertToType(data['exists'], 'Boolean');
            }
        }
        return obj;
    }

    /**
    * @member {Array.<module:model/KeystoreItems>} aliases
    */
    aliases = undefined;
    /**
    * False if truststore don't exist
    * @member {Boolean} exists
    */
    exists = undefined;








}


