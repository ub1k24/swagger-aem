/**
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech) (3.2.1-SNAPSHOT).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */
package org.openapitools.api;

import org.springframework.core.io.Resource;
import org.openapitools.model.TruststoreInfo;
import io.swagger.annotations.*;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestHeader;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestMethod;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RequestPart;
import org.springframework.web.context.request.NativeWebRequest;
import org.springframework.web.multipart.MultipartFile;

import javax.validation.Valid;
import javax.validation.constraints.*;
import java.util.List;
import java.util.Map;
import java.util.Optional;
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2019-08-04T23:44:43.219Z[GMT]")

@Validated
@Api(value = "libs", description = "the libs API")
public interface LibsApi {

    default Optional<NativeWebRequest> getRequest() {
        return Optional.empty();
    }

    @ApiOperation(value = "", nickname = "getLoginPage", notes = "", response = String.class, tags={ "cq", })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "Default response", response = String.class) })
    @RequestMapping(value = "/libs/granite/core/content/login.html",
        produces = { "text/html" }, 
        method = RequestMethod.GET)
    default ResponseEntity<String> getLoginPage() {
        return new ResponseEntity<>(HttpStatus.NOT_IMPLEMENTED);

    }


    @ApiOperation(value = "", nickname = "getTruststoreInfo", notes = "", response = TruststoreInfo.class, authorizations = {
        @Authorization(value = "aemAuth")
    }, tags={ "sling", })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "Retrieved AEM Truststore info", response = TruststoreInfo.class),
        @ApiResponse(code = 200, message = "Default response", response = String.class) })
    @RequestMapping(value = "/libs/granite/security/truststore.json",
        produces = { "application/json" }, 
        method = RequestMethod.GET)
    default ResponseEntity<TruststoreInfo> getTruststoreInfo() {
        getRequest().ifPresent(request -> {
            for (MediaType mediaType: MediaType.parseMediaTypes(request.getHeader("Accept"))) {
                if (mediaType.isCompatibleWith(MediaType.valueOf("application/json"))) {
                    ApiUtil.setExampleResponse(request, "application/json", "{  \"aliases\" : [ {    \"entryType\" : \"entryType\",    \"notAfter\" : \"notAfter\",    \"serialNumber\" : 0,    \"subject\" : \"subject\",    \"alias\" : \"alias\",    \"issuer\" : \"issuer\",    \"notBefore\" : \"notBefore\"  }, {    \"entryType\" : \"entryType\",    \"notAfter\" : \"notAfter\",    \"serialNumber\" : 0,    \"subject\" : \"subject\",    \"alias\" : \"alias\",    \"issuer\" : \"issuer\",    \"notBefore\" : \"notBefore\"  } ],  \"exists\" : true}");
                    break;
                }
            }
        });
        return new ResponseEntity<>(HttpStatus.NOT_IMPLEMENTED);

    }


    @ApiOperation(value = "", nickname = "postAuthorizables", notes = "", response = String.class, authorizations = {
        @Authorization(value = "aemAuth")
    }, tags={ "sling", })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "Default response", response = String.class) })
    @RequestMapping(value = "/libs/granite/security/post/authorizables",
        produces = { "text/html" }, 
        method = RequestMethod.POST)
    default ResponseEntity<String> postAuthorizables(@NotNull @ApiParam(value = "", required = true) @Valid @RequestParam(value = "authorizableId", required = true) String authorizableId,@NotNull @ApiParam(value = "", required = true) @Valid @RequestParam(value = "intermediatePath", required = true) String intermediatePath,@ApiParam(value = "") @Valid @RequestParam(value = "createUser", required = false) String createUser,@ApiParam(value = "") @Valid @RequestParam(value = "createGroup", required = false) String createGroup,@ApiParam(value = "") @Valid @RequestParam(value = "rep:password", required = false) String repColonPassword,@ApiParam(value = "") @Valid @RequestParam(value = "profile/givenName", required = false) String profileSlashGivenName) {
        return new ResponseEntity<>(HttpStatus.NOT_IMPLEMENTED);

    }


    @ApiOperation(value = "", nickname = "postTruststore", notes = "", response = String.class, authorizations = {
        @Authorization(value = "aemAuth")
    }, tags={ "sling", })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "Default response", response = String.class) })
    @RequestMapping(value = "/libs/granite/security/post/truststore",
        produces = { "text/plain" }, 
        consumes = { "multipart/form-data" },
        method = RequestMethod.POST)
    default ResponseEntity<String> postTruststore(@ApiParam(value = "") @Valid @RequestParam(value = ":operation", required = false) String colonOperation,@ApiParam(value = "") @Valid @RequestParam(value = "newPassword", required = false) String newPassword,@ApiParam(value = "") @Valid @RequestParam(value = "rePassword", required = false) String rePassword,@ApiParam(value = "") @Valid @RequestParam(value = "keyStoreType", required = false) String keyStoreType,@ApiParam(value = "") @Valid @RequestParam(value = "removeAlias", required = false) String removeAlias,@ApiParam(value = "file detail") @Valid @RequestPart("file") MultipartFile certificate) {
        return new ResponseEntity<>(HttpStatus.NOT_IMPLEMENTED);

    }

}