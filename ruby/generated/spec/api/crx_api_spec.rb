=begin
#Adobe Experience Manager (AEM) API

#Swagger AEM is an OpenAPI specification for Adobe Experience Manager (AEM) API

OpenAPI spec version: 2.3.0
Contact: opensource@shinesolutions.com
Generated by: https://github.com/swagger-api/swagger-codegen.git
Swagger Codegen version: 2.3.0-SNAPSHOT

=end

require 'spec_helper'
require 'json'

# Unit tests for SwaggerAemClient::CrxApi
# Automatically generated by swagger-codegen (github.com/swagger-api/swagger-codegen)
# Please update as you see appropriate
describe 'CrxApi' do
  before do
    # run before each test
    @instance = SwaggerAemClient::CrxApi.new
  end

  after do
    # run after each test
  end

  describe 'test an instance of CrxApi' do
    it 'should create an instance of CrxApi' do
      expect(@instance).to be_instance_of(SwaggerAemClient::CrxApi)
    end
  end

  # unit tests for get_crxde_status
  # 
  # 
  # @param [Hash] opts the optional parameters
  # @return [String]
  describe 'get_crxde_status test' do
    it "should work" do
      # assertion here. ref: https://www.relishapp.com/rspec/rspec-expectations/docs/built-in-matchers
    end
  end

  # unit tests for get_install_status
  # 
  # 
  # @param [Hash] opts the optional parameters
  # @return [InstallStatus]
  describe 'get_install_status test' do
    it "should work" do
      # assertion here. ref: https://www.relishapp.com/rspec/rspec-expectations/docs/built-in-matchers
    end
  end

  # unit tests for post_package_service
  # 
  # 
  # @param cmd 
  # @param [Hash] opts the optional parameters
  # @return [String]
  describe 'post_package_service test' do
    it "should work" do
      # assertion here. ref: https://www.relishapp.com/rspec/rspec-expectations/docs/built-in-matchers
    end
  end

  # unit tests for post_package_service_json
  # 
  # 
  # @param path 
  # @param cmd 
  # @param [Hash] opts the optional parameters
  # @option opts [String] :group_name 
  # @option opts [String] :package_name 
  # @option opts [String] :package_version 
  # @option opts [String] :_charset_ 
  # @option opts [BOOLEAN] :force 
  # @option opts [BOOLEAN] :recursive 
  # @option opts [File] :package 
  # @return [String]
  describe 'post_package_service_json test' do
    it "should work" do
      # assertion here. ref: https://www.relishapp.com/rspec/rspec-expectations/docs/built-in-matchers
    end
  end

  # unit tests for post_package_update
  # 
  # 
  # @param group_name 
  # @param package_name 
  # @param version 
  # @param path 
  # @param [Hash] opts the optional parameters
  # @option opts [String] :filter 
  # @option opts [String] :_charset_ 
  # @return [String]
  describe 'post_package_update test' do
    it "should work" do
      # assertion here. ref: https://www.relishapp.com/rspec/rspec-expectations/docs/built-in-matchers
    end
  end

  # unit tests for post_set_password
  # 
  # 
  # @param old 
  # @param plain 
  # @param verify 
  # @param [Hash] opts the optional parameters
  # @return [String]
  describe 'post_set_password test' do
    it "should work" do
      # assertion here. ref: https://www.relishapp.com/rspec/rspec-expectations/docs/built-in-matchers
    end
  end

end
