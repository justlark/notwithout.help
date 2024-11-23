terraform {
  required_providers {
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.0"
    }
  }

  cloud {
    organization = "lark"

    workspaces {
      name = "matchdrop"
    }
  }
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}
