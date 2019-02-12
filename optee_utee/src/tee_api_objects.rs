use libc::*;
use super::*;

extern "C" {
    pub fn TEE_CreatePersistentObject (storageID: uint32_t, objectID: *const c_void, objectIDLen: uint32_t, flags: uint32_t, attributes: TEE_ObjectHandle, initialData: *const c_void, initialDataLen: uint32_t, object: *mut TEE_ObjectHandle) -> TEE_Result;
    pub fn TEE_OpenPersistentObject (storageID: uint32_t, objectID: *const c_void, objectIDLen: uint32_t, flags: uint32_t, object: *mut TEE_ObjectHandle) -> TEE_Result;
    pub fn TEE_WriteObjectData(object: TEE_ObjectHandle, buffer: *const c_void, fsize: uint32_t) -> TEE_Result;
    pub fn TEE_CloseAndDeletePersistentObject1(object: TEE_ObjectHandle) -> TEE_Result;
    pub fn TEE_CloseObject(object: TEE_ObjectHandle) -> TEE_Result;
    pub fn TEE_ReadObjectData(object: TEE_ObjectHandle, buffer: *mut c_void, fsize: uint32_t, count: *mut uint32_t) -> TEE_Result;
    pub fn TEE_GetObjectInfo1(object: TEE_ObjectHandle, objectInfo: *mut TEE_ObjectInfo) -> TEE_Result;
}
