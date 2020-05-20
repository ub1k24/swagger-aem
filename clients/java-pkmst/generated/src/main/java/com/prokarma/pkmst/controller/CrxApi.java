/**
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech) (3.2.1-SNAPSHOT).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */
package com.prokarma.pkmst.controller;

import java.io.File;
import com.prokarma.pkmst.model.InstallStatus;

import io.swagger.annotations.*;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestHeader;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestMethod;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RequestPart;
import org.springframework.web.multipart.MultipartFile;
import java.io.IOException;

import java.util.List;
/**
 * Provides the info about api methods
 * @author pkmst
 *
 */
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaPKMSTServerCodegen", date = "2019-08-04T23:41:29.600Z[GMT]")

@Api(value = "Crx", description = "the Crx API")
public interface CrxApi {

    @ApiOperation(value = "", notes = "", response = String.class, authorizations = {
        @Authorization(value = "aemAuth")
    }, tags={ "crx", })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "CRXDE is enabled", response = String.class),
        @ApiResponse(code = 404, message = "CRXDE is disabled", response = String.class) })
    @RequestMapping(value = "/crx/server/crx.default/jcr:root/.1.json",
        produces = { "plain/text" }, 
        method = RequestMethod.GET)
    ResponseEntity<String> getCrxdeStatus( @RequestHeader(value = "Accept", required = false) String accept) throws Exception;


    @ApiOperation(value = "", notes = "", response = InstallStatus.class, authorizations = {
        @Authorization(value = "aemAuth")
    }, tags={ "crx", })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "Retrieved CRX package manager install status", response = InstallStatus.class),
        @ApiResponse(code = 200, message = "Default response", response = String.class) })
    @RequestMapping(value = "/crx/packmgr/installstatus.jsp",
        produces = { "application/json" }, 
        method = RequestMethod.GET)
    ResponseEntity<InstallStatus> getInstallStatus( @RequestHeader(value = "Accept", required = false) String accept) throws Exception;


    @ApiOperation(value = "", notes = "", response = Void.class, authorizations = {
        @Authorization(value = "aemAuth")
    }, tags={ "crx", })
    @ApiResponses(value = { 
        @ApiResponse(code = 404, message = "Package Manager Servlet is disabled", response = String.class),
        @ApiResponse(code = 405, message = "Package Manager Servlet is active", response = String.class) })
    @RequestMapping(value = "/crx/packmgr/service/script.html",
        produces = { "text/html" }, 
        method = RequestMethod.GET)
    ResponseEntity<Void> getPackageManagerServlet( @RequestHeader(value = "Accept", required = false) String accept) throws Exception;


    @ApiOperation(value = "", notes = "", response = String.class, authorizations = {
        @Authorization(value = "aemAuth")
    }, tags={ "crx", })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "Default response", response = String.class) })
    @RequestMapping(value = "/crx/packmgr/service.jsp",
        produces = { "text/xml" }, 
        method = RequestMethod.POST)
    ResponseEntity<String> postPackageService(@ApiParam(value = "", required = true)  @RequestParam(value = "cmd", required = true) String cmd, @RequestHeader(value = "Accept", required = false) String accept) throws Exception;


    @ApiOperation(value = "", notes = "", response = String.class, authorizations = {
        @Authorization(value = "aemAuth")
    }, tags={ "crx", })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "Default response", response = String.class) })
    @RequestMapping(value = "/crx/packmgr/service/.json/{path}",
        produces = { "application/json" }, 
        consumes = { "multipart/form-data" },
        method = RequestMethod.POST)
    ResponseEntity<String> postPackageServiceJson(@ApiParam(value = "",required=true ) @PathVariable("path") String path,@ApiParam(value = "", required = true)  @RequestParam(value = "cmd", required = true) String cmd,@ApiParam(value = "")  @RequestParam(value = "groupName", required = false) String groupName,@ApiParam(value = "")  @RequestParam(value = "packageName", required = false) String packageName,@ApiParam(value = "")  @RequestParam(value = "packageVersion", required = false) String packageVersion,@ApiParam(value = "")  @RequestParam(value = "_charset_", required = false) String charset,@ApiParam(value = "")  @RequestParam(value = "force", required = false) Boolean force,@ApiParam(value = "")  @RequestParam(value = "recursive", required = false) Boolean recursive,@ApiParam(value = "file detail")  @RequestPart("file") MultipartFile package, @RequestHeader(value = "Accept", required = false) String accept) throws Exception;


    @ApiOperation(value = "", notes = "", response = String.class, authorizations = {
        @Authorization(value = "aemAuth")
    }, tags={ "crx", })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "Default response", response = String.class) })
    @RequestMapping(value = "/crx/packmgr/update.jsp",
        produces = { "application/json" }, 
        method = RequestMethod.POST)
    ResponseEntity<String> postPackageUpdate(@ApiParam(value = "", required = true)  @RequestParam(value = "groupName", required = true) String groupName,@ApiParam(value = "", required = true)  @RequestParam(value = "packageName", required = true) String packageName,@ApiParam(value = "", required = true)  @RequestParam(value = "version", required = true) String version,@ApiParam(value = "", required = true)  @RequestParam(value = "path", required = true) String path,@ApiParam(value = "")  @RequestParam(value = "filter", required = false) String filter,@ApiParam(value = "")  @RequestParam(value = "_charset_", required = false) String charset, @RequestHeader(value = "Accept", required = false) String accept) throws Exception;


    @ApiOperation(value = "", notes = "", response = String.class, authorizations = {
        @Authorization(value = "aemAuth")
    }, tags={ "crx", })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "Default response", response = String.class) })
    @RequestMapping(value = "/crx/explorer/ui/setpassword.jsp",
        produces = { "text/plain" }, 
        method = RequestMethod.POST)
    ResponseEntity<String> postSetPassword(@ApiParam(value = "", required = true)  @RequestParam(value = "old", required = true) String old,@ApiParam(value = "", required = true)  @RequestParam(value = "plain", required = true) String plain,@ApiParam(value = "", required = true)  @RequestParam(value = "verify", required = true) String verify, @RequestHeader(value = "Accept", required = false) String accept) throws Exception;

}