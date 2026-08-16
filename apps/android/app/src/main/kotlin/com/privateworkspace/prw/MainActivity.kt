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
                        Text("Phase 145 Android foundation")
                        Text("State: ${state.connectionState}")
                        Text("Identity ready: ${state.identityReady}")
                        Text("Native bridge: ${state.nativeBridgeReady}")
                        Text("Authenticated bootstrap: ${state.bootstrapValidated}")
                        Text(state.detail)
                        Button(onClick = model::validateLocalBootstrap) {
                            Text("Validate local bootstrap")
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
