/*
 * Adobe Experience Manager (AEM) API
 * Swagger AEM is an OpenAPI specification for Adobe Experience Manager (AEM) API
 *
 * OpenAPI spec version: 3.2.0-pre.0
 * Contact: opensource@shinesolutions.com
 *
 * NOTE: This class is auto generated by the OAS code generator program.
 * https://github.com/OAS-api/OAS-codegen.git
 * Do not edit the class manually.
 */

/**
 * OASSamlConfigurationProperties
 */
public class OASSamlConfigurationProperties implements OAS.MappedProperties {
    /**
     * Get path
     * @return path
     */
    public OASSamlConfigurationPropertyItemsArr path { get; set; }

    /**
     * Get serviceRanking
     * @return serviceRanking
     */
    public OASSamlConfigurationPropertyItemsLon serviceRanking { get; set; }

    /**
     * Get idpUrl
     * @return idpUrl
     */
    public OASSamlConfigurationPropertyItemsStr idpUrl { get; set; }

    /**
     * Get idpCertAlias
     * @return idpCertAlias
     */
    public OASSamlConfigurationPropertyItemsStr idpCertAlias { get; set; }

    /**
     * Get idpHttpRedirect
     * @return idpHttpRedirect
     */
    public OASSamlConfigurationPropertyItemsBoo idpHttpRedirect { get; set; }

    /**
     * Get serviceProviderEntityId
     * @return serviceProviderEntityId
     */
    public OASSamlConfigurationPropertyItemsStr serviceProviderEntityId { get; set; }

    /**
     * Get assertionConsumerServiceURL
     * @return assertionConsumerServiceURL
     */
    public OASSamlConfigurationPropertyItemsStr assertionConsumerServiceURL { get; set; }

    /**
     * Get spPrivateKeyAlias
     * @return spPrivateKeyAlias
     */
    public OASSamlConfigurationPropertyItemsStr spPrivateKeyAlias { get; set; }

    /**
     * Get keyStorePassword
     * @return keyStorePassword
     */
    public OASSamlConfigurationPropertyItemsStr keyStorePassword { get; set; }

    /**
     * Get defaultRedirectUrl
     * @return defaultRedirectUrl
     */
    public OASSamlConfigurationPropertyItemsStr defaultRedirectUrl { get; set; }

    /**
     * Get userIDAttribute
     * @return userIDAttribute
     */
    public OASSamlConfigurationPropertyItemsStr userIDAttribute { get; set; }

    /**
     * Get useEncryption
     * @return useEncryption
     */
    public OASSamlConfigurationPropertyItemsBoo useEncryption { get; set; }

    /**
     * Get createUser
     * @return createUser
     */
    public OASSamlConfigurationPropertyItemsBoo createUser { get; set; }

    /**
     * Get addGroupMemberships
     * @return addGroupMemberships
     */
    public OASSamlConfigurationPropertyItemsBoo addGroupMemberships { get; set; }

    /**
     * Get groupMembershipAttribute
     * @return groupMembershipAttribute
     */
    public OASSamlConfigurationPropertyItemsStr groupMembershipAttribute { get; set; }

    /**
     * Get defaultGroups
     * @return defaultGroups
     */
    public OASSamlConfigurationPropertyItemsArr defaultGroups { get; set; }

    /**
     * Get nameIdFormat
     * @return nameIdFormat
     */
    public OASSamlConfigurationPropertyItemsStr nameIdFormat { get; set; }

    /**
     * Get synchronizeAttributes
     * @return synchronizeAttributes
     */
    public OASSamlConfigurationPropertyItemsArr synchronizeAttributes { get; set; }

    /**
     * Get handleLogout
     * @return handleLogout
     */
    public OASSamlConfigurationPropertyItemsBoo handleLogout { get; set; }

    /**
     * Get logoutUrl
     * @return logoutUrl
     */
    public OASSamlConfigurationPropertyItemsStr logoutUrl { get; set; }

    /**
     * Get clockTolerance
     * @return clockTolerance
     */
    public OASSamlConfigurationPropertyItemsLon clockTolerance { get; set; }

    /**
     * Get digestMethod
     * @return digestMethod
     */
    public OASSamlConfigurationPropertyItemsStr digestMethod { get; set; }

    /**
     * Get signatureMethod
     * @return signatureMethod
     */
    public OASSamlConfigurationPropertyItemsStr signatureMethod { get; set; }

    /**
     * Get userIntermediatePath
     * @return userIntermediatePath
     */
    public OASSamlConfigurationPropertyItemsStr userIntermediatePath { get; set; }

    private static final Map<String, String> propertyMappings = new Map<String, String>{
        'service.ranking' => 'serviceRanking'
    };

    public Map<String, String> getPropertyMappings() {
        return propertyMappings;
    }

    public static OASSamlConfigurationProperties getExample() {
        OASSamlConfigurationProperties samlConfigurationProperties = new OASSamlConfigurationProperties();
          samlConfigurationProperties.path = OASSamlConfigurationPropertyItemsArr.getExample();
          samlConfigurationProperties.serviceRanking = OASSamlConfigurationPropertyItemsLon.getExample();
          samlConfigurationProperties.idpUrl = OASSamlConfigurationPropertyItemsStr.getExample();
          samlConfigurationProperties.idpCertAlias = OASSamlConfigurationPropertyItemsStr.getExample();
          samlConfigurationProperties.idpHttpRedirect = OASSamlConfigurationPropertyItemsBoo.getExample();
          samlConfigurationProperties.serviceProviderEntityId = OASSamlConfigurationPropertyItemsStr.getExample();
          samlConfigurationProperties.assertionConsumerServiceURL = OASSamlConfigurationPropertyItemsStr.getExample();
          samlConfigurationProperties.spPrivateKeyAlias = OASSamlConfigurationPropertyItemsStr.getExample();
          samlConfigurationProperties.keyStorePassword = OASSamlConfigurationPropertyItemsStr.getExample();
          samlConfigurationProperties.defaultRedirectUrl = OASSamlConfigurationPropertyItemsStr.getExample();
          samlConfigurationProperties.userIDAttribute = OASSamlConfigurationPropertyItemsStr.getExample();
          samlConfigurationProperties.useEncryption = OASSamlConfigurationPropertyItemsBoo.getExample();
          samlConfigurationProperties.createUser = OASSamlConfigurationPropertyItemsBoo.getExample();
          samlConfigurationProperties.addGroupMemberships = OASSamlConfigurationPropertyItemsBoo.getExample();
          samlConfigurationProperties.groupMembershipAttribute = OASSamlConfigurationPropertyItemsStr.getExample();
          samlConfigurationProperties.defaultGroups = OASSamlConfigurationPropertyItemsArr.getExample();
          samlConfigurationProperties.nameIdFormat = OASSamlConfigurationPropertyItemsStr.getExample();
          samlConfigurationProperties.synchronizeAttributes = OASSamlConfigurationPropertyItemsArr.getExample();
          samlConfigurationProperties.handleLogout = OASSamlConfigurationPropertyItemsBoo.getExample();
          samlConfigurationProperties.logoutUrl = OASSamlConfigurationPropertyItemsStr.getExample();
          samlConfigurationProperties.clockTolerance = OASSamlConfigurationPropertyItemsLon.getExample();
          samlConfigurationProperties.digestMethod = OASSamlConfigurationPropertyItemsStr.getExample();
          samlConfigurationProperties.signatureMethod = OASSamlConfigurationPropertyItemsStr.getExample();
          samlConfigurationProperties.userIntermediatePath = OASSamlConfigurationPropertyItemsStr.getExample();
        return samlConfigurationProperties;
    }

    public Boolean equals(Object obj) {
        if (obj instanceof OASSamlConfigurationProperties) {           
            OASSamlConfigurationProperties samlConfigurationProperties = (OASSamlConfigurationProperties) obj;
            return this.path == samlConfigurationProperties.path
                && this.serviceRanking == samlConfigurationProperties.serviceRanking
                && this.idpUrl == samlConfigurationProperties.idpUrl
                && this.idpCertAlias == samlConfigurationProperties.idpCertAlias
                && this.idpHttpRedirect == samlConfigurationProperties.idpHttpRedirect
                && this.serviceProviderEntityId == samlConfigurationProperties.serviceProviderEntityId
                && this.assertionConsumerServiceURL == samlConfigurationProperties.assertionConsumerServiceURL
                && this.spPrivateKeyAlias == samlConfigurationProperties.spPrivateKeyAlias
                && this.keyStorePassword == samlConfigurationProperties.keyStorePassword
                && this.defaultRedirectUrl == samlConfigurationProperties.defaultRedirectUrl
                && this.userIDAttribute == samlConfigurationProperties.userIDAttribute
                && this.useEncryption == samlConfigurationProperties.useEncryption
                && this.createUser == samlConfigurationProperties.createUser
                && this.addGroupMemberships == samlConfigurationProperties.addGroupMemberships
                && this.groupMembershipAttribute == samlConfigurationProperties.groupMembershipAttribute
                && this.defaultGroups == samlConfigurationProperties.defaultGroups
                && this.nameIdFormat == samlConfigurationProperties.nameIdFormat
                && this.synchronizeAttributes == samlConfigurationProperties.synchronizeAttributes
                && this.handleLogout == samlConfigurationProperties.handleLogout
                && this.logoutUrl == samlConfigurationProperties.logoutUrl
                && this.clockTolerance == samlConfigurationProperties.clockTolerance
                && this.digestMethod == samlConfigurationProperties.digestMethod
                && this.signatureMethod == samlConfigurationProperties.signatureMethod
                && this.userIntermediatePath == samlConfigurationProperties.userIntermediatePath;
        }
        return false;
    }

    public Integer hashCode() {
        Integer hashCode = 43;
        hashCode = (17 * hashCode) + (path == null ? 0 : System.hashCode(path));
        hashCode = (17 * hashCode) + (serviceRanking == null ? 0 : System.hashCode(serviceRanking));
        hashCode = (17 * hashCode) + (idpUrl == null ? 0 : System.hashCode(idpUrl));
        hashCode = (17 * hashCode) + (idpCertAlias == null ? 0 : System.hashCode(idpCertAlias));
        hashCode = (17 * hashCode) + (idpHttpRedirect == null ? 0 : System.hashCode(idpHttpRedirect));
        hashCode = (17 * hashCode) + (serviceProviderEntityId == null ? 0 : System.hashCode(serviceProviderEntityId));
        hashCode = (17 * hashCode) + (assertionConsumerServiceURL == null ? 0 : System.hashCode(assertionConsumerServiceURL));
        hashCode = (17 * hashCode) + (spPrivateKeyAlias == null ? 0 : System.hashCode(spPrivateKeyAlias));
        hashCode = (17 * hashCode) + (keyStorePassword == null ? 0 : System.hashCode(keyStorePassword));
        hashCode = (17 * hashCode) + (defaultRedirectUrl == null ? 0 : System.hashCode(defaultRedirectUrl));
        hashCode = (17 * hashCode) + (userIDAttribute == null ? 0 : System.hashCode(userIDAttribute));
        hashCode = (17 * hashCode) + (useEncryption == null ? 0 : System.hashCode(useEncryption));
        hashCode = (17 * hashCode) + (createUser == null ? 0 : System.hashCode(createUser));
        hashCode = (17 * hashCode) + (addGroupMemberships == null ? 0 : System.hashCode(addGroupMemberships));
        hashCode = (17 * hashCode) + (groupMembershipAttribute == null ? 0 : System.hashCode(groupMembershipAttribute));
        hashCode = (17 * hashCode) + (defaultGroups == null ? 0 : System.hashCode(defaultGroups));
        hashCode = (17 * hashCode) + (nameIdFormat == null ? 0 : System.hashCode(nameIdFormat));
        hashCode = (17 * hashCode) + (synchronizeAttributes == null ? 0 : System.hashCode(synchronizeAttributes));
        hashCode = (17 * hashCode) + (handleLogout == null ? 0 : System.hashCode(handleLogout));
        hashCode = (17 * hashCode) + (logoutUrl == null ? 0 : System.hashCode(logoutUrl));
        hashCode = (17 * hashCode) + (clockTolerance == null ? 0 : System.hashCode(clockTolerance));
        hashCode = (17 * hashCode) + (digestMethod == null ? 0 : System.hashCode(digestMethod));
        hashCode = (17 * hashCode) + (signatureMethod == null ? 0 : System.hashCode(signatureMethod));
        hashCode = (17 * hashCode) + (userIntermediatePath == null ? 0 : System.hashCode(userIntermediatePath));
        return hashCode;
    }
}
