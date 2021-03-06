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
package org.openapitools.server.apis

import com.google.gson.Gson
import io.ktor.application.call
import io.ktor.auth.UserIdPrincipal
import io.ktor.auth.authentication
import io.ktor.auth.basicAuthentication
import io.ktor.auth.oauth
import io.ktor.auth.OAuthAccessTokenResponse
import io.ktor.auth.OAuthServerSettings
import io.ktor.http.ContentType
import io.ktor.http.HttpStatusCode
import io.ktor.locations.*
import io.ktor.response.respond
import io.ktor.response.respondText
import io.ktor.routing.*

import kotlinx.coroutines.experimental.asCoroutineDispatcher

import org.openapitools.server.ApplicationAuthProviders
import org.openapitools.server.Paths
import org.openapitools.server.ApplicationExecutors
import org.openapitools.server.HTTP.client
import org.openapitools.server.infrastructure.ApiPrincipal
import org.openapitools.server.infrastructure.apiKeyAuth

// ktor 0.9.x is missing io.ktor.locations.DELETE, this adds it.
// see https://github.com/ktorio/ktor/issues/288
import org.openapitools.server.delete

import org.openapitools.server.models.SamlConfigurationInfo

fun Route.ConsoleApi() {
    val gson = Gson()
    val empty = mutableMapOf<String, Any?>()

    get<Paths.getAemProductInfo> {  it: Paths.getAemProductInfo ->
        val principal = call.authentication.principal<UserIdPrincipal>()
        
        if (principal == null) {
            call.respond(HttpStatusCode.Unauthorized)
        } else {
            val exampleContentType = "application/json"
            val exampleContentString = """"""""
            
            when(exampleContentType) {
                "application/json" -> call.respond(gson.fromJson(exampleContentString, empty::class.java))
                "application/xml" -> call.respondText(exampleContentString, ContentType.Text.Xml)
                else -> call.respondText(exampleContentString)
            }
        }
    }
    .apply {
      // TODO: ktor doesn't allow different authentication registrations for endpoints sharing the same path but different methods.
      //       It could be the authentication block is being abused here. Until this is resolved, swallow duplicate exceptions.

        try {
            authentication {
                basicAuthentication("aemAuth") { credentials ->
                    // TODO: "Apply your basic authentication functionality."
                    // Accessible in-method via call.principal<UserIdPrincipal>()
                    if (credentials.name == "Swagger" && "Codegen" == credentials.password) {
                         UserIdPrincipal(credentials.name)
                    } else {
                        null
                    }
                }
            }
        } catch(e: io.ktor.application.DuplicateApplicationFeatureException){
            application.environment.log.warn("authentication block for '/system/console/status-productinfo.json' is duplicated in code. " +
            "Generated endpoints may need to be merged under a 'route' entry.")
        }
    }

    get<Paths.getConfigMgr> {  it: Paths.getConfigMgr ->
        val principal = call.authentication.principal<UserIdPrincipal>()
        
        if (principal == null) {
            call.respond(HttpStatusCode.Unauthorized)
        } else {
            call.respond(HttpStatusCode.NotImplemented)
        }
    }
    .apply {
      // TODO: ktor doesn't allow different authentication registrations for endpoints sharing the same path but different methods.
      //       It could be the authentication block is being abused here. Until this is resolved, swallow duplicate exceptions.

        try {
            authentication {
                basicAuthentication("aemAuth") { credentials ->
                    // TODO: "Apply your basic authentication functionality."
                    // Accessible in-method via call.principal<UserIdPrincipal>()
                    if (credentials.name == "Swagger" && "Codegen" == credentials.password) {
                         UserIdPrincipal(credentials.name)
                    } else {
                        null
                    }
                }
            }
        } catch(e: io.ktor.application.DuplicateApplicationFeatureException){
            application.environment.log.warn("authentication block for '/system/console/configMgr' is duplicated in code. " +
            "Generated endpoints may need to be merged under a 'route' entry.")
        }
    }

    route("/system/console/bundles/{name}") {
        post {
            val principal = call.authentication.principal<UserIdPrincipal>()
            
            if (principal == null) {
                call.respond(HttpStatusCode.Unauthorized)
            } else {
                call.respond(HttpStatusCode.NotImplemented)
            }
        }
    }
    .apply {
      // TODO: ktor doesn't allow different authentication registrations for endpoints sharing the same path but different methods.
      //       It could be the authentication block is being abused here. Until this is resolved, swallow duplicate exceptions.

        try {
            authentication {
                basicAuthentication("aemAuth") { credentials ->
                    // TODO: "Apply your basic authentication functionality."
                    // Accessible in-method via call.principal<UserIdPrincipal>()
                    if (credentials.name == "Swagger" && "Codegen" == credentials.password) {
                         UserIdPrincipal(credentials.name)
                    } else {
                        null
                    }
                }
            }
        } catch(e: io.ktor.application.DuplicateApplicationFeatureException){
            application.environment.log.warn("authentication block for '/system/console/bundles/{name}' is duplicated in code. " +
            "Generated endpoints may need to be merged under a 'route' entry.")
        }
    }

    route("/system/console/jmx/com.adobe.granite:type&#x3D;Repository/op/{action}") {
        post {
            val principal = call.authentication.principal<UserIdPrincipal>()
            
            if (principal == null) {
                call.respond(HttpStatusCode.Unauthorized)
            } else {
                call.respond(HttpStatusCode.NotImplemented)
            }
        }
    }
    .apply {
      // TODO: ktor doesn't allow different authentication registrations for endpoints sharing the same path but different methods.
      //       It could be the authentication block is being abused here. Until this is resolved, swallow duplicate exceptions.

        try {
            authentication {
                basicAuthentication("aemAuth") { credentials ->
                    // TODO: "Apply your basic authentication functionality."
                    // Accessible in-method via call.principal<UserIdPrincipal>()
                    if (credentials.name == "Swagger" && "Codegen" == credentials.password) {
                         UserIdPrincipal(credentials.name)
                    } else {
                        null
                    }
                }
            }
        } catch(e: io.ktor.application.DuplicateApplicationFeatureException){
            application.environment.log.warn("authentication block for '/system/console/jmx/com.adobe.granite:type&#x3D;Repository/op/{action}' is duplicated in code. " +
            "Generated endpoints may need to be merged under a 'route' entry.")
        }
    }

    route("/system/console/configMgr/com.adobe.granite.auth.saml.SamlAuthenticationHandler") {
        post {
            val principal = call.authentication.principal<UserIdPrincipal>()
            
            if (principal == null) {
                call.respond(HttpStatusCode.Unauthorized)
            } else {
                val exampleContentType = ""
                val exampleContentString = """"""
                
                when(exampleContentType) {
                    "application/json" -> call.respond(gson.fromJson(exampleContentString, empty::class.java))
                    "application/xml" -> call.respondText(exampleContentString, ContentType.Text.Xml)
                    else -> call.respondText(exampleContentString)
                }
            }
        }
    }
    .apply {
      // TODO: ktor doesn't allow different authentication registrations for endpoints sharing the same path but different methods.
      //       It could be the authentication block is being abused here. Until this is resolved, swallow duplicate exceptions.

        try {
            authentication {
                basicAuthentication("aemAuth") { credentials ->
                    // TODO: "Apply your basic authentication functionality."
                    // Accessible in-method via call.principal<UserIdPrincipal>()
                    if (credentials.name == "Swagger" && "Codegen" == credentials.password) {
                         UserIdPrincipal(credentials.name)
                    } else {
                        null
                    }
                }
            }
        } catch(e: io.ktor.application.DuplicateApplicationFeatureException){
            application.environment.log.warn("authentication block for '/system/console/configMgr/com.adobe.granite.auth.saml.SamlAuthenticationHandler' is duplicated in code. " +
            "Generated endpoints may need to be merged under a 'route' entry.")
        }
    }
}
