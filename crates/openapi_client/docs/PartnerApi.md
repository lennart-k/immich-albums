# \PartnerApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_partner**](PartnerApi.md#create_partner) | **POST** /partner/{id} | 
[**get_partners**](PartnerApi.md#get_partners) | **GET** /partner | 
[**remove_partner**](PartnerApi.md#remove_partner) | **DELETE** /partner/{id} | 
[**update_partner**](PartnerApi.md#update_partner) | **PUT** /partner/{id} | 



## create_partner

> models::PartnerResponseDto create_partner(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::PartnerResponseDto**](PartnerResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_partners

> Vec<models::PartnerResponseDto> get_partners(direction)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**direction** | **String** |  | [required] |

### Return type

[**Vec<models::PartnerResponseDto>**](PartnerResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_partner

> remove_partner(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_partner

> models::PartnerResponseDto update_partner(id, update_partner_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_partner_dto** | [**UpdatePartnerDto**](UpdatePartnerDto.md) |  | [required] |

### Return type

[**models::PartnerResponseDto**](PartnerResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

