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
from swaggeraem.model.install_status import InstallStatus


class CrxApi(object):
    """NOTE: This class is auto generated by OpenAPI Generator
    Ref: https://openapi-generator.tech

    Do not edit the class manually.
    """

    def __init__(self, api_client=None):
        if api_client is None:
            api_client = ApiClient()
        self.api_client = api_client

        def __get_crxde_status(
            self,
            **kwargs
        ):
            """get_crxde_status  # noqa: E501

            This method makes a synchronous HTTP request by default. To make an
            asynchronous HTTP request, please pass async_req=True

            >>> thread = api.get_crxde_status(async_req=True)
            >>> result = thread.get()


            Keyword Args:
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
            return self.call_with_http_info(**kwargs)

        self.get_crxde_status = Endpoint(
            settings={
                'response_type': (str,),
                'auth': [
                    'aemAuth'
                ],
                'endpoint_path': '/crx/server/crx.default/jcr:root/.1.json',
                'operation_id': 'get_crxde_status',
                'http_method': 'GET',
                'servers': None,
            },
            params_map={
                'all': [
                ],
                'required': [],
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
                },
                'attribute_map': {
                },
                'location_map': {
                },
                'collection_format_map': {
                }
            },
            headers_map={
                'accept': [
                    'plain/text'
                ],
                'content_type': [],
            },
            api_client=api_client,
            callable=__get_crxde_status
        )

        def __get_install_status(
            self,
            **kwargs
        ):
            """get_install_status  # noqa: E501

            This method makes a synchronous HTTP request by default. To make an
            asynchronous HTTP request, please pass async_req=True

            >>> thread = api.get_install_status(async_req=True)
            >>> result = thread.get()


            Keyword Args:
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
                InstallStatus
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
            return self.call_with_http_info(**kwargs)

        self.get_install_status = Endpoint(
            settings={
                'response_type': (InstallStatus,),
                'auth': [
                    'aemAuth'
                ],
                'endpoint_path': '/crx/packmgr/installstatus.jsp',
                'operation_id': 'get_install_status',
                'http_method': 'GET',
                'servers': None,
            },
            params_map={
                'all': [
                ],
                'required': [],
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
                },
                'attribute_map': {
                },
                'location_map': {
                },
                'collection_format_map': {
                }
            },
            headers_map={
                'accept': [
                    'application/json'
                ],
                'content_type': [],
            },
            api_client=api_client,
            callable=__get_install_status
        )

        def __get_package_manager_servlet(
            self,
            **kwargs
        ):
            """get_package_manager_servlet  # noqa: E501

            This method makes a synchronous HTTP request by default. To make an
            asynchronous HTTP request, please pass async_req=True

            >>> thread = api.get_package_manager_servlet(async_req=True)
            >>> result = thread.get()


            Keyword Args:
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
                None
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
            return self.call_with_http_info(**kwargs)

        self.get_package_manager_servlet = Endpoint(
            settings={
                'response_type': None,
                'auth': [
                    'aemAuth'
                ],
                'endpoint_path': '/crx/packmgr/service/script.html',
                'operation_id': 'get_package_manager_servlet',
                'http_method': 'GET',
                'servers': None,
            },
            params_map={
                'all': [
                ],
                'required': [],
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
                },
                'attribute_map': {
                },
                'location_map': {
                },
                'collection_format_map': {
                }
            },
            headers_map={
                'accept': [
                    'text/html'
                ],
                'content_type': [],
            },
            api_client=api_client,
            callable=__get_package_manager_servlet
        )

        def __post_package_service(
            self,
            cmd,
            **kwargs
        ):
            """post_package_service  # noqa: E501

            This method makes a synchronous HTTP request by default. To make an
            asynchronous HTTP request, please pass async_req=True

            >>> thread = api.post_package_service(cmd, async_req=True)
            >>> result = thread.get()

            Args:
                cmd (str):

            Keyword Args:
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
            kwargs['cmd'] = \
                cmd
            return self.call_with_http_info(**kwargs)

        self.post_package_service = Endpoint(
            settings={
                'response_type': (str,),
                'auth': [
                    'aemAuth'
                ],
                'endpoint_path': '/crx/packmgr/service.jsp',
                'operation_id': 'post_package_service',
                'http_method': 'POST',
                'servers': None,
            },
            params_map={
                'all': [
                    'cmd',
                ],
                'required': [
                    'cmd',
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
                    'cmd':
                        (str,),
                },
                'attribute_map': {
                    'cmd': 'cmd',
                },
                'location_map': {
                    'cmd': 'query',
                },
                'collection_format_map': {
                }
            },
            headers_map={
                'accept': [
                    'text/xml'
                ],
                'content_type': [],
            },
            api_client=api_client,
            callable=__post_package_service
        )

        def __post_package_service_json(
            self,
            path,
            cmd,
            **kwargs
        ):
            """post_package_service_json  # noqa: E501

            This method makes a synchronous HTTP request by default. To make an
            asynchronous HTTP request, please pass async_req=True

            >>> thread = api.post_package_service_json(path, cmd, async_req=True)
            >>> result = thread.get()

            Args:
                path (str):
                cmd (str):

            Keyword Args:
                group_name (str): [optional]
                package_name (str): [optional]
                package_version (str): [optional]
                charset_ (str): [optional]
                force (bool): [optional]
                recursive (bool): [optional]
                package (file_type): [optional]
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
            kwargs['path'] = \
                path
            kwargs['cmd'] = \
                cmd
            return self.call_with_http_info(**kwargs)

        self.post_package_service_json = Endpoint(
            settings={
                'response_type': (str,),
                'auth': [
                    'aemAuth'
                ],
                'endpoint_path': '/crx/packmgr/service/.json/{path}',
                'operation_id': 'post_package_service_json',
                'http_method': 'POST',
                'servers': None,
            },
            params_map={
                'all': [
                    'path',
                    'cmd',
                    'group_name',
                    'package_name',
                    'package_version',
                    'charset_',
                    'force',
                    'recursive',
                    'package',
                ],
                'required': [
                    'path',
                    'cmd',
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
                    'path':
                        (str,),
                    'cmd':
                        (str,),
                    'group_name':
                        (str,),
                    'package_name':
                        (str,),
                    'package_version':
                        (str,),
                    'charset_':
                        (str,),
                    'force':
                        (bool,),
                    'recursive':
                        (bool,),
                    'package':
                        (file_type,),
                },
                'attribute_map': {
                    'path': 'path',
                    'cmd': 'cmd',
                    'group_name': 'groupName',
                    'package_name': 'packageName',
                    'package_version': 'packageVersion',
                    'charset_': '_charset_',
                    'force': 'force',
                    'recursive': 'recursive',
                    'package': 'package',
                },
                'location_map': {
                    'path': 'path',
                    'cmd': 'query',
                    'group_name': 'query',
                    'package_name': 'query',
                    'package_version': 'query',
                    'charset_': 'query',
                    'force': 'query',
                    'recursive': 'query',
                    'package': 'form',
                },
                'collection_format_map': {
                }
            },
            headers_map={
                'accept': [
                    'application/json'
                ],
                'content_type': [
                    'multipart/form-data'
                ]
            },
            api_client=api_client,
            callable=__post_package_service_json
        )

        def __post_package_update(
            self,
            group_name,
            package_name,
            version,
            path,
            **kwargs
        ):
            """post_package_update  # noqa: E501

            This method makes a synchronous HTTP request by default. To make an
            asynchronous HTTP request, please pass async_req=True

            >>> thread = api.post_package_update(group_name, package_name, version, path, async_req=True)
            >>> result = thread.get()

            Args:
                group_name (str):
                package_name (str):
                version (str):
                path (str):

            Keyword Args:
                filter (str): [optional]
                charset_ (str): [optional]
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
            kwargs['group_name'] = \
                group_name
            kwargs['package_name'] = \
                package_name
            kwargs['version'] = \
                version
            kwargs['path'] = \
                path
            return self.call_with_http_info(**kwargs)

        self.post_package_update = Endpoint(
            settings={
                'response_type': (str,),
                'auth': [
                    'aemAuth'
                ],
                'endpoint_path': '/crx/packmgr/update.jsp',
                'operation_id': 'post_package_update',
                'http_method': 'POST',
                'servers': None,
            },
            params_map={
                'all': [
                    'group_name',
                    'package_name',
                    'version',
                    'path',
                    'filter',
                    'charset_',
                ],
                'required': [
                    'group_name',
                    'package_name',
                    'version',
                    'path',
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
                    'group_name':
                        (str,),
                    'package_name':
                        (str,),
                    'version':
                        (str,),
                    'path':
                        (str,),
                    'filter':
                        (str,),
                    'charset_':
                        (str,),
                },
                'attribute_map': {
                    'group_name': 'groupName',
                    'package_name': 'packageName',
                    'version': 'version',
                    'path': 'path',
                    'filter': 'filter',
                    'charset_': '_charset_',
                },
                'location_map': {
                    'group_name': 'query',
                    'package_name': 'query',
                    'version': 'query',
                    'path': 'query',
                    'filter': 'query',
                    'charset_': 'query',
                },
                'collection_format_map': {
                }
            },
            headers_map={
                'accept': [
                    'application/json'
                ],
                'content_type': [],
            },
            api_client=api_client,
            callable=__post_package_update
        )

        def __post_set_password(
            self,
            old,
            plain,
            verify,
            **kwargs
        ):
            """post_set_password  # noqa: E501

            This method makes a synchronous HTTP request by default. To make an
            asynchronous HTTP request, please pass async_req=True

            >>> thread = api.post_set_password(old, plain, verify, async_req=True)
            >>> result = thread.get()

            Args:
                old (str):
                plain (str):
                verify (str):

            Keyword Args:
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
            kwargs['old'] = \
                old
            kwargs['plain'] = \
                plain
            kwargs['verify'] = \
                verify
            return self.call_with_http_info(**kwargs)

        self.post_set_password = Endpoint(
            settings={
                'response_type': (str,),
                'auth': [
                    'aemAuth'
                ],
                'endpoint_path': '/crx/explorer/ui/setpassword.jsp',
                'operation_id': 'post_set_password',
                'http_method': 'POST',
                'servers': None,
            },
            params_map={
                'all': [
                    'old',
                    'plain',
                    'verify',
                ],
                'required': [
                    'old',
                    'plain',
                    'verify',
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
                    'old':
                        (str,),
                    'plain':
                        (str,),
                    'verify':
                        (str,),
                },
                'attribute_map': {
                    'old': 'old',
                    'plain': 'plain',
                    'verify': 'verify',
                },
                'location_map': {
                    'old': 'query',
                    'plain': 'query',
                    'verify': 'query',
                },
                'collection_format_map': {
                }
            },
            headers_map={
                'accept': [
                    'text/plain'
                ],
                'content_type': [],
            },
            api_client=api_client,
            callable=__post_set_password
        )
