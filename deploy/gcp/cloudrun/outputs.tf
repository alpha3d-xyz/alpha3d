output "backend_url" {
  description = "Public URL of the Cloud Run backend"
  value       = google_cloud_run_service.backend.status[0].url
}

output "frontend_url" {
  description = "Public URL of the Cloud Run frontend"
  value       = google_cloud_run_service.frontend.status[0].url
}
