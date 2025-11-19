terraform {
  required_version = ">= 1.6.0"
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.32"
    }
  }
}

provider "google" {
  project = var.project_id
  region  = var.region
}

resource "google_cloud_run_service" "backend" {
  name     = var.backend_service_name
  location = var.region

  template {
    spec {
      containers {
        image = var.backend_image
        env {
          name  = "RUST_LOG"
          value = "info"
        }
      }
    }
    metadata {
      annotations = {
        "run.googleapis.com/ingress" = "all"
        "autoscaling.knative.dev/minScale" = tostring(var.backend_min_instances)
        "autoscaling.knative.dev/maxScale" = tostring(var.backend_max_instances)
      }
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }

  autogenerate_revision_name = true
}

resource "google_cloud_run_service_iam_member" "backend_invoker" {
  location = google_cloud_run_service.backend.location
  project  = var.project_id
  service  = google_cloud_run_service.backend.name
  role     = "roles/run.invoker"
  member   = "allUsers"
}

resource "google_cloud_run_service" "frontend" {
  name     = var.frontend_service_name
  location = var.region

  template {
    spec {
      containers {
        image = var.frontend_image
        ports {
          container_port = 8080
        }
      }
    }
    metadata {
      annotations = {
        "run.googleapis.com/ingress" = "all"
        "autoscaling.knative.dev/minScale" = tostring(var.frontend_min_instances)
        "autoscaling.knative.dev/maxScale" = tostring(var.frontend_max_instances)
      }
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }

  autogenerate_revision_name = true
}

resource "google_cloud_run_service_iam_member" "frontend_invoker" {
  location = google_cloud_run_service.frontend.location
  project  = var.project_id
  service  = google_cloud_run_service.frontend.name
  role     = "roles/run.invoker"
  member   = "allUsers"
}
