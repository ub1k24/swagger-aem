/**
* Adobe Experience Manager (AEM) API
* Swagger AEM is an OpenAPI specification for Adobe Experience Manager (AEM) API
*
* OpenAPI spec version: 3.2.0-pre.0
* Contact: opensource@shinesolutions.com
*
* NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
* https://openapi-generator.tech
* Do not edit the class manually.
*/


#include "SamlConfigurationPropertyItemsString.h"

namespace org {
namespace openapitools {
namespace server {
namespace model {

SamlConfigurationPropertyItemsString::SamlConfigurationPropertyItemsString()
{
    m_Name = "";
    m_NameIsSet = false;
    m_Optional = false;
    m_OptionalIsSet = false;
    m_Is_set = false;
    m_Is_setIsSet = false;
    m_Type = 0;
    m_TypeIsSet = false;
    m_Value = "";
    m_ValueIsSet = false;
    m_Description = "";
    m_DescriptionIsSet = false;
    
}

SamlConfigurationPropertyItemsString::~SamlConfigurationPropertyItemsString()
{
}

void SamlConfigurationPropertyItemsString::validate()
{
    // TODO: implement validation
}

nlohmann::json SamlConfigurationPropertyItemsString::toJson() const
{
    nlohmann::json val = nlohmann::json::object();

    if(m_NameIsSet)
    {
        val["name"] = ModelBase::toJson(m_Name);
    }
    if(m_OptionalIsSet)
    {
        val["optional"] = m_Optional;
    }
    if(m_Is_setIsSet)
    {
        val["is_set"] = m_Is_set;
    }
    if(m_TypeIsSet)
    {
        val["type"] = m_Type;
    }
    if(m_ValueIsSet)
    {
        val["value"] = ModelBase::toJson(m_Value);
    }
    if(m_DescriptionIsSet)
    {
        val["description"] = ModelBase::toJson(m_Description);
    }
    

    return val;
}

void SamlConfigurationPropertyItemsString::fromJson(nlohmann::json& val)
{
    if(val.find("name") != val.end())
    {
        setName(val.at("name"));
    }
    if(val.find("optional") != val.end())
    {
        setOptional(val.at("optional"));
    }
    if(val.find("is_set") != val.end())
    {
        setIsSet(val.at("is_set"));
    }
    if(val.find("type") != val.end())
    {
        setType(val.at("type"));
    }
    if(val.find("value") != val.end())
    {
        setValue(val.at("value"));
    }
    if(val.find("description") != val.end())
    {
        setDescription(val.at("description"));
    }
    
}


std::string SamlConfigurationPropertyItemsString::getName() const
{
    return m_Name;
}
void SamlConfigurationPropertyItemsString::setName(std::string const& value)
{
    m_Name = value;
    m_NameIsSet = true;
}
bool SamlConfigurationPropertyItemsString::nameIsSet() const
{
    return m_NameIsSet;
}
void SamlConfigurationPropertyItemsString::unsetName()
{
    m_NameIsSet = false;
}
bool SamlConfigurationPropertyItemsString::isOptional() const
{
    return m_Optional;
}
void SamlConfigurationPropertyItemsString::setOptional(bool const value)
{
    m_Optional = value;
    m_OptionalIsSet = true;
}
bool SamlConfigurationPropertyItemsString::optionalIsSet() const
{
    return m_OptionalIsSet;
}
void SamlConfigurationPropertyItemsString::unsetOptional()
{
    m_OptionalIsSet = false;
}
bool SamlConfigurationPropertyItemsString::isIsSet() const
{
    return m_Is_set;
}
void SamlConfigurationPropertyItemsString::setIsSet(bool const value)
{
    m_Is_set = value;
    m_Is_setIsSet = true;
}
bool SamlConfigurationPropertyItemsString::isSetIsSet() const
{
    return m_Is_setIsSet;
}
void SamlConfigurationPropertyItemsString::unsetIs_set()
{
    m_Is_setIsSet = false;
}
int32_t SamlConfigurationPropertyItemsString::getType() const
{
    return m_Type;
}
void SamlConfigurationPropertyItemsString::setType(int32_t const value)
{
    m_Type = value;
    m_TypeIsSet = true;
}
bool SamlConfigurationPropertyItemsString::typeIsSet() const
{
    return m_TypeIsSet;
}
void SamlConfigurationPropertyItemsString::unsetType()
{
    m_TypeIsSet = false;
}
std::string SamlConfigurationPropertyItemsString::getValue() const
{
    return m_Value;
}
void SamlConfigurationPropertyItemsString::setValue(std::string const& value)
{
    m_Value = value;
    m_ValueIsSet = true;
}
bool SamlConfigurationPropertyItemsString::valueIsSet() const
{
    return m_ValueIsSet;
}
void SamlConfigurationPropertyItemsString::unsetValue()
{
    m_ValueIsSet = false;
}
std::string SamlConfigurationPropertyItemsString::getDescription() const
{
    return m_Description;
}
void SamlConfigurationPropertyItemsString::setDescription(std::string const& value)
{
    m_Description = value;
    m_DescriptionIsSet = true;
}
bool SamlConfigurationPropertyItemsString::descriptionIsSet() const
{
    return m_DescriptionIsSet;
}
void SamlConfigurationPropertyItemsString::unsetDescription()
{
    m_DescriptionIsSet = false;
}

}
}
}
}

