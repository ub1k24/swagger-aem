/*
 * Adobe Experience Manager (AEM) API
 * Swagger AEM is an OpenAPI specification for Adobe Experience Manager (AEM) API
 *
 * OpenAPI spec version: 1.1
 * Contact: opensource@shinesolutions.com
 *
 * NOTE: This class is auto generated by the swagger code generator program.
 * https://github.com/swagger-api/swagger-codegen.git
 * Do not edit the class manually.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */


package com.shinesolutions.swaggeraem4j.api;

import com.shinesolutions.swaggeraem4j.ApiCallback;
import com.shinesolutions.swaggeraem4j.ApiClient;
import com.shinesolutions.swaggeraem4j.ApiException;
import com.shinesolutions.swaggeraem4j.ApiResponse;
import com.shinesolutions.swaggeraem4j.Configuration;
import com.shinesolutions.swaggeraem4j.Pair;
import com.shinesolutions.swaggeraem4j.ProgressRequestBody;
import com.shinesolutions.swaggeraem4j.ProgressResponseBody;

import com.google.gson.reflect.TypeToken;

import java.io.IOException;

import java.io.File;

import java.lang.reflect.Type;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class CrxApi {
    private ApiClient apiClient;

    public CrxApi() {
        this(Configuration.getDefaultApiClient());
    }

    public CrxApi(ApiClient apiClient) {
        this.apiClient = apiClient;
    }

    public ApiClient getApiClient() {
        return apiClient;
    }

    public void setApiClient(ApiClient apiClient) {
        this.apiClient = apiClient;
    }

    /* Build call for postPackageService */
    private com.squareup.okhttp.Call postPackageServiceCall(String cmd, final ProgressResponseBody.ProgressListener progressListener, final ProgressRequestBody.ProgressRequestListener progressRequestListener) throws ApiException {
        Object localVarPostBody = null;
        
        // verify the required parameter 'cmd' is set
        if (cmd == null) {
            throw new ApiException("Missing the required parameter 'cmd' when calling postPackageService(Async)");
        }
        

        // create path and map variables
        String localVarPath = "/crx/packmgr/service.jsp".replaceAll("\\{format\\}","json");

        List<Pair> localVarQueryParams = new ArrayList<Pair>();
        if (cmd != null)
        localVarQueryParams.addAll(apiClient.parameterToPairs("", "cmd", cmd));

        Map<String, String> localVarHeaderParams = new HashMap<String, String>();

        Map<String, Object> localVarFormParams = new HashMap<String, Object>();

        final String[] localVarAccepts = {
            "text/xml"
        };
        final String localVarAccept = apiClient.selectHeaderAccept(localVarAccepts);
        if (localVarAccept != null) localVarHeaderParams.put("Accept", localVarAccept);

        final String[] localVarContentTypes = {
            
        };
        final String localVarContentType = apiClient.selectHeaderContentType(localVarContentTypes);
        localVarHeaderParams.put("Content-Type", localVarContentType);

        if(progressListener != null) {
            apiClient.getHttpClient().networkInterceptors().add(new com.squareup.okhttp.Interceptor() {
                @Override
                public com.squareup.okhttp.Response intercept(com.squareup.okhttp.Interceptor.Chain chain) throws IOException {
                    com.squareup.okhttp.Response originalResponse = chain.proceed(chain.request());
                    return originalResponse.newBuilder()
                    .body(new ProgressResponseBody(originalResponse.body(), progressListener))
                    .build();
                }
            });
        }

        String[] localVarAuthNames = new String[] { "aemAuth" };
        return apiClient.buildCall(localVarPath, "POST", localVarQueryParams, localVarPostBody, localVarHeaderParams, localVarFormParams, localVarAuthNames, progressRequestListener);
    }

    /**
     * 
     * 
     * @param cmd  (required)
     * @return String
     * @throws ApiException If fail to call the API, e.g. server error or cannot deserialize the response body
     */
    public String postPackageService(String cmd) throws ApiException {
        ApiResponse<String> resp = postPackageServiceWithHttpInfo(cmd);
        return resp.getData();
    }

    /**
     * 
     * 
     * @param cmd  (required)
     * @return ApiResponse&lt;String&gt;
     * @throws ApiException If fail to call the API, e.g. server error or cannot deserialize the response body
     */
    public ApiResponse<String> postPackageServiceWithHttpInfo(String cmd) throws ApiException {
        com.squareup.okhttp.Call call = postPackageServiceCall(cmd, null, null);
        Type localVarReturnType = new TypeToken<String>(){}.getType();
        return apiClient.execute(call, localVarReturnType);
    }

    /**
     *  (asynchronously)
     * 
     * @param cmd  (required)
     * @param callback The callback to be executed when the API call finishes
     * @return The request call
     * @throws ApiException If fail to process the API call, e.g. serializing the request body object
     */
    public com.squareup.okhttp.Call postPackageServiceAsync(String cmd, final ApiCallback<String> callback) throws ApiException {

        ProgressResponseBody.ProgressListener progressListener = null;
        ProgressRequestBody.ProgressRequestListener progressRequestListener = null;

        if (callback != null) {
            progressListener = new ProgressResponseBody.ProgressListener() {
                @Override
                public void update(long bytesRead, long contentLength, boolean done) {
                    callback.onDownloadProgress(bytesRead, contentLength, done);
                }
            };

            progressRequestListener = new ProgressRequestBody.ProgressRequestListener() {
                @Override
                public void onRequestProgress(long bytesWritten, long contentLength, boolean done) {
                    callback.onUploadProgress(bytesWritten, contentLength, done);
                }
            };
        }

        com.squareup.okhttp.Call call = postPackageServiceCall(cmd, progressListener, progressRequestListener);
        Type localVarReturnType = new TypeToken<String>(){}.getType();
        apiClient.executeAsync(call, localVarReturnType, callback);
        return call;
    }
    /* Build call for postPackageServiceJson */
    private com.squareup.okhttp.Call postPackageServiceJsonCall(String path, String cmd, String groupName, String packageName, String packageVersion, String charset_, Boolean force, File _package, final ProgressResponseBody.ProgressListener progressListener, final ProgressRequestBody.ProgressRequestListener progressRequestListener) throws ApiException {
        Object localVarPostBody = null;
        
        // verify the required parameter 'path' is set
        if (path == null) {
            throw new ApiException("Missing the required parameter 'path' when calling postPackageServiceJson(Async)");
        }
        
        // verify the required parameter 'cmd' is set
        if (cmd == null) {
            throw new ApiException("Missing the required parameter 'cmd' when calling postPackageServiceJson(Async)");
        }
        

        // create path and map variables
        String localVarPath = "/crx/packmgr/service/.json/{path}".replaceAll("\\{format\\}","json")
        .replaceAll("\\{" + "path" + "\\}", apiClient.escapeString(path.toString()));

        List<Pair> localVarQueryParams = new ArrayList<Pair>();
        if (cmd != null)
        localVarQueryParams.addAll(apiClient.parameterToPairs("", "cmd", cmd));
        if (groupName != null)
        localVarQueryParams.addAll(apiClient.parameterToPairs("", "groupName", groupName));
        if (packageName != null)
        localVarQueryParams.addAll(apiClient.parameterToPairs("", "packageName", packageName));
        if (packageVersion != null)
        localVarQueryParams.addAll(apiClient.parameterToPairs("", "packageVersion", packageVersion));
        if (charset_ != null)
        localVarQueryParams.addAll(apiClient.parameterToPairs("", "_charset_", charset_));
        if (force != null)
        localVarQueryParams.addAll(apiClient.parameterToPairs("", "force", force));

        Map<String, String> localVarHeaderParams = new HashMap<String, String>();

        Map<String, Object> localVarFormParams = new HashMap<String, Object>();
        if (_package != null)
        localVarFormParams.put("package", _package);

        final String[] localVarAccepts = {
            "application/json"
        };
        final String localVarAccept = apiClient.selectHeaderAccept(localVarAccepts);
        if (localVarAccept != null) localVarHeaderParams.put("Accept", localVarAccept);

        final String[] localVarContentTypes = {
            "multipart/form-data"
        };
        final String localVarContentType = apiClient.selectHeaderContentType(localVarContentTypes);
        localVarHeaderParams.put("Content-Type", localVarContentType);

        if(progressListener != null) {
            apiClient.getHttpClient().networkInterceptors().add(new com.squareup.okhttp.Interceptor() {
                @Override
                public com.squareup.okhttp.Response intercept(com.squareup.okhttp.Interceptor.Chain chain) throws IOException {
                    com.squareup.okhttp.Response originalResponse = chain.proceed(chain.request());
                    return originalResponse.newBuilder()
                    .body(new ProgressResponseBody(originalResponse.body(), progressListener))
                    .build();
                }
            });
        }

        String[] localVarAuthNames = new String[] { "aemAuth" };
        return apiClient.buildCall(localVarPath, "POST", localVarQueryParams, localVarPostBody, localVarHeaderParams, localVarFormParams, localVarAuthNames, progressRequestListener);
    }

    /**
     * 
     * 
     * @param path  (required)
     * @param cmd  (required)
     * @param groupName  (optional)
     * @param packageName  (optional)
     * @param packageVersion  (optional)
     * @param charset_  (optional)
     * @param force  (optional)
     * @param _package  (optional)
     * @return String
     * @throws ApiException If fail to call the API, e.g. server error or cannot deserialize the response body
     */
    public String postPackageServiceJson(String path, String cmd, String groupName, String packageName, String packageVersion, String charset_, Boolean force, File _package) throws ApiException {
        ApiResponse<String> resp = postPackageServiceJsonWithHttpInfo(path, cmd, groupName, packageName, packageVersion, charset_, force, _package);
        return resp.getData();
    }

    /**
     * 
     * 
     * @param path  (required)
     * @param cmd  (required)
     * @param groupName  (optional)
     * @param packageName  (optional)
     * @param packageVersion  (optional)
     * @param charset_  (optional)
     * @param force  (optional)
     * @param _package  (optional)
     * @return ApiResponse&lt;String&gt;
     * @throws ApiException If fail to call the API, e.g. server error or cannot deserialize the response body
     */
    public ApiResponse<String> postPackageServiceJsonWithHttpInfo(String path, String cmd, String groupName, String packageName, String packageVersion, String charset_, Boolean force, File _package) throws ApiException {
        com.squareup.okhttp.Call call = postPackageServiceJsonCall(path, cmd, groupName, packageName, packageVersion, charset_, force, _package, null, null);
        Type localVarReturnType = new TypeToken<String>(){}.getType();
        return apiClient.execute(call, localVarReturnType);
    }

    /**
     *  (asynchronously)
     * 
     * @param path  (required)
     * @param cmd  (required)
     * @param groupName  (optional)
     * @param packageName  (optional)
     * @param packageVersion  (optional)
     * @param charset_  (optional)
     * @param force  (optional)
     * @param _package  (optional)
     * @param callback The callback to be executed when the API call finishes
     * @return The request call
     * @throws ApiException If fail to process the API call, e.g. serializing the request body object
     */
    public com.squareup.okhttp.Call postPackageServiceJsonAsync(String path, String cmd, String groupName, String packageName, String packageVersion, String charset_, Boolean force, File _package, final ApiCallback<String> callback) throws ApiException {

        ProgressResponseBody.ProgressListener progressListener = null;
        ProgressRequestBody.ProgressRequestListener progressRequestListener = null;

        if (callback != null) {
            progressListener = new ProgressResponseBody.ProgressListener() {
                @Override
                public void update(long bytesRead, long contentLength, boolean done) {
                    callback.onDownloadProgress(bytesRead, contentLength, done);
                }
            };

            progressRequestListener = new ProgressRequestBody.ProgressRequestListener() {
                @Override
                public void onRequestProgress(long bytesWritten, long contentLength, boolean done) {
                    callback.onUploadProgress(bytesWritten, contentLength, done);
                }
            };
        }

        com.squareup.okhttp.Call call = postPackageServiceJsonCall(path, cmd, groupName, packageName, packageVersion, charset_, force, _package, progressListener, progressRequestListener);
        Type localVarReturnType = new TypeToken<String>(){}.getType();
        apiClient.executeAsync(call, localVarReturnType, callback);
        return call;
    }
    /* Build call for postPackageUpdate */
    private com.squareup.okhttp.Call postPackageUpdateCall(String groupName, String packageName, String version, String path, String filter, String charset_, final ProgressResponseBody.ProgressListener progressListener, final ProgressRequestBody.ProgressRequestListener progressRequestListener) throws ApiException {
        Object localVarPostBody = null;
        
        // verify the required parameter 'groupName' is set
        if (groupName == null) {
            throw new ApiException("Missing the required parameter 'groupName' when calling postPackageUpdate(Async)");
        }
        
        // verify the required parameter 'packageName' is set
        if (packageName == null) {
            throw new ApiException("Missing the required parameter 'packageName' when calling postPackageUpdate(Async)");
        }
        
        // verify the required parameter 'version' is set
        if (version == null) {
            throw new ApiException("Missing the required parameter 'version' when calling postPackageUpdate(Async)");
        }
        
        // verify the required parameter 'path' is set
        if (path == null) {
            throw new ApiException("Missing the required parameter 'path' when calling postPackageUpdate(Async)");
        }
        

        // create path and map variables
        String localVarPath = "/crx/packmgr/update.jsp".replaceAll("\\{format\\}","json");

        List<Pair> localVarQueryParams = new ArrayList<Pair>();
        if (groupName != null)
        localVarQueryParams.addAll(apiClient.parameterToPairs("", "groupName", groupName));
        if (packageName != null)
        localVarQueryParams.addAll(apiClient.parameterToPairs("", "packageName", packageName));
        if (version != null)
        localVarQueryParams.addAll(apiClient.parameterToPairs("", "version", version));
        if (path != null)
        localVarQueryParams.addAll(apiClient.parameterToPairs("", "path", path));
        if (filter != null)
        localVarQueryParams.addAll(apiClient.parameterToPairs("", "filter", filter));
        if (charset_ != null)
        localVarQueryParams.addAll(apiClient.parameterToPairs("", "_charset_", charset_));

        Map<String, String> localVarHeaderParams = new HashMap<String, String>();

        Map<String, Object> localVarFormParams = new HashMap<String, Object>();

        final String[] localVarAccepts = {
            "application/json"
        };
        final String localVarAccept = apiClient.selectHeaderAccept(localVarAccepts);
        if (localVarAccept != null) localVarHeaderParams.put("Accept", localVarAccept);

        final String[] localVarContentTypes = {
            
        };
        final String localVarContentType = apiClient.selectHeaderContentType(localVarContentTypes);
        localVarHeaderParams.put("Content-Type", localVarContentType);

        if(progressListener != null) {
            apiClient.getHttpClient().networkInterceptors().add(new com.squareup.okhttp.Interceptor() {
                @Override
                public com.squareup.okhttp.Response intercept(com.squareup.okhttp.Interceptor.Chain chain) throws IOException {
                    com.squareup.okhttp.Response originalResponse = chain.proceed(chain.request());
                    return originalResponse.newBuilder()
                    .body(new ProgressResponseBody(originalResponse.body(), progressListener))
                    .build();
                }
            });
        }

        String[] localVarAuthNames = new String[] { "aemAuth" };
        return apiClient.buildCall(localVarPath, "POST", localVarQueryParams, localVarPostBody, localVarHeaderParams, localVarFormParams, localVarAuthNames, progressRequestListener);
    }

    /**
     * 
     * 
     * @param groupName  (required)
     * @param packageName  (required)
     * @param version  (required)
     * @param path  (required)
     * @param filter  (optional)
     * @param charset_  (optional)
     * @return String
     * @throws ApiException If fail to call the API, e.g. server error or cannot deserialize the response body
     */
    public String postPackageUpdate(String groupName, String packageName, String version, String path, String filter, String charset_) throws ApiException {
        ApiResponse<String> resp = postPackageUpdateWithHttpInfo(groupName, packageName, version, path, filter, charset_);
        return resp.getData();
    }

    /**
     * 
     * 
     * @param groupName  (required)
     * @param packageName  (required)
     * @param version  (required)
     * @param path  (required)
     * @param filter  (optional)
     * @param charset_  (optional)
     * @return ApiResponse&lt;String&gt;
     * @throws ApiException If fail to call the API, e.g. server error or cannot deserialize the response body
     */
    public ApiResponse<String> postPackageUpdateWithHttpInfo(String groupName, String packageName, String version, String path, String filter, String charset_) throws ApiException {
        com.squareup.okhttp.Call call = postPackageUpdateCall(groupName, packageName, version, path, filter, charset_, null, null);
        Type localVarReturnType = new TypeToken<String>(){}.getType();
        return apiClient.execute(call, localVarReturnType);
    }

    /**
     *  (asynchronously)
     * 
     * @param groupName  (required)
     * @param packageName  (required)
     * @param version  (required)
     * @param path  (required)
     * @param filter  (optional)
     * @param charset_  (optional)
     * @param callback The callback to be executed when the API call finishes
     * @return The request call
     * @throws ApiException If fail to process the API call, e.g. serializing the request body object
     */
    public com.squareup.okhttp.Call postPackageUpdateAsync(String groupName, String packageName, String version, String path, String filter, String charset_, final ApiCallback<String> callback) throws ApiException {

        ProgressResponseBody.ProgressListener progressListener = null;
        ProgressRequestBody.ProgressRequestListener progressRequestListener = null;

        if (callback != null) {
            progressListener = new ProgressResponseBody.ProgressListener() {
                @Override
                public void update(long bytesRead, long contentLength, boolean done) {
                    callback.onDownloadProgress(bytesRead, contentLength, done);
                }
            };

            progressRequestListener = new ProgressRequestBody.ProgressRequestListener() {
                @Override
                public void onRequestProgress(long bytesWritten, long contentLength, boolean done) {
                    callback.onUploadProgress(bytesWritten, contentLength, done);
                }
            };
        }

        com.squareup.okhttp.Call call = postPackageUpdateCall(groupName, packageName, version, path, filter, charset_, progressListener, progressRequestListener);
        Type localVarReturnType = new TypeToken<String>(){}.getType();
        apiClient.executeAsync(call, localVarReturnType, callback);
        return call;
    }
    /* Build call for postSetPassword */
    private com.squareup.okhttp.Call postSetPasswordCall(String old, String plain, String verify, final ProgressResponseBody.ProgressListener progressListener, final ProgressRequestBody.ProgressRequestListener progressRequestListener) throws ApiException {
        Object localVarPostBody = null;
        
        // verify the required parameter 'old' is set
        if (old == null) {
            throw new ApiException("Missing the required parameter 'old' when calling postSetPassword(Async)");
        }
        
        // verify the required parameter 'plain' is set
        if (plain == null) {
            throw new ApiException("Missing the required parameter 'plain' when calling postSetPassword(Async)");
        }
        
        // verify the required parameter 'verify' is set
        if (verify == null) {
            throw new ApiException("Missing the required parameter 'verify' when calling postSetPassword(Async)");
        }
        

        // create path and map variables
        String localVarPath = "/crx/explorer/ui/setpassword.jsp".replaceAll("\\{format\\}","json");

        List<Pair> localVarQueryParams = new ArrayList<Pair>();
        if (old != null)
        localVarQueryParams.addAll(apiClient.parameterToPairs("", "old", old));
        if (plain != null)
        localVarQueryParams.addAll(apiClient.parameterToPairs("", "plain", plain));
        if (verify != null)
        localVarQueryParams.addAll(apiClient.parameterToPairs("", "verify", verify));

        Map<String, String> localVarHeaderParams = new HashMap<String, String>();

        Map<String, Object> localVarFormParams = new HashMap<String, Object>();

        final String[] localVarAccepts = {
            "text/plain"
        };
        final String localVarAccept = apiClient.selectHeaderAccept(localVarAccepts);
        if (localVarAccept != null) localVarHeaderParams.put("Accept", localVarAccept);

        final String[] localVarContentTypes = {
            
        };
        final String localVarContentType = apiClient.selectHeaderContentType(localVarContentTypes);
        localVarHeaderParams.put("Content-Type", localVarContentType);

        if(progressListener != null) {
            apiClient.getHttpClient().networkInterceptors().add(new com.squareup.okhttp.Interceptor() {
                @Override
                public com.squareup.okhttp.Response intercept(com.squareup.okhttp.Interceptor.Chain chain) throws IOException {
                    com.squareup.okhttp.Response originalResponse = chain.proceed(chain.request());
                    return originalResponse.newBuilder()
                    .body(new ProgressResponseBody(originalResponse.body(), progressListener))
                    .build();
                }
            });
        }

        String[] localVarAuthNames = new String[] { "aemAuth" };
        return apiClient.buildCall(localVarPath, "POST", localVarQueryParams, localVarPostBody, localVarHeaderParams, localVarFormParams, localVarAuthNames, progressRequestListener);
    }

    /**
     * 
     * 
     * @param old  (required)
     * @param plain  (required)
     * @param verify  (required)
     * @throws ApiException If fail to call the API, e.g. server error or cannot deserialize the response body
     */
    public void postSetPassword(String old, String plain, String verify) throws ApiException {
        postSetPasswordWithHttpInfo(old, plain, verify);
    }

    /**
     * 
     * 
     * @param old  (required)
     * @param plain  (required)
     * @param verify  (required)
     * @return ApiResponse&lt;Void&gt;
     * @throws ApiException If fail to call the API, e.g. server error or cannot deserialize the response body
     */
    public ApiResponse<Void> postSetPasswordWithHttpInfo(String old, String plain, String verify) throws ApiException {
        com.squareup.okhttp.Call call = postSetPasswordCall(old, plain, verify, null, null);
        return apiClient.execute(call);
    }

    /**
     *  (asynchronously)
     * 
     * @param old  (required)
     * @param plain  (required)
     * @param verify  (required)
     * @param callback The callback to be executed when the API call finishes
     * @return The request call
     * @throws ApiException If fail to process the API call, e.g. serializing the request body object
     */
    public com.squareup.okhttp.Call postSetPasswordAsync(String old, String plain, String verify, final ApiCallback<Void> callback) throws ApiException {

        ProgressResponseBody.ProgressListener progressListener = null;
        ProgressRequestBody.ProgressRequestListener progressRequestListener = null;

        if (callback != null) {
            progressListener = new ProgressResponseBody.ProgressListener() {
                @Override
                public void update(long bytesRead, long contentLength, boolean done) {
                    callback.onDownloadProgress(bytesRead, contentLength, done);
                }
            };

            progressRequestListener = new ProgressRequestBody.ProgressRequestListener() {
                @Override
                public void onRequestProgress(long bytesWritten, long contentLength, boolean done) {
                    callback.onUploadProgress(bytesWritten, contentLength, done);
                }
            };
        }

        com.squareup.okhttp.Call call = postSetPasswordCall(old, plain, verify, progressListener, progressRequestListener);
        apiClient.executeAsync(call, callback);
        return call;
    }
}
