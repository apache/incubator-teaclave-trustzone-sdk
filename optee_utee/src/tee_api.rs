use libc::*;
use super::*;

extern "C" {
    // Property access functions

    pub fn TEE_GetPropertyAsString(propsetOrEnumerator: TEE_PropSetHandle, name: *const c_char, valueBuffer: *mut c_char, valueBufferLen: *mut uint32_t) -> TEE_Result;
    pub fn TEE_GetPropertyAsBool(propsetOrEnumerator: TEE_PropSetHandle, name: *const c_char, value: *mut bool) -> TEE_Result;
    pub fn TEE_GetPropertyAsU32(propsetOrEnumerator: TEE_PropSetHandle, name: *const c_char, value: *mut uint32_t) -> TEE_Result;
    pub fn TEE_GetPropertyAsBinaryBlock(propsetOrEnumerator: TEE_PropSetHandle, name: *const c_char, valueBuffer: *mut c_void, valueBufferLen: *mut uint32_t) -> TEE_Result;
    pub fn TEE_GetPropertyAsUUID(propsetOrEnumerator: TEE_PropSetHandle, name: *const c_char, value: *mut TEE_UUID) -> TEE_Result;
    pub fn TEE_GetPropertyAsIdentity(propsetOrEnumerator: TEE_PropSetHandle, name: *const c_char, value: TEE_Identity) -> TEE_Result;
    pub fn TEE_AllocatePropertyEnumerator(enumerator: *mut TEE_PropSetHandle) -> TEE_Result;
    pub fn TEE_FreePropertyEnumerator(enumerator: TEE_PropSetHandle);
    pub fn TEE_StartPropertyEnumerator(enumerator: TEE_PropSetHandle, propSet: TEE_PropSetHandle);
    pub fn TEE_ResetPropertyEnumerator(enumerator: TEE_PropSetHandle);
    pub fn TEE_GetPropertyName(enumerator: TEE_PropSetHandle, nameBuffer: *mut c_void, nameBufferLen: *mut uint32_t) -> TEE_Result;
    pub fn TEE_GetNextProperty(enumerator: TEE_PropSetHandle) -> TEE_Result;

    // System API - Misc

    pub fn TEE_Panic(panicCode: TEE_Result);

    // System API - Internal Client API

    pub fn TEE_OpenTASession(destination: *const TEE_UUID, cancellationRequestTimeout: uint32_t, paramTypes: uint32_t, params: *mut TEE_Param, session: *mut TEE_TASessionHandle, returnOrigin: *mut uint32_t) -> TEE_Result;
    pub fn TEE_CloseTASession(session: TEE_TASessionHandle);
    pub fn TEE_InvokeTACommand(session: TEE_TASessionHandle, cancellationRequestTimeout: uint32_t, commandID: uint32_t, paramTypes: uint32_t, params: *mut TEE_Param, returnOrigin: *mut uint32_t) -> TEE_Result;

    // System API - Cancellations

    pub fn TEE_GetCancellationFlag() -> bool;
    pub fn TEE_UnmaskCancellation() -> bool;
    pub fn TEE_MaskCancellation() -> bool;

    // System API - Memory Management

    pub fn TEE_CheckMemoryAccessRights(accessFlags: uint32_t, buffer: *mut c_void, size: uint32_t) -> TEE_Result;
    pub fn TEE_SetInstanceData(instanceData: *const c_void);
    pub fn TEE_GetInstanceData() -> *const c_void;
    pub fn TEE_Malloc(size: uint32_t, hint: uint32_t) -> *mut c_void;
    pub fn TEE_Realloc(buffer: *mut c_void, newSize: uint32_t) -> *mut c_void;
    pub fn TEE_Free(buffer: *mut c_void);
    pub fn TEE_MemMove(dest: *mut c_void, src: *const c_void, size: uint32_t) -> *mut c_void;
    pub fn TEE_MemCompare(buffer1: *const c_void, buffer2: *const c_void, size: uint32_t) -> int32_t;
    pub fn TEE_MemFill(buff: *mut c_void, x: uint32_t, size: uint32_t) -> *mut c_void;

    // Data and Key Storage API  - Generic Object Functions

    pub fn TEE_GetObjectInfo(object: TEE_ObjectHandle, objectInfo: *mut TEE_ObjectInfo);
    pub fn TEE_GetObjectInfo1(object: TEE_ObjectHandle, objectInfo: *mut TEE_ObjectInfo) -> TEE_Result;
    pub fn TEE_RestrictObjectUsage(object: TEE_ObjectHandle, objectUsage: uint32_t);
    pub fn TEE_GetObjectBufferAttribute(object: TEE_ObjectHandle, attributeID: uint32_t, buffer: *mut c_void, size: *mut uint32_t) -> TEE_Result;
    pub fn TEE_GetObjectValueAttribute(object: TEE_ObjectHandle, attributeID: uint32_t, a: *mut uint32_t, b: *mut uint32_t) -> TEE_Result;
    pub fn TEE_CloseObject(object: TEE_ObjectHandle);

    /* Data and Key Storage API  - Transient Object Functions */

    pub fn TEE_AllocateTransientObject(objectType: TEE_ObjectType, maxKeySize: uint32_t, object: *mut TEE_ObjectHandle) -> TEE_Result;
    pub fn TEE_FreeTransientObject(object: TEE_ObjectHandle) -> c_void;
    pub fn TEE_ResetTransientObject(object: TEE_ObjectHandle) -> c_void;
    pub fn TEE_PopulateTransientObject(object: TEE_ObjectHandle, attrs: *const TEE_Attribute, attrCount: uint32_t) -> TEE_Result;
    pub fn TEE_InitRefAttribute(attr: *mut TEE_Attribute, attributeID: uint32_t, buffer: *const c_void, length: uint32_t) -> c_void;
    pub fn TEE_InitValueAttribute(attr: *mut TEE_Attribute, attributeID: uint32_t, a: uint32_t, b: uint32_t) -> c_void;
    pub fn TEE_CopyObjectAttributes(destObject: TEE_ObjectHandle, srcObject: TEE_ObjectHandle) -> c_void;
    pub fn TEE_CopyObjectAttributes1(destObject: TEE_ObjectHandle, srcObject: TEE_ObjectHandle) -> TEE_Result;
    pub fn TEE_GenerateKey(object: TEE_ObjectHandle, keySize: uint32_t, params: *const TEE_Attribute, paramCount: uint32_t) -> TEE_Result;

    // Data and Key Storage API  - Persistent Object Functions

    pub fn TEE_OpenPersistentObject (storageID: uint32_t, objectID: *const c_void, objectIDLen: uint32_t, flags: uint32_t, object: *mut TEE_ObjectHandle) -> TEE_Result;
    pub fn TEE_CreatePersistentObject (storageID: uint32_t, objectID: *const c_void, objectIDLen: uint32_t, flags: uint32_t, attributes: TEE_ObjectHandle, initialData: *const c_void, initialDataLen: uint32_t, object: *mut TEE_ObjectHandle) -> TEE_Result;
    pub fn TEE_CloseAndDeletePersistentObject(object: TEE_ObjectHandle);
    pub fn TEE_CloseAndDeletePersistentObject1(object: TEE_ObjectHandle) -> TEE_Result;
    pub fn TEE_RenamePersistentObject(object: TEE_ObjectHandle, newObjectID: *const c_void, newObjectIDLen: uint32_t) -> TEE_Result;
    pub fn TEE_AllocatePersistentObjectEnumerator(objectEnumerator: *mut TEE_ObjectEnumHandle) -> TEE_Result;
    pub fn TEE_FreePersistentObjectEnumerator(objectEnumerator: TEE_ObjectEnumHandle);
    pub fn TEE_ResetPersistentObjectEnumerator(objectEnumerator: TEE_ObjectEnumHandle);
    pub fn TEE_StartPersistentObjectEnumerator(objectEnumerator: TEE_ObjectEnumHandle, storageID: uint32_t) -> TEE_Result;
    pub fn TEE_GetNextPersistentObject(objectEnumerator: TEE_ObjectEnumHandle, objectInfo: *mut TEE_ObjectInfo, objectID: *mut c_void, objectIDLen: *mut uint32_t) -> TEE_Result;

    // Data and Key Storage API  - Data Stream Access Functions

    pub fn TEE_ReadObjectData(object: TEE_ObjectHandle, buffer: *mut c_void, fsize: uint32_t, count: *mut uint32_t) -> TEE_Result;
    pub fn TEE_WriteObjectData(object: TEE_ObjectHandle, buffer: *const c_void, fsize: uint32_t) -> TEE_Result;
    pub fn TEE_TruncateObjectData(object: TEE_ObjectHandle, size: uint32_t) -> TEE_Result;
    pub fn TEE_SeekObjectData(object: TEE_ObjectHandle, offset: int32_t, whence: TEE_Whence) -> TEE_Result;

    // Cryptographic Operations API - Generic Operation Functions

    pub fn TEE_AllocateOperation(operation: *mut TEE_OperationHandle, algorithm: uint32_t, mode: uint32_t, maxKeySize: uint32_t) -> TEE_Result;
    pub fn TEE_FreeOperation(operation: TEE_OperationHandle) -> c_void;
    pub fn TEE_GetOperationInfo(operation: TEE_OperationHandle, operationInfo: *mut TEE_OperationInfo) -> c_void;
    pub fn TEE_GetOperationInfoMultiple(operation: TEE_OperationHandle, operationInfoMultiple: *mut TEE_OperationInfoMultiple, operationSize: *mut uint32_t) -> TEE_Result;
    pub fn TEE_ResetOperation(operation: TEE_OperationHandle) -> c_void;
    pub fn TEE_SetOperationKey(operation: TEE_OperationHandle, key: TEE_ObjectHandle) -> TEE_Result;
    pub fn TEE_SetOperationKey2(operation: TEE_OperationHandle, key1: TEE_ObjectHandle, key2: TEE_ObjectHandle) -> TEE_Result;
    pub fn TEE_CopyOperation(dstOperation: TEE_OperationHandle, srcOperation: TEE_OperationHandle) -> c_void;

    // Cryptographic Operations API - Message Digest Functions

    pub fn TEE_DigestUpdate(operation: TEE_OperationHandle, chunk: *const c_void, chunkSize: uint32_t) -> c_void;
    pub fn TEE_DigestDoFinal(operation: TEE_OperationHandle, chunk: *const c_void, chunkLen: uint32_t, hash: *mut c_void, hashLen: *mut uint32_t) -> TEE_Result;

    // Cryptographic Operations API - Symmetric Cipher Functions

    pub fn TEE_CipherInit(operation: TEE_OperationHandle, IV: *const c_void, IVLen: uint32_t) -> c_void;
    pub fn TEE_CipherUpdate(operation: TEE_OperationHandle, srcData: *const c_void, srcLen: uint32_t, destData: *mut c_void, destLen: *mut uint32_t) -> TEE_Result;
    pub fn TEE_CipherDoFinal(operation: TEE_OperationHandle, srcData: *const c_void, srcLen: uint32_t, destData: *mut c_void, destLen: *mut uint32_t) -> TEE_Result;

    // Cryptographic Operations API - MAC Functions

    pub fn TEE_MACInit(operation: TEE_OperationHandle, IV: *const c_void, IVLen: uint32_t) -> c_void;
    pub fn TEE_MACUpdate(operation: TEE_OperationHandle, chunk: *const c_void, chunkSize: uint32_t) -> c_void;
    pub fn TEE_MACComputeFinal(operation: TEE_OperationHandle, message: *const c_void, messageLen: uint32_t, mac: *mut c_void, macLen: *mut uint32_t) -> TEE_Result;
    pub fn TEE_MACCompareFinal(operation: TEE_OperationHandle, message: *const c_void, messageLen: uint32_t, mac: *const c_void, macLen: uint32_t) -> TEE_Result;


    // Cryptographic Operations API - Authenticated Encryption Functions

    pub fn TEE_AEInit(operation: TEE_OperationHandle, nonce: *const c_void, nonceLen: uint32_t, tagLen: uint32_t, AADLen: uint32_t, payloadLen: uint32_t) -> TEE_Result;
    pub fn TEE_AEUpdateAAD(operation: TEE_OperationHandle, AADdata: *const c_void, AADdataLen: uint32_t) -> c_void;
    pub fn TEE_AEUpdate(operation: TEE_OperationHandle, srcData: *const c_void, srcLen: uint32_t, destData: *mut c_void, destLen: *mut uint32_t) -> TEE_Result;
    pub fn TEE_AEEncryptFinal(operation: TEE_OperationHandle, srcData: *const c_void, srcLen: uint32_t, destData: *mut c_void, destLen: *mut uint32_t, tag: *mut c_void, tagLen: *mut uint32_t) -> TEE_Result;
    pub fn TEE_AEDecryptFinal(operation: TEE_OperationHandle, srcData: *const c_void, srcLen: uint32_t, destData: *mut c_void, destLen: *mut uint32_t, tag: *mut c_void, tagLen: uint32_t) -> TEE_Result;

    // Cryptographic Operations API - Asymmetric Functions

    pub fn TEE_AsymmetricEncrypt(operation: TEE_OperationHandle, params: *const TEE_Attribute, paramCount: uint32_t, srcData: *const c_void, srcLen: uint32_t, destData: *mut c_void, destLen: *mut uint32_t) -> TEE_Result;
    pub fn TEE_AsymmetricDecrypt(operation: TEE_OperationHandle, params: *const TEE_Attribute, paramCount: uint32_t, srcData: *const c_void, srcLen: uint32_t, destData: *mut c_void, destLen: *mut uint32_t) -> TEE_Result;
    pub fn TEE_AsymmetricSignDigest(operation: TEE_OperationHandle, params: *const TEE_Attribute, paramCount: uint32_t, digest: *const c_void, digestLen: uint32_t, signature: *mut c_void, signatureLen: *mut uint32_t) -> TEE_Result;
    pub fn TEE_AsymmetricVerifyDigest(operation: TEE_OperationHandle, params: *const TEE_Attribute, paramCount: uint32_t, digest: *const c_void, digestLen: uint32_t, signature: *const c_void, signatureLen: uint32_t) -> TEE_Result;

    // Cryptographic Operations API - Key Derivation Functions

    pub fn TEE_DeriveKey(operation: TEE_OperationHandle, params: *const TEE_Attribute, paramCount: uint32_t, derivedKey: TEE_ObjectHandle) -> c_void;

    // Cryptographic Operations API - Random Number Generation Functions

    pub fn TEE_GenerateRandom(randomBuffer: *mut c_void, randomBufferLen: uint32_t) -> c_void;

    // Date & Time API

    pub fn TEE_GetSystemTime(time: *mut TEE_Time) -> c_void;
    pub fn TEE_Wait(timeout: uint32_t) -> TEE_Result;
    pub fn TEE_GetTAPersistentTime(time: *mut TEE_Time) -> TEE_Result;
    pub fn TEE_SetTAPersistentTime(time: *const TEE_Time) -> TEE_Result;
    pub fn TEE_GetREETime(time: *mut TEE_Time) -> c_void;

    // TEE Arithmetical API - Memory allocation and size of objects

    pub fn TEE_BigIntFMMSizeInU32(modulusSizeInBits: uint32_t) -> uint32_t;
    pub fn TEE_BigIntFMMContextSizeInU32(modulusSizeInBits: uint32_t) -> uint32_t;

    // TEE Arithmetical API - Initialization functions

    pub fn TEE_BigIntInit(bigInt: *mut TEE_BigInt, len: uint32_t) -> c_void;
    pub fn TEE_BigIntInitFMMContext(context: *mut TEE_BigIntFMMContext, len: uint32_t, modulus: *const TEE_BigInt) -> c_void;
    pub fn TEE_BigIntInitFMM(bigIntFMM: *mut TEE_BigIntFMM, len: uint32_t) -> c_void;

    // TEE Arithmetical API - Converter functions

    pub fn TEE_BigIntConvertFromOctetString(dest: *mut TEE_BigInt, buffer: *const uint8_t, bufferLen: uint32_t, sign: int32_t) -> TEE_Result;
    pub fn TEE_BigIntConvertToOctetString(buffer: *mut uint8_t, bufferLen: *mut uint32_t, bigInt: *const TEE_BigInt) -> TEE_Result;
    pub fn TEE_BigIntConvertFromS32(dest: *mut TEE_BigInt, shortVal: int32_t) -> c_void;
    pub fn TEE_BigIntConvertToS32(dest: *mut int32_t, src: *const TEE_BigInt) -> TEE_Result;

    // TEE Arithmetical API - Logical operations

    pub fn TEE_BigIntCmp(op1: *const TEE_BigInt, op2: *const TEE_BigInt) -> int32_t;
    pub fn TEE_BigIntCmpS32(op: *const TEE_BigInt, shortVal: int32_t) -> int32_t;
    pub fn TEE_BigIntShiftRight(dest: *mut TEE_BigInt, op: *const TEE_BigInt, bits: size_t) -> c_void;
    pub fn TEE_BigIntGetBit(src: *const TEE_BigInt, bitIndex: uint32_t) -> bool;
    pub fn TEE_BigIntGetBitCount(src: *const TEE_BigInt) -> uint32_t;
    pub fn TEE_BigIntAdd(dest: *mut TEE_BigInt, op1: *const TEE_BigInt, op2: *const TEE_BigInt) -> c_void;
    pub fn TEE_BigIntSub(dest: *mut TEE_BigInt, op1: *const TEE_BigInt, op2: *const TEE_BigInt) -> c_void;
    pub fn TEE_BigIntNeg(dest: *mut TEE_BigInt, op: *const TEE_BigInt) -> c_void;
    pub fn TEE_BigIntMul(dest: *mut TEE_BigInt, op1: *const TEE_BigInt, op2: *const TEE_BigInt) -> c_void;
    pub fn TEE_BigIntSquare(dest: *mut TEE_BigInt, op: *const TEE_BigInt) -> c_void;
    pub fn TEE_BigIntDiv(dest_q: *mut TEE_BigInt, dest_r: *mut TEE_BigInt, op1: *const TEE_BigInt, op2: *const TEE_BigInt) -> c_void;

    // TEE Arithmetical API - Modular arithmetic operations

    pub fn TEE_BigIntMod(dest: *mut TEE_BigInt, op: *const TEE_BigInt, n: *const TEE_BigInt) -> c_void;
    pub fn TEE_BigIntAddMod(dest: *mut TEE_BigInt, op1: *const TEE_BigInt, op2: *const TEE_BigInt, n: *const TEE_BigInt) -> c_void;
    pub fn TEE_BigIntSubMod(dest: *mut TEE_BigInt, op1: *const TEE_BigInt, op2: *const TEE_BigInt, n: *const TEE_BigInt) -> c_void;
    pub fn TEE_BigIntMulMod(dest: *mut TEE_BigInt, op1: *const TEE_BigInt, op2: *const TEE_BigInt, n: *const TEE_BigInt) -> c_void;
    pub fn TEE_BigIntSquareMod(dest: *mut TEE_BigInt, op: *const TEE_BigInt, n: *const TEE_BigInt) -> c_void;
    pub fn TEE_BigIntInvMod(dest: *mut TEE_BigInt, op: *const TEE_BigInt, n: *const TEE_BigInt) -> c_void;

    // TEE Arithmetical API - Other arithmetic operations

    pub fn TEE_BigIntRelativePrime(op1: *const TEE_BigInt, op2: *const TEE_BigInt) -> bool;
    pub fn TEE_BigIntComputeExtendedGcd(gcd: *mut TEE_BigInt, u: *mut TEE_BigInt, v: *mut TEE_BigInt, op1: *const TEE_BigInt, op2: *const TEE_BigInt) -> c_void;
    pub fn TEE_BigIntIsProbablePrime(op: *const TEE_BigInt, confidenceLevel: uint32_t) -> int32_t;

    // TEE Arithmetical API - Fast modular multiplication operations

    pub fn TEE_BigIntConvertToFMM(dest: *mut TEE_BigIntFMM, src: *const TEE_BigInt, n: *const TEE_BigInt, context: *const TEE_BigIntFMMContext) -> c_void;
    pub fn TEE_BigIntConvertFromFMM(dest: *mut TEE_BigInt, src: *const TEE_BigIntFMM, n: *const TEE_BigInt, context: *const TEE_BigIntFMMContext) -> c_void;
    pub fn TEE_BigIntFMMConvertToBigInt(dest: *mut TEE_BigInt, src: *const TEE_BigIntFMM, n: *const TEE_BigInt, context: *const TEE_BigIntFMMContext) -> c_void;
    pub fn TEE_BigIntComputeFMM(dest: *mut TEE_BigIntFMM, op1: *const TEE_BigIntFMM, op2: *const TEE_BigIntFMM, n: *const TEE_BigInt, context: *const TEE_BigIntFMMContext) -> c_void;
}
