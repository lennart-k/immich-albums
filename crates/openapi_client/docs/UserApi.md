# \UserApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_profile_image**](UserApi.md#create_profile_image) | **POST** /user/profile-image | 
[**create_user**](UserApi.md#create_user) | **POST** /user | 
[**delete_profile_image**](UserApi.md#delete_profile_image) | **DELETE** /user/profile-image | 
[**delete_user**](UserApi.md#delete_user) | **DELETE** /user/{id} | 
[**get_all_users**](UserApi.md#get_all_users) | **GET** /user | 
[**get_my_user_info**](UserApi.md#get_my_user_info) | **GET** /user/me | 
[**get_profile_image**](UserApi.md#get_profile_image) | **GET** /user/profile-image/{id} | 
[**get_user_by_id**](UserApi.md#get_user_by_id) | **GET** /user/info/{id} | 
[**restore_user**](UserApi.md#restore_user) | **POST** /user/{id}/restore | 
[**update_user**](UserApi.md#update_user) | **PUT** /user | 



## create_profile_image

> models::CreateProfileImageResponseDto create_profile_image(file)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** |  | [required] |

### Return type

[**models::CreateProfileImageResponseDto**](CreateProfileImageResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> models::UserResponseDto create_user(create_user_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_dto** | [**CreateUserDto**](CreateUserDto.md) |  | [required] |

### Return type

[**models::UserResponseDto**](UserResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_profile_image

> delete_profile_image()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> models::UserResponseDto delete_user(id, delete_user_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**delete_user_dto** | [**DeleteUserDto**](DeleteUserDto.md) |  | [required] |

### Return type

[**models::UserResponseDto**](UserResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_users

> Vec<models::UserResponseDto> get_all_users(is_all)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**is_all** | **bool** |  | [required] |

### Return type

[**Vec<models::UserResponseDto>**](UserResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_user_info

> models::UserResponseDto get_my_user_info()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserResponseDto**](UserResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profile_image

> std::path::PathBuf get_profile_image(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_id

> models::UserResponseDto get_user_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::UserResponseDto**](UserResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_user

> models::UserResponseDto restore_user(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::UserResponseDto**](UserResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> models::UserResponseDto update_user(update_user_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_user_dto** | [**UpdateUserDto**](UpdateUserDto.md) |  | [required] |

### Return type

[**models::UserResponseDto**](UserResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

