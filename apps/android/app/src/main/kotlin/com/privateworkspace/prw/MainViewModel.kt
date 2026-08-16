package com.privateworkspace.prw

import androidx.lifecycle.ViewModel
import java.security.SecureRandom
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

internal data class PrwUiState(
    val connectionState: ConnectionState = ConnectionState.Disconnected,
    val identityReady: Boolean = false,
    val nativeBridgeReady: Boolean = false,
    val bootstrapValidated: Boolean = false,
    val detail: String = "Development bootstrap only — no production endpoint",
)

internal class MainViewModel(
    private val custody: AndroidKeyCustody = AndroidKeyCustody(),
    private val controller: ConnectionController = ConnectionController(),
) : ViewModel() {
    private val mutableUiState = MutableStateFlow(PrwUiState())
    val uiState: StateFlow<PrwUiState> = mutableUiState.asStateFlow()

    fun validateLocalBootstrap() {
        if (!controller.transition(ConnectionState.Connecting)) return
        publish("Preparing non-exportable Android identity")
        runCatching {
            check(NativeBridge.protocolVersion() == 1)
            val spki = custody.ensureDeviceIdentitySpki()
            check(custody.deviceIdentityIsNonExportable())
            controller.transition(ConnectionState.Authenticating)
            publish("Validating typed local session proof")
            val nonce = ByteArray(32).also(SecureRandom()::nextBytes)
            val request = BootstrapRequest(
                workspaceId = "phase145-development-workspace",
                userId = "phase145-development-user",
                deviceId = "phase145-android-device",
                sessionId = "phase145-local-session",
                publicSpki = spki,
                nonce = nonce,
            ).encode()
            val canonical = NativeBridge.canonicalSessionMessage(request)
            check(canonical.isNotEmpty())
            val signature = custody.signCanonicalSessionProof(canonical)
            check(NativeBridge.verifySessionSignature(request, signature))
            check(controller.transition(ConnectionState.Connected))
        }.onSuccess {
            mutableUiState.value = PrwUiState(
                connectionState = ConnectionState.Connected,
                identityReady = true,
                nativeBridgeReady = true,
                bootstrapValidated = true,
                detail = "Local authenticated bootstrap validated; remote production networking remains disabled",
            )
        }.onFailure { error ->
            controller.transition(ConnectionState.Error)
            mutableUiState.value = PrwUiState(
                connectionState = ConnectionState.Error,
                identityReady = runCatching { custody.deviceIdentityIsNonExportable() }.getOrDefault(false),
                nativeBridgeReady = runCatching { NativeBridge.protocolVersion() == 1 }.getOrDefault(false),
                bootstrapValidated = false,
                detail = error.message ?: "Local bootstrap failed closed",
            )
        }
    }

    fun disconnect() {
        val current = controller.state.value
        if (current == ConnectionState.Connected || current == ConnectionState.Suspended) {
            check(controller.transition(ConnectionState.Disconnecting))
            check(controller.transition(ConnectionState.Disconnected))
        } else if (current == ConnectionState.Error) {
            check(controller.transition(ConnectionState.Disconnected))
        }
        mutableUiState.value = PrwUiState(
            connectionState = controller.state.value,
            identityReady = mutableUiState.value.identityReady,
            nativeBridgeReady = mutableUiState.value.nativeBridgeReady,
            bootstrapValidated = false,
        )
    }

    private fun publish(detail: String) {
        mutableUiState.value = mutableUiState.value.copy(
            connectionState = controller.state.value,
            detail = detail,
        )
    }
}
