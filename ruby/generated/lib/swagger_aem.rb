=begin
#Adobe Experience Manager (AEM) API

#Swagger AEM is an OpenAPI specification for Adobe Experience Manager (AEM) API

OpenAPI spec version: 1.1.8
Contact: opensource@shinesolutions.com
Generated by: https://github.com/swagger-api/swagger-codegen.git

=end

# Common files
require 'swagger_aem/api_client'
require 'swagger_aem/api_error'
require 'swagger_aem/version'
require 'swagger_aem/configuration'

# Models

# APIs
require 'swagger_aem/api/console_api'
require 'swagger_aem/api/cq_api'
require 'swagger_aem/api/crx_api'
require 'swagger_aem/api/sling_api'

module SwaggerAemClient
  class << self
    # Customize default settings for the SDK using block.
    #   SwaggerAemClient.configure do |config|
    #     config.username = "xxx"
    #     config.password = "xxx"
    #   end
    # If no block given, return the default Configuration object.
    def configure
      if block_given?
        yield(Configuration.default)
      else
        Configuration.default
      end
    end
  end
end
