# coding: utf-8

"""
    Adobe Experience Manager (AEM) API

    Swagger AEM is an OpenAPI specification for Adobe Experience Manager (AEM) API

    OpenAPI spec version: 2.2.0
    Contact: opensource@shinesolutions.com
    Generated by: https://github.com/swagger-api/swagger-codegen.git
"""


from __future__ import absolute_import

import sys
import os
import re

# python 2 and python 3 compatibility library
from six import iteritems

from ..api_client import ApiClient


class CrxApi(object):
    """
    NOTE: This class is auto generated by the swagger code generator program.
    Do not edit the class manually.
    Ref: https://github.com/swagger-api/swagger-codegen
    """

    def __init__(self, api_client=None):
        if api_client is None:
            api_client = ApiClient()
        self.api_client = api_client

    def get_crxde_status(self, **kwargs):
        """
        This method makes a synchronous HTTP request by default. To make an
        asynchronous HTTP request, please pass async=True
        >>> thread = api.get_crxde_status(async=True)
        >>> result = thread.get()

        :param async bool
        :return: str
                 If the method is called asynchronously,
                 returns the request thread.
        """
        kwargs['_return_http_data_only'] = True
        if kwargs.get('async'):
            return self.get_crxde_status_with_http_info(**kwargs)
        else:
            (data) = self.get_crxde_status_with_http_info(**kwargs)
            return data

    def get_crxde_status_with_http_info(self, **kwargs):
        """
        This method makes a synchronous HTTP request by default. To make an
        asynchronous HTTP request, please pass async=True
        >>> thread = api.get_crxde_status_with_http_info(async=True)
        >>> result = thread.get()

        :param async bool
        :return: str
                 If the method is called asynchronously,
                 returns the request thread.
        """

        all_params = []
        all_params.append('async')
        all_params.append('_return_http_data_only')
        all_params.append('_preload_content')
        all_params.append('_request_timeout')

        params = locals()
        for key, val in iteritems(params['kwargs']):
            if key not in all_params:
                raise TypeError(
                    "Got an unexpected keyword argument '%s'"
                    " to method get_crxde_status" % key
                )
            params[key] = val
        del params['kwargs']

        collection_formats = {}

        path_params = {}

        query_params = []

        header_params = {}

        form_params = []
        local_var_files = {}

        body_params = None
        # HTTP header `Accept`
        header_params['Accept'] = self.api_client.\
            select_header_accept(['plain/text'])

        # Authentication setting
        auth_settings = ['aemAuth']

        return self.api_client.call_api('/crx/server/crx.default/jcr:root/.1.json', 'GET',
                                        path_params,
                                        query_params,
                                        header_params,
                                        body=body_params,
                                        post_params=form_params,
                                        files=local_var_files,
                                        response_type='str',
                                        auth_settings=auth_settings,
                                        async_req=params.get('async'),
                                        _return_http_data_only=params.get('_return_http_data_only'),
                                        _preload_content=params.get('_preload_content', True),
                                        _request_timeout=params.get('_request_timeout'),
                                        collection_formats=collection_formats)

    def get_install_status(self, **kwargs):
        """
        This method makes a synchronous HTTP request by default. To make an
        asynchronous HTTP request, please pass async=True
        >>> thread = api.get_install_status(async=True)
        >>> result = thread.get()

        :param async bool
        :return: InstallStatus
                 If the method is called asynchronously,
                 returns the request thread.
        """
        kwargs['_return_http_data_only'] = True
        if kwargs.get('async'):
            return self.get_install_status_with_http_info(**kwargs)
        else:
            (data) = self.get_install_status_with_http_info(**kwargs)
            return data

    def get_install_status_with_http_info(self, **kwargs):
        """
        This method makes a synchronous HTTP request by default. To make an
        asynchronous HTTP request, please pass async=True
        >>> thread = api.get_install_status_with_http_info(async=True)
        >>> result = thread.get()

        :param async bool
        :return: InstallStatus
                 If the method is called asynchronously,
                 returns the request thread.
        """

        all_params = []
        all_params.append('async')
        all_params.append('_return_http_data_only')
        all_params.append('_preload_content')
        all_params.append('_request_timeout')

        params = locals()
        for key, val in iteritems(params['kwargs']):
            if key not in all_params:
                raise TypeError(
                    "Got an unexpected keyword argument '%s'"
                    " to method get_install_status" % key
                )
            params[key] = val
        del params['kwargs']

        collection_formats = {}

        path_params = {}

        query_params = []

        header_params = {}

        form_params = []
        local_var_files = {}

        body_params = None
        # HTTP header `Accept`
        header_params['Accept'] = self.api_client.\
            select_header_accept(['application/json'])

        # Authentication setting
        auth_settings = ['aemAuth']

        return self.api_client.call_api('/crx/packmgr/installstatus.jsp', 'GET',
                                        path_params,
                                        query_params,
                                        header_params,
                                        body=body_params,
                                        post_params=form_params,
                                        files=local_var_files,
                                        response_type='InstallStatus',
                                        auth_settings=auth_settings,
                                        async_req=params.get('async'),
                                        _return_http_data_only=params.get('_return_http_data_only'),
                                        _preload_content=params.get('_preload_content', True),
                                        _request_timeout=params.get('_request_timeout'),
                                        collection_formats=collection_formats)

    def post_package_service(self, cmd, **kwargs):
        """
        This method makes a synchronous HTTP request by default. To make an
        asynchronous HTTP request, please pass async=True
        >>> thread = api.post_package_service(cmd, async=True)
        >>> result = thread.get()

        :param async bool
        :param str cmd: (required)
        :return: str
                 If the method is called asynchronously,
                 returns the request thread.
        """
        kwargs['_return_http_data_only'] = True
        if kwargs.get('async'):
            return self.post_package_service_with_http_info(cmd, **kwargs)
        else:
            (data) = self.post_package_service_with_http_info(cmd, **kwargs)
            return data

    def post_package_service_with_http_info(self, cmd, **kwargs):
        """
        This method makes a synchronous HTTP request by default. To make an
        asynchronous HTTP request, please pass async=True
        >>> thread = api.post_package_service_with_http_info(cmd, async=True)
        >>> result = thread.get()

        :param async bool
        :param str cmd: (required)
        :return: str
                 If the method is called asynchronously,
                 returns the request thread.
        """

        all_params = ['cmd']
        all_params.append('async')
        all_params.append('_return_http_data_only')
        all_params.append('_preload_content')
        all_params.append('_request_timeout')

        params = locals()
        for key, val in iteritems(params['kwargs']):
            if key not in all_params:
                raise TypeError(
                    "Got an unexpected keyword argument '%s'"
                    " to method post_package_service" % key
                )
            params[key] = val
        del params['kwargs']
        # verify the required parameter 'cmd' is set
        if ('cmd' not in params) or (params['cmd'] is None):
            raise ValueError("Missing the required parameter `cmd` when calling `post_package_service`")


        collection_formats = {}

        path_params = {}

        query_params = []
        if 'cmd' in params:
            query_params.append(('cmd', params['cmd']))

        header_params = {}

        form_params = []
        local_var_files = {}

        body_params = None
        # HTTP header `Accept`
        header_params['Accept'] = self.api_client.\
            select_header_accept(['text/xml'])

        # Authentication setting
        auth_settings = ['aemAuth']

        return self.api_client.call_api('/crx/packmgr/service.jsp', 'POST',
                                        path_params,
                                        query_params,
                                        header_params,
                                        body=body_params,
                                        post_params=form_params,
                                        files=local_var_files,
                                        response_type='str',
                                        auth_settings=auth_settings,
                                        async_req=params.get('async'),
                                        _return_http_data_only=params.get('_return_http_data_only'),
                                        _preload_content=params.get('_preload_content', True),
                                        _request_timeout=params.get('_request_timeout'),
                                        collection_formats=collection_formats)

    def post_package_service_json(self, path, cmd, **kwargs):
        """
        This method makes a synchronous HTTP request by default. To make an
        asynchronous HTTP request, please pass async=True
        >>> thread = api.post_package_service_json(path, cmd, async=True)
        >>> result = thread.get()

        :param async bool
        :param str path: (required)
        :param str cmd: (required)
        :param str group_name:
        :param str package_name:
        :param str package_version:
        :param str charset_:
        :param bool force:
        :param bool recursive:
        :param file package:
        :return: str
                 If the method is called asynchronously,
                 returns the request thread.
        """
        kwargs['_return_http_data_only'] = True
        if kwargs.get('async'):
            return self.post_package_service_json_with_http_info(path, cmd, **kwargs)
        else:
            (data) = self.post_package_service_json_with_http_info(path, cmd, **kwargs)
            return data

    def post_package_service_json_with_http_info(self, path, cmd, **kwargs):
        """
        This method makes a synchronous HTTP request by default. To make an
        asynchronous HTTP request, please pass async=True
        >>> thread = api.post_package_service_json_with_http_info(path, cmd, async=True)
        >>> result = thread.get()

        :param async bool
        :param str path: (required)
        :param str cmd: (required)
        :param str group_name:
        :param str package_name:
        :param str package_version:
        :param str charset_:
        :param bool force:
        :param bool recursive:
        :param file package:
        :return: str
                 If the method is called asynchronously,
                 returns the request thread.
        """

        all_params = ['path', 'cmd', 'group_name', 'package_name', 'package_version', 'charset_', 'force', 'recursive', 'package']
        all_params.append('async')
        all_params.append('_return_http_data_only')
        all_params.append('_preload_content')
        all_params.append('_request_timeout')

        params = locals()
        for key, val in iteritems(params['kwargs']):
            if key not in all_params:
                raise TypeError(
                    "Got an unexpected keyword argument '%s'"
                    " to method post_package_service_json" % key
                )
            params[key] = val
        del params['kwargs']
        # verify the required parameter 'path' is set
        if ('path' not in params) or (params['path'] is None):
            raise ValueError("Missing the required parameter `path` when calling `post_package_service_json`")
        # verify the required parameter 'cmd' is set
        if ('cmd' not in params) or (params['cmd'] is None):
            raise ValueError("Missing the required parameter `cmd` when calling `post_package_service_json`")


        collection_formats = {}

        path_params = {}
        if 'path' in params:
            path_params['path'] = params['path']

        query_params = []
        if 'cmd' in params:
            query_params.append(('cmd', params['cmd']))
        if 'group_name' in params:
            query_params.append(('groupName', params['group_name']))
        if 'package_name' in params:
            query_params.append(('packageName', params['package_name']))
        if 'package_version' in params:
            query_params.append(('packageVersion', params['package_version']))
        if 'charset_' in params:
            query_params.append(('_charset_', params['charset_']))
        if 'force' in params:
            query_params.append(('force', params['force']))
        if 'recursive' in params:
            query_params.append(('recursive', params['recursive']))

        header_params = {}

        form_params = []
        local_var_files = {}
        if 'package' in params:
            local_var_files['package'] = params['package']

        body_params = None
        # HTTP header `Accept`
        header_params['Accept'] = self.api_client.\
            select_header_accept(['application/json'])

        # HTTP header `Content-Type`
        header_params['Content-Type'] = self.api_client.\
            select_header_content_type(['multipart/form-data'])

        # Authentication setting
        auth_settings = ['aemAuth']

        return self.api_client.call_api('/crx/packmgr/service/.json/{path}', 'POST',
                                        path_params,
                                        query_params,
                                        header_params,
                                        body=body_params,
                                        post_params=form_params,
                                        files=local_var_files,
                                        response_type='str',
                                        auth_settings=auth_settings,
                                        async_req=params.get('async'),
                                        _return_http_data_only=params.get('_return_http_data_only'),
                                        _preload_content=params.get('_preload_content', True),
                                        _request_timeout=params.get('_request_timeout'),
                                        collection_formats=collection_formats)

    def post_package_update(self, group_name, package_name, version, path, **kwargs):
        """
        This method makes a synchronous HTTP request by default. To make an
        asynchronous HTTP request, please pass async=True
        >>> thread = api.post_package_update(group_name, package_name, version, path, async=True)
        >>> result = thread.get()

        :param async bool
        :param str group_name: (required)
        :param str package_name: (required)
        :param str version: (required)
        :param str path: (required)
        :param str filter:
        :param str charset_:
        :return: str
                 If the method is called asynchronously,
                 returns the request thread.
        """
        kwargs['_return_http_data_only'] = True
        if kwargs.get('async'):
            return self.post_package_update_with_http_info(group_name, package_name, version, path, **kwargs)
        else:
            (data) = self.post_package_update_with_http_info(group_name, package_name, version, path, **kwargs)
            return data

    def post_package_update_with_http_info(self, group_name, package_name, version, path, **kwargs):
        """
        This method makes a synchronous HTTP request by default. To make an
        asynchronous HTTP request, please pass async=True
        >>> thread = api.post_package_update_with_http_info(group_name, package_name, version, path, async=True)
        >>> result = thread.get()

        :param async bool
        :param str group_name: (required)
        :param str package_name: (required)
        :param str version: (required)
        :param str path: (required)
        :param str filter:
        :param str charset_:
        :return: str
                 If the method is called asynchronously,
                 returns the request thread.
        """

        all_params = ['group_name', 'package_name', 'version', 'path', 'filter', 'charset_']
        all_params.append('async')
        all_params.append('_return_http_data_only')
        all_params.append('_preload_content')
        all_params.append('_request_timeout')

        params = locals()
        for key, val in iteritems(params['kwargs']):
            if key not in all_params:
                raise TypeError(
                    "Got an unexpected keyword argument '%s'"
                    " to method post_package_update" % key
                )
            params[key] = val
        del params['kwargs']
        # verify the required parameter 'group_name' is set
        if ('group_name' not in params) or (params['group_name'] is None):
            raise ValueError("Missing the required parameter `group_name` when calling `post_package_update`")
        # verify the required parameter 'package_name' is set
        if ('package_name' not in params) or (params['package_name'] is None):
            raise ValueError("Missing the required parameter `package_name` when calling `post_package_update`")
        # verify the required parameter 'version' is set
        if ('version' not in params) or (params['version'] is None):
            raise ValueError("Missing the required parameter `version` when calling `post_package_update`")
        # verify the required parameter 'path' is set
        if ('path' not in params) or (params['path'] is None):
            raise ValueError("Missing the required parameter `path` when calling `post_package_update`")


        collection_formats = {}

        path_params = {}

        query_params = []
        if 'group_name' in params:
            query_params.append(('groupName', params['group_name']))
        if 'package_name' in params:
            query_params.append(('packageName', params['package_name']))
        if 'version' in params:
            query_params.append(('version', params['version']))
        if 'path' in params:
            query_params.append(('path', params['path']))
        if 'filter' in params:
            query_params.append(('filter', params['filter']))
        if 'charset_' in params:
            query_params.append(('_charset_', params['charset_']))

        header_params = {}

        form_params = []
        local_var_files = {}

        body_params = None
        # HTTP header `Accept`
        header_params['Accept'] = self.api_client.\
            select_header_accept(['application/json'])

        # Authentication setting
        auth_settings = ['aemAuth']

        return self.api_client.call_api('/crx/packmgr/update.jsp', 'POST',
                                        path_params,
                                        query_params,
                                        header_params,
                                        body=body_params,
                                        post_params=form_params,
                                        files=local_var_files,
                                        response_type='str',
                                        auth_settings=auth_settings,
                                        async_req=params.get('async'),
                                        _return_http_data_only=params.get('_return_http_data_only'),
                                        _preload_content=params.get('_preload_content', True),
                                        _request_timeout=params.get('_request_timeout'),
                                        collection_formats=collection_formats)

    def post_set_password(self, old, plain, verify, **kwargs):
        """
        This method makes a synchronous HTTP request by default. To make an
        asynchronous HTTP request, please pass async=True
        >>> thread = api.post_set_password(old, plain, verify, async=True)
        >>> result = thread.get()

        :param async bool
        :param str old: (required)
        :param str plain: (required)
        :param str verify: (required)
        :return: str
                 If the method is called asynchronously,
                 returns the request thread.
        """
        kwargs['_return_http_data_only'] = True
        if kwargs.get('async'):
            return self.post_set_password_with_http_info(old, plain, verify, **kwargs)
        else:
            (data) = self.post_set_password_with_http_info(old, plain, verify, **kwargs)
            return data

    def post_set_password_with_http_info(self, old, plain, verify, **kwargs):
        """
        This method makes a synchronous HTTP request by default. To make an
        asynchronous HTTP request, please pass async=True
        >>> thread = api.post_set_password_with_http_info(old, plain, verify, async=True)
        >>> result = thread.get()

        :param async bool
        :param str old: (required)
        :param str plain: (required)
        :param str verify: (required)
        :return: str
                 If the method is called asynchronously,
                 returns the request thread.
        """

        all_params = ['old', 'plain', 'verify']
        all_params.append('async')
        all_params.append('_return_http_data_only')
        all_params.append('_preload_content')
        all_params.append('_request_timeout')

        params = locals()
        for key, val in iteritems(params['kwargs']):
            if key not in all_params:
                raise TypeError(
                    "Got an unexpected keyword argument '%s'"
                    " to method post_set_password" % key
                )
            params[key] = val
        del params['kwargs']
        # verify the required parameter 'old' is set
        if ('old' not in params) or (params['old'] is None):
            raise ValueError("Missing the required parameter `old` when calling `post_set_password`")
        # verify the required parameter 'plain' is set
        if ('plain' not in params) or (params['plain'] is None):
            raise ValueError("Missing the required parameter `plain` when calling `post_set_password`")
        # verify the required parameter 'verify' is set
        if ('verify' not in params) or (params['verify'] is None):
            raise ValueError("Missing the required parameter `verify` when calling `post_set_password`")


        collection_formats = {}

        path_params = {}

        query_params = []
        if 'old' in params:
            query_params.append(('old', params['old']))
        if 'plain' in params:
            query_params.append(('plain', params['plain']))
        if 'verify' in params:
            query_params.append(('verify', params['verify']))

        header_params = {}

        form_params = []
        local_var_files = {}

        body_params = None
        # HTTP header `Accept`
        header_params['Accept'] = self.api_client.\
            select_header_accept(['text/plain'])

        # Authentication setting
        auth_settings = ['aemAuth']

        return self.api_client.call_api('/crx/explorer/ui/setpassword.jsp', 'POST',
                                        path_params,
                                        query_params,
                                        header_params,
                                        body=body_params,
                                        post_params=form_params,
                                        files=local_var_files,
                                        response_type='str',
                                        auth_settings=auth_settings,
                                        async_req=params.get('async'),
                                        _return_http_data_only=params.get('_return_http_data_only'),
                                        _preload_content=params.get('_preload_content', True),
                                        _request_timeout=params.get('_request_timeout'),
                                        collection_formats=collection_formats)
