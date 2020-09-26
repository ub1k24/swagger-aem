# coding: utf-8

"""
    Adobe Experience Manager (AEM) API

    Swagger AEM is an OpenAPI specification for Adobe Experience Manager (AEM) API  # noqa: E501

    The version of the OpenAPI document: 3.4.0-pre.0
    Contact: opensource@shinesolutions.com
    Generated by: https://openapi-generator.tech
"""


import pprint
import re  # noqa: F401

import six

from swaggeraem.configuration import Configuration


class TruststoreInfo(object):
    """NOTE: This class is auto generated by OpenAPI Generator.
    Ref: https://openapi-generator.tech

    Do not edit the class manually.
    """

    """
    Attributes:
      openapi_types (dict): The key is attribute name
                            and the value is attribute type.
      attribute_map (dict): The key is attribute name
                            and the value is json key in definition.
    """
    openapi_types = {
        'aliases': 'list[TruststoreItems]',
        'exists': 'bool'
    }

    attribute_map = {
        'aliases': 'aliases',
        'exists': 'exists'
    }

    def __init__(self, aliases=None, exists=None, local_vars_configuration=None):  # noqa: E501
        """TruststoreInfo - a model defined in OpenAPI"""  # noqa: E501
        if local_vars_configuration is None:
            local_vars_configuration = Configuration()
        self.local_vars_configuration = local_vars_configuration

        self._aliases = None
        self._exists = None
        self.discriminator = None

        if aliases is not None:
            self.aliases = aliases
        if exists is not None:
            self.exists = exists

    @property
    def aliases(self):
        """Gets the aliases of this TruststoreInfo.  # noqa: E501


        :return: The aliases of this TruststoreInfo.  # noqa: E501
        :rtype: list[TruststoreItems]
        """
        return self._aliases

    @aliases.setter
    def aliases(self, aliases):
        """Sets the aliases of this TruststoreInfo.


        :param aliases: The aliases of this TruststoreInfo.  # noqa: E501
        :type: list[TruststoreItems]
        """

        self._aliases = aliases

    @property
    def exists(self):
        """Gets the exists of this TruststoreInfo.  # noqa: E501

        False if truststore don't exist  # noqa: E501

        :return: The exists of this TruststoreInfo.  # noqa: E501
        :rtype: bool
        """
        return self._exists

    @exists.setter
    def exists(self, exists):
        """Sets the exists of this TruststoreInfo.

        False if truststore don't exist  # noqa: E501

        :param exists: The exists of this TruststoreInfo.  # noqa: E501
        :type: bool
        """

        self._exists = exists

    def to_dict(self):
        """Returns the model properties as a dict"""
        result = {}

        for attr, _ in six.iteritems(self.openapi_types):
            value = getattr(self, attr)
            if isinstance(value, list):
                result[attr] = list(map(
                    lambda x: x.to_dict() if hasattr(x, "to_dict") else x,
                    value
                ))
            elif hasattr(value, "to_dict"):
                result[attr] = value.to_dict()
            elif isinstance(value, dict):
                result[attr] = dict(map(
                    lambda item: (item[0], item[1].to_dict())
                    if hasattr(item[1], "to_dict") else item,
                    value.items()
                ))
            else:
                result[attr] = value

        return result

    def to_str(self):
        """Returns the string representation of the model"""
        return pprint.pformat(self.to_dict())

    def __repr__(self):
        """For `print` and `pprint`"""
        return self.to_str()

    def __eq__(self, other):
        """Returns true if both objects are equal"""
        if not isinstance(other, TruststoreInfo):
            return False

        return self.to_dict() == other.to_dict()

    def __ne__(self, other):
        """Returns true if both objects are not equal"""
        if not isinstance(other, TruststoreInfo):
            return True

        return self.to_dict() != other.to_dict()
