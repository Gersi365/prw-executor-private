package com.privateworkspace.prw

internal object NativeBridge {
    init {
        System.loadLibrary("prw_android_adapter")
    }

    @JvmStatic external fun protocolVersion(): Int
    @JvmStatic external fun roundTripControlFrame(frame: ByteArray): ByteArray
    @JvmStatic external fun canonicalSessionMessage(request: ByteArray): ByteArray
    @JvmStatic external fun verifySessionSignature(request: ByteArray, signature: ByteArray): Boolean
}
