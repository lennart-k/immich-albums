# \AssetApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_bulk_upload**](AssetApi.md#check_bulk_upload) | **POST** /asset/bulk-upload-check | 
[**check_existing_assets**](AssetApi.md#check_existing_assets) | **POST** /asset/exist | 
[**delete_assets**](AssetApi.md#delete_assets) | **DELETE** /asset | 
[**get_all_assets**](AssetApi.md#get_all_assets) | **GET** /asset | 
[**get_all_user_assets_by_device_id**](AssetApi.md#get_all_user_assets_by_device_id) | **GET** /asset/device/{deviceId} | 
[**get_asset_info**](AssetApi.md#get_asset_info) | **GET** /asset/{id} | 
[**get_asset_statistics**](AssetApi.md#get_asset_statistics) | **GET** /asset/statistics | 
[**get_asset_thumbnail**](AssetApi.md#get_asset_thumbnail) | **GET** /asset/thumbnail/{id} | 
[**get_map_markers**](AssetApi.md#get_map_markers) | **GET** /asset/map-marker | 
[**get_memory_lane**](AssetApi.md#get_memory_lane) | **GET** /asset/memory-lane | 
[**get_random**](AssetApi.md#get_random) | **GET** /asset/random | 
[**run_asset_jobs**](AssetApi.md#run_asset_jobs) | **POST** /asset/jobs | 
[**serve_file**](AssetApi.md#serve_file) | **GET** /asset/file/{id} | 
[**update_asset**](AssetApi.md#update_asset) | **PUT** /asset/{id} | 
[**update_assets**](AssetApi.md#update_assets) | **PUT** /asset | 
[**update_stack_parent**](AssetApi.md#update_stack_parent) | **PUT** /asset/stack/parent | 
[**upload_file**](AssetApi.md#upload_file) | **POST** /asset/upload | 



## check_bulk_upload

> models::AssetBulkUploadCheckResponseDto check_bulk_upload(asset_bulk_upload_check_dto)


Checks if assets exist by checksums

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_bulk_upload_check_dto** | [**AssetBulkUploadCheckDto**](AssetBulkUploadCheckDto.md) |  | [required] |

### Return type

[**models::AssetBulkUploadCheckResponseDto**](AssetBulkUploadCheckResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_existing_assets

> models::CheckExistingAssetsResponseDto check_existing_assets(check_existing_assets_dto)


Checks if multiple assets exist on the server and returns all existing - used by background backup

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_existing_assets_dto** | [**CheckExistingAssetsDto**](CheckExistingAssetsDto.md) |  | [required] |

### Return type

[**models::CheckExistingAssetsResponseDto**](CheckExistingAssetsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_assets

> delete_assets(asset_bulk_delete_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_bulk_delete_dto** | [**AssetBulkDeleteDto**](AssetBulkDeleteDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_assets

> Vec<models::AssetResponseDto> get_all_assets(if_none_match, is_archived, is_favorite, skip, take, updated_after, updated_before, user_id)


Get all AssetEntity belong to the user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**if_none_match** | Option<**String**> | ETag of data already cached on the client |  |
**is_archived** | Option<**bool**> |  |  |
**is_favorite** | Option<**bool**> |  |  |
**skip** | Option<**i32**> |  |  |
**take** | Option<**i32**> |  |  |
**updated_after** | Option<**String**> |  |  |
**updated_before** | Option<**String**> |  |  |
**user_id** | Option<**uuid::Uuid**> |  |  |

### Return type

[**Vec<models::AssetResponseDto>**](AssetResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_user_assets_by_device_id

> Vec<String> get_all_user_assets_by_device_id(device_id)


Get all asset of a device that are in the database, ID only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_id** | **String** |  | [required] |

### Return type

**Vec<String>**

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_info

> models::AssetResponseDto get_asset_info(id, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**key** | Option<**String**> |  |  |

### Return type

[**models::AssetResponseDto**](AssetResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_statistics

> models::AssetStatsResponseDto get_asset_statistics(is_archived, is_favorite, is_trashed)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**is_archived** | Option<**bool**> |  |  |
**is_favorite** | Option<**bool**> |  |  |
**is_trashed** | Option<**bool**> |  |  |

### Return type

[**models::AssetStatsResponseDto**](AssetStatsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_thumbnail

> std::path::PathBuf get_asset_thumbnail(id, format, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**format** | Option<[**ThumbnailFormat**](.md)> |  |  |
**key** | Option<**String**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_map_markers

> Vec<models::MapMarkerResponseDto> get_map_markers(file_created_after, file_created_before, is_archived, is_favorite, with_partners)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_created_after** | Option<**String**> |  |  |
**file_created_before** | Option<**String**> |  |  |
**is_archived** | Option<**bool**> |  |  |
**is_favorite** | Option<**bool**> |  |  |
**with_partners** | Option<**bool**> |  |  |

### Return type

[**Vec<models::MapMarkerResponseDto>**](MapMarkerResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_memory_lane

> Vec<models::MemoryLaneResponseDto> get_memory_lane(day, month)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**day** | **i32** |  | [required] |
**month** | **i32** |  | [required] |

### Return type

[**Vec<models::MemoryLaneResponseDto>**](MemoryLaneResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_random

> Vec<models::AssetResponseDto> get_random(count)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | Option<**f64**> |  |  |

### Return type

[**Vec<models::AssetResponseDto>**](AssetResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_asset_jobs

> run_asset_jobs(asset_jobs_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_jobs_dto** | [**AssetJobsDto**](AssetJobsDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## serve_file

> std::path::PathBuf serve_file(id, is_thumb, is_web, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**is_thumb** | Option<**bool**> |  |  |
**is_web** | Option<**bool**> |  |  |
**key** | Option<**String**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_asset

> models::AssetResponseDto update_asset(id, update_asset_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_asset_dto** | [**UpdateAssetDto**](UpdateAssetDto.md) |  | [required] |

### Return type

[**models::AssetResponseDto**](AssetResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_assets

> update_assets(asset_bulk_update_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_bulk_update_dto** | [**AssetBulkUpdateDto**](AssetBulkUpdateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_stack_parent

> update_stack_parent(update_stack_parent_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_stack_parent_dto** | [**UpdateStackParentDto**](UpdateStackParentDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_file

> models::AssetFileUploadResponseDto upload_file(asset_data, device_asset_id, device_id, file_created_at, file_modified_at, key, x_immich_checksum, duration, is_archived, is_favorite, is_offline, is_visible, library_id, live_photo_data, sidecar_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_data** | **std::path::PathBuf** |  | [required] |
**device_asset_id** | **String** |  | [required] |
**device_id** | **String** |  | [required] |
**file_created_at** | **String** |  | [required] |
**file_modified_at** | **String** |  | [required] |
**key** | Option<**String**> |  |  |
**x_immich_checksum** | Option<**String**> | sha1 checksum that can be used for duplicate detection before the file is uploaded |  |
**duration** | Option<**String**> |  |  |
**is_archived** | Option<**bool**> |  |  |
**is_favorite** | Option<**bool**> |  |  |
**is_offline** | Option<**bool**> |  |  |
**is_visible** | Option<**bool**> |  |  |
**library_id** | Option<**uuid::Uuid**> |  |  |
**live_photo_data** | Option<**std::path::PathBuf**> |  |  |
**sidecar_data** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**models::AssetFileUploadResponseDto**](AssetFileUploadResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

