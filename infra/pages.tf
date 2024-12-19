resource "cloudflare_pages_project" "site" {
  account_id        = var.cloudflare_account_id
  name              = "notwithouthelp"
  production_branch = "prod"

  build_config {
    build_command   = "npm install && npm run build"
    root_dir        = "client"
    destination_dir = "dist"
  }

  source {
    type = "github"

    config {
      owner                      = "justlark"
      repo_name                  = "notwithout.help"
      production_branch          = "prod"
      preview_deployment_setting = "custom"
      preview_branch_includes    = ["main"]
    }
  }

  deployment_configs {
    production {
      environment_variables = {
        VITE_API_URL = "https://api.${data.cloudflare_zone.site.name}"
      }
    }

    preview {
      environment_variables = {
        VITE_API_URL = "https://api-dev.${data.cloudflare_zone.site.name}"
      }
    }
  }
}

resource "cloudflare_pages_domain" "site" {
  account_id   = var.cloudflare_account_id
  project_name = cloudflare_pages_project.site.name
  domain       = data.cloudflare_zone.site.name
}

resource "cloudflare_pages_domain" "site_dev" {
  account_id   = var.cloudflare_account_id
  project_name = cloudflare_pages_project.site.name
  domain       = "dev.${data.cloudflare_zone.site.name}"
}
