resource "cloudflare_zero_trust_access_application" "site_preview" {
  account_id                = var.cloudflare_account_id
  name                      = cloudflare_pages_project.site.name
  domain                    = "*.${cloudflare_pages_project.site.name}.pages.dev"
  type                      = "self_hosted"
  session_duration          = "720h"
  auto_redirect_to_identity = true
  policies = [
    cloudflare_zero_trust_access_policy.site_preview.id,
  ]
}

resource "cloudflare_zero_trust_access_policy" "site_preview" {
  account_id = var.cloudflare_account_id
  name       = cloudflare_pages_project.site.name
  decision   = "allow"

  include {
    email = var.cloudflare_access_emails
  }

  require {
    email = var.cloudflare_access_emails
  }
}
