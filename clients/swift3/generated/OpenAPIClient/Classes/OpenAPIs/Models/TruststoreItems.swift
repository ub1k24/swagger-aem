//
// TruststoreItems.swift
//
// Generated by openapi-generator
// https://openapi-generator.tech
//

import Foundation


open class TruststoreItems: JSONEncodable {

    /** Truststore alias name */
    public var alias: String?
    public var entryType: String?
    /** e.g. \&quot;CN&#x3D;localhost\&quot; */
    public var subject: String?
    /** e.g. \&quot;CN&#x3D;Admin\&quot; */
    public var issuer: String?
    /** e.g. \&quot;Sun Jul 01 12:00:00 AEST 2018\&quot; */
    public var notBefore: String?
    /** e.g. \&quot;Sun Jun 30 23:59:50 AEST 2019\&quot; */
    public var notAfter: String?
    /** 18165099476682912368 */
    public var serialNumber: Int32?

    public init() {}

    // MARK: JSONEncodable
    open func encodeToJSON() -> Any {
        var nillableDictionary = [String:Any?]()
        nillableDictionary["alias"] = self.alias
        nillableDictionary["entryType"] = self.entryType
        nillableDictionary["subject"] = self.subject
        nillableDictionary["issuer"] = self.issuer
        nillableDictionary["notBefore"] = self.notBefore
        nillableDictionary["notAfter"] = self.notAfter
        nillableDictionary["serialNumber"] = self.serialNumber?.encodeToJSON()

        let dictionary: [String:Any] = APIHelper.rejectNil(nillableDictionary) ?? [:]
        return dictionary
    }
}

