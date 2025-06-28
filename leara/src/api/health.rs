/*
 * Leara AI Assistant - Health Check API Handler
 * 
 * This module provides health check endpoints for monitoring system status.
 * Returns basic system information and service status.
 * 
 * Copyright (c) 2024 Leara AI Assistant Contributors
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * Author: KleaSCM
 * Created: 2024-06-28
 * Last Modified: 2024-06-28
 * Version: 0.1.0
 * 
 * File: src/api/health.rs
 * Purpose: Health check API endpoint handlers
 */

// Import Axum web framework components for HTTP handling
use axum::{
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
// Import Serde for JSON serialization
use serde::Serialize;
// Import chrono for timestamp handling
use chrono::Utc;
use crate::models::AppState;

/// Health check response structure
/// Contains essential system status information for monitoring and debugging
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// Current health status of the service ("healthy", "degraded", "unhealthy")
    pub status: String,
    /// ISO 8601 timestamp when the health check was performed
    pub timestamp: String,
    /// Application version from Cargo.toml (e.g., "0.1.0")
    pub version: String,
    /// System uptime in seconds since Unix epoch
    pub uptime: u64,
}

/// Health check endpoint for service monitoring
/// 
/// This function provides a lightweight health check that can be used by:
/// - Load balancers to determine if the service is ready to receive traffic
/// - Monitoring systems to track service availability
/// - DevOps tools for automated health monitoring
/// - Kubernetes liveness/readiness probes
/// 
/// The endpoint performs minimal processing to ensure fast response times
/// and doesn't require database connectivity or external service calls.
/// 
/// # Returns
/// * `(StatusCode::OK, Json<HealthResponse>)` - Service is healthy and operational
/// 
/// # Example Response
/// ```json
/// {
///   "status": "healthy",
///   "timestamp": "2024-06-28T10:15:30.123456Z",
///   "version": "0.1.0",
///   "uptime": 1732809330
/// }
/// ```
/// 
/// # Usage Examples
/// ```bash
/// # Basic health check
/// curl http://localhost:3000/health
/// 
/// # Health check with verbose output
/// curl -v http://localhost:3000/health
/// 
/// # Health check for monitoring systems
/// curl -f http://localhost:3000/health || echo "Service is down"
/// ```
pub async fn health_check() -> (StatusCode, Json<HealthResponse>) {
    // Create health response with current system information
    // This is a lightweight check that doesn't require external dependencies
    let health = HealthResponse {
        // Service status - could be enhanced to check actual service health
        status: "healthy".to_string(),
        // Current timestamp for monitoring and debugging
        timestamp: Utc::now().to_rfc3339(),
        // Application version from Cargo.toml build-time constants
        version: env!("CARGO_PKG_VERSION").to_string(),
        // System uptime calculated from Unix epoch
        // This provides a simple way to track how long the system has been running
        uptime: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    };

    // Return HTTP 200 OK with health information
    // This indicates the service is ready to handle requests
    (StatusCode::OK, Json(health))
}

/// Create router for health-related endpoints
pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", get(health_check))
} 