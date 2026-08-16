package com.privateworkspace.prw

internal object NativeBridge {
    init {
        System.loadLibrary("prw_android_adapter")
    }

    @JvmStatic external fun protocolVersion(): Int
    @JvmStatic external fun roundTripControlFrame(frame: ByteArray): ByteArray
    @JvmStatic external fun canonicalSessionMessage(request: ByteArray): ByteArray
    @JvmStatic external fun verifySessionSignature(request: ByteArray, signature: ByteArray): Boolean
    @JvmStatic external fun canonicalEnrollmentMessage(request: ByteArray): ByteArray
    @JvmStatic external fun verifyEnrollmentSignature(request: ByteArray, signature: ByteArray): Boolean
    @JvmStatic external fun terminalOpenPayload(sessionId: Long, profileCode: Int, columns: Int, rows: Int): ByteArray
    @JvmStatic external fun terminalInputPayload(sessionId: Long, input: ByteArray): ByteArray
    @JvmStatic external fun terminalResizePayload(sessionId: Long, columns: Int, rows: Int): ByteArray
    @JvmStatic external fun terminalReadPayload(sessionId: Long, maximumBytes: Int): ByteArray
    @JvmStatic external fun terminalClosePayload(sessionId: Long): ByteArray
    @JvmStatic external fun isTerminalPayload(payload: ByteArray): Boolean
}
