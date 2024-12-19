resource "cloudflare_zero_trust_access_application" "site_preview" {
  zone_id                   = data.cloudflare_zone.site.id
  name                      = cloudflare_pages_project.site.name
  domain                    = cloudflare_pages_domain.site_dev.domain
  type                      = "self_hosted"
  session_duration          = "720h"
  auto_redirect_to_identity = true

  policies = []
}

resource "cloudflare_zero_trust_access_policy" "site_preview" {
  zone_id        = data.cloudflare_zone.site.id
  application_id = cloudflare_zero_trust_access_application.site_preview.id
  name           = cloudflare_pages_project.site.name
  decision       = "allow"
  precedence     = 1

  include {
    email = var.cloudflare_access_emails
  }

  require {
    email = var.cloudflare_access_emails
  }
}
