<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST jsonplaceholder</name>
   <tag></tag>
   <elementGuidId>3e78d5be-d30c-48e8-901f-a87b16f37f3a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;postId\&quot;: 113,\n\t\&quot;id\&quot;: 113,\n\t\&quot;name\&quot;: \&quot;Syaeful Anwar\&quot;,\n\t\&quot;email\&quot;: \&quot;syaefulanwar08@gmail.com\&quot;,\n\t\&quot;body\&quot;: \&quot;Hi this my api automation testing\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>1b48f7a6-0cd6-4b65-b637-3b05d9322ab9</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/comments</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>113</defaultValue>
      <description></description>
      <id>b1e62a05-e02a-4af7-aed8-504be04c7dca</id>
      <masked>false</masked>
      <name>postId</name>
   </variables>
   <variables>
      <defaultValue>113</defaultValue>
      <description></description>
      <id>e9f5a49c-f67d-49fc-8488-75813d96f75b</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>'Syaeful Anwar'</defaultValue>
      <description></description>
      <id>3c2ff01f-57f5-4370-87d1-f054a11b1a9f</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>'syaefulanwar08@gmail.com'</defaultValue>
      <description></description>
      <id>67dbfc17-81ff-4d5f-b378-911b66508f46</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>'Hi this my api automation testing'</defaultValue>
      <description></description>
      <id>754b6704-074b-43ea-98a6-ded2b8f9d484</id>
      <masked>false</masked>
      <name>body</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyElementPropertyValue(response, 'postid', 113)
WS.verifyElementPropertyValue(response, 'id', 113)
WS.verifyElementPropertyValue(response, 'name', 'Syaeful Anwar')
WS.verifyElementPropertyValue(response, 'email', 'syaefulanwar08@gmail.com')
WS.verifyElementPropertyValue(response, 'body', 'Hi this my api automation testing')
WS.verifyElementPropertyValue(response, 'issues[0].fields.project.key', 'KTP')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
