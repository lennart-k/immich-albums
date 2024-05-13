# \AlbumApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_assets_to_album**](AlbumApi.md#add_assets_to_album) | **PUT** /album/{id}/assets | 
[**add_users_to_album**](AlbumApi.md#add_users_to_album) | **PUT** /album/{id}/users | 
[**create_album**](AlbumApi.md#create_album) | **POST** /album | 
[**delete_album**](AlbumApi.md#delete_album) | **DELETE** /album/{id} | 
[**get_album_count**](AlbumApi.md#get_album_count) | **GET** /album/count | 
[**get_album_info**](AlbumApi.md#get_album_info) | **GET** /album/{id} | 
[**get_all_albums**](AlbumApi.md#get_all_albums) | **GET** /album | 
[**remove_asset_from_album**](AlbumApi.md#remove_asset_from_album) | **DELETE** /album/{id}/assets | 
[**remove_user_from_album**](AlbumApi.md#remove_user_from_album) | **DELETE** /album/{id}/user/{userId} | 
[**update_album_info**](AlbumApi.md#update_album_info) | **PATCH** /album/{id} | 
[**update_album_user**](AlbumApi.md#update_album_user) | **PUT** /album/{id}/user/{userId} | 



## add_assets_to_album

> Vec<models::BulkIdResponseDto> add_assets_to_album(id, bulk_ids_dto, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**bulk_ids_dto** | [**BulkIdsDto**](BulkIdsDto.md) |  | [required] |
**key** | Option<**String**> |  |  |

### Return type

[**Vec<models::BulkIdResponseDto>**](BulkIdResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_users_to_album

> models::AlbumResponseDto add_users_to_album(id, add_users_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**add_users_dto** | [**AddUsersDto**](AddUsersDto.md) |  | [required] |

### Return type

[**models::AlbumResponseDto**](AlbumResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_album

> models::AlbumResponseDto create_album(create_album_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_album_dto** | [**CreateAlbumDto**](CreateAlbumDto.md) |  | [required] |

### Return type

[**models::AlbumResponseDto**](AlbumResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_album

> delete_album(id)


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


## get_album_count

> models::AlbumCountResponseDto get_album_count()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AlbumCountResponseDto**](AlbumCountResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_album_info

> models::AlbumResponseDto get_album_info(id, key, without_assets)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**key** | Option<**String**> |  |  |
**without_assets** | Option<**bool**> |  |  |

### Return type

[**models::AlbumResponseDto**](AlbumResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_albums

> Vec<models::AlbumResponseDto> get_all_albums(asset_id, shared)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | Option<**uuid::Uuid**> | Only returns albums that contain the asset Ignores the shared parameter undefined: get all albums |  |
**shared** | Option<**bool**> |  |  |

### Return type

[**Vec<models::AlbumResponseDto>**](AlbumResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_asset_from_album

> Vec<models::BulkIdResponseDto> remove_asset_from_album(id, bulk_ids_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**bulk_ids_dto** | [**BulkIdsDto**](BulkIdsDto.md) |  | [required] |

### Return type

[**Vec<models::BulkIdResponseDto>**](BulkIdResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_from_album

> remove_user_from_album(id, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_album_info

> models::AlbumResponseDto update_album_info(id, update_album_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_album_dto** | [**UpdateAlbumDto**](UpdateAlbumDto.md) |  | [required] |

### Return type

[**models::AlbumResponseDto**](AlbumResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_album_user

> update_album_user(id, user_id, update_album_user_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**user_id** | **String** |  | [required] |
**update_album_user_dto** | [**UpdateAlbumUserDto**](UpdateAlbumUserDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

