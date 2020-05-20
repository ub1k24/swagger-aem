--[[
  Adobe Experience Manager (AEM) API
 
  Swagger AEM is an OpenAPI specification for Adobe Experience Manager (AEM) API
 
  OpenAPI spec version: 3.2.0-pre.0
  Contact: opensource@shinesolutions.com
  Generated by: https://openapi-generator.tech
]]

-- truststore_info class
local truststore_info = {}
local truststore_info_mt = {
	__name = "truststore_info";
	__index = truststore_info;
}

local function cast_truststore_info(t)
	return setmetatable(t, truststore_info_mt)
end

local function new_truststore_info(aliases, exists)
	return cast_truststore_info({
		["aliases"] = aliases;
		["exists"] = exists;
	})
end

return {
	cast = cast_truststore_info;
	new = new_truststore_info;
}