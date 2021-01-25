"""
    Adobe Experience Manager (AEM) API

    Swagger AEM is an OpenAPI specification for Adobe Experience Manager (AEM) API  # noqa: E501

    The version of the OpenAPI document: 3.5.0-pre.0
    Contact: opensource@shinesolutions.com
    Generated by: https://openapi-generator.tech
"""


import re  # noqa: F401
import sys  # noqa: F401

from swaggeraem.api_client import ApiClient, Endpoint
from swaggeraem.model_utils import (  # noqa: F401
    check_allowed_values,
    check_validations,
    date,
    datetime,
    file_type,
    none_type,
    validate_and_convert_types
)


class GraniteApi(object):
    """NOTE: This class is auto generated by OpenAPI Generator
    Ref: https://openapi-generator.tech

    Do not edit the class manually.
    """

    def __init__(self, api_client=None):
        if api_client is None:
            api_client = ApiClient()
        self.api_client = api_client

        def __ssl_setup(
            self,
            keystore_password,
            keystore_password_confirm,
            truststore_password,
            truststore_password_confirm,
            https_hostname,
            https_port,
            **kwargs
        ):
            """ssl_setup  # noqa: E501

            This method makes a synchronous HTTP request by default. To make an
            asynchronous HTTP request, please pass async_req=True

            >>> thread = api.ssl_setup(keystore_password, keystore_password_confirm, truststore_password, truststore_password_confirm, https_hostname, https_port, async_req=True)
            >>> result = thread.get()

            Args:
                keystore_password (str):
                keystore_password_confirm (str):
                truststore_password (str):
                truststore_password_confirm (str):
                https_hostname (str):
                https_port (str):

            Keyword Args:
                privatekey_file (file_type): [optional]
                certificate_file (file_type): [optional]
                _return_http_data_only (bool): response data without head status
                    code and headers. Default is True.
                _preload_content (bool): if False, the urllib3.HTTPResponse object
                    will be returned without reading/decoding response data.
                    Default is True.
                _request_timeout (float/tuple): timeout setting for this request. If one
                    number provided, it will be total request timeout. It can also
                    be a pair (tuple) of (connection, read) timeouts.
                    Default is None.
                _check_input_type (bool): specifies if type checking
                    should be done one the data sent to the server.
                    Default is True.
                _check_return_type (bool): specifies if type checking
                    should be done one the data received from the server.
                    Default is True.
                _host_index (int/None): specifies the index of the server
                    that we want to use.
                    Default is read from the configuration.
                async_req (bool): execute request asynchronously

            Returns:
                str
                    If the method is called asynchronously, returns the request
                    thread.
            """
            kwargs['async_req'] = kwargs.get(
                'async_req', False
            )
            kwargs['_return_http_data_only'] = kwargs.get(
                '_return_http_data_only', True
            )
            kwargs['_preload_content'] = kwargs.get(
                '_preload_content', True
            )
            kwargs['_request_timeout'] = kwargs.get(
                '_request_timeout', None
            )
            kwargs['_check_input_type'] = kwargs.get(
                '_check_input_type', True
            )
            kwargs['_check_return_type'] = kwargs.get(
                '_check_return_type', True
            )
            kwargs['_host_index'] = kwargs.get('_host_index')
            kwargs['keystore_password'] = \
                keystore_password
            kwargs['keystore_password_confirm'] = \
                keystore_password_confirm
            kwargs['truststore_password'] = \
                truststore_password
            kwargs['truststore_password_confirm'] = \
                truststore_password_confirm
            kwargs['https_hostname'] = \
                https_hostname
            kwargs['https_port'] = \
                https_port
            return self.call_with_http_info(**kwargs)

        self.ssl_setup = Endpoint(
            settings={
                'response_type': (str,),
                'auth': [
                    'aemAuth'
                ],
                'endpoint_path': '/libs/granite/security/post/sslSetup.html',
                'operation_id': 'ssl_setup',
                'http_method': 'POST',
                'servers': None,
            },
            params_map={
                'all': [
                    'keystore_password',
                    'keystore_password_confirm',
                    'truststore_password',
                    'truststore_password_confirm',
                    'https_hostname',
                    'https_port',
                    'privatekey_file',
                    'certificate_file',
                ],
                'required': [
                    'keystore_password',
                    'keystore_password_confirm',
                    'truststore_password',
                    'truststore_password_confirm',
                    'https_hostname',
                    'https_port',
                ],
                'nullable': [
                ],
                'enum': [
                ],
                'validation': [
                ]
            },
            root_map={
                'validations': {
                },
                'allowed_values': {
                },
                'openapi_types': {
                    'keystore_password':
                        (str,),
                    'keystore_password_confirm':
                        (str,),
                    'truststore_password':
                        (str,),
                    'truststore_password_confirm':
                        (str,),
                    'https_hostname':
                        (str,),
                    'https_port':
                        (str,),
                    'privatekey_file':
                        (file_type,),
                    'certificate_file':
                        (file_type,),
                },
                'attribute_map': {
                    'keystore_password': 'keystorePassword',
                    'keystore_password_confirm': 'keystorePasswordConfirm',
                    'truststore_password': 'truststorePassword',
                    'truststore_password_confirm': 'truststorePasswordConfirm',
                    'https_hostname': 'httpsHostname',
                    'https_port': 'httpsPort',
                    'privatekey_file': 'privatekeyFile',
                    'certificate_file': 'certificateFile',
                },
                'location_map': {
                    'keystore_password': 'query',
                    'keystore_password_confirm': 'query',
                    'truststore_password': 'query',
                    'truststore_password_confirm': 'query',
                    'https_hostname': 'query',
                    'https_port': 'query',
                    'privatekey_file': 'form',
                    'certificate_file': 'form',
                },
                'collection_format_map': {
                }
            },
            headers_map={
                'accept': [
                    'text/plain'
                ],
                'content_type': [
                    'multipart/form-data'
                ]
            },
            api_client=api_client,
            callable=__ssl_setup
        )