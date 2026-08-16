package com.privateworkspace.prw

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel

internal class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            MaterialTheme {
                val model: MainViewModel = viewModel()
                val state by model.uiState.collectAsState()
                Surface(modifier = Modifier.fillMaxSize()) {
                    Column(
                        modifier = Modifier.padding(24.dp),
                        verticalArrangement = Arrangement.spacedBy(12.dp),
                    ) {
                        Text("Private Remote Workspace", style = MaterialTheme.typography.headlineMedium)
                        Text("Phase 146 enrollment and device management")
                        Text("Connection: ${state.connectionState}")
                        Text("Identity ready: ${state.identityReady}")
                        Text("Native bridge: ${state.nativeBridgeReady}")
                        Text("Authenticated bootstrap: ${state.bootstrapValidated}")
                        Text("Enrollment proof: ${state.enrollmentState}")
                        Text(state.detail)
                        Button(onClick = model::validateLocalBootstrap) {
                            Text("Validate local bootstrap")
                        }
                        Button(onClick = model::validateLocalEnrollmentProof) {
                            Text("Validate local enrollment proof")
                        }
                        Button(onClick = model::loadDisposableDeviceSnapshots) {
                            Text("Load disposable device snapshots")
                        }
                        state.devices.forEach { device ->
                            Text("${device.deviceId}: ${device.lifecycle}")
                            if (device.lifecycle == DeviceLifecycleView.Enrolled) {
                                Button(onClick = { model.requestRevocation(device.deviceId) }) {
                                    Text(
                                        if (state.pendingRevocationDeviceId == device.deviceId) {
                                            "Revocation pending"
                                        } else {
                                            "Request revocation"
                                        },
                                    )
                                }
                            }
                        }
                        Button(onClick = model::disconnect) {
                            Text("Disconnect")
                        }
                    }
                }
            }
        }
    }
}
