terraform {
  required_providers {
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.48"
    }
  }

  cloud {
    organization = "lark"

    workspaces {
      name = "notwithouthelp"
    }
  }
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}
