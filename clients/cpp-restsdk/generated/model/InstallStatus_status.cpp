/**
 * Adobe Experience Manager (AEM) API
 * Swagger AEM is an OpenAPI specification for Adobe Experience Manager (AEM) API
 *
 * OpenAPI spec version: 3.2.0-pre.0
 * Contact: opensource@shinesolutions.com
 *
 * NOTE: This class is auto generated by OpenAPI-Generator 3.2.1-SNAPSHOT.
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */



#include "InstallStatus_status.h"

namespace org {
namespace openapitools {
namespace client {
namespace model {

InstallStatus_status::InstallStatus_status()
{
    m_Finished = false;
    m_FinishedIsSet = false;
    m_ItemCount = 0;
    m_ItemCountIsSet = false;
}

InstallStatus_status::~InstallStatus_status()
{
}

void InstallStatus_status::validate()
{
    // TODO: implement validation
}

web::json::value InstallStatus_status::toJson() const
{
    web::json::value val = web::json::value::object();

    if(m_FinishedIsSet)
    {
        val[utility::conversions::to_string_t("finished")] = ModelBase::toJson(m_Finished);
    }
    if(m_ItemCountIsSet)
    {
        val[utility::conversions::to_string_t("itemCount")] = ModelBase::toJson(m_ItemCount);
    }

    return val;
}

void InstallStatus_status::fromJson(web::json::value& val)
{
    if(val.has_field(utility::conversions::to_string_t("finished")))
    {
        setFinished(ModelBase::boolFromJson(val[utility::conversions::to_string_t("finished")]));
    }
    if(val.has_field(utility::conversions::to_string_t("itemCount")))
    {
        setItemCount(ModelBase::int32_tFromJson(val[utility::conversions::to_string_t("itemCount")]));
    }
}

void InstallStatus_status::toMultipart(std::shared_ptr<MultipartFormData> multipart, const utility::string_t& prefix) const
{
    utility::string_t namePrefix = prefix;
    if(namePrefix.size() > 0 && namePrefix.substr(namePrefix.size() - 1) != utility::conversions::to_string_t("."))
    {
        namePrefix += utility::conversions::to_string_t(".");
    }

    if(m_FinishedIsSet)
    {
        multipart->add(ModelBase::toHttpContent(namePrefix + utility::conversions::to_string_t("finished"), m_Finished));
    }
    if(m_ItemCountIsSet)
    {
        multipart->add(ModelBase::toHttpContent(namePrefix + utility::conversions::to_string_t("itemCount"), m_ItemCount));
    }
}

void InstallStatus_status::fromMultiPart(std::shared_ptr<MultipartFormData> multipart, const utility::string_t& prefix)
{
    utility::string_t namePrefix = prefix;
    if(namePrefix.size() > 0 && namePrefix.substr(namePrefix.size() - 1) != utility::conversions::to_string_t("."))
    {
        namePrefix += utility::conversions::to_string_t(".");
    }

    if(multipart->hasContent(utility::conversions::to_string_t("finished")))
    {
        setFinished(ModelBase::boolFromHttpContent(multipart->getContent(utility::conversions::to_string_t("finished"))));
    }
    if(multipart->hasContent(utility::conversions::to_string_t("itemCount")))
    {
        setItemCount(ModelBase::int32_tFromHttpContent(multipart->getContent(utility::conversions::to_string_t("itemCount"))));
    }
}

bool InstallStatus_status::isFinished() const
{
    return m_Finished;
}


void InstallStatus_status::setFinished(bool value)
{
    m_Finished = value;
    m_FinishedIsSet = true;
}
bool InstallStatus_status::finishedIsSet() const
{
    return m_FinishedIsSet;
}

void InstallStatus_status::unsetFinished()
{
    m_FinishedIsSet = false;
}

int32_t InstallStatus_status::getItemCount() const
{
    return m_ItemCount;
}


void InstallStatus_status::setItemCount(int32_t value)
{
    m_ItemCount = value;
    m_ItemCountIsSet = true;
}
bool InstallStatus_status::itemCountIsSet() const
{
    return m_ItemCountIsSet;
}

void InstallStatus_status::unsetItemCount()
{
    m_ItemCountIsSet = false;
}

}
}
}
}

