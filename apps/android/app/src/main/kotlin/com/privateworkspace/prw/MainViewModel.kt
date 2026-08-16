package com.privateworkspace.prw

import androidx.lifecycle.ViewModel
import java.security.SecureRandom
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

internal enum class EnrollmentPresentationState {
    NotReady,
    Ready,
    ProofValidated,
    Error,
}

internal data class PrwUiState(
    val connectionState: ConnectionState = ConnectionState.Disconnected,
    val identityReady: Boolean = false,
    val nativeBridgeReady: Boolean = false,
    val bootstrapValidated: Boolean = false,
    val enrollmentState: EnrollmentPresentationState = EnrollmentPresentationState.NotReady,
    val devices: List<DeviceSnapshot> = emptyList(),
    val pendingRevocationDeviceId: String? = null,
    val terminal: TerminalUiState = TerminalUiState(),
    val detail: String = "Development bootstrap only — no production endpoint",
)

internal class MainViewModel(
    private val custody: AndroidKeyCustody = AndroidKeyCustody(),
    private val controller: ConnectionController = ConnectionController(),
    private val deviceManagement: DeviceManagementController = DeviceManagementController(),
    private val terminalController: TerminalSessionController = TerminalSessionController(NativeTerminalCommandEncoder),
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
            mutableUiState.value = mutableUiState.value.copy(
                connectionState = ConnectionState.Connected,
                identityReady = true,
                nativeBridgeReady = true,
                bootstrapValidated = true,
                detail = "Local authenticated bootstrap validated; remote production networking remains disabled",
            )
        }.onFailure { error ->
            controller.transition(ConnectionState.Error)
            mutableUiState.value = mutableUiState.value.copy(
                connectionState = ConnectionState.Error,
                identityReady = runCatching { custody.deviceIdentityIsNonExportable() }.getOrDefault(false),
                nativeBridgeReady = runCatching { NativeBridge.protocolVersion() == 1 }.getOrDefault(false),
                bootstrapValidated = false,
                detail = error.message ?: "Local bootstrap failed closed",
            )
        }
    }

    fun validateLocalEnrollmentProof() {
        mutableUiState.value = mutableUiState.value.copy(
            enrollmentState = EnrollmentPresentationState.Ready,
            detail = "Preparing disposable typed enrollment proof; no enrollment authority is contacted",
        )
        runCatching {
            check(NativeBridge.protocolVersion() == 1)
            val spki = custody.ensureDeviceIdentitySpki()
            check(custody.deviceIdentityIsNonExportable())
            val nonce = ByteArray(32).also(SecureRandom()::nextBytes)
            val request = EnrollmentProofRequest(
                enrollmentId = "phase146-local-enrollment",
                workspaceId = "phase146-development-workspace",
                userId = "phase146-development-user",
                deviceId = "phase146-android-device",
                publicSpki = spki,
                nonce = nonce,
            ).encode()
            val canonical = NativeBridge.canonicalEnrollmentMessage(request)
            check(canonical.isNotEmpty())
            val signature = custody.signCanonicalEnrollmentProof(canonical)
            check(NativeBridge.verifyEnrollmentSignature(request, signature))
        }.onSuccess {
            mutableUiState.value = mutableUiState.value.copy(
                identityReady = true,
                nativeBridgeReady = true,
                enrollmentState = EnrollmentPresentationState.ProofValidated,
                detail = "Disposable enrollment proof validated locally; authoritative enrollment remains unchanged",
            )
        }.onFailure { error ->
            mutableUiState.value = mutableUiState.value.copy(
                enrollmentState = EnrollmentPresentationState.Error,
                detail = error.message ?: "Enrollment proof failed closed",
            )
        }
    }

    fun loadDisposableDeviceSnapshots() {
        val accepted = deviceManagement.applyAuthoritativeSnapshot(
            listOf(
                DeviceSnapshot("phase146-android-device", DeviceLifecycleView.Enrolled),
                DeviceSnapshot("phase146-pending-device", DeviceLifecycleView.PendingEnrollment),
            ),
        )
        check(accepted)
        publishDeviceState("Loaded disposable authoritative device snapshots; no production registry contacted")
    }

    fun requestRevocation(deviceId: String) {
        val accepted = deviceManagement.requestRevocation(deviceId)
        publishDeviceState(
            if (accepted) {
                "Revocation intent pending for $deviceId; authoritative lifecycle remains Enrolled"
            } else {
                "Revocation intent rejected by current authoritative lifecycle/pending state"
            },
        )
    }

    fun requestDisposableTerminal(profile: TerminalProfileView): Boolean {
        val accepted = terminalController.requestOpen(DISPOSABLE_TERMINAL_SESSION_ID, profile, 80, 24)
        publishTerminalState(
            if (accepted) {
                "Disposable terminal open intent encoded; no production endpoint contacted"
            } else {
                "Terminal open intent rejected by local bounded lifecycle"
            },
        )
        return accepted
    }

    fun acceptDisposableTerminalOpen(): Boolean {
        val accepted = terminalController.applyAuthoritativeOpen(DISPOSABLE_TERMINAL_SESSION_ID)
        publishTerminalState(
            if (accepted) "Disposable terminal open acceptance applied" else "Terminal open acceptance rejected",
        )
        return accepted
    }

    fun sendDisposableTerminalInput(text: String): Boolean {
        val accepted = terminalController.sendInput(text.encodeToByteArray())
        publishTerminalState(
            if (accepted) "Terminal input payload encoded through existing PRWC bridge" else "Terminal input rejected",
        )
        return accepted
    }

    fun requestDisposableTerminalRead(): Boolean {
        val accepted = terminalController.requestRead(4096)
        publishTerminalState(
            if (accepted) "Bounded terminal read payload encoded" else "Terminal read request rejected",
        )
        return accepted
    }

    fun injectDisposableTerminalOutput(): Boolean {
        val accepted = terminalController.applyAuthoritativeOutput(
            "phase147 disposable remote output\n".encodeToByteArray(),
        )
        publishTerminalState(
            if (accepted) "Disposable authoritative output applied" else "Terminal output rejected",
        )
        return accepted
    }

    fun resizeDisposableTerminal(columns: Int, rows: Int): Boolean {
        val accepted = terminalController.resize(columns, rows)
        publishTerminalState(if (accepted) "Terminal resize payload encoded" else "Terminal resize rejected")
        return accepted
    }

    fun requestDisposableTerminalClose(): Boolean {
        val accepted = terminalController.requestClose()
        publishTerminalState(
            if (accepted) "Terminal close intent encoded; awaiting disposable completion" else "Terminal close rejected",
        )
        return accepted
    }

    fun acceptDisposableTerminalClosed(): Boolean {
        val accepted = terminalController.applyAuthoritativeClosed(DISPOSABLE_TERMINAL_SESSION_ID)
        publishTerminalState(
            if (accepted) "Disposable terminal close completed" else "Terminal close completion rejected",
        )
        return accepted
    }

    fun disconnect() {
        val current = controller.state.value
        if (current == ConnectionState.Connected || current == ConnectionState.Suspended) {
            check(controller.transition(ConnectionState.Disconnecting))
            check(controller.transition(ConnectionState.Disconnected))
        } else if (current == ConnectionState.Error) {
            check(controller.transition(ConnectionState.Disconnected))
        }
        mutableUiState.value = mutableUiState.value.copy(
            connectionState = controller.state.value,
            bootstrapValidated = false,
            detail = "Development bootstrap only — no production endpoint",
        )
    }

    private fun publish(detail: String) {
        mutableUiState.value = mutableUiState.value.copy(
            connectionState = controller.state.value,
            detail = detail,
        )
    }

    private fun publishDeviceState(detail: String) {
        val deviceState = deviceManagement.state()
        mutableUiState.value = mutableUiState.value.copy(
            devices = deviceState.devices,
            pendingRevocationDeviceId = deviceState.pendingRevocationDeviceId,
            detail = detail,
        )
    }

    private fun publishTerminalState(detail: String) {
        mutableUiState.value = mutableUiState.value.copy(
            terminal = terminalController.state(),
            detail = detail,
        )
    }

    companion object {
        private const val DISPOSABLE_TERMINAL_SESSION_ID = 147_001L
    }
}
