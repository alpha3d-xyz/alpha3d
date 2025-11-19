variable "project_id" {
  description = "GCP project ID"
  type        = string
}

variable "region" {
  description = "GCP region for Cloud Run"
  type        = string
}

variable "backend_image" {
  description = "Artifact Registry image for the Axum backend"
  type        = string
}

variable "frontend_image" {
  description = "Artifact Registry image for the Vue frontend"
  type        = string
}

variable "backend_service_name" {
  description = "Cloud Run service name for the backend"
  type        = string
  default     = "vue-axum-backend"
}

variable "frontend_service_name" {
  description = "Cloud Run service name for the frontend"
  type        = string
  default     = "vue-axum-frontend"
}

variable "backend_min_instances" {
  description = "Minimum number of backend instances"
  type        = number
  default     = 0
}

variable "backend_max_instances" {
  description = "Maximum number of backend instances"
  type        = number
  default     = 3
}

variable "frontend_min_instances" {
  description = "Minimum number of frontend instances"
  type        = number
  default     = 0
}

variable "frontend_max_instances" {
  description = "Maximum number of frontend instances"
  type        = number
  default     = 3
}
