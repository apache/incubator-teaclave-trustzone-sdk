use super::*;
use libc::*;

extern "C" {
    pub fn TEE_SEServiceOpen(seServiceHandle: *mut TEE_SEServiceHandle) -> TEE_Result;
    pub fn TEE_SEServiceClose(seServiceHandle: TEE_SEServiceHandle);
    pub fn TEE_SEServiceGetReaders(
        seServiceHandle: TEE_SEServiceHandle,
        seReaderHandleList: *mut TEE_SEReaderHandle,
        seReaderHandleListLen: *mut size_t,
    ) -> TEE_Result;
    pub fn TEE_SEReaderGetProperties(
        seReaderHandle: TEE_SEReaderHandle,
        readerProperties: *mut TEE_SEReaderProperties,
    );
    pub fn TEE_SEReaderGetName(
        seReaderHandle: TEE_SEReaderHandle,
        readerName: *mut c_char,
        readerNameLen: *mut size_t,
    ) -> TEE_Result;
    pub fn TEE_SEReaderOpenSession(
        seReaderHandle: TEE_SEReaderHandle,
        seSessionHandle: *mut TEE_SESessionHandle,
    ) -> TEE_Result;
    pub fn TEE_SEReaderCloseSessions(seReaderHandle: TEE_SEReaderHandle);
    pub fn TEE_SESessionGetATR(
        seSessionHandle: TEE_SESessionHandle,
        atr: *mut c_void,
        atrLen: *mut size_t,
    ) -> TEE_Result;
    pub fn TEE_SESessionIsClosed(seSessionHandle: TEE_SESessionHandle) -> TEE_Result;
    pub fn TEE_SESessionClose(seSessionHandle: TEE_SESessionHandle);
    pub fn TEE_SESessionOpenBasicChannel(
        seSessionHandle: TEE_SESessionHandle,
        seAID: *mut TEE_SEAID,
        seChannelHandle: *mut TEE_SEChannelHandle,
    ) -> TEE_Result;
    pub fn TEE_SESessionOpenLogicalChannel(
        seSessionHandle: TEE_SESessionHandle,
        seAID: *mut TEE_SEAID,
        seChannelHandle: *mut TEE_SEChannelHandle,
    ) -> TEE_Result;
    pub fn TEE_SEChannelSelectNext(seChannelHandle: TEE_SEChannelHandle) -> TEE_Result;
    pub fn TEE_SEChannelGetSelectResponse(
        seChannelHandle: TEE_SEChannelHandle,
        response: *mut c_void,
        responseLen: *mut size_t,
    ) -> TEE_Result;
    pub fn TEE_SEChannelTransmit(
        seChannelHandle: TEE_SEChannelHandle,
        command: *mut c_void,
        commandLen: size_t,
        response: *mut c_void,
        responseLen: *mut size_t,
    ) -> TEE_Result;
    pub fn TEE_SEChannelClose(seChannelHandle: TEE_SEChannelHandle);
}
